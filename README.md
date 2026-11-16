# Mechanical Engineering (mechanical-engineering-ai999x)

[![tests](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/mechanical-engineering-ai999x)](https://pypi.org/project/mechanical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/mechanical-engineering-ai999x)

Mechanical Engineering covers statics, stress and strain, beams and frames, section properties, torsion, columns, fatigue, machine elements, vibrations and structural analysis: 97 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Mechanical Engineering desktop app: Endurance limit with the Marin factors, with its worked example and live scene](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/mechanical-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `mechanical-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/mechanical-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install mechanical-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import mechanical_engineering_ai999x as mechanical_engineering

a = mechanical_engineering.ask(
    "Shrink fit: delta = 0.02 mm, R = 25 mm, ro = 50 mm, E = 207 GPa"
)
print(a.outcome, a.values)
# SOLVED {'p_MPa': 62.1, 'hub_hoop_MPa': 103.5, 'shaft_hoop_MPa': -62.1}

a = mechanical_engineering.ask(
    "Bending stress: M = 10 kN*m, I = 8e-6 m^4, y = 100 mm"
)
print(a.outcome, a.values)
# SOLVED {'sigma': -125000000.0, 'sigma_MPa': -125.0}

a = mechanical_engineering.ask(
    "Rainflow fatigue damage: history = [0, 90, 0], Sc = 90, m = 3"
)
print(a.outcome, a.values)
# SOLVED {'damage': 5e-07, 'repeats_to_failure': 2000000.0, 'cycles': 1.0}

a = mechanical_engineering.ask(
    "Bolt torque: K = 0.2, d = 12 mm, F = 25 kN"
)
print(a.outcome, a.values)
# SOLVED {'T': 60.0, 'F': 25000.0, 'F_kN': 25.0}

a = mechanical_engineering.ask(
    "Moment of a force: Fx = 100 N, Fy = 200 N, x = 2 m, y = 1 m"
)
print(a.outcome, a.values)
# SOLVED {'M': 300.0, 'd': 1.3416407865}

a = mechanical_engineering.ask(
    "Resultant of concurrent forces: F = [3, 4], theta = [0, 90]"
)
print(a.outcome, a.values)
# SOLVED {'R': 5.0, 'Rx': 3.0, 'Ry': 4.0, 'direction_deg': 53.1301023542}

a = mechanical_engineering.ask(
    "Dunkerley critical speed: masses = [50], positions = [0.5], L = 1 "
    "m, E = 200 GPa, I = 2e-7 m^4"
)
print(a.outcome, a.values)
# SOLVED {'omega_cr': 195.959179423, 'N_cr_rpm': 1871.27232296}

a = mechanical_engineering.ask(
    "Two cable tensions: W = 500 N, alpha = 30 deg, beta = 45 deg"
)
print(a.outcome, a.values)
# SOLVED {'T1': 366.025403784, 'T2': 448.287736084}

a = mechanical_engineering.ask(
    "ASME-elliptic safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 "
    "MPa, Sy = 450 MPa"
)
print(a.outcome, a.values)
# SOLVED {'n_fatigue': 2.11105175963, 'sigma_ar_MPa': 100.900919099}

a = mechanical_engineering.ask(
    "Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322"
)
print(a.outcome, a.values)
# SOLVED {'sigma': 48524844.7205, 'sigma_MPa': 48.5248447205}

a = mechanical_engineering.ask("Transmissibility: zeta = 0, r = 2")
print(a.outcome, a.values)
# SOLVED {'TR': 0.333333333333, 'isolation_percent': 66.6666666667}

a = mechanical_engineering.ask(
    "Gerber safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, "
    "Sut = 700 MPa"
)
print(a.outcome, a.values)
# SOLVED {'n_fatigue': 1.97698086182, 'sigma_ar_MPa': 121.58808933}

a = mechanical_engineering.ask(
    "Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, "
    "Sut = 700 MPa"
)
print(a.outcome, a.values)
# SOLVED {'n_fatigue': 1.68269230769, 'sigma_ar_MPa': 135.483870968}

a = mechanical_engineering.ask(
    "Soderberg safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, "
    "Sy = 450 MPa"
)
print(a.outcome, a.values)
# SOLVED {'n_fatigue': 1.70103092784, 'sigma_ar_MPa': 115.384615385}

a = mechanical_engineering.ask(
    "S-N fatigue strength: Sut = 700 MPa, Se = 300 MPa, f = 0.85, N = "
    "1000000"
)
print(a.outcome, a.values)
# SOLVED {'Sf_MPa': 300.0}
```

## Screenshots

[![The Mechanical Engineering desktop app: Endurance limit with the Marin factors, with its worked example and live scene](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/mechanical-engineering-ai999x/)

[![Torsional natural frequency of a two-rotor system: the live scene in the Mechanical Engineering desktop app](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/mechanical-engineering-ai999x/)

*Torsional natural frequency of a two-rotor system*

[![Cantilever with a point load: the live scene in the Mechanical Engineering desktop app](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/mechanical-engineering-ai999x/)

*Cantilever with a point load*

[![Section properties of a solid rectangle: the live scene in the Mechanical Engineering desktop app](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/mechanical-engineering-ai999x/)

*Section properties of a solid rectangle*

Every calculation, with its formula, a worked example and its figures: **[Mechanical Engineering on ai999x.com](https://www.ai999x.com/models/mechanical-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/mechanical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/mechanical-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `mechanical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/mechanical-engineering`) and its data (`rust/models/mechanical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p mechanical-engineering-ai999x
./target/release/mechanical-engineering --help
cargo test -p mechanical-engineering-ai999x
```
