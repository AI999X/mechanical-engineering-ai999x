# mechanical-engineering-ai999x — offline, verified mechanical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/mechanical-engineering-ai999x)](https://pypi.org/project/mechanical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/mechanical-engineering-ai999x)

**[Install](https://pypi.org/project/mechanical-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

mechanical-engineering-ai999x is an offline, deterministic Python solver that answers mechanical engineering questions with verified results from 90 certified capabilities, gives 312 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**mechanical-engineering-ai999x** is a compact, deterministic mechanical engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified mechanical engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified mechanical engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 312 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** statics, beams and section properties, stress and strain, torsion, columns, fatigue, machine elements, vibrations and structural analysis.

## Model overview

| Property | Value |
|---|---|
| Package | `mechanical-engineering-ai999x` 1.0.3 (import `mechanical_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic mechanical engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 90 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 312 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install mechanical-engineering-ai999x
```

```python
import mechanical_engineering_ai999x as mechanical_engineering

mechanical_engineering.ask("Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4").show()
# -> SOLVED — RA = 13333.3, RB = 6666.67, M_max = 26666.7, V_max = 13333.3  (simply_supported_beam_point_load)
mechanical_engineering.ask("Principal stresses: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa").show()
# -> SOLVED — s1_MPa = 70, s2_MPa = -30, tau_max_inplane_MPa = 50, theta_p_deg = 26.5651  (principal_stresses_2d)
mechanical_engineering.ask("Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa").show()
# -> SOLVED — n_fatigue = 1.68269, sigma_ar_MPa = 135.484  (goodman_fatigue_safety_factor)
mechanical_engineering.ask("Rainflow counting: history = [0, 5, -3, 4, -2, 6, -4, 1, 0]").show()
# -> SOLVED — ranges = [10.0, 9.0, 8.0, 6.0, 5.0, 1.0], counts = [0.5, 0.5, 0.5, 1.0, 1.0, 0.5], total_cycles = 4, max_range = 10  (rainflow_counting)
mechanical_engineering.solve("euler_buckling", E="200 GPa", I="8e-6 m^4", L="4 m", ends="fixed-pinned").show()
# -> SOLVED — P_cr = 2.01907e+06, P_cr_kN = 2019.07, K = 0.699156, Le = 2.79662  (euler_buckling)
mechanical_engineering.ask("Who created you?").message
# -> mechanical-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `mechanical-engineering-ai999x "<question>"`, `mechanical-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on mechanical-engineering-ai999x 1.0.3.

## Find the right capability, and draw it

`mechanical_engineering.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "mechanical-engineering-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import mechanical_engineering_ai999x as mechanical_engineering

mechanical_engineering.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `mechanical-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Simply supported beam with a point load** · _Beams_

> Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4

→ **RA = 13333.33333 N, RB = 6666.666667 N, M_max = 26666.66667 N*m, V_max = 13333.33333 N, deflection_at_load_mm = 4.444444444, max_deflection_mm = 4.838498257, x_max = 2.734013676 m**

<details><summary>Steps</summary>

- Simply supported beam with a point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4 → RA = 13333.33333 N, RB = 6666.666667 N, M_max = 26666.66667 N*m, V_max = 13333.33333 N, deflection_at_load_mm = 4.444444444 N*m, max_deflection_mm = 4.838498257 N*m, x_max = 2.734013676 m

</details>

**Principal stresses and maximum shear (Mohr's circle, plane stress)** · _Stress analysis_

> Principal stresses: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa

→ **s1_MPa = 70, s2_MPa = -30, tau_max_inplane_MPa = 50, theta_p_deg = 26.56505118, tau_abs_max_MPa = 50, von_mises_MPa = 88.88194417**

<details><summary>Steps</summary>

- Principal stresses and maximum shear (Mohr's circle, plane stress): sx = 50 MPa, sy = -10 MPa, txy = 40 MPa → s1_MPa = 70, s2_MPa = -30, tau_max_inplane_MPa = 50, theta_p_deg = 26.56505118, tau_abs_max_MPa = 50, von_mises_MPa = 88.88194417

</details>

**Fatigue safety factor by the modified Goodman criterion** · _Fatigue_

> Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa

→ **n_fatigue = 1.682692308, sigma_ar_MPa = 135.483871**

<details><summary>Steps</summary>

- Fatigue safety factor by the modified Goodman criterion: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa → n_fatigue = 1.682692308, sigma_ar_MPa = 135.483871

</details>

**Rainflow cycle counting of a load history (ASTM E1049)** · _Fatigue_

> Rainflow counting: history = [0, 5, -3, 4, -2, 6, -4, 1, 0]

→ **ranges = [10, 9, 8, 6, 5, 1], counts = [0.5, 0.5, 0.5, 1, 1, 0.5], total_cycles = 4, max_range = 10**

<details><summary>Steps</summary>

- Rainflow cycle counting of a load history (ASTM E1049): history = [0, 5, -3, 4, -2, 6, -4, 1, 0] → ranges = [10, 9, 8, 6, 5, 1], counts = [0.5, 0.5, 0.5, 1, 1, 0.5], total_cycles = 4, max_range = 10

</details>

**Euler buckling load of a column with given end conditions** · _Columns_

`m.solve("euler_buckling", E="200 GPa", I="8e-6 m^4", L="4 m", ends="fixed-pinned")`

→ **P_cr = 2019072.856 N, P_cr_kN = 2019.072856, K = 0.6991556596, Le = 2.796622639 m**

<details><summary>Steps</summary>

- Euler buckling load of a column with given end conditions: E = 200 GPa, I = 8e-06 m^4, L = 4 m, ends = fixed-pinned → P_cr = 2019072.856 N, P_cr_kN = 2019.072856 N, K = 0.6991556596 N, Le = 2.796622639 m

</details>

**Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr)** · _Failure theories_

> Brittle failure: sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = 750 MPa

→ **n_max_normal = 2.333333333, n_coulomb_mohr = 2.019230769, n_modified_mohr = 2.333333333**

<details><summary>Steps</summary>

- Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr): sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = 750 MPa → n_max_normal = 2.333333333, n_coulomb_mohr = 2.019230769, n_modified_mohr = 2.333333333

</details>

## Figures

Every point was computed by `mechanical-engineering-ai999x`.

**Mohr's circle of a plane stress state**

![Mohr's circle of a plane stress state — computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/principal-stresses-2d-mohr.svg)

_Mohr's circle for σx = 50 MPa, σy = -10 MPa, τxy = 40 MPa: the normal and shear stress on planes rotated by 0–180° (73 states computed by the package's stress transformation) trace the circle; the principal stresses it returned are σ1 = 70 MPa and σ2 = -30 MPa at θp = 26.5651°; marked: centre σ = 20 MPa, τmax = 50 MPa._

**Shear and bending-moment diagrams of a beam**

![Shear and bending-moment diagrams of a beam — computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/simply-supported-beam-point-loads-beam.svg)

_Shear-force and bending-moment diagrams of a simply supported beam of span 8 m with point loads 10, 20 kN at x = 2, 5 m, drawn from the reactions the package returned (RA = 15 kN, RB = 15 kN); marked: the maximum moment it returned, x = 5 m, M = 45 kN·m._

**A cantilever's deflected shape under a tip load**

![A cantilever's deflected shape under a tip load — computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/cantilever-point-load-beam.svg)

_Deflected shape of a cantilever (L = 2 m, E = 2.0000e+11 Pa, I = 8.0000e-06 m⁴) under a tip load P = 5000 N: the deflection at each of 40 positions x is the tip deflection the package returns for the load placed at x (Maxwell's reciprocal theorem). Marked: x = 2 m, tip deflection = 8.33333 mm (downward); the wall and beam depth are drawn for orientation._

**Fatigue diagram: Goodman, Gerber and Soderberg envelopes**

![Fatigue diagram: Goodman, Gerber and Soderberg envelopes — computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/goodman-fatigue-safety-factor-fatigue.svg)

_Fatigue failure envelopes for Se = 250 MPa, Sut = 700 MPa, Sy = 500 MPa: each point is the stress state at which the package's safety factor along a load line equals 1 (47 load lines per criterion). Marked: the operating point σm = 80 MPa, σa = 120 MPa, Goodman safety factor n = 1.68269._

**Forced vibration: resonance curves for four dampings**

![Forced vibration: resonance curves for four dampings — computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/forced-vibration-amplitude-response.svg)

_Steady-state magnification of a forced spring–mass–damper (m = 10 kg, k = 4000 N/m, natural frequency 20 rad/s) for four damping coefficients, 120 forcing frequencies each computed by the package; marked: ω = 20 rad/s, c = 40 N·s/m, magnification = 5._

## Capabilities

90 certified procedures. A few of them:

- **Procedures:** All critical (whirling) speeds of a shaft with several disks; Angle of twist of a stepped shaft; Bar fixed at both ends with an axial load at a step; Belt or rope friction on a drum (capstan equation); Bolt tightening torque and preload (T = K F d); Cantilever with a point load; Cantilever with a triangular (linearly varying) load; Cantilever with a uniformly distributed load; Combined axial force and biaxial bending of a rectangular section; Composite section of rectangles (parallel-axis theorem); Continuous beam under uniform loads (three-moment equation); Critical speed of a shaft carrying one disk; Critical stress of an intermediate column (Johnson parabola / Euler); Damping ratio, damped frequency and logarithmic decrement; Eccentrically loaded bolt group (elastic method); Eccentrically loaded column (secant formula); Endurance limit with the Marin factors; Euler buckling load of a column with given end conditions; Fatigue damage of a stress history (rainflow counting + Miner's rule); Fatigue damage on the trilinear (Eurocode 3) S-N curve; Fatigue life from the S-N line (10^3 to 10^6 cycles); Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law; Fatigue safety factor by the ASME-elliptic criterion; Fatigue safety factor by the Gerber criterion …

`mechanical_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified mechanical engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (statics, beams and section properties, stress and strain, torsion, columns, fatigue, machine elements, vibrations and structural analysis); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{mechanical_engineering_ai999x_1_0_3,
  title  = {mechanical-engineering-ai999x: an offline, verified mechanical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
  url    = {https://huggingface.co/AI999X/mechanical-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/mechanical-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

mechanical-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/mechanical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/mechanical-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/onnx).
