# mechanical-engineering-ai999x — offline, verified mechanical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/mechanical-engineering-ai999x)](https://pypi.org/project/mechanical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/mechanical-engineering-ai999x)

**[Install](https://pypi.org/project/mechanical-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

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
| Package | `mechanical-engineering-ai999x` 1.0.0 (import `mechanical_engineering_ai999x`) |
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

Command line: `mechanical-engineering-ai999x "<question>"`, `mechanical-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on mechanical-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
@software{mechanical_engineering_ai999x_1_0_0,
  title  = {mechanical-engineering-ai999x: an offline, verified mechanical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
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
