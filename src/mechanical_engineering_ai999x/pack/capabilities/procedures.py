"""Mechanical engineering procedures: deterministic capabilities with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: statics, stress and strain, beams and frames, section properties, torsion, columns, fatigue, machine
elements, vibrations and structural analysis.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question template it parses,
worked examples, an INDEPENDENT reference computation (used only by certification; the checking libraries anastruct,
sectionproperties, fatpack, rainflow, numpy, scipy and sympy are imported inside the reference functions) and a
deliberately wrong variant that the checks must reject. The shipped code (run, verify, parse) is plain Python
(math, re, fractions, itertools).

Conventions (fixed, so run and reference agree exactly):
- SI inside: N, m, Pa, N*m, rad. Results also come in well-scaled companions (kN, kN*m, MPa, mm, mm^4, degrees).
- Angles of forces are measured counterclockwise from the +x axis, in degrees; a moment is positive counterclockwise.
- Beams are horizontal, x runs from the left end A (x = 0) to the right end B (x = L); loads act downward and are
  given as positive numbers. Reactions are positive upward. Bending moments are reported as magnitudes and named:
  "hogging" end moments of fixed ends, "sagging" span moments. Deflections are positive downward.
- Euler-Bernoulli beams (plane sections, small deflections, no shear deformation), linear elastic, prismatic.
- Sections: x is the horizontal centroidal axis, y the vertical one; a T-section's flange is on top and its centroid
  is measured from the bottom of the stem; a channel's web is on the left and its centroid is measured from the back
  of the web; an angle's corner is at the origin with one leg along +x and one along +y. The product of inertia is
  I_xy = integral of x*y dA about the centroid; the principal angle is that of the major axis, counterclockwise from
  x, in (-90, 90] degrees. Lists written in a question carry no unit: section dimensions in lists are in mm.
- Results built from cos, sin, exp, log or sqrt are rounded to 12 significant digits (identical on every platform and
  Python version); values that are zero by symmetry or balance (below 1e-12 of their scale) are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Cantilever with end load: L = 2 m, P = 5 kN, E = 200 GPa, I = 8e-6 m^4`); physical values carry a unit (psi,
ksi, lbf, kip, in and their products are converted here); lists are written in brackets (`F = [10, 20]`), matrices
as rows separated by semicolons (`rects = [200 20 0 0; 20 300 0 160]`). Any other number or an unknown name is
refused, never guessed.
"""
from __future__ import annotations

import functools
import math
import re

# ================================================================================================ shared helpers
G0 = 9.80665                                   # standard gravity, m/s^2 (exact by definition)


def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, (bool, str, int)):
        return x
    if isinstance(x, list):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _snap(x, scale):
    """A value that is zero by symmetry or balance: below 1e-12 of its scale it is written as exactly 0."""
    return 0.0 if abs(x) <= 1e-12 * abs(scale) else x


def _rel(a, b):
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    if isinstance(a, list) or isinstance(b, list):
        if not (isinstance(a, list) and isinstance(b, list)) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] or [0.0])
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    s = max(abs(a), abs(b))
    return d / s if s > 0 else math.inf


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _pos(name, *vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and math.isfinite(v) and v > 0):
            raise ValueError(f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and math.isfinite(v) and v >= 0):
            raise ValueError(f"{name} must be zero or positive")


def _fin(name, *vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and math.isfinite(v)):
            raise ValueError(f"{name} must be a finite number")


# units the shared unit reader does not know: converted here to SI (factor, SI unit)
_IN, _LBF = 0.0254, 4.4482216152605
_EXTRA_UNITS = {
    "psi": (6894.757293168361, "Pa"), "ksi": (6894757.293168361, "Pa"), "Mpsi": (6894757293.168361, "Pa"),
    "lbf": (_LBF, "N"), "lb": (_LBF, "N"), "kip": (1000 * _LBF, "N"), "kips": (1000 * _LBF, "N"),
    "in": (_IN, "m"), "inch": (_IN, "m"), "inches": (_IN, "m"),
    "in^2": (_IN ** 2, "m^2"), "in^3": (_IN ** 3, "m^3"), "in^4": (_IN ** 4, "m^4"),
    "lbf*in": (_LBF * _IN, "N*m"), "lb*in": (_LBF * _IN, "N*m"), "lbf in": (_LBF * _IN, "N*m"), "lb in": (_LBF * _IN, "N*m"),
    "lbf*ft": (_LBF * 0.3048, "N*m"), "lb*ft": (_LBF * 0.3048, "N*m"), "lbf ft": (_LBF * 0.3048, "N*m"), "lb ft": (_LBF * 0.3048, "N*m"),
    "kip*in": (1000 * _LBF * _IN, "N*m"), "kip*ft": (1000 * _LBF * 0.3048, "N*m"),
    "lbf/in": (_LBF / _IN, "N/m"), "lb/in": (_LBF / _IN, "N/m"), "kip/in": (1000 * _LBF / _IN, "N/m"),
    "lbf/ft": (_LBF / 0.3048, "N/m"), "lb/ft": (_LBF / 0.3048, "N/m"), "kip/ft": (1000 * _LBF / 0.3048, "N/m"),
}
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_LIST = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*\[([^\[\]]*)\]\s*(.*?)\s*$")


def _split(rest):
    """Split 'a = 1, b = [1, 2, 3], c = 4' on commas and semicolons outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch in ",;" and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts if depth == 0 else None


def _pairs(question, phrase, names, phys=(), words=(), vectors=(), matrices=(), edges=(), num_units=None):
    """{symbol: value} from '<phrase>: name = value, ...'. None unless the phrase opens the question and every part is
    a 'name = value' pair of a known name. `phys`: symbols that need a unit (returned as 'value unit' strings, read by
    the runtime; psi, ksi, lbf, kip, in ... are converted here); `words`: symbols whose value is a word (a choice);
    `vectors`, `matrices`, `edges`: lists written in brackets; `num_units`: {symbol: [allowed unit words]} for plain
    numbers (percent ...)."""
    if not isinstance(question, str):
        return None
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
    if not rest:
        return {}
    segs = _split(rest)
    if segs is None:
        return None
    out = {}
    num_units = num_units or {}
    look = lambda s: names.get(s, names.get(s.lower()))
    for seg in segs:
        if not seg.strip():
            continue
        ml = _LIST.match(seg)
        if ml:
            key = look(ml.group(1))
            if key is None or key in out or ml.group(3) or key not in set(vectors) | set(matrices) | set(edges):
                return None
            out[key] = ml.group(2).strip()
            continue
        mw = _WORD.match(seg)
        if mw and look(mw.group(1)) in set(words):
            key = look(mw.group(1))
            if key in out:
                return None
            out[key] = mw.group(2).strip().lower()
            continue
        ms = _SEG.match(seg)
        if not ms:
            return None
        key = look(ms.group(1))
        if key is None or key in out or key in words or key in vectors or key in matrices or key in edges:
            return None
        num, unit = ms.group(2), ms.group(3).strip()
        if key in phys:
            if not unit:
                return None
            if unit in _EXTRA_UNITS:
                f, si = _EXTRA_UNITS[unit]
                out[key] = f"{float(num) * f!r} {si}"
            else:
                out[key] = f"{num} {unit}"
        else:
            if unit and unit not in num_units.get(key, ()):
                return None
            out[key] = float(num) / (100.0 if unit in ("%", "percent") else 1.0)
    return out


def _phys(sym, kind, meaning, required=True):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _vec(sym, meaning, lo=1, hi=200, required=True):
    return {"symbol": sym, "type": "vector", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def _mat(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "meaning": meaning, "required": required}


def _edges(sym, meaning, required=True, lo=1, hi=60):
    return {"symbol": sym, "type": "edges", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec.setdefault("verify", _recheck(spec["run"]))
    PROCEDURES[key] = spec


def _deg(x):
    return x * math.pi / 180.0


def _cosd(a):
    """cos of an angle in degrees, exact at multiples of 90 degrees."""
    r = a % 360.0
    return {0.0: 1.0, 90.0: 0.0, 180.0: -1.0, 270.0: 0.0}.get(r, math.cos(_deg(a)))


def _sind(a):
    r = a % 360.0
    return {0.0: 0.0, 90.0: 1.0, 180.0: 0.0, 270.0: -1.0}.get(r, math.sin(_deg(a)))


# ================================================================================================ statics
def resultant_forces(F, theta):
    if len(F) != len(theta):
        raise ValueError("F and theta must have the same number of entries")
    _fin("each force and angle", *F, *theta)
    if any(f < 0 for f in F):
        raise ValueError("force magnitudes must be zero or positive (give the direction by the angle)")
    scale = sum(F)
    if scale <= 0:
        raise ValueError("at least one force must be non-zero")
    Rx = _snap(math.fsum(f * _cosd(t) for f, t in zip(F, theta)), scale)
    Ry = _snap(math.fsum(f * _sind(t) for f, t in zip(F, theta)), scale)
    R = math.hypot(Rx, Ry)
    if R <= 1e-12 * scale:
        return _out(R=0.0, Rx=0.0, Ry=0.0, direction_deg=0.0)
    return _out(R=R, Rx=Rx, Ry=Ry, direction_deg=math.degrees(math.atan2(Ry, Rx)))


def ref_resultant_forces(F, theta):
    """sympy.physics.vector: each force as a vector of a reference frame (30-digit trigonometry), added by the
    library and its magnitude and direction taken from the summed vector."""
    import sympy as sp
    from sympy.physics.vector import ReferenceFrame
    N = ReferenceFrame("N")
    v = 0 * N.x
    for f, t in zip(F, theta):
        ang = sp.pi * sp.Float(t, 30) / 180
        v = v + sp.Float(f, 30) * (sp.cos(ang) * N.x + sp.sin(ang) * N.y)
    Rx, Ry = float(sp.N(v.dot(N.x), 30)), float(sp.N(v.dot(N.y), 30))
    scale = sum(F)
    Rx, Ry = _snap(Rx, scale), _snap(Ry, scale)
    R = float(sp.N(v.magnitude(), 30))
    if R <= 1e-12 * scale:
        return {"R": 0.0}
    return {"R": R, "Rx": Rx, "Ry": Ry, "direction_deg": float(sp.N(sp.atan2(v.dot(N.y), v.dot(N.x)) * 180 / sp.pi, 30))}


def _rand_forces(rng):
    n = rng.randint(2, 6)
    return {"F": [round(rng.uniform(5, 500), 2) for _ in range(n)], "theta": [round(rng.uniform(-180, 360), 1) for _ in range(n)]}


_add("resultant_of_concurrent_forces",
     name="Resultant of concurrent coplanar forces", topic="statics / force systems",
     purpose="The magnitude, components and direction of the resultant of forces acting through one point in a plane.",
     inputs=[_vec("F", "force magnitudes in N (a list)"), _vec("theta", "direction of each force in degrees, counterclockwise from +x (a list)")],
     outputs=[_o("R", "force", "magnitude of the resultant"), _o("Rx", "force", "x component of the resultant"),
              _o("Ry", "force", "y component of the resultant"), _o("direction_deg", "number", "direction of the resultant in degrees, counterclockwise from +x, in (-180, 180]")],
     primary="R", conditions=["F and theta have the same length", "magnitudes >= 0, at least one > 0", "a zero resultant is reported with direction 0"],
     method="Rx = sum F_i cos(theta_i), Ry = sum F_i sin(theta_i), R = sqrt(Rx^2 + Ry^2), direction = atan2(Ry, Rx)",
     latex=r"R = \sqrt{\Big(\sum F_i\cos\theta_i\Big)^2 + \Big(\sum F_i\sin\theta_i\Big)^2},\quad \theta_R = \operatorname{atan2}(R_y, R_x)",
     run=resultant_forces,
     parse=lambda q: _pairs(q, r"resultant of (?:concurrent )?forces|resultant force", {"F": "F", "forces": "F", "theta": "theta", "angles": "theta"},
                            vectors={"F", "theta"}),
     reference=ref_resultant_forces,
     wrong=_safe(lambda F, theta: (lambda rx, ry: _out(R=math.hypot(rx, ry), Rx=rx, Ry=ry, direction_deg=math.degrees(math.atan2(ry, rx))))(
         sum(f * math.sin(_deg(t)) for f, t in zip(F, theta)), sum(f * math.cos(_deg(t)) for f, t in zip(F, theta)))),
     random_inputs=_rand_forces,
     templates=["Resultant of concurrent forces: F = [300, 200, 150], theta = [0, 60, 135]"],
     examples=[{"inputs": {"F": "3, 4", "theta": "0, 90"}, "outputs": {"R": 5.0, "Rx": 3.0, "Ry": 4.0, "direction_deg": math.degrees(math.atan2(4, 3))}},
               {"inputs": {"F": "10, 10", "theta": "0, 180"}, "outputs": {"R": 0.0}}],
     questions=[{"q": "Resultant of concurrent forces: F = [3, 4], theta = [0, 90]", "outputs": {"R": 5.0, "Rx": 3.0, "Ry": 4.0}},
                {"q": "Resultant of concurrent forces: F = [3, 4], theta = [0, 90, 45]", "outcome": "INVALID_INPUT"},
                {"q": "Resultant of concurrent forces: F = [3, 4]", "outcome": "NEEDS_INPUT"}])


def two_cable_tension(W, alpha, beta):
    _pos("the weight W", W)
    for nm, v in (("alpha", alpha), ("beta", beta)):
        if not 0 < v < math.pi / 2:
            raise ValueError(f"{nm} must lie strictly between 0 and 90 degrees (measured from the horizontal)")
    s = math.sin(alpha + beta)
    T1 = W * math.cos(beta) / s
    T2 = W * math.cos(alpha) / s
    return _out(T1=T1, T2=T2)


def ref_two_cable_tension(W, alpha, beta):
    """Equilibrium of the knot as a linear system (sum Fx = 0, sum Fy = 0) solved by numpy."""
    import numpy as np
    A = np.array([[-math.cos(alpha), math.cos(beta)], [math.sin(alpha), math.sin(beta)]])
    T = np.linalg.solve(A, np.array([0.0, W]))
    return {"T1": float(T[0]), "T2": float(T[1])}


_add("two_cable_equilibrium",
     name="Tensions in two cables holding a weight", topic="statics / equilibrium of a particle",
     purpose="The tensions in two cables, one rising to the left at angle alpha and one rising to the right at angle beta above the horizontal, that hold a weight W at rest.",
     inputs=[_phys("W", "force", "weight held"), _phys("alpha", "angle", "angle of the left cable above the horizontal"),
             _phys("beta", "angle", "angle of the right cable above the horizontal")],
     outputs=[_o("T1", "force", "tension in the left cable"), _o("T2", "force", "tension in the right cable")],
     primary="T1", conditions=["W > 0", "0 < alpha, beta < 90 degrees"],
     method="Lami's theorem: T1 = W cos(beta)/sin(alpha + beta), T2 = W cos(alpha)/sin(alpha + beta)",
     latex=r"T_1 = \frac{W\cos\beta}{\sin(\alpha+\beta)},\quad T_2 = \frac{W\cos\alpha}{\sin(\alpha+\beta)}",
     run=two_cable_tension,
     parse=lambda q: _pairs(q, r"two cable tensions?|tensions? in two cables", {"W": "W", "weight": "W", "alpha": "alpha", "beta": "beta"},
                            phys={"W", "alpha", "beta"}),
     reference=ref_two_cable_tension,
     wrong=_safe(lambda W, alpha, beta: _out(T1=W * math.cos(alpha) / math.sin(alpha + beta), T2=W * math.cos(beta) / math.sin(alpha + beta))),
     random_inputs=lambda rng: {"W": rng.uniform(10, 5e4), "alpha": _deg(rng.uniform(5, 85)), "beta": _deg(rng.uniform(5, 85))},
     templates=["Two cable tensions: W = 500 N, alpha = 30 deg, beta = 45 deg"],
     examples=[{"inputs": {"W": 500.0, "alpha": _deg(30), "beta": _deg(45)},
                "outputs": {"T1": 500 * math.cos(_deg(45)) / math.sin(_deg(75)), "T2": 500 * math.cos(_deg(30)) / math.sin(_deg(75))}},
               {"inputs": {"W": 100.0, "alpha": _deg(45), "beta": _deg(45)}, "outputs": {"T1": 100 / math.sqrt(2), "T2": 100 / math.sqrt(2)}}],
     questions=[{"q": "Two cable tensions: W = 500 N, alpha = 30 deg, beta = 45 deg",
                 "outputs": {"T1": 500 * math.cos(_deg(45)) / math.sin(_deg(75))}},
                {"q": "Two cable tensions: W = 500 N, alpha = 0 deg, beta = 45 deg", "outcome": "INVALID_INPUT"},
                {"q": "Two cable tensions: W = 500 N, alpha = 30 deg", "outcome": "NEEDS_INPUT"}])


def moment_of_force(Fx, Fy, x, y, x0=0.0, y0=0.0):
    _fin("every component and coordinate", Fx, Fy, x, y, x0, y0)
    F = math.hypot(Fx, Fy)
    if F == 0:
        raise ValueError("the force must not be zero")
    rx, ry = x - x0, y - y0
    M = rx * Fy - ry * Fx
    M = _snap(M, F * math.hypot(rx, ry))
    return _out(M=M, d=abs(M) / F)


def ref_moment_of_force(Fx, Fy, x, y, x0=0.0, y0=0.0):
    """Varignon / perpendicular distance: sympy.geometry's distance from the point to the line of action, times the
    force magnitude; the sense from sympy's orientation of (point, application point, point along the force)."""
    import sympy as sp
    from sympy.geometry import Line, Point
    q = lambda v: sp.Rational(repr(float(v)))
    P0, A = Point(q(x0), q(y0)), Point(q(x), q(y))
    B = Point(q(x) + q(Fx), q(y) + q(Fy))
    d = Line(A, B).distance(P0)
    area2 = (A.x - P0.x) * (B.y - P0.y) - (A.y - P0.y) * (B.x - P0.x)     # twice the signed area of (P0, A, B)
    sense = 0 if area2 == 0 else (1 if area2 > 0 else -1)
    Fm = sp.sqrt(q(Fx) ** 2 + q(Fy) ** 2)
    return {"M": float(sense * d * Fm), "d": float(d)}


_add("moment_of_a_force",
     name="Moment of a force about a point (2D)", topic="statics / moments",
     purpose="The moment of a force (Fx, Fy) applied at (x, y) about the point (x0, y0), counterclockwise positive, and the perpendicular distance to its line of action.",
     inputs=[_phys("Fx", "force", "x component of the force"), _phys("Fy", "force", "y component of the force"),
             _phys("x", "length", "x coordinate of the point of application"), _phys("y", "length", "y coordinate of the point of application"),
             _phys("x0", "length", "x coordinate of the moment centre (default 0)", required=False),
             _phys("y0", "length", "y coordinate of the moment centre (default 0)", required=False)],
     outputs=[_o("M", "torque", "moment about the point (counterclockwise positive)"), _o("d", "length", "perpendicular distance from the point to the line of action")],
     primary="M", conditions=["the force is not zero"],
     method="M = (x - x0) Fy - (y - y0) Fx, d = |M| / |F|",
     latex=r"M_O = (x - x_0)F_y - (y - y_0)F_x,\quad d = \frac{|M_O|}{|\mathbf F|}",
     run=moment_of_force,
     parse=lambda q: _pairs(q, r"moment of (?:a )?force(?: about a point)?", {k: k for k in ("Fx", "Fy", "x", "y", "x0", "y0")},
                            phys={"Fx", "Fy", "x", "y", "x0", "y0"}),
     reference=ref_moment_of_force,
     wrong=_safe(lambda Fx, Fy, x, y, x0=0.0, y0=0.0: _out(M=(x - x0) * Fx - (y - y0) * Fy, d=abs((x - x0) * Fx - (y - y0) * Fy) / math.hypot(Fx, Fy))),
     random_inputs=lambda rng: {"Fx": rng.uniform(-500, 500), "Fy": rng.uniform(-500, 500), "x": rng.uniform(-3, 3), "y": rng.uniform(-3, 3),
                                "x0": rng.uniform(-1, 1), "y0": rng.uniform(-1, 1)},
     templates=["Moment of a force: Fx = 100 N, Fy = 200 N, x = 2 m, y = 1 m"],
     examples=[{"inputs": {"Fx": 100.0, "Fy": 200.0, "x": 2.0, "y": 1.0}, "outputs": {"M": 300.0, "d": 300 / math.hypot(100, 200)}},
               {"inputs": {"Fx": 0.0, "Fy": -50.0, "x": 3.0, "y": 0.0, "x0": 1.0, "y0": 0.0}, "outputs": {"M": -100.0, "d": 2.0}}],
     questions=[{"q": "Moment of a force: Fx = 100 N, Fy = 200 N, x = 2 m, y = 1 m", "outputs": {"M": 300.0}},
                {"q": "Moment of a force: Fx = 0 N, Fy = -50 N, x = 3 m, y = 0 m, x0 = 1 m, y0 = 0 m", "outputs": {"M": -100.0, "d": 2.0}},
                {"q": "Moment of a force: Fx = 0 N, Fy = 0 N, x = 3 m, y = 0 m", "outcome": "INVALID_INPUT"}])


# ================================================================================================ beams: references
@functools.lru_cache(maxsize=None)
def _sympy_beam(case):
    """A beam solved once SYMBOLICALLY by sympy.physics.continuum_mechanics.Beam (singularity functions, boundary
    conditions) and turned into fast numeric functions of (x, L, a, P, w, E, I). Load positive downward."""
    import sympy as sp
    from sympy.physics.continuum_mechanics.beam import Beam
    L, a, P, w, E, I = sp.symbols("L a P w E I", positive=True)
    x = sp.Symbol("x")
    R1, R2, M1, M2 = sp.symbols("R1 R2 M1 M2")
    b = Beam(L, E, I)
    unknowns = []
    if case == "ss_udl":
        b.apply_load(R1, 0, -1)
        b.apply_load(R2, L, -1)
        b.bc_deflection = [(0, 0), (L, 0)]
        unknowns = [R1, R2]
    elif case in ("cant_udl", "cant_tri", "cant_point"):
        b.apply_load(R1, 0, -1)
        b.apply_load(M1, 0, -2)
        b.bc_deflection = [(0, 0)]
        b.bc_slope = [(0, 0)]
        unknowns = [R1, M1]
    elif case in ("ff_udl", "ff_point"):
        b.apply_load(R1, 0, -1)
        b.apply_load(M1, 0, -2)
        b.apply_load(R2, L, -1)
        b.apply_load(M2, L, -2)
        b.bc_deflection = [(0, 0), (L, 0)]
        b.bc_slope = [(0, 0), (L, 0)]
        unknowns = [R1, R2, M1, M2]
    elif case in ("pc_udl", "pc_point"):
        b.apply_load(R1, 0, -1)
        b.apply_load(M1, 0, -2)
        b.apply_load(R2, L, -1)
        b.bc_deflection = [(0, 0), (L, 0)]
        b.bc_slope = [(0, 0)]
        unknowns = [R1, R2, M1]
    if case.endswith("udl"):
        b.apply_load(w, 0, 0, end=L)
    elif case.endswith("point"):
        b.apply_load(P, a, -1)
    elif case == "cant_tri":                      # w at the fixed end, falling linearly to 0 at the free end
        b.apply_load(w, 0, 0, end=L)
        b.apply_load(-w / L, 0, 1, end=L)
    b.solve_for_reaction_loads(*unknowns)
    args = (x, L, a, P, w, E, I)
    fn = lambda e: sp.lambdify(args, e, "math")
    reac = {str(k): sp.lambdify(args[1:], v, "math") for k, v in b.reaction_loads.items()}
    return {"reactions": reac, "M": fn(b.bending_moment()), "V": fn(b.shear_force()), "slope": fn(b.slope()), "y": fn(b.deflection())}


def _beam_eval(case, L, a=0.0, P=0.0, w=0.0, E=1.0, I=1.0):
    s = _sympy_beam(case)
    r = {k: f(L, a, P, w, E, I) for k, f in s["reactions"].items()}
    return s, r, (lambda fn, xx: s[fn](xx, L, a, P, w, E, I))


def _max_deflection_numeric(case, L, a, P, w, E, I, lo=0.0, hi=None):
    """The largest downward deflection: roots of the slope (sympy's slope function) bracketed on a grid and found by
    scipy brentq; the deflection taken there from sympy's deflection function."""
    from scipy.optimize import brentq
    s, _r0, ev = _beam_eval(case, L, a, P, w, E, I)
    hi = L if hi is None else hi
    xs = [lo + (hi - lo) * i / 400 for i in range(401)]
    sl = [ev("slope", xx) for xx in xs]
    best = (0.0, 0.0)
    for i in range(400):
        if sl[i] == 0 or sl[i] * sl[i + 1] < 0:
            xr = xs[i] if sl[i] == 0 else brentq(lambda t: ev("slope", t), xs[i], xs[i + 1], xtol=1e-15, rtol=8.9e-16, maxiter=500)
            yv = abs(ev("y", xr))
            if yv > best[0]:
                best = (yv, xr)
    return best


def _mm(v):
    """A coordinate in whole millimetres: anastruct stores node coordinates in float32, exact only for such values
    (random inputs are drawn on a 1 mm grid for that reason)."""
    m = round(v * 1000.0)
    if abs(v * 1000.0 - m) > 1e-6 or abs(m) > 2 ** 24:
        raise ValueError("anastruct reference needs coordinates in whole millimetres")
    return float(m)


def _ana(L, supports, loads, EI):
    """anastruct (2D frame finite elements, exact for point loads at nodes): a horizontal beam with nodes at the
    supports and load points, modelled in N and mm (EI in N*mm^2). supports: {x: 'hinged' | 'roll' | 'fixed'};
    loads: [(x, P downward)]. Returns (system, {x: node id}, xs); node results come back in N, N*mm and mm."""
    from anastruct import SystemElements
    xs = sorted(set([0.0, float(L)] + [float(k) for k in supports] + [float(x) for x, _ in loads]))
    ss = SystemElements(EI=EI * 1e6, EA=EI * 1e6)
    for i in range(len(xs) - 1):
        ss.add_element(location=[[_mm(xs[i]), 0.0], [_mm(xs[i + 1]), 0.0]])
    node = {xv: i + 1 for i, xv in enumerate(xs)}
    for xv, kind in supports.items():
        nid = node[float(xv)]
        if kind == "hinged":
            ss.add_support_hinged(node_id=nid)
        elif kind == "roll":
            ss.add_support_roll(node_id=nid, direction=2)
        else:
            ss.add_support_fixed(node_id=nid)
    for xv, Pv in loads:
        ss.point_load(node_id=node[float(xv)], Fy=-Pv)
    ss.solve()
    return ss, node, xs


def _ana_node_moments(ss, xs):
    """Bending moment at every node (sagging positive) from anastruct's element end values."""
    out = []
    for i in range(len(xs)):
        if i < len(xs) - 1:
            M = ss.get_element_results(element_id=i + 1, verbose=True)["M"]
            out.append(-float(M[0]))
        else:
            M = ss.get_element_results(element_id=i, verbose=True)["M"]
            out.append(-float(M[-1]))
    return out


def _beam_inputs(E_lo=70e9, E_hi=210e9):
    return lambda rng: {"E": rng.uniform(E_lo, E_hi), "I": _lu(rng, 1e-6, 5e-4)}


_BEAM_NAMES = {"L": "L", "P": "P", "a": "a", "w": "w", "E": "E", "I": "I"}


def _defl_check(E, I):
    _pos("E and I", E, I)


# ================================================================================================ beams: simply supported
def ss_point_load(L, P, a, E, I):
    _pos("L and P", L, P)
    _defl_check(E, I)
    if not 0 < a < L:
        raise ValueError("the load position a must lie strictly between the supports (0 < a < L)")
    b = L - a
    EI = E * I
    RA, RB = P * b / L, P * a / L
    Mmax = P * a * b / L
    d_load = P * a * a * b * b / (3 * EI * L)
    if a >= b:                      # the maximum deflection lies in the longer segment, between A and the load
        xm = math.sqrt((L * L - b * b) / 3)
        dmax = P * b * (L * L - b * b) ** 1.5 / (9 * math.sqrt(3) * EI * L)
    else:
        xm = L - math.sqrt((L * L - a * a) / 3)
        dmax = P * a * (L * L - a * a) ** 1.5 / (9 * math.sqrt(3) * EI * L)
    return _out(RA=RA, RB=RB, M_max=Mmax, V_max=max(RA, RB), deflection_at_load_mm=d_load * 1e3, max_deflection_mm=dmax * 1e3, x_max=xm)


def _integrated_curve(xs, Ms, EI, L):
    """Elastic curve of a simply supported beam by numerical integration (scipy quad) of v'' = -M/EI for a
    piecewise-linear moment diagram through (xs, Ms): returns slope(x) and deflection(x), downward positive."""
    from scipy.integrate import quad
    def M(t):
        for i in range(len(xs) - 1):
            if xs[i] <= t <= xs[i + 1]:
                return Ms[i] + (Ms[i + 1] - Ms[i]) * (t - xs[i]) / (xs[i + 1] - xs[i])
        return 0.0
    q = lambda f, lo, hi: quad(f, lo, hi, points=[v for v in xs if lo < v < hi] or None, epsabs=0, epsrel=1e-13, limit=200)[0]
    thA = q(lambda t: (L - t) * M(t), 0.0, L) / (L * EI)
    slope = lambda x: thA - q(M, 0.0, x) / EI
    defl = lambda x: thA * x - q(lambda t: (x - t) * M(t), 0.0, x) / EI
    return slope, defl


def ref_ss_point_load(L, P, a, E, I):
    """anastruct (frame model with a node at the load) for the reactions, the moment diagram and the deflection under
    the load; the maximum deflection by numerically integrating anastruct's moment diagram (scipy quad) and finding
    where the slope vanishes (scipy brentq)."""
    from scipy.optimize import brentq
    ss, node, xs = _ana(L, {0.0: "hinged", float(L): "roll"}, [(a, P)], E * I)
    nA, nB, nP = (ss.get_node_results_system(node_id=node[float(v)]) for v in (0.0, L, a))
    Ms = [m / 1e3 for m in _ana_node_moments(ss, xs)]
    slope, defl = _integrated_curve(xs, Ms, E * I, L)
    xr = brentq(slope, 1e-9 * L, L * (1 - 1e-9), xtol=1e-15, rtol=8.9e-16, maxiter=500)
    return {"RA": -nA["Fy"], "RB": -nB["Fy"], "M_max": abs(Ms[1]), "deflection_at_load_mm": abs(nP["uy"]),
            "max_deflection_mm": defl(xr) * 1e3, "x_max": xr}


def _rand_ss_point(rng):
    L = round(rng.uniform(1, 10), 3)
    return {"L": L, "P": rng.uniform(1e3, 1e5), "a": round(L * rng.uniform(0.08, 0.92), 3), **_beam_inputs()(rng)}


_add("simply_supported_beam_point_load",
     name="Simply supported beam with a point load", topic="beams / simply supported beams",
     purpose="Reactions, maximum bending moment and shear, deflection under the load and the maximum deflection with its position, for a simply supported beam carrying one point load.",
     inputs=[_phys("L", "length", "span"), _phys("P", "force", "point load (downward)"), _phys("a", "length", "distance of the load from the left support A"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area of the section")],
     outputs=[_o("RA", "force", "reaction at the left support A"), _o("RB", "force", "reaction at the right support B"),
              _o("M_max", "bending_moment", "maximum bending moment (under the load)"), _o("V_max", "force", "maximum shear force"),
              _o("deflection_at_load_mm", "number", "deflection under the load in mm"), _o("max_deflection_mm", "number", "maximum deflection in mm"),
              _o("x_max", "length", "position of the maximum deflection from A")],
     primary="M_max", conditions=["0 < a < L", "Euler-Bernoulli beam, linear elastic, prismatic"],
     method="RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise)",
     latex=r"M_{max} = \frac{Pab}{L},\quad \delta_{max} = \frac{Pb\,(L^2-b^2)^{3/2}}{9\sqrt3\,EIL}",
     run=ss_point_load,
     parse=lambda q: _pairs(q, r"simply supported beam with (?:a )?point load", _BEAM_NAMES, phys={"L", "P", "a", "E", "I"}),
     reference=ref_ss_point_load,
     wrong=_safe(lambda L, P, a, E, I: _out(RA=P * a / L, RB=P * (L - a) / L, M_max=P * a * (L - a) / (2 * L), V_max=P,
                                          deflection_at_load_mm=P * a * a * (L - a) ** 2 / (6 * E * I * L) * 1e3,
                                          max_deflection_mm=P * L ** 3 / (48 * E * I) * 1e3, x_max=L / 2)),
     random_inputs=_rand_ss_point,
     templates=["Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4"],
     examples=[{"inputs": {"L": 6.0, "P": 20e3, "a": 2.0, "E": 200e9, "I": 8e-5},
                "outputs": {"RA": 20e3 * 4 / 6, "RB": 20e3 * 2 / 6, "M_max": 20e3 * 8 / 6, "deflection_at_load_mm": 20e3 * 4 * 16 / (3 * 200e9 * 8e-5 * 6) * 1e3,
                            "max_deflection_mm": 20e3 * 2 * (36 - 4) ** 1.5 / (9 * math.sqrt(3) * 200e9 * 8e-5 * 6) * 1e3,
                            "x_max": 6 - math.sqrt(32 / 3)}},
               {"inputs": {"L": 4.0, "P": 1000.0, "a": 2.0, "E": 1.0, "I": 1.0}, "outputs": {"M_max": 1000.0, "max_deflection_mm": 1000 * 64 / 48 * 1e3, "x_max": 2.0}}],
     questions=[{"q": "Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4",
                 "outputs": {"RA": 20e3 * 4 / 6, "M_max": 20e3 * 8 / 6}},
                {"q": "Simply supported beam with point load: L = 6 m, P = 20 kN, a = 7 m, E = 200 GPa, I = 8e-5 m^4", "outcome": "INVALID_INPUT"},
                {"q": "Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m", "outcome": "NEEDS_INPUT"}])


def ss_udl(L, w, E, I):
    _pos("L and w", L, w)
    _defl_check(E, I)
    return _out(RA=w * L / 2, RB=w * L / 2, M_max=w * L * L / 8, V_max=w * L / 2,
                max_deflection_mm=5 * w * L ** 4 / (384 * E * I) * 1e3, end_slope_deg=math.degrees(w * L ** 3 / (24 * E * I)))


def ref_ss_udl(L, w, E, I):
    """sympy's Beam solved symbolically; moment and deflection evaluated at midspan, slope at the support."""
    s, r, ev = _beam_eval("ss_udl", L, w=w, E=E, I=I)
    return {"RA": -r["R1"], "RB": -r["R2"], "M_max": abs(ev("M", L / 2)), "V_max": max(abs(r["R1"]), abs(r["R2"])),
            "max_deflection_mm": abs(ev("y", L / 2)) * 1e3, "end_slope_deg": math.degrees(abs(ev("slope", 0.0)))}


_add("simply_supported_beam_udl",
     name="Simply supported beam with a uniformly distributed load", topic="beams / simply supported beams",
     purpose="Reactions, maximum bending moment and shear, maximum (midspan) deflection and end slope of a simply supported beam under a uniform load over its whole span.",
     inputs=[_phys("L", "length", "span"), _phys("w", "distributed_load", "uniform load per unit length (downward)"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area of the section")],
     outputs=[_o("RA", "force", "reaction at A"), _o("RB", "force", "reaction at B"), _o("M_max", "bending_moment", "maximum bending moment (midspan)"),
              _o("V_max", "force", "maximum shear force (at the supports)"), _o("max_deflection_mm", "number", "maximum deflection (midspan) in mm"),
              _o("end_slope_deg", "number", "slope at the supports in degrees")],
     primary="M_max", conditions=["L, w, E, I > 0", "Euler-Bernoulli beam, small deflections"],
     method="R = wL/2, M_max = wL^2/8, delta_max = 5wL^4/(384EI), theta_end = wL^3/(24EI)",
     latex=r"M_{max} = \frac{wL^2}{8},\quad \delta_{max} = \frac{5wL^4}{384EI}",
     run=ss_udl,
     parse=lambda q: _pairs(q, r"simply supported beam with (?:a )?(?:uniform(?:ly distributed)? load|udl)", _BEAM_NAMES, phys={"L", "w", "E", "I"}),
     reference=ref_ss_udl,
     wrong=_safe(lambda L, w, E, I: _out(RA=w * L / 2, RB=w * L / 2, M_max=w * L * L / 12, V_max=w * L / 2,
                                       max_deflection_mm=w * L ** 4 / (384 * E * I) * 1e3, end_slope_deg=math.degrees(w * L ** 3 / (48 * E * I)))),
     random_inputs=lambda rng: {"L": rng.uniform(1, 12), "w": rng.uniform(500, 5e4), **_beam_inputs()(rng)},
     templates=["Simply supported beam with uniform load: L = 5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4"],
     examples=[{"inputs": {"L": 5.0, "w": 10e3, "E": 200e9, "I": 4e-5},
                "outputs": {"RA": 25e3, "M_max": 31250.0, "max_deflection_mm": 5 * 10e3 * 625 / (384 * 200e9 * 4e-5) * 1e3}}],
     questions=[{"q": "Simply supported beam with uniform load: L = 5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4",
                 "outputs": {"RA": 25e3, "M_max": 31250.0, "max_deflection_mm": 5 * 10e3 * 625 / (384 * 200e9 * 4e-5) * 1e3}},
                {"q": "Simply supported beam with UDL: L = 5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4", "outputs": {"M_max": 31250.0}},
                {"q": "Simply supported beam with uniform load: L = -5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4", "outcome": "INVALID_INPUT"}])


def ss_point_loads(L, x, P):
    _pos("the span L", L)
    if len(x) != len(P):
        raise ValueError("x and P must have the same number of entries")
    if any(not (0 <= xi <= L) for xi in x):
        raise ValueError("every load position must lie on the span (0 <= x <= L)")
    if any(not (math.isfinite(p) and p > 0) for p in P):
        raise ValueError("every load must be positive (downward)")
    tot = math.fsum(P)
    RB = math.fsum(p * xi for p, xi in zip(P, x)) / L
    RA = tot - RB
    pts = sorted(set(x))
    best_M, best_x = 0.0, 0.0
    for xv in pts:
        M = RA * xv - math.fsum(p * (xv - xi) for p, xi in zip(P, x) if xi < xv)
        if M > best_M * (1 + 1e-9):
            best_M, best_x = M, xv
    return _out(RA=RA, RB=RB, M_max=best_M, x_Mmax=best_x, V_max=max(abs(RA), abs(RB)))


def ref_ss_point_loads(L, x, P):
    """anastruct: a frame model of the beam with nodes at every load (exact for point loads), reactions from the
    node results and the moment at each node from the element end values (unit EI; statics does not depend on it)."""
    loads = {}
    for xi, p in zip(x, P):
        loads[float(xi)] = loads.get(float(xi), 0.0) + p * 1e3
    ss, node, xs = _ana(L, {0.0: "hinged", float(L): "roll"}, list(loads.items()), EI=1e6)
    RA = -ss.get_node_results_system(node_id=node[0.0])["Fy"] / 1e3
    RB = -ss.get_node_results_system(node_id=node[float(L)])["Fy"] / 1e3
    # a load ON a support goes straight into it; anastruct's node result then holds reaction minus load
    RA += loads.get(0.0, 0.0) / 1e3
    RB += loads.get(float(L), 0.0) / 1e3
    Ms = [m / 1e6 for m in _ana_node_moments(ss, xs)]
    best_M, best_x = 0.0, 0.0
    for xv, M in zip(xs, Ms):
        if xv in loads and M > best_M * (1 + 1e-9):
            best_M, best_x = M, xv
    return {"RA": RA, "RB": RB, "M_max": best_M, "x_Mmax": best_x}


def _rand_ss_loads(rng):
    """Load points at distinct twelfths of the span (1 mm grid): anastruct's solution loses digits when two nodes
    are very close (a very short element makes its stiffness matrix ill-conditioned)."""
    L = round(rng.uniform(2, 12), 2)
    n = rng.randint(1, 5)
    js = rng.sample(range(1, 12), n)
    return {"L": L, "x": [round(L * j / 12, 3) for j in js], "P": [round(rng.uniform(1, 80), 2) for _ in range(n)]}


_add("simply_supported_beam_point_loads",
     name="Simply supported beam with several point loads", topic="beams / simply supported beams",
     purpose="Support reactions, the maximum bending moment and its position, and the maximum shear for a simply supported beam carrying several downward point loads.",
     inputs=[_phys("L", "length", "span"), _vec("x", "load positions from the left support A, in m (a list)"),
             _vec("P", "point loads in kN, downward (a list, same order as x)")],
     outputs=[_o("RA", "number", "reaction at A in kN"), _o("RB", "number", "reaction at B in kN"),
              _o("M_max", "number", "maximum bending moment in kN*m"), _o("x_Mmax", "number", "position of the maximum moment from A, in m (the first one if several are equal)"),
              _o("V_max", "number", "maximum shear force in kN")],
     primary="M_max", conditions=["0 <= x_i <= L", "P_i > 0 (downward)", "x and P of equal length"],
     method="moments about A: RB = sum P_i x_i / L, RA = sum P_i - RB; M(x) = RA x - sum P_i (x - x_i) for x_i < x, largest at a load point",
     latex=r"R_B = \frac{\sum P_i x_i}{L},\quad R_A = \sum P_i - R_B,\quad M(x) = R_A x - \sum_{x_i<x} P_i(x-x_i)",
     run=ss_point_loads,
     parse=lambda q: _pairs(q, r"simply supported beam with (?:several |multiple )?point loads", {"L": "L", "x": "x", "P": "P"}, phys={"L"}, vectors={"x", "P"}),
     reference=ref_ss_point_loads,
     wrong=_safe(lambda L, x, P: (lambda RA: _out(RA=RA, RB=sum(P) - RA, M_max=RA * L / 2, x_Mmax=L / 2, V_max=max(RA, sum(P) - RA)))(
         sum(p * xi for p, xi in zip(P, x)) / L)),
     random_inputs=_rand_ss_loads,
     templates=["Simply supported beam with point loads: L = 8 m, x = [2, 5], P = [10, 20]"],
     examples=[{"inputs": {"L": 8.0, "x": "2, 5", "P": "10, 20"}, "outputs": {"RA": 15.0, "RB": 15.0, "M_max": 45.0, "x_Mmax": 5.0, "V_max": 15.0}},
               {"inputs": {"L": 6.0, "x": "3", "P": "12"}, "outputs": {"RA": 6.0, "RB": 6.0, "M_max": 18.0, "x_Mmax": 3.0}}],
     questions=[{"q": "Simply supported beam with point loads: L = 8 m, x = [2, 5], P = [10, 20]", "outputs": {"RA": 15.0, "RB": 15.0, "M_max": 45.0, "x_Mmax": 5.0}},
                {"q": "Simply supported beam with point loads: L = 8 m, x = [2, 9], P = [10, 20]", "outcome": "INVALID_INPUT"},
                {"q": "Simply supported beam with point loads: L = 8 m, x = [2, 5]", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ beams: cantilevers
def cantilever_point_load(L, P, E, I, a=None):
    _pos("L and P", L, P)
    _defl_check(E, I)
    a = L if a is None else a
    if not 0 < a <= L:
        raise ValueError("the load position a (from the fixed end) must satisfy 0 < a <= L")
    EI = E * I
    d_load = P * a ** 3 / (3 * EI)
    th = P * a * a / (2 * EI)
    d_tip = d_load + th * (L - a)
    return _out(R=P, M_fixed=P * a, deflection_at_load_mm=d_load * 1e3, tip_deflection_mm=d_tip * 1e3, tip_slope_deg=math.degrees(th))


def ref_cantilever_point_load(L, P, E, I, a=None):
    """anastruct: the cantilever as a frame model with nodes at the load and the tip (exact for point loads)."""
    a = L if a is None else a
    ss, node, xs = _ana(L, {0.0: "fixed"}, [(a, P)], E * I)
    n0, nl, nt = (ss.get_node_results_system(node_id=node[float(v)]) for v in (0.0, a, L))
    return {"R": -n0["Fy"], "M_fixed": abs(n0["Tz"]) / 1e3, "deflection_at_load_mm": abs(nl["uy"]), "tip_deflection_mm": abs(nt["uy"]),
            "tip_slope_deg": math.degrees(abs(nt["phi_z"]))}


_add("cantilever_point_load",
     name="Cantilever with a point load", topic="beams / cantilevers",
     purpose="Reaction, fixed-end moment, deflection under the load, tip deflection and tip slope of a cantilever carrying one point load at distance a from the fixed end (default: at the free end).",
     inputs=[_phys("L", "length", "length of the cantilever"), _phys("P", "force", "point load (downward)"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area"),
             _phys("a", "length", "distance of the load from the fixed end (default L: at the free end)", required=False)],
     outputs=[_o("R", "force", "vertical reaction at the fixed end"), _o("M_fixed", "bending_moment", "hogging moment at the fixed end"),
              _o("deflection_at_load_mm", "number", "deflection under the load in mm"), _o("tip_deflection_mm", "number", "deflection of the free end in mm"),
              _o("tip_slope_deg", "number", "slope at the free end in degrees")],
     primary="tip_deflection_mm", conditions=["0 < a <= L", "Euler-Bernoulli beam, small deflections"],
     method="M = Pa, deflection under the load Pa^3/(3EI), slope there Pa^2/(2EI), tip deflection Pa^2(3L - a)/(6EI)",
     latex=r"\delta_{tip} = \frac{Pa^2(3L-a)}{6EI},\quad \theta_{tip} = \frac{Pa^2}{2EI}",
     run=cantilever_point_load,
     parse=lambda q: _pairs(q, r"cantilever with (?:an? )?(?:end |point )load", _BEAM_NAMES, phys={"L", "P", "a", "E", "I"}),
     reference=ref_cantilever_point_load,
     wrong=_safe(lambda L, P, E, I, a=None: (lambda a: _out(R=P, M_fixed=P * a, deflection_at_load_mm=P * a ** 3 / (8 * E * I) * 1e3,
                                                          tip_deflection_mm=P * a ** 3 / (8 * E * I) * 1e3, tip_slope_deg=math.degrees(P * a * a / (6 * E * I))))(L if a is None else a)),
     random_inputs=lambda rng: (lambda L: {"L": L, "P": rng.uniform(100, 2e4), "a": rng.choice([L, round(L * rng.uniform(0.2, 0.95), 3)]), **_beam_inputs()(rng)})(round(rng.uniform(0.5, 5), 3)),
     templates=["Cantilever with end load: L = 2 m, P = 5 kN, E = 200 GPa, I = 8e-6 m^4",
                "Cantilever with point load: L = 3 m, P = 4 kN, a = 2 m, E = 70 GPa, I = 2e-5 m^4"],
     examples=[{"inputs": {"L": 2.0, "P": 5e3, "E": 200e9, "I": 8e-6}, "outputs": {"M_fixed": 1e4, "tip_deflection_mm": 5e3 * 8 / (3 * 200e9 * 8e-6) * 1e3}},
               {"inputs": {"L": 3.0, "P": 4e3, "a": 2.0, "E": 70e9, "I": 2e-5},
                "outputs": {"tip_deflection_mm": 4e3 * 4 * (9 - 2) / (6 * 70e9 * 2e-5) * 1e3, "deflection_at_load_mm": 4e3 * 8 / (3 * 70e9 * 2e-5) * 1e3}}],
     questions=[{"q": "Cantilever with end load: L = 2 m, P = 5 kN, E = 200 GPa, I = 8e-6 m^4", "outputs": {"tip_deflection_mm": 5e3 * 8 / (3 * 200e9 * 8e-6) * 1e3}},
                {"q": "Cantilever with point load: L = 3 m, P = 4 kN, a = 2 m, E = 70 GPa, I = 2e-5 m^4",
                 "outputs": {"tip_deflection_mm": 4e3 * 4 * 7 / (6 * 70e9 * 2e-5) * 1e3}},
                {"q": "Cantilever with point load: L = 3 m, P = 4 kN, a = 4 m, E = 70 GPa, I = 2e-5 m^4", "outcome": "INVALID_INPUT"}])


def cantilever_udl(L, w, E, I):
    _pos("L and w", L, w)
    _defl_check(E, I)
    return _out(R=w * L, M_fixed=w * L * L / 2, tip_deflection_mm=w * L ** 4 / (8 * E * I) * 1e3, tip_slope_deg=math.degrees(w * L ** 3 / (6 * E * I)))


def ref_cantilever_udl(L, w, E, I):
    """sympy's Beam solved symbolically (fixed end: zero deflection and slope)."""
    s, r, ev = _beam_eval("cant_udl", L, w=w, E=E, I=I)
    return {"R": -r["R1"], "M_fixed": abs(r["M1"]), "tip_deflection_mm": abs(ev("y", L)) * 1e3, "tip_slope_deg": math.degrees(abs(ev("slope", L)))}


_add("cantilever_udl",
     name="Cantilever with a uniformly distributed load", topic="beams / cantilevers",
     purpose="Reaction, fixed-end moment, tip deflection and tip slope of a cantilever under a uniform load over its whole length.",
     inputs=[_phys("L", "length", "length of the cantilever"), _phys("w", "distributed_load", "uniform load per unit length (downward)"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area")],
     outputs=[_o("R", "force", "vertical reaction at the fixed end"), _o("M_fixed", "bending_moment", "hogging moment at the fixed end"),
              _o("tip_deflection_mm", "number", "deflection of the free end in mm"), _o("tip_slope_deg", "number", "slope at the free end in degrees")],
     primary="tip_deflection_mm", conditions=["L, w, E, I > 0", "Euler-Bernoulli beam, small deflections"],
     method="R = wL, M = wL^2/2, delta_tip = wL^4/(8EI), theta_tip = wL^3/(6EI)",
     latex=r"\delta_{tip} = \frac{wL^4}{8EI},\quad \theta_{tip} = \frac{wL^3}{6EI}",
     run=cantilever_udl,
     parse=lambda q: _pairs(q, r"cantilever with (?:a )?(?:uniform(?:ly distributed)? load|udl)", _BEAM_NAMES, phys={"L", "w", "E", "I"}),
     reference=ref_cantilever_udl,
     wrong=_safe(lambda L, w, E, I: _out(R=w * L, M_fixed=w * L * L / 2, tip_deflection_mm=w * L ** 4 / (3 * E * I) * 1e3, tip_slope_deg=math.degrees(w * L ** 3 / (2 * E * I)))),
     random_inputs=lambda rng: {"L": rng.uniform(0.5, 6), "w": rng.uniform(100, 3e4), **_beam_inputs()(rng)},
     templates=["Cantilever with uniform load: L = 3 m, w = 2 kN/m, E = 200 GPa, I = 1e-5 m^4"],
     examples=[{"inputs": {"L": 3.0, "w": 2e3, "E": 200e9, "I": 1e-5}, "outputs": {"M_fixed": 9e3, "tip_deflection_mm": 2e3 * 81 / (8 * 200e9 * 1e-5) * 1e3}}],
     questions=[{"q": "Cantilever with uniform load: L = 3 m, w = 2 kN/m, E = 200 GPa, I = 1e-5 m^4", "outputs": {"tip_deflection_mm": 2e3 * 81 / (8 * 200e9 * 1e-5) * 1e3}},
                {"q": "Cantilever with UDL: L = 3 m, w = 2 kN/m, E = 200 GPa, I = 0 m^4", "outcome": "INVALID_INPUT"}])


def cantilever_triangular(L, w, E, I):
    _pos("L and w", L, w)
    _defl_check(E, I)
    return _out(R=w * L / 2, M_fixed=w * L * L / 6, tip_deflection_mm=w * L ** 4 / (30 * E * I) * 1e3, tip_slope_deg=math.degrees(w * L ** 3 / (24 * E * I)))


def ref_cantilever_triangular(L, w, E, I):
    """sympy's Beam solved symbolically with a linearly varying load (w at the fixed end, 0 at the tip)."""
    s, r, ev = _beam_eval("cant_tri", L, w=w, E=E, I=I)
    return {"R": -r["R1"], "M_fixed": abs(r["M1"]), "tip_deflection_mm": abs(ev("y", L)) * 1e3, "tip_slope_deg": math.degrees(abs(ev("slope", L)))}


_add("cantilever_triangular_load",
     name="Cantilever with a triangular (linearly varying) load", topic="beams / cantilevers",
     purpose="Reaction, fixed-end moment, tip deflection and tip slope of a cantilever whose load falls linearly from w at the fixed end to zero at the free end.",
     inputs=[_phys("L", "length", "length of the cantilever"), _phys("w", "distributed_load", "load intensity at the fixed end (downward)"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area")],
     outputs=[_o("R", "force", "vertical reaction at the fixed end"), _o("M_fixed", "bending_moment", "hogging moment at the fixed end"),
              _o("tip_deflection_mm", "number", "deflection of the free end in mm"), _o("tip_slope_deg", "number", "slope at the free end in degrees")],
     primary="tip_deflection_mm", conditions=["L, w, E, I > 0", "maximum intensity at the fixed end", "Euler-Bernoulli beam"],
     method="R = wL/2, M = wL^2/6, delta_tip = wL^4/(30EI), theta_tip = wL^3/(24EI)",
     latex=r"\delta_{tip} = \frac{w_0L^4}{30EI},\quad \theta_{tip} = \frac{w_0L^3}{24EI}",
     run=cantilever_triangular,
     parse=lambda q: _pairs(q, r"cantilever with (?:a )?(?:triangular|linearly varying) load", _BEAM_NAMES, phys={"L", "w", "E", "I"}),
     reference=ref_cantilever_triangular,
     wrong=_safe(lambda L, w, E, I: _out(R=w * L / 2, M_fixed=w * L * L / 3, tip_deflection_mm=w * L ** 4 / (120 * E * I) * 1e3, tip_slope_deg=math.degrees(w * L ** 3 / (6 * E * I)))),
     random_inputs=lambda rng: {"L": rng.uniform(0.5, 6), "w": rng.uniform(100, 3e4), **_beam_inputs()(rng)},
     templates=["Cantilever with triangular load: L = 3 m, w = 6 kN/m, E = 200 GPa, I = 2e-5 m^4"],
     examples=[{"inputs": {"L": 3.0, "w": 6e3, "E": 200e9, "I": 2e-5}, "outputs": {"M_fixed": 9e3, "tip_deflection_mm": 6e3 * 81 / (30 * 200e9 * 2e-5) * 1e3}}],
     questions=[{"q": "Cantilever with triangular load: L = 3 m, w = 6 kN/m, E = 200 GPa, I = 2e-5 m^4", "outputs": {"R": 9e3, "M_fixed": 9e3}},
                {"q": "Cantilever with triangular load: L = 3 m, w = 6 kN/m", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ beams: indeterminate
def ff_udl(L, w, E, I):
    _pos("L and w", L, w)
    _defl_check(E, I)
    return _out(RA=w * L / 2, RB=w * L / 2, M_end=w * L * L / 12, M_mid=w * L * L / 24, max_deflection_mm=w * L ** 4 / (384 * E * I) * 1e3)


def ref_ff_udl(L, w, E, I):
    """sympy's Beam solved symbolically: four unknown reactions from equilibrium and the four boundary conditions."""
    s, r, ev = _beam_eval("ff_udl", L, w=w, E=E, I=I)
    return {"RA": -r["R1"], "RB": -r["R2"], "M_end": abs(r["M1"]), "M_mid": abs(ev("M", L / 2)), "max_deflection_mm": abs(ev("y", L / 2)) * 1e3}


_add("fixed_fixed_beam_udl",
     name="Fixed-fixed (built-in) beam with a uniformly distributed load", topic="beams / statically indeterminate beams",
     purpose="Reactions, fixed-end (hogging) moments, midspan (sagging) moment and maximum deflection of a beam built in at both ends under a uniform load.",
     inputs=[_phys("L", "length", "span"), _phys("w", "distributed_load", "uniform load per unit length (downward)"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area")],
     outputs=[_o("RA", "force", "reaction at A"), _o("RB", "force", "reaction at B"), _o("M_end", "bending_moment", "hogging moment at each fixed end"),
              _o("M_mid", "bending_moment", "sagging moment at midspan"), _o("max_deflection_mm", "number", "maximum (midspan) deflection in mm")],
     primary="M_end", conditions=["L, w, E, I > 0", "both ends fully fixed against rotation and deflection"],
     method="R = wL/2, M_end = wL^2/12, M_mid = wL^2/24, delta_max = wL^4/(384EI)",
     latex=r"M_{end} = \frac{wL^2}{12},\quad M_{mid} = \frac{wL^2}{24},\quad \delta_{max} = \frac{wL^4}{384EI}",
     run=ff_udl,
     parse=lambda q: _pairs(q, r"fixed[- ]fixed beam with (?:a )?(?:uniform(?:ly distributed)? load|udl)|built[- ]in beam with (?:a )?(?:uniform load|udl)", _BEAM_NAMES, phys={"L", "w", "E", "I"}),
     reference=ref_ff_udl,
     wrong=_safe(lambda L, w, E, I: _out(RA=w * L / 2, RB=w * L / 2, M_end=w * L * L / 8, M_mid=w * L * L / 8, max_deflection_mm=5 * w * L ** 4 / (384 * E * I) * 1e3)),
     random_inputs=lambda rng: {"L": rng.uniform(1, 12), "w": rng.uniform(500, 5e4), **_beam_inputs()(rng)},
     templates=["Fixed-fixed beam with uniform load: L = 6 m, w = 12 kN/m, E = 200 GPa, I = 6e-5 m^4"],
     examples=[{"inputs": {"L": 6.0, "w": 12e3, "E": 200e9, "I": 6e-5}, "outputs": {"M_end": 36e3, "M_mid": 18e3, "max_deflection_mm": 12e3 * 1296 / (384 * 200e9 * 6e-5) * 1e3}}],
     questions=[{"q": "Fixed-fixed beam with uniform load: L = 6 m, w = 12 kN/m, E = 200 GPa, I = 6e-5 m^4", "outputs": {"M_end": 36e3, "M_mid": 18e3}},
                {"q": "Built-in beam with UDL: L = 6 m, w = 12 kN/m, E = 200 GPa, I = 6e-5 m^4", "outputs": {"M_end": 36e3}}])


def ff_point(L, P, a, E, I):
    _pos("L and P", L, P)
    _defl_check(E, I)
    if not 0 < a < L:
        raise ValueError("the load position a must lie strictly between the ends (0 < a < L)")
    b = L - a
    L2, L3 = L * L, L ** 3
    return _out(RA=P * b * b * (3 * a + b) / L3, RB=P * a * a * (a + 3 * b) / L3, MA=P * a * b * b / L2, MB=P * a * a * b / L2,
                M_load=2 * P * a * a * b * b / L3, deflection_at_load_mm=P * a ** 3 * b ** 3 / (3 * E * I * L3) * 1e3)


def ref_ff_point(L, P, a, E, I):
    """anastruct: a frame model, both ends fixed, a node at the load (exact for point loads)."""
    ss, node, xs = _ana(L, {0.0: "fixed", float(L): "fixed"}, [(a, P)], E * I)
    nA, nB, nP = (ss.get_node_results_system(node_id=node[float(v)]) for v in (0.0, L, a))
    return {"RA": -nA["Fy"], "RB": -nB["Fy"], "MA": abs(nA["Tz"]) / 1e3, "MB": abs(nB["Tz"]) / 1e3, "M_load": abs(_ana_node_moments(ss, xs)[1]) / 1e3,
            "deflection_at_load_mm": abs(nP["uy"])}


_add("fixed_fixed_beam_point_load",
     name="Fixed-fixed (built-in) beam with a point load", topic="beams / statically indeterminate beams",
     purpose="Reactions, fixed-end moments, the moment under the load and the deflection under the load of a beam built in at both ends carrying one point load.",
     inputs=[_phys("L", "length", "span"), _phys("P", "force", "point load (downward)"), _phys("a", "length", "distance of the load from end A"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area")],
     outputs=[_o("RA", "force", "reaction at A"), _o("RB", "force", "reaction at B"), _o("MA", "bending_moment", "hogging moment at A"),
              _o("MB", "bending_moment", "hogging moment at B"), _o("M_load", "bending_moment", "sagging moment under the load"),
              _o("deflection_at_load_mm", "number", "deflection under the load in mm")],
     primary="MA", conditions=["0 < a < L", "both ends fully fixed"],
     method="MA = Pab^2/L^2, MB = Pa^2b/L^2, RA = Pb^2(3a + b)/L^3, RB = Pa^2(a + 3b)/L^3, M_load = 2Pa^2b^2/L^3, delta_load = Pa^3b^3/(3EIL^3)",
     latex=r"M_A = \frac{Pab^2}{L^2},\quad M_B = \frac{Pa^2b}{L^2},\quad \delta_P = \frac{Pa^3b^3}{3EIL^3}",
     run=ff_point,
     parse=lambda q: _pairs(q, r"fixed[- ]fixed beam with (?:a )?point load|built[- ]in beam with (?:a )?point load", _BEAM_NAMES, phys={"L", "P", "a", "E", "I"}),
     reference=ref_ff_point,
     wrong=_safe(lambda L, P, a, E, I: _out(RA=P * (L - a) / L, RB=P * a / L, MA=P * a * a * (L - a) / L ** 2, MB=P * a * (L - a) ** 2 / L ** 2,
                                          M_load=P * a * (L - a) / L, deflection_at_load_mm=P * a ** 3 * (L - a) ** 3 / (3 * E * I * L ** 3) * 1e3)),
     random_inputs=lambda rng: (lambda L: {"L": L, "P": rng.uniform(1e3, 1e5), "a": round(L * rng.uniform(0.1, 0.9), 3), **_beam_inputs()(rng)})(round(rng.uniform(1, 10), 3)),
     templates=["Fixed-fixed beam with point load: L = 6 m, P = 30 kN, a = 2 m, E = 200 GPa, I = 5e-5 m^4"],
     examples=[{"inputs": {"L": 6.0, "P": 30e3, "a": 2.0, "E": 200e9, "I": 5e-5},
                "outputs": {"MA": 30e3 * 2 * 16 / 36, "MB": 30e3 * 4 * 4 / 36, "RA": 30e3 * 16 * 10 / 216, "M_load": 2 * 30e3 * 4 * 16 / 216,
                            "deflection_at_load_mm": 30e3 * 8 * 64 / (3 * 200e9 * 5e-5 * 216) * 1e3}}],
     questions=[{"q": "Fixed-fixed beam with point load: L = 6 m, P = 30 kN, a = 2 m, E = 200 GPa, I = 5e-5 m^4", "outputs": {"MA": 30e3 * 32 / 36, "MB": 30e3 * 16 / 36}},
                {"q": "Fixed-fixed beam with point load: L = 6 m, P = 30 kN, a = 0 m, E = 200 GPa, I = 5e-5 m^4", "outcome": "INVALID_INPUT"}])


def pc_udl(L, w, E, I):
    _pos("L and w", L, w)
    _defl_check(E, I)
    xm = L * (15 - math.sqrt(33)) / 16
    dmax = (39 + 55 * math.sqrt(33)) / 65536 * w * L ** 4 / (E * I)
    return _out(RA=5 * w * L / 8, RB=3 * w * L / 8, MA=w * L * L / 8, M_span=9 * w * L * L / 128, x_M_span=5 * L / 8,
                max_deflection_mm=dmax * 1e3, x_max=xm)


def ref_pc_udl(L, w, E, I):
    """sympy's Beam solved symbolically (fixed at A, roller at B); the span moment's maximum where sympy's shear force
    vanishes and the largest deflection where its slope vanishes (scipy brentq)."""
    from scipy.optimize import brentq
    s, r, ev = _beam_eval("pc_udl", L, w=w, E=E, I=I)
    xv = brentq(lambda t: ev("V", t), 0.3 * L, 0.9 * L, xtol=1e-15, rtol=8.9e-16)
    dmax, xm = _max_deflection_numeric("pc_udl", L, 0.0, 0.0, w, E, I, lo=0.05 * L, hi=0.95 * L)
    return {"RA": -r["R1"], "RB": -r["R2"], "MA": abs(r["M1"]), "M_span": abs(ev("M", xv)), "x_M_span": xv, "max_deflection_mm": dmax * 1e3, "x_max": xm}


_add("propped_cantilever_udl",
     name="Propped cantilever with a uniformly distributed load", topic="beams / statically indeterminate beams",
     purpose="Reactions, fixed-end moment, maximum span moment and its position, and the maximum deflection with its position, for a beam fixed at A and simply supported at B under a uniform load.",
     inputs=[_phys("L", "length", "span"), _phys("w", "distributed_load", "uniform load per unit length (downward)"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area")],
     outputs=[_o("RA", "force", "reaction at the fixed end A"), _o("RB", "force", "reaction at the prop B"), _o("MA", "bending_moment", "hogging moment at A"),
              _o("M_span", "bending_moment", "largest sagging moment in the span"), _o("x_M_span", "length", "its position from A"),
              _o("max_deflection_mm", "number", "maximum deflection in mm"), _o("x_max", "length", "position of the maximum deflection from A")],
     primary="MA", conditions=["L, w, E, I > 0", "A fully fixed, B a simple support"],
     method="RB = 3wL/8, RA = 5wL/8, MA = wL^2/8, M_span = 9wL^2/128 at 5L/8, delta_max = (39 + 55 sqrt(33)) wL^4/(65536 EI) at x = L(15 - sqrt(33))/16",
     latex=r"R_B = \frac{3wL}{8},\quad M_A = \frac{wL^2}{8},\quad \delta_{max} \approx \frac{wL^4}{185EI}",
     run=pc_udl,
     parse=lambda q: _pairs(q, r"propped cantilever with (?:a )?(?:uniform(?:ly distributed)? load|udl)", _BEAM_NAMES, phys={"L", "w", "E", "I"}),
     reference=ref_pc_udl,
     wrong=_safe(lambda L, w, E, I: _out(RA=w * L / 2, RB=w * L / 2, MA=w * L * L / 12, M_span=w * L * L / 8, x_M_span=L / 2,
                                       max_deflection_mm=w * L ** 4 / (185 * E * I) * 1e3, x_max=0.4215 * L)),
     random_inputs=lambda rng: {"L": rng.uniform(1, 12), "w": rng.uniform(500, 5e4), **_beam_inputs()(rng)},
     templates=["Propped cantilever with uniform load: L = 8 m, w = 10 kN/m, E = 200 GPa, I = 1e-4 m^4"],
     examples=[{"inputs": {"L": 8.0, "w": 10e3, "E": 200e9, "I": 1e-4},
                "outputs": {"RB": 30e3, "RA": 50e3, "MA": 80e3, "M_span": 45e3, "x_M_span": 5.0,
                            "max_deflection_mm": (39 + 55 * math.sqrt(33)) / 65536 * 10e3 * 4096 / (200e9 * 1e-4) * 1e3}}],
     questions=[{"q": "Propped cantilever with uniform load: L = 8 m, w = 10 kN/m, E = 200 GPa, I = 1e-4 m^4", "outputs": {"RB": 30e3, "MA": 80e3, "M_span": 45e3}}])


def pc_point(L, P, a, E, I):
    _pos("L and P", L, P)
    _defl_check(E, I)
    if not 0 < a < L:
        raise ValueError("the load position a must lie strictly between the fixed end and the prop (0 < a < L)")
    b = L - a
    RB = P * a * a * (3 * L - a) / (2 * L ** 3)
    RA = P - RB
    MA = P * a * b * (L + b) / (2 * L * L)
    Mload = RB * b
    EI = E * I
    d = P * a ** 3 * b * b * (3 * L + b) / (12 * EI * L ** 3)
    return _out(RA=RA, RB=RB, MA=MA, M_load=Mload, deflection_at_load_mm=d * 1e3)


def ref_pc_point(L, P, a, E, I):
    """anastruct: a frame model, fixed at A and on a roller at B, with a node at the load."""
    ss, node, xs = _ana(L, {0.0: "fixed", float(L): "roll"}, [(a, P)], E * I)
    nA, nB, nP = (ss.get_node_results_system(node_id=node[float(v)]) for v in (0.0, L, a))
    return {"RA": -nA["Fy"], "RB": -nB["Fy"], "MA": abs(nA["Tz"]) / 1e3, "M_load": abs(_ana_node_moments(ss, xs)[1]) / 1e3, "deflection_at_load_mm": abs(nP["uy"])}


_add("propped_cantilever_point_load",
     name="Propped cantilever with a point load", topic="beams / statically indeterminate beams",
     purpose="Reactions, fixed-end moment, moment under the load and deflection under the load of a beam fixed at A and simply supported at B with one point load.",
     inputs=[_phys("L", "length", "span"), _phys("P", "force", "point load (downward)"), _phys("a", "length", "distance of the load from the fixed end A"),
             _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area")],
     outputs=[_o("RA", "force", "reaction at the fixed end A"), _o("RB", "force", "reaction at the prop B"), _o("MA", "bending_moment", "hogging moment at A"),
              _o("M_load", "bending_moment", "sagging moment under the load"), _o("deflection_at_load_mm", "number", "deflection under the load in mm")],
     primary="RB", conditions=["0 < a < L", "A fully fixed, B a simple support"],
     method="RB = Pa^2(3L - a)/(2L^3), RA = P - RB, MA = Pab(L + b)/(2L^2), M_load = RB b, delta_load = Pa^3b^2(3L + b)/(12EIL^3)",
     latex=r"R_B = \frac{Pa^2(3L-a)}{2L^3},\quad M_A = \frac{Pab(L+b)}{2L^2}",
     run=pc_point,
     parse=lambda q: _pairs(q, r"propped cantilever with (?:a )?point load", _BEAM_NAMES, phys={"L", "P", "a", "E", "I"}),
     reference=ref_pc_point,
     wrong=_safe(lambda L, P, a, E, I: (lambda RB: _out(RA=P - RB, RB=RB, MA=P * a * (L - a) / L, M_load=RB * (L - a),
                                                      deflection_at_load_mm=P * a * a * (L - a) ** 2 / (3 * E * I * L) * 1e3))(P * a / L)),
     random_inputs=lambda rng: (lambda L: {"L": L, "P": rng.uniform(1e3, 1e5), "a": round(L * rng.uniform(0.1, 0.9), 3), **_beam_inputs()(rng)})(round(rng.uniform(1, 10), 3)),
     templates=["Propped cantilever with point load: L = 4 m, P = 16 kN, a = 2 m, E = 200 GPa, I = 2e-5 m^4"],
     examples=[{"inputs": {"L": 4.0, "P": 16e3, "a": 2.0, "E": 200e9, "I": 2e-5},
                "outputs": {"RB": 5e3, "RA": 11e3, "MA": 12e3, "M_load": 10e3, "deflection_at_load_mm": 7 * 16e3 * 64 / (768 * 200e9 * 2e-5) * 1e3}}],
     questions=[{"q": "Propped cantilever with point load: L = 4 m, P = 16 kN, a = 2 m, E = 200 GPa, I = 2e-5 m^4", "outputs": {"RB": 5e3, "MA": 12e3}}])


# ================================================================================================ section properties
def _rect_set(rects):
    """A, centroid, centroidal Ix, Iy, Ixy of a set of rectangles (b, h, xc, yc); a negative b*h is a hole."""
    A = math.fsum(s * b * h for s, b, h, _x, _y in rects)
    if A <= 0:
        raise ValueError("the section has no positive area")
    xc = math.fsum(s * b * h * x for s, b, h, x, _y in rects) / A
    yc = math.fsum(s * b * h * y for s, b, h, _x, y in rects) / A
    Ix = math.fsum(s * (b * h ** 3 / 12 + b * h * (y - yc) ** 2) for s, b, h, _x, y in rects)
    Iy = math.fsum(s * (h * b ** 3 / 12 + b * h * (x - xc) ** 2) for s, b, h, x, _y in rects)
    Ixy = math.fsum(s * b * h * (x - xc) * (y - yc) for s, b, h, x, y in rects)
    return A, xc, yc, Ix, Iy, Ixy


def _principal(Ix, Iy, Ixy):
    c, rr = (Ix + Iy) / 2, math.hypot((Ix - Iy) / 2, Ixy)
    if rr <= 1e-12 * c:
        return c, c, 0.0
    phi = 0.5 * math.degrees(math.atan2(-2 * Ixy, Ix - Iy))
    if phi <= -90:
        phi += 180
    return c + rr, c - rr, phi


def _sp(poly_pts, holes=(), plastic=False):
    """sectionproperties: the section meshed (6-node triangles, exact for straight-edged regions) and its
    geometric (and plastic) properties computed by the library."""
    from shapely.geometry import Polygon
    from sectionproperties.analysis.section import Section
    from sectionproperties.pre.geometry import Geometry
    g = Geometry(Polygon(poly_pts, holes=[list(h) for h in holes]))
    g.create_mesh(mesh_sizes=[0])
    s = Section(geometry=g)
    s.calculate_geometric_properties()
    if plastic:
        s.calculate_plastic_properties()
    return s


def _sp_zp(poly_pts, holes, axis, at):
    """Plastic section modulus of a section whose plastic neutral axis is its axis of symmetry: sectionproperties
    splits the geometry along that axis and analyses each half; Zp = sum of A_half * |centroid_half - axis|."""
    from shapely.geometry import Polygon
    from sectionproperties.analysis.section import Section
    from sectionproperties.pre.geometry import Geometry
    g = Geometry(Polygon(poly_pts, holes=[list(h) for h in holes]))
    p1, p2 = ((0.0, at), (1.0, at)) if axis == "x" else ((at, 0.0), (at, 1.0))
    halves = g.split_section(point_i=p1, point_j=p2)
    zp = 0.0
    for side in halves:
        for gi in side:
            gi.create_mesh(mesh_sizes=[0])
            sec = Section(geometry=gi)
            sec.calculate_geometric_properties()
            cx, cy = sec.get_c()
            zp += sec.get_area() * abs((cy if axis == "x" else cx) - at)
    return zp


def _sp_props(s, ymax_top=None):
    A = s.get_area()
    cx, cy = s.get_c()
    ixx, iyy, ixy = s.get_ic()
    return A, cx, cy, ixx, iyy, ixy


def rect_section(b, h):
    _pos("b and h", b, h)
    A = b * h
    Ix, Iy = b * h ** 3 / 12, h * b ** 3 / 12
    return _out(A=A, Ix=Ix, Iy=Iy, Zx=Ix / (h / 2), Zy=Iy / (b / 2), rx=math.sqrt(Ix / A), ry=math.sqrt(Iy / A), J_polar=Ix + Iy)


def ref_rect_section(b, h):
    """sectionproperties' geometric analysis of the rectangle (the library's section analysis)."""
    s = _sp([(0, 0), (b, 0), (b, h), (0, h)])
    zx = s.get_z()
    rx, ry = s.get_rc()
    ixx, iyy, _ = s.get_ic()
    return {"A": s.get_area(), "Ix": ixx, "Iy": iyy, "Zx": min(zx[0], zx[1]), "Zy": min(zx[2], zx[3]), "rx": rx, "ry": ry, "J_polar": ixx + iyy}


_SECT_OUT = lambda: [_o("A", "area", "cross-sectional area"), _o("Ix", "second_moment_of_area", "second moment of area about the horizontal centroidal axis"),
                     _o("Iy", "second_moment_of_area", "second moment of area about the vertical centroidal axis"),
                     _o("Zx", "section_modulus", "elastic section modulus about x (Ix / distance to the extreme fibre)"),
                     _o("Zy", "section_modulus", "elastic section modulus about y"), _o("rx", "length", "radius of gyration about x"),
                     _o("ry", "length", "radius of gyration about y")]

_add("rectangular_section_properties",
     name="Section properties of a solid rectangle", topic="section properties / standard shapes",
     purpose="Area, second moments of area, elastic section moduli, radii of gyration and polar moment of a solid rectangle b wide and h deep.",
     inputs=[_phys("b", "length", "width (horizontal)"), _phys("h", "length", "depth (vertical)")],
     outputs=_SECT_OUT() + [_o("J_polar", "second_moment_of_area", "polar second moment about the centroid (Ix + Iy; not the torsion constant)")],
     primary="Ix", conditions=["b, h > 0"],
     method="A = bh, Ix = bh^3/12, Iy = hb^3/12, Zx = bh^2/6, Zy = hb^2/6, r = sqrt(I/A)",
     latex=r"I_x = \frac{bh^3}{12},\quad Z_x = \frac{bh^2}{6},\quad r_x = \sqrt{I_x/A}",
     run=rect_section,
     parse=lambda q: _pairs(q, r"rectangular section(?: properties)?|section properties of a rectangle", {"b": "b", "h": "h"}, phys={"b", "h"}),
     reference=ref_rect_section,
     wrong=_safe(lambda b, h: _out(A=b * h, Ix=b * h ** 3 / 3, Iy=h * b ** 3 / 3, Zx=b * h * h / 3, Zy=h * b * b / 3, rx=h / math.sqrt(3), ry=b / math.sqrt(3), J_polar=b * h * (b * b + h * h) / 3)),
     random_inputs=lambda rng: {"b": _lu(rng, 0.005, 0.5), "h": _lu(rng, 0.005, 1.0)},
     templates=["Rectangular section: b = 100 mm, h = 200 mm"],
     examples=[{"inputs": {"b": 0.1, "h": 0.2}, "outputs": {"A": 0.02, "Ix": 0.1 * 0.008 / 12, "Zx": 0.1 * 0.04 / 6, "ry": 0.1 / math.sqrt(12)}}],
     questions=[{"q": "Rectangular section: b = 100 mm, h = 200 mm", "outputs": {"Ix": 0.1 * 0.008 / 12}},
                {"q": "Rectangular section: b = 100 mm, h = 0 mm", "outcome": "INVALID_INPUT"}])


def _quad_alg(f, lo, hi, p):
    from scipy.integrate import quad
    return quad(f, lo, hi, weight="alg", wvar=(p, p), epsabs=0, epsrel=1e-13, limit=200)[0]


def circle_section(d):
    _pos("the diameter d", d)
    A = math.pi * d * d / 4
    I = math.pi * d ** 4 / 64
    return _out(A=A, I=I, J=2 * I, Z=I / (d / 2), Zp=d ** 3 / 6, r=d / 4)


def ref_circle_section(d):
    """Area and second moments by numerical quadrature over the disk (scipy QUADPACK with the algebraic end weight
    of the circle's chord, (R - y)^(1/2) (R + y)^(1/2)); the plastic modulus as the first moment of both halves."""
    R = d / 2
    A = _quad_alg(lambda y: 2.0, -R, R, 0.5)
    I = _quad_alg(lambda y: 2.0 * y * y, -R, R, 0.5)
    J = I + _quad_alg(lambda y: 2.0 / 3.0, -R, R, 1.5)                 # integral of x^2 over the disk
    from scipy.integrate import quad
    Q = quad(lambda y: 2 * y * math.sqrt(R + y), 0.0, R, weight="alg", wvar=(0.0, 0.5), epsabs=0, epsrel=1e-13, limit=200)[0]
    return {"A": A, "I": I, "J": J, "Z": I / R, "Zp": 2 * Q, "r": math.sqrt(I / A)}


_add("circular_section_properties",
     name="Section properties of a solid circle", topic="section properties / standard shapes",
     purpose="Area, second moment of area, polar moment, elastic and plastic section moduli and radius of gyration of a solid circular section.",
     inputs=[_phys("d", "length", "diameter")],
     outputs=[_o("A", "area", "area"), _o("I", "second_moment_of_area", "second moment of area about a diameter"),
              _o("J", "second_moment_of_area", "polar second moment of area (torsion constant of a circle)"),
              _o("Z", "section_modulus", "elastic section modulus"), _o("Zp", "section_modulus", "plastic section modulus"), _o("r", "length", "radius of gyration")],
     primary="I", conditions=["d > 0"],
     method="A = pi d^2/4, I = pi d^4/64, J = pi d^4/32, Z = pi d^3/32, Zp = d^3/6, r = d/4",
     latex=r"I = \frac{\pi d^4}{64},\quad J = \frac{\pi d^4}{32},\quad Z = \frac{\pi d^3}{32}",
     run=circle_section,
     parse=lambda q: _pairs(q, r"circular section(?: properties)?|solid circular section", {"d": "d", "D": "d"}, phys={"d"}),
     reference=ref_circle_section,
     wrong=_safe(lambda d: _out(A=math.pi * d * d / 4, I=math.pi * d ** 4 / 32, J=math.pi * d ** 4 / 16, Z=math.pi * d ** 3 / 16, Zp=d ** 3 / 3, r=d / 2)),
     random_inputs=lambda rng: {"d": _lu(rng, 0.002, 1.0)},
     templates=["Circular section: d = 50 mm"],
     examples=[{"inputs": {"d": 0.05}, "outputs": {"I": math.pi * 0.05 ** 4 / 64, "J": math.pi * 0.05 ** 4 / 32, "Zp": 0.05 ** 3 / 6}}],
     questions=[{"q": "Circular section: d = 50 mm", "outputs": {"I": math.pi * 0.05 ** 4 / 64}}])


def tube_section(D, d):
    _pos("the outer diameter D", D)
    _nonneg("the inner diameter d", d)
    if d >= D:
        raise ValueError("the inner diameter must be smaller than the outer diameter")
    A = math.pi * (D * D - d * d) / 4
    I = math.pi * (D ** 4 - d ** 4) / 64
    return _out(A=A, I=I, J=2 * I, Z=I / (D / 2), Zp=(D ** 3 - d ** 3) / 6, r=math.sqrt(D * D + d * d) / 4)


def ref_tube_section(D, d):
    """The outer disk minus the inner one, each integrated by scipy QUADPACK with the chord's algebraic weight."""
    def disk(R):
        if R == 0:
            return 0.0, 0.0, 0.0
        return _quad_alg(lambda y: 2.0, -R, R, 0.5), _quad_alg(lambda y: 2.0 * y * y, -R, R, 0.5), _quad_alg(lambda y: 2.0 / 3.0, -R, R, 1.5)
    from scipy.integrate import quad
    (Ao, Io, Xo), (Ai, Ii, Xi) = disk(D / 2), disk(d / 2)
    A, I = Ao - Ai, Io - Ii
    half = lambda R: quad(lambda y: 2 * y * math.sqrt(R + y), 0.0, R, weight="alg", wvar=(0.0, 0.5), epsabs=0, epsrel=1e-13, limit=200)[0] if R else 0.0
    return {"A": A, "I": I, "J": I + (Xo - Xi), "Z": I / (D / 2), "Zp": 2 * (half(D / 2) - half(d / 2)), "r": math.sqrt(I / A)}


_add("hollow_circular_section_properties",
     name="Section properties of a hollow circle (tube)", topic="section properties / standard shapes",
     purpose="Area, second moment of area, polar moment, elastic and plastic section moduli and radius of gyration of a circular tube.",
     inputs=[_phys("D", "length", "outer diameter"), _phys("d", "length", "inner diameter")],
     outputs=[_o("A", "area", "area"), _o("I", "second_moment_of_area", "second moment of area about a diameter"),
              _o("J", "second_moment_of_area", "polar second moment of area (torsion constant)"), _o("Z", "section_modulus", "elastic section modulus"),
              _o("Zp", "section_modulus", "plastic section modulus"), _o("r", "length", "radius of gyration")],
     primary="I", conditions=["D > d >= 0"],
     method="A = pi(D^2 - d^2)/4, I = pi(D^4 - d^4)/64, J = 2I, Z = 2I/D, Zp = (D^3 - d^3)/6, r = sqrt(D^2 + d^2)/4",
     latex=r"I = \frac{\pi(D^4-d^4)}{64},\quad J = \frac{\pi(D^4-d^4)}{32}",
     run=tube_section,
     parse=lambda q: _pairs(q, r"hollow circular section(?: properties)?|tube section(?: properties)?|circular tube section", {"D": "D", "d": "d"}, phys={"D", "d"}),
     reference=ref_tube_section,
     wrong=_safe(lambda D, d: _out(A=math.pi * (D - d) ** 2 / 4, I=math.pi * (D - d) ** 4 / 64, J=math.pi * (D - d) ** 4 / 32, Z=math.pi * (D - d) ** 4 / (32 * D),
                                 Zp=(D - d) ** 3 / 6, r=(D - d) / 4)),
     random_inputs=lambda rng: (lambda D: {"D": D, "d": D * rng.uniform(0.1, 0.95)})(_lu(rng, 0.005, 1.0)),
     templates=["Hollow circular section: D = 60 mm, d = 40 mm"],
     examples=[{"inputs": {"D": 0.06, "d": 0.04}, "outputs": {"I": math.pi * (0.06 ** 4 - 0.04 ** 4) / 64, "Zp": (0.06 ** 3 - 0.04 ** 3) / 6}}],
     questions=[{"q": "Hollow circular section: D = 60 mm, d = 40 mm", "outputs": {"I": math.pi * (0.06 ** 4 - 0.04 ** 4) / 64}},
                {"q": "Hollow circular section: D = 40 mm, d = 60 mm", "outcome": "INVALID_INPUT"}])


def box_section(B, H, t):
    _pos("B, H and t", B, H, t)
    if 2 * t >= min(B, H):
        raise ValueError("the wall thickness must be less than half the smaller outer dimension")
    b, h = B - 2 * t, H - 2 * t
    A = B * H - b * h
    Ix = (B * H ** 3 - b * h ** 3) / 12
    Iy = (H * B ** 3 - h * b ** 3) / 12
    Zpx = (B * H * H - b * h * h) / 4
    return _out(A=A, Ix=Ix, Iy=Iy, Zx=Ix / (H / 2), Zy=Iy / (B / 2), rx=math.sqrt(Ix / A), ry=math.sqrt(Iy / A), Zpx=Zpx)


def ref_box_section(B, H, t):
    """sectionproperties: the outer rectangle with the inner one as a hole (elastic and plastic analysis)."""
    outer, inner = [(0, 0), (B, 0), (B, H), (0, H)], [(t, t), (B - t, t), (B - t, H - t), (t, H - t)]
    s = _sp(outer, holes=[inner])
    z = s.get_z()
    ixx, iyy, _ = s.get_ic()
    rx, ry = s.get_rc()
    return {"A": s.get_area(), "Ix": ixx, "Iy": iyy, "Zx": min(z[0], z[1]), "Zy": min(z[2], z[3]), "rx": rx, "ry": ry,
            "Zpx": _sp_zp(outer, [inner], "x", H / 2)}


_add("hollow_rectangular_section_properties",
     name="Section properties of a hollow rectangle (box)", topic="section properties / standard shapes",
     purpose="Area, second moments, elastic section moduli, radii of gyration and the plastic modulus about x of a rectangular hollow section of uniform wall thickness.",
     inputs=[_phys("B", "length", "outer width"), _phys("H", "length", "outer depth"), _phys("t", "length", "wall thickness")],
     outputs=_SECT_OUT() + [_o("Zpx", "section_modulus", "plastic section modulus about x")],
     primary="Ix", conditions=["B, H, t > 0", "2t < min(B, H)", "square corners (no corner radii)"],
     method="outer rectangle minus inner rectangle: Ix = (BH^3 - bh^3)/12 with b = B - 2t, h = H - 2t; Zpx = (BH^2 - bh^2)/4",
     latex=r"I_x = \frac{BH^3 - bh^3}{12},\quad Z_{p,x} = \frac{BH^2 - bh^2}{4}",
     run=box_section,
     parse=lambda q: _pairs(q, r"hollow rectangular section(?: properties)?|box section(?: properties)?", {"B": "B", "H": "H", "t": "t"}, phys={"B", "H", "t"}),
     reference=ref_box_section,
     wrong=_safe(lambda B, H, t: (lambda b, h: _out(A=B * H - b * h, Ix=(B * H ** 3 - b * h ** 3) / 3, Iy=(H * B ** 3 - h * b ** 3) / 3, Zx=(B * H ** 3 - b * h ** 3) / (3 * H),
                                                  Zy=(H * B ** 3 - h * b ** 3) / (3 * B), rx=H / 2, ry=B / 2, Zpx=(B * H * H - b * h * h) / 6))(B - t, H - t)),
     random_inputs=lambda rng: (lambda B, H: {"B": B, "H": H, "t": min(B, H) * rng.uniform(0.02, 0.4)})(_lu(rng, 0.02, 0.5), _lu(rng, 0.02, 0.8)),
     templates=["Box section: B = 100 mm, H = 200 mm, t = 10 mm"],
     examples=[{"inputs": {"B": 0.1, "H": 0.2, "t": 0.01}, "outputs": {"A": 0.1 * 0.2 - 0.08 * 0.18, "Ix": (0.1 * 0.008 - 0.08 * 0.18 ** 3) / 12}}],
     questions=[{"q": "Box section: B = 100 mm, H = 200 mm, t = 10 mm", "outputs": {"Ix": (0.1 * 0.008 - 0.08 * 0.18 ** 3) / 12}},
                {"q": "Box section: B = 100 mm, H = 200 mm, t = 60 mm", "outcome": "INVALID_INPUT"}])


def _i_check(b, h, tf, tw):
    _pos("b, h, tf and tw", b, h, tf, tw)
    if 2 * tf >= h:
        raise ValueError("the flanges (2 tf) must be thinner than the overall depth h")
    if tw >= b:
        raise ValueError("the web thickness must be smaller than the flange width")


def i_section(b, h, tf, tw):
    _i_check(b, h, tf, tw)
    hw = h - 2 * tf
    rects = [(1, b, tf, 0.0, (h - tf) / 2), (1, b, tf, 0.0, -(h - tf) / 2), (1, tw, hw, 0.0, 0.0)]
    A, _xc, _yc, Ix, Iy, _ = _rect_set(rects)
    Zx, Zy = Ix / (h / 2), Iy / (b / 2)
    Zpx = b * tf * (h - tf) + tw * hw * hw / 4
    Zpy = tf * b * b / 2 + hw * tw * tw / 4
    return _out(A=A, Ix=Ix, Iy=Iy, Zx=Zx, Zy=Zy, rx=math.sqrt(Ix / A), ry=math.sqrt(Iy / A), Zpx=Zpx, Zpy=Zpy, shape_factor_x=Zpx / Zx)


def ref_i_section(b, h, tf, tw):
    """sectionproperties: the I outline as one polygon, elastic and plastic analysis by the library."""
    x0, x1 = (b - tw) / 2, (b + tw) / 2
    pts = [(0, 0), (b, 0), (b, tf), (x1, tf), (x1, h - tf), (b, h - tf), (b, h), (0, h), (0, h - tf), (x0, h - tf), (x0, tf), (0, tf)]
    s = _sp(pts)
    z = s.get_z()
    ixx, iyy, _ = s.get_ic()
    rx, ry = s.get_rc()
    sxx, syy = _sp_zp(pts, [], "x", h / 2), _sp_zp(pts, [], "y", b / 2)
    Zx = min(z[0], z[1])
    return {"A": s.get_area(), "Ix": ixx, "Iy": iyy, "Zx": Zx, "Zy": min(z[2], z[3]), "rx": rx, "ry": ry, "Zpx": sxx, "Zpy": syy, "shape_factor_x": sxx / Zx}


def _rand_i(rng):
    h = _lu(rng, 0.08, 1.0)
    b = h * rng.uniform(0.3, 1.0)
    return {"b": b, "h": h, "tf": h * rng.uniform(0.02, 0.12), "tw": b * rng.uniform(0.03, 0.3)}


_add("i_section_properties",
     name="Section properties of a symmetric I-section", topic="section properties / standard shapes",
     purpose="Area, second moments, elastic and plastic section moduli, radii of gyration and the shape factor of a doubly symmetric I-section (flanges b x tf, web tw, overall depth h, no fillets).",
     inputs=[_phys("b", "length", "flange width"), _phys("h", "length", "overall depth"), _phys("tf", "length", "flange thickness"), _phys("tw", "length", "web thickness")],
     outputs=_SECT_OUT() + [_o("Zpx", "section_modulus", "plastic section modulus about x"), _o("Zpy", "section_modulus", "plastic section modulus about y"),
                            _o("shape_factor_x", "number", "shape factor Zpx / Zx")],
     primary="Ix", conditions=["2 tf < h", "tw < b", "no root fillets"],
     method="three rectangles with the parallel-axis theorem: Ix = sum(bh^3/12 + A d^2); plastic moduli about the axes of symmetry: Zpx = b tf (h - tf) + tw (h - 2tf)^2/4, Zpy = tf b^2/2 + (h - 2tf) tw^2/4",
     latex=r"I_x = \frac{bh^3 - (b-t_w)(h-2t_f)^3}{12},\quad Z_{p,x} = bt_f(h-t_f) + \frac{t_w(h-2t_f)^2}{4}",
     run=i_section,
     parse=lambda q: _pairs(q, r"i[- ]section(?: properties)?|i[- ]beam section(?: properties)?", {"b": "b", "h": "h", "tf": "tf", "tw": "tw"}, phys={"b", "h", "tf", "tw"}),
     reference=ref_i_section,
     wrong=_safe(lambda b, h, tf, tw: (lambda hw: _out(A=2 * b * tf + tw * hw, Ix=(2 * b * tf ** 3 + tw * hw ** 3) / 12, Iy=(2 * tf * b ** 3 + hw * tw ** 3) / 12,
                                                     Zx=(2 * b * tf ** 3 + tw * hw ** 3) / (6 * h), Zy=(2 * tf * b ** 3 + hw * tw ** 3) / (6 * b), rx=0.0, ry=0.0,
                                                     Zpx=b * tf * h, Zpy=tf * b * b, shape_factor_x=1.0))(h - 2 * tf)),     # flanges without their parallel-axis term
     random_inputs=_rand_i,
     templates=["I-section: b = 150 mm, h = 300 mm, tf = 12 mm, tw = 8 mm"],
     examples=[{"inputs": {"b": 0.15, "h": 0.3, "tf": 0.012, "tw": 0.008},
                "outputs": {"Ix": (0.15 * 0.3 ** 3 - 0.142 * 0.276 ** 3) / 12, "Zpx": 0.15 * 0.012 * 0.288 + 0.008 * 0.276 ** 2 / 4}}],
     questions=[{"q": "I-section: b = 150 mm, h = 300 mm, tf = 12 mm, tw = 8 mm", "outputs": {"Ix": (0.15 * 0.3 ** 3 - 0.142 * 0.276 ** 3) / 12}},
                {"q": "I-section: b = 150 mm, h = 300 mm, tf = 160 mm, tw = 8 mm", "outcome": "INVALID_INPUT"}])


def t_section(bf, tf, h, tw):
    _pos("bf, tf, h and tw", bf, tf, h, tw)
    if tf >= h:
        raise ValueError("the flange thickness must be less than the overall depth")
    if tw > bf:
        raise ValueError("the stem must not be wider than the flange")
    hs = h - tf
    A, _xc, yb, Ix, Iy, _ = _rect_set([(1, bf, tf, 0.0, h - tf / 2), (1, tw, hs, 0.0, hs / 2)])
    return _out(A=A, y_bar=yb, Ix=Ix, Iy=Iy, Z_top=Ix / (h - yb), Z_bottom=Ix / yb, rx=math.sqrt(Ix / A), ry=math.sqrt(Iy / A))


def ref_t_section(bf, tf, h, tw):
    """sectionproperties: the T outline as one polygon (stem at the bottom, flange on top)."""
    x0, x1 = (bf - tw) / 2, (bf + tw) / 2
    s = _sp([(x0, 0), (x1, 0), (x1, h - tf), (bf, h - tf), (bf, h), (0, h), (0, h - tf), (x0, h - tf)])
    _cx, cy = s.get_c()
    z = s.get_z()
    ixx, iyy, _ = s.get_ic()
    rx, ry = s.get_rc()
    return {"A": s.get_area(), "y_bar": cy, "Ix": ixx, "Iy": iyy, "Z_top": z[0], "Z_bottom": z[1], "rx": rx, "ry": ry}


_add("t_section_properties",
     name="Section properties of a T-section", topic="section properties / standard shapes",
     purpose="Area, centroid height, second moments, top and bottom section moduli and radii of gyration of a T-section (flange on top).",
     inputs=[_phys("bf", "length", "flange width"), _phys("tf", "length", "flange thickness"), _phys("h", "length", "overall depth"), _phys("tw", "length", "stem (web) thickness")],
     outputs=[_o("A", "area", "area"), _o("y_bar", "length", "height of the centroid above the bottom of the stem"),
              _o("Ix", "second_moment_of_area", "second moment about the horizontal centroidal axis"), _o("Iy", "second_moment_of_area", "second moment about the vertical axis of symmetry"),
              _o("Z_top", "section_modulus", "section modulus for the top fibre"), _o("Z_bottom", "section_modulus", "section modulus for the bottom fibre"),
              _o("rx", "length", "radius of gyration about x"), _o("ry", "length", "radius of gyration about y")],
     primary="Ix", conditions=["tf < h", "tw <= bf", "no fillets"],
     method="two rectangles: y_bar = sum(A y)/sum(A); Ix = sum(bh^3/12 + A (y - y_bar)^2); Z = Ix / distance to the fibre",
     latex=r"\bar y = \frac{\sum A_i y_i}{\sum A_i},\quad I_x = \sum\Big(\frac{b_ih_i^3}{12} + A_i(y_i - \bar y)^2\Big)",
     run=t_section,
     parse=lambda q: _pairs(q, r"t[- ]section(?: properties)?", {"bf": "bf", "tf": "tf", "h": "h", "tw": "tw"}, phys={"bf", "tf", "h", "tw"}),
     reference=ref_t_section,
     wrong=_safe(lambda bf, tf, h, tw: (lambda yb: _out(A=bf * tf + tw * (h - tf), y_bar=yb, Ix=bf * tf ** 3 / 12 + tw * (h - tf) ** 3 / 12, Iy=tf * bf ** 3 / 12,
                                                      Z_top=1.0, Z_bottom=1.0, rx=1.0, ry=1.0))(h / 2)),
     random_inputs=lambda rng: (lambda h: (lambda bf: {"bf": bf, "tf": h * rng.uniform(0.05, 0.3), "h": h, "tw": bf * rng.uniform(0.05, 0.5)})(h * rng.uniform(0.4, 1.5)))(_lu(rng, 0.04, 0.8)),
     templates=["T-section: bf = 120 mm, tf = 20 mm, h = 150 mm, tw = 15 mm"],
     examples=[{"inputs": {"bf": 0.12, "tf": 0.02, "h": 0.15, "tw": 0.015},
                "outputs": {"y_bar": (0.12 * 0.02 * 0.14 + 0.015 * 0.13 * 0.065) / (0.12 * 0.02 + 0.015 * 0.13)}}],
     questions=[{"q": "T-section: bf = 120 mm, tf = 20 mm, h = 150 mm, tw = 15 mm",
                 "outputs": {"y_bar": (0.12 * 0.02 * 0.14 + 0.015 * 0.13 * 0.065) / (0.12 * 0.02 + 0.015 * 0.13)}},
                {"q": "T-section: bf = 120 mm, tf = 200 mm, h = 150 mm, tw = 15 mm", "outcome": "INVALID_INPUT"}])


def channel_section(b, h, tf, tw):
    _pos("b, h, tf and tw", b, h, tf, tw)
    if 2 * tf >= h or tw >= b:
        raise ValueError("need 2 tf < h and tw < b")
    hw = h - 2 * tf
    A, xb, _yc, Ix, Iy, _ = _rect_set([(1, b, tf, b / 2, h - tf / 2), (1, b, tf, b / 2, tf / 2), (1, tw, hw, tw / 2, h / 2)])
    return _out(A=A, x_bar=xb, Ix=Ix, Iy=Iy, Zx=Ix / (h / 2), Zy_min=Iy / (b - xb), rx=math.sqrt(Ix / A), ry=math.sqrt(Iy / A))


def ref_channel_section(b, h, tf, tw):
    """sectionproperties: the channel outline (web on the left) as one polygon."""
    s = _sp([(0, 0), (b, 0), (b, tf), (tw, tf), (tw, h - tf), (b, h - tf), (b, h), (0, h)])
    cx, _cy = s.get_c()
    z = s.get_z()
    ixx, iyy, _ = s.get_ic()
    rx, ry = s.get_rc()
    return {"A": s.get_area(), "x_bar": cx, "Ix": ixx, "Iy": iyy, "Zx": min(z[0], z[1]), "Zy_min": min(z[2], z[3]), "rx": rx, "ry": ry}


_add("channel_section_properties",
     name="Section properties of a channel (C) section", topic="section properties / standard shapes",
     purpose="Area, centroid distance from the back of the web, second moments, section moduli and radii of gyration of a channel with square corners.",
     inputs=[_phys("b", "length", "flange width (including the web)"), _phys("h", "length", "overall depth"),
             _phys("tf", "length", "flange thickness"), _phys("tw", "length", "web thickness")],
     outputs=[_o("A", "area", "area"), _o("x_bar", "length", "distance of the centroid from the back of the web"),
              _o("Ix", "second_moment_of_area", "second moment about the horizontal axis of symmetry"), _o("Iy", "second_moment_of_area", "second moment about the vertical centroidal axis"),
              _o("Zx", "section_modulus", "section modulus about x"), _o("Zy_min", "section_modulus", "smaller section modulus about y (flange tips)"),
              _o("rx", "length", "radius of gyration about x"), _o("ry", "length", "radius of gyration about y")],
     primary="Iy", conditions=["2 tf < h", "tw < b", "square corners, no taper"],
     method="three rectangles (two flanges b x tf, web tw x (h - 2tf)) with the parallel-axis theorem",
     latex=r"\bar x = \frac{\sum A_i x_i}{\sum A_i},\quad I_y = \sum\Big(\frac{h_ib_i^3}{12} + A_i(x_i-\bar x)^2\Big)",
     run=channel_section,
     parse=lambda q: _pairs(q, r"channel section(?: properties)?|c[- ]section(?: properties)?", {"b": "b", "h": "h", "tf": "tf", "tw": "tw"}, phys={"b", "h", "tf", "tw"}),
     reference=ref_channel_section,
     wrong=_safe(lambda b, h, tf, tw: (lambda r: dict(r, Iy=r["Iy"] * 0 + (2 * tf * b ** 3 + (h - 2 * tf) * tw ** 3) / 12, x_bar=b / 2))(channel_section(b, h, tf, tw))),
     random_inputs=lambda rng: (lambda h: (lambda b: {"b": b, "h": h, "tf": h * rng.uniform(0.03, 0.15), "tw": b * rng.uniform(0.05, 0.4)})(h * rng.uniform(0.25, 0.7)))(_lu(rng, 0.05, 0.6)),
     templates=["Channel section: b = 80 mm, h = 200 mm, tf = 10 mm, tw = 7 mm"],
     examples=[{"inputs": {"b": 0.08, "h": 0.2, "tf": 0.01, "tw": 0.007},
                "outputs": {"x_bar": (2 * 0.08 * 0.01 * 0.04 + 0.007 * 0.18 * 0.0035) / (2 * 0.08 * 0.01 + 0.007 * 0.18)}}],
     questions=[{"q": "Channel section: b = 80 mm, h = 200 mm, tf = 10 mm, tw = 7 mm",
                 "outputs": {"x_bar": (2 * 0.08 * 0.01 * 0.04 + 0.007 * 0.18 * 0.0035) / (2 * 0.08 * 0.01 + 0.007 * 0.18)}}])


def angle_section(b, h, t):
    _pos("b, h and t", b, h, t)
    if t >= min(b, h):
        raise ValueError("the thickness must be less than both leg lengths")
    A, xc, yc, Ix, Iy, Ixy = _rect_set([(1, b, t, b / 2, t / 2), (1, t, h - t, t / 2, t + (h - t) / 2)])
    I1, I2, phi = _principal(Ix, Iy, Ixy)
    return _out(A=A, x_bar=xc, y_bar=yc, Ix=Ix, Iy=Iy, Ixy=Ixy, I1=I1, I2=I2, principal_angle_deg=phi, r_min=math.sqrt(I2 / A))


def ref_angle_section(b, h, t):
    """sectionproperties: the L outline as one polygon; centroidal and principal properties by the library."""
    s = _sp([(0, 0), (b, 0), (b, t), (t, t), (t, h), (0, h)])
    cx, cy = s.get_c()
    ixx, iyy, ixy = s.get_ic()
    i11, i22 = s.get_ip()
    phi = s.get_phi()
    phi = phi - 180 if phi > 90 else (phi + 180 if phi <= -90 else phi)
    return {"A": s.get_area(), "x_bar": cx, "y_bar": cy, "Ix": ixx, "Iy": iyy, "Ixy": ixy, "I1": i11, "I2": i22, "principal_angle_deg": phi,
            "r_min": math.sqrt(i22 / s.get_area())}


_add("angle_section_properties",
     name="Section properties of an angle (L) section, with principal axes", topic="section properties / unsymmetric sections",
     purpose="Area, centroid, centroidal second moments and product of inertia, principal second moments, the principal angle and the least radius of gyration of an equal or unequal angle.",
     inputs=[_phys("b", "length", "horizontal leg length"), _phys("h", "length", "vertical leg length"), _phys("t", "length", "thickness (both legs)")],
     outputs=[_o("A", "area", "area"), _o("x_bar", "length", "centroid distance from the back of the vertical leg"), _o("y_bar", "length", "centroid distance from the bottom of the horizontal leg"),
              _o("Ix", "second_moment_of_area", "centroidal second moment about x"), _o("Iy", "second_moment_of_area", "centroidal second moment about y"),
              _o("Ixy", "second_moment_of_area", "centroidal product of inertia (integral of x y dA)"), _o("I1", "second_moment_of_area", "major principal second moment"),
              _o("I2", "second_moment_of_area", "minor principal second moment"), _o("principal_angle_deg", "number", "angle of the major principal axis from x, degrees, in (-90, 90]"),
              _o("r_min", "length", "least radius of gyration")],
     primary="I2", conditions=["t < b and t < h", "square corners, no root radius"],
     method="two rectangles with the parallel-axis theorem (also for Ixy); I1,2 = (Ix + Iy)/2 +- sqrt(((Ix - Iy)/2)^2 + Ixy^2), tan(2 phi) = -2 Ixy/(Ix - Iy)",
     latex=r"I_{1,2} = \frac{I_x+I_y}{2} \pm \sqrt{\Big(\frac{I_x-I_y}{2}\Big)^2 + I_{xy}^2},\quad \tan2\varphi = \frac{-2I_{xy}}{I_x-I_y}",
     run=angle_section,
     parse=lambda q: _pairs(q, r"angle section(?: properties)?|l[- ]section(?: properties)?", {"b": "b", "h": "h", "t": "t"}, phys={"b", "h", "t"}),
     reference=ref_angle_section,
     wrong=_safe(lambda b, h, t: (lambda r: _out(**dict(r, I1=r["Ix"] if r["Ix"] > r["Iy"] else r["Iy"], I2=min(r["Ix"], r["Iy"]), r_min=math.sqrt(min(r["Ix"], r["Iy"]) / r["A"]))))(angle_section(b, h, t))),
     random_inputs=lambda rng: (lambda b, h: {"b": b, "h": h, "t": min(b, h) * rng.uniform(0.05, 0.3)})(_lu(rng, 0.02, 0.3), _lu(rng, 0.02, 0.3)),
     templates=["Angle section: b = 100 mm, h = 150 mm, t = 10 mm"],
     examples=[{"inputs": {"b": 0.1, "h": 0.1, "t": 0.01}, "outputs": {"principal_angle_deg": 45.0, "x_bar": (0.1 * 0.01 * 0.05 + 0.01 * 0.09 * 0.005) / 0.0019}}],
     questions=[{"q": "Angle section: b = 100 mm, h = 100 mm, t = 10 mm", "outputs": {"principal_angle_deg": 45.0}},
                {"q": "Angle section: b = 100 mm, h = 150 mm, t = 200 mm", "outcome": "INVALID_INPUT"}])


def composite_rects(rects):
    rows = [list(r) for r in rects]
    if any(len(r) != 4 for r in rows):
        raise ValueError("each rectangle is a row 'b h xc yc' (width, height, centre x, centre y in mm)")
    if not rows or len(rows) > 20:
        raise ValueError("give 1 to 20 rectangles")
    for b, h, _x, _y in rows:
        _pos("each rectangle's b and h", b, h)
    for i in range(len(rows)):
        for j in range(i + 1, len(rows)):
            b1, h1, x1, y1 = rows[i]
            b2, h2, x2, y2 = rows[j]
            ox = min(x1 + b1 / 2, x2 + b2 / 2) - max(x1 - b1 / 2, x2 - b2 / 2)
            oy = min(y1 + h1 / 2, y2 + h2 / 2) - max(y1 - h1 / 2, y2 - h2 / 2)
            if ox > 1e-9 * max(b1, b2) and oy > 1e-9 * max(h1, h2):
                raise ValueError(f"rectangles {i + 1} and {j + 1} overlap: the parts of a composite section must not overlap")
    A, xc, yc, Ix, Iy, Ixy = _rect_set([(1, b, h, x, y) for b, h, x, y in rows])
    size = max(abs(x) + b / 2 + abs(y) + h / 2 for b, h, x, y in rows)
    xc, yc = _snap(xc, size), _snap(yc, size)
    sc = max(Ix, Iy)
    Ixy = _snap(Ixy, sc)
    I1, I2, phi = _principal(Ix, Iy, Ixy)
    return _out(A=A, x_bar=xc, y_bar=yc, Ix=Ix, Iy=Iy, Ixy=Ixy, I1=I1, I2=I2)


def ref_composite_rects(rects):
    """sectionproperties: every rectangle a geometry, combined into one compound section and analysed together."""
    from shapely.geometry import Polygon
    from sectionproperties.analysis.section import Section
    from sectionproperties.pre.geometry import Geometry
    g = None
    for b, h, x, y in rects:
        gi = Geometry(Polygon([(x - b / 2, y - h / 2), (x + b / 2, y - h / 2), (x + b / 2, y + h / 2), (x - b / 2, y + h / 2)]))
        g = gi if g is None else g + gi
    g.create_mesh(mesh_sizes=[0])
    s = Section(geometry=g)
    s.calculate_geometric_properties()
    size = max(abs(x) + b / 2 + abs(y) + h / 2 for b, h, x, y in rects)
    cx, cy = (_snap(c, size) for c in s.get_c())
    ixx, iyy, ixy = s.get_ic()
    i11, i22 = s.get_ip()
    return {"A": s.get_area(), "x_bar": cx, "y_bar": cy, "Ix": ixx, "Iy": iyy, "Ixy": _snap(ixy, max(ixx, iyy)), "I1": i11, "I2": i22}


def _rand_composite(rng):
    """Rectangles stacked or placed side by side without overlap (integer mm)."""
    rows, y = [], 0.0
    for _ in range(rng.randint(2, 4)):
        b, h = rng.randint(5, 200), rng.randint(5, 100)
        x = rng.randint(-60, 60)
        rows.append([float(b), float(h), float(x), y + h / 2])
        y += h
    return {"rects": rows}


_add("composite_section_properties",
     name="Composite section of rectangles (parallel-axis theorem)", topic="section properties / composite sections",
     purpose="Area, centroid, centroidal second moments, product of inertia and principal second moments of a section built from non-overlapping rectangles.",
     inputs=[_mat("rects", "one row per rectangle: width b, height h, centre x, centre y, all in mm")],
     outputs=[_o("A", "number", "area in mm^2"), _o("x_bar", "number", "centroid x in mm"), _o("y_bar", "number", "centroid y in mm"),
              _o("Ix", "number", "centroidal second moment about x in mm^4"), _o("Iy", "number", "centroidal second moment about y in mm^4"),
              _o("Ixy", "number", "centroidal product of inertia in mm^4"), _o("I1", "number", "major principal second moment in mm^4"),
              _o("I2", "number", "minor principal second moment in mm^4")],
     primary="Ix", conditions=["rectangles do not overlap (touching is allowed)", "1 to 20 rectangles, all dimensions in mm"],
     method="parallel-axis theorem: x_bar = sum A x / sum A, Ix = sum(bh^3/12 + A (y - y_bar)^2), Ixy = sum A (x - x_bar)(y - y_bar)",
     latex=r"I_x = \sum_i\Big(\frac{b_ih_i^3}{12} + A_i(y_i-\bar y)^2\Big),\quad I_{xy} = \sum_i A_i(x_i-\bar x)(y_i-\bar y)",
     run=composite_rects,
     parse=lambda q: _pairs(q, r"composite section(?: properties)?|section of rectangles", {"rects": "rects", "rectangles": "rects"}, matrices={"rects"}),
     reference=ref_composite_rects,
     wrong=_safe(lambda rects: (lambda A: _out(A=A, x_bar=0.0, y_bar=0.0, Ix=sum(b * h ** 3 / 12 for b, h, _x, _y in rects), Iy=sum(h * b ** 3 / 12 for b, h, _x, _y in rects),
                                              Ixy=0.0, I1=0.0, I2=0.0))(sum(b * h for b, h, _x, _y in rects))),
     random_inputs=_rand_composite,
     templates=["Composite section: rects = [200 20 0 10; 20 300 0 170]"],
     examples=[{"inputs": {"rects": "200 20 0 10; 20 300 0 170"},
                "outputs": {"A": 10000.0, "y_bar": (4000 * 10 + 6000 * 170) / 10000.0, "Ixy": 0.0}}],
     questions=[{"q": "Composite section: rects = [200 20 0 10; 20 300 0 170]", "outputs": {"A": 10000.0, "y_bar": 106.0}},
                {"q": "Composite section: rects = [200 20 0 10; 20 300 0 15]", "outcome": "INVALID_INPUT"}])


def _segments_cross(p1, p2, p3, p4):
    def orient(a, b, c):
        v = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
        return 0 if v == 0 else (1 if v > 0 else -1)
    o1, o2, o3, o4 = orient(p1, p2, p3), orient(p1, p2, p4), orient(p3, p4, p1), orient(p3, p4, p2)
    if o1 != o2 and o3 != o4:
        return True
    on = lambda a, b, c: min(a[0], b[0]) <= c[0] <= max(a[0], b[0]) and min(a[1], b[1]) <= c[1] <= max(a[1], b[1])
    return (o1 == 0 and on(p1, p2, p3)) or (o2 == 0 and on(p1, p2, p4)) or (o3 == 0 and on(p3, p4, p1)) or (o4 == 0 and on(p3, p4, p2))


def polygon_section(vertices):
    pts = [tuple(r) for r in vertices]
    if any(len(p) != 2 for p in pts) or not 3 <= len(pts) <= 60:
        raise ValueError("give 3 to 60 vertices as rows 'x y' (mm)")
    n = len(pts)
    if len(set(pts)) != n:
        raise ValueError("vertices must be distinct")
    for i in range(n):
        for j in range(i + 1, n):
            if j == i + 1 or (i == 0 and j == n - 1):
                continue
            if _segments_cross(pts[i], pts[(i + 1) % n], pts[j], pts[(j + 1) % n]):
                raise ValueError("the polygon must be simple (its edges must not cross)")
    a = cx = cy = ixx = iyy = ixy = 0.0
    terms = []
    for i in range(n):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        terms.append((c, x0, y0, x1, y1))
    a = math.fsum(c for c, *_ in terms) / 2
    if abs(a) <= 1e-12 * max(abs(v) for p in pts for v in p) ** 2:
        raise ValueError("the polygon has no area")
    cx = math.fsum(c * (x0 + x1) for c, x0, y0, x1, y1 in terms) / (6 * a)
    cy = math.fsum(c * (y0 + y1) for c, x0, y0, x1, y1 in terms) / (6 * a)
    ixx = math.fsum(c * (y0 * y0 + y0 * y1 + y1 * y1) for c, x0, y0, x1, y1 in terms) / 12
    iyy = math.fsum(c * (x0 * x0 + x0 * x1 + x1 * x1) for c, x0, y0, x1, y1 in terms) / 12
    ixy = math.fsum(c * (x0 * y1 + 2 * x0 * y0 + 2 * x1 * y1 + x1 * y0) for c, x0, y0, x1, y1 in terms) / 24
    sgn = 1.0 if a > 0 else -1.0                 # clockwise vertices: every integral changes sign
    A = abs(a)
    Ix = sgn * ixx - A * cy * cy
    Iy = sgn * iyy - A * cx * cx
    size = max(abs(v) for p in pts for v in p)
    cx, cy = _snap(cx, size), _snap(cy, size)
    Ixy = _snap(sgn * ixy - A * cx * cy, max(Ix, Iy))
    I1, I2, phi = _principal(Ix, Iy, Ixy)
    return _out(A=A, x_bar=cx, y_bar=cy, Ix=Ix, Iy=Iy, Ixy=Ixy, I1=I1, I2=I2)


def ref_polygon_section(vertices):
    """sectionproperties: the polygon meshed and analysed by the library (finite-element integration, not the
    vertex formulas)."""
    pts = [tuple(r) for r in vertices]
    s = _sp(pts)
    size = max(abs(v) for p in pts for v in p)
    cx, cy = (_snap(c, size) for c in s.get_c())
    ixx, iyy, ixy = s.get_ic()
    i11, i22 = s.get_ip()
    return {"A": s.get_area(), "x_bar": cx, "y_bar": cy, "Ix": ixx, "Iy": iyy, "Ixy": _snap(ixy, max(ixx, iyy)), "I1": i11, "I2": i22}


def _rand_polygon(rng):
    """A star-shaped polygon: vertices at increasing angles around a centre, random radii (integer mm)."""
    n = rng.randint(3, 9)
    angs = sorted(rng.uniform(0, 2 * math.pi) for _ in range(n))
    if max(b - a for a, b in zip(angs, angs[1:] + [angs[0] + 2 * math.pi])) > 0.9 * math.pi:
        angs = [2 * math.pi * k / n + rng.uniform(-0.2, 0.2) for k in range(n)]
    cx, cy = rng.randint(-50, 50), rng.randint(-50, 50)
    return {"vertices": [[float(round(cx + rng.uniform(20, 150) * math.cos(t))), float(round(cy + rng.uniform(20, 150) * math.sin(t)))] for t in angs]}


_add("polygon_section_properties",
     name="Section properties of a polygonal section", topic="section properties / arbitrary sections",
     purpose="Area, centroid, centroidal second moments, product of inertia and principal second moments of any simple polygon given by its vertices.",
     inputs=[_mat("vertices", "vertices in order (either sense), one row 'x y' per vertex, in mm")],
     outputs=[_o("A", "number", "area in mm^2"), _o("x_bar", "number", "centroid x in mm"), _o("y_bar", "number", "centroid y in mm"),
              _o("Ix", "number", "centroidal second moment about x in mm^4"), _o("Iy", "number", "centroidal second moment about y in mm^4"),
              _o("Ixy", "number", "centroidal product of inertia in mm^4"), _o("I1", "number", "major principal second moment in mm^4"),
              _o("I2", "number", "minor principal second moment in mm^4")],
     primary="Ix", conditions=["3 to 60 distinct vertices", "a simple polygon (edges do not cross)", "no holes"],
     method="Green's theorem over the edges: A = 1/2 sum c_i, Ix = 1/12 sum c_i (y_i^2 + y_i y_i+1 + y_i+1^2) with c_i = x_i y_i+1 - x_i+1 y_i; then shifted to the centroid",
     latex=r"A = \tfrac12\sum c_i,\quad I_x = \tfrac1{12}\sum c_i\,(y_i^2+y_iy_{i+1}+y_{i+1}^2) - A\bar y^2",
     run=polygon_section,
     parse=lambda q: _pairs(q, r"polygon section(?: properties)?|polygonal section(?: properties)?", {"vertices": "vertices", "points": "vertices"}, matrices={"vertices"}),
     reference=ref_polygon_section,
     wrong=_safe(lambda vertices: (lambda r: dict(r, Ix=r["Ix"] + r["A"] * r["y_bar"] ** 2, Iy=r["Iy"] + r["A"] * r["x_bar"] ** 2))(polygon_section(vertices))
                 if abs(polygon_section(vertices)["y_bar"]) > 1 else {"Ix": -1.0, "A": -1.0}),
     random_inputs=_rand_polygon,
     templates=["Polygon section: vertices = [0 0; 100 0; 100 200; 0 200]"],
     examples=[{"inputs": {"vertices": "0 0; 100 0; 100 200; 0 200"}, "outputs": {"A": 20000.0, "x_bar": 50.0, "y_bar": 100.0, "Ix": 100 * 200 ** 3 / 12, "Ixy": 0.0}},
               {"inputs": {"vertices": "0 0; 0 30; 40 0"}, "outputs": {"A": 600.0, "x_bar": 40 / 3, "y_bar": 10.0, "Ix": 40 * 30 ** 3 / 36, "Ixy": -40 * 40 * 30 * 30 / 72}}],
     questions=[{"q": "Polygon section: vertices = [0 0; 100 0; 100 200; 0 200]", "outputs": {"A": 20000.0, "Ix": 100 * 200 ** 3 / 12}},
                {"q": "Polygon section: vertices = [0 0; 100 100; 100 0; 0 100]", "outcome": "INVALID_INPUT"}])


# ================================================================================================ torsion
def _prandtl_disk(Ro, Ri=0.0):
    """Torque per unit (G * twist rate) of a circular or annular shaft from Prandtl's stress function
    phi = (Ro^2 - r^2)/2 (flat at phi(Ri) over a hole): T = 2 * integral of phi over the material + 2 phi(Ri) * hole
    area, each integral computed numerically by scipy QUADPACK in Cartesian strips. The boundary shear stress per unit
    (G * twist rate) is the stress function's gradient there, Ro."""
    T = _quad_alg(lambda y: 4.0 / 3.0, -Ro, Ro, 1.5)                   # integral of (Ro^2 - r^2) over the full disk
    if Ri > 0:
        r2 = _quad_alg(lambda y: 2.0 * y * y, -Ri, Ri, 0.5) + _quad_alg(lambda y: 2.0 / 3.0, -Ri, Ri, 1.5)
        hole = _quad_alg(lambda y: 2.0, -Ri, Ri, 0.5)
        T = T + r2 - Ri * Ri * hole                                     # the hole: paraboloid removed, flat phi(Ri) added
    return T, Ro


def shaft_torsion(T, d, L, G, d_i=0.0):
    _pos("T, d, L and G", T, d, L, G)
    _nonneg("the inner diameter", d_i)
    if d_i >= d:
        raise ValueError("the inner diameter must be smaller than the outer diameter")
    J = math.pi * (d ** 4 - d_i ** 4) / 32
    tau = T * (d / 2) / J
    phi = T * L / (G * J)
    return _out(tau_max_MPa=tau / 1e6, tau_inner_MPa=tau * d_i / d / 1e6, twist_rad=phi, twist_deg=math.degrees(phi), J=J)


def ref_shaft_torsion(T, d, L, G, d_i=0.0):
    """Elasticity route: Prandtl's stress function of the circular (or annular) section, its integral for the
    torque per unit twist rate computed by scipy quadrature; stress from the stress-function gradient."""
    Tu, Ro = _prandtl_disk(d / 2, d_i / 2)
    rate = T / (G * Tu)                    # twist per unit length
    return {"tau_max_MPa": G * rate * Ro / 1e6, "tau_inner_MPa": G * rate * d_i / 2 / 1e6, "twist_rad": rate * L, "twist_deg": math.degrees(rate * L), "J": Tu}


_TORS_OUT = [_o("tau_max_MPa", "number", "maximum shear stress (outer surface) in MPa"), _o("tau_inner_MPa", "number", "shear stress at the inner surface in MPa (0 for a solid shaft)"),
             _o("twist_rad", "angle", "angle of twist"), _o("twist_deg", "number", "angle of twist in degrees"), _o("J", "second_moment_of_area", "polar second moment of area")]

_add("solid_shaft_torsion",
     name="Torsion of a solid circular shaft", topic="torsion / circular shafts",
     purpose="Maximum shear stress, polar moment and angle of twist of a solid circular shaft under a torque.",
     inputs=[_phys("T", "torque", "applied torque"), _phys("d", "length", "diameter"), _phys("L", "length", "length"), _phys("G", "shear_modulus", "shear modulus")],
     outputs=_TORS_OUT, primary="tau_max_MPa", conditions=["T, d, L, G > 0", "linear elastic, circular section, uniform torque"],
     method="J = pi d^4/32, tau_max = T (d/2)/J = 16T/(pi d^3), phi = TL/(GJ)",
     latex=r"\tau_{max} = \frac{16T}{\pi d^3},\quad \varphi = \frac{TL}{GJ},\quad J = \frac{\pi d^4}{32}",
     run=lambda T, d, L, G: shaft_torsion(T, d, L, G),
     parse=lambda q: _pairs(q, r"solid shaft torsion|torsion of a solid shaft", {"T": "T", "d": "d", "L": "L", "G": "G"}, phys={"T", "d", "L", "G"}),
     reference=lambda T, d, L, G: ref_shaft_torsion(T, d, L, G),
     wrong=_safe(lambda T, d, L, G: (lambda J: _out(tau_max_MPa=T * d / J / 1e6, tau_inner_MPa=0.0, twist_rad=T * L / (G * J), twist_deg=math.degrees(T * L / (G * J)), J=J))(math.pi * d ** 4 / 64)),
     random_inputs=lambda rng: {"T": _lu(rng, 1, 5e4), "d": _lu(rng, 0.005, 0.3), "L": rng.uniform(0.1, 5), "G": rng.uniform(25e9, 85e9)},
     templates=["Solid shaft torsion: T = 1.2 kN*m, d = 50 mm, L = 1.5 m, G = 80 GPa"],
     examples=[{"inputs": {"T": 1200.0, "d": 0.05, "L": 1.5, "G": 80e9},
                "outputs": {"tau_max_MPa": 16 * 1200 / (math.pi * 0.05 ** 3) / 1e6, "twist_rad": 1200 * 1.5 / (80e9 * math.pi * 0.05 ** 4 / 32)}}],
     questions=[{"q": "Solid shaft torsion: T = 1.2 kN*m, d = 50 mm, L = 1.5 m, G = 80 GPa", "outputs": {"tau_max_MPa": 16 * 1200 / (math.pi * 0.05 ** 3) / 1e6}},
                {"q": "Solid shaft torsion: T = 1.2 kN*m, d = 50 mm, L = 1.5 m", "outcome": "NEEDS_INPUT"}])

def _hollow_shaft(T, D, d, L, G):
    _pos("the inner diameter d (use the solid shaft for d = 0)", d)
    return shaft_torsion(T, D, L, G, d)


_add("hollow_shaft_torsion",
     name="Torsion of a hollow circular shaft", topic="torsion / circular shafts",
     purpose="Shear stresses at the outer and inner surfaces, polar moment and angle of twist of a hollow circular shaft under a torque.",
     inputs=[_phys("T", "torque", "applied torque"), _phys("D", "length", "outer diameter"), _phys("d", "length", "inner diameter"),
             _phys("L", "length", "length"), _phys("G", "shear_modulus", "shear modulus")],
     outputs=_TORS_OUT, primary="tau_max_MPa", conditions=["D > d > 0", "linear elastic"],
     method="J = pi (D^4 - d^4)/32, tau_max = T (D/2)/J, tau_inner = T (d/2)/J, phi = TL/(GJ)",
     latex=r"\tau_{max} = \frac{16TD}{\pi(D^4-d^4)},\quad \varphi = \frac{TL}{GJ}",
     run=lambda T, D, d, L, G: _hollow_shaft(T, D, d, L, G),
     parse=lambda q: _pairs(q, r"hollow shaft torsion|torsion of a hollow shaft", {"T": "T", "D": "D", "d": "d", "L": "L", "G": "G"}, phys={"T", "D", "d", "L", "G"}),
     reference=lambda T, D, d, L, G: ref_shaft_torsion(T, D, L, G, d),
     wrong=_safe(lambda T, D, d, L, G: (lambda J: _out(tau_max_MPa=T * D / 2 / J / 1e6, tau_inner_MPa=T * d / 2 / J / 1e6, twist_rad=T * L / (G * J), twist_deg=math.degrees(T * L / (G * J)), J=J))(math.pi * (D - d) ** 4 / 32)),
     random_inputs=lambda rng: (lambda D: {"T": _lu(rng, 1, 5e4), "D": D, "d": D * rng.uniform(0.2, 0.95), "L": rng.uniform(0.1, 5), "G": rng.uniform(25e9, 85e9)})(_lu(rng, 0.01, 0.3)),
     templates=["Hollow shaft torsion: T = 2 kN*m, D = 60 mm, d = 40 mm, L = 2 m, G = 80 GPa"],
     examples=[{"inputs": {"T": 2000.0, "D": 0.06, "d": 0.04, "L": 2.0, "G": 80e9},
                "outputs": {"tau_max_MPa": 2000 * 0.03 / (math.pi * (0.06 ** 4 - 0.04 ** 4) / 32) / 1e6, "tau_inner_MPa": 2000 * 0.02 / (math.pi * (0.06 ** 4 - 0.04 ** 4) / 32) / 1e6}}],
     questions=[{"q": "Hollow shaft torsion: T = 2 kN*m, D = 60 mm, d = 40 mm, L = 2 m, G = 80 GPa",
                 "outputs": {"tau_max_MPa": 2000 * 0.03 / (math.pi * (0.06 ** 4 - 0.04 ** 4) / 32) / 1e6}},
                {"q": "Hollow shaft torsion: T = 2 kN*m, D = 40 mm, d = 60 mm, L = 2 m, G = 80 GPa", "outcome": "INVALID_INPUT"}])


def shaft_power(P, n, tau_allow):
    _pos("P, n and the allowable shear stress", P, n, tau_allow)
    T = P / n
    d = (16 * T / (math.pi * tau_allow)) ** (1 / 3)
    return _out(T=T, d_min_mm=d * 1e3, frequency_Hz=n / (2 * math.pi))


def ref_shaft_power(P, n, tau_allow):
    """Torque from the work done per revolution (P * period / (2 pi)); the smallest diameter by scipy brentq on the
    stress-function shear stress of the solid section."""
    from scipy.optimize import brentq
    T = P * (2 * math.pi / n) / (2 * math.pi)
    def excess(d):
        Tu, Ro = _prandtl_disk(d / 2)
        return T / Tu * Ro - tau_allow
    d = brentq(excess, 1e-6, 10.0, xtol=1e-16, rtol=8.9e-16, maxiter=500)
    return {"T": T, "d_min_mm": d * 1e3}


_add("shaft_power_transmission",
     name="Torque and minimum diameter of a power-transmission shaft", topic="torsion / power transmission",
     purpose="The torque a solid shaft transmits at a given power and speed, and the smallest diameter that keeps the torsional shear stress within an allowable value.",
     inputs=[_phys("P", "power", "transmitted power"), _phys("n", "angular_frequency", "rotational speed (rpm or rad/s)"), _phys("tau_allow", "shear_stress", "allowable shear stress")],
     outputs=[_o("T", "torque", "transmitted torque"), _o("d_min_mm", "number", "minimum solid-shaft diameter in mm"), _o("frequency_Hz", "number", "rotational frequency in Hz")],
     primary="d_min_mm", conditions=["P, n, tau_allow > 0", "pure torsion (no bending), solid circular shaft"],
     method="T = P/omega, d_min = (16T/(pi tau_allow))^(1/3)",
     latex=r"T = \frac{P}{\omega},\quad d_{min} = \Big(\frac{16T}{\pi\tau_{allow}}\Big)^{1/3}",
     run=shaft_power,
     parse=lambda q: _pairs(q, r"shaft power transmission|power transmission shaft", {"P": "P", "n": "n", "N": "n", "tau_allow": "tau_allow", "tau": "tau_allow"}, phys={"P", "n", "tau_allow"}),
     reference=ref_shaft_power,
     wrong=_safe(lambda P, n, tau_allow: _out(T=P / n, d_min_mm=(32 * P / n / (math.pi * tau_allow)) ** (1 / 3) * 1e3, frequency_Hz=n)),
     random_inputs=lambda rng: {"P": _lu(rng, 100, 5e6), "n": _lu(rng, 10, 1000), "tau_allow": rng.uniform(20e6, 200e6)},
     templates=["Shaft power transmission: P = 50 kW, n = 1500 rpm, tau_allow = 40 MPa"],
     examples=[{"inputs": {"P": 50e3, "n": 1500 * 2 * math.pi / 60, "tau_allow": 40e6},
                "outputs": {"T": 50e3 / (50 * math.pi), "d_min_mm": (16 * 50e3 / (50 * math.pi) / (math.pi * 40e6)) ** (1 / 3) * 1e3}}],
     questions=[{"q": "Shaft power transmission: P = 50 kW, n = 1500 rpm, tau_allow = 40 MPa", "outputs": {"T": 50e3 / (50 * math.pi)}},
                {"q": "Shaft power transmission: P = 50 kW, n = 1500 rpm, tau_allow = 0 MPa", "outcome": "INVALID_INPUT"}])


def _bar_series_ana(lengths_mm, stiff, loads, fixed_both=False):
    """anastruct: a straight bar along x, one truss element per segment (axial stiffness `stiff` per segment, N),
    fixed at the left end (and at the right end if fixed_both), axial point loads at the joints {node index: Fx}.
    Coordinates in whole mm. Returns (system, number of nodes)."""
    from anastruct import SystemElements
    ss = SystemElements(EA=1.0)
    x = 0.0
    for Li, k in zip(lengths_mm, stiff):
        ss.add_truss_element(location=[[_mm(x / 1e3), 0.0], [_mm((x + Li) / 1e3), 0.0]], EA=k)
        x += Li
    n = len(lengths_mm) + 1
    ss.add_support_fixed(node_id=1)
    for nid in range(2, n + 1):
        if fixed_both and nid == n:
            ss.add_support_fixed(node_id=nid)
        else:
            ss.add_support_roll(node_id=nid, direction=2)
    for nid, F in loads.items():
        if F:
            ss.point_load(node_id=nid, Fx=F)
    ss.solve()
    return ss, n


def stepped_shaft(T, lengths, diameters, G):
    _pos("T and G", T, G)
    if len(lengths) != len(diameters):
        raise ValueError("lengths and diameters must have the same number of entries")
    for v in list(lengths) + list(diameters):
        _pos("every length and diameter", v)
    tw, tau = 0.0, 0.0
    for Li, di in zip(lengths, diameters):
        J = math.pi * (di * 1e-3) ** 4 / 32
        tw += T * Li * 1e-3 / (G * J)
        tau = max(tau, T * di * 1e-3 / 2 / J)
    return _out(twist_rad=tw, twist_deg=math.degrees(tw), tau_max_MPa=tau / 1e6)


def ref_stepped_shaft(T, lengths, diameters, G):
    """anastruct, by the exact torsion/axial analogy: each segment a truss bar of axial stiffness G*J (J from the
    Prandtl stress-function quadrature; lengths in mm, so EA = G J * 1000), the torque an axial end load; the twist
    is the end displacement."""
    Js = [_prandtl_disk(di * 1e-3 / 2)[0] for di in diameters]
    ss, n = _bar_series_ana(lengths, [G * J * 1e3 for J in Js], {len(lengths) + 1: T})
    tw = -ss.get_node_results_system(node_id=n)["ux"]               # anastruct reports displacements with the opposite sign
    tau = max(T * di * 1e-3 / 2 / J for di, J in zip(diameters, Js))
    return {"twist_rad": tw, "twist_deg": math.degrees(tw), "tau_max_MPa": tau / 1e6}


_add("stepped_shaft_twist",
     name="Angle of twist of a stepped shaft", topic="torsion / compound shafts",
     purpose="Total angle of twist and the largest shear stress of a shaft made of solid circular segments in series carrying one torque.",
     inputs=[_phys("T", "torque", "torque carried by every segment"), _vec("lengths", "segment lengths in mm (a list)"),
             _vec("diameters", "segment diameters in mm (a list, same order)"), _phys("G", "shear_modulus", "shear modulus")],
     outputs=[_o("twist_rad", "angle", "total angle of twist"), _o("twist_deg", "number", "total angle of twist in degrees"),
              _o("tau_max_MPa", "number", "largest shear stress (smallest segment) in MPa")],
     primary="twist_deg", conditions=["equal-length lists", "the same torque in every segment", "stress concentrations at the steps not included"],
     method="phi = sum T L_i/(G J_i), J_i = pi d_i^4/32, tau_max = max 16T/(pi d_i^3)",
     latex=r"\varphi = \sum_i\frac{TL_i}{GJ_i},\quad J_i = \frac{\pi d_i^4}{32}",
     run=stepped_shaft,
     parse=lambda q: _pairs(q, r"stepped shaft(?: twist)?", {"T": "T", "lengths": "lengths", "L": "lengths", "diameters": "diameters", "d": "diameters", "G": "G"},
                            phys={"T", "G"}, vectors={"lengths", "diameters"}),
     reference=ref_stepped_shaft,
     wrong=_safe(lambda T, lengths, diameters, G: (lambda tw: _out(twist_rad=tw, twist_deg=math.degrees(tw), tau_max_MPa=16 * T / (math.pi * (min(diameters) * 1e-3) ** 3) / 1e6))(
         T * sum(lengths) * 1e-3 / (G * math.pi * (sum(diameters) / len(diameters) * 1e-3) ** 4 / 32))),
     random_inputs=lambda rng: (lambda n: {"T": _lu(rng, 10, 5e3), "lengths": [float(rng.randint(50, 1500)) for _ in range(n)],
                                           "diameters": [float(rng.randint(10, 120)) for _ in range(n)], "G": rng.uniform(25e9, 85e9)})(rng.randint(2, 4)),
     templates=["Stepped shaft: T = 500 N*m, lengths = [400, 600], diameters = [40, 30], G = 80 GPa"],
     examples=[{"inputs": {"T": 500.0, "lengths": "400, 600", "diameters": "40, 30", "G": 80e9},
                "outputs": {"twist_rad": 500 * 0.4 / (80e9 * math.pi * 0.04 ** 4 / 32) + 500 * 0.6 / (80e9 * math.pi * 0.03 ** 4 / 32),
                            "tau_max_MPa": 16 * 500 / (math.pi * 0.03 ** 3) / 1e6}}],
     questions=[{"q": "Stepped shaft: T = 500 N*m, lengths = [400, 600], diameters = [40, 30], G = 80 GPa", "outputs": {"tau_max_MPa": 16 * 500 / (math.pi * 0.03 ** 3) / 1e6}},
                {"q": "Stepped shaft: T = 500 N*m, lengths = [400, 600], diameters = [40], G = 80 GPa", "outcome": "INVALID_INPUT"}])


def fixed_shaft(T0, a, b, d1, d2, G):
    _pos("T0, a, b, d1, d2 and G", T0, a, b, d1, d2, G)
    J1, J2 = math.pi * d1 ** 4 / 32, math.pi * d2 ** 4 / 32
    f1, f2 = a / (G * J1), b / (G * J2)
    TA = T0 * f2 / (f1 + f2)
    TB = T0 - TA
    phi = TA * f1
    return _out(TA=TA, TB=TB, tau1_MPa=TA * d1 / 2 / J1 / 1e6, tau2_MPa=TB * d2 / 2 / J2 / 1e6, twist_at_load_deg=math.degrees(phi))


def ref_fixed_shaft(T0, a, b, d1, d2, G):
    """anastruct, by the torsion/axial analogy: two truss bars (axial stiffness G*J) fixed at both ends with the torque
    as an axial load at the joint; reactions and the joint displacement from the finite-element solution."""
    J1, J2 = _prandtl_disk(d1 / 2)[0], _prandtl_disk(d2 / 2)[0]
    ss, n = _bar_series_ana([a * 1e3, b * 1e3], [G * J1 * 1e3, G * J2 * 1e3], {2: T0}, fixed_both=True)
    N1 = ss.get_element_results(element_id=1)["Nmax"]
    N2 = ss.get_element_results(element_id=2)["Nmax"]
    TA, TB = abs(N1), abs(N2)
    phi = abs(ss.get_node_results_system(node_id=2)["ux"])
    return {"TA": TA, "TB": TB, "tau1_MPa": TA * d1 / 2 / J1 / 1e6, "tau2_MPa": TB * d2 / 2 / J2 / 1e6, "twist_at_load_deg": math.degrees(phi)}


_add("statically_indeterminate_shaft",
     name="Shaft fixed at both ends with a torque between them", topic="torsion / statically indeterminate shafts",
     purpose="The reaction torques at both fixed ends, the shear stress in each part and the rotation at the loaded section of a shaft built in at both ends.",
     inputs=[_phys("T0", "torque", "torque applied between the ends"), _phys("a", "length", "length of part 1 (from end A to the load)"),
             _phys("b", "length", "length of part 2 (from the load to end B)"), _phys("d1", "length", "diameter of part 1"), _phys("d2", "length", "diameter of part 2"),
             _phys("G", "shear_modulus", "shear modulus")],
     outputs=[_o("TA", "torque", "reaction torque at A"), _o("TB", "torque", "reaction torque at B"), _o("tau1_MPa", "number", "maximum shear stress in part 1, MPa"),
              _o("tau2_MPa", "number", "maximum shear stress in part 2, MPa"), _o("twist_at_load_deg", "number", "rotation of the loaded section in degrees")],
     primary="TA", conditions=["both ends fully fixed", "solid circular parts of one material"],
     method="compatibility: TA a/(G J1) = TB b/(G J2), equilibrium TA + TB = T0",
     latex=r"T_A = T_0\frac{b/J_2}{a/J_1 + b/J_2},\quad T_B = T_0 - T_A",
     run=fixed_shaft,
     parse=lambda q: _pairs(q, r"statically indeterminate shaft|shaft fixed at both ends", {k: k for k in ("T0", "a", "b", "d1", "d2", "G")}, phys={"T0", "a", "b", "d1", "d2", "G"}),
     reference=ref_fixed_shaft,
     wrong=_safe(lambda T0, a, b, d1, d2, G: (lambda TA: _out(TA=TA, TB=T0 - TA, tau1_MPa=16 * TA / (math.pi * d1 ** 3) / 1e6, tau2_MPa=16 * (T0 - TA) / (math.pi * d2 ** 3) / 1e6,
                                                            twist_at_load_deg=math.degrees(TA * a / (G * math.pi * d1 ** 4 / 32))))(T0 * b / (a + b) if abs(d1 - d2) > 1e-3 * d1 else T0 * a / (a + b))),
     random_inputs=lambda rng: {"T0": _lu(rng, 10, 1e4), "a": round(rng.uniform(0.1, 2), 3), "b": round(rng.uniform(0.1, 2), 3), "d1": _lu(rng, 0.01, 0.15), "d2": _lu(rng, 0.01, 0.15),
                                "G": rng.uniform(25e9, 85e9)},
     templates=["Statically indeterminate shaft: T0 = 1 kN*m, a = 0.6 m, b = 0.9 m, d1 = 50 mm, d2 = 40 mm, G = 80 GPa"],
     examples=[{"inputs": {"T0": 1000.0, "a": 0.5, "b": 1.5, "d1": 0.05, "d2": 0.05, "G": 80e9}, "outputs": {"TA": 750.0, "TB": 250.0}}],
     questions=[{"q": "Statically indeterminate shaft: T0 = 1 kN*m, a = 0.5 m, b = 1.5 m, d1 = 50 mm, d2 = 50 mm, G = 80 GPa", "outputs": {"TA": 750.0, "TB": 250.0}}])


# ================================================================================================ axial members
def axial_bar(lengths, areas, loads, E):
    _pos("E", E)
    n = len(lengths)
    if not (len(areas) == n and len(loads) == n):
        raise ValueError("lengths, areas and loads must have the same number of entries (one per segment)")
    for v in list(lengths) + list(areas):
        _pos("every length and area", v)
    _fin("every load", *loads)
    N = [math.fsum(loads[i:]) for i in range(n)]              # internal force of segment i: all loads beyond it
    scale = max(abs(v) for v in loads) or 1.0
    N = [_snap(v, scale) for v in N]
    if all(v == 0 for v in N):
        raise ValueError("the loads balance to zero in every segment")
    stress = [Ni * 1e3 / Ai for Ni, Ai in zip(N, areas)]       # kN / mm^2 -> MPa
    delta = math.fsum(Ni * 1e3 * Li / (Ai * E * 1e-6) for Ni, Li, Ai in zip(N, lengths, areas))
    return _out(elongation_mm=_snap(delta, sum(abs(Ni) * 1e3 * Li / (Ai * E * 1e-6) for Ni, Li, Ai in zip(N, lengths, areas))),
                N_kN=N, stress_MPa=stress, max_abs_stress_MPa=max(abs(v) for v in stress))


def ref_axial_bar(lengths, areas, loads, E):
    """anastruct: one truss bar per segment (EA in N, lengths in mm), fixed at the left end, the loads at the joints;
    element axial forces and the free-end displacement from the finite-element solution."""
    ss, n = _bar_series_ana(lengths, [E * 1e-6 * A for A in areas], {i + 2: F * 1e3 for i, F in enumerate(loads)})
    N = [ss.get_element_results(element_id=i + 1)["Nmax"] / 1e3 for i in range(len(lengths))]
    scale = max(abs(v) for v in loads) or 1.0
    N = [_snap(v, scale) for v in N]
    d = -ss.get_node_results_system(node_id=n)["ux"]
    return {"elongation_mm": _snap(d, sum(abs(Ni) * 1e3 * Li / (Ai * E * 1e-6) for Ni, Li, Ai in zip(N, lengths, areas))),
            "N_kN": N, "stress_MPa": [Ni * 1e3 / Ai for Ni, Ai in zip(N, areas)], "max_abs_stress_MPa": max(abs(Ni * 1e3 / Ai) for Ni, Ai in zip(N, areas))}


_add("axial_bar_segments",
     name="Stepped bar under axial loads at its joints", topic="stress and strain / axial loading",
     purpose="Internal axial force, normal stress of every segment and the total elongation of a bar fixed at its left end, made of segments in series, with axial loads at the joints.",
     inputs=[_vec("lengths", "segment lengths in mm, from the fixed end (a list)"), _vec("areas", "segment cross-sectional areas in mm^2 (a list)"),
             _vec("loads", "axial load at the far end of each segment in kN, positive pulling away from the support (a list)"),
             _phys("E", "elastic_modulus", "Young's modulus (all segments)")],
     outputs=[_o("elongation_mm", "number", "total elongation of the bar in mm (negative = shortening)"), _o("N_kN", "number", "internal axial force of each segment in kN (tension positive)"),
              _o("stress_MPa", "number", "normal stress of each segment in MPa"), _o("max_abs_stress_MPa", "number", "largest stress magnitude in MPa")],
     primary="elongation_mm", conditions=["equal-length lists", "left end fixed, right end free", "linear elastic, no buckling"],
     method="N_i = sum of the loads beyond segment i; sigma_i = N_i/A_i; delta = sum N_i L_i/(A_i E)",
     latex=r"\delta = \sum_i \frac{N_iL_i}{A_iE},\quad \sigma_i = \frac{N_i}{A_i}",
     run=axial_bar,
     parse=lambda q: _pairs(q, r"axial bar segments|stepped bar", {"lengths": "lengths", "L": "lengths", "areas": "areas", "A": "areas", "loads": "loads", "P": "loads", "E": "E"},
                            phys={"E"}, vectors={"lengths", "areas", "loads"}),
     reference=ref_axial_bar,
     wrong=_safe(lambda lengths, areas, loads, E: (lambda N: _out(elongation_mm=sum(Ni * 1e3 * Li / (Ai * E * 1e-6) for Ni, Li, Ai in zip(N, lengths, areas)), N_kN=N,
                                                                stress_MPa=[Ni * 1e3 / Ai for Ni, Ai in zip(N, areas)], max_abs_stress_MPa=max(abs(Ni * 1e3 / Ai) for Ni, Ai in zip(N, areas))))(list(loads))),
     random_inputs=lambda rng: (lambda n: {"lengths": [float(rng.randint(100, 2000)) for _ in range(n)], "areas": [float(rng.randint(50, 5000)) for _ in range(n)],
                                           "loads": [round(rng.uniform(-80, 80), 2) for _ in range(n)], "E": rng.uniform(70e9, 210e9)})(rng.randint(2, 4)),
     templates=["Stepped bar: lengths = [300, 500], areas = [400, 200], loads = [-20, 30], E = 200 GPa"],
     examples=[{"inputs": {"lengths": "300, 500", "areas": "400, 200", "loads": "-20, 30", "E": 200e9},
                "outputs": {"N_kN": [10.0, 30.0], "stress_MPa": [25.0, 150.0], "elongation_mm": 10e3 * 300 / (400 * 200e3) + 30e3 * 500 / (200 * 200e3)}}],
     questions=[{"q": "Stepped bar: lengths = [300, 500], areas = [400, 200], loads = [-20, 30], E = 200 GPa", "outputs": {"N_kN": [10.0, 30.0], "stress_MPa": [25.0, 150.0]}},
                {"q": "Stepped bar: lengths = [300, 500], areas = [400], loads = [-20, 30], E = 200 GPa", "outcome": "INVALID_INPUT"}])


def bar_fixed_ends(P, L1, A1, E1, L2, A2, E2):
    _pos("P, L1, A1, E1, L2, A2 and E2", P, L1, A1, E1, L2, A2, E2)
    k1, k2 = A1 * E1 / L1, A2 * E2 / L2
    RA = P * k1 / (k1 + k2)
    RB = P - RA
    return _out(RA=RA, RB=RB, stress1_MPa=RA / A1 / 1e6, stress2_MPa=-RB / A2 / 1e6, displacement_mm=P / (k1 + k2) * 1e3)


def ref_bar_fixed_ends(P, L1, A1, E1, L2, A2, E2):
    """anastruct: two truss bars between two fixed supports with the load at the joint (finite-element solution)."""
    ss, n = _bar_series_ana([L1 * 1e3, L2 * 1e3], [A1 * E1, A2 * E2], {2: P}, fixed_both=True)
    N1, N2 = (ss.get_element_results(element_id=i)["Nmax"] for i in (1, 2))
    return {"RA": abs(N1), "RB": abs(N2), "stress1_MPa": N1 / A1 / 1e6, "stress2_MPa": N2 / A2 / 1e6,
            "displacement_mm": abs(ss.get_node_results_system(node_id=2)["ux"])}


_add("bar_fixed_at_both_ends",
     name="Bar fixed at both ends with an axial load at a step", topic="stress and strain / statically indeterminate members",
     purpose="Reactions, stresses in both parts and the displacement of the loaded joint of a two-part bar held between two rigid supports, loaded axially at the joint.",
     inputs=[_phys("P", "force", "axial load at the joint (towards end B)"), _phys("L1", "length", "length of part 1 (end A to the joint)"),
             _phys("A1", "area", "area of part 1"), _phys("E1", "elastic_modulus", "modulus of part 1"), _phys("L2", "length", "length of part 2"),
             _phys("A2", "area", "area of part 2"), _phys("E2", "elastic_modulus", "modulus of part 2")],
     outputs=[_o("RA", "force", "reaction at A"), _o("RB", "force", "reaction at B"), _o("stress1_MPa", "number", "stress in part 1 in MPa (tension positive)"),
              _o("stress2_MPa", "number", "stress in part 2 in MPa (compression negative)"), _o("displacement_mm", "number", "displacement of the joint in mm")],
     primary="RA", conditions=["rigid supports, no gap", "linear elastic, no buckling of the compressed part"],
     method="compatibility: the joint moves the same amount in both parts; RA = P k1/(k1 + k2), k = AE/L",
     latex=r"R_A = P\frac{k_1}{k_1+k_2},\quad k_i = \frac{A_iE_i}{L_i},\quad \delta = \frac{P}{k_1+k_2}",
     run=bar_fixed_ends,
     parse=lambda q: _pairs(q, r"bar fixed at both ends", {k: k for k in ("P", "L1", "A1", "E1", "L2", "A2", "E2")}, phys={"P", "L1", "A1", "E1", "L2", "A2", "E2"}),
     reference=ref_bar_fixed_ends,
     wrong=_safe(lambda P, L1, A1, E1, L2, A2, E2: (lambda RA: _out(RA=RA, RB=P - RA, stress1_MPa=RA / A1 / 1e6, stress2_MPa=-(P - RA) / A2 / 1e6,
                                                                  displacement_mm=RA * L1 / (A1 * E1) * 1e3))(P * L1 / (L1 + L2) if abs(L1 - L2) > 1e-3 * L1 else P * 0.3)),
     random_inputs=lambda rng: {"P": _lu(rng, 1e3, 5e5), "L1": round(rng.uniform(0.1, 2), 3), "A1": _lu(rng, 1e-5, 1e-2), "E1": rng.uniform(70e9, 210e9),
                                "L2": round(rng.uniform(0.1, 2), 3), "A2": _lu(rng, 1e-5, 1e-2), "E2": rng.uniform(70e9, 210e9)},
     templates=["Bar fixed at both ends: P = 60 kN, L1 = 0.3 m, A1 = 600 mm^2, E1 = 200 GPa, L2 = 0.5 m, A2 = 400 mm^2, E2 = 70 GPa"],
     examples=[{"inputs": {"P": 60e3, "L1": 0.4, "A1": 5e-4, "E1": 200e9, "L2": 0.4, "A2": 5e-4, "E2": 200e9}, "outputs": {"RA": 30e3, "RB": 30e3, "stress1_MPa": 60.0}}],
     questions=[{"q": "Bar fixed at both ends: P = 60 kN, L1 = 0.4 m, A1 = 500 mm^2, E1 = 200 GPa, L2 = 0.4 m, A2 = 500 mm^2, E2 = 200 GPa", "outputs": {"RA": 30e3, "stress2_MPa": -60.0}}])


# ================================================================================================ stress analysis
def _stress_scale(*v):
    return max([abs(x) for x in v] + [1e-300])


def plane_transform(sx, sy, txy, theta):
    _fin("every stress and the angle", sx, sy, txy, theta)
    c2, s2 = math.cos(2 * theta), math.sin(2 * theta)
    m, d = (sx + sy) / 2, (sx - sy) / 2
    a = m + d * c2 + txy * s2
    b = m - d * c2 - txy * s2
    t = -d * s2 + txy * c2
    sc = _stress_scale(sx, sy, txy)
    return _out(sx_t_MPa=_snap(a, sc) / 1e6, sy_t_MPa=_snap(b, sc) / 1e6, txy_t_MPa=_snap(t, sc) / 1e6)


def ref_plane_transform(sx, sy, txy, theta):
    """The stress tensor rotated as a matrix with numpy: sigma' = R sigma R^T."""
    import numpy as np
    c, s = math.cos(theta), math.sin(theta)
    R = np.array([[c, s], [-s, c]])
    S = R @ np.array([[sx, txy], [txy, sy]]) @ R.T
    sc = _stress_scale(sx, sy, txy)
    return {"sx_t_MPa": _snap(float(S[0, 0]), sc) / 1e6, "sy_t_MPa": _snap(float(S[1, 1]), sc) / 1e6, "txy_t_MPa": _snap(float(S[0, 1]), sc) / 1e6}


_STRESS_NAMES = {"sx": "sx", "sigma_x": "sx", "sy": "sy", "sigma_y": "sy", "sz": "sz", "sigma_z": "sz", "txy": "txy", "tau_xy": "txy",
                 "tyz": "tyz", "tau_yz": "tyz", "tzx": "tzx", "tau_zx": "tzx", "theta": "theta"}

_add("plane_stress_transformation",
     name="Plane stress transformation to rotated axes", topic="stress analysis / stress transformation",
     purpose="The normal and shear stresses on axes rotated counterclockwise by theta from x, for a plane state of stress.",
     inputs=[_phys("sx", "stress", "normal stress sigma_x"), _phys("sy", "stress", "normal stress sigma_y"), _phys("txy", "shear_stress", "shear stress tau_xy"),
             _phys("theta", "angle", "rotation of the axes, counterclockwise from x")],
     outputs=[_o("sx_t_MPa", "number", "sigma_x' in MPa"), _o("sy_t_MPa", "number", "sigma_y' in MPa"), _o("txy_t_MPa", "number", "tau_x'y' in MPa")],
     primary="sx_t_MPa", conditions=["plane stress", "tension positive, theta counterclockwise"],
     method="sigma_x' = (sx + sy)/2 + (sx - sy)/2 cos 2theta + txy sin 2theta; tau_x'y' = -(sx - sy)/2 sin 2theta + txy cos 2theta",
     latex=r"\sigma_{x'} = \frac{\sigma_x+\sigma_y}{2} + \frac{\sigma_x-\sigma_y}{2}\cos2\theta + \tau_{xy}\sin2\theta",
     run=plane_transform,
     parse=lambda q: _pairs(q, r"plane stress transformation|stress transformation", _STRESS_NAMES, phys={"sx", "sy", "txy", "theta"}),
     reference=ref_plane_transform,
     wrong=_safe(lambda sx, sy, txy, theta: plane_transform(sx, sy, -txy, theta)),
     random_inputs=lambda rng: {"sx": rng.uniform(-300e6, 300e6), "sy": rng.uniform(-300e6, 300e6), "txy": rng.uniform(-200e6, 200e6), "theta": _deg(rng.uniform(-90, 90))},
     templates=["Plane stress transformation: sx = 80 MPa, sy = -40 MPa, txy = 25 MPa, theta = 30 deg"],
     examples=[{"inputs": {"sx": 80e6, "sy": -40e6, "txy": 25e6, "theta": _deg(30)},
                "outputs": {"sx_t_MPa": 20 + 60 * 0.5 + 25 * math.sin(_deg(60)), "txy_t_MPa": -60 * math.sin(_deg(60)) + 25 * 0.5}}],
     questions=[{"q": "Plane stress transformation: sx = 80 MPa, sy = -40 MPa, txy = 25 MPa, theta = 30 deg",
                 "outputs": {"sx_t_MPa": 20 + 60 * 0.5 + 25 * math.sin(_deg(60))}},
                {"q": "Plane stress transformation: sx = 80 MPa, sy = -40 MPa, txy = 25 MPa", "outcome": "NEEDS_INPUT"}])


def principal_2d(sx, sy, txy):
    _fin("every stress", sx, sy, txy)
    sc = _stress_scale(sx, sy, txy)
    c, R = (sx + sy) / 2, math.hypot((sx - sy) / 2, txy)
    s1, s2 = _snap(c + R, sc), _snap(c - R, sc)
    th = 0.0 if R <= 1e-12 * sc else 0.5 * math.degrees(math.atan2(2 * txy, sx - sy))
    if th <= -90:
        th += 180
    tabs = max(s1 - s2, abs(s1), abs(s2)) / 2
    vm = math.sqrt(max(s1 * s1 - s1 * s2 + s2 * s2, 0.0))
    return _out(s1_MPa=s1 / 1e6, s2_MPa=s2 / 1e6, tau_max_inplane_MPa=_snap(R, sc) / 1e6, theta_p_deg=th, tau_abs_max_MPa=tabs / 1e6, von_mises_MPa=_snap(vm, sc) / 1e6)


def ref_principal_2d(sx, sy, txy):
    """numpy's symmetric eigen-solver on the 3 x 3 plane-stress tensor (sigma_z = 0): principal values, the direction
    of sigma_1 from its eigenvector, the absolute maximum shear from all three; von Mises from the deviator."""
    import numpy as np
    S = np.array([[sx, txy, 0.0], [txy, sy, 0.0], [0.0, 0.0, 0.0]])
    w, v = np.linalg.eigh(S[:2, :2])
    sc = _stress_scale(sx, sy, txy)
    s2, s1 = (_snap(float(x), sc) for x in w)
    R = (s1 - s2) / 2
    if R <= 1e-12 * sc:
        th = 0.0
    else:
        th = math.degrees(math.atan2(v[1, 1], v[0, 1]))
        th = th - 180 if th > 90 else (th + 180 if th <= -90 else th)
    allw = np.linalg.eigvalsh(S)
    dev = S - np.trace(S) / 3 * np.eye(3)
    vm = math.sqrt(1.5 * float(np.sum(dev * dev)))
    return {"s1_MPa": s1 / 1e6, "s2_MPa": s2 / 1e6, "tau_max_inplane_MPa": _snap(R, sc) / 1e6, "theta_p_deg": th,
            "tau_abs_max_MPa": float(allw[-1] - allw[0]) / 2 / 1e6, "von_mises_MPa": _snap(vm, sc) / 1e6}


_add("principal_stresses_2d",
     name="Principal stresses and maximum shear (Mohr's circle, plane stress)", topic="stress analysis / principal stresses",
     purpose="Principal stresses, their direction, the maximum in-plane and absolute shear stresses and the von Mises stress of a plane state of stress.",
     inputs=[_phys("sx", "stress", "normal stress sigma_x"), _phys("sy", "stress", "normal stress sigma_y"), _phys("txy", "shear_stress", "shear stress tau_xy")],
     outputs=[_o("s1_MPa", "number", "larger in-plane principal stress, MPa"), _o("s2_MPa", "number", "smaller in-plane principal stress, MPa"),
              _o("tau_max_inplane_MPa", "number", "maximum in-plane shear stress (radius of Mohr's circle), MPa"),
              _o("theta_p_deg", "number", "angle from x to the sigma_1 direction, degrees in (-90, 90] (0 when the circle is a point)"),
              _o("tau_abs_max_MPa", "number", "absolute maximum shear stress including sigma_z = 0, MPa"), _o("von_mises_MPa", "number", "von Mises equivalent stress, MPa")],
     primary="s1_MPa", conditions=["plane stress (sigma_z = 0)", "tension positive"],
     method="sigma_1,2 = (sx + sy)/2 +- R, R = sqrt(((sx - sy)/2)^2 + txy^2); tan 2theta_p = 2 txy/(sx - sy); tau_abs = max(s1 - s2, |s1|, |s2|)/2",
     latex=r"\sigma_{1,2} = \frac{\sigma_x+\sigma_y}{2} \pm \sqrt{\Big(\frac{\sigma_x-\sigma_y}{2}\Big)^2 + \tau_{xy}^2},\quad \tan2\theta_p = \frac{2\tau_{xy}}{\sigma_x-\sigma_y}",
     run=principal_2d,
     parse=lambda q: _pairs(q, r"principal stresses(?: 2d)?|mohr'?s circle", _STRESS_NAMES, phys={"sx", "sy", "txy"}),
     reference=ref_principal_2d,
     wrong=_safe(lambda sx, sy, txy: (lambda c, R: _out(s1_MPa=(c + R) / 1e6, s2_MPa=(c - R) / 1e6, tau_max_inplane_MPa=R / 1e6, theta_p_deg=math.degrees(math.atan2(txy, sx - sy)),
                                                      tau_abs_max_MPa=R / 1e6, von_mises_MPa=abs(c + R) / 1e6))((sx + sy) / 2, math.hypot(sx - sy, txy) / 2)),
     random_inputs=lambda rng: {"sx": rng.uniform(-300e6, 300e6), "sy": rng.uniform(-300e6, 300e6), "txy": rng.uniform(-200e6, 200e6)},
     templates=["Principal stresses: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa"],
     examples=[{"inputs": {"sx": 50e6, "sy": -10e6, "txy": 40e6}, "outputs": {"s1_MPa": 70.0, "s2_MPa": -30.0, "tau_max_inplane_MPa": 50.0,
                                                                          "theta_p_deg": 0.5 * math.degrees(math.atan2(80, 60)), "tau_abs_max_MPa": 50.0}},
               {"inputs": {"sx": 100e6, "sy": 0.0, "txy": 0.0}, "outputs": {"s1_MPa": 100.0, "s2_MPa": 0.0, "theta_p_deg": 0.0, "tau_abs_max_MPa": 50.0, "von_mises_MPa": 100.0}}],
     questions=[{"q": "Principal stresses: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa", "outputs": {"s1_MPa": 70.0, "s2_MPa": -30.0}},
                {"q": "Mohr's circle: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa", "outputs": {"tau_max_inplane_MPa": 50.0}}])


def _jacobi3(S):
    """Eigenvalues of a symmetric 3 x 3 matrix by cyclic Jacobi rotations (accurate to rounding, also for repeated
    eigenvalues), ascending."""
    a = [row[:] for row in S]
    for _sweep in range(60):
        off = math.sqrt(sum(a[i][j] ** 2 for i in range(3) for j in range(3) if i != j))
        nrm = math.sqrt(sum(a[i][j] ** 2 for i in range(3) for j in range(3)))
        if off <= 1e-17 * nrm or off == 0:
            break
        for p, q in ((0, 1), (0, 2), (1, 2)):
            if a[p][q] == 0:
                continue
            th = (a[q][q] - a[p][p]) / (2 * a[p][q])
            t = math.copysign(1.0, th) / (abs(th) + math.sqrt(th * th + 1))
            c = 1 / math.sqrt(t * t + 1)
            s = t * c
            for k in range(3):
                akp, akq = a[k][p], a[k][q]
                a[k][p], a[k][q] = c * akp - s * akq, s * akp + c * akq
            for k in range(3):
                apk, aqk = a[p][k], a[q][k]
                a[p][k], a[q][k] = c * apk - s * aqk, s * apk + c * aqk
    return sorted(a[i][i] for i in range(3))


def principal_3d(sx, sy, sz, txy, tyz, tzx):
    _fin("every stress component", sx, sy, sz, txy, tyz, tzx)
    sc = _stress_scale(sx, sy, sz, txy, tyz, tzx)
    S = [[sx, txy, tzx], [txy, sy, tyz], [tzx, tyz, sz]]
    s3, s2, s1 = (_snap(v, sc) for v in _jacobi3(S))
    vm = math.sqrt(max(0.5 * ((sx - sy) ** 2 + (sy - sz) ** 2 + (sz - sx) ** 2) + 3 * (txy * txy + tyz * tyz + tzx * tzx), 0.0))
    I1 = sx + sy + sz
    return _out(s1_MPa=s1 / 1e6, s2_MPa=s2 / 1e6, s3_MPa=s3 / 1e6, tau_max_MPa=_snap((s1 - s3) / 2, sc) / 1e6, von_mises_MPa=_snap(vm, sc) / 1e6,
                hydrostatic_MPa=_snap(I1 / 3, sc) / 1e6, tau_oct_MPa=_snap(math.sqrt(2) / 3 * vm, sc) / 1e6)


def ref_principal_3d(sx, sy, sz, txy, tyz, tzx):
    """numpy's symmetric eigen-solver (LAPACK) on the stress tensor; von Mises and octahedral shear from the
    deviatoric tensor's contraction."""
    import numpy as np
    S = np.array([[sx, txy, tzx], [txy, sy, tyz], [tzx, tyz, sz]], dtype=float)
    sc = _stress_scale(sx, sy, sz, txy, tyz, tzx)
    w = np.linalg.eigvalsh(S)
    s3, s2, s1 = (_snap(float(v), sc) for v in w)
    dev = S - np.trace(S) / 3 * np.eye(3)
    J2 = 0.5 * float(np.sum(dev * dev))
    vm = math.sqrt(3 * J2)
    return {"s1_MPa": s1 / 1e6, "s2_MPa": s2 / 1e6, "s3_MPa": s3 / 1e6, "tau_max_MPa": _snap((s1 - s3) / 2, sc) / 1e6, "von_mises_MPa": _snap(vm, sc) / 1e6,
            "hydrostatic_MPa": _snap(float(np.trace(S)) / 3, sc) / 1e6, "tau_oct_MPa": _snap(math.sqrt(2 * J2 / 3), sc) / 1e6}


_add("principal_stresses_3d",
     name="Principal stresses of a three-dimensional stress state", topic="stress analysis / principal stresses",
     purpose="The three principal stresses, the absolute maximum shear, von Mises, hydrostatic and octahedral shear stresses of a general 3D state of stress.",
     inputs=[_phys("sx", "stress", "sigma_x"), _phys("sy", "stress", "sigma_y"), _phys("sz", "stress", "sigma_z"),
             _phys("txy", "shear_stress", "tau_xy"), _phys("tyz", "shear_stress", "tau_yz"), _phys("tzx", "shear_stress", "tau_zx")],
     outputs=[_o("s1_MPa", "number", "largest principal stress, MPa"), _o("s2_MPa", "number", "intermediate principal stress, MPa"), _o("s3_MPa", "number", "smallest principal stress, MPa"),
              _o("tau_max_MPa", "number", "absolute maximum shear stress (s1 - s3)/2, MPa"), _o("von_mises_MPa", "number", "von Mises stress, MPa"),
              _o("hydrostatic_MPa", "number", "mean (hydrostatic) stress, MPa"), _o("tau_oct_MPa", "number", "octahedral shear stress, MPa")],
     primary="s1_MPa", conditions=["symmetric stress tensor", "tension positive"],
     method="eigenvalues of the stress tensor by Jacobi rotations; sigma_vm = sqrt(((sx-sy)^2 + (sy-sz)^2 + (sz-sx)^2)/2 + 3(txy^2 + tyz^2 + tzx^2)); tau_oct = sqrt(2)/3 sigma_vm",
     latex=r"\det(\boldsymbol\sigma - \sigma\mathbf I) = 0,\quad \tau_{max} = \frac{\sigma_1-\sigma_3}{2},\quad \tau_{oct} = \frac{\sqrt2}{3}\sigma_{vm}",
     run=principal_3d,
     parse=lambda q: _pairs(q, r"principal stresses 3d|3d principal stresses|three[- ]dimensional principal stresses", _STRESS_NAMES, phys={"sx", "sy", "sz", "txy", "tyz", "tzx"}),
     reference=ref_principal_3d,
     wrong=_safe(lambda sx, sy, sz, txy, tyz, tzx: (lambda r: dict(r, s1_MPa=max(sx, sy, sz) / 1e6, s3_MPa=min(sx, sy, sz) / 1e6))(principal_3d(sx, sy, sz, txy, tyz, tzx))),
     random_inputs=lambda rng: {k: rng.uniform(-300e6, 300e6) for k in ("sx", "sy", "sz")} | {k: rng.uniform(-150e6, 150e6) for k in ("txy", "tyz", "tzx")},
     templates=["Principal stresses 3D: sx = 60 MPa, sy = 20 MPa, sz = -10 MPa, txy = 30 MPa, tyz = 0 MPa, tzx = 15 MPa"],
     examples=[{"inputs": {"sx": 100e6, "sy": 0.0, "sz": 0.0, "txy": 0.0, "tyz": 0.0, "tzx": 0.0}, "outputs": {"s1_MPa": 100.0, "s2_MPa": 0.0, "s3_MPa": 0.0, "tau_max_MPa": 50.0, "von_mises_MPa": 100.0}},
               {"inputs": {"sx": 0.0, "sy": 0.0, "sz": 0.0, "txy": 50e6, "tyz": 0.0, "tzx": 0.0}, "outputs": {"s1_MPa": 50.0, "s2_MPa": 0.0, "s3_MPa": -50.0, "von_mises_MPa": 50 * math.sqrt(3)}}],
     questions=[{"q": "Principal stresses 3D: sx = 100 MPa, sy = 0 MPa, sz = 0 MPa, txy = 0 MPa, tyz = 0 MPa, tzx = 0 MPa", "outputs": {"s1_MPa": 100.0, "s3_MPa": 0.0}},
                {"q": "Principal stresses 3D: sx = 100 MPa, sy = 0 MPa, sz = 0 MPa", "outcome": "NEEDS_INPUT"}])


def yield_safety(Sy, sx, sy=0.0, sz=0.0, txy=0.0, tyz=0.0, tzx=0.0):
    _pos("the yield strength Sy", Sy)
    r = principal_3d(sx, sy, sz, txy, tyz, tzx)
    vm, tr = r["von_mises_MPa"] * 1e6, (r["s1_MPa"] - r["s3_MPa"]) * 1e6
    if vm <= 0:
        raise ValueError("the stress state is purely hydrostatic: neither criterion predicts yielding")
    return _out(von_mises_MPa=vm / 1e6, tresca_MPa=tr / 1e6, n_von_mises=Sy / vm, n_tresca=Sy / tr)


def ref_yield_safety(Sy, sx, sy=0.0, sz=0.0, txy=0.0, tyz=0.0, tzx=0.0):
    """numpy: the distortion energy from the deviatoric stress tensor (sqrt(3 J2)); Tresca from LAPACK eigenvalues."""
    import numpy as np
    S = np.array([[sx, txy, tzx], [txy, sy, tyz], [tzx, tyz, sz]], dtype=float)
    dev = S - np.trace(S) / 3 * np.eye(3)
    vm = math.sqrt(1.5 * float(np.sum(dev * dev)))
    w = np.linalg.eigvalsh(S)
    tr = float(w[-1] - w[0])
    return {"von_mises_MPa": vm / 1e6, "tresca_MPa": tr / 1e6, "n_von_mises": Sy / vm, "n_tresca": Sy / tr}


_add("yield_criteria_safety_factor",
     name="Von Mises and Tresca equivalent stresses and safety factors", topic="failure theories / ductile materials",
     purpose="Equivalent stresses by the distortion-energy (von Mises) and maximum-shear (Tresca) criteria and the corresponding factors of safety against yielding.",
     inputs=[_phys("Sy", "stress", "yield strength"), _phys("sx", "stress", "sigma_x"), _phys("sy", "stress", "sigma_y (default 0)", required=False),
             _phys("sz", "stress", "sigma_z (default 0)", required=False), _phys("txy", "shear_stress", "tau_xy (default 0)", required=False),
             _phys("tyz", "shear_stress", "tau_yz (default 0)", required=False), _phys("tzx", "shear_stress", "tau_zx (default 0)", required=False)],
     outputs=[_o("von_mises_MPa", "number", "von Mises stress, MPa"), _o("tresca_MPa", "number", "Tresca equivalent stress (s1 - s3), MPa"),
              _o("n_von_mises", "number", "safety factor by distortion energy"), _o("n_tresca", "number", "safety factor by maximum shear")],
     primary="n_von_mises", conditions=["Sy > 0", "not a purely hydrostatic stress", "ductile, isotropic material"],
     method="sigma_vm = sqrt(((sx-sy)^2 + (sy-sz)^2 + (sz-sx)^2)/2 + 3(txy^2+tyz^2+tzx^2)), sigma_T = s1 - s3, n = Sy/sigma_eq",
     latex=r"\sigma_{vm} = \sqrt{\tfrac12\big[(\sigma_1-\sigma_2)^2+(\sigma_2-\sigma_3)^2+(\sigma_3-\sigma_1)^2\big]},\quad n = \frac{S_y}{\sigma_{eq}}",
     run=yield_safety,
     parse=lambda q: _pairs(q, r"yield safety factor|von mises and tresca|yield criteria", dict(_STRESS_NAMES, Sy="Sy"), phys={"Sy", "sx", "sy", "sz", "txy", "tyz", "tzx"}),
     reference=ref_yield_safety,
     wrong=_safe(lambda Sy, sx, sy=0.0, sz=0.0, txy=0.0, tyz=0.0, tzx=0.0: (lambda vm: _out(von_mises_MPa=vm / 1e6, tresca_MPa=vm / 1e6, n_von_mises=Sy / vm, n_tresca=Sy / vm))(
         math.sqrt(sx * sx + sy * sy + sz * sz + txy * txy + tyz * tyz + tzx * tzx))),
     random_inputs=lambda rng: {"Sy": rng.uniform(200e6, 1200e6), **{k: rng.uniform(-300e6, 300e6) for k in ("sx", "sy", "sz")}, **{k: rng.uniform(-150e6, 150e6) for k in ("txy", "tyz", "tzx")}},
     templates=["Yield safety factor: Sy = 350 MPa, sx = 120 MPa, sy = -40 MPa, txy = 60 MPa"],
     examples=[{"inputs": {"Sy": 350e6, "sx": 120e6, "sy": -40e6, "txy": 60e6},
                "outputs": {"von_mises_MPa": math.sqrt(120 ** 2 + 40 ** 2 + 120 * 40 + 3 * 3600), "tresca_MPa": 2 * math.hypot(80, 60)}}],
     questions=[{"q": "Yield safety factor: Sy = 350 MPa, sx = 120 MPa, sy = -40 MPa, txy = 60 MPa", "outputs": {"tresca_MPa": 200.0, "n_tresca": 1.75}},
                {"q": "Yield safety factor: Sy = 350 MPa, sx = 100 MPa, sy = 100 MPa, sz = 100 MPa", "outcome": "INVALID_INPUT"}])


def brittle_mohr(sA, sB, Sut, Suc):
    _pos("Sut and Suc", Sut, Suc)
    _fin("sA and sB", sA, sB)
    s1, s3 = max(sA, sB), min(sA, sB)
    if s1 == 0 and s3 == 0:
        raise ValueError("the stress state is zero")
    mns = min((Sut / s1) if s1 > 0 else math.inf, (Suc / -s3) if s3 < 0 else math.inf)
    bcm = 1.0 / (max(s1, 0.0) / Sut - min(s3, 0.0) / Suc)
    if s3 >= 0:
        mm = Sut / s1
    elif s1 <= 0:
        mm = Suc / -s3
    elif -s3 <= s1:
        mm = Sut / s1
    else:
        mm = 1.0 / ((Suc - Sut) * s1 / (Suc * Sut) - s3 / Suc)
    return _out(n_max_normal=mns, n_coulomb_mohr=bcm, n_modified_mohr=mm)


def ref_brittle_mohr(sA, sB, Sut, Suc):
    """Geometry with shapely: each criterion's safe region drawn as a polygon in the (sigma_A, sigma_B) plane and the
    load line from the origin intersected with its boundary; n = distance to the boundary / distance to the load."""
    from shapely.geometry import LineString, Polygon
    BIG = 10.0
    regions = {
        "n_max_normal": [(Sut, Sut), (-Suc, Sut), (-Suc, -Suc), (Sut, -Suc)],
        "n_coulomb_mohr": [(Sut, Sut), (0, Sut), (-Suc, 0), (-Suc, -Suc), (0, -Suc), (Sut, 0)],
        "n_modified_mohr": [(Sut, Sut), (-Sut, Sut), (-Suc, 0), (-Suc, -Suc), (0, -Suc), (Sut, -Sut)],
    }
    L = math.hypot(sA, sB)
    far = (sA / L * BIG * max(Sut, Suc), sB / L * BIG * max(Sut, Suc))
    out = {}
    for k, pts in regions.items():
        hit = Polygon(pts).exterior.intersection(LineString([(0, 0), far]))
        pt = hit if hit.geom_type == "Point" else list(hit.geoms)[0]
        out[k] = math.hypot(pt.x, pt.y) / L
    return out


_add("brittle_failure_mohr",
     name="Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr)", topic="failure theories / brittle materials",
     purpose="Factors of safety of a brittle material under plane stress by the maximum-normal-stress, brittle Coulomb-Mohr and modified-Mohr criteria (proportional loading).",
     inputs=[_phys("sA", "stress", "first in-plane principal stress"), _phys("sB", "stress", "second in-plane principal stress"),
             _phys("Sut", "stress", "ultimate tensile strength"), _phys("Suc", "stress", "ultimate compressive strength (positive number)")],
     outputs=[_o("n_max_normal", "number", "safety factor, maximum normal stress theory"), _o("n_coulomb_mohr", "number", "safety factor, brittle Coulomb-Mohr"),
              _o("n_modified_mohr", "number", "safety factor, modified Mohr")],
     primary="n_coulomb_mohr", conditions=["plane stress (third principal stress zero)", "Sut, Suc > 0 (Suc given as a magnitude)", "proportional loading"],
     method="BCM: 1/n = s1/Sut - s3/Suc (with s1 = max(sA, sB, 0), s3 = min(sA, sB, 0)); MM: 1/n = (Suc - Sut) sA/(Suc Sut) - sB/Suc when sA > 0 > sB and |sB| > sA; MNS: n = min(Sut/s1, Suc/|s3|)",
     latex=r"\frac{\sigma_1}{S_{ut}} - \frac{\sigma_3}{S_{uc}} = \frac1n,\quad \frac{(S_{uc}-S_{ut})\sigma_A}{S_{uc}S_{ut}} - \frac{\sigma_B}{S_{uc}} = \frac1n",
     run=brittle_mohr,
     parse=lambda q: _pairs(q, r"brittle failure|coulomb[- ]mohr|modified mohr", {"sA": "sA", "sB": "sB", "Sut": "Sut", "Suc": "Suc"}, phys={"sA", "sB", "Sut", "Suc"}),
     reference=ref_brittle_mohr,
     wrong=_safe(lambda sA, sB, Sut, Suc: _out(n_max_normal=Sut / max(abs(sA), abs(sB)), n_coulomb_mohr=1 / (max(sA, sB, 0) / Sut + abs(min(sA, sB, 0)) / Sut),
                                             n_modified_mohr=1 / (max(sA, sB, 0) / Sut + abs(min(sA, sB, 0)) / Suc))),
     random_inputs=lambda rng: (lambda Sut: {"sA": rng.uniform(-400e6, 200e6), "sB": rng.uniform(-400e6, 200e6), "Sut": Sut, "Suc": Sut * rng.uniform(2.5, 4.5)})(rng.uniform(100e6, 350e6)),
     templates=["Brittle failure: sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = 750 MPa"],
     examples=[{"inputs": {"sA": 90e6, "sB": -50e6, "Sut": 210e6, "Suc": 750e6},
                "outputs": {"n_coulomb_mohr": 1 / (90 / 210 + 50 / 750), "n_modified_mohr": 210 / 90, "n_max_normal": 210 / 90}},
               {"inputs": {"sA": 50e6, "sB": -300e6, "Sut": 210e6, "Suc": 750e6},
                "outputs": {"n_modified_mohr": 1 / ((750 - 210) * 50 / (750 * 210) + 300 / 750), "n_coulomb_mohr": 1 / (50 / 210 + 300 / 750)}}],
     questions=[{"q": "Brittle failure: sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = 750 MPa", "outputs": {"n_coulomb_mohr": 1 / (90 / 210 + 50 / 750)}},
                {"q": "Brittle failure: sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = -750 MPa", "outcome": "INVALID_INPUT"}])


def rosette(ea, eb, ec, E, nu, layout):
    _fin("the three gauge strains", ea, eb, ec)
    _pos("E", E)
    if not 0 <= nu < 0.5:
        raise ValueError("Poisson's ratio must lie in [0, 0.5)")
    if layout == "rectangular":
        ex, ey, gxy = ea, ec, 2 * eb - ea - ec
    else:
        ex, ey, gxy = ea, (2 * eb + 2 * ec - ea) / 3, 2 * (eb - ec) / math.sqrt(3)
    sc = max(abs(ea), abs(eb), abs(ec)) or 1.0
    c, R = (ex + ey) / 2, math.hypot((ex - ey) / 2, gxy / 2)
    e1, e2 = _snap(c + R, sc), _snap(c - R, sc)
    th = 0.0 if R <= 1e-12 * sc else 0.5 * math.degrees(math.atan2(gxy, ex - ey))
    if th <= -90:
        th += 180
    f = E / (1 - nu * nu) * 1e-6 / 1e6                      # microstrain -> MPa
    return _out(ex=_snap(ex, sc), ey=_snap(ey, sc), gxy=_snap(gxy, sc), e1=e1, e2=e2, theta_p_deg=th,
                s1_MPa=_snap(f * (e1 + nu * e2), sc * f), s2_MPa=_snap(f * (e2 + nu * e1), sc * f))


def ref_rosette(ea, eb, ec, E, nu, layout):
    """numpy: the strain components from the three gauge equations eps(phi) = ex cos^2 + ey sin^2 + gxy sin cos
    (least squares), principal strains and direction from eigh of the strain tensor, stresses by solving the
    plane-stress compliance relations."""
    import numpy as np
    angs = [0.0, 45.0, 90.0] if layout == "rectangular" else [0.0, 60.0, 120.0]
    A = np.array([[math.cos(math.radians(a)) ** 2, math.sin(math.radians(a)) ** 2, math.sin(math.radians(a)) * math.cos(math.radians(a))] for a in angs])
    ex, ey, g = np.linalg.lstsq(A, np.array([ea, eb, ec], dtype=float), rcond=None)[0]
    sc = max(abs(ea), abs(eb), abs(ec)) or 1.0
    w, v = np.linalg.eigh(np.array([[ex, g / 2], [g / 2, ey]]))
    e2, e1 = (_snap(float(x), sc) for x in w)
    if (e1 - e2) / 2 <= 1e-12 * sc:
        th = 0.0
    else:
        th = math.degrees(math.atan2(v[1, 1], v[0, 1]))
        th = th - 180 if th > 90 else (th + 180 if th <= -90 else th)
    C = np.array([[1.0, -nu], [-nu, 1.0]]) / E
    s1, s2 = np.linalg.solve(C, np.array([e1, e2]) * 1e-6) / 1e6
    f = E / (1 - nu * nu) * 1e-12
    return {"ex": _snap(float(ex), sc), "ey": _snap(float(ey), sc), "gxy": _snap(float(g), sc), "e1": e1, "e2": e2, "theta_p_deg": th,
            "s1_MPa": _snap(float(s1), sc * f), "s2_MPa": _snap(float(s2), sc * f)}


_add("strain_rosette",
     name="Strain rosette: strains, principal strains and principal stresses", topic="stress analysis / strain measurement",
     purpose="From the readings of a three-gauge rosette (rectangular 0/45/90 or delta 0/60/120 degrees): the strain components, the principal strains and their direction, and the principal stresses (plane stress).",
     inputs=[_numin("ea", "strain of gauge a (at 0 degrees), microstrain"), _numin("eb", "strain of gauge b (45 or 60 degrees), microstrain"),
             _numin("ec", "strain of gauge c (90 or 120 degrees), microstrain"), _phys("E", "elastic_modulus", "Young's modulus"), _numin("nu", "Poisson's ratio"),
             _choice("layout", ["rectangular", "delta"], "rosette layout: rectangular (0/45/90) or delta (0/60/120)")],
     outputs=[_o("ex", "number", "strain eps_x, microstrain"), _o("ey", "number", "strain eps_y, microstrain"), _o("gxy", "number", "shear strain gamma_xy, microstrain"),
              _o("e1", "number", "larger principal strain, microstrain"), _o("e2", "number", "smaller principal strain, microstrain"),
              _o("theta_p_deg", "number", "angle from gauge a to the eps_1 direction, degrees in (-90, 90]"),
              _o("s1_MPa", "number", "larger principal stress, MPa"), _o("s2_MPa", "number", "smaller principal stress, MPa")],
     primary="e1", conditions=["gauge angles measured counterclockwise from gauge a", "plane stress, linear elastic isotropic", "0 <= nu < 0.5"],
     method="rectangular: ex = ea, ey = ec, gxy = 2eb - ea - ec; delta: ex = ea, ey = (2eb + 2ec - ea)/3, gxy = 2(eb - ec)/sqrt(3); e1,2 = (ex+ey)/2 +- sqrt(((ex-ey)/2)^2 + (gxy/2)^2); s1 = E(e1 + nu e2)/(1 - nu^2)",
     latex=r"\varepsilon_{1,2} = \frac{\varepsilon_x+\varepsilon_y}{2} \pm \sqrt{\Big(\frac{\varepsilon_x-\varepsilon_y}{2}\Big)^2 + \Big(\frac{\gamma_{xy}}{2}\Big)^2},\quad \sigma_1 = \frac{E(\varepsilon_1+\nu\varepsilon_2)}{1-\nu^2}",
     run=rosette,
     parse=lambda q: _pairs(q, r"strain rosette", {"ea": "ea", "eb": "eb", "ec": "ec", "E": "E", "nu": "nu", "layout": "layout", "type": "layout"},
                            phys={"E"}, words={"layout"}),
     reference=ref_rosette,
     wrong=_safe(lambda ea, eb, ec, E, nu, layout: rosette(ea, eb, ec, E, nu, "delta" if layout == "rectangular" else "rectangular")),
     random_inputs=lambda rng: {"ea": rng.uniform(-1500, 1500), "eb": rng.uniform(-1500, 1500), "ec": rng.uniform(-1500, 1500), "E": rng.uniform(70e9, 210e9),
                                "nu": rng.uniform(0.2, 0.35), "layout": rng.choice(["rectangular", "delta"])},
     templates=["Strain rosette: ea = 400, eb = 300, ec = -100, E = 200 GPa, nu = 0.3, layout = rectangular"],
     examples=[{"inputs": {"ea": 400.0, "eb": 300.0, "ec": -100.0, "E": 200e9, "nu": 0.3, "layout": "rectangular"},
                "outputs": {"ex": 400.0, "ey": -100.0, "gxy": 300.0, "e1": 150 + math.hypot(250, 150), "e2": 150 - math.hypot(250, 150),
                            "s1_MPa": 200e3 / 0.91 * ((150 + math.hypot(250, 150)) + 0.3 * (150 - math.hypot(250, 150))) * 1e-6}}],
     questions=[{"q": "Strain rosette: ea = 400, eb = 300, ec = -100, E = 200 GPa, nu = 0.3, layout = rectangular", "outputs": {"gxy": 300.0, "e1": 150 + math.hypot(250, 150)}},
                {"q": "Strain rosette: ea = 400, eb = 300, ec = -100, E = 200 GPa, nu = 0.3, layout = star", "outcome": "INVALID_INPUT"}])


def hooke_3d(sx, sy, sz, txy, tyz, tzx, E, nu):
    _fin("every stress", sx, sy, sz, txy, tyz, tzx)
    _pos("E", E)
    if not -1 < nu < 0.5:
        raise ValueError("Poisson's ratio must lie between -1 and 0.5")
    G = E / (2 * (1 + nu))
    sc = _stress_scale(sx, sy, sz, txy, tyz, tzx) / E * 1e6
    ue = lambda v: _snap(v * 1e6, sc)
    ex = (sx - nu * (sy + sz)) / E
    ey = (sy - nu * (sz + sx)) / E
    ez = (sz - nu * (sx + sy)) / E
    return _out(ex=ue(ex), ey=ue(ey), ez=ue(ez), gxy=ue(txy / G), gyz=ue(tyz / G), gzx=ue(tzx / G), volumetric=ue(ex + ey + ez))


def ref_hooke_3d(sx, sy, sz, txy, tyz, tzx, E, nu):
    """numpy: the isotropic 6 x 6 stiffness matrix from the Lame constants, solved for the engineering strains."""
    import numpy as np
    lam, mu = E * nu / ((1 + nu) * (1 - 2 * nu)), E / (2 * (1 + nu))
    C = np.zeros((6, 6))
    C[:3, :3] = lam
    C[np.arange(3), np.arange(3)] += 2 * mu
    C[3:, 3:] = np.eye(3) * mu
    e = np.linalg.solve(C, np.array([sx, sy, sz, txy, tyz, tzx], dtype=float))
    sc = _stress_scale(sx, sy, sz, txy, tyz, tzx) / E * 1e6
    ue = lambda v: _snap(float(v) * 1e6, sc)
    return {"ex": ue(e[0]), "ey": ue(e[1]), "ez": ue(e[2]), "gxy": ue(e[3]), "gyz": ue(e[4]), "gzx": ue(e[5]), "volumetric": ue(e[0] + e[1] + e[2])}


_add("generalized_hookes_law",
     name="Generalized Hooke's law: strains from a 3D stress state", topic="stress and strain / constitutive law",
     purpose="The six strain components and the volumetric strain of an isotropic linear-elastic material under a general three-dimensional stress state.",
     inputs=[_phys("sx", "stress", "sigma_x"), _phys("sy", "stress", "sigma_y"), _phys("sz", "stress", "sigma_z"), _phys("txy", "shear_stress", "tau_xy"),
             _phys("tyz", "shear_stress", "tau_yz"), _phys("tzx", "shear_stress", "tau_zx"), _phys("E", "elastic_modulus", "Young's modulus"), _numin("nu", "Poisson's ratio")],
     outputs=[_o("ex", "number", "eps_x, microstrain"), _o("ey", "number", "eps_y, microstrain"), _o("ez", "number", "eps_z, microstrain"),
              _o("gxy", "number", "gamma_xy, microstrain"), _o("gyz", "number", "gamma_yz, microstrain"), _o("gzx", "number", "gamma_zx, microstrain"),
              _o("volumetric", "number", "volumetric strain eps_x + eps_y + eps_z, microstrain")],
     primary="ex", conditions=["isotropic, linear elastic", "-1 < nu < 0.5", "small strains"],
     method="eps_x = (sx - nu(sy + sz))/E (and cyclic), gamma = tau/G with G = E/(2(1 + nu))",
     latex=r"\varepsilon_x = \frac{1}{E}\big[\sigma_x - \nu(\sigma_y+\sigma_z)\big],\quad \gamma_{xy} = \frac{\tau_{xy}}{G},\quad G = \frac{E}{2(1+\nu)}",
     run=hooke_3d,
     parse=lambda q: _pairs(q, r"generalized hooke'?s law|hooke'?s law 3d", dict(_STRESS_NAMES, E="E", nu="nu"), phys={"sx", "sy", "sz", "txy", "tyz", "tzx", "E"}),
     reference=ref_hooke_3d,
     wrong=_safe(lambda sx, sy, sz, txy, tyz, tzx, E, nu: (lambda r: dict(r, ex=(sx - nu * (sy - sz)) / E * 1e6))(hooke_3d(sx, sy, sz, txy, tyz, tzx, E, nu))),
     random_inputs=lambda rng: {**{k: rng.uniform(-300e6, 300e6) for k in ("sx", "sy", "sz")}, **{k: rng.uniform(-150e6, 150e6) for k in ("txy", "tyz", "tzx")},
                                "E": rng.uniform(50e9, 400e9), "nu": rng.uniform(0.1, 0.45)},
     templates=["Generalized Hooke's law: sx = 100 MPa, sy = 50 MPa, sz = 0 MPa, txy = 30 MPa, tyz = 0 MPa, tzx = 0 MPa, E = 200 GPa, nu = 0.3"],
     examples=[{"inputs": {"sx": 100e6, "sy": 50e6, "sz": 0.0, "txy": 30e6, "tyz": 0.0, "tzx": 0.0, "E": 200e9, "nu": 0.3},
                "outputs": {"ex": (100 - 15) / 200e3 * 1e6, "ez": -0.3 * 150 / 200e3 * 1e6, "gxy": 30 / (200e3 / 2.6) * 1e6, "gyz": 0.0}}],
     questions=[{"q": "Generalized Hooke's law: sx = 100 MPa, sy = 50 MPa, sz = 0 MPa, txy = 30 MPa, tyz = 0 MPa, tzx = 0 MPa, E = 200 GPa, nu = 0.3",
                 "outputs": {"ex": 425.0}},
                {"q": "Generalized Hooke's law: sx = 100 MPa, sy = 50 MPa, sz = 0 MPa, txy = 30 MPa, tyz = 0 MPa, tzx = 0 MPa, E = 200 GPa, nu = 0.6", "outcome": "INVALID_INPUT"}])


# ================================================================================================ pressure vessels
def thin_vessel(p, r, t, shape):
    _pos("p, r and t", p, r, t)
    if r / t < 10:
        raise ValueError("r/t is below 10: the wall is not thin (use the thick-walled cylinder)")
    if shape == "cylinder":
        sh, sl = p * r / t, p * r / (2 * t)
    else:
        sh = sl = p * r / (2 * t)
    return _out(hoop_MPa=sh / 1e6, longitudinal_MPa=sl / 1e6, tau_inplane_MPa=(sh - sl) / 2 / 1e6, tau_abs_MPa=sh / 2 / 1e6)


def ref_thin_vessel(p, r, t, shape):
    """Free-body equilibrium by quadrature (scipy): the pressure resultant on a half-cylinder strip (integral of
    p r sin(theta)) carried by two wall sections, and the pressure resultant on an end cap (or a hemisphere's
    projected disk) carried by the wall's circumference 2 pi r t."""
    from scipy.integrate import quad
    cap = _quad_alg(lambda y: 2.0 * p, -r, r, 0.5)                 # pressure force on the disk of radius r
    sl = cap / (2 * math.pi * r * t)
    if shape == "cylinder":
        strip = quad(lambda th: p * r * math.sin(th), 0.0, math.pi, epsabs=0, epsrel=1e-13)[0]
        sh = strip / (2 * t)
    else:
        sh = sl
    return {"hoop_MPa": sh / 1e6, "longitudinal_MPa": sl / 1e6, "tau_inplane_MPa": (sh - sl) / 2 / 1e6, "tau_abs_MPa": sh / 2 / 1e6}


_add("thin_walled_pressure_vessel",
     name="Thin-walled pressure vessel stresses (cylinder or sphere)", topic="pressure vessels / thin-walled vessels",
     purpose="Hoop and longitudinal stresses and the in-plane and absolute maximum shear stresses in the wall of a thin cylindrical or spherical pressure vessel.",
     inputs=[_phys("p", "pressure", "internal gauge pressure"), _phys("r", "length", "inner radius"), _phys("t", "length", "wall thickness"),
             _choice("shape", ["cylinder", "sphere"], "vessel shape")],
     outputs=[_o("hoop_MPa", "number", "hoop (circumferential) stress, MPa"), _o("longitudinal_MPa", "number", "longitudinal (axial) stress, MPa"),
              _o("tau_inplane_MPa", "number", "maximum in-plane shear stress, MPa"), _o("tau_abs_MPa", "number", "absolute maximum shear stress at the outer surface, MPa")],
     primary="hoop_MPa", conditions=["r/t >= 10 (thin wall)", "closed ends, internal pressure only", "r is the inner radius"],
     method="cylinder: sigma_h = pr/t, sigma_l = pr/(2t); sphere: sigma = pr/(2t); tau_abs = sigma_h/2 (radial stress 0 at the outer surface)",
     latex=r"\sigma_h = \frac{pr}{t},\quad \sigma_l = \frac{pr}{2t}",
     run=thin_vessel,
     parse=lambda q: _pairs(q, r"thin[- ]walled pressure vessel|thin pressure vessel", {"p": "p", "r": "r", "t": "t", "shape": "shape"}, phys={"p", "r", "t"}, words={"shape"}),
     reference=ref_thin_vessel,
     wrong=_safe(lambda p, r, t, shape: _out(hoop_MPa=p * r / (2 * t) / 1e6, longitudinal_MPa=p * r / t / 1e6, tau_inplane_MPa=p * r / (4 * t) / 1e6, tau_abs_MPa=p * r / (4 * t) / 1e6)),
     random_inputs=lambda rng: (lambda r: {"p": _lu(rng, 1e5, 3e7), "r": r, "t": r / rng.uniform(10, 200), "shape": rng.choice(["cylinder", "cylinder", "sphere"])})(_lu(rng, 0.05, 3)),
     templates=["Thin-walled pressure vessel: p = 2 MPa, r = 500 mm, t = 10 mm, shape = cylinder"],
     examples=[{"inputs": {"p": 2e6, "r": 0.5, "t": 0.01, "shape": "cylinder"}, "outputs": {"hoop_MPa": 100.0, "longitudinal_MPa": 50.0, "tau_inplane_MPa": 25.0, "tau_abs_MPa": 50.0}},
               {"inputs": {"p": 2e6, "r": 0.5, "t": 0.01, "shape": "sphere"}, "outputs": {"hoop_MPa": 50.0, "longitudinal_MPa": 50.0, "tau_inplane_MPa": 0.0}}],
     questions=[{"q": "Thin-walled pressure vessel: p = 2 MPa, r = 500 mm, t = 10 mm, shape = cylinder", "outputs": {"hoop_MPa": 100.0, "longitudinal_MPa": 50.0}},
                {"q": "Thin-walled pressure vessel: p = 2 MPa, r = 50 mm, t = 10 mm, shape = cylinder", "outcome": "INVALID_INPUT"}])


@functools.lru_cache(maxsize=None)
def _lame_symbolic():
    """sympy: the axisymmetric plane-stress displacement equation u'' + u'/r - u/r^2 = 0 solved by dsolve, the
    constants fixed by the traction conditions sigma_r(a) = -pi, sigma_r(b) = -po; numeric functions of
    (r, a, b, pi, po) for sigma_r, sigma_theta (E and nu cancel) and of (r, a, b, pi, po, E, nu) for u."""
    import sympy as sp
    r, a, b, pi_, po, E, nu = sp.symbols("r a b p_i p_o E nu", positive=True)
    u = sp.Function("u")
    sol = sp.dsolve(sp.Eq(u(r).diff(r, 2) + u(r).diff(r) / r - u(r) / r ** 2, 0), u(r)).rhs
    C = sorted(sol.free_symbols - {r}, key=str)
    sr = E / (1 - nu ** 2) * (sol.diff(r) + nu * sol / r)
    st = E / (1 - nu ** 2) * (sol / r + nu * sol.diff(r))
    cs = sp.solve([sp.Eq(sr.subs(r, a), -pi_), sp.Eq(sr.subs(r, b), -po)], C, dict=True)[0]
    f = lambda e: sp.lambdify((r, a, b, pi_, po, E, nu), sp.simplify(e.subs(cs)), "math")
    return f(sr), f(st), f(sol)


def lame(pi, po, a, b, r=None):
    _nonneg("the pressures", pi, po)
    _pos("a and b", a, b)
    if a >= b:
        raise ValueError("the inner radius a must be smaller than the outer radius b")
    if pi == 0 and po == 0:
        raise ValueError("at least one pressure must be positive")
    r = a if r is None else r
    if not a <= r <= b:
        raise ValueError("the radius r must lie in the wall (a <= r <= b)")
    A = (pi * a * a - po * b * b) / (b * b - a * a)
    B = (pi - po) * a * a * b * b / (b * b - a * a)
    sr = lambda x: A - B / (x * x)
    st = lambda x: A + B / (x * x)
    sc = max(pi, po)
    return _out(sr_MPa=_snap(sr(r), sc) / 1e6, st_MPa=_snap(st(r), sc) / 1e6, st_inner_MPa=_snap(st(a), sc) / 1e6, st_outer_MPa=_snap(st(b), sc) / 1e6,
                tau_max_inner_MPa=_snap((st(a) - sr(a)) / 2, sc) / 1e6, sz_closed_MPa=_snap(A, sc) / 1e6)


def ref_lame(pi, po, a, b, r=None):
    """sympy dsolve of the axisymmetric displacement equation with the traction boundary conditions; the closed-end
    axial stress from the end-cap force balance (areas by scipy quadrature)."""
    srf, stf, _u = _lame_symbolic()
    r = a if r is None else r
    E, nu = 1.0, 0.3
    sc = max(pi, po)
    ev = lambda f, x: _snap(f(x, a, b, pi, po, E, nu), sc) / 1e6
    disk = lambda R: _quad_alg(lambda y: 2.0, -R, R, 0.5)
    sz = (pi * disk(a) - po * disk(b)) / (disk(b) - disk(a))
    return {"sr_MPa": ev(srf, r), "st_MPa": ev(stf, r), "st_inner_MPa": ev(stf, a), "st_outer_MPa": ev(stf, b),
            "tau_max_inner_MPa": _snap((stf(a, a, b, pi, po, E, nu) - srf(a, a, b, pi, po, E, nu)) / 2, sc) / 1e6, "sz_closed_MPa": _snap(sz, sc) / 1e6}


_add("thick_walled_cylinder",
     name="Thick-walled cylinder under pressure (Lame equations)", topic="pressure vessels / thick-walled cylinders",
     purpose="Radial and hoop stresses at a radius, the hoop stresses at both surfaces, the maximum shear at the bore and the closed-end axial stress of a thick cylinder under internal and external pressure.",
     inputs=[_phys("pi", "pressure", "internal pressure"), _phys("po", "pressure", "external pressure"), _phys("a", "length", "inner radius"), _phys("b", "length", "outer radius"),
             _phys("r", "length", "radius at which to evaluate (default: the bore, r = a)", required=False)],
     outputs=[_o("sr_MPa", "number", "radial stress at r, MPa"), _o("st_MPa", "number", "hoop stress at r, MPa"), _o("st_inner_MPa", "number", "hoop stress at the bore, MPa"),
              _o("st_outer_MPa", "number", "hoop stress at the outer surface, MPa"), _o("tau_max_inner_MPa", "number", "maximum shear stress at the bore, MPa"),
              _o("sz_closed_MPa", "number", "axial stress with closed ends, MPa")],
     primary="st_inner_MPa", conditions=["a < b", "pressures >= 0, not both zero", "a <= r <= b", "linear elastic"],
     method="sigma_r = A - B/r^2, sigma_t = A + B/r^2 with A = (pi a^2 - po b^2)/(b^2 - a^2), B = (pi - po) a^2 b^2/(b^2 - a^2); closed ends sigma_z = A",
     latex=r"\sigma_{r,\theta} = \frac{p_ia^2 - p_ob^2}{b^2-a^2} \mp \frac{(p_i-p_o)a^2b^2}{(b^2-a^2)r^2}",
     run=lame,
     parse=lambda q: _pairs(q, r"thick[- ]walled cylinder|lame equations?", {"pi": "pi", "po": "po", "a": "a", "b": "b", "r": "r"}, phys={"pi", "po", "a", "b", "r"}),
     reference=ref_lame,
     wrong=_safe(lambda pi, po, a, b, r=None: (lambda A, B, rr: _out(sr_MPa=(A + B / rr ** 2) / 1e6, st_MPa=(A - B / rr ** 2) / 1e6, st_inner_MPa=(A - B / a ** 2) / 1e6,
                                                                   st_outer_MPa=(A - B / b ** 2) / 1e6, tau_max_inner_MPa=B / a ** 2 / 1e6, sz_closed_MPa=A / 1e6))(
         (pi * a * a - po * b * b) / (b * b - a * a), (pi - po) * a * a * b * b / (b * b - a * a), a if r is None else r)),
     random_inputs=lambda rng: (lambda a: (lambda b: {"pi": rng.choice([0.0, _lu(rng, 1e6, 3e8)]) or _lu(rng, 1e6, 3e8), "po": rng.choice([0.0, _lu(rng, 1e5, 1e8)]), "a": a, "b": b,
                                                    "r": rng.uniform(a, b)})(a * rng.uniform(1.1, 4)))(_lu(rng, 0.01, 0.5)),
     templates=["Thick-walled cylinder: pi = 100 MPa, po = 0 MPa, a = 50 mm, b = 100 mm"],
     examples=[{"inputs": {"pi": 100e6, "po": 0.0, "a": 0.05, "b": 0.1},
                "outputs": {"sr_MPa": -100.0, "st_inner_MPa": 100 * (0.01 + 0.0025) / 0.0075, "st_outer_MPa": 200 * 0.0025 / 0.0075, "sz_closed_MPa": 100 * 0.0025 / 0.0075}}],
     questions=[{"q": "Thick-walled cylinder: pi = 100 MPa, po = 0 MPa, a = 50 mm, b = 100 mm", "outputs": {"st_inner_MPa": 100 * 0.0125 / 0.0075}},
                {"q": "Thick-walled cylinder: pi = 100 MPa, po = 0 MPa, a = 100 mm, b = 50 mm", "outcome": "INVALID_INPUT"}])


def shrink_fit(delta, R, ro, E, ri=0.0):
    _pos("delta, R, ro and E", delta, R, ro, E)
    _nonneg("ri", ri)
    if not ri < R < ro:
        raise ValueError("need ri < R < ro (shaft bore < interface < hub outer radius)")
    p = E * delta / (2 * R ** 3) * (ro * ro - R * R) * (R * R - ri * ri) / (ro * ro - ri * ri)
    st_hub = p * (ro * ro + R * R) / (ro * ro - R * R)
    st_shaft = -p * (R * R + ri * ri) / (R * R - ri * ri)
    return _out(p_MPa=p / 1e6, hub_hoop_MPa=st_hub / 1e6, shaft_hoop_MPa=st_shaft / 1e6)


def ref_shrink_fit(delta, R, ro, E, ri=0.0):
    """sympy's dsolve-based Lame solution (plane stress): the hub's bore expansion under unit interface pressure and
    the shaft's contraction (a solid shaft as the limit of a vanishing bore), the pressure from compatibility
    u_hub - u_shaft = delta; hoop stresses from the same solution. Poisson's ratio is set to 0.3 and cancels."""
    srf, stf, uf = _lame_symbolic()
    nu = 0.3
    rib = ri if ri > 0 else R * 1e-8                  # a solid shaft as the limit of a vanishing bore (error ~1e-16)
    u_hub = uf(R, R, ro, 1.0, 0.0, E, nu)
    u_sh = uf(R, rib, R, 0.0, 1.0, E, nu)
    p = delta / (u_hub - u_sh)
    return {"p_MPa": p / 1e6, "hub_hoop_MPa": stf(R, R, ro, p, 0.0, E, nu) / 1e6, "shaft_hoop_MPa": stf(R, rib, R, 0.0, p, E, nu) / 1e6}


_add("shrink_fit_pressure",
     name="Shrink (press) fit: interface pressure and hoop stresses", topic="pressure vessels / interference fits",
     purpose="The contact pressure of an interference fit between a hub and a solid or hollow shaft of the same material, and the hoop stresses it causes at the interface.",
     inputs=[_phys("delta", "length", "radial interference (shaft radius minus hub bore radius)"), _phys("R", "length", "nominal interface radius"),
             _phys("ro", "length", "outer radius of the hub"), _phys("E", "elastic_modulus", "Young's modulus (both parts)"),
             _phys("ri", "length", "bore radius of a hollow shaft (default 0: solid)", required=False)],
     outputs=[_o("p_MPa", "number", "interface pressure, MPa"), _o("hub_hoop_MPa", "number", "hoop stress in the hub at the interface, MPa"),
              _o("shaft_hoop_MPa", "number", "hoop stress in the shaft at the interface, MPa")],
     primary="p_MPa", conditions=["ri < R < ro", "both parts of the same material", "plane stress, elastic, long hub"],
     method="p = E delta/(2R^3) (ro^2 - R^2)(R^2 - ri^2)/(ro^2 - ri^2); hub sigma_t = p (ro^2 + R^2)/(ro^2 - R^2); shaft sigma_t = -p (R^2 + ri^2)/(R^2 - ri^2)",
     latex=r"p = \frac{E\delta}{2R^3}\,\frac{(r_o^2-R^2)(R^2-r_i^2)}{r_o^2-r_i^2}",
     run=shrink_fit,
     parse=lambda q: _pairs(q, r"shrink fit|press fit|interference fit", {"delta": "delta", "R": "R", "ro": "ro", "ri": "ri", "E": "E"}, phys={"delta", "R", "ro", "ri", "E"}),
     reference=ref_shrink_fit,
     wrong=_safe(lambda delta, R, ro, E, ri=0.0: (lambda p: _out(p_MPa=p / 1e6, hub_hoop_MPa=p * (ro * ro + R * R) / (ro * ro - R * R) / 1e6, shaft_hoop_MPa=-p / 1e6))(
         E * delta / R * (ro * ro - R * R) * (R * R - ri * ri) / (ro * ro - ri * ri) / R ** 2)),
     random_inputs=lambda rng: (lambda R: {"delta": R * rng.uniform(2e-4, 1.5e-3), "R": R, "ro": R * rng.uniform(1.3, 3), "E": rng.uniform(70e9, 210e9),
                                           "ri": rng.choice([0.0, R * rng.uniform(0.2, 0.7)])})(_lu(rng, 0.005, 0.2)),
     templates=["Shrink fit: delta = 0.02 mm, R = 25 mm, ro = 50 mm, E = 207 GPa"],
     examples=[{"inputs": {"delta": 2e-5, "R": 0.025, "ro": 0.05, "E": 207e9},
                "outputs": {"p_MPa": 207e9 * 2e-5 / (2 * 0.025 ** 3) * (0.0025 - 0.000625) * 0.000625 / 0.0025 / 1e6}}],
     questions=[{"q": "Shrink fit: delta = 0.02 mm, R = 25 mm, ro = 50 mm, E = 207 GPa", "outputs": {"p_MPa": 207e9 * 2e-5 / (2 * 0.025 ** 3) * 0.001875 * 0.000625 / 0.0025 / 1e6}},
                {"q": "Shrink fit: delta = 0.02 mm, R = 60 mm, ro = 50 mm, E = 207 GPa", "outcome": "INVALID_INPUT"}])


@functools.lru_cache(maxsize=None)
def _disk_symbolic(solid):
    """sympy: the rotating-disk displacement equation (plane stress) r^2 u'' + r u' - u = -(1 - nu^2) rho w^2 r^3/E
    solved by dsolve; constants from sigma_r = 0 at both radii (annular) or a finite centre and sigma_r(b) = 0
    (solid). Numeric functions of (r, a, b, rho, w, nu) for sigma_r, sigma_t and d(sigma_r)/dr."""
    import sympy as sp
    r, a, b, rho, w, nu, E = sp.symbols("r a b rho omega nu E", positive=True)
    u = sp.Function("u")
    ode = sp.Eq(r ** 2 * u(r).diff(r, 2) + r * u(r).diff(r) - u(r), -(1 - nu ** 2) * rho * w ** 2 * r ** 3 / E)
    sol = sp.dsolve(ode, u(r)).rhs
    C = sorted(sol.free_symbols - {r, rho, w, nu, E}, key=str)
    sr = E / (1 - nu ** 2) * (sol.diff(r) + nu * sol / r)
    st = E / (1 - nu ** 2) * (sol / r + nu * sol.diff(r))
    if solid:
        inv = {c: 0 for c in C if not sp.simplify(sol.diff(c) * r).has(r)}     # the 1/r term: zero for a finite centre
        sr, st = sr.subs(inv), st.subs(inv)
        rest = [c for c in C if c not in inv]
        cs = sp.solve([sp.Eq(sr.subs(r, b), 0)], rest, dict=True)[0]
    else:
        cs = sp.solve([sp.Eq(sr.subs(r, a), 0), sp.Eq(sr.subs(r, b), 0)], C, dict=True)[0]
    SR, ST = sp.simplify(sr.subs(cs)), sp.simplify(st.subs(cs))
    f = lambda e: sp.lambdify((r, a, b, rho, w, nu), e, "math")
    return f(SR), f(ST), f(sp.diff(SR, r))


def rotating_disk(rho, omega, nu, ro, ri=0.0):
    _pos("rho, omega and ro", rho, omega, ro)
    _nonneg("ri", ri)
    if not 0 <= nu < 0.5:
        raise ValueError("Poisson's ratio must lie in [0, 0.5)")
    if ri >= ro:
        raise ValueError("the bore radius must be smaller than the outer radius")
    k = (3 + nu) / 8 * rho * omega ** 2
    if ri == 0:
        st_max = sr_max = k * ro * ro
        r_sr = 0.0
    else:
        st_max = (3 + nu) / 4 * rho * omega ** 2 * (ro * ro + (1 - nu) / (3 + nu) * ri * ri)
        sr_max = k * (ro - ri) ** 2
        r_sr = math.sqrt(ri * ro)
    st_rim = (3 + nu) / 4 * rho * omega ** 2 * (ri * ri + (1 - nu) / (3 + nu) * ro * ro)
    return _out(hoop_max_MPa=st_max / 1e6, radial_max_MPa=sr_max / 1e6, r_radial_max=r_sr, hoop_rim_MPa=st_rim / 1e6)


def ref_rotating_disk(rho, omega, nu, ro, ri=0.0):
    """sympy dsolve of the rotating-disk equation with its boundary conditions; the largest radial stress where its
    derivative vanishes (scipy brentq) for an annular disk."""
    from scipy.optimize import brentq
    if ri == 0:
        SR, ST, dSR = _disk_symbolic(True)
        c = ro * 1e-9
        return {"hoop_max_MPa": ST(c, 0.0, ro, rho, omega, nu) / 1e6, "radial_max_MPa": SR(c, 0.0, ro, rho, omega, nu) / 1e6,
                "hoop_rim_MPa": ST(ro, 0.0, ro, rho, omega, nu) / 1e6}
    SR, ST, dSR = _disk_symbolic(False)
    rs = brentq(lambda x: dSR(x, ri, ro, rho, omega, nu), ri * (1 + 1e-9), ro * (1 - 1e-9), xtol=1e-16, rtol=8.9e-16)
    return {"hoop_max_MPa": ST(ri, ri, ro, rho, omega, nu) / 1e6, "radial_max_MPa": SR(rs, ri, ro, rho, omega, nu) / 1e6, "r_radial_max": rs,
            "hoop_rim_MPa": ST(ro, ri, ro, rho, omega, nu) / 1e6}


_add("rotating_disk_stresses",
     name="Stresses in a rotating disk (solid or with a central hole)", topic="pressure vessels / rotating disks",
     purpose="The largest hoop and radial stresses, the radius of the largest radial stress and the hoop stress at the rim of a thin disk of uniform thickness spinning at constant speed.",
     inputs=[_phys("rho", "density", "density"), _phys("omega", "angular_frequency", "angular speed (rpm or rad/s)"), _numin("nu", "Poisson's ratio"),
             _phys("ro", "length", "outer radius"), _phys("ri", "length", "radius of the central hole (default 0: solid)", required=False)],
     outputs=[_o("hoop_max_MPa", "number", "largest hoop stress (at the bore, or the centre of a solid disk), MPa"), _o("radial_max_MPa", "number", "largest radial stress, MPa"),
              _o("r_radial_max", "length", "radius of the largest radial stress (sqrt(ri ro); 0 for a solid disk)"), _o("hoop_rim_MPa", "number", "hoop stress at the outer rim, MPa")],
     primary="hoop_max_MPa", conditions=["thin disk of uniform thickness, plane stress", "free inner and outer edges", "0 <= nu < 0.5"],
     method="solid: sigma_r = sigma_t = (3 + nu)/8 rho w^2 ro^2 at the centre; annular: sigma_t(ri) = (3 + nu)/4 rho w^2 (ro^2 + (1 - nu)/(3 + nu) ri^2), sigma_r,max = (3 + nu)/8 rho w^2 (ro - ri)^2 at sqrt(ri ro)",
     latex=r"\sigma_\theta(r_i) = \frac{3+\nu}{4}\rho\omega^2\Big(r_o^2 + \frac{1-\nu}{3+\nu}r_i^2\Big),\quad \sigma_{r,max} = \frac{3+\nu}{8}\rho\omega^2(r_o-r_i)^2",
     run=rotating_disk,
     parse=lambda q: _pairs(q, r"rotating dis[ck](?: stresses)?", {"rho": "rho", "omega": "omega", "n": "omega", "nu": "nu", "ro": "ro", "ri": "ri"}, phys={"rho", "omega", "ro", "ri"}),
     reference=ref_rotating_disk,
     wrong=_safe(lambda rho, omega, nu, ro, ri=0.0: _out(hoop_max_MPa=(1 + 3 * nu) / 8 * rho * omega ** 2 * (ro * ro + ri * ri) / 1e6, radial_max_MPa=(3 + nu) / 8 * rho * omega ** 2 * (ro * ro - ri * ri) / 1e6,
                                                       r_radial_max=(ri + ro) / 2, hoop_rim_MPa=(1 - nu) / 4 * rho * omega ** 2 * ro * ro / 1e6)),
     random_inputs=lambda rng: (lambda ro: {"rho": rng.uniform(2000, 8500), "omega": _lu(rng, 50, 2000), "nu": rng.uniform(0.25, 0.35), "ro": ro,
                                            "ri": rng.choice([0.0, ro * rng.uniform(0.05, 0.8)])})(_lu(rng, 0.05, 1.0)),
     templates=["Rotating disk: rho = 7800 kg/m^3, omega = 3000 rpm, nu = 0.3, ro = 250 mm, ri = 50 mm"],
     examples=[{"inputs": {"rho": 7800.0, "omega": 100 * math.pi, "nu": 0.3, "ro": 0.25, "ri": 0.05},
                "outputs": {"hoop_max_MPa": 3.3 / 4 * 7800 * (100 * math.pi) ** 2 * (0.0625 + 0.7 / 3.3 * 0.0025) / 1e6, "radial_max_MPa": 3.3 / 8 * 7800 * (100 * math.pi) ** 2 * 0.04 / 1e6}},
               {"inputs": {"rho": 7800.0, "omega": 100 * math.pi, "nu": 0.3, "ro": 0.25}, "outputs": {"hoop_max_MPa": 3.3 / 8 * 7800 * (100 * math.pi) ** 2 * 0.0625 / 1e6}}],
     questions=[{"q": "Rotating disk: rho = 7800 kg/m^3, omega = 3000 rpm, nu = 0.3, ro = 250 mm, ri = 50 mm",
                 "outputs": {"radial_max_MPa": 3.3 / 8 * 7800 * (100 * math.pi) ** 2 * 0.04 / 1e6}},
                {"q": "Rotating disk: rho = 7800 kg/m^3, omega = 3000 rpm, nu = 0.3, ro = 250 mm, ri = 300 mm", "outcome": "INVALID_INPUT"}])


# ================================================================================================ columns
_ENDS = ["pinned-pinned", "fixed-free", "fixed-fixed", "fixed-pinned"]


def _k_factor(ends):
    """Theoretical effective-length factor: 1, 2, 0.5, and pi/x for fixed-pinned with x the first positive root of
    tan x = x (found by bisection)."""
    if ends == "pinned-pinned":
        return 1.0
    if ends == "fixed-free":
        return 2.0
    if ends == "fixed-fixed":
        return 0.5
    lo, hi = math.pi + 1e-9, 1.5 * math.pi - 1e-9
    for _ in range(200):
        mid = (lo + hi) / 2
        if math.tan(mid) - mid > 0:
            hi = mid
        else:
            lo = mid
    return math.pi / ((lo + hi) / 2)


@functools.lru_cache(maxsize=None)
def _ref_k_factor(ends):
    """The buckling condition as the determinant of the boundary conditions of w = A sin kx + B cos kx + Cx + D
    (numpy), its smallest positive root found by scipy brentq; K = pi/(kL)."""
    import numpy as np
    from scipy.optimize import brentq
    bot, top = ends.split("-")

    def det(k):
        s, c = math.sin(k), math.cos(k)
        rows = [[0, 1, 0, 1]] + ([[0, -k * k, 0, 0]] if bot == "pinned" else [[k, 0, 1, 0]])
        if top == "pinned":
            rows += [[s, c, 1, 1], [-k * k * s, -k * k * c, 0, 0]]
        elif top == "fixed":
            rows += [[s, c, 1, 1], [k * c, -k * s, 1, 0]]
        else:
            rows += [[-k * k * s, -k * k * c, 0, 0], [0, 0, k * k, 0]]
        return float(np.linalg.det(np.array(rows, dtype=float)))
    ks = [0.05 + 0.005 * i for i in range(2000)]
    vals = [det(k) for k in ks]
    for i in range(len(ks) - 1):
        if vals[i] == 0:
            return math.pi / ks[i]
        if vals[i] * vals[i + 1] < 0:
            return math.pi / brentq(det, ks[i], ks[i + 1], xtol=1e-15, rtol=8.9e-16)
    raise RuntimeError("no root")


def euler_column(E, I, L, ends, A=None):
    _pos("E, I and L", E, I, L)
    K = _k_factor(ends)
    Le = K * L
    P = math.pi ** 2 * E * I / Le ** 2
    out = {"P_cr": P, "P_cr_kN": P / 1e3, "K": K, "Le": Le}
    if A is not None:
        _pos("A", A)
        out.update(sigma_cr_MPa=P / A / 1e6, slenderness=Le / math.sqrt(I / A))
    return _out(**out)


def ref_euler_column(E, I, L, ends, A=None):
    K = _ref_k_factor(ends)
    P = (math.pi / (K * L)) ** 2 * E * I
    out = {"P_cr": P, "P_cr_kN": P / 1e3, "K": K, "Le": K * L}
    if A is not None:
        out.update(sigma_cr_MPa=P / A / 1e6, slenderness=K * L / math.sqrt(I / A))
    return out


_add("euler_buckling",
     name="Euler buckling load of a column with given end conditions", topic="columns / elastic buckling",
     purpose="The critical (Euler) buckling load of an ideal elastic column, its effective length factor and length, and with the area also the critical stress and slenderness ratio.",
     inputs=[_phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "least second moment of area"), _phys("L", "length", "column length"),
             _choice("ends", _ENDS, "end conditions: pinned-pinned, fixed-free, fixed-fixed or fixed-pinned"),
             _phys("A", "area", "cross-sectional area (optional: gives the stress and slenderness)", required=False)],
     outputs=[_o("P_cr", "force", "critical buckling load"), _o("P_cr_kN", "number", "critical load in kN"), _o("K", "number", "effective length factor (theoretical)"),
              _o("Le", "length", "effective length K L"), _o("sigma_cr_MPa", "number", "critical stress P_cr/A, MPa (with A)"), _o("slenderness", "number", "slenderness ratio K L / r (with A)")],
     primary="P_cr", conditions=["ideal straight elastic column, centric load", "theoretical K: 1, 2, 0.5, 0.6992 (fixed-pinned, from tan x = x)", "valid only below the proportional limit"],
     method="P_cr = pi^2 E I/(K L)^2",
     latex=r"P_{cr} = \frac{\pi^2EI}{(KL)^2}",
     run=euler_column,
     parse=lambda q: _pairs(q, r"euler buckling(?: load)?|critical buckling load", {"E": "E", "I": "I", "L": "L", "ends": "ends", "A": "A"}, phys={"E", "I", "L", "A"}, words={"ends"}),
     reference=ref_euler_column,
     wrong=_safe(lambda E, I, L, ends, A=None: euler_column(E, I, L, {"pinned-pinned": "fixed-pinned", "fixed-free": "pinned-pinned", "fixed-fixed": "fixed-pinned", "fixed-pinned": "fixed-fixed"}[ends], A)),
     random_inputs=lambda rng: {"E": rng.uniform(70e9, 210e9), "I": _lu(rng, 1e-8, 1e-4), "L": rng.uniform(0.5, 10), "ends": rng.choice(_ENDS),
                                **({"A": _lu(rng, 1e-4, 2e-2)} if rng.random() < 0.6 else {})},
     templates=["Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = pinned-pinned"],
     examples=[{"inputs": {"E": 200e9, "I": 8e-6, "L": 4.0, "ends": "pinned-pinned"}, "outputs": {"P_cr": math.pi ** 2 * 200e9 * 8e-6 / 16, "K": 1.0}},
               {"inputs": {"E": 200e9, "I": 8e-6, "L": 4.0, "ends": "fixed-pinned", "A": 0.005}, "outputs": {"K": 0.699155659643, "slenderness": 0.699155659643 * 4 / math.sqrt(8e-6 / 0.005)}}],
     questions=[{"q": "Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = pinned-pinned", "outputs": {"P_cr": math.pi ** 2 * 200e9 * 8e-6 / 16}},
                {"q": "Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = fixed-fixed, A = 5000 mm^2", "outputs": {"P_cr": 4 * math.pi ** 2 * 200e9 * 8e-6 / 16}},
                {"q": "Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = hinged", "outcome": "INVALID_INPUT"}])


def johnson_column(Sy, E, L, r, ends, A=None):
    _pos("Sy, E, L and r", Sy, E, L, r)
    K = _k_factor(ends)
    lam = K * L / r
    Cc = math.sqrt(2 * math.pi ** 2 * E / Sy)
    if lam < Cc:
        sc, regime = Sy - (Sy * lam / (2 * math.pi)) ** 2 / E, "Johnson"
    else:
        sc, regime = math.pi ** 2 * E / lam ** 2, "Euler"
    out = {"slenderness": lam, "transition_slenderness": Cc, "regime": regime, "sigma_cr_MPa": sc / 1e6}
    if A is not None:
        _pos("A", A)
        out["P_cr_kN"] = sc * A / 1e3
    return _out(**out)


@functools.lru_cache(maxsize=None)
def _johnson_tangency():
    """sympy: the parabola sigma = Sy - c lam^2 that touches the Euler hyperbola pi^2 E/lam^2 (equal value and slope);
    solved for c and the touching slenderness."""
    import sympy as sp
    Sy, E, lam, c = sp.symbols("S_y E lambda c", positive=True)
    euler = sp.pi ** 2 * E / lam ** 2
    para = Sy - c * lam ** 2
    sol = sp.solve([sp.Eq(para, euler), sp.Eq(sp.diff(para, lam), sp.diff(euler, lam))], [c, lam], dict=True)[0]
    return sp.lambdify((Sy, E), sol[c], "math"), sp.lambdify((Sy, E), sol[lam], "math")


def ref_johnson_column(Sy, E, L, r, ends, A=None):
    cf, lf = _johnson_tangency()
    K = _ref_k_factor(ends)
    lam = K * L / r
    Cc = lf(Sy, E)
    sc = Sy - cf(Sy, E) * lam ** 2 if lam < Cc else math.pi ** 2 * E / lam ** 2
    out = {"slenderness": lam, "transition_slenderness": Cc, "regime": "Johnson" if lam < Cc else "Euler", "sigma_cr_MPa": sc / 1e6}
    if A is not None:
        out["P_cr_kN"] = sc * A / 1e3
    return out


_add("johnson_column",
     name="Critical stress of an intermediate column (Johnson parabola / Euler)", topic="columns / inelastic buckling",
     purpose="The critical stress of a column by the Johnson parabola below the transition slenderness and by Euler above it, with the transition slenderness and (with the area) the critical load.",
     inputs=[_phys("Sy", "stress", "yield strength"), _phys("E", "elastic_modulus", "Young's modulus"), _phys("L", "length", "column length"),
             _phys("r", "length", "least radius of gyration"), _choice("ends", _ENDS, "end conditions (theoretical K)"),
             _phys("A", "area", "cross-sectional area (optional: gives the load)", required=False)],
     outputs=[_o("slenderness", "number", "slenderness ratio K L / r"), _o("transition_slenderness", "number", "transition slenderness C_c = sqrt(2 pi^2 E / Sy)"),
              _o("regime", "text", "Johnson or Euler"), _o("sigma_cr_MPa", "number", "critical stress, MPa"), _o("P_cr_kN", "number", "critical load in kN (with A)")],
     primary="sigma_cr_MPa", conditions=["centric load", "theoretical effective length factors"],
     method="C_c = sqrt(2 pi^2 E/Sy); KL/r < C_c: sigma_cr = Sy - (Sy KL/(2 pi r))^2/E; otherwise sigma_cr = pi^2 E/(KL/r)^2",
     latex=r"\sigma_{cr} = S_y - \frac{1}{E}\Big(\frac{S_y}{2\pi}\frac{KL}{r}\Big)^2 \quad (KL/r < C_c),\quad C_c = \sqrt{\frac{2\pi^2E}{S_y}}",
     run=johnson_column,
     parse=lambda q: _pairs(q, r"johnson column|johnson parabola", {"Sy": "Sy", "E": "E", "L": "L", "r": "r", "ends": "ends", "A": "A"}, phys={"Sy", "E", "L", "r", "A"}, words={"ends"}),
     reference=ref_johnson_column,
     wrong=_safe(lambda Sy, E, L, r, ends, A=None: (lambda lam: _out(slenderness=lam, transition_slenderness=math.sqrt(math.pi ** 2 * E / Sy), regime="Johnson",
                                                                   sigma_cr_MPa=(Sy - (Sy * lam / math.pi) ** 2 / E) / 1e6, P_cr_kN=0.0))(_k_factor(ends) * L / r)),
     random_inputs=lambda rng: {"Sy": rng.uniform(200e6, 800e6), "E": rng.uniform(70e9, 210e9), "L": rng.uniform(0.3, 4), "r": rng.uniform(0.005, 0.06), "ends": rng.choice(_ENDS),
                                **({"A": _lu(rng, 1e-4, 1e-2)} if rng.random() < 0.5 else {})},
     templates=["Johnson column: Sy = 250 MPa, E = 200 GPa, L = 1.5 m, r = 20 mm, ends = pinned-pinned"],
     examples=[{"inputs": {"Sy": 250e6, "E": 200e9, "L": 1.5, "r": 0.02, "ends": "pinned-pinned"},
                "outputs": {"slenderness": 75.0, "regime": "Johnson", "sigma_cr_MPa": (250e6 - (250e6 * 75 / (2 * math.pi)) ** 2 / 200e9) / 1e6}},
               {"inputs": {"Sy": 250e6, "E": 200e9, "L": 4.0, "r": 0.02, "ends": "pinned-pinned"}, "outputs": {"regime": "Euler", "sigma_cr_MPa": math.pi ** 2 * 200e9 / 200 ** 2 / 1e6}}],
     questions=[{"q": "Johnson column: Sy = 250 MPa, E = 200 GPa, L = 1.5 m, r = 20 mm, ends = pinned-pinned",
                 "outputs": {"sigma_cr_MPa": (250e6 - (250e6 * 75 / (2 * math.pi)) ** 2 / 200e9) / 1e6}},
                {"q": "Johnson column: Sy = 250 MPa, E = 200 GPa, L = 1.5 m, ends = pinned-pinned", "outcome": "NEEDS_INPUT"}])


def secant_column(P, e, L, A, I, c, E):
    _pos("P, L, A, I, c and E", P, L, A, I, c, E)
    _nonneg("the eccentricity e", e)
    Pcr = math.pi ** 2 * E * I / L ** 2
    if P >= Pcr:
        raise ValueError("the load reaches the Euler load of the pinned column: no equilibrium")
    k = math.sqrt(P / (E * I))
    sec = 1 / math.cos(k * L / 2)
    smax = P / A * (1 + e * c / (I / A) * sec)
    return _out(sigma_max_MPa=smax / 1e6, deflection_mm=e * (sec - 1) * 1e3, M_max=P * e * sec, P_over_Pcr=P / Pcr)


@functools.lru_cache(maxsize=None)
def _beam_column_symbolic():
    """sympy dsolve of the eccentrically loaded pinned column w'' + k^2 w = -k^2 e with w(0) = w(L) = 0; the midspan
    deflection as a numeric function of (k, L, e)."""
    import sympy as sp
    x, k, L, e = sp.symbols("x k L e", positive=True)
    w = sp.Function("w")
    sol = sp.dsolve(sp.Eq(w(x).diff(x, 2) + k ** 2 * w(x), -k ** 2 * e), w(x), ics={w(0): 0, w(L): 0}).rhs
    return sp.lambdify((k, L, e), sp.simplify(sol.subs(x, L / 2)), "math")


def ref_secant_column(P, e, L, A, I, c, E):
    wm = _beam_column_symbolic()(math.sqrt(P / (E * I)), L, e)
    M = P * (e + wm)
    return {"sigma_max_MPa": (P / A + M * c / I) / 1e6, "deflection_mm": wm * 1e3, "M_max": M, "P_over_Pcr": P * L * L / (math.pi ** 2 * E * I)}


_add("secant_column",
     name="Eccentrically loaded column (secant formula)", topic="columns / eccentric loading",
     purpose="The maximum compressive stress, midspan deflection and largest bending moment of a pinned column under a load with equal end eccentricities.",
     inputs=[_phys("P", "force", "compressive load"), _phys("e", "length", "eccentricity of the load"), _phys("L", "length", "length (pinned ends)"),
             _phys("A", "area", "area"), _phys("I", "second_moment_of_area", "second moment of area in the bending plane"),
             _phys("c", "length", "distance from the centroid to the extreme fibre"), _phys("E", "elastic_modulus", "Young's modulus")],
     outputs=[_o("sigma_max_MPa", "number", "maximum compressive stress (magnitude), MPa"), _o("deflection_mm", "number", "midspan lateral deflection, mm"),
              _o("M_max", "bending_moment", "largest bending moment P (e + deflection)"), _o("P_over_Pcr", "number", "P / P_Euler")],
     primary="sigma_max_MPa", conditions=["pinned ends, equal eccentricities on the same side", "P below the Euler load", "elastic"],
     method="sigma_max = P/A [1 + (e c/r^2) sec(L/2 sqrt(P/(EI)))], deflection = e [sec(kL/2) - 1]",
     latex=r"\sigma_{max} = \frac{P}{A}\Big[1 + \frac{ec}{r^2}\sec\Big(\frac{L}{2}\sqrt{\frac{P}{EI}}\Big)\Big]",
     run=secant_column,
     parse=lambda q: _pairs(q, r"secant (?:column|formula)|eccentric column", {k: k for k in ("P", "e", "L", "A", "I", "c", "E")}, phys={"P", "e", "L", "A", "I", "c", "E"}),
     reference=ref_secant_column,
     wrong=_safe(lambda P, e, L, A, I, c, E: _out(sigma_max_MPa=P / A * (1 + e * c / (I / A)) / 1e6, deflection_mm=0.0, M_max=P * e, P_over_Pcr=P * L * L / (math.pi ** 2 * E * I))),
     random_inputs=lambda rng: (lambda E, I, L: {"P": rng.uniform(0.05, 0.85) * math.pi ** 2 * E * I / L ** 2, "e": rng.uniform(0.001, 0.05), "L": L, "A": _lu(rng, 1e-3, 2e-2),
                                                 "I": I, "c": rng.uniform(0.02, 0.2), "E": E})(rng.uniform(70e9, 210e9), _lu(rng, 1e-6, 1e-4), rng.uniform(1, 8)),
     templates=["Secant column: P = 200 kN, e = 10 mm, L = 3 m, A = 4000 mm^2, I = 1.2e-5 m^4, c = 75 mm, E = 200 GPa"],
     examples=[{"inputs": {"P": 200e3, "e": 0.01, "L": 3.0, "A": 4e-3, "I": 1.2e-5, "c": 0.075, "E": 200e9},
                "outputs": {"sigma_max_MPa": 200e3 / 4e-3 * (1 + 0.01 * 0.075 / (1.2e-5 / 4e-3) / math.cos(1.5 * math.sqrt(200e3 / (200e9 * 1.2e-5)))) / 1e6}}],
     questions=[{"q": "Secant column: P = 200 kN, e = 10 mm, L = 3 m, A = 4000 mm^2, I = 1.2e-5 m^4, c = 75 mm, E = 200 GPa",
                 "outputs": {"sigma_max_MPa": 200e3 / 4e-3 * (1 + 0.01 * 0.075 / (1.2e-5 / 4e-3) / math.cos(1.5 * math.sqrt(200e3 / (200e9 * 1.2e-5)))) / 1e6}},
                {"q": "Secant column: P = 5000 kN, e = 10 mm, L = 3 m, A = 4000 mm^2, I = 1.2e-5 m^4, c = 75 mm, E = 200 GPa", "outcome": "INVALID_INPUT"}])


# ================================================================================================ combined loading
def combined_rect(b, h, N, Mx, My):
    _pos("b and h", b, h)
    _fin("N, Mx and My", N, Mx, My)
    A, Ix, Iy = b * h, b * h ** 3 / 12, h * b ** 3 / 12
    sig = lambda x, y: N / A + Mx * y / Ix + My * x / Iy
    sc = abs(N) / A + abs(Mx) * h / 2 / Ix + abs(My) * b / 2 / Iy
    if sc == 0:
        raise ValueError("no load is given")
    corners = [sig(b / 2, h / 2), sig(-b / 2, h / 2), sig(-b / 2, -h / 2), sig(b / 2, -h / 2)]
    corners = [_snap(v, sc) / 1e6 for v in corners]
    return _out(sigma_corners_MPa=corners, sigma_max_MPa=max(corners), sigma_min_MPa=min(corners), sigma_axial_MPa=N / A / 1e6)


def ref_combined_rect(b, h, N, Mx, My):
    """sectionproperties: the rectangle meshed, the axial force and moments applied by the library's stress analysis
    (its positive myy is the opposite sense of the one used here); stresses read at the four corner nodes."""
    import numpy as np
    from sectionproperties.analysis.section import Section
    from sectionproperties.pre.library import rectangular_section
    g = rectangular_section(d=h, b=b)
    g.create_mesh(mesh_sizes=[0])
    s = Section(geometry=g)
    s.calculate_geometric_properties()
    st = s.calculate_stress(n=N, mxx=Mx, myy=-My).get_stress()[0]["sig_zz"]
    v = np.array(s.mesh["vertices"])
    sc = abs(N) / (b * h) + abs(Mx) * 6 / (b * h * h) + abs(My) * 6 / (h * b * b)
    at = lambda x, y: _snap(float(st[int(np.argmin((v[:, 0] - x) ** 2 + (v[:, 1] - y) ** 2))]), sc) / 1e6
    corners = [at(b, h), at(0, h), at(0, 0), at(b, 0)]
    return {"sigma_corners_MPa": corners, "sigma_max_MPa": max(corners), "sigma_min_MPa": min(corners), "sigma_axial_MPa": N / s.get_area() / 1e6}


def _rand_combined(rng):
    """Well-conditioned draws: no corner stress below 1 % of the largest (the finite-element reference loses digits
    to cancellation there)."""
    for _ in range(200):
        x = {"b": _lu(rng, 0.02, 0.4), "h": _lu(rng, 0.02, 0.6), "N": rng.uniform(-5e5, 5e5), "Mx": rng.uniform(-5e4, 5e4), "My": rng.uniform(-3e4, 3e4)}
        c = combined_rect(**x)["sigma_corners_MPa"]
        if min(abs(v) for v in c) > 0.01 * max(abs(v) for v in c):
            return x
    return {"b": 0.1, "h": 0.2, "N": -5e4, "Mx": 8e3, "My": 2e3}


_add("combined_axial_and_bending_stress",
     name="Combined axial force and biaxial bending of a rectangular section", topic="stress analysis / combined loading",
     purpose="Normal stresses at the four corners of a rectangular section carrying an axial force and bending moments about both axes, with the extreme values.",
     inputs=[_phys("b", "length", "width (along x)"), _phys("h", "length", "depth (along y)"), _phys("N", "force", "axial force (tension positive)"),
             _phys("Mx", "bending_moment", "moment about x (positive: tension at +y)"), _phys("My", "bending_moment", "moment about y (positive: tension at +x)")],
     outputs=[_o("sigma_corners_MPa", "number", "stresses at the corners (+x,+y), (-x,+y), (-x,-y), (+x,-y), MPa"), _o("sigma_max_MPa", "number", "largest (most tensile) stress, MPa"),
              _o("sigma_min_MPa", "number", "smallest (most compressive) stress, MPa"), _o("sigma_axial_MPa", "number", "uniform axial part N/A, MPa")],
     primary="sigma_max_MPa", conditions=["linear elastic, plane sections", "moments about the centroidal axes"],
     method="sigma(x, y) = N/A + Mx y/Ix + My x/Iy at the corners (x = +-b/2, y = +-h/2)",
     latex=r"\sigma = \frac{N}{A} + \frac{M_xy}{I_x} + \frac{M_yx}{I_y}",
     run=combined_rect,
     parse=lambda q: _pairs(q, r"combined axial and bending stress|combined stress(?: rectangle)?", {"b": "b", "h": "h", "N": "N", "P": "N", "Mx": "Mx", "My": "My"}, phys={"b", "h", "N", "Mx", "My"}),
     reference=ref_combined_rect,
     wrong=_safe(lambda b, h, N, Mx, My: (lambda r: dict(r, sigma_max_MPa=(N / (b * h) + 6 * abs(Mx) / (b * h * h) + 6 * abs(My) / (b * b * h)) / 1e6 * 2))(combined_rect(b, h, N, Mx, My))),
     random_inputs=lambda rng: _rand_combined(rng),
     templates=["Combined axial and bending stress: b = 100 mm, h = 200 mm, N = -50 kN, Mx = 8 kN*m, My = 2 kN*m"],
     examples=[{"inputs": {"b": 0.1, "h": 0.2, "N": -50e3, "Mx": 8e3, "My": 2e3},
                "outputs": {"sigma_max_MPa": -2.5 + 8e3 * 0.1 / (0.1 * 0.008 / 12) / 1e6 + 2e3 * 0.05 / (0.2 * 0.001 / 12) / 1e6, "sigma_axial_MPa": -2.5}}],
     questions=[{"q": "Combined axial and bending stress: b = 100 mm, h = 200 mm, N = -50 kN, Mx = 8 kN*m, My = 2 kN*m", "outputs": {"sigma_max_MPa": -2.5 + 12 + 6}},
                {"q": "Combined axial and bending stress: b = 100 mm, h = 200 mm, N = -50 kN, Mx = 8 kN*m", "outcome": "NEEDS_INPUT"}])


def bolt_group(bolts, P, e):
    pts = [tuple(r) for r in bolts]
    if any(len(p) != 2 for p in pts) or not 2 <= len(pts) <= 40:
        raise ValueError("give 2 to 40 bolts as rows 'x y' (mm)")
    _pos("the load P", P)
    _fin("the eccentricity e", e)
    n = len(pts)
    xc, yc = math.fsum(p[0] for p in pts) / n, math.fsum(p[1] for p in pts) / n
    J = math.fsum((x - xc) ** 2 + (y - yc) ** 2 for x, y in pts)
    if J <= 0:
        raise ValueError("the bolts must not all coincide")
    em = e * 1e3                                               # mm
    M = P * em                                                 # N*mm, clockwise for a downward load right of the centroid
    forces = []
    for x, y in pts:
        fx = M * (y - yc) / J
        fy = -P / n - M * (x - xc) / J
        forces.append(math.hypot(fx, fy))
    fmax = max(forces)
    idx = next(i for i, f in enumerate(forces) if f >= fmax * (1 - 1e-9)) + 1
    return _out(direct_kN=P / n / 1e3, max_bolt_force_kN=fmax / 1e3, critical_bolt=idx, polar_sum_mm2=J)


def ref_bolt_group(bolts, P, e):
    """Displacement method with numpy: the plate as a rigid body on identical bolt springs (translations u, v and a
    rotation about the bolt centroid), solved for equilibrium with the eccentric load; bolt forces from the springs."""
    import numpy as np
    pts = np.array(bolts, dtype=float)
    c = pts.mean(axis=0)
    d = pts - c
    K = np.zeros((3, 3))
    for dx, dy in d:
        B = np.array([[1.0, 0.0, -dy], [0.0, 1.0, dx]])
        K += B.T @ B
    F = np.array([0.0, -P, -P * e * 1e3])
    q = np.linalg.solve(K, F)
    forces = [float(np.hypot(*(np.array([[1.0, 0.0, -dy], [0.0, 1.0, dx]]) @ q))) for dx, dy in d]
    fmax = max(forces)
    idx = next(i for i, f in enumerate(forces) if f >= fmax * (1 - 1e-9)) + 1
    return {"direct_kN": P / len(forces) / 1e3, "max_bolt_force_kN": fmax / 1e3, "critical_bolt": idx, "polar_sum_mm2": float(np.sum(d * d))}


def _rand_bolts(rng):
    rows, cols = rng.randint(1, 4), rng.randint(2, 3)
    sx, sy = rng.randint(40, 120), rng.randint(40, 120)
    pts = [[float(i * sx + rng.choice([0, 0, rng.randint(-5, 5)])), float(j * sy)] for i in range(cols) for j in range(rows)]
    return {"bolts": pts, "P": _lu(rng, 1e3, 2e5), "e": rng.uniform(0.02, 0.5) * rng.choice([1, -1])}


_add("eccentric_bolt_group",
     name="Eccentrically loaded bolt group (elastic method)", topic="machine elements / bolted joints",
     purpose="The direct shear per bolt, the largest resultant bolt force and which bolt carries it, for a group of identical bolts under a vertical load applied with a horizontal eccentricity.",
     inputs=[_mat("bolts", "bolt positions, one row 'x y' per bolt, in mm"), _phys("P", "force", "vertical load (downward)"),
             _phys("e", "length", "horizontal distance of the load line from the bolt-group centroid (positive to the right)")],
     outputs=[_o("direct_kN", "number", "direct shear per bolt, kN"), _o("max_bolt_force_kN", "number", "largest resultant bolt force, kN"),
              _o("critical_bolt", "integer", "number of the most loaded bolt (1-based, the first if several are equal)"), _o("polar_sum_mm2", "number", "sum of r^2 about the centroid, mm^2")],
     primary="max_bolt_force_kN", conditions=["identical bolts, rigid plate, elastic method", "2 to 40 bolts, not all at one point"],
     method="direct F/n plus torsional M r_i / sum r^2 perpendicular to r_i, with M = P e about the bolt centroid; vector sum per bolt",
     latex=r"\mathbf F_i = -\frac{P}{n}\hat{\mathbf y} + \frac{Pe}{\sum r_j^2}\,(y_i-\bar y,\; -(x_i-\bar x))",
     run=bolt_group,
     parse=lambda q: _pairs(q, r"eccentric bolt group|bolt group", {"bolts": "bolts", "P": "P", "e": "e"}, phys={"P", "e"}, matrices={"bolts"}),
     reference=ref_bolt_group,
     wrong=_safe(lambda bolts, P, e: (lambda r: dict(r, max_bolt_force_kN=r["direct_kN"] + abs(r["max_bolt_force_kN"] - r["direct_kN"]) * 2 + 1.0))(bolt_group(bolts, P, e))),
     random_inputs=_rand_bolts,
     templates=["Bolt group: bolts = [0 0; 0 100; 100 0; 100 100], P = 20 kN, e = 250 mm"],
     examples=[{"inputs": {"bolts": "0 0; 0 100; 100 0; 100 100", "P": 20e3, "e": 0.25},
                "outputs": {"direct_kN": 5.0, "polar_sum_mm2": 20000.0, "max_bolt_force_kN": math.hypot(20e3 * 250 * 50 / 20000, 5e3 + 20e3 * 250 * 50 / 20000) / 1e3, "critical_bolt": 3}}],
     questions=[{"q": "Bolt group: bolts = [0 0; 0 100; 100 0; 100 100], P = 20 kN, e = 250 mm", "outputs": {"direct_kN": 5.0, "critical_bolt": 3}},
                {"q": "Bolt group: bolts = [0 0; 0 0], P = 20 kN, e = 250 mm", "outcome": "INVALID_INPUT"}])


# ================================================================================================ fatigue
def _fatpack_load_line(sa, sm, Se, strength):
    """Factor of safety along the proportional load line, with fatpack's own mean-stress corrector (Goodman form
    S/(1 - Sm/Su), here with the strength given): the n at which the corrected amplitude of (n sa, n sm) reaches Se,
    found by scipy brentq."""
    import fatpack
    import numpy as np
    from scipy.optimize import brentq
    f = lambda n: float(fatpack.find_goodman_equivalent_stress(np.array([2 * n * sa]), np.array([n * sm]), strength)[0]) / 2 - Se
    hi = 0.999999 * strength / sm if sm > 0 else 1e6 * Se / sa
    return brentq(f, 1e-12, hi, xtol=1e-15, rtol=8.9e-16, maxiter=500)


def _fat_check(sa, sm, Se, S):
    _pos("the alternating stress, Se and the strength", sa, Se, S)
    _nonneg("the mean stress (compressive means are not covered)", sm)
    if sm >= S:
        raise ValueError("the mean stress must be below the strength")


def goodman(sa, sm, Se, Sut, Sy=None):
    _fat_check(sa, sm, Se, Sut)
    n = 1.0 / (sa / Se + sm / Sut)
    out = {"n_fatigue": n, "sigma_ar_MPa": sa / (1 - sm / Sut) / 1e6}
    if Sy is not None:
        _pos("Sy", Sy)
        out["n_yield"] = Sy / (sa + sm)
    return _out(**out)


def ref_goodman(sa, sm, Se, Sut, Sy=None):
    """fatpack's Goodman mean-stress corrector: the equivalent fully reversed amplitude directly, and the safety
    factor as the load-line multiple at which that amplitude reaches Se (scipy brentq)."""
    import fatpack
    import numpy as np
    out = {"n_fatigue": _fatpack_load_line(sa, sm, Se, Sut),
           "sigma_ar_MPa": float(fatpack.find_goodman_equivalent_stress(np.array([2 * sa]), np.array([sm]), Sut)[0]) / 2 / 1e6}
    if Sy is not None:
        out["n_yield"] = Sy / (sa + sm)
    return out


_FAT_IN = [_phys("sa", "stress", "alternating stress amplitude"), _phys("sm", "stress", "mean stress (>= 0)"), _phys("Se", "stress", "fully corrected endurance limit")]

_add("goodman_fatigue_safety_factor",
     name="Fatigue safety factor by the modified Goodman criterion", topic="fatigue / mean-stress criteria",
     purpose="The fatigue factor of safety along a proportional load line by the modified Goodman line, the equivalent fully reversed amplitude, and (with Sy) Langer's first-cycle yield factor.",
     inputs=_FAT_IN + [_phys("Sut", "stress", "ultimate tensile strength"), _phys("Sy", "stress", "yield strength (optional: first-cycle yield check)", required=False)],
     outputs=[_o("n_fatigue", "number", "fatigue safety factor (Goodman, proportional loading)"), _o("sigma_ar_MPa", "number", "equivalent fully reversed amplitude sa/(1 - sm/Sut), MPa"),
              _o("n_yield", "number", "Langer first-cycle yield factor Sy/(sa + sm) (with Sy)")],
     primary="n_fatigue", conditions=["sm >= 0 (compressive means not covered)", "sm < Sut", "proportional loading (the load line through the origin)"],
     method="1/n = sa/Se + sm/Sut; sigma_ar = sa/(1 - sm/Sut); n_y = Sy/(sa + sm)",
     latex=r"\frac{\sigma_a}{S_e} + \frac{\sigma_m}{S_{ut}} = \frac1n",
     run=goodman,
     parse=lambda q: _pairs(q, r"goodman(?: fatigue)?(?: safety factor)?", {"sa": "sa", "sm": "sm", "Se": "Se", "Sut": "Sut", "Sy": "Sy"}, phys={"sa", "sm", "Se", "Sut", "Sy"}),
     reference=ref_goodman,
     wrong=_safe(lambda sa, sm, Se, Sut, Sy=None: _out(n_fatigue=1 / (sa / Se + sm / (Sy or Sut) * 1.3), sigma_ar_MPa=sa / (1 - (sm / Sut) ** 2) / 1e6, n_yield=Sy / sa if Sy else 0.0)),
     random_inputs=lambda rng: (lambda Sut: {"sa": rng.uniform(20e6, 300e6), "sm": rng.uniform(0, 0.7) * Sut, "Se": rng.uniform(0.3, 0.5) * Sut, "Sut": Sut,
                                             **({"Sy": rng.uniform(0.55, 0.9) * Sut} if rng.random() < 0.5 else {})})(rng.uniform(400e6, 1400e6)),
     templates=["Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa"],
     examples=[{"inputs": {"sa": 120e6, "sm": 80e6, "Se": 250e6, "Sut": 700e6, "Sy": 500e6},
                "outputs": {"n_fatigue": 1 / (120 / 250 + 80 / 700), "sigma_ar_MPa": 120 / (1 - 80 / 700), "n_yield": 2.5}}],
     questions=[{"q": "Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa", "outputs": {"n_fatigue": 1 / (120 / 250 + 80 / 700)}},
                {"q": "Goodman safety factor: sa = 120 MPa, sm = -80 MPa, Se = 250 MPa, Sut = 700 MPa", "outcome": "INVALID_INPUT"}])


def soderberg(sa, sm, Se, Sy):
    _fat_check(sa, sm, Se, Sy)
    return _out(n_fatigue=1.0 / (sa / Se + sm / Sy), sigma_ar_MPa=sa / (1 - sm / Sy) / 1e6)


def ref_soderberg(sa, sm, Se, Sy):
    """fatpack's mean-stress corrector of the Goodman form with the yield strength in place of Sut (the Soderberg
    line); the safety factor along the load line by scipy brentq."""
    import fatpack
    import numpy as np
    return {"n_fatigue": _fatpack_load_line(sa, sm, Se, Sy),
            "sigma_ar_MPa": float(fatpack.find_goodman_equivalent_stress(np.array([2 * sa]), np.array([sm]), Sy)[0]) / 2 / 1e6}


_add("soderberg_fatigue_safety_factor",
     name="Fatigue safety factor by the Soderberg criterion", topic="fatigue / mean-stress criteria",
     purpose="The conservative Soderberg fatigue factor of safety (the line from Se to the yield strength) along a proportional load line, and the equivalent amplitude.",
     inputs=_FAT_IN + [_phys("Sy", "stress", "yield strength")],
     outputs=[_o("n_fatigue", "number", "fatigue safety factor (Soderberg)"), _o("sigma_ar_MPa", "number", "equivalent fully reversed amplitude sa/(1 - sm/Sy), MPa")],
     primary="n_fatigue", conditions=["sm >= 0", "sm < Sy", "proportional loading"],
     method="1/n = sa/Se + sm/Sy",
     latex=r"\frac{\sigma_a}{S_e} + \frac{\sigma_m}{S_y} = \frac1n",
     run=soderberg,
     parse=lambda q: _pairs(q, r"soderberg(?: fatigue)?(?: safety factor)?", {"sa": "sa", "sm": "sm", "Se": "Se", "Sy": "Sy"}, phys={"sa", "sm", "Se", "Sy"}),
     reference=ref_soderberg,
     wrong=_safe(lambda sa, sm, Se, Sy: _out(n_fatigue=1 / (sa / Se + sm / (2 * Sy)), sigma_ar_MPa=sa / (1 + sm / Sy) / 1e6)),
     random_inputs=lambda rng: (lambda Sy: {"sa": rng.uniform(20e6, 250e6), "sm": rng.uniform(0, 0.7) * Sy, "Se": rng.uniform(0.3, 0.6) * Sy, "Sy": Sy})(rng.uniform(250e6, 1200e6)),
     templates=["Soderberg safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, Sy = 450 MPa"],
     examples=[{"inputs": {"sa": 100e6, "sm": 60e6, "Se": 220e6, "Sy": 450e6}, "outputs": {"n_fatigue": 1 / (100 / 220 + 60 / 450)}}],
     questions=[{"q": "Soderberg safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, Sy = 450 MPa", "outputs": {"n_fatigue": 1 / (100 / 220 + 60 / 450)}},
                {"q": "Soderberg safety factor: sa = 100 MPa, sm = 500 MPa, Se = 220 MPa, Sy = 450 MPa", "outcome": "INVALID_INPUT"}])


def mean_stress_life(sa, sm, sf, b, method, gamma=None):
    _pos("sa and sf", sa, sf)
    _fin("sm", sm)
    if not -0.2 < b < 0:
        raise ValueError("the fatigue strength exponent b must be negative (typically -0.05 to -0.15)")
    smax = sm + sa
    if method in ("goodman", "morrow"):
        if sm >= sf:
            raise ValueError("the mean stress must be below the strength used by the correction")
        sar = sa / (1 - sm / sf)
    else:
        if smax <= 0:
            raise ValueError("SWT and Walker are not defined for a maximum stress <= 0")
        g = 0.5 if method == "swt" else gamma
        if g is None or not 0 < g <= 1:
            raise ValueError("the Walker exponent gamma must lie in (0, 1]")
        sar = smax ** (1 - g) * sa ** g
    Nf = 0.5 * (sar / sf) ** (1 / b)
    return _out(sigma_ar_MPa=sar / 1e6, Nf=Nf, log10_Nf=math.log10(Nf))


def ref_mean_stress_life(sa, sm, sf, b, method, gamma=None):
    """fatpack: its Morrow (strength sf), SWT and Walker mean-stress correctors on the stress range, then its
    LinearEnduranceCurve through (Nc = 1/2 cycle, Sc = 2 sf as a range) with slope m = -1/b for the life."""
    import fatpack
    import numpy as np
    S, Sm = np.array([2 * sa]), np.array([sm])
    if method in ("goodman", "morrow"):
        Seq = fatpack.find_morrow_equivalent_stress(S, Sm, sf)
    elif method == "swt":
        Seq = fatpack.find_swt_equivalent_stress(S, Sm)
    else:
        Seq = fatpack.find_walker_equivalent_stress(S, Sm, gamma)
    curve = fatpack.LinearEnduranceCurve(2 * sf)
    curve.Nc, curve.m = 0.5, -1.0 / b
    N = float(curve.get_endurance(np.array([float(Seq[0])]))[0])
    return {"sigma_ar_MPa": float(Seq[0]) / 2 / 1e6, "Nf": N, "log10_Nf": math.log10(N)}


_add("fatigue_life_mean_stress_correction",
     name="Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law", topic="fatigue / stress-life",
     purpose="The equivalent fully reversed stress amplitude of a cycle with a mean stress, by a chosen correction, and the cycles to failure from Basquin's law sigma_ar = sf' (2N)^b.",
     inputs=[_phys("sa", "stress", "stress amplitude"), _phys("sm", "stress", "mean stress"), _phys("sf", "stress", "fatigue strength coefficient sf' (Morrow and Goodman use it as the strength)"),
             _numin("b", "fatigue strength exponent (negative)"), _choice("method", ["morrow", "goodman", "swt", "walker"], "mean-stress correction"),
             _numin("gamma", "Walker exponent (walker only)", required=False)],
     outputs=[_o("sigma_ar_MPa", "number", "equivalent fully reversed amplitude, MPa"), _o("Nf", "number", "cycles to failure"), _o("log10_Nf", "number", "log10 of the cycles to failure")],
     primary="Nf", conditions=["-0.2 < b < 0", "Morrow/Goodman: sm < sf'", "SWT/Walker: sigma_max > 0", "goodman here uses sf' as the strength (Morrow form)"],
     method="Morrow: sigma_ar = sa/(1 - sm/sf'); SWT: sigma_ar = sqrt(sigma_max sa); Walker: sigma_max^(1-gamma) sa^gamma; Nf = (sigma_ar/sf')^(1/b)/2",
     latex=r"\sigma_{ar} = \frac{\sigma_a}{1-\sigma_m/\sigma_f'},\quad \sigma_{ar} = \sqrt{\sigma_{max}\sigma_a},\quad N_f = \tfrac12\Big(\frac{\sigma_{ar}}{\sigma_f'}\Big)^{1/b}",
     run=mean_stress_life,
     parse=lambda q: _pairs(q, r"fatigue life with mean stress(?: correction)?|mean stress correction", {"sa": "sa", "sm": "sm", "sf": "sf", "b": "b", "method": "method", "gamma": "gamma"},
                            phys={"sa", "sm", "sf"}, words={"method"}),
     reference=ref_mean_stress_life,
     wrong=_safe(lambda sa, sm, sf, b, method, gamma=None: (lambda sar: _out(sigma_ar_MPa=sar / 1e6, Nf=(sar / sf) ** (1 / b), log10_Nf=math.log10((sar / sf) ** (1 / b))))(sa / (1 - (sm / sf)) if method in ("goodman", "morrow") else sa)),
     random_inputs=lambda rng: (lambda sf, m: {"sa": rng.uniform(0.15, 0.5) * sf, "sm": rng.uniform(-0.1, 0.3) * sf, "sf": sf, "b": rng.uniform(-0.12, -0.06), "method": m,
                                               **({"gamma": rng.uniform(0.3, 0.8)} if m == "walker" else {})})(rng.uniform(600e6, 2000e6), rng.choice(["morrow", "goodman", "swt", "walker"])),
     templates=["Fatigue life with mean stress: sa = 300 MPa, sm = 100 MPa, sf = 1200 MPa, b = -0.09, method = swt"],
     examples=[{"inputs": {"sa": 300e6, "sm": 100e6, "sf": 1200e6, "b": -0.09, "method": "swt"},
                "outputs": {"sigma_ar_MPa": math.sqrt(400 * 300), "Nf": 0.5 * (math.sqrt(400 * 300) / 1200) ** (1 / -0.09)}},
               {"inputs": {"sa": 300e6, "sm": 100e6, "sf": 1200e6, "b": -0.09, "method": "morrow"}, "outputs": {"sigma_ar_MPa": 300 / (1 - 100 / 1200)}}],
     questions=[{"q": "Fatigue life with mean stress: sa = 300 MPa, sm = 100 MPa, sf = 1200 MPa, b = -0.09, method = swt", "outputs": {"sigma_ar_MPa": math.sqrt(400 * 300)}},
                {"q": "Fatigue life with mean stress: sa = 300 MPa, sm = 100 MPa, sf = 1200 MPa, b = -0.09, method = walker", "outcome": "INVALID_INPUT"}])


def sn_life(Sut, Se, f, sa):
    _pos("Sut, Se and sa", Sut, Se, sa)
    if not 0 < f < 1 or f * Sut <= Se:
        raise ValueError("need 0 < f < 1 and f*Sut > Se")
    if not Se < sa < f * Sut:
        raise ValueError("the amplitude must lie between Se (infinite life) and f*Sut (low-cycle range below 1000 cycles)")
    a = (f * Sut) ** 2 / Se
    b = -math.log10(f * Sut / Se) / 3
    N = (sa / a) ** (1 / b)
    return _out(a_MPa=a / 1e6, b=b, N=N, log10_N=math.log10(N))


def ref_sn_life(Sut, Se, f, sa):
    """fatpack's LinearEnduranceCurve through (1e6 cycles, Se) and (1e3 cycles, f Sut): its own endurance at sa."""
    import fatpack
    import numpy as np
    curve = fatpack.LinearEnduranceCurve(Se)
    curve.Nc, curve.m = 1e6, 3.0 / math.log10(f * Sut / Se)
    N = float(curve.get_endurance(np.array([sa]))[0])
    a = float(curve.get_stress(np.array([1.0]))[0])
    return {"a_MPa": a / 1e6, "b": -1.0 / curve.m, "N": N, "log10_N": math.log10(N)}


_add("sn_fatigue_life",
     name="Fatigue life from the S-N line (10^3 to 10^6 cycles)", topic="fatigue / stress-life",
     purpose="Cycles to failure of a fully reversed amplitude on the straight log-log S-N line through f*Sut at 10^3 cycles and Se at 10^6 cycles, with the line's constants a and b.",
     inputs=[_phys("Sut", "stress", "ultimate tensile strength"), _phys("Se", "stress", "endurance limit"), _numin("f", "fatigue strength fraction at 10^3 cycles (e.g. 0.9)"),
             _phys("sa", "stress", "fully reversed stress amplitude")],
     outputs=[_o("a_MPa", "number", "S-N constant a, MPa"), _o("b", "number", "S-N exponent b"), _o("N", "number", "cycles to failure"), _o("log10_N", "number", "log10 of the cycles to failure")],
     primary="N", conditions=["Se < sa < f Sut", "0 < f < 1", "fully reversed loading"],
     method="a = (f Sut)^2/Se, b = -log10(f Sut/Se)/3, N = (sa/a)^(1/b)",
     latex=r"a = \frac{(fS_{ut})^2}{S_e},\quad b = -\frac13\log_{10}\frac{fS_{ut}}{S_e},\quad N = \Big(\frac{\sigma_a}{a}\Big)^{1/b}",
     run=sn_life,
     parse=lambda q: _pairs(q, r"s-n fatigue life|fatigue life from (?:the )?s-n (?:line|curve)", {"Sut": "Sut", "Se": "Se", "f": "f", "sa": "sa"}, phys={"Sut", "Se", "sa"}),
     reference=ref_sn_life,
     wrong=_safe(lambda Sut, Se, f, sa: (lambda a, b: _out(a_MPa=a / 1e6, b=b, N=(sa / a) ** (1 / b), log10_N=math.log10((sa / a) ** (1 / b))))(f * Sut, -math.log10(f * Sut / Se) / 6)),
     random_inputs=lambda rng: (lambda Sut: (lambda Se, f: {"Sut": Sut, "Se": Se, "f": f, "sa": Se * rng.uniform(1.02, 0.98 * f * Sut / Se)})(rng.uniform(0.35, 0.5) * Sut, rng.uniform(0.77, 0.9)))(rng.uniform(400e6, 1400e6)),
     templates=["S-N fatigue life: Sut = 700 MPa, Se = 300 MPa, f = 0.85, sa = 400 MPa"],
     examples=[{"inputs": {"Sut": 700e6, "Se": 300e6, "f": 0.85, "sa": 400e6},
                "outputs": {"a_MPa": 595 ** 2 / 300, "b": -math.log10(595 / 300) / 3, "N": (400 / (595 ** 2 / 300)) ** (1 / (-math.log10(595 / 300) / 3))}}],
     questions=[{"q": "S-N fatigue life: Sut = 700 MPa, Se = 300 MPa, f = 0.85, sa = 400 MPa", "outputs": {"a_MPa": 595 ** 2 / 300}},
                {"q": "S-N fatigue life: Sut = 700 MPa, Se = 300 MPa, f = 0.85, sa = 250 MPa", "outcome": "INVALID_INPUT"}])


def sn_strength(Sut, Se, f, N):
    _pos("Sut and Se", Sut, Se)
    if not 0 < f < 1 or f * Sut <= Se:
        raise ValueError("need 0 < f < 1 and f*Sut > Se")
    if not 1e3 <= N <= 1e6:
        raise ValueError("N must lie between 10^3 and 10^6 cycles")
    a = (f * Sut) ** 2 / Se
    b = -math.log10(f * Sut / Se) / 3
    return _out(Sf_MPa=a * N ** b / 1e6)


def ref_sn_strength(Sut, Se, f, N):
    import fatpack
    import numpy as np
    curve = fatpack.LinearEnduranceCurve(Se)
    curve.Nc, curve.m = 1e6, 3.0 / math.log10(f * Sut / Se)
    return {"Sf_MPa": float(curve.get_stress(np.array([float(N)]))[0]) / 1e6}


_add("sn_fatigue_strength",
     name="Fatigue strength at a given number of cycles (S-N line)", topic="fatigue / stress-life",
     purpose="The fully reversed fatigue strength at N cycles (10^3 to 10^6) on the straight log-log S-N line through f*Sut at 10^3 and Se at 10^6 cycles.",
     inputs=[_phys("Sut", "stress", "ultimate tensile strength"), _phys("Se", "stress", "endurance limit"), _numin("f", "fatigue strength fraction at 10^3 cycles"),
             _numin("N", "number of cycles (10^3 to 10^6)")],
     outputs=[_o("Sf_MPa", "number", "fatigue strength at N cycles, MPa")],
     primary="Sf_MPa", conditions=["10^3 <= N <= 10^6", "0 < f < 1, f Sut > Se"],
     method="Sf = a N^b with a = (f Sut)^2/Se, b = -log10(f Sut/Se)/3",
     latex=r"S_f = aN^b",
     run=sn_strength,
     parse=lambda q: _pairs(q, r"s-n fatigue strength", {"Sut": "Sut", "Se": "Se", "f": "f", "N": "N"}, phys={"Sut", "Se"}),
     reference=ref_sn_strength,
     wrong=_safe(lambda Sut, Se, f, N: _out(Sf_MPa=f * Sut * (N / 1e3) ** (-math.log10(f * Sut / Se) / 6) / 1e6)),
     random_inputs=lambda rng: (lambda Sut: {"Sut": Sut, "Se": rng.uniform(0.35, 0.5) * Sut, "f": rng.uniform(0.77, 0.9), "N": round(10 ** rng.uniform(3, 6))})(rng.uniform(400e6, 1400e6)),
     templates=["S-N fatigue strength: Sut = 700 MPa, Se = 300 MPa, f = 0.85, N = 50000"],
     examples=[{"inputs": {"Sut": 700e6, "Se": 300e6, "f": 0.85, "N": 1e3}, "outputs": {"Sf_MPa": 595.0}},
               {"inputs": {"Sut": 700e6, "Se": 300e6, "f": 0.85, "N": 1e6}, "outputs": {"Sf_MPa": 300.0}}],
     questions=[{"q": "S-N fatigue strength: Sut = 700 MPa, Se = 300 MPa, f = 0.85, N = 1000000", "outputs": {"Sf_MPa": 300.0}},
                {"q": "S-N fatigue strength: Sut = 700 MPa, Se = 300 MPa, f = 0.85, N = 100", "outcome": "INVALID_INPUT"}])


def _counts_check(ranges, counts):
    if len(ranges) != len(counts):
        raise ValueError("ranges and counts must have the same number of entries")
    for v in ranges:
        _pos("every stress range", v)
    for v in counts:
        _pos("every cycle count", v)


def miner_linear(ranges, counts, Sc, m, Nc=2e6):
    _counts_check(ranges, counts)
    _pos("Sc, m and Nc", Sc, m, Nc)
    D = math.fsum(n / (Nc * (Sc / s) ** m) for s, n in zip(ranges, counts))
    return _out(damage=D, repeats_to_failure=1 / D)


def ref_miner_linear(ranges, counts, Sc, m, Nc=2e6):
    """fatpack: a LinearEnduranceCurve (detail category Sc at Nc, slope m) and its own Miner sum over (range, count)."""
    import fatpack
    import numpy as np
    curve = fatpack.LinearEnduranceCurve(Sc)
    curve.m, curve.Nc = m, Nc
    D = float(curve.find_miner_sum(np.array(list(zip(ranges, counts)), dtype=float)))
    return {"damage": D, "repeats_to_failure": 1 / D}


_add("miner_damage_sn_curve",
     name="Miner's rule damage with a single-slope S-N curve", topic="fatigue / cumulative damage",
     purpose="The Palmgren-Miner damage sum of stress-range blocks on an S-N curve N = Nc (Sc/S)^m, and how many repeats of the spectrum the part survives.",
     inputs=[_vec("ranges", "stress ranges in MPa (a list)"), _vec("counts", "number of cycles of each range (a list)"), _numin("Sc", "reference stress range of the curve at Nc cycles, MPa"),
             _numin("m", "S-N slope (e.g. 3 for welded details)"), _numin("Nc", "cycles at Sc (default 2e6)", required=False)],
     outputs=[_o("damage", "number", "Miner damage sum D"), _o("repeats_to_failure", "number", "repeats of the spectrum until D = 1")],
     primary="damage", conditions=["linear damage accumulation (Palmgren-Miner)", "a single-slope curve without endurance limit"],
     method="D = sum n_i / N_i, N_i = Nc (Sc/S_i)^m",
     latex=r"D = \sum_i\frac{n_i}{N_i},\quad N_i = N_c\Big(\frac{S_c}{S_i}\Big)^m",
     run=miner_linear,
     parse=lambda q: _pairs(q, r"miner'?s rule(?: damage)?|miner damage", {"ranges": "ranges", "S": "ranges", "counts": "counts", "n": "counts", "Sc": "Sc", "m": "m", "Nc": "Nc"},
                            vectors={"ranges", "counts"}),
     reference=ref_miner_linear,
     wrong=_safe(lambda ranges, counts, Sc, m, Nc=2e6: (lambda D: _out(damage=D, repeats_to_failure=1 / D))(sum(n / (Nc * (Sc / s) ** (m - 1)) for s, n in zip(ranges, counts)))),
     random_inputs=lambda rng: (lambda k: {"ranges": [round(rng.uniform(30, 250), 1) for _ in range(k)], "counts": [float(rng.randint(1000, 500000)) for _ in range(k)],
                                           "Sc": rng.choice([36.0, 50.0, 71.0, 90.0, 112.0]), "m": rng.choice([3.0, 4.0, 5.0])})(rng.randint(1, 6)),
     templates=["Miner's rule: ranges = [120, 80, 50], counts = [20000, 100000, 500000], Sc = 90, m = 3"],
     examples=[{"inputs": {"ranges": "90", "counts": "1000000", "Sc": 90.0, "m": 3.0}, "outputs": {"damage": 0.5, "repeats_to_failure": 2.0}}],
     questions=[{"q": "Miner's rule: ranges = [90], counts = [1000000], Sc = 90, m = 3", "outputs": {"damage": 0.5}},
                {"q": "Miner's rule: ranges = [120, 80], counts = [20000], Sc = 90, m = 3", "outcome": "INVALID_INPUT"}])


def eurocode_damage(ranges, counts, dsc):
    _counts_check(ranges, counts)
    _pos("the detail category", dsc)
    sD = dsc * (2 / 5) ** (1 / 3)
    sL = sD * (5e6 / 1e8) ** (1 / 5)
    D = 0.0
    terms = []
    for s, n in zip(ranges, counts):
        if s >= sD:
            terms.append(n / (2e6 * (dsc / s) ** 3))
        elif s >= sL:
            terms.append(n / (5e6 * (sD / s) ** 5))
    D = math.fsum(terms)
    if D == 0:
        raise ValueError("every range is below the cut-off limit: no damage")
    return _out(damage=D, delta_sigma_D=sD, delta_sigma_L=sL, repeats_to_failure=1 / D)


def ref_eurocode_damage(ranges, counts, dsc):
    """fatpack's TriLinearEnduranceCurve (slopes 3 and 5, knees at 2e6, 5e6 and the cut-off at 1e8 cycles) and its
    Miner sum."""
    import fatpack
    import numpy as np
    curve = fatpack.TriLinearEnduranceCurve(dsc)
    D = float(curve.find_miner_sum(np.array(list(zip(ranges, counts)), dtype=float)))
    return {"damage": D, "delta_sigma_D": float(curve.Sd), "delta_sigma_L": float(curve.Sl), "repeats_to_failure": 1 / D}


_add("eurocode_fatigue_damage",
     name="Fatigue damage on the trilinear (Eurocode 3) S-N curve", topic="fatigue / cumulative damage",
     purpose="The Miner damage of stress-range blocks on the trilinear S-N curve of a detail category (slope 3 to 5e6 cycles, slope 5 to 1e8, cut-off below), with the constant-amplitude and cut-off limits.",
     inputs=[_vec("ranges", "stress ranges in MPa (a list)"), _vec("counts", "cycles of each range (a list)"), _numin("dsc", "detail category: stress range at 2e6 cycles, MPa")],
     outputs=[_o("damage", "number", "Miner damage sum"), _o("delta_sigma_D", "number", "constant-amplitude fatigue limit (5e6 cycles), MPa"),
              _o("delta_sigma_L", "number", "cut-off limit (1e8 cycles), MPa"), _o("repeats_to_failure", "number", "repeats of the spectrum until D = 1")],
     primary="damage", conditions=["partial safety factors not applied (gamma_Mf = gamma_Ff = 1)", "at least one range above the cut-off"],
     method="N = 2e6 (dsc/S)^3 for S >= sD = 0.737 dsc; N = 5e6 (sD/S)^5 for sL <= S < sD with sL = 0.549 sD; no damage below sL",
     latex=r"N = 2\cdot10^6\Big(\frac{\Delta\sigma_C}{\Delta\sigma}\Big)^3\ (\Delta\sigma\ge\Delta\sigma_D),\quad N = 5\cdot10^6\Big(\frac{\Delta\sigma_D}{\Delta\sigma}\Big)^5",
     run=eurocode_damage,
     parse=lambda q: _pairs(q, r"eurocode fatigue damage|trilinear s-n damage", {"ranges": "ranges", "counts": "counts", "dsc": "dsc", "detail_category": "dsc"}, vectors={"ranges", "counts"}),
     reference=ref_eurocode_damage,
     wrong=_safe(lambda ranges, counts, dsc: (lambda D: _out(damage=D, delta_sigma_D=dsc * 0.737, delta_sigma_L=dsc * 0.405, repeats_to_failure=1 / D))(
         sum(n / (2e6 * (dsc / s) ** 3) for s, n in zip(ranges, counts)))),
     random_inputs=lambda rng: (lambda k, dsc: {"ranges": [round(rng.uniform(0.3, 2.5) * dsc, 1) for _ in range(k)], "counts": [float(rng.randint(1000, 5000000)) for _ in range(k)],
                                                "dsc": dsc})(rng.randint(2, 6), rng.choice([36.0, 45.0, 56.0, 71.0, 80.0, 90.0, 112.0, 125.0])),
     templates=["Eurocode fatigue damage: ranges = [150, 60, 40], counts = [10000, 2000000, 50000000], dsc = 90"],
     examples=[{"inputs": {"ranges": "90", "counts": "1000000", "dsc": 90.0}, "outputs": {"damage": 0.5, "delta_sigma_D": 90 * 0.4 ** (1 / 3)}}],
     questions=[{"q": "Eurocode fatigue damage: ranges = [90], counts = [1000000], dsc = 90", "outputs": {"damage": 0.5}},
                {"q": "Eurocode fatigue damage: ranges = [10], counts = [1000000], dsc = 90", "outcome": "INVALID_INPUT"}])


def _reversals(series):
    """Peaks and valleys of a series, keeping its first and last points (ASTM E1049-85); flat stretches are skipped."""
    out = [series[0]]
    x, d_last = series[1], series[1] - series[0]
    for x_next in series[2:]:
        if x_next == x:
            continue
        d_next = x_next - x
        if d_last * d_next < 0:
            out.append(x)
        x, d_last = x_next, d_next
    out.append(series[-1])
    return out


def _rainflow(series):
    """ASTM E1049-85 three-point rainflow counting: (range, mean, count) with half cycles for the residue."""
    pts, cyc = [], []
    for p in _reversals(series):
        pts.append(p)
        while len(pts) >= 3:
            X, Y = abs(pts[-1] - pts[-2]), abs(pts[-2] - pts[-3])
            if X < Y:
                break
            if len(pts) == 3:
                cyc.append((Y, (pts[0] + pts[1]) / 2, 0.5))
                pts.pop(0)
            else:
                cyc.append((Y, (pts[-3] + pts[-2]) / 2, 1.0))
                last = pts.pop()
                pts.pop()
                pts.pop()
                pts.append(last)
    for i in range(len(pts) - 1):
        cyc.append((abs(pts[i + 1] - pts[i]), (pts[i] + pts[i + 1]) / 2, 0.5))
    return cyc


def _hist_check(history):
    if len(history) < 3:
        raise ValueError("give at least 3 points of the load history")
    _fin("every point", *history)
    if max(history) == min(history):
        raise ValueError("the history is constant: there are no cycles")


def rainflow_count(history):
    _hist_check(history)
    agg = {}
    for rng_, _m, c in _rainflow(list(history)):
        if rng_ > 0:
            k = _r(rng_)
            agg[k] = agg.get(k, 0.0) + c
    keys = sorted(agg, reverse=True)
    return _out(ranges=keys, counts=[agg[k] for k in keys], total_cycles=sum(agg.values()), max_range=keys[0])


def ref_rainflow_count(history):
    """The `rainflow` package (ASTM E1049-85 counting with half cycles); ranges aggregated as it aggregates them."""
    import rainflow
    agg = {}
    for rng_, c in rainflow.count_cycles(list(history)):
        if rng_ > 0:
            k = _r(float(rng_))
            agg[k] = agg.get(k, 0.0) + c
    keys = sorted(agg, reverse=True)
    return {"ranges": keys, "counts": [agg[k] for k in keys], "total_cycles": sum(agg.values()), "max_range": keys[0]}


def _rand_history(rng):
    return {"history": [float(rng.randint(-100, 100)) for _ in range(rng.randint(5, 40))]}


_add("rainflow_counting",
     name="Rainflow cycle counting of a load history (ASTM E1049)", topic="fatigue / cycle counting",
     purpose="The cycles in a load or stress history by rainflow counting (three-point method, half cycles for the residue), as distinct ranges with their counts.",
     inputs=[_vec("history", "the load or stress history, point by point (a list; any one unit)", lo=3, hi=2000)],
     outputs=[_o("ranges", "number", "distinct ranges, largest first (in the history's unit)"), _o("counts", "number", "cycles of each range (half cycles count 0.5)"),
              _o("total_cycles", "number", "total number of cycles"), _o("max_range", "number", "largest range")],
     primary="total_cycles", conditions=["at least 3 points, not constant", "ASTM E1049-85: the first and last points are reversals, the residue counts as half cycles",
                                         "ranges equal to 12 significant digits are merged"],
     method="reversals (peaks and valleys), then the three-point rule: when a range Y is not larger than the next range X, count Y (a half cycle if it contains the start) and remove its points",
     latex=r"|x_3-x_2| \ge |x_2-x_1| \Rightarrow \text{count } |x_2-x_1|",
     run=rainflow_count,
     parse=lambda q: _pairs(q, r"rainflow count(?:ing)?|rainflow", {"history": "history", "loads": "history", "series": "history"}, vectors={"history"}),
     reference=ref_rainflow_count,
     wrong=_safe(lambda history: (lambda r: dict(r, total_cycles=r["total_cycles"] + 0.5 * len(r["ranges"]), counts=[1.0] * len(r["ranges"])))(rainflow_count(history))),
     random_inputs=_rand_history,
     templates=["Rainflow counting: history = [0, 5, -3, 4, -2, 6, -4, 1, 0]"],
     examples=[{"inputs": {"history": "0, 5, -3, 4, -2, 6, -4, 1, 0"},
                "outputs": {"ranges": [10.0, 9.0, 8.0, 6.0, 5.0, 1.0], "counts": [0.5, 0.5, 0.5, 1.0, 1.0, 0.5], "total_cycles": 4.0}}],
     questions=[{"q": "Rainflow counting: history = [0, 5, -3, 4, -2, 6, -4, 1, 0]", "outputs": {"total_cycles": 4.0, "max_range": 10.0}},
                {"q": "Rainflow counting: history = [2, 2, 2]", "outcome": "INVALID_INPUT"}])


def rainflow_damage(history, Sc, m, Nc=2e6):
    _hist_check(history)
    _pos("Sc, m and Nc", Sc, m, Nc)
    cyc = [(r_, c) for r_, _m, c in _rainflow(list(history)) if r_ > 0]
    D = math.fsum(c * (r_ / Sc) ** m / Nc for r_, c in cyc)
    return _out(damage=D, repeats_to_failure=1 / D, cycles=sum(c for _r_, c in cyc))


def ref_rainflow_damage(history, Sc, m, Nc=2e6):
    """The `rainflow` package for the cycles (with half cycles) and fatpack's LinearEnduranceCurve Miner sum."""
    import fatpack
    import numpy as np
    import rainflow
    cyc = [(float(r_), float(c)) for r_, _mn, c, _i, _j in rainflow.extract_cycles(list(history)) if r_ > 0]
    curve = fatpack.LinearEnduranceCurve(Sc)
    curve.m, curve.Nc = m, Nc
    D = float(curve.find_miner_sum(np.array(cyc)))
    return {"damage": D, "repeats_to_failure": 1 / D, "cycles": sum(c for _r_, c in cyc)}


_add("rainflow_fatigue_damage",
     name="Fatigue damage of a stress history (rainflow counting + Miner's rule)", topic="fatigue / cumulative damage",
     purpose="The Miner damage of one pass of a stress history: rainflow cycles (ASTM, half cycles) on a single-slope S-N curve N = Nc (Sc/S)^m, and the passes to failure.",
     inputs=[_vec("history", "the stress history in MPa, point by point (a list)", lo=3, hi=2000), _numin("Sc", "reference stress range at Nc cycles, MPa"),
             _numin("m", "S-N slope"), _numin("Nc", "cycles at Sc (default 2e6)", required=False)],
     outputs=[_o("damage", "number", "damage of one pass of the history"), _o("repeats_to_failure", "number", "passes until D = 1"), _o("cycles", "number", "cycles counted per pass")],
     primary="damage", conditions=["ASTM E1049-85 rainflow counting, half cycles weighted 0.5", "no mean-stress correction", "single-slope S-N curve, no endurance limit"],
     method="rainflow ranges S_i with counts n_i; D = sum n_i S_i^m/(Nc Sc^m)",
     latex=r"D = \sum_i \frac{n_i}{N_c}\Big(\frac{S_i}{S_c}\Big)^m",
     run=rainflow_damage,
     parse=lambda q: _pairs(q, r"rainflow fatigue damage|rainflow damage", {"history": "history", "Sc": "Sc", "m": "m", "Nc": "Nc"}, vectors={"history"}),
     reference=ref_rainflow_damage,
     wrong=_safe(lambda history, Sc, m, Nc=2e6: (lambda D: _out(damage=D, repeats_to_failure=1 / D, cycles=0.0))(
         sum(abs(b - a) ** m / (Nc * Sc ** m) for a, b in zip(history, history[1:]) if b != a))),
     random_inputs=lambda rng: {"history": [float(rng.randint(-150, 150)) for _ in range(rng.randint(5, 40))], "Sc": rng.choice([50.0, 71.0, 90.0]), "m": rng.choice([3.0, 5.0])},
     templates=["Rainflow fatigue damage: history = [0, 120, -40, 80, -100, 60, 0], Sc = 90, m = 3"],
     examples=[{"inputs": {"history": "0, 90, 0", "Sc": 90.0, "m": 3.0}, "outputs": {"damage": 1 / 2e6, "cycles": 1.0}}],
     questions=[{"q": "Rainflow fatigue damage: history = [0, 90, 0], Sc = 90, m = 3", "outputs": {"damage": 5e-7}},
                {"q": "Rainflow fatigue damage: history = [0, 90], Sc = 90, m = 3", "outcome": "INVALID_INPUT"}])


def _de_stresses(d, Ma, Mm, Ta, Tm, Kf, Kfs):
    c = 1 / (math.pi * d ** 3)
    sa = math.sqrt((32 * Kf * Ma * c) ** 2 + 3 * (16 * Kfs * Ta * c) ** 2)
    sm = math.sqrt((32 * Kf * Mm * c) ** 2 + 3 * (16 * Kfs * Tm * c) ** 2)
    return sa, sm


def _de_check(Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut):
    _nonneg("the moments and torques", Ma, Mm, Ta, Tm)
    if Ma == 0 and Ta == 0:
        raise ValueError("there is no alternating load: a static check, not a fatigue one")
    if Kf < 1 or Kfs < 1:
        raise ValueError("fatigue stress-concentration factors are >= 1")
    _pos("Se and Sut", Se, Sut)


def shaft_de_goodman_n(d, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut, Sy=None):
    _pos("d", d)
    _de_check(Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut)
    sa, sm = _de_stresses(d, Ma, Mm, Ta, Tm, Kf, Kfs)
    if sm >= Sut:
        raise ValueError("the mean von Mises stress reaches Sut")
    out = {"n_fatigue": 1 / (sa / Se + sm / Sut), "sigma_a_vm_MPa": sa / 1e6, "sigma_m_vm_MPa": sm / 1e6}
    if Sy is not None:
        _pos("Sy", Sy)
        smax = math.sqrt((32 * Kf * (Mm + Ma) / (math.pi * d ** 3)) ** 2 + 3 * (16 * Kfs * (Tm + Ta) / (math.pi * d ** 3)) ** 2)
        out["n_yield"] = Sy / smax
    return _out(**out)


def _ref_de_vm(d, M, T, Kf, Kfs):
    """von Mises stress of the shaft surface (bending sigma, torsion tau) from numpy's deviatoric stress tensor."""
    import numpy as np
    s, t = 32 * Kf * M / (math.pi * d ** 3), 16 * Kfs * T / (math.pi * d ** 3)
    S = np.array([[s, t, 0.0], [t, 0.0, 0.0], [0.0, 0.0, 0.0]])
    dev = S - np.trace(S) / 3 * np.eye(3)
    return math.sqrt(1.5 * float(np.sum(dev * dev)))


def ref_shaft_de_goodman_n(d, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut, Sy=None):
    """von Mises alternating and mean stresses from numpy stress tensors, then fatpack's Goodman corrector along the
    load line (scipy brentq)."""
    sa, sm = _ref_de_vm(d, Ma, Ta, Kf, Kfs), _ref_de_vm(d, Mm, Tm, Kf, Kfs)
    out = {"n_fatigue": _fatpack_load_line(sa, sm, Se, Sut), "sigma_a_vm_MPa": sa / 1e6, "sigma_m_vm_MPa": sm / 1e6}
    if Sy is not None:
        out["n_yield"] = Sy / _ref_de_vm(d, Ma + Mm, Ta + Tm, Kf, Kfs)
    return out


_SHAFT_IN = [_phys("Ma", "bending_moment", "alternating bending moment"), _phys("Mm", "bending_moment", "mean bending moment"),
             _phys("Ta", "torque", "alternating torque"), _phys("Tm", "torque", "mean torque"), _numin("Kf", "fatigue stress-concentration factor, bending"),
             _numin("Kfs", "fatigue stress-concentration factor, torsion"), _phys("Se", "stress", "fully corrected endurance limit"), _phys("Sut", "stress", "ultimate tensile strength")]
_SHAFT_NAMES = {k: k for k in ("d", "n", "Ma", "Mm", "Ta", "Tm", "Kf", "Kfs", "Se", "Sut", "Sy")}


def _rand_shaft(rng, with_d=True):
    Sut = rng.uniform(500e6, 1200e6)
    d = _lu(rng, 0.02, 0.1)
    scale = math.pi * d ** 3 / 32 * Sut
    out = {"Ma": rng.uniform(0.02, 0.15) * scale, "Mm": rng.choice([0.0, rng.uniform(0, 0.1) * scale]), "Ta": rng.choice([0.0, rng.uniform(0, 0.1) * scale]),
           "Tm": rng.uniform(0, 0.15) * scale, "Kf": rng.uniform(1.0, 2.5), "Kfs": rng.uniform(1.0, 2.0), "Se": rng.uniform(0.3, 0.5) * Sut, "Sut": Sut}
    if with_d:
        out["d"] = d
        if rng.random() < 0.5:
            out["Sy"] = rng.uniform(0.6, 0.9) * Sut
    else:
        out["n"] = rng.uniform(1.2, 3.0)
    return out


_add("shaft_fatigue_safety_factor",
     name="Shaft fatigue safety factor (DE-Goodman)", topic="fatigue / shaft design",
     purpose="The fatigue factor of safety of a rotating solid shaft under alternating and mean bending and torsion by the distortion-energy Goodman criterion, with the von Mises stresses and (with Sy) the first-cycle yield factor.",
     inputs=[_phys("d", "length", "shaft diameter")] + _SHAFT_IN + [_phys("Sy", "stress", "yield strength (optional)", required=False)],
     outputs=[_o("n_fatigue", "number", "fatigue safety factor (DE-Goodman)"), _o("sigma_a_vm_MPa", "number", "alternating von Mises stress, MPa"),
              _o("sigma_m_vm_MPa", "number", "mean von Mises stress, MPa"), _o("n_yield", "number", "first-cycle yield factor (with Sy)")],
     primary="n_fatigue", conditions=["solid round shaft, no axial load", "Kf, Kfs >= 1", "an alternating moment or torque present"],
     method="sigma_a' = sqrt((32 Kf Ma/(pi d^3))^2 + 3(16 Kfs Ta/(pi d^3))^2), likewise sigma_m'; 1/n = sigma_a'/Se + sigma_m'/Sut",
     latex=r"\frac1n = \frac{16}{\pi d^3}\Big\{\frac{\sqrt{4(K_fM_a)^2+3(K_{fs}T_a)^2}}{S_e} + \frac{\sqrt{4(K_fM_m)^2+3(K_{fs}T_m)^2}}{S_{ut}}\Big\}",
     run=shaft_de_goodman_n,
     parse=lambda q: _pairs(q, r"shaft fatigue safety factor|de-goodman safety factor", _SHAFT_NAMES, phys={"d", "Ma", "Mm", "Ta", "Tm", "Se", "Sut", "Sy"}),
     reference=ref_shaft_de_goodman_n,
     wrong=_safe(lambda d, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut, Sy=None: (lambda sa, sm: _out(n_fatigue=1 / (sa / Se + sm / Sut), sigma_a_vm_MPa=sa / 1e6, sigma_m_vm_MPa=sm / 1e6,
                                                                                      n_yield=1.0))(32 * Kf * Ma / (math.pi * d ** 3) + 16 * Kfs * Ta / (math.pi * d ** 3), 32 * Kf * Mm / (math.pi * d ** 3) + 16 * Kfs * Tm / (math.pi * d ** 3))),
     random_inputs=lambda rng: _rand_shaft(rng, True),
     templates=["Shaft fatigue safety factor: d = 40 mm, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa"],
     examples=[{"inputs": {"d": 0.04, "Ma": 150.0, "Mm": 0.0, "Ta": 0.0, "Tm": 200.0, "Kf": 1.6, "Kfs": 1.4, "Se": 200e6, "Sut": 700e6},
                "outputs": {"n_fatigue": 1 / (32 * 1.6 * 150 / (math.pi * 0.04 ** 3) / 200e6 + math.sqrt(3) * 16 * 1.4 * 200 / (math.pi * 0.04 ** 3) / 700e6)}}],
     questions=[{"q": "Shaft fatigue safety factor: d = 40 mm, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa",
                 "outputs": {"n_fatigue": 1 / (32 * 1.6 * 150 / (math.pi * 0.04 ** 3) / 200e6 + math.sqrt(3) * 16 * 1.4 * 200 / (math.pi * 0.04 ** 3) / 700e6)}},
                {"q": "Shaft fatigue safety factor: d = 40 mm, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 0.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa", "outcome": "INVALID_INPUT"}])


def shaft_de_goodman_d(n, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut):
    _de_check(Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut)
    if not n >= 1:
        raise ValueError("the design factor n must be at least 1")
    A = math.sqrt(4 * (Kf * Ma) ** 2 + 3 * (Kfs * Ta) ** 2)
    B = math.sqrt(4 * (Kf * Mm) ** 2 + 3 * (Kfs * Tm) ** 2)
    d = (16 * n / math.pi * (A / Se + B / Sut)) ** (1 / 3)
    return _out(d_mm=d * 1e3)


def ref_shaft_de_goodman_d(n, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut):
    """The diameter at which the reference safety factor (numpy von Mises + fatpack Goodman load line) equals n,
    found by scipy brentq."""
    from scipy.optimize import brentq
    f = lambda d: ref_shaft_de_goodman_n(d, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut)["n_fatigue"] - n
    lo, hi = 1e-4, 1e-3
    while f(hi) < 0:
        lo, hi = hi, hi * 2
    while f(lo) > 0:
        lo /= 2
    return {"d_mm": brentq(f, lo, hi, xtol=1e-16, rtol=8.9e-16, maxiter=500) * 1e3}


_add("shaft_diameter_de_goodman",
     name="Shaft diameter for a design factor (DE-Goodman)", topic="fatigue / shaft design",
     purpose="The smallest solid-shaft diameter giving a required fatigue design factor under alternating and mean bending and torsion (distortion-energy Goodman).",
     inputs=[_numin("n", "required fatigue design factor (>= 1)")] + _SHAFT_IN,
     outputs=[_o("d_mm", "number", "required diameter, mm")],
     primary="d_mm", conditions=["solid round shaft", "the stress-concentration factors for the final size are assumed known", "n >= 1"],
     method="d = (16n/pi [sqrt(4(Kf Ma)^2 + 3(Kfs Ta)^2)/Se + sqrt(4(Kf Mm)^2 + 3(Kfs Tm)^2)/Sut])^(1/3)",
     latex=r"d = \Big(\frac{16n}{\pi}\Big\{\frac{\sqrt{4(K_fM_a)^2+3(K_{fs}T_a)^2}}{S_e} + \frac{\sqrt{4(K_fM_m)^2+3(K_{fs}T_m)^2}}{S_{ut}}\Big\}\Big)^{1/3}",
     run=shaft_de_goodman_d,
     parse=lambda q: _pairs(q, r"shaft diameter de-goodman|de-goodman shaft diameter|shaft diameter for fatigue", _SHAFT_NAMES, phys={"Ma", "Mm", "Ta", "Tm", "Se", "Sut"}),
     reference=ref_shaft_de_goodman_d,
     wrong=_safe(lambda n, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut: _out(d_mm=(32 * n / math.pi * (Kf * Ma / Se + Kfs * Tm / Sut)) ** (1 / 3) * 1e3 * 1.05)),
     random_inputs=lambda rng: _rand_shaft(rng, False),
     templates=["Shaft diameter DE-Goodman: n = 1.5, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa"],
     examples=[{"inputs": {"n": 1.5, "Ma": 150.0, "Mm": 0.0, "Ta": 0.0, "Tm": 200.0, "Kf": 1.6, "Kfs": 1.4, "Se": 200e6, "Sut": 700e6},
                "outputs": {"d_mm": (16 * 1.5 / math.pi * (2 * 1.6 * 150 / 200e6 + math.sqrt(3) * 1.4 * 200 / 700e6)) ** (1 / 3) * 1e3}}],
     questions=[{"q": "Shaft diameter DE-Goodman: n = 1.5, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa",
                 "outputs": {"d_mm": (16 * 1.5 / math.pi * (2 * 1.6 * 150 / 200e6 + math.sqrt(3) * 1.4 * 200 / 700e6)) ** (1 / 3) * 1e3}},
                {"q": "Shaft diameter DE-Goodman: n = 0.5, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa", "outcome": "INVALID_INPUT"}])


# ================================================================================================ machine elements
def _wrap_factor(mu, theta, beta=None):
    _pos("mu and the wrap angle", mu, theta)
    if beta is not None and not 0 < beta < math.pi:
        raise ValueError("the groove angle must lie between 0 and 180 degrees")
    mu_e = mu / math.sin(beta / 2) if beta is not None else mu
    if mu_e * theta > 50:
        raise ValueError("mu*theta is unrealistically large (> 50)")
    return mu_e


def capstan(T2, mu, theta, beta=None):
    _pos("the slack-side tension T2", T2)
    mu_e = _wrap_factor(mu, theta, beta)
    r = math.exp(mu_e * theta)
    return _out(T1=T2 * r, ratio=r, friction_force=T2 * (r - 1))


def _belt_ivp(rate, T0, theta, forward=True, Tc=0.0):
    from scipy.integrate import solve_ivp
    span = (0.0, theta) if forward else (theta, 0.0)
    sol = solve_ivp(lambda t, y: [rate * (y[0] - Tc)], span, [T0], method="DOP853", rtol=1e-13, atol=1e-14 * abs(T0))
    return float(sol.y[0, -1])


def ref_capstan(T2, mu, theta, beta=None):
    """The equilibrium of a belt element, dT/dtheta = mu' T (mu' = mu / sin(beta/2) in a V-groove), integrated
    numerically over the wrap by scipy's DOP853."""
    mu_e = mu / math.sin(beta / 2) if beta is not None else mu
    T1 = _belt_ivp(mu_e, T2, theta)
    return {"T1": T1, "ratio": T1 / T2, "friction_force": T1 - T2}


_add("belt_friction_capstan",
     name="Belt or rope friction on a drum (capstan equation)", topic="machine elements / belts and band brakes",
     purpose="The largest tight-side tension a flat belt, rope or V-belt can hold for a given slack-side tension, friction coefficient and wrap angle.",
     inputs=[_phys("T2", "force", "slack-side tension"), _numin("mu", "coefficient of friction"), _phys("theta", "angle", "angle of wrap"),
             _phys("beta", "angle", "included groove angle of a V-belt (optional; flat belt when absent)", required=False)],
     outputs=[_o("T1", "force", "tight-side tension at impending slip"), _o("ratio", "number", "tension ratio T1/T2"), _o("friction_force", "force", "net friction force T1 - T2")],
     primary="T1", conditions=["impending slip, no centrifugal effect", "mu > 0, wrap > 0"],
     method="T1/T2 = exp(mu theta), V-belt exp(mu theta / sin(beta/2))",
     latex=r"\frac{T_1}{T_2} = e^{\mu\theta}\quad\Big(e^{\mu\theta/\sin(\beta/2)}\ \text{for a V-belt}\Big)",
     run=capstan,
     parse=lambda q: _pairs(q, r"capstan(?: equation)?|belt friction", {"T2": "T2", "mu": "mu", "theta": "theta", "beta": "beta"}, phys={"T2", "theta", "beta"}),
     reference=ref_capstan,
     wrong=_safe(lambda T2, mu, theta, beta=None: _out(T1=T2 * (1 + mu * theta), ratio=1 + mu * theta, friction_force=T2 * mu * theta)),
     random_inputs=lambda rng: {"T2": _lu(rng, 10, 1e4), "mu": rng.uniform(0.1, 0.5), "theta": _deg(rng.uniform(90, 540)), **({"beta": _deg(rng.uniform(30, 40))} if rng.random() < 0.4 else {})},
     templates=["Capstan equation: T2 = 200 N, mu = 0.3, theta = 180 deg", "Belt friction: T2 = 200 N, mu = 0.3, theta = 180 deg, beta = 36 deg"],
     examples=[{"inputs": {"T2": 200.0, "mu": 0.3, "theta": math.pi}, "outputs": {"T1": 200 * math.exp(0.3 * math.pi)}},
               {"inputs": {"T2": 200.0, "mu": 0.3, "theta": math.pi, "beta": _deg(36)}, "outputs": {"ratio": math.exp(0.3 * math.pi / math.sin(_deg(18)))}}],
     questions=[{"q": "Capstan equation: T2 = 200 N, mu = 0.3, theta = 180 deg", "outputs": {"T1": 200 * math.exp(0.3 * math.pi)}},
                {"q": "Capstan equation: T2 = 200 N, mu = 0, theta = 180 deg", "outcome": "INVALID_INPUT"}])


def belt_power(T1, m, v, mu, theta, beta=None):
    _pos("T1, m and v", T1, m, v)
    mu_e = _wrap_factor(mu, theta, beta)
    Tc = m * v * v
    if Tc >= T1:
        raise ValueError("the centrifugal tension m v^2 reaches the allowable tension: the belt transmits nothing")
    T2 = Tc + (T1 - Tc) / math.exp(mu_e * theta)
    P = (T1 - T2) * v
    return _out(Tc=Tc, T2=T2, P=P, P_kW=P / 1e3)


def ref_belt_power(T1, m, v, mu, theta, beta=None):
    """The belt element with centrifugal tension, dT/dtheta = mu'(T - m v^2), integrated by scipy DOP853 from the
    tight side back over the wrap; power = (T1 - T2) v."""
    mu_e = mu / math.sin(beta / 2) if beta is not None else mu
    Tc = m * v * v
    T2 = _belt_ivp(mu_e, T1, theta, forward=False, Tc=Tc)
    return {"Tc": Tc, "T2": T2, "P": (T1 - T2) * v, "P_kW": (T1 - T2) * v / 1e3}


_add("belt_drive_power",
     name="Power of a belt drive with centrifugal tension", topic="machine elements / belts and band brakes",
     purpose="The centrifugal tension, slack-side tension and transmitted power of a flat or V-belt running at the allowable tight-side tension and impending slip.",
     inputs=[_phys("T1", "force", "allowable tight-side tension"), _phys("m", "linear_mass_density", "belt mass per unit length"), _phys("v", "speed", "belt speed"),
             _numin("mu", "coefficient of friction"), _phys("theta", "angle", "wrap angle on the smaller pulley"),
             _phys("beta", "angle", "included groove angle for a V-belt (optional)", required=False)],
     outputs=[_o("Tc", "force", "centrifugal tension m v^2"), _o("T2", "force", "slack-side tension"), _o("P", "power", "transmitted power"), _o("P_kW", "number", "transmitted power in kW")],
     primary="P", conditions=["impending slip on the smaller pulley", "m v^2 < T1"],
     method="(T1 - Tc)/(T2 - Tc) = exp(mu theta) (mu/sin(beta/2) for a V-belt), Tc = m v^2, P = (T1 - T2) v",
     latex=r"\frac{T_1 - mv^2}{T_2 - mv^2} = e^{\mu\theta},\quad P = (T_1-T_2)v",
     run=belt_power,
     parse=lambda q: _pairs(q, r"belt drive power|power of a belt drive", {"T1": "T1", "m": "m", "v": "v", "mu": "mu", "theta": "theta", "beta": "beta"}, phys={"T1", "m", "v", "theta", "beta"}),
     reference=ref_belt_power,
     wrong=_safe(lambda T1, m, v, mu, theta, beta=None: (lambda T2: _out(Tc=m * v * v, T2=T2, P=(T1 - T2) * v, P_kW=(T1 - T2) * v / 1e3))(T1 / math.exp(mu * theta))),
     random_inputs=lambda rng: (lambda m, v: {"T1": m * v * v * rng.uniform(1.5, 20), "m": m, "v": v, "mu": rng.uniform(0.15, 0.5), "theta": _deg(rng.uniform(120, 200)),
                                              **({"beta": _deg(rng.uniform(32, 40))} if rng.random() < 0.4 else {})})(rng.uniform(0.05, 1.0), rng.uniform(5, 40)),
     templates=["Belt drive power: T1 = 1.2 kN, m = 0.4 kg/m, v = 20 m/s, mu = 0.3, theta = 160 deg"],
     examples=[{"inputs": {"T1": 1200.0, "m": 0.4, "v": 20.0, "mu": 0.3, "theta": _deg(160)},
                "outputs": {"Tc": 160.0, "T2": 160 + 1040 / math.exp(0.3 * _deg(160)), "P": (1040 - 1040 / math.exp(0.3 * _deg(160))) * 20}}],
     questions=[{"q": "Belt drive power: T1 = 1.2 kN, m = 0.4 kg/m, v = 20 m/s, mu = 0.3, theta = 160 deg", "outputs": {"Tc": 160.0}},
                {"q": "Belt drive power: T1 = 100 N, m = 0.4 kg/m, v = 20 m/s, mu = 0.3, theta = 160 deg", "outcome": "INVALID_INPUT"}])


def power_screw(F, dm, l, f, alpha=0.0, fc=0.0, dc=None):
    _pos("F, dm, l and f", F, dm, l, f)
    _nonneg("alpha and fc", alpha, fc)
    if alpha >= math.pi / 4:
        raise ValueError("the thread half-angle must be below 45 degrees")
    if fc > 0 and dc is None:
        raise ValueError("a collar friction coefficient needs the collar mean diameter dc")
    sec = 1 / math.cos(alpha)
    collar = F * fc * (dc or 0.0) / 2
    TR = F * dm / 2 * (l + math.pi * f * dm * sec) / (math.pi * dm - f * l * sec) + collar
    if math.pi * dm - f * l * sec <= 0:
        raise ValueError("the lead is too steep for this friction: the screw cannot raise the load")
    TL_thread = F * dm / 2 * (math.pi * f * dm * sec - l) / (math.pi * dm + f * l * sec)
    return _out(T_raise=TR, T_lower=TL_thread + collar, efficiency=F * l / (2 * math.pi * TR), self_locking="yes" if TL_thread > 0 else "no")


def ref_power_screw(F, dm, l, f, alpha=0.0, fc=0.0, dc=None):
    """The thread unrolled into an inclined plane at the lead angle: the equilibrium of the nut as a 2 x 2 linear
    system (tangential force P and normal force N) solved by numpy for raising and lowering; thread friction
    f/cos(alpha) (the usual thread-angle convention), collar torque added."""
    import numpy as np
    lam = math.atan2(l, math.pi * dm)
    fe = f / math.cos(alpha)
    s, c = math.sin(lam), math.cos(lam)
    up = np.linalg.solve(np.array([[1.0, -(s + fe * c)], [0.0, c - fe * s]]), np.array([0.0, F]))
    dn = np.linalg.solve(np.array([[1.0, s - fe * c], [0.0, c + fe * s]]), np.array([0.0, F]))
    collar = F * fc * (dc or 0.0) / 2
    TR = float(up[0]) * dm / 2 + collar
    TLt = float(dn[0]) * dm / 2
    return {"T_raise": TR, "T_lower": TLt + collar, "efficiency": F * l / (2 * math.pi * TR), "self_locking": "yes" if TLt > 0 else "no"}


_add("power_screw_torque",
     name="Power screw: raising and lowering torque, efficiency, self-locking", topic="machine elements / power screws",
     purpose="The torque to raise and to lower a load with a square or Acme power screw (thread and collar friction), the raising efficiency and whether the screw is self-locking.",
     inputs=[_phys("F", "force", "axial load"), _phys("dm", "length", "mean thread diameter"), _phys("l", "length", "lead"), _numin("f", "thread friction coefficient"),
             _phys("alpha", "angle", "thread half-angle (0 for square, 14.5 deg for Acme; default 0)", required=False),
             _numin("fc", "collar friction coefficient (default 0)", required=False), _phys("dc", "length", "collar mean diameter (needed with fc)", required=False)],
     outputs=[_o("T_raise", "torque", "torque to raise the load"), _o("T_lower", "torque", "torque to lower the load (negative: the load lowers itself)"),
              _o("efficiency", "number", "efficiency when raising"), _o("self_locking", "text", "yes if the thread alone holds the load")],
     primary="T_raise", conditions=["alpha < 45 degrees", "thread friction taken as f/cos(alpha)", "single thrust collar"],
     method="T_R = F dm/2 (l + pi f dm sec a)/(pi dm - f l sec a) + F fc dc/2; T_L = F dm/2 (pi f dm sec a - l)/(pi dm + f l sec a) + F fc dc/2; e = F l/(2 pi T_R)",
     latex=r"T_R = \frac{Fd_m}{2}\,\frac{l + \pi fd_m\sec\alpha}{\pi d_m - fl\sec\alpha} + \frac{Ff_cd_c}{2}",
     run=power_screw,
     parse=lambda q: _pairs(q, r"power screw(?: torque)?", {"F": "F", "dm": "dm", "l": "l", "f": "f", "alpha": "alpha", "fc": "fc", "dc": "dc"}, phys={"F", "dm", "l", "alpha", "dc"}),
     reference=ref_power_screw,
     wrong=_safe(lambda F, dm, l, f, alpha=0.0, fc=0.0, dc=None: (lambda TR: _out(T_raise=TR, T_lower=F * dm / 2 * (math.pi * f * dm - l) / (math.pi * dm), efficiency=F * l / (2 * math.pi * TR),
                                                                                self_locking="no"))(F * dm / 2 * (l + math.pi * f * dm) / (math.pi * dm) + F * fc * (dc or 0) / 2)),
     random_inputs=lambda rng: (lambda dm, fc: {"F": _lu(rng, 500, 1e5), "dm": dm, "l": dm * rng.uniform(0.05, 0.5), "f": rng.uniform(0.08, 0.2), "alpha": rng.choice([0.0, _deg(14.5), _deg(15)]),
                                                **({"fc": fc, "dc": dm * rng.uniform(1.2, 2.0)} if fc else {})})(_lu(rng, 0.01, 0.08), rng.choice([0.0, rng.uniform(0.08, 0.2)])),
     templates=["Power screw: F = 6.4 kN, dm = 30 mm, l = 12 mm, f = 0.08, fc = 0.08, dc = 40 mm"],
     examples=[{"inputs": {"F": 6400.0, "dm": 0.03, "l": 0.012, "f": 0.08, "fc": 0.08, "dc": 0.04},
                "outputs": {"T_raise": 6400 * 0.03 / 2 * (0.012 + math.pi * 0.08 * 0.03) / (math.pi * 0.03 - 0.08 * 0.012) + 6400 * 0.08 * 0.04 / 2, "self_locking": "no"}}],
     questions=[{"q": "Power screw: F = 6.4 kN, dm = 30 mm, l = 12 mm, f = 0.08, fc = 0.08, dc = 40 mm",
                 "outputs": {"T_raise": 6400 * 0.015 * (0.012 + math.pi * 0.0024) / (math.pi * 0.03 - 0.00096) + 10.24}},
                {"q": "Power screw: F = 6.4 kN, dm = 30 mm, l = 12 mm, f = 0.08, fc = 0.08", "outcome": "INVALID_INPUT"}])


_CLUTCH = ["uniform wear", "uniform pressure"]


def disc_clutch(F, f, D, d, model, pairs=1):
    _pos("F, f, D and d", F, f, D, d)
    if d >= D:
        raise ValueError("the inner diameter must be smaller than the outer diameter")
    if not (isinstance(pairs, int) and pairs >= 1):
        raise ValueError("the number of friction pairs must be a whole number >= 1")
    if model == "uniform wear":
        T = F * f * (D + d) / 4
        p = 2 * F / (math.pi * d * (D - d))
    else:
        T = F * f * (D ** 3 - d ** 3) / (3 * (D * D - d * d))
        p = 4 * F / (math.pi * (D * D - d * d))
    return _out(T=T * pairs, p_max_MPa=p / 1e6)


def ref_disc_clutch(F, f, D, d, model, pairs=1):
    """The friction surface integrated numerically (scipy quad): the pressure distribution (p ~ 1/r for uniform
    wear, constant for uniform pressure) scaled so that its integral over the annulus is F, the torque as the
    integral of f p r dA."""
    from scipy.integrate import quad
    shape = (lambda r: 1.0 / r) if model == "uniform wear" else (lambda r: 1.0)
    force_per = quad(lambda r: shape(r) * 2 * math.pi * r, d / 2, D / 2, epsabs=0, epsrel=1e-13)[0]
    c = F / force_per
    T = quad(lambda r: f * c * shape(r) * 2 * math.pi * r * r, d / 2, D / 2, epsabs=0, epsrel=1e-13)[0]
    return {"T": T * pairs, "p_max_MPa": c * max(shape(d / 2), shape(D / 2)) / 1e6}


_add("disc_clutch_torque",
     name="Torque capacity of a disc clutch or brake", topic="machine elements / clutches and brakes",
     purpose="The friction torque and the largest contact pressure of a plate (disc) clutch or brake with an annular friction face, by the uniform-wear or uniform-pressure assumption.",
     inputs=[_phys("F", "force", "axial actuating force"), _numin("f", "coefficient of friction"), _phys("D", "length", "outer diameter of the friction face"),
             _phys("d", "length", "inner diameter of the friction face"), _choice("model", _CLUTCH, "uniform wear (worn-in) or uniform pressure (new)"),
             _intin("pairs", "number of friction-surface pairs (default 1)", lo=1, hi=40, required=False)],
     outputs=[_o("T", "torque", "torque capacity"), _o("p_max_MPa", "number", "largest contact pressure, MPa")],
     primary="T", conditions=["d < D", "the same force on every pair"],
     method="uniform wear: T = F f (D + d)/4, p_max = 2F/(pi d (D - d)); uniform pressure: T = F f (D^3 - d^3)/(3(D^2 - d^2)), p = 4F/(pi (D^2 - d^2)); times the number of pairs",
     latex=r"T_{wear} = \frac{Ff(D+d)}{4},\quad T_{press} = \frac{Ff}{3}\,\frac{D^3-d^3}{D^2-d^2}",
     run=disc_clutch,
     parse=lambda q: _pairs(q, r"disc clutch(?: torque)?|disk clutch(?: torque)?|plate clutch", {"F": "F", "f": "f", "D": "D", "d": "d", "model": "model", "pairs": "pairs", "N": "pairs"},
                            phys={"F", "D", "d"}, words={"model"}),
     reference=ref_disc_clutch,
     wrong=_safe(lambda F, f, D, d, model, pairs=1: _out(T=F * f * (D + d) / 2 * pairs, p_max_MPa=F / (math.pi * (D * D - d * d)) / 1e6)),
     random_inputs=lambda rng: (lambda D: {"F": _lu(rng, 500, 5e4), "f": rng.uniform(0.1, 0.5), "D": D, "d": D * rng.uniform(0.3, 0.8), "model": rng.choice(_CLUTCH),
                                           "pairs": rng.randint(1, 6)})(_lu(rng, 0.08, 0.5)),
     templates=["Disc clutch: F = 4 kN, f = 0.3, D = 200 mm, d = 100 mm, model = uniform wear"],
     examples=[{"inputs": {"F": 4000.0, "f": 0.3, "D": 0.2, "d": 0.1, "model": "uniform wear"}, "outputs": {"T": 4000 * 0.3 * 0.3 / 4, "p_max_MPa": 8000 / (math.pi * 0.1 * 0.1) / 1e6}},
               {"inputs": {"F": 4000.0, "f": 0.3, "D": 0.2, "d": 0.1, "model": "uniform pressure", "pairs": 2}, "outputs": {"T": 2 * 4000 * 0.3 * (0.008 - 0.001) / (3 * 0.03)}}],
     questions=[{"q": "Disc clutch: F = 4 kN, f = 0.3, D = 200 mm, d = 100 mm, model = uniform wear", "outputs": {"T": 90.0}},
                {"q": "Disc clutch: F = 4 kN, f = 0.3, D = 200 mm, d = 100 mm, model = uniform pressure, pairs = 2", "outputs": {"T": 2 * 4000 * 0.3 * 0.007 / 0.09}},
                {"q": "Disc clutch: F = 4 kN, f = 0.3, D = 100 mm, d = 200 mm, model = uniform wear", "outcome": "INVALID_INPUT"}])


_RPM = 60 / (2 * math.pi)


def planetary(Ns, Nr, ws=None, wr=None, wc=None):
    if not (Ns > 0 and Nr > Ns and (Nr - Ns) % 2 == 0):
        raise ValueError("need Nr > Ns > 0 with Nr - Ns even (the planets have (Nr - Ns)/2 teeth)")
    given = [v is not None for v in (ws, wr, wc)]
    if sum(given) != 2:
        raise ValueError("give exactly two of the three speeds ws, wr, wc")
    for v in (ws, wr, wc):
        if v is not None:
            _fin("every speed", v)
    k = Nr / Ns
    if ws is None:
        ws = wc - k * (wr - wc)
    elif wr is None:
        wr = wc - (ws - wc) / k
    else:
        wc = (ws + k * wr) / (1 + k)
    Np = (Nr - Ns) // 2
    wp = wc - (ws - wc) * Ns / Np
    sc = max(abs(ws), abs(wr), abs(wc))
    return _out(ws_rpm=_snap(ws, sc) * _RPM, wr_rpm=_snap(wr, sc) * _RPM, wc_rpm=_snap(wc, sc) * _RPM, wp_rpm=_snap(wp, sc) * _RPM, Np=Np)


def _planetary_wrong(Ns, Nr, ws, wr, wc):
    """Willis' equation with the tooth ratio inverted (a plausible mistake)."""
    k = Ns / Nr
    if ws is None:
        ws = wc - k * (wr - wc)
    elif wr is None:
        wr = wc - (ws - wc) / k
    else:
        wc = (ws + k * wr) / (1 + k)
    Np = (Nr - Ns) // 2
    return _out(ws_rpm=ws * _RPM, wr_rpm=wr * _RPM, wc_rpm=wc * _RPM, wp_rpm=(wc - (ws - wc) * Ns / Np) * _RPM, Np=Np)


def ref_planetary(Ns, Nr, ws=None, wr=None, wc=None):
    """Rolling contact at the two meshes as a linear system (numpy): with module 1 the planet centre moves at
    wc (rs + rp); the sun mesh point at ws rs = wc (rs + rp) - wp rp and the ring mesh point at wr rr = wc (rs + rp)
    + wp rp; solved for the planet's absolute speed and the unknown speed."""
    import numpy as np
    Np = (Nr - Ns) // 2
    rs, rp, rr = Ns / 2, Np / 2, Nr / 2
    rc = rs + rp
    # unknown vector [w_unknown, wp]; equations: wc rc - wp rp - ws rs = 0 ; wc rc + wp rp - wr rr = 0
    if ws is None:
        A, b = np.array([[-rs, -rp], [0.0, rp]]), np.array([-wc * rc, wr * rr - wc * rc])
        x = np.linalg.solve(A, b)
        ws, wp = float(x[0]), float(x[1])
    elif wr is None:
        A, b = np.array([[0.0, -rp], [-rr, rp]]), np.array([ws * rs - wc * rc, -wc * rc])
        x = np.linalg.solve(A, b)
        wr, wp = float(x[0]), float(x[1])
    else:
        A, b = np.array([[rc, -rp], [rc, rp]]), np.array([ws * rs, wr * rr])
        x = np.linalg.solve(A, b)
        wc, wp = float(x[0]), float(x[1])
    sc = max(abs(ws), abs(wr), abs(wc))
    return {"ws_rpm": _snap(ws, sc) * _RPM, "wr_rpm": _snap(wr, sc) * _RPM, "wc_rpm": _snap(wc, sc) * _RPM, "wp_rpm": _snap(wp, sc) * _RPM, "Np": Np}


def _rand_planetary(rng):
    Ns = rng.randint(12, 40)
    Nr = Ns + 2 * rng.randint(10, 40)
    sp = lambda: rng.uniform(-3000, 3000) / _RPM
    which = rng.choice(["ws", "wr", "wc"])
    out = {"Ns": Ns, "Nr": Nr, "ws": sp(), "wr": rng.choice([0.0, sp()]), "wc": sp()}
    del out[which]
    return out


_add("planetary_gear_train",
     name="Speeds of a simple planetary (epicyclic) gear train", topic="machine elements / gear trains",
     purpose="The third speed of a sun-planet-ring-carrier train from the other two (any member may be held: speed 0), with the planets' absolute speed and tooth number.",
     inputs=[_intin("Ns", "teeth on the sun", lo=1), _intin("Nr", "teeth on the ring (internal gear)", lo=1),
             _phys("ws", "angular_frequency", "sun speed (signed; rpm or rad/s)", required=False), _phys("wr", "angular_frequency", "ring speed (signed)", required=False),
             _phys("wc", "angular_frequency", "carrier (arm) speed (signed)", required=False)],
     outputs=[_o("ws_rpm", "number", "sun speed, rpm"), _o("wr_rpm", "number", "ring speed, rpm"), _o("wc_rpm", "number", "carrier speed, rpm"),
              _o("wp_rpm", "number", "absolute speed of each planet, rpm"), _o("Np", "integer", "teeth on each planet")],
     primary="wc_rpm", conditions=["exactly two of the three speeds given", "Nr > Ns, Nr - Ns even", "same sign convention for every speed"],
     method="Willis: (ws - wc)/(wr - wc) = -Nr/Ns; planet: wp = wc - (ws - wc) Ns/Np, Np = (Nr - Ns)/2",
     latex=r"\frac{\omega_s - \omega_c}{\omega_r - \omega_c} = -\frac{N_r}{N_s}",
     run=planetary,
     parse=lambda q: _pairs(q, r"planetary gear(?: train)?|epicyclic gear(?: train)?", {"Ns": "Ns", "Nr": "Nr", "ws": "ws", "wr": "wr", "wc": "wc"}, phys={"ws", "wr", "wc"}),
     reference=ref_planetary,
     wrong=_safe(lambda Ns, Nr, ws=None, wr=None, wc=None: _planetary_wrong(Ns, Nr, ws, wr, wc)),
     random_inputs=_rand_planetary,
     templates=["Planetary gear train: Ns = 20, Nr = 80, ws = 1200 rpm, wr = 0 rpm"],
     examples=[{"inputs": {"Ns": 20, "Nr": 80, "ws": 1200 / _RPM, "wr": 0.0}, "outputs": {"wc_rpm": 240.0, "Np": 30, "wp_rpm": 240 - 960 * 20 / 30}},
               {"inputs": {"Ns": 20, "Nr": 80, "wc": 100 / _RPM, "wr": 0.0}, "outputs": {"ws_rpm": 500.0}}],
     questions=[{"q": "Planetary gear train: Ns = 20, Nr = 80, ws = 1200 rpm, wr = 0 rpm", "outputs": {"wc_rpm": 240.0, "Np": 30}},
                {"q": "Planetary gear train: Ns = 20, Nr = 81, ws = 1200 rpm, wr = 0 rpm", "outcome": "INVALID_INPUT"},
                {"q": "Planetary gear train: Ns = 20, Nr = 80, ws = 1200 rpm, wr = 0 rpm, wc = 10 rpm", "outcome": "INVALID_INPUT"}])


def spur_contact(N1, N2, m, phi):
    if N1 < 1 or N2 < 1:
        raise ValueError("tooth numbers must be positive")
    _pos("the module", m)
    if not _deg(10) <= phi <= _deg(30):
        raise ValueError("the pressure angle must lie between 10 and 30 degrees")
    r1, r2 = N1 * m / 2, N2 * m / 2
    ra1, ra2 = r1 + m, r2 + m
    rb1, rb2 = r1 * math.cos(phi), r2 * math.cos(phi)
    C = r1 + r2
    u1, u2 = math.sqrt(ra1 ** 2 - rb1 ** 2), math.sqrt(ra2 ** 2 - rb2 ** 2)
    if u1 > C * math.sin(phi) or u2 > C * math.sin(phi):
        raise ValueError("involute interference: an addendum passes the other gear's base-circle tangency (too few teeth)")
    Z = u1 + u2 - C * math.sin(phi)
    pb = math.pi * m * math.cos(phi)
    return _out(d1_mm=2 * r1 * 1e3, d2_mm=2 * r2 * 1e3, C_mm=C * 1e3, Z_mm=Z * 1e3, pb_mm=pb * 1e3, contact_ratio=Z / pb)


def ref_spur_contact(N1, N2, m, phi):
    """Geometry of the mesh: the line of action through the pitch point at the pressure angle, its intersections
    with both addendum circles as the roots of |P + t u - O|^2 = R^2 (numpy.roots); the path of contact is the part
    of the line inside both circles. Base pitch as the base circle's circumference over the tooth number."""
    import numpy as np
    r1, r2 = N1 * m / 2, N2 * m / 2
    P = np.array([r1, 0.0])
    u = np.array([math.sin(phi), math.cos(phi)])
    def chord(O, R):
        w = P - np.array(O)
        t = np.sort(np.roots([1.0, 2 * float(w @ u), float(w @ w) - R * R]).real)
        return t
    a1, a2 = chord((0.0, 0.0), r1 + m), chord((r1 + r2, 0.0), r2 + m)
    Z = min(a1[1], a2[1]) - max(a1[0], a2[0])
    pb = 2 * math.pi * r1 * math.cos(phi) / N1
    return {"d1_mm": 2 * r1 * 1e3, "d2_mm": 2 * r2 * 1e3, "C_mm": (r1 + r2) * 1e3, "Z_mm": float(Z) * 1e3, "pb_mm": pb * 1e3, "contact_ratio": float(Z) / pb}


_add("spur_gear_contact_ratio",
     name="Spur gear mesh geometry and contact ratio", topic="machine elements / gears",
     purpose="Pitch diameters, centre distance, length of the path of contact, base pitch and contact ratio of a pair of standard full-depth spur gears.",
     inputs=[_intin("N1", "teeth on the pinion", lo=1), _intin("N2", "teeth on the gear", lo=1), _phys("m", "length", "module (e.g. 4 mm)"), _phys("phi", "angle", "pressure angle")],
     outputs=[_o("d1_mm", "number", "pinion pitch diameter, mm"), _o("d2_mm", "number", "gear pitch diameter, mm"), _o("C_mm", "number", "centre distance, mm"),
              _o("Z_mm", "number", "length of the path of contact, mm"), _o("pb_mm", "number", "base pitch, mm"), _o("contact_ratio", "number", "contact ratio Z / p_b")],
     primary="contact_ratio", conditions=["standard full-depth teeth: addendum = 1 module", "standard centre distance", "no interference (refused otherwise)", "10 <= pressure angle <= 30 degrees"],
     method="Z = sqrt(ra1^2 - rb1^2) + sqrt(ra2^2 - rb2^2) - C sin(phi), rb = r cos(phi), ra = r + m, pb = pi m cos(phi), m_c = Z/pb",
     latex=r"Z = \sqrt{r_{a1}^2-r_{b1}^2} + \sqrt{r_{a2}^2-r_{b2}^2} - C\sin\phi,\quad m_c = \frac{Z}{\pi m\cos\phi}",
     run=spur_contact,
     parse=lambda q: _pairs(q, r"spur gear contact ratio|contact ratio", {"N1": "N1", "N2": "N2", "m": "m", "phi": "phi"}, phys={"m", "phi"}),
     reference=ref_spur_contact,
     wrong=_safe(lambda N1, N2, m, phi: (lambda r: dict(r, contact_ratio=r["Z_mm"] / (math.pi * m * 1e3)))(spur_contact(N1, N2, m, phi))),
     random_inputs=lambda rng: {"N1": rng.randint(18, 40), "N2": rng.randint(18, 120), "m": rng.choice([1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0]) * 1e-3, "phi": _deg(rng.choice([20.0, 22.5, 25.0]))},
     templates=["Spur gear contact ratio: N1 = 20, N2 = 50, m = 4 mm, phi = 20 deg"],
     examples=[{"inputs": {"N1": 20, "N2": 50, "m": 0.004, "phi": _deg(20)},
                "outputs": {"C_mm": 140.0, "contact_ratio": (math.sqrt(44 ** 2 - (40 * math.cos(_deg(20))) ** 2) + math.sqrt(104 ** 2 - (100 * math.cos(_deg(20))) ** 2) - 140 * math.sin(_deg(20))) / (4 * math.pi * math.cos(_deg(20)))}}],
     questions=[{"q": "Spur gear contact ratio: N1 = 20, N2 = 50, m = 4 mm, phi = 20 deg", "outputs": {"C_mm": 140.0}},
                {"q": "Spur gear contact ratio: N1 = 8, N2 = 50, m = 4 mm, phi = 20 deg", "outcome": "INVALID_INPUT"}])


_WEIBULL = {"x0": 0.02, "theta": 4.459, "b": 1.483}


def _weib(x0, theta, b):
    x0 = _WEIBULL["x0"] if x0 is None else x0
    theta = _WEIBULL["theta"] if theta is None else theta
    b = _WEIBULL["b"] if b is None else b
    if not (0 <= x0 < theta and b > 0):
        raise ValueError("Weibull parameters need 0 <= x0 < theta and b > 0")
    return x0, theta, b


def bearing_life(C, F, n, R, kind, LR=1e6, x0=None, theta=None, b=None):
    _pos("C, F and n", C, F, n)
    if not 0 < R < 1:
        raise ValueError("the reliability must lie strictly between 0 and 1")
    x0, theta, b = _weib(x0, theta, b)
    a = 3.0 if kind == "ball" else 10 / 3
    L10 = LR * (C / F) ** a
    x = x0 + (theta - x0) * math.log(1 / R) ** (1 / b)
    rev_per_h = n * 3600 / (2 * math.pi)
    return _out(L10_rev=L10, L10_hours=L10 / rev_per_h, life_multiple=x, L_rev=L10 * x, L_hours=L10 * x / rev_per_h)


def ref_bearing_life(C, F, n, R, kind, LR=1e6, x0=None, theta=None, b=None):
    """scipy.stats.weibull_min (three-parameter: location x0, scale theta - x0, shape b) for the life multiple at the
    given reliability (its inverse survival function); the rating life from the load-life relation."""
    from scipy.stats import weibull_min
    x0, theta, b = _weib(x0, theta, b)
    a = 3.0 if kind == "ball" else 10 / 3
    L10 = LR * math.exp(a * math.log(C / F))
    x = float(weibull_min.isf(R, b, loc=x0, scale=theta - x0))
    rph = n / (2 * math.pi) * 3600
    return {"L10_rev": L10, "L10_hours": L10 / rph, "life_multiple": x, "L_rev": L10 * x, "L_hours": L10 * x / rph}


_BRG_IN = [_choice("kind", ["ball", "roller"], "ball (a = 3) or roller (a = 10/3) bearing"), _numin("LR", "rating life of the catalogue, revolutions (default 1e6)", required=False),
           _numin("x0", "Weibull guaranteed life multiple (default 0.02)", required=False), _numin("theta", "Weibull characteristic life multiple (default 4.459)", required=False),
           _numin("b", "Weibull shape (default 1.483)", required=False)]
_BRG_NAMES = {k: k for k in ("C", "F", "n", "R", "kind", "LR", "x0", "theta", "b", "FD", "LD", "af")}

_add("bearing_life_reliability",
     name="Rolling-bearing life at a given reliability", topic="machine elements / rolling bearings",
     purpose="The rating (L10) life of a ball or roller bearing from its catalogue rating and load, and the life at another reliability from the three-parameter Weibull distribution, in revolutions and hours.",
     inputs=[_phys("C", "force", "catalogue (basic dynamic) load rating"), _phys("F", "force", "equivalent radial load"), _phys("n", "angular_frequency", "speed (rpm)"),
             _numin("R", "required reliability (e.g. 0.95 or 95%)")] + _BRG_IN,
     outputs=[_o("L10_rev", "number", "rating life (90 % reliability), revolutions"), _o("L10_hours", "number", "rating life in hours"),
              _o("life_multiple", "number", "life at R as a multiple of the rating life"), _o("L_rev", "number", "life at reliability R, revolutions"), _o("L_hours", "number", "life at reliability R, hours")],
     primary="L_hours", conditions=["0 < R < 1", "Weibull parameters of the catalogue (defaults: x0 = 0.02, theta = 4.459, b = 1.483)", "constant load and speed"],
     method="L10 = LR (C/F)^a; x = x0 + (theta - x0) (ln 1/R)^(1/b); L = x L10; hours = revolutions / (60 n_rpm)",
     latex=r"L_{10} = L_R\Big(\frac{C}{F}\Big)^a,\quad x = x_0 + (\theta-x_0)\Big(\ln\frac1R\Big)^{1/b}",
     run=bearing_life,
     parse=lambda q: _pairs(q, r"bearing life(?: at reliability)?", _BRG_NAMES, phys={"C", "F", "n"}, words={"kind"}, num_units={"R": ["%", "percent"]}),
     reference=ref_bearing_life,
     wrong=_safe(lambda C, F, n, R, kind, LR=1e6, x0=None, theta=None, b=None: (lambda L10: _out(L10_rev=L10, L10_hours=L10 / (n * 3600), life_multiple=math.log(1 / R) / math.log(1 / 0.9),
                                                                                                L_rev=L10 * math.log(1 / R) / math.log(1 / 0.9), L_hours=L10 / (n * 3600)))(LR * (C / F) ** 3)),
     random_inputs=lambda rng: {"C": _lu(rng, 5e3, 3e5), "F": _lu(rng, 500, 5e4), "n": rng.uniform(100, 5000) / _RPM, "R": rng.uniform(0.8, 0.995), "kind": rng.choice(["ball", "roller"])},
     templates=["Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 0.95, kind = ball"],
     examples=[{"inputs": {"C": 30e3, "F": 4e3, "n": 1500 / _RPM, "R": 0.9, "kind": "ball"}, "outputs": {"L10_rev": 1e6 * 7.5 ** 3, "L10_hours": 1e6 * 7.5 ** 3 / 90000}}],
     questions=[{"q": "Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 0.95, kind = ball", "outputs": {"L10_rev": 1e6 * 7.5 ** 3}},
                {"q": "Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 95%, kind = ball", "outputs": {"L10_rev": 1e6 * 7.5 ** 3}},
                {"q": "Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 1.2, kind = ball", "outcome": "INVALID_INPUT"}])


def bearing_rating(FD, LD, n, R, kind, af=1.0, LR=1e6, x0=None, theta=None, b=None):
    _pos("FD, LD, n and af", FD, LD, n, af)
    if not 0 < R < 1:
        raise ValueError("the reliability must lie strictly between 0 and 1")
    x0, theta, b = _weib(x0, theta, b)
    a = 3.0 if kind == "ball" else 10 / 3
    xD = LD * n / (2 * math.pi) / LR
    C = af * FD * (xD / (x0 + (theta - x0) * math.log(1 / R) ** (1 / b))) ** (1 / a)
    return _out(C10=C, C10_kN=C / 1e3, design_life_multiple=xD)


def ref_bearing_rating(FD, LD, n, R, kind, af=1.0, LR=1e6, x0=None, theta=None, b=None):
    """The rating at which the bearing's reliability at the design life equals R: scipy.stats.weibull_min's survival
    function of the design life in multiples of the rating life, solved for C by scipy brentq."""
    from scipy.optimize import brentq
    from scipy.stats import weibull_min
    x0, theta, b = _weib(x0, theta, b)
    a = 3.0 if kind == "ball" else 10 / 3
    xD = LD * n / (2 * math.pi) / LR
    f = lambda Cr: float(weibull_min.sf(xD / (Cr / (af * FD)) ** a, b, loc=x0, scale=theta - x0)) - R
    lo, hi = af * FD * 1e-3, af * FD * 1e3
    C = brentq(f, lo, hi, xtol=1e-12 * FD, rtol=8.9e-16, maxiter=1000)
    return {"C10": C, "C10_kN": C / 1e3, "design_life_multiple": xD}


_add("bearing_catalog_rating",
     name="Required catalogue rating of a rolling bearing", topic="machine elements / rolling bearings",
     purpose="The basic dynamic load rating C10 a ball or roller bearing needs for a design load, life and speed at a required reliability (three-parameter Weibull, application factor).",
     inputs=[_phys("FD", "force", "design (equivalent radial) load"), _phys("LD", "time", "design life (hours)"), _phys("n", "angular_frequency", "speed (rpm)"),
             _numin("R", "required reliability"), _choice("kind", ["ball", "roller"], "ball (a = 3) or roller (a = 10/3)"),
             _numin("af", "application (load) factor (default 1)", required=False)] + _BRG_IN[1:],
     outputs=[_o("C10", "force", "required catalogue rating"), _o("C10_kN", "number", "required rating in kN"), _o("design_life_multiple", "number", "design life as a multiple of LR")],
     primary="C10", conditions=["0 < R < 1", "Weibull defaults x0 = 0.02, theta = 4.459, b = 1.483 and LR = 1e6 revolutions unless given"],
     method="C10 = af FD [xD/(x0 + (theta - x0)(ln 1/R)^(1/b))]^(1/a), xD = LD n 60/LR",
     latex=r"C_{10} = a_fF_D\Big[\frac{x_D}{x_0 + (\theta-x_0)(\ln 1/R_D)^{1/b}}\Big]^{1/a}",
     run=bearing_rating,
     parse=lambda q: _pairs(q, r"bearing catalog(?:ue)? rating|required bearing rating", _BRG_NAMES, phys={"FD", "LD", "n"}, words={"kind"}, num_units={"R": ["%", "percent"]}),
     reference=ref_bearing_rating,
     wrong=_safe(lambda FD, LD, n, R, kind, af=1.0, LR=1e6, x0=None, theta=None, b=None: (lambda xD: _out(C10=af * FD * xD ** (1 / 3), C10_kN=af * FD * xD ** (1 / 3) / 1e3, design_life_multiple=xD))(LD * n / LR)),
     random_inputs=lambda rng: {"FD": _lu(rng, 500, 3e4), "LD": rng.uniform(2e3, 5e4) * 3600, "n": rng.uniform(100, 5000) / _RPM, "R": rng.uniform(0.85, 0.995),
                                "kind": rng.choice(["ball", "roller"]), "af": rng.choice([1.0, 1.2, 1.5])},
     templates=["Bearing catalogue rating: FD = 1.8 kN, LD = 30000 h, n = 500 rpm, R = 0.99, kind = ball, af = 1.2"],
     examples=[{"inputs": {"FD": 1800.0, "LD": 30000 * 3600.0, "n": 500 / _RPM, "R": 0.99, "kind": "ball", "af": 1.2},
                "outputs": {"design_life_multiple": 900.0, "C10": 1.2 * 1800 * (900 / (0.02 + 4.439 * math.log(1 / 0.99) ** (1 / 1.483))) ** (1 / 3)}}],
     questions=[{"q": "Bearing catalogue rating: FD = 1.8 kN, LD = 30000 h, n = 500 rpm, R = 0.99, kind = ball, af = 1.2",
                 "outputs": {"C10": 1.2 * 1800 * (900 / (0.02 + 4.439 * math.log(1 / 0.99) ** (1 / 1.483))) ** (1 / 3)}}])


def helical_spring(d, D, Na, G, F):
    _pos("d, D, Na, G and F", d, D, Na, G, F)
    C = D / d
    if C <= 2:
        raise ValueError("the spring index D/d must exceed 2")
    k = G * d ** 4 / (8 * D ** 3 * Na)
    tau0 = 8 * F * D / (math.pi * d ** 3)
    return _out(index=C, k=k, deflection_mm=F / k * 1e3, tau_torsion_MPa=tau0 / 1e6, tau_Ks_MPa=tau0 * (1 + 0.5 / C) / 1e6)


def ref_helical_spring(d, D, Na, G, F):
    """Energy method: the wire's torsional strain energy U = T^2 l/(2 G J) along its length l = pi D Na (J by the
    Prandtl stress-function quadrature), deflection 2U/F (Clapeyron); stress as torsion (stress function) plus the
    average direct shear F/A (area by quadrature)."""
    J = _prandtl_disk(d / 2)[0]
    A = _quad_alg(lambda y: 2.0, -d / 2, d / 2, 0.5)
    T, l = F * D / 2, math.pi * D * Na
    U = T * T * l / (2 * G * J)
    delta = 2 * U / F
    tau0 = T * (d / 2) / J
    return {"index": D / d, "k": F / delta, "deflection_mm": delta * 1e3, "tau_torsion_MPa": tau0 / 1e6, "tau_Ks_MPa": (tau0 + F / A) / 1e6}


_add("helical_compression_spring",
     name="Helical compression spring: rate, deflection and shear stress", topic="machine elements / springs",
     purpose="The spring index, rate, deflection under a load and the wire shear stress (torsion alone, and with the direct-shear factor Ks = 1 + 0.5/C) of a round-wire helical spring.",
     inputs=[_phys("d", "length", "wire diameter"), _phys("D", "length", "mean coil diameter"), _numin("Na", "number of active coils"),
             _phys("G", "shear_modulus", "shear modulus of the wire"), _phys("F", "force", "axial load")],
     outputs=[_o("index", "number", "spring index C = D/d"), _o("k", "spring_constant", "spring rate"), _o("deflection_mm", "number", "deflection under F, mm"),
              _o("tau_torsion_MPa", "number", "torsional shear stress 8FD/(pi d^3), MPa"), _o("tau_Ks_MPa", "number", "shear stress with the direct-shear factor Ks, MPa")],
     primary="k", conditions=["C > 2", "close-coiled (small helix angle)", "curvature (Wahl) correction not included"],
     method="k = G d^4/(8 D^3 Na), delta = F/k, tau = Ks 8FD/(pi d^3) with Ks = 1 + 0.5/C",
     latex=r"k = \frac{Gd^4}{8D^3N_a},\quad \tau = K_s\frac{8FD}{\pi d^3},\quad K_s = 1 + \frac{0.5}{C}",
     run=helical_spring,
     parse=lambda q: _pairs(q, r"helical (?:compression )?spring", {"d": "d", "D": "D", "Na": "Na", "N": "Na", "G": "G", "F": "F"}, phys={"d", "D", "G", "F"}),
     reference=ref_helical_spring,
     wrong=_safe(lambda d, D, Na, G, F: (lambda k: _out(index=D / d, k=k, deflection_mm=F / k * 1e3, tau_torsion_MPa=16 * F * D / (math.pi * d ** 3) / 1e6,
                                                     tau_Ks_MPa=16 * F * D / (math.pi * d ** 3) / 1e6))(G * d ** 4 / (64 * D ** 3 * Na))),
     random_inputs=lambda rng: (lambda d: {"d": d, "D": d * rng.uniform(4, 12), "Na": rng.uniform(3, 20), "G": rng.uniform(70e9, 82e9), "F": _lu(rng, 5, 2000)})(_lu(rng, 0.0005, 0.012)),
     templates=["Helical spring: d = 3 mm, D = 24 mm, Na = 10, G = 79 GPa, F = 100 N"],
     examples=[{"inputs": {"d": 0.003, "D": 0.024, "Na": 10.0, "G": 79e9, "F": 100.0},
                "outputs": {"index": 8.0, "k": 79e9 * 0.003 ** 4 / (8 * 0.024 ** 3 * 10), "tau_Ks_MPa": 8 * 100 * 0.024 / (math.pi * 0.003 ** 3) * 1.0625 / 1e6}}],
     questions=[{"q": "Helical spring: d = 3 mm, D = 24 mm, Na = 10, G = 79 GPa, F = 100 N", "outputs": {"index": 8.0}},
                {"q": "Helical spring: d = 3 mm, D = 5 mm, Na = 10, G = 79 GPa, F = 100 N", "outcome": "INVALID_INPUT"}])


# ================================================================================================ vibrations
def _roots_sdof(m, c, k):
    """Characteristic roots of m s^2 + c s + k by numpy (eigenvalues of the companion matrix)."""
    import numpy as np
    return [complex(z) for z in np.roots([m, c, k])]


def sdof_natural(m, k):
    _pos("m and k", m, k)
    wn = math.sqrt(k / m)
    return _out(omega_n=wn, f_n=wn / (2 * math.pi), period=2 * math.pi / wn, static_deflection_mm=m * G0 / k * 1e3)


def ref_sdof_natural(m, k):
    """numpy's roots of the characteristic polynomial m s^2 + k (purely imaginary, +- i omega_n)."""
    s = _roots_sdof(m, 0.0, k)
    wn = max(abs(z.imag) for z in s)
    return {"omega_n": wn, "f_n": wn / (2 * math.pi), "period": 2 * math.pi / wn, "static_deflection_mm": G0 / wn ** 2 * 1e3}


_add("sdof_natural_frequency",
     name="Natural frequency of a spring-mass system", topic="vibrations / free vibration",
     purpose="The natural circular frequency, frequency, period and static deflection (under gravity) of an undamped single-degree-of-freedom spring-mass system.",
     inputs=[_phys("m", "mass", "mass"), _phys("k", "spring_constant", "spring stiffness")],
     outputs=[_o("omega_n", "angular_frequency", "natural circular frequency"), _o("f_n", "frequency", "natural frequency"), _o("period", "time", "period"),
              _o("static_deflection_mm", "number", "static deflection m g/k in mm (g = 9.80665 m/s^2)")],
     primary="omega_n", conditions=["m, k > 0", "linear spring, no damping"],
     method="omega_n = sqrt(k/m), f_n = omega_n/(2 pi), T = 1/f_n, delta_st = m g/k",
     latex=r"\omega_n = \sqrt{\frac{k}{m}},\quad f_n = \frac{1}{2\pi}\sqrt{\frac{k}{m}} = \frac{1}{2\pi}\sqrt{\frac{g}{\delta_{st}}}",
     run=sdof_natural,
     parse=lambda q: _pairs(q, r"natural frequency of a spring[- ]mass(?: system)?|sdof natural frequency|spring[- ]mass natural frequency", {"m": "m", "k": "k"}, phys={"m", "k"}),
     reference=ref_sdof_natural,
     wrong=_safe(lambda m, k: (lambda wn: _out(omega_n=wn, f_n=wn / (2 * math.pi), period=2 * math.pi / wn, static_deflection_mm=m * G0 / k * 1e3))(math.sqrt(k / m) * 2 * math.pi)),
     random_inputs=lambda rng: {"m": _lu(rng, 0.01, 1e4), "k": _lu(rng, 10, 1e8)},
     templates=["SDOF natural frequency: m = 20 kg, k = 8 kN/m"],
     examples=[{"inputs": {"m": 20.0, "k": 8000.0}, "outputs": {"omega_n": 20.0, "f_n": 10 / math.pi, "period": math.pi / 10}}],
     questions=[{"q": "SDOF natural frequency: m = 20 kg, k = 8 kN/m", "outputs": {"omega_n": 20.0}},
                {"q": "Natural frequency of a spring-mass system: m = 20 kg, k = 8000 N/m", "outputs": {"omega_n": 20.0}},
                {"q": "SDOF natural frequency: m = 20 kg", "outcome": "NEEDS_INPUT"}])


def damped_sdof(m, c, k):
    _pos("m and k", m, k)
    _nonneg("c", c)
    wn = math.sqrt(k / m)
    cc = 2 * math.sqrt(k * m)
    z = c / cc
    if z < 1:
        wd = wn * math.sqrt(1 - z * z)
        dec = 2 * math.pi * z / math.sqrt(1 - z * z)
        regime = "underdamped"
    else:
        wd, dec, regime = 0.0, 0.0, "critically damped" if z == 1 else "overdamped"
    return _out(omega_n=wn, zeta=z, omega_d=wd, f_d=wd / (2 * math.pi), c_critical=cc, log_decrement=dec, regime=regime)


def ref_damped_sdof(m, c, k):
    """numpy's roots of m s^2 + c s + k: omega_n = sqrt(s1 s2), zeta = -(s1 + s2)/(2 omega_n) (Vieta), omega_d the
    imaginary part, the logarithmic decrement 2 pi |Re s| / Im s."""
    s1, s2 = _roots_sdof(m, c, k)
    wn = math.sqrt((s1 * s2).real)
    z = -(s1 + s2).real / (2 * wn)
    wd = abs(s1.imag)
    out = {"omega_n": wn, "zeta": z, "c_critical": c / z if z > 0 else 2 * m * wn}
    if z < 1:
        out.update(omega_d=wd, f_d=wd / (2 * math.pi), log_decrement=2 * math.pi * abs(s1.real) / wd, regime="underdamped")
    else:
        out.update(omega_d=0.0, log_decrement=0.0, regime="overdamped")
    return out


_add("damped_sdof_parameters",
     name="Damping ratio, damped frequency and logarithmic decrement", topic="vibrations / damped vibration",
     purpose="The natural frequency, damping ratio, critical damping, damped natural frequency and logarithmic decrement of a viscously damped spring-mass system, and whether it is under- or overdamped.",
     inputs=[_phys("m", "mass", "mass"), _phys("c", "damping", "viscous damping coefficient (N*s/m)"), _phys("k", "spring_constant", "stiffness")],
     outputs=[_o("omega_n", "angular_frequency", "undamped natural frequency"), _o("zeta", "number", "damping ratio c/c_c"), _o("omega_d", "angular_frequency", "damped natural frequency (0 if not underdamped)"),
              _o("f_d", "frequency", "damped natural frequency in Hz"), _o("c_critical", "damping", "critical damping 2 sqrt(k m)"),
              _o("log_decrement", "number", "logarithmic decrement (0 if not underdamped)"), _o("regime", "text", "underdamped, critically damped or overdamped")],
     primary="zeta", conditions=["m, k > 0, c >= 0", "linear viscous damping"],
     method="zeta = c/(2 sqrt(k m)), omega_d = omega_n sqrt(1 - zeta^2), delta = 2 pi zeta/sqrt(1 - zeta^2)",
     latex=r"\zeta = \frac{c}{2\sqrt{km}},\quad \omega_d = \omega_n\sqrt{1-\zeta^2},\quad \delta = \frac{2\pi\zeta}{\sqrt{1-\zeta^2}}",
     run=damped_sdof,
     parse=lambda q: _pairs(q, r"damped sdof(?: parameters)?|damping ratio", {"m": "m", "c": "c", "k": "k"}, phys={"m", "c", "k"}),
     reference=ref_damped_sdof,
     wrong=_safe(lambda m, c, k: (lambda z, wn: _out(omega_n=wn, zeta=z, omega_d=wn * (1 - z * z), f_d=wn * (1 - z * z) / (2 * math.pi), c_critical=math.sqrt(k * m),
                                                  log_decrement=2 * math.pi * z, regime="underdamped"))(c / math.sqrt(k * m), math.sqrt(k / m))),
     random_inputs=lambda rng: (lambda m, k: {"m": m, "k": k, "c": 2 * math.sqrt(k * m) * rng.choice([rng.uniform(0.01, 0.9), rng.uniform(1.1, 3)])})(_lu(rng, 0.1, 1e3), _lu(rng, 100, 1e7)),
     templates=["Damped SDOF: m = 10 kg, c = 40 N*s/m, k = 4000 N/m"],
     examples=[{"inputs": {"m": 10.0, "c": 40.0, "k": 4000.0}, "outputs": {"omega_n": 20.0, "zeta": 0.1, "omega_d": 20 * math.sqrt(0.99), "log_decrement": 2 * math.pi * 0.1 / math.sqrt(0.99)}}],
     questions=[{"q": "Damped SDOF: m = 10 kg, c = 40 N*s/m, k = 4000 N/m", "outputs": {"zeta": 0.1}},
                {"q": "Damping ratio: m = 10 kg, c = 800 N*s/m, k = 4000 N/m", "outputs": {"zeta": 2.0, "regime": "overdamped"}}])


def sdof_free(m, c, k, x0, v0, t):
    _pos("m and k", m, k)
    _nonneg("c and t", c, t)
    _fin("x0 and v0", x0, v0)
    wn = math.sqrt(k / m)
    z = c / (2 * math.sqrt(k * m))
    if abs(z - 1) < 1e-9:
        e = math.exp(-wn * t)
        x = (x0 + (v0 + wn * x0) * t) * e
        v = (v0 - wn * (v0 + wn * x0) * t) * e
    elif z < 1:
        wd = wn * math.sqrt(1 - z * z)
        B = (v0 + z * wn * x0) / wd
        e = math.exp(-z * wn * t)
        cs, sn = math.cos(wd * t), math.sin(wd * t)
        x = e * (x0 * cs + B * sn)
        v = e * (-z * wn * (x0 * cs + B * sn) + wd * (-x0 * sn + B * cs))
    else:
        q = wn * math.sqrt(z * z - 1)
        s1, s2 = -z * wn + q, -z * wn - q
        A1, A2 = (v0 - s2 * x0) / (s1 - s2), (s1 * x0 - v0) / (s1 - s2)
        x = A1 * math.exp(s1 * t) + A2 * math.exp(s2 * t)
        v = A1 * s1 * math.exp(s1 * t) + A2 * s2 * math.exp(s2 * t)
    return _out(x=x, x_mm=x * 1e3, v=v)


def ref_sdof_free(m, c, k, x0, v0, t):
    """The state-space solution x(t) = expm(A t) x(0) with scipy.linalg.expm (no closed form)."""
    import numpy as np
    from scipy.linalg import expm
    A = np.array([[0.0, 1.0], [-k / m, -c / m]])
    y = expm(A * t) @ np.array([x0, v0])
    return {"x": float(y[0]), "x_mm": float(y[0]) * 1e3, "v": float(y[1])}


_add("sdof_free_response",
     name="Free vibration response of a damped spring-mass system", topic="vibrations / damped vibration",
     purpose="The displacement and velocity at time t of a damped single-degree-of-freedom system released from an initial displacement and velocity (under-, critically or overdamped).",
     inputs=[_phys("m", "mass", "mass"), _phys("c", "damping", "damping coefficient"), _phys("k", "spring_constant", "stiffness"),
             _phys("x0", "length", "initial displacement"), _phys("v0", "speed", "initial velocity"), _phys("t", "time", "time")],
     outputs=[_o("x", "length", "displacement at t"), _o("x_mm", "number", "displacement at t in mm"), _o("v", "speed", "velocity at t")],
     primary="x", conditions=["linear viscous damping", "no external force", "zeta within 1e-9 of 1 is treated as critical"],
     method="underdamped x = e^(-zeta wn t)(x0 cos wd t + (v0 + zeta wn x0)/wd sin wd t); critical x = (x0 + (v0 + wn x0) t) e^(-wn t); overdamped sum of two exponentials",
     latex=r"x(t) = e^{-\zeta\omega_nt}\Big(x_0\cos\omega_dt + \frac{v_0+\zeta\omega_nx_0}{\omega_d}\sin\omega_dt\Big)",
     run=sdof_free,
     parse=lambda q: _pairs(q, r"sdof free response|free vibration response", {"m": "m", "c": "c", "k": "k", "x0": "x0", "v0": "v0", "t": "t"}, phys={"m", "c", "k", "x0", "v0", "t"}),
     reference=ref_sdof_free,
     wrong=_safe(lambda m, c, k, x0, v0, t: (lambda wn: _out(x=x0 * math.cos(wn * t) + v0 / wn * math.sin(wn * t), x_mm=(x0 * math.cos(wn * t) + v0 / wn * math.sin(wn * t)) * 1e3,
                                                         v=-x0 * wn * math.sin(wn * t) + v0 * math.cos(wn * t)))(math.sqrt(k / m))),
     random_inputs=lambda rng: (lambda m, k: {"m": m, "k": k, "c": 2 * math.sqrt(k * m) * rng.choice([rng.uniform(0.02, 0.9), rng.uniform(1.1, 2.5)]), "x0": rng.uniform(-0.05, 0.05),
                                              "v0": rng.uniform(-1, 1), "t": rng.uniform(0, 3) / math.sqrt(k / m)})(_lu(rng, 0.1, 100), _lu(rng, 100, 1e5)),
     templates=["SDOF free response: m = 1 kg, c = 2 N*s/m, k = 100 N/m, x0 = 10 mm, v0 = 0 m/s, t = 0.5 s"],
     examples=[{"inputs": {"m": 1.0, "c": 2.0, "k": 100.0, "x0": 0.01, "v0": 0.0, "t": 0.5},
                "outputs": {"x": math.exp(-0.5) * (0.01 * math.cos(math.sqrt(99) * 0.5) + 0.01 / math.sqrt(99) * math.sin(math.sqrt(99) * 0.5))}},
               {"inputs": {"m": 1.0, "c": 20.0, "k": 100.0, "x0": 0.01, "v0": 0.0, "t": 0.1}, "outputs": {"x": (0.01 + 0.1 * 0.1) * math.exp(-1.0)}}],
     questions=[{"q": "SDOF free response: m = 1 kg, c = 20 N*s/m, k = 100 N/m, x0 = 10 mm, v0 = 0 m/s, t = 0.1 s", "outputs": {"x": 0.02 * math.exp(-1.0)}},
                {"q": "SDOF free response: m = 1 kg, c = 20 N*s/m, k = 100 N/m, x0 = 10 mm, v0 = 0 m/s", "outcome": "NEEDS_INPUT"}])


def forced_response(m, c, k, F0, omega):
    _pos("m, k, F0 and omega", m, k, F0, omega)
    _nonneg("c", c)
    re_, im = k - m * omega ** 2, c * omega
    if re_ == 0 and im == 0:
        raise ValueError("undamped resonance: the amplitude grows without bound")
    X = F0 / math.hypot(re_, im)
    wn = math.sqrt(k / m)
    return _out(X=X, X_mm=X * 1e3, phase_deg=math.degrees(math.atan2(im, re_)), magnification=k * X / F0, r=omega / wn)


def ref_forced_response(m, c, k, F0, omega):
    """scipy.signal: the frequency response of the transfer function 1/(m s^2 + c s + k) at omega (freqresp)."""
    from scipy import signal
    w, H = signal.freqresp(signal.lti([1.0], [m, c, k]), w=[omega])
    h = complex(H[0])
    return {"X": F0 * abs(h), "X_mm": F0 * abs(h) * 1e3, "phase_deg": -math.degrees(math.atan2(h.imag, h.real)), "magnification": k * abs(h), "r": omega / math.sqrt(k / m)}


_add("forced_vibration_amplitude",
     name="Steady-state response to a harmonic force", topic="vibrations / forced vibration",
     purpose="The steady-state amplitude, phase lag and dynamic magnification factor of a damped spring-mass system driven by a harmonic force F0 cos(omega t).",
     inputs=[_phys("m", "mass", "mass"), _phys("c", "damping", "damping coefficient"), _phys("k", "spring_constant", "stiffness"),
             _phys("F0", "force", "force amplitude"), _phys("omega", "angular_frequency", "forcing frequency (rad/s or rpm)")],
     outputs=[_o("X", "length", "steady-state amplitude"), _o("X_mm", "number", "amplitude in mm"), _o("phase_deg", "number", "phase lag of the displacement behind the force, degrees (0 to 180)"),
              _o("magnification", "number", "dynamic magnification X k/F0"), _o("r", "number", "frequency ratio omega/omega_n")],
     primary="X", conditions=["steady state (transient decayed)", "not undamped resonance"],
     method="X = F0/sqrt((k - m w^2)^2 + (c w)^2), phi = atan2(c w, k - m w^2)",
     latex=r"X = \frac{F_0/k}{\sqrt{(1-r^2)^2 + (2\zeta r)^2}},\quad \tan\phi = \frac{2\zeta r}{1-r^2}",
     run=forced_response,
     parse=lambda q: _pairs(q, r"forced vibration(?: amplitude)?|forced response", {"m": "m", "c": "c", "k": "k", "F0": "F0", "omega": "omega", "w": "omega"}, phys={"m", "c", "k", "F0", "omega"}),
     reference=ref_forced_response,
     wrong=_safe(lambda m, c, k, F0, omega: (lambda X: _out(X=X, X_mm=X * 1e3, phase_deg=math.degrees(math.atan(c * omega / (k - m * omega ** 2))), magnification=k * X / F0, r=omega / math.sqrt(k / m)))(
         F0 / (abs(k - m * omega ** 2) + c * omega))),
     random_inputs=lambda rng: (lambda m, k: {"m": m, "k": k, "c": 2 * math.sqrt(k * m) * rng.uniform(0.02, 0.6), "F0": _lu(rng, 1, 1e4), "omega": math.sqrt(k / m) * rng.uniform(0.2, 3)})(_lu(rng, 0.1, 1e3), _lu(rng, 100, 1e7)),
     templates=["Forced vibration: m = 10 kg, c = 40 N*s/m, k = 4000 N/m, F0 = 100 N, omega = 15 rad/s"],
     examples=[{"inputs": {"m": 10.0, "c": 40.0, "k": 4000.0, "F0": 100.0, "omega": 20.0}, "outputs": {"X": 100 / 800, "phase_deg": 90.0, "magnification": 5.0, "r": 1.0}}],
     questions=[{"q": "Forced vibration: m = 10 kg, c = 40 N*s/m, k = 4000 N/m, F0 = 100 N, omega = 20 rad/s", "outputs": {"X": 0.125, "phase_deg": 90.0}},
                {"q": "Forced vibration: m = 10 kg, c = 0 N*s/m, k = 4000 N/m, F0 = 100 N, omega = 20 rad/s", "outcome": "INVALID_INPUT"}])


def transmissibility(zeta, r):
    _nonneg("zeta", zeta)
    _pos("the frequency ratio r", r)
    num = 1 + (2 * zeta * r) ** 2
    den = (1 - r * r) ** 2 + (2 * zeta * r) ** 2
    if den == 0:
        raise ValueError("undamped resonance: the transmissibility is unbounded")
    TR = math.sqrt(num / den)
    return _out(TR=TR, isolation_percent=(1 - TR) * 100 if TR < 1 else 0.0)


def ref_transmissibility(zeta, r):
    """scipy.signal.freqresp of the transmitted-force transfer function (2 zeta s + 1)/(s^2 + 2 zeta s + 1) at the
    frequency ratio r."""
    from scipy import signal
    w, H = signal.freqresp(signal.lti([2 * zeta, 1.0], [1.0, 2 * zeta, 1.0]), w=[r])
    TR = abs(complex(H[0]))
    return {"TR": TR, "isolation_percent": (1 - TR) * 100 if TR < 1 else 0.0}


_add("vibration_transmissibility",
     name="Force (or base-motion) transmissibility of a vibration isolator", topic="vibrations / vibration isolation",
     purpose="The ratio of transmitted to applied force amplitude (equally, of mass to base displacement amplitude) for a damped isolator at a frequency ratio, and the isolation achieved.",
     inputs=[_numin("zeta", "damping ratio"), _numin("r", "frequency ratio omega/omega_n")],
     outputs=[_o("TR", "number", "transmissibility"), _o("isolation_percent", "number", "isolation (1 - TR) in percent (0 when TR >= 1)")],
     primary="TR", conditions=["zeta >= 0, r > 0", "not undamped resonance (r = 1 with zeta = 0)"],
     method="TR = sqrt((1 + (2 zeta r)^2)/((1 - r^2)^2 + (2 zeta r)^2))",
     latex=r"TR = \sqrt{\frac{1 + (2\zeta r)^2}{(1-r^2)^2 + (2\zeta r)^2}}",
     run=transmissibility,
     parse=lambda q: _pairs(q, r"(?:vibration )?transmissibility", {"zeta": "zeta", "r": "r"}),
     reference=ref_transmissibility,
     wrong=_safe(lambda zeta, r: (lambda TR: _out(TR=TR, isolation_percent=(1 - TR) * 100 if TR < 1 else 0.0))(1 / math.sqrt((1 - r * r) ** 2 + (2 * zeta * r) ** 2))),
     random_inputs=lambda rng: {"zeta": rng.uniform(0.01, 0.7), "r": rng.uniform(0.1, 5)},
     templates=["Transmissibility: zeta = 0.1, r = 2.5"],
     examples=[{"inputs": {"zeta": 0.0, "r": 2.0}, "outputs": {"TR": 1 / 3, "isolation_percent": 200 / 3}}, {"inputs": {"zeta": 0.1, "r": math.sqrt(2)}, "outputs": {"TR": 1.0}}],
     questions=[{"q": "Transmissibility: zeta = 0, r = 2", "outputs": {"TR": 1 / 3}},
                {"q": "Transmissibility: zeta = 0, r = 1", "outcome": "INVALID_INPUT"}])


def unbalance(M, mu, e, k, c, omega):
    _pos("M, mu, e, k and omega", M, mu, e, k, omega)
    _nonneg("c", c)
    if mu >= M:
        raise ValueError("the unbalanced mass must be part of the total mass M")
    den = math.hypot(k - M * omega ** 2, c * omega)
    if den == 0:
        raise ValueError("undamped resonance")
    X = mu * e * omega ** 2 / den
    FT = X * math.hypot(k, c * omega)
    return _out(X=X, X_mm=X * 1e3, F_transmitted=FT, F_unbalance=mu * e * omega ** 2)


def ref_unbalance(M, mu, e, k, c, omega):
    """scipy.signal.freqresp of 1/(M s^2 + c s + k) (displacement) and (c s + k)/(M s^2 + c s + k) (transmitted force)
    for the rotating force mu e omega^2."""
    from scipy import signal
    F = mu * e * omega ** 2
    _w, H = signal.freqresp(signal.lti([1.0], [M, c, k]), w=[omega])
    _w, T = signal.freqresp(signal.lti([c, k], [M, c, k]), w=[omega])
    X = F * abs(complex(H[0]))
    return {"X": X, "X_mm": X * 1e3, "F_transmitted": F * abs(complex(T[0])), "F_unbalance": F}


_add("rotating_unbalance_response",
     name="Vibration due to rotating unbalance", topic="vibrations / forced vibration",
     purpose="The steady-state amplitude of a machine on springs and dampers excited by a rotating unbalance, the unbalance force and the force transmitted to the foundation.",
     inputs=[_phys("M", "mass", "total mass of the machine"), _phys("mu", "mass", "unbalanced mass"), _phys("e", "length", "eccentricity of the unbalanced mass"),
             _phys("k", "spring_constant", "total stiffness of the mounts"), _phys("c", "damping", "damping coefficient"), _phys("omega", "angular_frequency", "rotational speed")],
     outputs=[_o("X", "length", "amplitude of the machine"), _o("X_mm", "number", "amplitude in mm"), _o("F_transmitted", "force", "force amplitude transmitted to the foundation"),
              _o("F_unbalance", "force", "rotating unbalance force mu e omega^2")],
     primary="X", conditions=["vertical motion only, steady state", "mu < M"],
     method="X = mu e w^2/sqrt((k - M w^2)^2 + (c w)^2), F_T = X sqrt(k^2 + (c w)^2)",
     latex=r"X = \frac{m_ue\omega^2}{\sqrt{(k-M\omega^2)^2 + (c\omega)^2}},\quad F_T = X\sqrt{k^2+(c\omega)^2}",
     run=unbalance,
     parse=lambda q: _pairs(q, r"rotating unbalance(?: response)?", {"M": "M", "mu": "mu", "m": "mu", "e": "e", "k": "k", "c": "c", "omega": "omega", "n": "omega"}, phys={"M", "mu", "e", "k", "c", "omega"}),
     reference=ref_unbalance,
     wrong=_safe(lambda M, mu, e, k, c, omega: (lambda X: _out(X=X, X_mm=X * 1e3, F_transmitted=k * X, F_unbalance=mu * e * omega ** 2))(mu * e * omega ** 2 / math.hypot(k - mu * omega ** 2, c * omega))),
     random_inputs=lambda rng: (lambda M, k: {"M": M, "mu": M * rng.uniform(0.001, 0.05), "e": rng.uniform(0.001, 0.2), "k": k, "c": 2 * math.sqrt(k * M) * rng.uniform(0.02, 0.5),
                                              "omega": math.sqrt(k / M) * rng.uniform(0.3, 4)})(_lu(rng, 10, 5e3), _lu(rng, 1e4, 1e8)),
     templates=["Rotating unbalance: M = 100 kg, mu = 0.5 kg, e = 50 mm, k = 400 kN/m, c = 1 kN*s/m, omega = 1200 rpm"],
     examples=[{"inputs": {"M": 100.0, "mu": 0.5, "e": 0.05, "k": 4e5, "c": 0.0, "omega": 100.0},
                "outputs": {"X": 0.5 * 0.05 * 1e4 / abs(4e5 - 1e6), "F_transmitted": 0.5 * 0.05 * 1e4 / 6e5 * 4e5}}],
     questions=[{"q": "Rotating unbalance: M = 100 kg, mu = 0.5 kg, e = 50 mm, k = 400 kN/m, c = 0 N*s/m, omega = 100 rad/s", "outputs": {"X": 250 / 6e5}},
                {"q": "Rotating unbalance: M = 100 kg, mu = 200 kg, e = 50 mm, k = 400 kN/m, c = 0 N*s/m, omega = 100 rad/s", "outcome": "INVALID_INPUT"}])


def isolator_design(m, omega, TR):
    _pos("m and omega", m, omega)
    if not 0 < TR < 1:
        raise ValueError("the target transmissibility must lie strictly between 0 and 1")
    r = math.sqrt(1 + 1 / TR)
    wn = omega / r
    k = m * wn ** 2
    return _out(k=k, f_n=wn / (2 * math.pi), r=r, static_deflection_mm=m * G0 / k * 1e3)


def ref_isolator_design(m, omega, TR):
    """The frequency ratio at which scipy.signal.freqresp of the undamped isolator 1/(s^2 + 1) has magnitude TR,
    found by scipy brentq above resonance; the stiffness from that natural frequency."""
    from scipy import signal
    from scipy.optimize import brentq
    sys_ = signal.lti([1.0], [1.0, 0.0, 1.0])
    f = lambda r: abs(complex(signal.freqresp(sys_, w=[r])[1][0])) - TR
    r = brentq(f, 1.0 + 1e-9, 1e4, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    wn = omega / r
    return {"k": m * wn * wn, "f_n": wn / (2 * math.pi), "r": r, "static_deflection_mm": G0 / wn ** 2 * 1e3}


_add("vibration_isolator_design",
     name="Stiffness of an undamped isolator for a target transmissibility", topic="vibrations / vibration isolation",
     purpose="The mount stiffness, natural frequency, frequency ratio and static deflection an undamped isolator needs so that a machine running at omega transmits only a fraction TR of its force.",
     inputs=[_phys("m", "mass", "isolated mass"), _phys("omega", "angular_frequency", "forcing frequency (rad/s or rpm)"), _numin("TR", "target transmissibility (0 < TR < 1, e.g. 0.1 or 10%)")],
     outputs=[_o("k", "spring_constant", "required total stiffness"), _o("f_n", "frequency", "natural frequency of the mounted system"), _o("r", "number", "frequency ratio omega/omega_n"),
              _o("static_deflection_mm", "number", "static deflection of the mounts, mm")],
     primary="k", conditions=["undamped isolator", "0 < TR < 1 (isolation region r > sqrt(2))"],
     method="TR = 1/(r^2 - 1) -> r = sqrt(1 + 1/TR), k = m (omega/r)^2",
     latex=r"TR = \frac{1}{r^2-1},\quad k = m\Big(\frac{\omega}{r}\Big)^2",
     run=isolator_design,
     parse=lambda q: _pairs(q, r"vibration isolator design|isolator design", {"m": "m", "omega": "omega", "n": "omega", "TR": "TR"}, phys={"m", "omega"}, num_units={"TR": ["%", "percent"]}),
     reference=ref_isolator_design,
     wrong=_safe(lambda m, omega, TR: (lambda r: _out(k=m * (omega / r) ** 2, f_n=omega / r / (2 * math.pi), r=r, static_deflection_mm=G0 / (omega / r) ** 2 * 1e3))(math.sqrt(1 / TR))),
     random_inputs=lambda rng: {"m": _lu(rng, 1, 5e3), "omega": _lu(rng, 20, 1000), "TR": rng.uniform(0.01, 0.9)},
     templates=["Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 0.1"],
     examples=[{"inputs": {"m": 200.0, "omega": 50 * math.pi, "TR": 0.1}, "outputs": {"r": math.sqrt(11), "k": 200 * (50 * math.pi) ** 2 / 11}}],
     questions=[{"q": "Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 0.1", "outputs": {"r": math.sqrt(11)}},
                {"q": "Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 10%", "outputs": {"r": math.sqrt(11)}},
                {"q": "Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 1.5", "outcome": "INVALID_INPUT"}])


def two_rotor(J1, J2, kt, L=None):
    _pos("J1, J2 and kt", J1, J2, kt)
    w = math.sqrt(kt * (J1 + J2) / (J1 * J2))
    out = {"omega": w, "f": w / (2 * math.pi), "amplitude_ratio": -J1 / J2}
    if L is not None:
        _pos("L", L)
        out["node_from_rotor1"] = L * J2 / (J1 + J2)
    return _out(**out)


def ref_two_rotor(J1, J2, kt, L=None):
    """scipy.linalg.eigh of the generalized problem K phi = w^2 J phi of the free-free two-rotor system (one rigid
    mode, one elastic); the node by linear interpolation of the elastic mode shape along the shaft."""
    import numpy as np
    from scipy.linalg import eigh
    K = kt * np.array([[1.0, -1.0], [-1.0, 1.0]])
    lam, V = eigh(K, np.diag([J1, J2]))
    w = math.sqrt(float(lam[-1]))
    p1, p2 = float(V[0, -1]), float(V[1, -1])
    out = {"omega": w, "f": w / (2 * math.pi), "amplitude_ratio": p2 / p1}
    if L is not None:
        out["node_from_rotor1"] = L * p1 / (p1 - p2)
    return out


_add("two_rotor_torsional_vibration",
     name="Torsional natural frequency of a two-rotor system", topic="vibrations / torsional vibration",
     purpose="The torsional natural frequency of two rotors joined by an elastic shaft, the ratio of their vibration amplitudes and (with the shaft length) where the node lies.",
     inputs=[_phys("J1", "moment_of_inertia", "mass moment of inertia of rotor 1"), _phys("J2", "moment_of_inertia", "mass moment of inertia of rotor 2"),
             _phys("kt", "torsional_stiffness", "torsional stiffness of the shaft (N*m/rad)"), _phys("L", "length", "shaft length (optional: gives the node position)", required=False)],
     outputs=[_o("omega", "angular_frequency", "torsional natural frequency"), _o("f", "frequency", "natural frequency in Hz"),
              _o("amplitude_ratio", "number", "amplitude of rotor 2 / rotor 1 (negative: opposite phase)"), _o("node_from_rotor1", "length", "distance of the node from rotor 1 (with L)")],
     primary="omega", conditions=["shaft inertia neglected", "free-free system (no fixed end)"],
     method="omega^2 = kt (J1 + J2)/(J1 J2); amplitude ratio -J1/J2; node at L J2/(J1 + J2) from rotor 1",
     latex=r"\omega = \sqrt{\frac{k_t(J_1+J_2)}{J_1J_2}},\quad l_1 = \frac{J_2L}{J_1+J_2}",
     run=two_rotor,
     parse=lambda q: _pairs(q, r"two[- ]rotor(?: torsional vibration| system)?", {"J1": "J1", "J2": "J2", "kt": "kt", "L": "L"}, phys={"J1", "J2", "kt", "L"}),
     reference=ref_two_rotor,
     wrong=_safe(lambda J1, J2, kt, L=None: _out(omega=math.sqrt(kt / (J1 + J2)), f=math.sqrt(kt / (J1 + J2)) / (2 * math.pi), amplitude_ratio=-J2 / J1, node_from_rotor1=(L or 1.0) * J1 / (J1 + J2))),
     random_inputs=lambda rng: {"J1": _lu(rng, 0.01, 100), "J2": _lu(rng, 0.01, 100), "kt": _lu(rng, 100, 1e7), **({"L": rng.uniform(0.2, 3)} if rng.random() < 0.6 else {})},
     templates=["Two-rotor torsional vibration: J1 = 2 kg*m^2, J2 = 6 kg*m^2, kt = 30 kN*m/rad, L = 1.2 m"],
     examples=[{"inputs": {"J1": 2.0, "J2": 6.0, "kt": 30e3, "L": 1.2}, "outputs": {"omega": math.sqrt(30e3 * 8 / 12), "amplitude_ratio": -1 / 3, "node_from_rotor1": 0.9}}],
     questions=[{"q": "Two-rotor torsional vibration: J1 = 2 kg*m^2, J2 = 6 kg*m^2, kt = 30 kN*m/rad, L = 1.2 m", "outputs": {"omega": math.sqrt(20e3), "node_from_rotor1": 0.9}},
                {"q": "Two-rotor torsional vibration: J1 = 2 kg*m^2, J2 = 6 kg*m^2", "outcome": "NEEDS_INPUT"}])


def _jacobi_sym(A):
    """Eigenvalues (ascending) and eigenvectors (columns) of a symmetric matrix by cyclic Jacobi rotations."""
    n = len(A)
    a = [row[:] for row in A]
    V = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    for _sweep in range(100):
        off = math.sqrt(sum(a[i][j] ** 2 for i in range(n) for j in range(n) if i != j))
        nrm = math.sqrt(sum(a[i][j] ** 2 for i in range(n) for j in range(n)))
        if off <= 1e-17 * nrm or off == 0:
            break
        for p in range(n - 1):
            for q in range(p + 1, n):
                if a[p][q] == 0:
                    continue
                th = (a[q][q] - a[p][p]) / (2 * a[p][q])
                t = math.copysign(1.0, th) / (abs(th) + math.sqrt(th * th + 1))
                c = 1 / math.sqrt(t * t + 1)
                s = t * c
                for k in range(n):
                    akp, akq = a[k][p], a[k][q]
                    a[k][p], a[k][q] = c * akp - s * akq, s * akp + c * akq
                for k in range(n):
                    apk, aqk = a[p][k], a[q][k]
                    a[p][k], a[q][k] = c * apk - s * aqk, s * apk + c * aqk
                for k in range(n):
                    vkp, vkq = V[k][p], V[k][q]
                    V[k][p], V[k][q] = c * vkp - s * vkq, s * vkp + c * vkq
    order = sorted(range(n), key=lambda i: a[i][i])
    return [a[i][i] for i in order], [[V[r][i] for r in range(n)] for i in order]


def _norm_mode(v):
    """Scale a mode shape so that its largest entry is +1 (the first one within 1e-9 of the largest magnitude)."""
    big = max(abs(x) for x in v)
    piv = next(x for x in v if abs(x) >= big * (1 - 1e-9))
    return [_snap(x / piv, 1.0) for x in v]


def _chain_K(springs, n, ends):
    K = [[0.0] * n for _ in range(n)]
    for i in range(n):
        K[i][i] += springs[i]
        if i > 0:
            K[i - 1][i - 1] += springs[i]
            K[i - 1][i] -= springs[i]
            K[i][i - 1] -= springs[i]
    if ends == "fixed-fixed":
        K[n - 1][n - 1] += springs[n]
    return K


def mdof_chain(masses, springs, ends):
    n = len(masses)
    if not 1 <= n <= 12:
        raise ValueError("give 1 to 12 masses")
    need = n if ends == "fixed-free" else n + 1
    if len(springs) != need:
        raise ValueError(f"a {ends} chain of {n} masses needs {need} springs (wall spring first)")
    for v in list(masses) + list(springs):
        _pos("every mass and spring", v)
    K = _chain_K(list(springs), n, ends)
    h = [1 / math.sqrt(mi) for mi in masses]
    A = [[K[i][j] * h[i] * h[j] for j in range(n)] for i in range(n)]
    lam, vecs = _jacobi_sym(A)
    w = [math.sqrt(max(l_, 0.0)) for l_ in lam]
    mode1 = _norm_mode([vecs[0][i] * h[i] for i in range(n)])
    return _out(omega=w, f_Hz=[x / (2 * math.pi) for x in w], mode1=mode1)


def ref_mdof_chain(masses, springs, ends):
    """scipy.linalg.eigh of the generalized eigenproblem K phi = w^2 M phi (LAPACK)."""
    import numpy as np
    from scipy.linalg import eigh
    n = len(masses)
    K = np.array(_chain_K(list(springs), n, ends))
    lam, V = eigh(K, np.diag(masses))
    w = [math.sqrt(float(x)) for x in lam]
    return {"omega": w, "f_Hz": [x / (2 * math.pi) for x in w], "mode1": _norm_mode([float(x) for x in V[:, 0]])}


_add("mdof_spring_mass_chain",
     name="Natural frequencies and first mode of a spring-mass chain", topic="vibrations / multi-degree-of-freedom systems",
     purpose="All natural frequencies and the first mode shape of a chain of masses joined by springs, fixed to a wall at the left end (and optionally at the right end).",
     inputs=[_vec("masses", "masses in kg, left to right (a list)", lo=1, hi=12), _vec("springs", "spring stiffnesses in N/m, the wall spring first (a list)", lo=1, hi=13),
             _choice("ends", ["fixed-free", "fixed-fixed"], "fixed-free (n springs) or fixed-fixed (n + 1 springs)")],
     outputs=[_o("omega", "number", "natural circular frequencies in rad/s, ascending"), _o("f_Hz", "number", "natural frequencies in Hz"),
              _o("mode1", "number", "first mode shape, largest entry +1")],
     primary="omega", conditions=["1 to 12 masses", "linear springs, no damping", "springs: n (fixed-free) or n + 1 (fixed-fixed)"],
     method="K phi = w^2 M phi with tridiagonal K; solved as the symmetric M^(-1/2) K M^(-1/2) by Jacobi rotations",
     latex=r"(\mathbf K - \omega^2\mathbf M)\boldsymbol\phi = \mathbf 0",
     run=mdof_chain,
     parse=lambda q: _pairs(q, r"spring[- ]mass chain|mdof (?:spring[- ]mass )?chain|multi[- ]degree[- ]of[- ]freedom chain", {"masses": "masses", "m": "masses", "springs": "springs", "k": "springs", "ends": "ends"},
                            vectors={"masses", "springs"}, words={"ends"}),
     reference=ref_mdof_chain,
     wrong=_safe(lambda masses, springs, ends: (lambda r: dict(r, omega=[math.sqrt(springs[i] / masses[i]) for i in range(len(masses))]))(mdof_chain(masses, springs, ends))),
     random_inputs=lambda rng: (lambda n, ends: {"masses": [round(_lu(rng, 0.5, 200), 3) for _ in range(n)], "springs": [round(_lu(rng, 1e3, 1e6), 1) for _ in range(n + (ends == "fixed-fixed"))],
                                                 "ends": ends})(rng.randint(1, 6), rng.choice(["fixed-free", "fixed-fixed"])),
     templates=["Spring-mass chain: masses = [2, 1], springs = [2000, 1000], ends = fixed-free"],
     examples=[{"inputs": {"masses": "2, 1", "springs": "2000, 1000", "ends": "fixed-free"},
                "outputs": {"omega": [math.sqrt(500), math.sqrt(2000)], "mode1": [0.5, 1.0]}},
               {"inputs": {"masses": "1, 1", "springs": "100, 100, 100", "ends": "fixed-fixed"}, "outputs": {"omega": [10.0, math.sqrt(300)], "mode1": [1.0, 1.0]}}],
     questions=[{"q": "Spring-mass chain: masses = [2, 1], springs = [2000, 1000], ends = fixed-free", "outputs": {"omega": [math.sqrt(500), math.sqrt(2000)]}},
                {"q": "Spring-mass chain: masses = [2, 1], springs = [2000], ends = fixed-free", "outcome": "INVALID_INPUT"}])


# ------------------------------------------------------------------- shafts: critical speeds (simply supported, uniform)
def _ss_influence(x, a, L):
    """Deflection at x of a simply supported beam (EI = 1) under a unit load at a."""
    b = L - a
    if x <= a:
        return b * x * (L * L - b * b - x * x) / (6 * L)
    return a * (L - x) * (L * L - a * a - (L - x) ** 2) / (6 * L)


def _shaft_check(masses, positions, L, E, I):
    _pos("L, E and I", L, E, I)
    if len(masses) != len(positions) or not 1 <= len(masses) <= 10:
        raise ValueError("masses and positions must be lists of the same length (1 to 10 entries)")
    for v in masses:
        _pos("every mass", v)
    if any(not 0 < p < L for p in positions):
        raise ValueError("every disk must sit between the bearings (0 < position < L)")
    if len(set(positions)) != len(positions):
        raise ValueError("two disks at one position: combine them")


def single_disk(m, E, I, L, a):
    _pos("m, E, I and L", m, E, I, L)
    if not 0 < a < L:
        raise ValueError("the disk must sit between the bearings (0 < a < L)")
    b = L - a
    k = 3 * E * I * L / (a * a * b * b)
    w = math.sqrt(k / m)
    return _out(k=k, omega_cr=w, N_cr_rpm=w * _RPM, static_deflection_mm=m * G0 / k * 1e3)


def ref_single_disk(m, E, I, L, a):
    """anastruct: the shaft as a frame model on two simple supports with a node at the disk; its stiffness from the
    deflection under a unit load."""
    ss, node, xs = _ana(L, {0.0: "hinged", float(L): "roll"}, [(a, 1.0)], E * I)
    k = 1.0 / (abs(ss.get_node_results_system(node_id=node[float(a)])["uy"]) * 1e-3)
    w = math.sqrt(k / m)
    return {"k": k, "omega_cr": w, "N_cr_rpm": w * _RPM, "static_deflection_mm": m * G0 / k * 1e3}


_add("shaft_critical_speed_single_disk",
     name="Critical speed of a shaft carrying one disk", topic="vibrations / critical speeds of shafts",
     purpose="The lateral stiffness at the disk, the first critical (whirling) speed and the static deflection of a simply supported massless shaft carrying one disk.",
     inputs=[_phys("m", "mass", "disk mass"), _phys("E", "elastic_modulus", "Young's modulus of the shaft"), _phys("I", "second_moment_of_area", "second moment of area of the shaft"),
             _phys("L", "length", "bearing span"), _phys("a", "length", "distance of the disk from the left bearing")],
     outputs=[_o("k", "spring_constant", "lateral stiffness of the shaft at the disk"), _o("omega_cr", "angular_frequency", "critical speed"),
              _o("N_cr_rpm", "number", "critical speed in rpm"), _o("static_deflection_mm", "number", "static deflection under the disk's weight, mm")],
     primary="omega_cr", conditions=["simple (short) bearings, uniform massless shaft", "gyroscopic effects neglected", "0 < a < L"],
     method="k = 3 E I L/(a^2 b^2), omega_cr = sqrt(k/m)",
     latex=r"k = \frac{3EIL}{a^2b^2},\quad \omega_{cr} = \sqrt{\frac{k}{m}}",
     run=single_disk,
     parse=lambda q: _pairs(q, r"critical speed of a shaft with (?:a|one) disk|shaft critical speed", {"m": "m", "E": "E", "I": "I", "L": "L", "a": "a"}, phys={"m", "E", "I", "L", "a"}),
     reference=ref_single_disk,
     wrong=_safe(lambda m, E, I, L, a: (lambda k: _out(k=k, omega_cr=math.sqrt(k / m), N_cr_rpm=math.sqrt(k / m) * _RPM, static_deflection_mm=m * G0 / k * 1e3))(48 * E * I / L ** 3)),
     random_inputs=lambda rng: (lambda L: {"m": _lu(rng, 1, 500), "E": rng.uniform(190e9, 210e9), "I": _lu(rng, 1e-9, 1e-5), "L": L, "a": round(L * rng.uniform(0.1, 0.9), 3)})(round(rng.uniform(0.3, 3), 3)),
     templates=["Shaft critical speed: m = 50 kg, E = 200 GPa, I = 2e-7 m^4, L = 1 m, a = 0.4 m"],
     examples=[{"inputs": {"m": 50.0, "E": 200e9, "I": 2e-7, "L": 1.0, "a": 0.5}, "outputs": {"k": 48 * 200e9 * 2e-7, "omega_cr": math.sqrt(48 * 200e9 * 2e-7 / 50)}}],
     questions=[{"q": "Shaft critical speed: m = 50 kg, E = 200 GPa, I = 2e-7 m^4, L = 1 m, a = 0.5 m", "outputs": {"k": 1.92e6}},
                {"q": "Shaft critical speed: m = 50 kg, E = 200 GPa, I = 2e-7 m^4, L = 1 m, a = 1.5 m", "outcome": "INVALID_INPUT"}])


def _flex(positions, L, EI):
    return [[_ss_influence(xi, aj, L) / EI for aj in positions] for xi in positions]


def rayleigh_shaft(masses, positions, L, E, I):
    _shaft_check(masses, positions, L, E, I)
    A = _flex(positions, L, E * I)
    y = [G0 * math.fsum(A[i][j] * masses[j] for j in range(len(masses))) for i in range(len(masses))]
    w = math.sqrt(G0 * math.fsum(m * yi for m, yi in zip(masses, y)) / math.fsum(m * yi * yi for m, yi in zip(masses, y)))
    return _out(omega_cr=w, N_cr_rpm=w * _RPM, deflections_mm=[v * 1e3 for v in y])


def _ana_shaft_deflections(masses, positions, L, E, I, loads):
    at = {float(p) for p, _v in loads}
    loads = list(loads) + [(p, 0.0) for p in positions if float(p) not in at]     # a node at every disk
    ss, node, xs = _ana(L, {0.0: "hinged", float(L): "roll"}, loads, E * I)
    return [abs(ss.get_node_results_system(node_id=node[float(p)])["uy"]) * 1e-3 for p in positions]


def ref_rayleigh_shaft(masses, positions, L, E, I):
    """anastruct: the static deflections of the shaft under all the disks' weights (one frame model); Rayleigh's
    quotient over them."""
    y = _ana_shaft_deflections(masses, positions, L, E, I, [(p, m * G0) for m, p in zip(masses, positions)])
    w = math.sqrt(G0 * sum(m * yi for m, yi in zip(masses, y)) / sum(m * yi * yi for m, yi in zip(masses, y)))
    return {"omega_cr": w, "N_cr_rpm": w * _RPM, "deflections_mm": [v * 1e3 for v in y]}


def _rand_shaft_disks(rng):
    L = round(rng.uniform(0.6, 3), 2)
    n = rng.randint(1, 4)
    js = rng.sample(range(1, 12), n)
    return {"masses": [round(_lu(rng, 1, 300), 2) for _ in range(n)], "positions": [round(L * j / 12, 3) for j in js], "L": L, "E": rng.uniform(190e9, 210e9), "I": _lu(rng, 1e-8, 1e-5)}


_SHAFT_DISK_IN = [_vec("masses", "disk masses in kg (a list)", hi=10), _vec("positions", "disk positions from the left bearing, in m (a list)", hi=10),
                  _phys("L", "length", "bearing span"), _phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area of the shaft")]
_SHAFT_DISK_NAMES = {"masses": "masses", "m": "masses", "positions": "positions", "x": "positions", "L": "L", "E": "E", "I": "I"}

_add("rayleigh_critical_speed",
     name="First critical speed of a shaft with several disks (Rayleigh)", topic="vibrations / critical speeds of shafts",
     purpose="Rayleigh's estimate of the first critical speed of a simply supported massless shaft carrying several disks, from the static deflections under the disks' weights.",
     inputs=_SHAFT_DISK_IN,
     outputs=[_o("omega_cr", "angular_frequency", "first critical speed (Rayleigh, an upper bound)"), _o("N_cr_rpm", "number", "critical speed in rpm"),
              _o("deflections_mm", "number", "static deflection at each disk, mm")],
     primary="omega_cr", conditions=["simply supported uniform massless shaft", "disks at distinct positions inside the span", "g = 9.80665 m/s^2"],
     method="y_i = sum_j a_ij m_j g (influence coefficients of the simply supported beam); omega^2 = g sum m_i y_i / sum m_i y_i^2",
     latex=r"\omega_{cr}^2 = \frac{g\sum m_iy_i}{\sum m_iy_i^2}",
     run=rayleigh_shaft,
     parse=lambda q: _pairs(q, r"rayleigh critical speed|critical speed by rayleigh", _SHAFT_DISK_NAMES, phys={"L", "E", "I"}, vectors={"masses", "positions"}),
     reference=ref_rayleigh_shaft,
     wrong=_safe(lambda masses, positions, L, E, I: (lambda r: dict(r, omega_cr=r["omega_cr"] * math.sqrt(len(masses) + 1)))(rayleigh_shaft(masses, positions, L, E, I))),
     random_inputs=_rand_shaft_disks,
     templates=["Rayleigh critical speed: masses = [40, 25], positions = [0.4, 0.9], L = 1.2 m, E = 200 GPa, I = 3e-7 m^4"],
     examples=[{"inputs": {"masses": "50", "positions": "0.5", "L": 1.0, "E": 200e9, "I": 2e-7}, "outputs": {"omega_cr": math.sqrt(48 * 200e9 * 2e-7 / 50)}}],
     questions=[{"q": "Rayleigh critical speed: masses = [50], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4", "outputs": {"omega_cr": math.sqrt(48 * 200e9 * 2e-7 / 50)}},
                {"q": "Rayleigh critical speed: masses = [50, 20], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4", "outcome": "INVALID_INPUT"}])


def dunkerley_shaft(masses, positions, L, E, I):
    _shaft_check(masses, positions, L, E, I)
    s = math.fsum(m * _ss_influence(p, p, L) / (E * I) for m, p in zip(masses, positions))
    w = 1 / math.sqrt(s)
    return _out(omega_cr=w, N_cr_rpm=w * _RPM)


def ref_dunkerley_shaft(masses, positions, L, E, I):
    """anastruct: each disk's own influence coefficient (deflection under a unit load at it, one frame model per
    disk); Dunkerley's sum."""
    s = 0.0
    for m, p in zip(masses, positions):
        s += m * _ana_shaft_deflections([m], [p], L, E, I, [(p, 1.0)])[0]
    w = 1 / math.sqrt(s)
    return {"omega_cr": w, "N_cr_rpm": w * _RPM}


_add("dunkerley_critical_speed",
     name="First critical speed of a shaft with several disks (Dunkerley)", topic="vibrations / critical speeds of shafts",
     purpose="Dunkerley's lower-bound estimate of the first critical speed of a simply supported massless shaft carrying several disks.",
     inputs=_SHAFT_DISK_IN,
     outputs=[_o("omega_cr", "angular_frequency", "first critical speed (Dunkerley, a lower bound)"), _o("N_cr_rpm", "number", "critical speed in rpm")],
     primary="omega_cr", conditions=["simply supported uniform massless shaft", "disks at distinct positions inside the span"],
     method="1/omega^2 = sum m_i a_ii with a_ii = b_i^2 a_i^2/(3 E I L)",
     latex=r"\frac{1}{\omega_{cr}^2} = \sum_i m_ia_{ii},\quad a_{ii} = \frac{a_i^2b_i^2}{3EIL}",
     run=dunkerley_shaft,
     parse=lambda q: _pairs(q, r"dunkerley critical speed|critical speed by dunkerley", _SHAFT_DISK_NAMES, phys={"L", "E", "I"}, vectors={"masses", "positions"}),
     reference=ref_dunkerley_shaft,
     wrong=_safe(lambda masses, positions, L, E, I: (lambda s: _out(omega_cr=1 / math.sqrt(s), N_cr_rpm=_RPM / math.sqrt(s)))(sum(masses) * L ** 3 / (48 * E * I))),
     random_inputs=_rand_shaft_disks,
     templates=["Dunkerley critical speed: masses = [40, 25], positions = [0.4, 0.9], L = 1.2 m, E = 200 GPa, I = 3e-7 m^4"],
     examples=[{"inputs": {"masses": "50", "positions": "0.5", "L": 1.0, "E": 200e9, "I": 2e-7}, "outputs": {"omega_cr": math.sqrt(48 * 200e9 * 2e-7 / 50)}}],
     questions=[{"q": "Dunkerley critical speed: masses = [50], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4", "outputs": {"omega_cr": math.sqrt(48 * 200e9 * 2e-7 / 50)}}])


def exact_whirl(masses, positions, L, E, I):
    _shaft_check(masses, positions, L, E, I)
    A = _flex(positions, L, E * I)
    s = [math.sqrt(m) for m in masses]
    n = len(masses)
    B = [[s[i] * A[i][j] * s[j] for j in range(n)] for i in range(n)]
    lam, _v = _jacobi_sym(B)
    w = sorted(1 / math.sqrt(l_) for l_ in lam if l_ > 0)
    return _out(omega=w, N_rpm=[x * _RPM for x in w])


def ref_exact_whirl(masses, positions, L, E, I):
    """anastruct: the flexibility matrix column by column (unit load at each disk); scipy.linalg.eigh of
    M^(1/2) A M^(1/2) for the whirling speeds."""
    import numpy as np
    from scipy.linalg import eigh
    n = len(masses)
    A = np.zeros((n, n))
    for j, p in enumerate(positions):
        A[:, j] = _ana_shaft_deflections(masses, positions, L, E, I, [(p, 1.0)])
    S = np.diag(np.sqrt(masses))
    lam = eigh(S @ ((A + A.T) / 2) @ S, eigvals_only=True)
    w = sorted(1 / math.sqrt(float(x)) for x in lam)
    return {"omega": w, "N_rpm": [x * _RPM for x in w]}


_add("shaft_whirling_speeds",
     name="All critical (whirling) speeds of a shaft with several disks", topic="vibrations / critical speeds of shafts",
     purpose="The exact critical speeds of a simply supported massless shaft carrying point-mass disks, from the eigenvalues of its flexibility matrix.",
     inputs=_SHAFT_DISK_IN,
     outputs=[_o("omega", "number", "critical speeds in rad/s, ascending"), _o("N_rpm", "number", "critical speeds in rpm")],
     primary="omega", conditions=["simply supported uniform massless shaft, point-mass disks (no rotary inertia or gyroscopics)", "distinct positions"],
     method="det(A M - (1/omega^2) I) = 0 with the influence coefficients a_ij; eigenvalues of the symmetric M^(1/2) A M^(1/2) by Jacobi rotations",
     latex=r"\det\Big(\mathbf A\mathbf M - \frac{1}{\omega^2}\mathbf I\Big) = 0",
     run=exact_whirl,
     parse=lambda q: _pairs(q, r"shaft whirling speeds|whirling speeds|critical speeds of a shaft", _SHAFT_DISK_NAMES, phys={"L", "E", "I"}, vectors={"masses", "positions"}),
     reference=ref_exact_whirl,
     wrong=_safe(lambda masses, positions, L, E, I: (lambda r: dict(r, omega=[x * 1.1 for x in r["omega"]]))(exact_whirl(masses, positions, L, E, I))),
     random_inputs=_rand_shaft_disks,
     templates=["Shaft whirling speeds: masses = [40, 25], positions = [0.4, 0.9], L = 1.2 m, E = 200 GPa, I = 3e-7 m^4"],
     examples=[{"inputs": {"masses": "50", "positions": "0.5", "L": 1.0, "E": 200e9, "I": 2e-7}, "outputs": {"omega": [math.sqrt(48 * 200e9 * 2e-7 / 50)]}}],
     questions=[{"q": "Shaft whirling speeds: masses = [50], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4", "outputs": {"omega": [math.sqrt(48 * 200e9 * 2e-7 / 50)]}}])


_BEAM_ENDS = ["simply-supported", "cantilever", "fixed-fixed", "fixed-pinned", "free-free"]


def _beta_L(ends, n):
    if ends == "simply-supported":
        return n * math.pi
    if ends == "cantilever":
        f, c = (lambda x: math.cos(x) + 1 / math.cosh(x)), (2 * n - 1) * math.pi / 2
    elif ends in ("fixed-fixed", "free-free"):
        f, c = (lambda x: math.cos(x) - 1 / math.cosh(x)), (2 * n + 1) * math.pi / 2
    else:
        f, c = (lambda x: math.sin(x) - math.cos(x) * math.tanh(x)), (4 * n + 1) * math.pi / 4
    lo, hi = c - 0.5, c + 0.5
    if f(lo) * f(hi) > 0:
        raise ValueError("no root bracketed")
    for _ in range(200):
        mid = (lo + hi) / 2
        if f(lo) * f(mid) <= 0:
            hi = mid
        else:
            lo = mid
    return (lo + hi) / 2


def beam_frequency(E, I, rho, A, L, ends, n=1):
    _pos("E, I, rho, A and L", E, I, rho, A, L)
    if not (isinstance(n, int) and 1 <= n <= 6):
        raise ValueError("the mode number must be 1 to 6")
    bl = _beta_L(ends, n)
    w = bl * bl * math.sqrt(E * I / (rho * A * L ** 4))
    return _out(beta_L=bl, omega=w, f=w / (2 * math.pi))


def ref_beam_frequency(E, I, rho, A, L, ends, n=1):
    """The boundary conditions of W = A sin(kx) + B cos(kx) + C e^(k(x-1)) + D e^(-kx) (unit length, a bounded basis)
    as a 4 x 4 determinant evaluated by numpy; its n-th positive root above the rigid-body modes by scipy brentq."""
    import numpy as np
    from scipy.optimize import brentq
    conds = {"simply-supported": (("w", "w2"), ("w", "w2")), "cantilever": (("w", "w1"), ("w2", "w3")), "fixed-fixed": (("w", "w1"), ("w", "w1")),
             "fixed-pinned": (("w", "w1"), ("w", "w2")), "free-free": (("w2", "w3"), ("w2", "w3"))}[ends]

    def row(k, x, p):
        return [math.sin(k * x + p * math.pi / 2), math.cos(k * x + p * math.pi / 2), math.exp(k * (x - 1)), (-1) ** p * math.exp(-k * x)]

    def det(k):
        rows = [row(k, 0.0, int(c[1:] or 0)) for c in conds[0]] + [row(k, 1.0, int(c[1:] or 0)) for c in conds[1]]
        return float(np.linalg.det(np.array(rows)))
    ks = [0.5 + 0.01 * i for i in range(int(((n + 2) * math.pi) / 0.01))]
    vals = [det(k) for k in ks]
    roots = []
    for i in range(len(ks) - 1):
        if vals[i] * vals[i + 1] < 0:
            roots.append(brentq(det, ks[i], ks[i + 1], xtol=1e-15, rtol=8.9e-16, maxiter=500))
    bl = roots[n - 1]
    w = bl * bl * math.sqrt(E * I / (rho * A * L ** 4))
    return {"beta_L": bl, "omega": w, "f": w / (2 * math.pi)}


_add("beam_natural_frequency",
     name="Natural frequencies of a uniform beam (Euler-Bernoulli)", topic="vibrations / continuous systems",
     purpose="The n-th bending natural frequency of a uniform Euler-Bernoulli beam with simply supported, cantilever, fixed-fixed, fixed-pinned or free-free ends.",
     inputs=[_phys("E", "elastic_modulus", "Young's modulus"), _phys("I", "second_moment_of_area", "second moment of area"), _phys("rho", "density", "density"),
             _phys("A", "area", "cross-sectional area"), _phys("L", "length", "length"), _choice("ends", _BEAM_ENDS, "end conditions"),
             _intin("n", "mode number (default 1; free-free counts the elastic modes)", lo=1, hi=6, required=False)],
     outputs=[_o("beta_L", "number", "eigenvalue beta L"), _o("omega", "angular_frequency", "natural circular frequency"), _o("f", "frequency", "natural frequency")],
     primary="omega", conditions=["slender uniform beam, no shear deformation or rotary inertia", "mode 1 to 6"],
     method="omega_n = (beta_n L)^2 sqrt(E I/(rho A L^4)); beta L from sin = 0 (SS), cos cosh = -1 (cantilever), cos cosh = 1 (fixed-fixed, free-free), tan = tanh (fixed-pinned)",
     latex=r"\omega_n = (\beta_nL)^2\sqrt{\frac{EI}{\rho AL^4}},\quad \cos\beta L\cosh\beta L = -1\ \text{(cantilever)}",
     run=beam_frequency,
     parse=lambda q: _pairs(q, r"beam natural frequency|natural frequency of a beam", {"E": "E", "I": "I", "rho": "rho", "A": "A", "L": "L", "ends": "ends", "n": "n"},
                            phys={"E", "I", "rho", "A", "L"}, words={"ends"}),
     reference=ref_beam_frequency,
     wrong=_safe(lambda E, I, rho, A, L, ends, n=1: (lambda bl: _out(beta_L=bl, omega=bl * bl * math.sqrt(E * I / (rho * A * L ** 4)), f=bl * bl * math.sqrt(E * I / (rho * A * L ** 4)) / (2 * math.pi)))(
         (2 * n - 1) * math.pi / 2 if ends != "simply-supported" else n * math.pi / 2)),
     random_inputs=lambda rng: {"E": rng.uniform(70e9, 210e9), "I": _lu(rng, 1e-10, 1e-5), "rho": rng.uniform(2700, 7850), "A": _lu(rng, 1e-5, 1e-2), "L": rng.uniform(0.2, 10),
                                "ends": rng.choice(_BEAM_ENDS), "n": rng.randint(1, 5)},
     templates=["Beam natural frequency: E = 200 GPa, I = 8.33e-10 m^4, rho = 7850 kg/m^3, A = 1e-4 m^2, L = 1 m, ends = cantilever"],
     examples=[{"inputs": {"E": 200e9, "I": 1e-8, "rho": 7850.0, "A": 1e-3, "L": 2.0, "ends": "simply-supported"},
                "outputs": {"beta_L": math.pi, "omega": math.pi ** 2 * math.sqrt(200e9 * 1e-8 / (7.85 * 16))}},
               {"inputs": {"E": 200e9, "I": 1e-8, "rho": 7850.0, "A": 1e-3, "L": 2.0, "ends": "cantilever"}, "outputs": {"beta_L": 1.87510406871}}],
     questions=[{"q": "Beam natural frequency: E = 200 GPa, I = 1e-8 m^4, rho = 7850 kg/m^3, A = 1000 mm^2, L = 2 m, ends = simply-supported", "outputs": {"beta_L": math.pi}},
                {"q": "Beam natural frequency: E = 200 GPa, I = 1e-8 m^4, rho = 7850 kg/m^3, A = 1000 mm^2, L = 2 m, ends = cantilever, n = 9", "outcome": "INVALID_INPUT"}])


# ================================================================================================ structural analysis
def _gauss_solve(A, b):
    """Solve A x = b by Gaussian elimination with partial pivoting; ValueError when the matrix is singular."""
    n = len(A)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    scale = max(abs(v) for row in A for v in row) or 1.0
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(M[r][col]))
        if abs(M[piv][col]) <= 1e-12 * scale:
            raise ValueError("the structure is unstable or not statically determinate (singular equilibrium equations)")
        M[col], M[piv] = M[piv], M[col]
        for r in range(col + 1, n):
            f = M[r][col] / M[col][col]
            if f:
                for c in range(col, n + 1):
                    M[r][c] -= f * M[col][c]
    x = [0.0] * n
    for r in range(n - 1, -1, -1):
        x[r] = (M[r][n] - math.fsum(M[r][c] * x[c] for c in range(r + 1, n))) / M[r][r]
    return x


def _truss_read(nodes, members, pin, roller, loads):
    pts = [tuple(r) for r in nodes]
    if any(len(p) != 2 for p in pts) or not 3 <= len(pts) <= 30:
        raise ValueError("give 3 to 30 nodes as rows 'x y' (m)")
    n = len(pts)
    mem = []
    for e in members:
        if len(e) != 2:
            raise ValueError("members are pairs of node numbers, e.g. 1-2")
        try:
            i, j = int(e[0]), int(e[1])
        except ValueError:
            raise ValueError("members are pairs of node numbers, e.g. 1-2") from None
        if not (1 <= i <= n and 1 <= j <= n) or i == j:
            raise ValueError(f"member {e[0]}-{e[1]} does not join two different nodes")
        if pts[i - 1] == pts[j - 1]:
            raise ValueError("a member has zero length")
        mem.append((i - 1, j - 1))
    if len(set(tuple(sorted(m)) for m in mem)) != len(mem):
        raise ValueError("a member is listed twice")
    if not (1 <= pin <= n and 1 <= roller <= n) or pin == roller:
        raise ValueError("the pin and the roller must be two different nodes")
    ld = []
    for row in loads:
        if len(row) != 3 or row[0] != int(row[0]) or not 1 <= int(row[0]) <= n:
            raise ValueError("each load is a row 'node Fx Fy' (kN)")
        ld.append((int(row[0]) - 1, row[1], row[2]))
    if len(mem) + 3 != 2 * n:
        raise ValueError(f"not statically determinate: {len(mem)} members + 3 reactions must equal 2 x {n} joint equations")
    return pts, mem, pin - 1, roller - 1, ld


def truss(nodes, members, pin, roller, loads):
    pts, mem, pin, rol, ld = _truss_read(nodes, members, pin, roller, loads)
    n, m = len(pts), len(mem)
    A = [[0.0] * (m + 3) for _ in range(2 * n)]
    b = [0.0] * (2 * n)
    for k, (i, j) in enumerate(mem):
        dx, dy = pts[j][0] - pts[i][0], pts[j][1] - pts[i][1]
        L = math.hypot(dx, dy)
        cx, cy = dx / L, dy / L
        A[2 * i][k] += cx
        A[2 * i + 1][k] += cy
        A[2 * j][k] -= cx
        A[2 * j + 1][k] -= cy
    A[2 * pin][m] = 1.0
    A[2 * pin + 1][m + 1] = 1.0
    A[2 * rol + 1][m + 2] = 1.0
    for node, fx, fy in ld:
        b[2 * node] -= fx
        b[2 * node + 1] -= fy
    x = _gauss_solve(A, b)
    scale = max([abs(f) for _n, f, g in ld] + [abs(g) for _n, f, g in ld] + [1e-300])
    x = [_snap(v, scale) for v in x]
    forces = x[:m]
    return _out(forces_kN=forces, pin_Rx_kN=x[m], pin_Ry_kN=x[m + 1], roller_Ry_kN=x[m + 2],
                max_tension_kN=max([f for f in forces if f > 0] or [0.0]), max_compression_kN=-min([f for f in forces if f < 0] or [0.0]))


def ref_truss(nodes, members, pin, roller, loads):
    """sympy.physics.continuum_mechanics.truss.Truss: the same truss built in the library (exact rational
    arithmetic) and solved by it; member forces tension positive."""
    import sympy as sp
    from sympy.physics.continuum_mechanics.truss import Truss
    q = lambda v: sp.Rational(repr(round(float(v), 9)))          # short decimals: sympy's exact roots stay small
    t = Truss()
    for k, (x, y) in enumerate(nodes):
        t.add_node((f"n{k + 1}", q(x), q(y)))
    for k, e in enumerate(members):
        t.add_member((f"m{k + 1}", f"n{int(e[0])}", f"n{int(e[1])}"))
    t.apply_support((f"n{int(pin)}", "pinned"), (f"n{int(roller)}", "roller"))
    for node, fx, fy in loads:
        if fx:
            t.apply_load((f"n{int(node)}", q(abs(fx)), 0 if fx > 0 else 180))
        if fy:
            t.apply_load((f"n{int(node)}", q(abs(fy)), 90 if fy > 0 else 270))
    t.solve()
    scale = max([abs(r[1]) for r in loads] + [abs(r[2]) for r in loads] + [1e-300])
    forces = [_snap(float(t.internal_forces[f"m{k + 1}"]), scale) for k in range(len(members))]
    R = t.reaction_loads
    return {"forces_kN": forces, "pin_Rx_kN": _snap(float(R[f"R_n{int(pin)}_x"]), scale), "pin_Ry_kN": _snap(float(R[f"R_n{int(pin)}_y"]), scale),
            "roller_Ry_kN": _snap(float(R[f"R_n{int(roller)}_y"]), scale),
            "max_tension_kN": max([f for f in forces if f > 0] or [0.0]), "max_compression_kN": -min([f for f in forces if f < 0] or [0.0])}


def _rand_truss(rng):
    """A Pratt-type truss: bottom chord nodes 1..p+1, top nodes above the interior bottom nodes, verticals and
    diagonals; pin at the left end, roller at the right end, random joint loads."""
    p = rng.randint(2, 5)
    w, h = round(rng.uniform(1, 4), 1), round(rng.uniform(1, 4), 1)
    nodes = [[round(i * w, 6), 0.0] for i in range(p + 1)] + [[round(i * w, 6), h] for i in range(1, p)]
    top = lambda i: p + 1 + i                       # node number of the top node above bottom node i (1..p-1), 1-based
    mem = [[i + 1, i + 2] for i in range(p)]
    mem += [[top(i), top(i + 1)] for i in range(1, p - 1)]
    mem += [[i + 1, top(i)] for i in range(1, p)]
    mem += [[1, top(1)], [p + 1, top(p - 1)]]
    for i in range(1, p - 1):
        mem.append([top(i), i + 2] if i < p / 2 else [i + 1, top(i + 1)])
    loads = []
    for node in rng.sample(range(1, len(nodes) + 1), rng.randint(1, 3)):
        loads.append([float(node), rng.choice([0.0, round(rng.uniform(-20, 20), 1)]), round(rng.uniform(-60, -1), 1)])
    return {"nodes": nodes, "members": ", ".join(f"{a}-{b}" for a, b in mem), "pin": 1, "roller": p + 1, "loads": loads}


_add("truss_member_forces",
     name="Member forces of a plane truss (method of joints)", topic="structural analysis / trusses",
     purpose="The axial force in every member and the support reactions of a statically determinate plane pin-jointed truss on a pin and a roller, under joint loads.",
     inputs=[_mat("nodes", "node coordinates, one row 'x y' per node in m (node 1 is the first row)"), _edges("members", "members as pairs of node numbers, e.g. 1-2, 2-3", hi=57),
             _intin("pin", "node number of the pin support", lo=1), _intin("roller", "node number of the roller support (vertical reaction)", lo=1),
             _mat("loads", "joint loads, one row 'node Fx Fy' in kN (up and right positive)")],
     outputs=[_o("forces_kN", "number", "member forces in kN in the order given (tension positive)"), _o("pin_Rx_kN", "number", "horizontal reaction at the pin, kN"),
              _o("pin_Ry_kN", "number", "vertical reaction at the pin, kN"), _o("roller_Ry_kN", "number", "vertical reaction at the roller, kN"),
              _o("max_tension_kN", "number", "largest tension, kN"), _o("max_compression_kN", "number", "largest compression (magnitude), kN")],
     primary="pin_Ry_kN", conditions=["pin-jointed, loads at the joints", "statically determinate: members + 3 = 2 x nodes, geometrically stable", "roller on a horizontal surface"],
     method="equilibrium of every joint (sum Fx = 0, sum Fy = 0) as one linear system in the member forces and three reactions, solved by Gaussian elimination",
     latex=r"\sum_{\text{members at } j} F_m\,\hat{\mathbf e}_{m} + \mathbf R_j + \mathbf P_j = \mathbf 0",
     run=truss,
     parse=lambda q: _pairs(q, r"truss member forces|truss forces|method of joints", {"nodes": "nodes", "members": "members", "pin": "pin", "roller": "roller", "loads": "loads"},
                            matrices={"nodes", "loads"}, edges={"members"}),
     reference=ref_truss,
     wrong=_safe(lambda nodes, members, pin, roller, loads: (lambda r: dict(r, forces_kN=[-f for f in r["forces_kN"]], pin_Ry_kN=r["roller_Ry_kN"], roller_Ry_kN=r["pin_Ry_kN"]))(
         truss(nodes, members, pin, roller, loads))),
     random_inputs=_rand_truss,
     templates=["Truss member forces: nodes = [0 0; 4 0; 2 3], members = [1-2, 2-3, 3-1], pin = 1, roller = 2, loads = [3 0 -10]"],
     examples=[{"inputs": {"nodes": "0 0; 4 0; 2 3", "members": "1-2, 2-3, 3-1", "pin": 1, "roller": 2, "loads": "3 0 -10"},
                "outputs": {"forces_kN": [10 / 3, -5 * math.sqrt(13) / 3, -5 * math.sqrt(13) / 3], "pin_Ry_kN": 5.0, "roller_Ry_kN": 5.0, "pin_Rx_kN": 0.0}}],
     questions=[{"q": "Truss member forces: nodes = [0 0; 4 0; 2 3], members = [1-2, 2-3, 3-1], pin = 1, roller = 2, loads = [3 0 -10]",
                 "outputs": {"forces_kN": [10 / 3, -5 * math.sqrt(13) / 3, -5 * math.sqrt(13) / 3]}},
                {"q": "Truss member forces: nodes = [0 0; 4 0; 2 3], members = [1-2, 2-3], pin = 1, roller = 2, loads = [3 0 -10]", "outcome": "INVALID_INPUT"}])


def continuous_beam(spans, w):
    n = len(spans)
    if len(w) != n or not 2 <= n <= 6:
        raise ValueError("give 2 to 6 spans and one uniform load per span")
    for v in spans:
        _pos("every span", v)
    for v in w:
        _nonneg("every load", v)
    if not any(w):
        raise ValueError("no load")
    # three-moment equations for the interior support moments M1..M(n-1) (sagging positive), ends M0 = Mn = 0
    k = n - 1
    A = [[0.0] * k for _ in range(k)]
    b = [0.0] * k
    for i in range(1, n):
        L1, L2 = spans[i - 1], spans[i]
        r = i - 1
        A[r][r] = 2 * (L1 + L2)
        if r > 0:
            A[r][r - 1] = L1
        if r < k - 1:
            A[r][r + 1] = L2
        b[r] = -(w[i - 1] * L1 ** 3 + w[i] * L2 ** 3) / 4
    Mi = _gauss_solve(A, b)
    M = [0.0] + Mi + [0.0]
    R = [0.0] * (n + 1)
    for i in range(n):
        L = spans[i]
        VL = w[i] * L / 2 + (M[i + 1] - M[i]) / L
        R[i] += VL
        R[i + 1] += w[i] * L - VL
    sc = max(w) * max(spans) ** 2
    return _out(support_moments_kNm=[_snap(v, sc) for v in M], reactions_kN=[_snap(v, sc / max(spans)) for v in R])


def _ana_continuous(spans, w, per):
    from anastruct import SystemElements
    xs, x = [0.0], 0.0
    for L in spans:
        for i in range(1, per + 1):
            xs.append(x + L * i / per)
        x += L
    ss = SystemElements(EI=1e12, EA=1e12)
    for i in range(len(xs) - 1):
        ss.add_element(location=[[_mm(xs[i]), 0.0], [_mm(xs[i + 1]), 0.0]])
    sup = [per * i for i in range(len(spans) + 1)]
    ss.add_support_hinged(node_id=1)
    for s_ in sup[1:]:
        ss.add_support_roll(node_id=s_ + 1, direction=2)
    for i, wi in enumerate(w):
        if wi:
            ss.q_load(q=-wi, element_id=list(range(per * i + 1, per * i + per + 1)), direction="element")      # kN/m = N/mm
    ss.solve()
    Ms = [0.0] + [-float(ss.get_element_results(element_id=s_, verbose=True)["M"][-1]) / 1e6 for s_ in sup[1:-1]] + [0.0]
    R = [-float(ss.get_node_results_system(node_id=s_ + 1)["Fy"]) / 1e3 for s_ in sup]
    return Ms, R


def ref_continuous_beam(spans, w):
    """anastruct: the continuous beam as a frame model meshed with 4 and with 8 elements per span; its distributed
    loads go to the nodes as lumped forces, so its results converge exactly as 1/n^2 and the two meshes are combined
    by Richardson extrapolation ((4 r8 - r4)/3, measured error about 1e-12); moments and reactions at the supports."""
    M1, R1 = _ana_continuous(spans, w, 4)
    M2, R2 = _ana_continuous(spans, w, 8)
    sc = max(w) * max(spans) ** 2
    Ms = [_snap((4 * b - a) / 3, sc) for a, b in zip(M1, M2)]
    R = [_snap((4 * b - a) / 3, sc / max(spans)) for a, b in zip(R1, R2)]
    return {"support_moments_kNm": Ms, "reactions_kN": R}


def _rand_cont(rng):
    n = rng.randint(2, 4)
    return {"spans": [rng.randint(25, 100) * 80 / 1000 for _ in range(n)], "w": [round(rng.uniform(0, 40), 1) if rng.random() < 0.85 else 0.0 for _ in range(n)]}


_add("continuous_beam_three_moment",
     name="Continuous beam under uniform loads (three-moment equation)", topic="structural analysis / continuous beams",
     purpose="The bending moments over the supports and the support reactions of a continuous beam of equal stiffness over 2 to 6 simply supported spans, each with its own uniform load.",
     inputs=[_vec("spans", "span lengths in m, left to right (a list)", lo=2, hi=6), _vec("w", "uniform load on each span in kN/m (a list)", lo=2, hi=6)],
     outputs=[_o("support_moments_kNm", "number", "bending moment at each support in kN*m, end supports first and last (hogging negative)"),
              _o("reactions_kN", "number", "reaction at each support in kN")],
     primary="reactions_kN", conditions=["constant EI along the beam", "simple supports, no settlement", "end supports carry no moment"],
     method="Clapeyron: M_(i-1) L_i + 2 M_i (L_i + L_(i+1)) + M_(i+1) L_(i+1) = -(w_i L_i^3 + w_(i+1) L_(i+1)^3)/4; reactions from the span shears",
     latex=r"M_{i-1}L_i + 2M_i(L_i+L_{i+1}) + M_{i+1}L_{i+1} = -\frac{w_iL_i^3 + w_{i+1}L_{i+1}^3}{4}",
     run=continuous_beam,
     parse=lambda q: _pairs(q, r"continuous beam(?: three[- ]moment)?|three[- ]moment equation", {"spans": "spans", "L": "spans", "w": "w", "loads": "w"}, vectors={"spans", "w"}),
     reference=ref_continuous_beam,
     wrong=_safe(lambda spans, w: _out(support_moments_kNm=[0.0] * (len(spans) + 1), reactions_kN=[w[0] * spans[0] / 2] + [(w[i] * spans[i] + w[i + 1] * spans[i + 1]) / 2 for i in range(len(spans) - 1)] + [w[-1] * spans[-1] / 2])),
     random_inputs=_rand_cont,
     templates=["Continuous beam: spans = [6, 6], w = [10, 10]"],
     examples=[{"inputs": {"spans": "6, 6", "w": "10, 10"}, "outputs": {"support_moments_kNm": [0.0, -45.0, 0.0], "reactions_kN": [22.5, 75.0, 22.5]}}],
     questions=[{"q": "Continuous beam: spans = [6, 6], w = [10, 10]", "outputs": {"support_moments_kNm": [0.0, -45.0, 0.0], "reactions_kN": [22.5, 75.0, 22.5]}},
                {"q": "Continuous beam: spans = [6, 6], w = [10]", "outcome": "INVALID_INPUT"}])


# ================================================================================================ mechanisms
def slider_crank(r, l, theta, omega):
    _pos("r and l", r, l)
    _fin("theta and omega", theta, omega)
    if l <= r:
        raise ValueError("the connecting rod must be longer than the crank")
    s, c = math.sin(theta), math.cos(theta)
    root = math.sqrt(l * l - r * r * s * s)
    x = r * c + root
    dx = -r * s - r * r * s * c / root
    d2x = -r * c - r * r * (c * c - s * s) / root - r ** 4 * s * s * c * c / root ** 3
    sc = r * max(abs(omega), 1e-300) ** 2
    return _out(x=x, v=_snap(dx * omega, r * abs(omega)), a=_snap(d2x * omega * omega, sc), rod_angle_deg=_snap(math.degrees(math.asin(r * s / l)), 90.0))


@functools.lru_cache(maxsize=None)
def _slider_symbolic():
    """sympy: the slider position x(t) = r cos(w t) + sqrt(l^2 - r^2 sin^2(w t)) differentiated symbolically."""
    import sympy as sp
    r, l, w, t = sp.symbols("r l omega t", positive=True)
    x = r * sp.cos(w * t) + sp.sqrt(l ** 2 - r ** 2 * sp.sin(w * t) ** 2)
    return sp.lambdify((r, l, w, t), sp.diff(x, t), "math"), sp.lambdify((r, l, w, t), sp.diff(x, t, 2), "math")


def ref_slider_crank(r, l, theta, omega):
    """Position from the geometry (the slider on the line of stroke at distance l from the crank pin: the larger
    root of the quadratic, numpy.roots); velocity and acceleration by sympy's symbolic differentiation of x(t) at
    t = theta/omega (constant crank speed)."""
    import numpy as np
    px, py = r * math.cos(theta), r * math.sin(theta)
    x = max(np.roots([1.0, -2 * px, px * px + py * py - l * l]).real)
    vf, af = _slider_symbolic()
    if omega == 0:
        v = a = 0.0
    else:
        w = abs(omega)
        t = theta / omega if omega > 0 else -theta / w
        v = vf(r, l, w, t) * (1 if omega > 0 else -1)
        a = af(r, l, w, t)
    return {"x": float(x), "v": _snap(v, r * abs(omega)), "a": _snap(a, r * max(abs(omega), 1e-300) ** 2), "rod_angle_deg": _snap(math.degrees(math.atan2(py, x - px)), 90.0)}


_add("slider_crank_kinematics",
     name="Slider-crank mechanism: slider position, velocity and acceleration", topic="machine elements / mechanisms",
     purpose="The slider position, velocity and acceleration and the connecting-rod angle of an in-line slider-crank mechanism at a crank angle, the crank turning at constant speed.",
     inputs=[_phys("r", "length", "crank radius"), _phys("l", "length", "connecting rod length"), _phys("theta", "angle", "crank angle from the line of stroke (top dead centre at 0)"),
             _phys("omega", "angular_frequency", "constant crank speed (rad/s or rpm)")],
     outputs=[_o("x", "length", "distance of the slider pin from the crank centre"), _o("v", "speed", "slider velocity (positive away from the crank)"),
              _o("a", "acceleration", "slider acceleration"), _o("rod_angle_deg", "number", "connecting-rod angle to the line of stroke, degrees")],
     primary="x", conditions=["in-line (zero offset)", "l > r", "constant crank speed"],
     method="x = r cos(theta) + sqrt(l^2 - r^2 sin^2 theta); v = dx/dt and a = d^2x/dt^2 with dtheta/dt = omega",
     latex=r"x = r\cos\theta + \sqrt{l^2 - r^2\sin^2\theta},\quad v = \omega\frac{dx}{d\theta},\quad a = \omega^2\frac{d^2x}{d\theta^2}",
     run=slider_crank,
     parse=lambda q: _pairs(q, r"slider[- ]crank(?: kinematics)?", {"r": "r", "l": "l", "theta": "theta", "omega": "omega", "n": "omega"}, phys={"r", "l", "theta", "omega"}),
     reference=ref_slider_crank,
     wrong=_safe(lambda r, l, theta, omega: _out(x=r * math.cos(theta) + l, v=-r * omega * math.sin(theta), a=-r * omega * omega * math.cos(theta), rod_angle_deg=math.degrees(r * math.sin(theta) / l))),
     random_inputs=lambda rng: (lambda r: {"r": r, "l": r * rng.uniform(2.5, 6), "theta": rng.uniform(0, 2 * math.pi), "omega": rng.uniform(10, 600)})(_lu(rng, 0.01, 0.5)),
     templates=["Slider-crank: r = 50 mm, l = 200 mm, theta = 60 deg, omega = 3000 rpm"],
     examples=[{"inputs": {"r": 0.05, "l": 0.2, "theta": 0.0, "omega": 100.0}, "outputs": {"x": 0.25, "v": 0.0, "a": -0.05 * 1e4 * (1 + 0.25), "rod_angle_deg": 0.0}}],
     questions=[{"q": "Slider-crank: r = 50 mm, l = 200 mm, theta = 0 deg, omega = 100 rad/s", "outputs": {"x": 0.25, "a": -625.0}},
                {"q": "Slider-crank: r = 50 mm, l = 40 mm, theta = 60 deg, omega = 100 rad/s", "outcome": "INVALID_INPUT"}])


def four_bar(a, b, c, d, theta2, branch):
    _pos("a, b, c and d", a, b, c, d)
    _fin("theta2", theta2)
    K1, K2, K3 = d / a, d / c, (a * a - b * b + c * c + d * d) / (2 * a * c)
    K4, K5 = d / b, (c * c - d * d - a * a - b * b) / (2 * a * b)
    ct, st = math.cos(theta2), math.sin(theta2)
    A = ct - K1 - K2 * ct + K3
    B = -2 * st
    C = K1 - (K2 + 1) * ct + K3
    D = ct - K1 + K4 * ct + K5
    E = -2 * st
    F = K1 + (K4 - 1) * ct + K5
    disc4, disc3 = B * B - 4 * A * C, E * E - 4 * D * F
    if disc4 < 0 or disc3 < 0:
        raise ValueError("the linkage cannot be assembled at this input angle")
    sg = -1 if branch == "open" else 1
    t4 = 2 * math.atan2(-B + sg * math.sqrt(disc4), 2 * A)
    t3 = 2 * math.atan2(-E + sg * math.sqrt(disc3), 2 * D)
    deg = lambda t: (math.degrees(t) + 180.0) % 360.0 - 180.0
    mu = abs(deg(t4 - t3))
    mu = 180 - mu if mu > 90 else mu
    return _out(theta3_deg=deg(t3), theta4_deg=deg(t4), transmission_angle_deg=mu)


def ref_four_bar(a, b, c, d, theta2, branch):
    """Geometric construction with numpy: the coupler-rocker pin as an intersection of the circles about the crank
    pin (radius b) and the rocker pivot (radius c); the open branch is the one with the rocker pivot to the right of
    the directed coupler (crossed: to the left). The transmission angle from the law of cosines."""
    import numpy as np
    A = np.array([a * math.cos(theta2), a * math.sin(theta2)])
    O4 = np.array([d, 0.0])
    dv = O4 - A
    R = float(np.linalg.norm(dv))
    t = (b * b - c * c + R * R) / (2 * R)
    h = math.sqrt(max(b * b - t * t, 0.0))
    base = A + t * dv / R
    perp = np.array([-dv[1], dv[0]]) / R
    cands = [base + h * perp, base - h * perp]
    def side(B):
        u = B - A
        return u[0] * (O4[1] - A[1]) - u[1] * (O4[0] - A[0])
    B = min(cands, key=side) if branch == "open" else max(cands, key=side)
    deg = lambda x: (math.degrees(x) + 180.0) % 360.0 - 180.0
    t3 = math.atan2(B[1] - A[1], B[0] - A[0])
    t4 = math.atan2(B[1] - O4[1], B[0] - O4[0])
    mu = math.degrees(math.acos(max(-1.0, min(1.0, (b * b + c * c - R * R) / (2 * b * c)))))
    return {"theta3_deg": deg(t3), "theta4_deg": deg(t4), "transmission_angle_deg": 180 - mu if mu > 90 else mu}


def _rand_four_bar(rng):
    for _ in range(100):
        a, b, c, d = (round(rng.uniform(1, 10), 2) for _ in range(4))
        th = rng.uniform(0, 2 * math.pi)
        br = rng.choice(["open", "crossed"])
        try:
            four_bar(a, b, c, d, th, br)
        except ValueError:
            continue
        # stay away from the toggle positions (where the two branches meet and the angles are ill-conditioned)
        A = (a * math.cos(th), a * math.sin(th))
        R = math.hypot(d - A[0], A[1])
        if abs(R - (b + c)) > 0.05 * (b + c) and abs(R - abs(b - c)) > 0.05 * (b + c):
            return {"a": a, "b": b, "c": c, "d": d, "theta2": th, "branch": br}
    return {"a": 2.0, "b": 7.0, "c": 9.0, "d": 6.0, "theta2": 1.0, "branch": "open"}


_add("four_bar_linkage_position",
     name="Four-bar linkage position analysis", topic="machine elements / mechanisms",
     purpose="The coupler and output-rocker angles and the transmission angle of a four-bar linkage at a given crank angle, on the open or the crossed branch.",
     inputs=[_phys("a", "length", "input crank length"), _phys("b", "length", "coupler length"), _phys("c", "length", "output rocker length"), _phys("d", "length", "ground link length"),
             _phys("theta2", "angle", "crank angle from the ground link"), _choice("branch", ["open", "crossed"], "assembly branch")],
     outputs=[_o("theta3_deg", "number", "coupler angle, degrees in [-180, 180)"), _o("theta4_deg", "number", "output rocker angle, degrees in [-180, 180)"),
              _o("transmission_angle_deg", "number", "transmission angle (acute), degrees")],
     primary="theta4_deg", conditions=["ground pivots at (0, 0) and (d, 0)", "open branch: the rocker pivot lies to the right of the directed coupler", "refused when the linkage cannot close"],
     method="Freudenstein's equation: K1 cos th4 - K2 cos th2 + K3 = cos(th2 - th4) solved by the half-angle substitution, th4 = 2 atan2(-B -+ sqrt(B^2 - 4AC), 2A); likewise th3",
     latex=r"K_1\cos\theta_4 - K_2\cos\theta_2 + K_3 = \cos(\theta_2-\theta_4),\quad \theta_4 = 2\arctan\frac{-B\mp\sqrt{B^2-4AC}}{2A}",
     run=four_bar,
     parse=lambda q: _pairs(q, r"four[- ]bar linkage(?: position)?|four[- ]bar", {"a": "a", "b": "b", "c": "c", "d": "d", "theta2": "theta2", "theta": "theta2", "branch": "branch"},
                            phys={"a", "b", "c", "d", "theta2"}, words={"branch"}),
     reference=ref_four_bar,
     wrong=_safe(lambda a, b, c, d, theta2, branch: four_bar(a, b, c, d, theta2, "crossed" if branch == "open" else "open")),
     random_inputs=_rand_four_bar,
     templates=["Four-bar linkage: a = 40 mm, b = 120 mm, c = 80 mm, d = 100 mm, theta2 = 40 deg, branch = open"],
     examples=[{"inputs": {"a": 2.0, "b": 7.0, "c": 9.0, "d": 6.0, "theta2": _deg(30), "branch": "open"},
                "outputs": {"theta3_deg": 88.8372413003, "theta4_deg": 117.28606786, "transmission_angle_deg": 28.4488265601}},
               {"inputs": {"a": 2.0, "b": 7.0, "c": 9.0, "d": 6.0, "theta2": _deg(30), "branch": "crossed"},
                "outputs": {"theta3_deg": -115.210812164, "theta4_deg": -143.659638724}}],
     questions=[{"q": "Four-bar linkage: a = 2 m, b = 7 m, c = 9 m, d = 6 m, theta2 = 30 deg, branch = open", "outputs": {"theta4_deg": 117.28606786}},
                {"q": "Four-bar linkage: a = 1 m, b = 1 m, c = 1 m, d = 5 m, theta2 = 90 deg, branch = open", "outcome": "INVALID_INPUT"}])


# ================================================================================================ definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Gerber and
# ASME-elliptic fatigue criteria, Lewis gear bending stress, Wahl factor, Marin endurance-limit factors, bolt
# preload torque.

def _brentq_ref(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def gerber(sa, sm, Se, Sut):
    _fat_check(sa, sm, Se, Sut)
    if sm == 0:
        return _out(n_fatigue=Se / sa, sigma_ar_MPa=sa / 1e6)
    r = sa / sm
    x = 2 * Se / (Sut * r)
    n = 0.5 * (Sut / sm) ** 2 * (sa / Se) * x * x / (1 + math.sqrt(1 + x * x))    # = ...(-1 + sqrt(1 + x^2)), no cancellation
    return _out(n_fatigue=n, sigma_ar_MPa=sa / (1 - (sm / Sut) ** 2) / 1e6)


def ref_gerber(sa, sm, Se, Sut):
    """The load-line multiple n at which n sa/Se + (n sm/Sut)^2 = 1 (the Gerber parabola), by scipy brentq; the
    equivalent amplitude sa/(1 - (sm/Sut)^2) by sympy's exact rational arithmetic."""
    import sympy as sp
    n = _brentq_ref(lambda k: k * sa / Se + (k * sm / Sut) ** 2 - 1, 1e-12, Se / sa * 1.000001)
    q = sp.Rational(repr(sa)) / (1 - (sp.Rational(repr(sm)) / sp.Rational(repr(Sut))) ** 2)
    return {"n_fatigue": n, "sigma_ar_MPa": float(q) / 1e6}


_add("gerber_fatigue_safety_factor",
     name="Fatigue safety factor by the Gerber criterion", topic="fatigue / mean-stress criteria",
     purpose="The fatigue factor of safety along a proportional load line by the Gerber parabola sa/Se + (sm/Sut)^2 = 1, and the equivalent fully reversed amplitude.",
     inputs=_FAT_IN + [_phys("Sut", "stress", "ultimate tensile strength")],
     outputs=[_o("n_fatigue", "number", "fatigue safety factor (Gerber, proportional loading)"),
              _o("sigma_ar_MPa", "number", "equivalent fully reversed amplitude sa/(1 - (sm/Sut)^2), MPa")],
     primary="n_fatigue", conditions=["sm >= 0 (compressive means not covered)", "sm < Sut", "proportional loading"],
     method="n sa/Se + (n sm/Sut)^2 = 1: n = (1/2)(Sut/sm)^2 (sa/Se) [-1 + sqrt(1 + (2 sm Se/(Sut sa))^2)]",
     latex=r"\frac{n\sigma_a}{S_e} + \left(\frac{n\sigma_m}{S_{ut}}\right)^2 = 1",
     run=gerber,
     parse=lambda q: _pairs(q, r"gerber(?: fatigue)?(?: safety factor)?", {"sa": "sa", "sm": "sm", "Se": "Se", "Sut": "Sut"}, phys={"sa", "sm", "Se", "Sut"}),
     reference=ref_gerber,
     wrong=_safe(lambda sa, sm, Se, Sut: _out(n_fatigue=1 / (sa / Se + sm / Sut), sigma_ar_MPa=sa / (1 - sm / Sut) / 1e6)),
     random_inputs=lambda rng: (lambda Sut: {"sa": rng.uniform(20e6, 300e6), "sm": rng.uniform(0.1, 0.7) * Sut, "Se": rng.uniform(0.3, 0.5) * Sut, "Sut": Sut})(rng.uniform(400e6, 1400e6)),
     templates=["Gerber safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa"],
     examples=[{"inputs": {"sa": 120e6, "sm": 80e6, "Se": 250e6, "Sut": 700e6},
                "outputs": {"n_fatigue": 0.5 * (700 / 80) ** 2 * (120 / 250) * (-1 + math.sqrt(1 + (2 * 80 * 250 / (700 * 120)) ** 2)),
                            "sigma_ar_MPa": 120 / (1 - (80 / 700) ** 2)}},
               {"inputs": {"sa": 100e6, "sm": 0.0, "Se": 250e6, "Sut": 700e6}, "outputs": {"n_fatigue": 2.5}}],
     questions=[{"q": "Gerber safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa",
                 "outputs": {"n_fatigue": 0.5 * (700 / 80) ** 2 * (120 / 250) * (-1 + math.sqrt(1 + (2 * 80 * 250 / (700 * 120)) ** 2))}},
                {"q": "Gerber safety factor: sa = 120 MPa, sm = 800 MPa, Se = 250 MPa, Sut = 700 MPa", "outcome": "INVALID_INPUT"},
                {"q": "Gerber safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa", "outcome": "NEEDS_INPUT"}])


def asme_elliptic(sa, sm, Se, Sy):
    _fat_check(sa, sm, Se, Sy)
    return _out(n_fatigue=1.0 / math.hypot(sa / Se, sm / Sy), sigma_ar_MPa=sa / math.sqrt(1 - (sm / Sy) ** 2) / 1e6)


def ref_asme_elliptic(sa, sm, Se, Sy):
    """The load-line multiple at which (n sa/Se)^2 + (n sm/Sy)^2 = 1 (the ASME ellipse), by scipy brentq; the
    equivalent amplitude as the Se-axis intercept of the ellipse through (sm, sa) scaled to Se (brentq)."""
    n = _brentq_ref(lambda k: (k * sa / Se) ** 2 + (k * sm / Sy) ** 2 - 1, 1e-12, Se / sa * 1.000001)
    sar = _brentq_ref(lambda s: (sa / s) ** 2 + (sm / Sy) ** 2 - 1, sa * 0.999999, sa * 1e6)
    return {"n_fatigue": n, "sigma_ar_MPa": sar / 1e6}


_add("asme_elliptic_fatigue_safety_factor",
     name="Fatigue safety factor by the ASME-elliptic criterion", topic="fatigue / mean-stress criteria",
     purpose="The fatigue factor of safety along a proportional load line by the ASME ellipse (sa/Se)^2 + (sm/Sy)^2 = 1, and the equivalent fully reversed amplitude.",
     inputs=_FAT_IN + [_phys("Sy", "stress", "yield strength")],
     outputs=[_o("n_fatigue", "number", "fatigue safety factor (ASME-elliptic, proportional loading)"),
              _o("sigma_ar_MPa", "number", "equivalent fully reversed amplitude sa/sqrt(1 - (sm/Sy)^2), MPa")],
     primary="n_fatigue", conditions=["sm >= 0", "sm < Sy", "proportional loading"],
     method="(n sa/Se)^2 + (n sm/Sy)^2 = 1: n = 1/sqrt((sa/Se)^2 + (sm/Sy)^2)",
     latex=r"\left(\frac{n\sigma_a}{S_e}\right)^2 + \left(\frac{n\sigma_m}{S_y}\right)^2 = 1",
     run=asme_elliptic,
     parse=lambda q: _pairs(q, r"asme[- ]elliptic(?: fatigue)?(?: safety factor)?", {"sa": "sa", "sm": "sm", "Se": "Se", "Sy": "Sy"}, phys={"sa", "sm", "Se", "Sy"}),
     reference=ref_asme_elliptic,
     wrong=_safe(lambda sa, sm, Se, Sy: _out(n_fatigue=1 / (sa / Se + sm / Sy), sigma_ar_MPa=sa / (1 - sm / Sy) / 1e6)),
     random_inputs=lambda rng: (lambda Sy: {"sa": rng.uniform(20e6, 250e6), "sm": rng.uniform(0.1, 0.8) * Sy, "Se": rng.uniform(0.3, 0.6) * Sy, "Sy": Sy})(rng.uniform(250e6, 1200e6)),
     templates=["ASME-elliptic safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, Sy = 450 MPa"],
     examples=[{"inputs": {"sa": 100e6, "sm": 60e6, "Se": 220e6, "Sy": 450e6}, "outputs": {"n_fatigue": 1 / math.sqrt((100 / 220) ** 2 + (60 / 450) ** 2)}},
               {"inputs": {"sa": 30e6, "sm": 40e6, "Se": 100e6, "Sy": 100e6}, "outputs": {"n_fatigue": 2.0}}],
     questions=[{"q": "ASME-elliptic safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, Sy = 450 MPa",
                 "outputs": {"n_fatigue": 1 / math.sqrt((100 / 220) ** 2 + (60 / 450) ** 2)}},
                {"q": "ASME-elliptic safety factor: sa = 100 MPa, sm = 500 MPa, Se = 220 MPa, Sy = 450 MPa", "outcome": "INVALID_INPUT"},
                {"q": "ASME-elliptic safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa", "outcome": "NEEDS_INPUT"}])


def lewis_stress(Wt, F, m, Y, Kv=1.0):
    _pos("Wt, F, m and Y", Wt, F, m, Y)
    if not (math.isfinite(Kv) and Kv >= 1):
        raise ValueError("the velocity factor Kv must be at least 1")
    if Y > 1:
        raise ValueError("the Lewis form factor Y must be at most 1 (based on the module)")
    s = Kv * Wt / (F * m * Y)
    return _out(sigma=s, sigma_MPa=s / 1e6)


def ref_lewis_stress(Wt, F, m, Y, Kv=1.0):
    """The tooth as a cantilever of the inscribed Lewis parabola: with the tooth thickness t and load height l chosen
    to give the form factor Y = 2x/(3m), x = t^2/(4l), the bending stress 6 Wt l/(F t^2) (numpy) times Kv."""
    t = 1.0 * m
    x = 1.5 * m * Y
    l = t * t / (4 * x)
    return {"sigma_MPa": Kv * 6 * Wt * l / (F * t * t) / 1e6}


_add("lewis_gear_bending_stress",
     name="Lewis bending stress of a gear tooth", topic="machine elements / gears",
     purpose="The bending stress at the root of a spur-gear tooth by the Lewis equation sigma = Kv Wt/(F m Y) (SI, module m, form factor Y based on the module).",
     inputs=[_phys("Wt", "force", "transmitted (tangential) load"), _phys("F", "length", "face width"), _phys("m", "length", "module"),
             _numin("Y", "Lewis form factor (from the tooth count and pressure angle, based on the module)"),
             _numin("Kv", "velocity (dynamic) factor, default 1", required=False)],
     outputs=[_o("sigma", "stress", "tooth bending stress"), _o("sigma_MPa", "number", "tooth bending stress, MPa")],
     primary="sigma_MPa", conditions=["Wt, F, m > 0", "0 < Y <= 1", "Kv >= 1", "load at the tip, one tooth in contact"],
     method="sigma = Kv Wt/(F m Y)", latex=r"\sigma = \frac{K_v W_t}{F\,m\,Y}",
     run=lewis_stress,
     parse=lambda q: _pairs(q, r"lewis(?: gear)?(?: bending)? stress|lewis equation", {"Wt": "Wt", "F": "F", "m": "m", "Y": "Y", "Kv": "Kv"}, phys={"Wt", "F", "m"}),
     reference=ref_lewis_stress,
     wrong=_safe(lambda Wt, F, m, Y, Kv=1.0: _out(sigma=Kv * Wt / (F * m), sigma_MPa=Kv * Wt / (F * m) / 1e6)),
     random_inputs=lambda rng: {"Wt": _lu(rng, 100, 50000), "F": rng.uniform(0.01, 0.15), "m": rng.uniform(0.001, 0.012), "Y": rng.uniform(0.2, 0.48),
                                **({"Kv": rng.uniform(1.0, 2.0)} if rng.random() < 0.5 else {})},
     templates=["Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322", "Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322, Kv = 1.3"],
     examples=[{"inputs": {"Wt": 2500.0, "F": 0.04, "m": 0.004, "Y": 0.322}, "outputs": {"sigma_MPa": 2500 / (0.04 * 0.004 * 0.322) / 1e6}},
               {"inputs": {"Wt": 1000.0, "F": 0.05, "m": 0.002, "Y": 0.25, "Kv": 1.5}, "outputs": {"sigma_MPa": 60.0}}],
     questions=[{"q": "Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322", "outputs": {"sigma_MPa": 2500 / (0.04 * 0.004 * 0.322) / 1e6}},
                {"q": "Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322, Kv = 0.5", "outcome": "INVALID_INPUT"},
                {"q": "Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm", "outcome": "NEEDS_INPUT"}])


def wahl_factor(d, D, F=None):
    _pos("d and D", d, D)
    C = D / d
    if C <= 2:
        raise ValueError("the spring index D/d must exceed 2")
    KW = (4 * C - 1) / (4 * C - 4) + 0.615 / C
    out = dict(index=C, K_wahl=KW, K_bergstrasser=(4 * C + 2) / (4 * C - 3), K_s=1 + 0.5 / C)
    if F is not None:
        _pos("F", F)
        out["tau_MPa"] = KW * 8 * F * D / (math.pi * d ** 3) / 1e6
    return _out(**out)


def ref_wahl_factor(d, D, F=None):
    """The factors in exact rational arithmetic (sympy) from the index C = D/d; the stress from the torsion of the
    wire (stress-function quadrature of J, as the helical-spring check) times the Wahl factor."""
    import sympy as sp
    C = sp.Rational(repr(D)) / sp.Rational(repr(d))
    KW = (4 * C - 1) / (4 * C - 4) + sp.Rational("0.615") / C
    out = {"index": float(C), "K_wahl": float(KW), "K_bergstrasser": float((4 * C + 2) / (4 * C - 3)), "K_s": float(1 + sp.Rational(1, 2) / C)}
    if F is not None:
        J = _prandtl_disk(d / 2)[0]
        out["tau_MPa"] = float(KW) * (F * D / 2) * (d / 2) / J / 1e6
    return out


_add("wahl_factor",
     name="Wahl curvature factor of a helical spring", topic="machine elements / springs",
     purpose="The Wahl stress-correction factor K_W = (4C - 1)/(4C - 4) + 0.615/C of a round-wire helical spring (with the Bergstrasser and direct-shear factors), and with a load the corrected maximum shear stress.",
     inputs=[_phys("d", "length", "wire diameter"), _phys("D", "length", "mean coil diameter"), _phys("F", "force", "axial load (optional)", required=False)],
     outputs=[_o("index", "number", "spring index C = D/d"), _o("K_wahl", "number", "Wahl factor"), _o("K_bergstrasser", "number", "Bergstrasser factor (4C + 2)/(4C - 3)"),
              _o("K_s", "number", "direct-shear factor 1 + 0.5/C"), _o("tau_MPa", "number", "maximum shear stress K_W 8FD/(pi d^3), MPa (with F)")],
     primary="K_wahl", conditions=["C = D/d > 2", "round wire, close-coiled"],
     method="K_W = (4C - 1)/(4C - 4) + 0.615/C; tau = K_W 8 F D/(pi d^3)",
     latex=r"K_W = \frac{4C-1}{4C-4} + \frac{0.615}{C},\quad \tau = K_W\frac{8FD}{\pi d^3}",
     run=wahl_factor,
     parse=lambda q: _pairs(q, r"wahl(?: curvature)? factor|spring curvature correction", {"d": "d", "D": "D", "F": "F"}, phys={"d", "D", "F"}),
     reference=ref_wahl_factor,
     wrong=_safe(lambda d, D, F=None: (lambda C: _out(index=C, K_wahl=(4 * C - 1) / (4 * C - 4), K_bergstrasser=(4 * C + 2) / (4 * C - 3), K_s=1 + 0.5 / C,
                                                     **({"tau_MPa": (4 * C - 1) / (4 * C - 4) * 8 * F * D / (math.pi * d ** 3) / 1e6} if F else {})))(D / d)),
     random_inputs=lambda rng: (lambda d: {"d": d, "D": d * rng.uniform(3, 14), **({"F": _lu(rng, 5, 2000)} if rng.random() < 0.5 else {})})(_lu(rng, 0.0005, 0.012)),
     templates=["Wahl factor: d = 3 mm, D = 24 mm", "Wahl factor: d = 3 mm, D = 24 mm, F = 100 N"],
     examples=[{"inputs": {"d": 0.003, "D": 0.024}, "outputs": {"index": 8.0, "K_wahl": 31 / 28 + 0.615 / 8, "K_bergstrasser": 34 / 29}},
               {"inputs": {"d": 0.002, "D": 0.012, "F": 50.0}, "outputs": {"tau_MPa": (23 / 20 + 0.615 / 6) * 8 * 50 * 0.012 / (math.pi * 0.002 ** 3) / 1e6}}],
     questions=[{"q": "Wahl factor: d = 3 mm, D = 24 mm", "outputs": {"K_wahl": 31 / 28 + 0.615 / 8}},
                {"q": "Wahl factor: d = 3 mm, D = 5 mm", "outcome": "INVALID_INPUT"},
                {"q": "Wahl factor: d = 3 mm", "outcome": "NEEDS_INPUT"}])

# Marin surface factor k_a = a Sut^b, Sut in MPa (Shigley, Table 6-2)
_MARIN_SURFACE = {"ground": (1.58, -0.085), "machined": (4.51, -0.265), "cold-drawn": (4.51, -0.265), "hot-rolled": (57.7, -0.718),
                  "as-forged": (272.0, -0.995)}
_MARIN_LOAD = {"bending": 1.0, "axial": 0.85, "torsion": 0.59}


def marin(Sut, surface, load, d=None, R=0.5, kd=1.0, kf=1.0):
    from statistics import NormalDist
    _pos("Sut", Sut)
    if surface not in _MARIN_SURFACE:
        raise ValueError("surface must be ground, machined, cold-drawn, hot-rolled or as-forged")
    if load not in _MARIN_LOAD:
        raise ValueError("load must be bending, axial or torsion")
    if not (0.5 <= R < 1):
        raise ValueError("the reliability must lie in [0.5, 1) (e.g. 0.99 or 99%)")
    for name, v in (("kd", kd), ("kf", kf)):
        if not (math.isfinite(v) and 0 < v <= 1.5):
            raise ValueError(f"{name} must lie in (0, 1.5]")
    S = Sut / 1e6
    a, b = _MARIN_SURFACE[surface]
    ka = a * S ** b
    if load == "axial":
        kb = 1.0
    else:
        if d is None:
            raise ValueError("the diameter d is needed for bending and torsion (size factor)")
        dm = d * 1e3
        if not (2.79 <= dm <= 254):
            raise ValueError("the size factor covers 2.79 mm <= d <= 254 mm")
        kb = 1.24 * dm ** -0.107 if dm <= 51 else 1.51 * dm ** -0.157
    kc = _MARIN_LOAD[load]
    ke = 1 - 0.08 * NormalDist().inv_cdf(R)
    Sep = 0.5 * S if S <= 1400 else 700.0
    return _out(ka=ka, kb=kb, kc=kc, ke=ke, Se_prime_MPa=Sep, Se_MPa=ka * kb * kc * kd * ke * kf * Sep, Se=ka * kb * kc * kd * ke * kf * Sep * 1e6)


def ref_marin(Sut, surface, load, d=None, R=0.5, kd=1.0, kf=1.0):
    """The reliability factor from scipy's normal quantile (k_e = 1 - 0.08 z_a); the surface and size factors from the
    stated power laws evaluated with numpy; the endurance limit as their product (numpy.prod)."""
    import numpy as np
    from scipy.stats import norm
    S = Sut / 1e6
    a, b = _MARIN_SURFACE[surface]
    ka = float(np.power(S, b) * a)
    kb = 1.0 if load == "axial" else float(np.where(d * 1e3 <= 51, 1.24 * np.power(d * 1e3, -0.107), 1.51 * np.power(d * 1e3, -0.157)))
    ke = 1 - 0.08 * float(norm.ppf(R))
    Sep = float(np.minimum(0.5 * S, 700.0))
    Se = float(np.prod([ka, kb, _MARIN_LOAD[load], kd, ke, kf, Sep]))
    return {"ka": ka, "kb": kb, "ke": ke, "Se_prime_MPa": Sep, "Se_MPa": Se}


def _rand_marin(rng):
    load = rng.choice(["bending", "bending", "torsion", "axial"])
    x = {"Sut": rng.uniform(300e6, 1800e6), "surface": rng.choice(list(_MARIN_SURFACE)), "load": load}
    if load != "axial" or rng.random() < 0.3:
        x["d"] = rng.uniform(0.003, 0.25)
    if rng.random() < 0.8:
        x["R"] = rng.choice([0.9, 0.95, 0.99, 0.999, rng.uniform(0.5, 0.9999)])
    if rng.random() < 0.3:
        x["kd"] = rng.uniform(0.8, 1.02)
    if rng.random() < 0.3:
        x["kf"] = rng.uniform(0.6, 1.0)
    return x


_add("marin_endurance_limit",
     name="Endurance limit with the Marin factors", topic="fatigue / endurance limit",
     purpose="The corrected endurance limit Se = ka kb kc kd ke kf Se' of a steel part from its tensile strength, surface finish, size, loading and reliability (Se' = 0.5 Sut up to 1400 MPa, else 700 MPa).",
     inputs=[_phys("Sut", "stress", "ultimate tensile strength"),
             _choice("surface", list(_MARIN_SURFACE), "surface finish (ground, machined, cold-drawn, hot-rolled, as-forged)"),
             _choice("load", list(_MARIN_LOAD), "loading (bending, axial, torsion)"),
             _phys("d", "length", "diameter (needed for bending and torsion, 2.79-254 mm)", required=False),
             _numin("R", "reliability (default 0.5; written as 0.99 or 99%)", required=False),
             _numin("kd", "temperature factor (default 1)", required=False), _numin("kf", "miscellaneous-effects factor (default 1)", required=False)],
     outputs=[_o("ka", "number", "surface factor a Sut^b"), _o("kb", "number", "size factor"), _o("kc", "number", "load factor"),
              _o("ke", "number", "reliability factor 1 - 0.08 z_a"), _o("Se_prime_MPa", "number", "uncorrected endurance limit, MPa"),
              _o("Se_MPa", "number", "corrected endurance limit, MPa"), _o("Se", "stress", "corrected endurance limit")],
     primary="Se_MPa", conditions=["steels (Se' = 0.5 Sut, at most 700 MPa)", "size factor for 2.79 <= d <= 254 mm (rotating round sections)", "0.5 <= R < 1"],
     method="ka = a Sut^b (a, b by surface, Sut in MPa); kb = 1.24 d^-0.107 (d <= 51 mm) or 1.51 d^-0.157; kc = 1, 0.85, 0.59; ke = 1 - 0.08 z_a",
     latex=r"S_e = k_a k_b k_c k_d k_e k_f\,S_e',\quad k_a = aS_{ut}^b,\quad k_e = 1 - 0.08\,z_a",
     run=marin,
     parse=lambda q: _pairs(q, r"marin(?: factors)?|marin endurance limit|corrected endurance limit",
                            {"Sut": "Sut", "surface": "surface", "load": "load", "d": "d", "R": "R", "reliability": "R", "kd": "kd", "kf": "kf"},
                            phys={"Sut", "d"}, words={"surface", "load"}, num_units={"R": ["%", "percent"]}),
     reference=ref_marin,
     wrong=_safe(lambda Sut, surface, load, d=None, R=0.5, kd=1.0, kf=1.0: (lambda o: dict(o, kb=1.0, Se_MPa=_r(o["Se_MPa"] / o["kb"]), Se=_r(o["Se"] / o["kb"])))(
         marin(Sut, surface, load, d if d is not None else 0.01, R, kd, kf))),
     random_inputs=_rand_marin,
     templates=["Marin factors: Sut = 690 MPa, surface = machined, load = bending, d = 32 mm, R = 0.99"],
     examples=[{"inputs": {"Sut": 690e6, "surface": "machined", "load": "bending", "d": 0.032, "R": 0.99},
                "outputs": {"ka": 4.51 * 690 ** -0.265, "kb": 1.24 * 32 ** -0.107, "Se_prime_MPa": 345.0}},
               {"inputs": {"Sut": 1600e6, "surface": "ground", "load": "axial"}, "outputs": {"ka": 1.58 * 1600 ** -0.085, "kb": 1.0, "kc": 0.85, "ke": 1.0,
                                                                                         "Se_MPa": 1.58 * 1600 ** -0.085 * 0.85 * 700}}],
     questions=[{"q": "Marin factors: Sut = 690 MPa, surface = machined, load = bending, d = 32 mm, R = 99%",
                 "outputs": {"ka": 4.51 * 690 ** -0.265, "kb": 1.24 * 32 ** -0.107}},
                {"q": "Marin factors: Sut = 1600 MPa, surface = ground, load = axial", "outputs": {"Se_MPa": 1.58 * 1600 ** -0.085 * 0.85 * 700}},
                {"q": "Marin factors: Sut = 690 MPa, surface = machined, load = bending", "outcome": "INVALID_INPUT"},
                {"q": "Marin factors: Sut = 690 MPa, surface = machined", "outcome": "NEEDS_INPUT"}])


def bolt_torque(K, d, F=None, T=None):
    if not (math.isfinite(K) and 0.05 <= K <= 0.5):
        raise ValueError("the nut (torque) factor K must lie between 0.05 and 0.5 (typically 0.2)")
    _pos("d", d)
    if (F is None) == (T is None):
        raise ValueError("give exactly one of the preload F and the torque T")
    if F is not None:
        _pos("F", F)
        T = K * F * d
    else:
        _pos("T", T)
        F = T / (K * d)
    return _out(T=T, F=F, F_kN=F / 1e3)


def ref_bolt_torque(K, d, F=None, T=None):
    """Torque and preload in exact rational arithmetic (sympy) from T = K F d, solved for the unknown with
    sympy.solve."""
    import sympy as sp
    Ts, Fs = sp.symbols("T F", positive=True)
    eq = sp.Eq(Ts, sp.Rational(repr(K)) * Fs * sp.Rational(repr(d)))
    if F is not None:
        t = sp.solve(eq.subs(Fs, sp.Rational(repr(F))), Ts)[0]
        return {"T": float(t), "F": F}
    f = sp.solve(eq.subs(Ts, sp.Rational(repr(T))), Fs)[0]
    return {"T": T, "F": float(f), "F_kN": float(f) / 1e3}


_add("bolt_preload_torque",
     name="Bolt tightening torque and preload (T = K F d)", topic="machine elements / bolted joints",
     purpose="The tightening torque T = K F d that produces a bolt preload F with nut factor K and nominal diameter d, or the preload produced by a torque.",
     inputs=[_numin("K", "nut (torque) factor (about 0.2 for unlubricated steel)"), _phys("d", "length", "nominal bolt diameter"),
             _phys("F", "force", "preload (give F or T)", required=False), _phys("T", "torque", "tightening torque (give F or T)", required=False)],
     outputs=[_o("T", "torque", "tightening torque"), _o("F", "force", "bolt preload"), _o("F_kN", "number", "bolt preload, kN")],
     primary="T", conditions=["0.05 <= K <= 0.5", "exactly one of F and T"],
     method="T = K F d; F = T/(K d)", latex=r"T = K F_i\,d",
     run=bolt_torque,
     parse=lambda q: _pairs(q, r"bolt (?:preload |tightening )?torque|bolt preload", {"K": "K", "d": "d", "F": "F", "T": "T"}, phys={"d", "F", "T"}),
     reference=ref_bolt_torque,
     wrong=_safe(lambda K, d, F=None, T=None: (lambda o: dict(o, T=_r(o["T"] / 2), F=_r(o["F"] * 2), F_kN=_r(o["F_kN"] * 2)) if F is None
                                               else dict(o, T=_r(o["T"] / 2)))(bolt_torque(K, d, F, T))),
     random_inputs=lambda rng: (lambda K, d: {"K": K, "d": d, **({"F": _lu(rng, 1e3, 3e5)} if rng.random() < 0.5 else {"T": _lu(rng, 1, 3000)})})(
         rng.uniform(0.1, 0.35), rng.uniform(0.004, 0.036)),
     templates=["Bolt torque: K = 0.2, d = 12 mm, F = 25 kN", "Bolt torque: K = 0.2, d = 12 mm, T = 60 N*m"],
     examples=[{"inputs": {"K": 0.2, "d": 0.012, "F": 25000.0}, "outputs": {"T": 60.0}},
               {"inputs": {"K": 0.2, "d": 0.012, "T": 60.0}, "outputs": {"F": 25000.0, "F_kN": 25.0}}],
     questions=[{"q": "Bolt torque: K = 0.2, d = 12 mm, F = 25 kN", "outputs": {"T": 60.0}},
                {"q": "Bolt torque: K = 0.2, d = 12 mm, T = 60 N*m", "outputs": {"F_kN": 25.0}},
                {"q": "Bolt torque: K = 0.2, d = 12 mm, F = 25 kN, T = 60 N*m", "outcome": "INVALID_INPUT"},
                {"q": "Bolt torque: K = 0.2, F = 25 kN", "outcome": "NEEDS_INPUT"}])
