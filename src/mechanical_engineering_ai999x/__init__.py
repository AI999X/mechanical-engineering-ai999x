"""mechanical-engineering-ai999x — offline mechanical engineering answers with their formula, steps and verification.

    import mechanical_engineering_ai999x as m
    m.ask("Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4").show()
    m.ask("Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa").show()
    m.solve("euler_buckling", E="200 GPa", I="8e-6 m^4", L="4 m", ends="fixed-pinned").show()
    m.capabilities()                   # every certified capability

What it covers (84 certified capabilities, beginner to expert):
- statics: resultant of concurrent forces, two-cable equilibrium, moment of a force, plane truss member forces
  (method of joints);
- beams and frames: simply supported, cantilever, fixed-fixed and propped beams under point, uniform and triangular
  loads (reactions, moments, deflections, maximum deflection and its position), several point loads, continuous
  beams by the three-moment equation;
- section properties: rectangle, circle, tube, box, I, T, channel and angle sections, composite sections of
  rectangles and arbitrary polygons (centroid, second moments, product of inertia, principal axes, elastic and
  plastic moduli, radii of gyration);
- stress and strain: stepped and statically indeterminate bars, stress transformation, principal stresses in 2D and
  3D, von Mises and Tresca, brittle Coulomb-Mohr and modified Mohr, strain rosettes, generalized Hooke's law,
  combined axial force and biaxial bending, thin- and thick-walled pressure vessels, shrink fits, rotating disks;
- torsion: solid and hollow shafts, power transmission, stepped and statically indeterminate shafts;
- columns: Euler buckling with end conditions, Johnson parabola, secant formula;
- fatigue: Goodman and Soderberg safety factors, mean-stress corrections (Morrow, SWT, Walker) with Basquin life,
  S-N life and strength, Miner's rule on single-slope and Eurocode curves, rainflow counting and damage, shaft
  design by DE-Goodman;
- machine elements: belts (capstan, power with centrifugal tension), power screws, disc clutches, planetary trains,
  spur gear contact ratio, rolling-bearing life and rating at a reliability, helical springs, eccentric bolt groups,
  slider-crank and four-bar mechanisms;
- vibrations: natural frequency, damping, free and forced response, transmissibility, rotating unbalance, isolator
  design, two-rotor torsional systems, spring-mass chains, shaft critical speeds (single disk, Rayleigh, Dunkerley,
  exact whirling) and beam natural frequencies.

Questions are written as the capability phrase, a colon and `name = value` pairs (see templates()); psi, ksi, lbf,
kip and inch units are accepted. Built by the AI999X model machine from the same engine as every domain; a question
that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("mechanical_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
