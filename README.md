# mechanical-engineering-ai999x: offline, verified mechanical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/mechanical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/mechanical-engineering-ai999x)](https://pypi.org/project/mechanical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/mechanical-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/mechanical-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/mechanical-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/mechanical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.5.0

Field: **Engineering** · Tasks: Question Answering

mechanical-engineering-ai999x is an offline, deterministic mechanical engineering solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers mechanical engineering questions with verified results from 90 certified capabilities, gives 312 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**mechanical-engineering-ai999x** reads a question in plain English (or a typed call), picks a certified mechanical engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** mechanical-engineering-ai999x comes as the AI999X desktop app and as the command-line program `mechanical-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Mechanical Engineering desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-mechanical-engineering-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-mechanical-engineering-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`mechanical-engineering`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/mechanical-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install mechanical-engineering-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Mechanical Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Mechanical Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-mechanical-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-mechanical-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-mechanical-engineering-1.5.0-linux-x86_64/ai999x-mechanical-engineering
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
mechanical-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
mechanical-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ mechanical-engineering ask "Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4"
Read as: Simply supported beam with a point load
RA = 13333.33333 N
RB = 6666.666667 N
M_max = 26666.66667 N*m
V_max = 13333.33333 N
deflection_at_load_mm = 4.444444444
max_deflection_mm = 4.838498257
x_max = 2.734013676 m
  by: Simply supported beam with a point load  (RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise))
  E = 2e+11 Pa
  I = 8e-05 m^4
  L = 6 m
  P = 20000 N
  a = 2 m
  assumes: 0 < a < L; Euler-Bernoulli beam, linear elastic, prismatic
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ mechanical-engineering info simply_supported_beam_point_load
Simply supported beam with a point load
  RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise)
  in   L        span [m]
  in   P        point load (downward) [N]
  in   a        distance of the load from the left support A [m]
  in   E        Young's modulus [Pa]
  in   I        second moment of area of the section [m^4]
  out  RA       reaction at the left support A [N]
  out  RB       reaction at the right support B [N]
  out  M_max    maximum bending moment (under the load) [N*m]
  out  V_max    maximum shear force [N]
  out  deflection_at_load_mm deflection under the load in mm []
  out  max_deflection_mm maximum deflection in mm []
  out  x_max    position of the maximum deflection from A [m]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ mechanical-engineering solve simply_supported_beam_point_load "L=6 m" "P=20 kN" "a=2 m" "E=200 GPa" "I=8e-05 m^4"
RA = 13333.33333 N
RB = 6666.666667 N
M_max = 26666.66667 N*m
V_max = 13333.33333 N
deflection_at_load_mm = 4.444444444
max_deflection_mm = 4.838498257
x_max = 2.734013676 m
  by: Simply supported beam with a point load  (RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise))
  L = 6 m
  P = 20000 N
  a = 2 m
  E = 2e+11 Pa
  I = 8e-05 m^4
  assumes: 0 < a < L; Euler-Bernoulli beam, linear elastic, prismatic
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | mechanical-engineering batch

`cases.jsonl`:

```json
{"key": "simply_supported_beam_point_load", "inputs": {"L": "6 m", "P": "20 kN", "a": "2 m", "E": "200 GPa", "I": "8e-05 m^4"}}
{"key": "principal_stresses_2d", "inputs": {"sx": "50 MPa", "sy": "-10 MPa", "txy": "40 MPa"}}
```

```console
$ mechanical-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":26666.6666667,"unit":"N*m","solved_for":"M_max","si":[["L",6.0],["P",20000.0],["a",2.0],["E",200000000000.0],["I",0.00008]],"values":[["RA",13333.3333333,"N"],["RB",6666.66666667,"N"],["M_max",26666.6666667,"N*m"],["V_max",13333.3333333,"N"],["deflection_at_load_mm",4.44444444444,""],["max_deflection_mm",4.83849825735,""],["x_max",2.73401367629,"m"]],"target":"M_max","phrase":"reaction at the left support A","assumptions":["0 < a < L","Euler-Bernoulli beam, linear elastic, prismatic"],"name":"Simply supported beam with a point load","equation":"RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise)"}
{"outcome":"SOLVED","message":"","value":70.0,"unit":"","solved_for":"s1_MPa","si":[["sx",50000000.0],["sy",-10000000.0],["txy",40000000.0]],"values":[["s1_MPa",70.0,""],["s2_MPa",-30.0,""],["tau_max_inplane_MPa",50.0,""],["theta_p_deg",26.5650511771,""],["tau_abs_max_MPa",50.0,""],["von_mises_MPa",88.8819441732,""]],"target":"s1_MPa","phrase":"larger in-plane principal stress, MPa","assumptions":["plane stress (sigma_z = 0)","tension positive"],"name":"Principal stresses and maximum shear (Mohr's circle, plane stress)","equation":"sigma_1,2 = (sx + sy)/2 +- R, R = sqrt(((sx - sy)/2)^2 + txy^2); tan 2theta_p = 2 txy/(sx - sy); tau_abs = max(s1 - s2, |s1|, |s2|)/2"}
```

### In your browser

Open [Mechanical Engineering on ai999x.com](https://www.ai999x.com/models/mechanical-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `mechanical_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install mechanical-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Cantilever with a point load

Reaction, fixed-end moment, deflection under the load, tip deflection and tip slope of a cantilever carrying one point load at distance a from the fixed end (default: at the free end).

Command line:

```console
$ mechanical-engineering solve cantilever_point_load "L=2 m" "P=5 kN" "E=200 GPa" "I=8e-06 m^4"
R = 5000 N
M_fixed = 10000 N*m
deflection_at_load_mm = 8.333333333
tip_deflection_mm = 8.333333333
tip_slope_deg = 0.358098622
  by: Cantilever with a point load  (M = Pa, deflection under the load Pa^3/(3EI), slope there Pa^2/(2EI), tip deflection Pa^2(3L - a)/(6EI))
  L = 2 m
  P = 5000 N
  E = 2e+11 Pa
  I = 8e-06 m^4
  assumes: 0 < a <= L; Euler-Bernoulli beam, small deflections
```

Python:

```python
import mechanical_engineering_ai999x as m

a = m.solve("cantilever_point_load", L="2 m", P="5 kN", E="200 GPa", I="8e-06 m^4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/mechanical-engineering-ai999x/#cap-cantilever_point_load) and press **Compute in your browser**.

_What this means:_ The cantilever deflects 8.33333 under the load and 8.33333 at the tip, with a slope of 0.358099.

### Cantilever with a triangular (linearly varying) load

Reaction, fixed-end moment, tip deflection and tip slope of a cantilever whose load falls linearly from w at the fixed end to zero at the free end.

Command line:

```console
$ mechanical-engineering solve cantilever_triangular_load "L=3 m" "w=6 kN/m" "E=200 GPa" "I=2e-05 m^4"
R = 9000 N
M_fixed = 9000 N*m
tip_deflection_mm = 4.05
tip_slope_deg = 0.09668662793
  by: Cantilever with a triangular (linearly varying) load  (R = wL/2, M = wL^2/6, delta_tip = wL^4/(30EI), theta_tip = wL^3/(24EI))
  L = 3 m
  w = 6000 N/m
  E = 2e+11 Pa
  I = 2e-05 m^4
  assumes: L, w, E, I > 0; maximum intensity at the fixed end; Euler-Bernoulli beam
```

Python:

```python
import mechanical_engineering_ai999x as m

a = m.solve("cantilever_triangular_load", L="3 m", w="6 kN/m", E="200 GPa", I="2e-05 m^4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/mechanical-engineering-ai999x/#cap-cantilever_triangular_load) and press **Compute in your browser**.

_What this means:_ This calculation determines the structural response of a cantilever beam subjected to a triangular load that decreases linearly from maximum intensity at the fixed support to zero at the free end. Using the input values for length, load intensity, Young's modulus, and second moment of area, it computes the vertical reaction force, the hogging bending moment at the fixed end, and the displacement and rotation at the tip. The resulting fixed-end moment is 9000 N*m, while the free end deflects by 4.05 and rotates through an angle of 0.0966866. These outputs are essential for verifying that the beam meets design requirements for strength and serviceability.

### Cantilever with a uniformly distributed load

Reaction, fixed-end moment, tip deflection and tip slope of a cantilever under a uniform load over its whole length.

Command line:

```console
$ mechanical-engineering solve cantilever_udl "L=3 m" "w=2 kN/m" "E=200 GPa" "I=1e-05 m^4"
R = 6000 N
M_fixed = 9000 N*m
tip_deflection_mm = 10.125
tip_slope_deg = 0.2578310078
  by: Cantilever with a uniformly distributed load  (R = wL, M = wL^2/2, delta_tip = wL^4/(8EI), theta_tip = wL^3/(6EI))
  L = 3 m
  w = 2000 N/m
  E = 2e+11 Pa
  I = 1e-05 m^4
  assumes: L, w, E, I > 0; Euler-Bernoulli beam, small deflections
```

Python:

```python
import mechanical_engineering_ai999x as m

a = m.solve("cantilever_udl", L="3 m", w="2 kN/m", E="200 GPa", I="1e-05 m^4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/mechanical-engineering-ai999x/#cap-cantilever_udl) and press **Compute in your browser**.

_What this means:_ This calculation determines the structural response of a cantilever beam subjected to a uniform load across its entire length. Given a length of 3 m, a load intensity of 2 kN/m, a Young's modulus of 200 GPa, and a second moment of area of 1e-05 m^4, the model computes the vertical reaction at the support as 6000 N, the hogging moment at the fixed end as 9000 N*m, the downward deflection at the free tip as 10.125, and the angular slope at the tip as 0.257831. These values indicate how much the beam resists bending and how far it displaces under the applied pressure.

### Fixed-fixed (built-in) beam with a point load

Reactions, fixed-end moments, the moment under the load and the deflection under the load of a beam built in at both ends carrying one point load.

Command line:

```console
$ mechanical-engineering solve fixed_fixed_beam_point_load "L=6 m" "P=30 kN" "a=2 m" "E=200 GPa" "I=5e-05 m^4"
RA = 22222.22222 N
RB = 7777.777778 N
MA = 26666.66667 N*m
MB = 13333.33333 N*m
M_load = 17777.77778 N*m
deflection_at_load_mm = 2.37037037
  by: Fixed-fixed (built-in) beam with a point load  (MA = Pab^2/L^2, MB = Pa^2b/L^2, RA = Pb^2(3a + b)/L^3, RB = Pa^2(a + 3b)/L^3, M_load = 2Pa^2b^2/L^3, delta_load = Pa^3b^3/(3EIL^3))
  L = 6 m
  P = 30000 N
  a = 2 m
  E = 2e+11 Pa
  I = 5e-05 m^4
  assumes: 0 < a < L; both ends fully fixed
```

Python:

```python
import mechanical_engineering_ai999x as m

a = m.solve("fixed_fixed_beam_point_load", L="6 m", P="30 kN", a="2 m", E="200 GPa", I="5e-05 m^4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/mechanical-engineering-ai999x/#cap-fixed_fixed_beam_point_load) and press **Compute in your browser**.

_What this means:_ This calculation determines the structural response of a beam built in at both ends with a single point load. Using the span, load magnitude, and position, it computes the reactions and internal moments. The resulting hogging moment at support A is 26666.7 N*m, while the sagging moment directly beneath the applied force is 17777.8 N*m. The vertical displacement at the load point is found to be 2.37037, reflecting the stiffness provided by the material and cross-section.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/mechanical-engineering-ai999x/blob/main/docs/mechanical_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/mechanical-engineering-ai999x/blob/main/docs/mechanical_engineering_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/mechanical-engineering-ai999x/blob/main/docs/mechanical_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/mechanical-engineering-ai999x/blob/main/docs/mechanical_engineering_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/mechanical-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified mechanical engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 312 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `mechanical-engineering` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** statics, beams and section properties, stress and strain, torsion, columns, fatigue, machine elements, vibrations and structural analysis.

## Model overview

| Property | Value |
|---|---|
| Package | `mechanical-engineering-ai999x` 1.5.0 (import `mechanical_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic mechanical engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 90 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 312 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `mechanical-engineering-ai999x "<question>"`; for the native program use `mechanical-engineering` (above). The outputs above were produced by running this exact code on mechanical-engineering-ai999x 1.5.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![Mohr's circle of a plane stress state, computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/principal-stresses-2d-mohr.svg)

_Mohr's circle for σx = 50 MPa, σy = -10 MPa, τxy = 40 MPa: the normal and shear stress on planes rotated by 0–180° (73 states computed by the package's stress transformation) trace the circle; the principal stresses it returned are σ1 = 70 MPa and σ2 = -30 MPa at θp = 26.5651°; marked: centre σ = 20 MPa, τmax = 50 MPa._

**Shear and bending-moment diagrams of a beam**

![Shear and bending-moment diagrams of a beam, computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/simply-supported-beam-point-loads-beam.svg)

_Shear-force and bending-moment diagrams of a simply supported beam of span 8 m with point loads 10, 20 kN at x = 2, 5 m, drawn from the reactions the package returned (RA = 15 kN, RB = 15 kN); marked: the maximum moment it returned, x = 5 m, M = 45 kN·m._

**A cantilever's deflected shape under a tip load**

![A cantilever's deflected shape under a tip load, computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/cantilever-point-load-beam.svg)

_Deflected shape of a cantilever (L = 2 m, E = 2.0000e+11 Pa, I = 8.0000e-06 m⁴) under a tip load P = 5000 N: the deflection at each of 40 positions x is the tip deflection the package returns for the load placed at x (Maxwell's reciprocal theorem). Marked: x = 2 m, tip deflection = 8.33333 mm (downward); the wall and beam depth are drawn for orientation._

**Fatigue diagram: Goodman, Gerber and Soderberg envelopes**

![Fatigue diagram: Goodman, Gerber and Soderberg envelopes, computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/goodman-fatigue-safety-factor-fatigue.svg)

_Fatigue failure envelopes for Se = 250 MPa, Sut = 700 MPa, Sy = 500 MPa: each point is the stress state at which the package's safety factor along a load line equals 1 (47 load lines per criterion). Marked: the operating point σm = 80 MPa, σa = 120 MPa, Goodman safety factor n = 1.68269._

**Forced vibration: resonance curves for four dampings**

![Forced vibration: resonance curves for four dampings, computed by mechanical-engineering-ai999x](https://huggingface.co/AI999X/mechanical-engineering-ai999x/resolve/main/figures/forced-vibration-amplitude-response.svg)

_Steady-state magnification of a forced spring–mass–damper (m = 10 kg, k = 4000 N/m, natural frequency 20 rad/s) for four damping coefficients, 120 forcing frequencies each computed by the package; marked: ω = 20 rad/s, c = 40 N·s/m, magnification = 5._

## Capabilities

90 certified procedures. A few of them:

- **Procedures:** All critical (whirling) speeds of a shaft with several disks; Angle of twist of a stepped shaft; Bar fixed at both ends with an axial load at a step; Belt or rope friction on a drum (capstan equation); Bolt tightening torque and preload (T = K F d); Cantilever with a point load; Cantilever with a triangular (linearly varying) load; Cantilever with a uniformly distributed load; Combined axial force and biaxial bending of a rectangular section; Composite section of rectangles (parallel-axis theorem); Continuous beam under uniform loads (three-moment equation); Critical speed of a shaft carrying one disk; Critical stress of an intermediate column (Johnson parabola / Euler); Damping ratio, damped frequency and logarithmic decrement; Eccentrically loaded bolt group (elastic method); Eccentrically loaded column (secant formula); Endurance limit with the Marin factors; Euler buckling load of a column with given end conditions; Fatigue damage of a stress history (rainflow counting + Miner's rule); Fatigue damage on the trilinear (Eurocode 3) S-N curve; Fatigue life from the S-N line (10^3 to 10^6 cycles); Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law; Fatigue safety factor by the ASME-elliptic criterion; Fatigue safety factor by the Gerber criterion …

`mechanical_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified mechanical engineering capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (statics, beams and section properties, stress and strain, torsion, columns, fatigue, machine elements, vibrations and structural analysis); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{mechanical_engineering_ai999x_1_5_0,
  title  = {mechanical-engineering-ai999x: an offline, verified mechanical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.5.0},
  url    = {https://huggingface.co/AI999X/mechanical-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/mechanical-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

mechanical-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/mechanical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.5.0`; the [GitHub release](https://github.com/AI999X/mechanical-engineering-ai999x/releases/tag/v1.5.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `mechanical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/mechanical-engineering`) and its data (`rust/models/mechanical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p mechanical-engineering-ai999x
./target/release/mechanical-engineering --help
cargo test -p mechanical-engineering-ai999x
```
