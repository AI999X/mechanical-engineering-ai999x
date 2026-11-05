# mechanical-engineering: the Mechanical Engineering program — user guide

**mechanical-engineering-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Mechanical Engineering covers statics, stress and strain, beams and frames, section properties, torsion, columns, fatigue, machine elements, vibrations and structural analysis. This guide is about `mechanical-engineering`, the model as a self-contained program for Windows and Linux: 90 capabilities in 13 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/mechanical-engineering-ai999x/); the Python package (`pip install mechanical-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Mechanical Engineering | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-mechanical-engineering-1.4.0-windows-x86_64.zip |
| Desktop app: Mechanical Engineering | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-mechanical-engineering-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Mechanical Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-mechanical-engineering-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-mechanical-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-mechanical-engineering-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-mechanical-engineering-1.4.0-linux-x86_64.tar.gz
./ai999x-mechanical-engineering-1.4.0-linux-x86_64/ai999x-mechanical-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `mechanical-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
mechanical-engineering --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
mechanical-engineering --version
```

### In your browser

Open https://www.ai999x.com/models/mechanical-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install mechanical-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ mechanical-engineering --version
mechanical-engineering-ai999x 1.0.3 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ mechanical-engineering search "simply supported beam with a point load"
simply_supported_beam_point_load     Simply supported beam with a point load  (beams / simply supported beams)
simply_supported_beam_point_loads    Simply supported beam with several point loads  (beams / simply supported beams)
simply_supported_beam_udl            Simply supported beam with a uniformly distributed load  (beams / simply supported beams)
fixed_fixed_beam_point_load          Fixed-fixed (built-in) beam with a point load  (beams / statically indeterminate beams)
cantilever_point_load                Cantilever with a point load  (beams / cantilevers)
propped_cantilever_point_load        Propped cantilever with a point load  (beams / statically indeterminate beams)
continuous_beam_three_moment         Continuous beam under uniform loads (three-moment equation)  (structural analysis / continuous beams)
axial_bar_segments                   Stepped bar under axial loads at its joints  (stress and strain / axial loading)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ mechanical-engineering list
angle_section_properties                 Section properties of an angle (L) section, with principal axes
asme_elliptic_fatigue_safety_factor      Fatigue safety factor by the ASME-elliptic criterion
axial_bar_segments                       Stepped bar under axial loads at its joints
bar_fixed_at_both_ends                   Bar fixed at both ends with an axial load at a step
beam_natural_frequency                   Natural frequencies of a uniform beam (Euler-Bernoulli)
bearing_catalog_rating                   Required catalogue rating of a rolling bearing
bearing_life_reliability                 Rolling-bearing life at a given reliability
belt_drive_power                         Power of a belt drive with centrifugal tension
… 82 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ mechanical-engineering solve simply_supported_beam_point_load "L=6 m" "P=20 kN" "a=2 m" "E=200 GPa" "I=8e-05 m^4" --json
{"outcome":"SOLVED","message":"","value":26666.6666667,"unit":"N*m","solved_for":"M_max","si":[["L",6.0],["P",20000.0],["a",2.0],["E",200000000000.0],["I",0.00008]],"values":[["RA",13333.3333333,"N"],["RB",6666.66666667,"N"],["M_max",26666.6666667,"N*m"],["V_max",13333.3333333,"N"],["deflection_at_load_mm",4.44444444444,""],["max_deflection_mm",4.83849825735,""],["x_max",2.73401367629,"m"]],"target":"M_max","phrase":"reaction at the left support A","assumptions":["0 < a < L","Euler-Bernoulli beam, linear elastic, prismatic"],"name":"Simply supported beam with a point load","equation":"RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ mechanical-engineering solve simply_supported_beam_point_load "L=6 m" "P=20 kN" "a=2 m" "E=200 GPa"
NEEDS_INPUT: simply_supported_beam_point_load needs I (second moment of area of the section). Example: solve('simply_supported_beam_point_load', L='6 m', P='20000 N', a='2 m', E='2e+11 Pa', I='8e-05 m^4').
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ mechanical-engineering figures simply_supported_beam_point_load "L=6 m" "P=20 kN" "a=2 m" "E=200 GPa" "I=8e-05 m^4" --out .
[
  {
    "slug": "simply-supported-beam-point-load-beam",
    "title": "Simply supported beam with a point load",
    "caption": "Simply supported beam, span L = 6 m, point load P = 20 kN at a = 2 m, drawn to scale with its shear-force and bending-moment diagrams, derived by statics from the reactions the model returned. Marked: RA = 13.33 kN, RB = 6.667 kN, Mmax = 26.67 kN·m, maximum deflection 4.838 mm. In SI units: M_max = 2.667e+04 N*m. Your inputs: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4.",
    "file": ".\\simply-supported-beam-point-load-beam.svg"
  },
  {
    "slug": "simply-supported-beam-point-load-live-1",
    "title": "Maximum bending moment (under the load) as span varies",
    "caption": "Maximum bending moment (under the load) as L runs from 2.175 to 10.5 m, every other input as you entered it (38 solutions). Marked: your L = 6 m gives M_max = 2.667e+04. Your inputs: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4.",
    "file": ".\\simply-supported-beam-point-load-live-1.svg"
  },
  {
    "slug": "simply-supported-beam-point-load-live-2",
    "title": "Maximum bending moment (under the load) as distance of the load from the left support A varies",
    "caption": "Maximum bending moment (under the load) as a runs from 0.5 to 3.5 m, every other input as you entered it (29 solutions). Marked: your a = 2 m gives M_max = 2.667e+04. Your inputs: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4.",
    "file": ".\\simply-supported-beam-point-load-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | mechanical-engineering batch

`cases.jsonl`:

```json
{"key": "simply_supported_beam_point_load", "inputs": {"L": "6 m", "P": "20 kN", "a": "2 m", "E": "200 GPa", "I": "8e-05 m^4"}}
{"key": "principal_stresses_2d", "inputs": {"sx": "50 MPa", "sy": "-10 MPa", "txy": "40 MPa"}}
```

```shell
$ mechanical-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":26666.6666667,"unit":"N*m","solved_for":"M_max","si":[["L",6.0],["P",20000.0],["a",2.0],["E",200000000000.0],["I",0.00008]],"values":[["RA",13333.3333333,"N"],["RB",6666.66666667,"N"],["M_max",26666.6666667,"N*m"],["V_max",13333.3333333,"N"],["deflection_at_load_mm",4.44444444444,""],["max_deflection_mm",4.83849825735,""],["x_max",2.73401367629,"m"]],"target":"M_max","phrase":"reaction at the left support A","assumptions":["0 < a < L","Euler-Bernoulli beam, linear elastic, prismatic"],"name":"Simply supported beam with a point load","equation":"RA = Pb/L, RB = Pa/L, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb(L^2-b^2)^(3/2)/(9 sqrt(3) EIL) at x = sqrt((L^2-b^2)/3) (a >= b; mirrored otherwise)"}
{"outcome":"SOLVED","message":"","value":70.0,"unit":"","solved_for":"s1_MPa","si":[["sx",50000000.0],["sy",-10000000.0],["txy",40000000.0]],"values":[["s1_MPa",70.0,""],["s2_MPa",-30.0,""],["tau_max_inplane_MPa",50.0,""],["theta_p_deg",26.5650511771,""],["tau_abs_max_MPa",50.0,""],["von_mises_MPa",88.8819441732,""]],"target":"s1_MPa","phrase":"larger in-plane principal stress, MPa","assumptions":["plane stress (sigma_z = 0)","tension positive"],"name":"Principal stresses and maximum shear (Mohr's circle, plane stress)","equation":"sigma_1,2 = (sx + sy)/2 +- R, R = sqrt(((sx - sy)/2)^2 + txy^2); tan 2theta_p = 2 txy/(sx - sy); tau_abs = max(s1 - s2, |s1|, |s2|)/2"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `mechanical-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`mechanical-engineering solve ... --json` prints one answer as one line of JSON. `mechanical-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | mechanical-engineering batch`. `mechanical-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["mechanical-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`mechanical-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ mechanical-engineering figures simply_supported_beam_point_load "L=6 m" "P=20 kN" "a=2 m" "E=200 GPa" "I=8e-05 m^4" --out .
[
  {
    "slug": "simply-supported-beam-point-load-beam",
    "title": "Simply supported beam with a point load",
    "caption": "Simply supported beam, span L = 6 m, point load P = 20 kN at a = 2 m, drawn to scale with its shear-force and bending-moment diagrams, derived by statics from the reactions the model returned. Marked: RA = 13.33 kN, RB = 6.667 kN, Mmax = 26.67 kN·m, maximum deflection 4.838 mm. In SI units: M_max = 2.667e+04 N*m. Your inputs: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4.",
    "file": ".\\simply-supported-beam-point-load-beam.svg"
  },
  {
    "slug": "simply-supported-beam-point-load-live-1",
    "title": "Maximum bending moment (under the load) as span varies",
    "caption": "Maximum bending moment (under the load) as L runs from 2.175 to 10.5 m, every other input as you entered it (38 solutions). Marked: your L = 6 m gives M_max = 2.667e+04. Your inputs: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4.",
    "file": ".\\simply-supported-beam-point-load-live-1.svg"
  },
  {
    "slug": "simply-supported-beam-point-load-live-2",
    "title": "Maximum bending moment (under the load) as distance of the load from the left support A varies",
    "caption": "Maximum bending moment (under the load) as a runs from 0.5 to 3.5 m, every other input as you entered it (29 solutions). Marked: your a = 2 m gives M_max = 2.667e+04. Your inputs: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-05 m^4.",
    "file": ".\\simply-supported-beam-point-load-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `mechanical_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/mechanical_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("mechanical_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `mechanical-engineering info <id>` shows the same inputs and outputs in the terminal.

### Beams (10)

<a id="cantilever_point_load"></a>

#### Cantilever with a point load — `cantilever_point_load`

Reaction, fixed-end moment, deflection under the load, tip deflection and tip slope of a cantilever carrying one point load at distance a from the fixed end (default: at the free end).

$$
\delta_{tip} = \frac{Pa^2(3L-a)}{6EI},\quad \theta_{tip} = \frac{Pa^2}{2EI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | length of the cantilever | m |
| `P` | in | point load (downward) | N |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `a` | in | distance of the load from the fixed end (default L: at the free end) (optional) | m |
| `R` | out | vertical reaction at the fixed end | N |
| `M_fixed` | out | hogging moment at the fixed end | N*m |
| `deflection_at_load_mm` | out | deflection under the load in mm | — |
| `tip_deflection_mm` | out | deflection of the free end in mm | — |
| `tip_slope_deg` | out | slope at the free end in degrees | — |

**Applies when:** 0 < a <= L; Euler-Bernoulli beam, small deflections.

```shell
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

**What the worked example means:** The cantilever deflects 8.33333 under the load and 8.33333 at the tip, with a slope of 0.358099.

<a id="cantilever_triangular_load"></a>

#### Cantilever with a triangular (linearly varying) load — `cantilever_triangular_load`

Reaction, fixed-end moment, tip deflection and tip slope of a cantilever whose load falls linearly from w at the fixed end to zero at the free end.

$$
\delta_{tip} = \frac{w_0L^4}{30EI},\quad \theta_{tip} = \frac{w_0L^3}{24EI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | length of the cantilever | m |
| `w` | in | load intensity at the fixed end (downward) | N/m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `R` | out | vertical reaction at the fixed end | N |
| `M_fixed` | out | hogging moment at the fixed end | N*m |
| `tip_deflection_mm` | out | deflection of the free end in mm | — |
| `tip_slope_deg` | out | slope at the free end in degrees | — |

**Applies when:** L, w, E, I > 0; maximum intensity at the fixed end; Euler-Bernoulli beam.

```shell
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

**What the worked example means:** This calculation determines the structural response of a cantilever beam subjected to a triangular load that decreases linearly from maximum intensity at the fixed support to zero at the free end. Using the input values for length, load intensity, Young's modulus, and second moment of area, it computes the vertical reaction force, the hogging bending moment at the fixed end, and the displacement and rotation at the tip. The resulting fixed-end moment is 9000 N*m, while the free end deflects by 4.05 and rotates through an angle of 0.0966866. These outputs are essential for verifying that the beam meets design requirements for strength and serviceability.

<a id="cantilever_udl"></a>

#### Cantilever with a uniformly distributed load — `cantilever_udl`

Reaction, fixed-end moment, tip deflection and tip slope of a cantilever under a uniform load over its whole length.

$$
\delta_{tip} = \frac{wL^4}{8EI},\quad \theta_{tip} = \frac{wL^3}{6EI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | length of the cantilever | m |
| `w` | in | uniform load per unit length (downward) | N/m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `R` | out | vertical reaction at the fixed end | N |
| `M_fixed` | out | hogging moment at the fixed end | N*m |
| `tip_deflection_mm` | out | deflection of the free end in mm | — |
| `tip_slope_deg` | out | slope at the free end in degrees | — |

**Applies when:** L, w, E, I > 0; Euler-Bernoulli beam, small deflections.

```shell
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

**What the worked example means:** This calculation determines the structural response of a cantilever beam subjected to a uniform load across its entire length. Given a length of 3 m, a load intensity of 2 kN/m, a Young's modulus of 200 GPa, and a second moment of area of 1e-05 m^4, the model computes the vertical reaction at the support as 6000 N, the hogging moment at the fixed end as 9000 N*m, the downward deflection at the free tip as 10.125, and the angular slope at the tip as 0.257831. These values indicate how much the beam resists bending and how far it displaces under the applied pressure.

<a id="fixed_fixed_beam_point_load"></a>

#### Fixed-fixed (built-in) beam with a point load — `fixed_fixed_beam_point_load`

Reactions, fixed-end moments, the moment under the load and the deflection under the load of a beam built in at both ends carrying one point load.

$$
M_A = \frac{Pab^2}{L^2},\quad M_B = \frac{Pa^2b}{L^2},\quad \delta_P = \frac{Pa^3b^3}{3EIL^3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `P` | in | point load (downward) | N |
| `a` | in | distance of the load from end A | m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `RA` | out | reaction at A | N |
| `RB` | out | reaction at B | N |
| `MA` | out | hogging moment at A | N*m |
| `MB` | out | hogging moment at B | N*m |
| `M_load` | out | sagging moment under the load | N*m |
| `deflection_at_load_mm` | out | deflection under the load in mm | — |

**Applies when:** 0 < a < L; both ends fully fixed.

```shell
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

**What the worked example means:** This calculation determines the structural response of a beam built in at both ends with a single point load. Using the span, load magnitude, and position, it computes the reactions and internal moments. The resulting hogging moment at support A is 26666.7 N*m, while the sagging moment directly beneath the applied force is 17777.8 N*m. The vertical displacement at the load point is found to be 2.37037, reflecting the stiffness provided by the material and cross-section.

<a id="fixed_fixed_beam_udl"></a>

#### Fixed-fixed (built-in) beam with a uniformly distributed load — `fixed_fixed_beam_udl`

Reactions, fixed-end (hogging) moments, midspan (sagging) moment and maximum deflection of a beam built in at both ends under a uniform load.

$$
M_{end} = \frac{wL^2}{12},\quad M_{mid} = \frac{wL^2}{24},\quad \delta_{max} = \frac{wL^4}{384EI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `w` | in | uniform load per unit length (downward) | N/m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `RA` | out | reaction at A | N |
| `RB` | out | reaction at B | N |
| `M_end` | out | hogging moment at each fixed end | N*m |
| `M_mid` | out | sagging moment at midspan | N*m |
| `max_deflection_mm` | out | maximum (midspan) deflection in mm | — |

**Applies when:** L, w, E, I > 0; both ends fully fixed against rotation and deflection.

```shell
$ mechanical-engineering solve fixed_fixed_beam_udl "L=6 m" "w=12 kN/m" "E=200 GPa" "I=6e-05 m^4"
RA = 36000 N
RB = 36000 N
M_end = 36000 N*m
M_mid = 18000 N*m
max_deflection_mm = 3.375
  by: Fixed-fixed (built-in) beam with a uniformly distributed load  (R = wL/2, M_end = wL^2/12, M_mid = wL^2/24, delta_max = wL^4/(384EI))
  L = 6 m
  w = 12000 N/m
  E = 2e+11 Pa
  I = 6e-05 m^4
  assumes: L, w, E, I > 0; both ends fully fixed against rotation and deflection
```

**What the worked example means:** This calculation determines the structural response of a beam rigidly supported at both ends carrying a uniform load. The vertical reactions at each support are computed as 36000 N and 36000 N, reflecting the symmetry of the loading. The bending moments include a hogging value of 36000 N*m at the supports and a sagging value of 18000 N*m at the center, while the maximum deflection is recorded as 3.375. These results highlight how full rotational restraint significantly reduces central displacement compared to simply supported beams but introduces substantial negative moments at the connections.

<a id="propped_cantilever_point_load"></a>

#### Propped cantilever with a point load — `propped_cantilever_point_load`

Reactions, fixed-end moment, moment under the load and deflection under the load of a beam fixed at A and simply supported at B with one point load.

$$
R_B = \frac{Pa^2(3L-a)}{2L^3},\quad M_A = \frac{Pab(L+b)}{2L^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `P` | in | point load (downward) | N |
| `a` | in | distance of the load from the fixed end A | m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `RA` | out | reaction at the fixed end A | N |
| `RB` | out | reaction at the prop B | N |
| `MA` | out | hogging moment at A | N*m |
| `M_load` | out | sagging moment under the load | N*m |
| `deflection_at_load_mm` | out | deflection under the load in mm | — |

**Applies when:** 0 < a < L; A fully fixed, B a simple support.

```shell
$ mechanical-engineering solve propped_cantilever_point_load "L=4 m" "P=16 kN" "a=2 m" "E=200 GPa" "I=2e-05 m^4"
RA = 11000 N
RB = 5000 N
MA = 12000 N*m
M_load = 10000 N*m
deflection_at_load_mm = 2.333333333
  by: Propped cantilever with a point load  (RB = Pa^2(3L - a)/(2L^3), RA = P - RB, MA = Pab(L + b)/(2L^2), M_load = RB b, delta_load = Pa^3b^2(3L + b)/(12EIL^3))
  L = 4 m
  P = 16000 N
  a = 2 m
  E = 2e+11 Pa
  I = 2e-05 m^4
  assumes: 0 < a < L; A fully fixed, B a simple support
```

**What the worked example means:** This calculation determines the support reactions, internal bending moments, and deflection for a beam fixed at one end and simply supported at the other, subjected to a point load P at distance a from the fixed support. Using the span L, Young's modulus E, and second moment of area I, it computes the vertical reaction at the prop RB as 5000 N and the hogging moment at the fixed end MA as 12000 N*m. The resulting values indicate how the redundant support shares the load and resists rotation, which is critical for assessing structural safety and serviceability limits under linear elastic conditions.

<a id="propped_cantilever_udl"></a>

#### Propped cantilever with a uniformly distributed load — `propped_cantilever_udl`

Reactions, fixed-end moment, maximum span moment and its position, and the maximum deflection with its position, for a beam fixed at A and simply supported at B under a uniform load.

$$
R_B = \frac{3wL}{8},\quad M_A = \frac{wL^2}{8},\quad \delta_{max} \approx \frac{wL^4}{185EI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `w` | in | uniform load per unit length (downward) | N/m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `RA` | out | reaction at the fixed end A | N |
| `RB` | out | reaction at the prop B | N |
| `MA` | out | hogging moment at A | N*m |
| `M_span` | out | largest sagging moment in the span | N*m |
| `x_M_span` | out | its position from A | m |
| `max_deflection_mm` | out | maximum deflection in mm | — |
| `x_max` | out | position of the maximum deflection from A | m |

**Applies when:** L, w, E, I > 0; A fully fixed, B a simple support.

```shell
$ mechanical-engineering solve propped_cantilever_udl "L=8 m" "w=10 kN/m" "E=200 GPa" "I=0.0001 m^4"
RA = 50000 N
RB = 30000 N
MA = 80000 N*m
M_span = 45000 N*m
x_M_span = 5 m
max_deflection_mm = 11.09221705
x_max = 4.627718677 m
  by: Propped cantilever with a uniformly distributed load  (RB = 3wL/8, RA = 5wL/8, MA = wL^2/8, M_span = 9wL^2/128 at 5L/8, delta_max = (39 + 55 sqrt(33)) wL^4/(65536 EI) at x = L(15 - sqrt(33))/16)
  L = 8 m
  w = 10000 N/m
  E = 2e+11 Pa
  I = 0.0001 m^4
  assumes: L, w, E, I > 0; A fully fixed, B a simple support
```

**What the worked example means:** This calculation determines the structural response of a beam fixed at one end and simply supported at the other, spanning 8 m with a uniform load of 10 kN/m. The reactions are 50000 N at the fixed support and 30000 N at the prop. The hogging moment at the fixed end is 80000 N*m, while the largest sagging moment in the span is 45000 N*m, occurring at 5 m. The beam deflects downward by a maximum of 11.0922 at position 4.62772 m. These values help assess bending stresses and serviceability limits.

<a id="simply_supported_beam_point_load"></a>

#### Simply supported beam with a point load — `simply_supported_beam_point_load`

Reactions, maximum bending moment and shear, deflection under the load and the maximum deflection with its position, for a simply supported beam carrying one point load.

$$
M_{max} = \frac{Pab}{L},\quad \delta_{max} = \frac{Pb\,(L^2-b^2)^{3/2}}{9\sqrt3\,EIL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `P` | in | point load (downward) | N |
| `a` | in | distance of the load from the left support A | m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area of the section | m^4 |
| `RA` | out | reaction at the left support A | N |
| `RB` | out | reaction at the right support B | N |
| `M_max` | out | maximum bending moment (under the load) | N*m |
| `V_max` | out | maximum shear force | N |
| `deflection_at_load_mm` | out | deflection under the load in mm | — |
| `max_deflection_mm` | out | maximum deflection in mm | — |
| `x_max` | out | position of the maximum deflection from A | m |

**Applies when:** 0 < a < L; Euler-Bernoulli beam, linear elastic, prismatic.

```shell
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

**What the worked example means:** This calculation determines the structural response of a simply supported beam carrying a single concentrated load. Using the span, load magnitude, load position, material stiffness, and section geometry, it computes the support reactions, the peak internal bending moment, and the resulting vertical displacements. The key results are a maximum bending moment of 26666.7 N*m and a deflection directly beneath the applied force of 4.44444, which help assess if the beam can safely carry the intended weight without excessive sagging or failure.

<a id="simply_supported_beam_udl"></a>

#### Simply supported beam with a uniformly distributed load — `simply_supported_beam_udl`

Reactions, maximum bending moment and shear, maximum (midspan) deflection and end slope of a simply supported beam under a uniform load over its whole span.

$$
M_{max} = \frac{wL^2}{8},\quad \delta_{max} = \frac{5wL^4}{384EI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `w` | in | uniform load per unit length (downward) | N/m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area of the section | m^4 |
| `RA` | out | reaction at A | N |
| `RB` | out | reaction at B | N |
| `M_max` | out | maximum bending moment (midspan) | N*m |
| `V_max` | out | maximum shear force (at the supports) | N |
| `max_deflection_mm` | out | maximum deflection (midspan) in mm | — |
| `end_slope_deg` | out | slope at the supports in degrees | — |

**Applies when:** L, w, E, I > 0; Euler-Bernoulli beam, small deflections.

```shell
$ mechanical-engineering solve simply_supported_beam_udl "L=5 m" "w=10 kN/m" "E=200 GPa" "I=4e-05 m^4"
RA = 25000 N
RB = 25000 N
M_max = 31250 N*m
V_max = 25000 N
max_deflection_mm = 10.17252604
end_slope_deg = 0.3730193979
  by: Simply supported beam with a uniformly distributed load  (R = wL/2, M_max = wL^2/8, delta_max = 5wL^4/(384EI), theta_end = wL^3/(24EI))
  L = 5 m
  w = 10000 N/m
  E = 2e+11 Pa
  I = 4e-05 m^4
  assumes: L, w, E, I > 0; Euler-Bernoulli beam, small deflections
```

**What the worked example means:** This calculation determines the structural response of a simply supported beam carrying a uniform load across its entire length. Given a span of 5 m and a load intensity of 10 kN/m, the support reactions are each 25000 N. The internal forces peak at the supports, where the shear force reaches 25000 N, and at midspan, where the bending moment hits 31250 N*m. The resulting deformation shows a maximum downward displacement of 10.1725 at the center, accompanied by an end slope of 0.373019 degrees at the supports.

<a id="simply_supported_beam_point_loads"></a>

#### Simply supported beam with several point loads — `simply_supported_beam_point_loads`

Support reactions, the maximum bending moment and its position, and the maximum shear for a simply supported beam carrying several downward point loads.

$$
R_B = \frac{\sum P_i x_i}{L},\quad R_A = \sum P_i - R_B,\quad M(x) = R_A x - \sum_{x_i<x} P_i(x-x_i)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | span | m |
| `x` | in | load positions from the left support A, in m (a list) | — |
| `P` | in | point loads in kN, downward (a list, same order as x) | — |
| `RA` | out | reaction at A in kN | — |
| `RB` | out | reaction at B in kN | — |
| `M_max` | out | maximum bending moment in kN*m | — |
| `x_Mmax` | out | position of the maximum moment from A, in m (the first one if several are equal) | — |
| `V_max` | out | maximum shear force in kN | — |

**Applies when:** 0 <= x_i <= L; P_i > 0 (downward); x and P of equal length.

```shell
$ mechanical-engineering solve simply_supported_beam_point_loads "L=8 m" "x=[2, 5]" "P=[10, 20]"
RA = 15
RB = 15
M_max = 45
x_Mmax = 5
V_max = 15
  by: Simply supported beam with several point loads  (moments about A: RB = sum P_i x_i / L, RA = sum P_i - RB; M(x) = RA x - sum P_i (x - x_i) for x_i < x, largest at a load point)
  L = 8 m
  assumes: 0 <= x_i <= L; P_i > 0 (downward); x and P of equal length
```

**What the worked example means:** This calculation determines the support reactions and internal forces for a simply supported beam carrying multiple downward point loads. Summing moments about one support gives the reaction at the other support; vertical equilibrium then gives the first reaction. The bending moment is computed along the span by considering the forces to the left of any section, identifying the peak value as 45 located at 5. The maximum shear force is also reported as 15, which typically occurs near the supports where the load transfer is most abrupt.

### Columns (3)

<a id="johnson_column"></a>

#### Critical stress of an intermediate column (Johnson parabola / Euler) — `johnson_column`

The critical stress of a column by the Johnson parabola below the transition slenderness and by Euler above it, with the transition slenderness and (with the area) the critical load.

$$
\sigma_{cr} = S_y - \frac{1}{E}\Big(\frac{S_y}{2\pi}\frac{KL}{r}\Big)^2 \quad (KL/r < C_c),\quad C_c = \sqrt{\frac{2\pi^2E}{S_y}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sy` | in | yield strength | Pa |
| `E` | in | Young's modulus | Pa |
| `L` | in | column length | m |
| `r` | in | least radius of gyration | m |
| `ends` | in | end conditions (theoretical K) | — |
| `A` | in | cross-sectional area (optional: gives the load) (optional) | m^2 |
| `slenderness` | out | slenderness ratio K L / r | — |
| `transition_slenderness` | out | transition slenderness C_c = sqrt(2 pi^2 E / Sy) | — |
| `regime` | out | Johnson or Euler | — |
| `sigma_cr_MPa` | out | critical stress, MPa | — |
| `P_cr_kN` | out | critical load in kN (with A) | — |

**Applies when:** centric load; theoretical effective length factors.

```shell
$ mechanical-engineering solve johnson_column "Sy=250 MPa" "E=200 GPa" "L=1.5 m" "r=20 mm" ends=pinned-pinned
slenderness = 75
transition_slenderness = 125.6637061
regime = Johnson
sigma_cr_MPa = 205.4740892
  by: Critical stress of an intermediate column (Johnson parabola / Euler)  (C_c = sqrt(2 pi^2 E/Sy); KL/r < C_c: sigma_cr = Sy - (Sy KL/(2 pi r))^2/E; otherwise sigma_cr = pi^2 E/(KL/r)^2)
  Sy = 250000000 Pa
  E = 2e+11 Pa
  L = 1.5 m
  r = 0.02 m
  ends = pinned-pinned
  assumes: centric load; theoretical effective length factors
```

**What the worked example means:** The critical stress is 205.474 because the slenderness ratio of 75 places the column in the Johnson regime.

<a id="secant_column"></a>

#### Eccentrically loaded column (secant formula) — `secant_column`

The maximum compressive stress, midspan deflection and largest bending moment of a pinned column under a load with equal end eccentricities.

$$
\sigma_{max} = \frac{P}{A}\Big[1 + \frac{ec}{r^2}\sec\Big(\frac{L}{2}\sqrt{\frac{P}{EI}}\Big)\Big]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | compressive load | N |
| `e` | in | eccentricity of the load | m |
| `L` | in | length (pinned ends) | m |
| `A` | in | area | m^2 |
| `I` | in | second moment of area in the bending plane | m^4 |
| `c` | in | distance from the centroid to the extreme fibre | m |
| `E` | in | Young's modulus | Pa |
| `sigma_max_MPa` | out | maximum compressive stress (magnitude), MPa | — |
| `deflection_mm` | out | midspan lateral deflection, mm | — |
| `M_max` | out | largest bending moment P (e + deflection) | N*m |
| `P_over_Pcr` | out | P / P_Euler | — |

**Applies when:** pinned ends, equal eccentricities on the same side; P below the Euler load; elastic.

```shell
$ mechanical-engineering solve secant_column "P=200 kN" "e=10 mm" "L=3 m" "A=4000 mm^2" "I=1.2e-05 m^4" "c=75 mm" "E=200 GPa"
sigma_max_MPa = 63.77098297
deflection_mm = 1.016786377
M_max = 2203.357275 N*m
P_over_Pcr = 0.07599088773
  by: Eccentrically loaded column (secant formula)  (sigma_max = P/A [1 + (e c/r^2) sec(L/2 sqrt(P/(EI)))], deflection = e [sec(kL/2) - 1])
  P = 200000 N
  e = 0.01 m
  L = 3 m
  A = 0.004 m^2
  I = 1.2e-05 m^4
  c = 0.075 m
  E = 2e+11 Pa
  assumes: pinned ends, equal eccentricities on the same side; P below the Euler load; elastic
```

**What the worked example means:** This calculation determines the response of an eccentrically loaded pinned column using the secant formula. Given a compressive load P of 200 kN, eccentricity e of 10 mm, length L of 3 m, area A of 4000 mm^2, second moment of area I of 1.2e-05 m^4, extreme fiber distance c of 75 mm, and Young's modulus E of 200 GPa, the model computes the resulting stresses and deformations. The output shows a maximum compressive stress of 63.771, which accounts for both direct compression and bending amplified by lateral displacement. The midspan deflection is 1.01679, leading to a largest bending moment of 2203.36 N*m. The ratio of the applied load to the Euler buckling load is 0.0759909, indicating how close the column is to instability.

<a id="euler_buckling"></a>

#### Euler buckling load of a column with given end conditions — `euler_buckling`

The critical (Euler) buckling load of an ideal elastic column, its effective length factor and length, and with the area also the critical stress and slenderness ratio.

$$
P_{cr} = \frac{\pi^2EI}{(KL)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | Young's modulus | Pa |
| `I` | in | least second moment of area | m^4 |
| `L` | in | column length | m |
| `ends` | in | end conditions: pinned-pinned, fixed-free, fixed-fixed or fixed-pinned | — |
| `A` | in | cross-sectional area (optional: gives the stress and slenderness) (optional) | m^2 |
| `P_cr` | out | critical buckling load | N |
| `P_cr_kN` | out | critical load in kN | — |
| `K` | out | effective length factor (theoretical) | — |
| `Le` | out | effective length K L | m |
| `sigma_cr_MPa` | out | critical stress P_cr/A, MPa (with A) | — |
| `slenderness` | out | slenderness ratio K L / r (with A) | — |

**Applies when:** ideal straight elastic column, centric load; theoretical K: 1, 2, 0.5, 0.6992 (fixed-pinned, from tan x = x); valid only below the proportional limit.

```shell
$ mechanical-engineering solve euler_buckling "E=200 GPa" "I=8e-06 m^4" "L=4 m" ends=pinned-pinned
P_cr = 986960.4401 N
P_cr_kN = 986.9604401
K = 1
Le = 4 m
  by: Euler buckling load of a column with given end conditions  (P_cr = pi^2 E I/(K L)^2)
  E = 2e+11 Pa
  I = 8e-06 m^4
  L = 4 m
  ends = pinned-pinned
  assumes: ideal straight elastic column, centric load; theoretical K: 1, 2, 0.5, 0.6992 (fixed-pinned, from tan x = x); valid only below the proportional limit
```

In the worked example, E = 200 GPa, I = 8e-06 m^4, L = 4 m, ends = pinned-pinned go through the equation above and give P_cr = 986960.4401 N, P_cr_kN = 986.9604401, K = 1, Le = 4 m. It applies only when ideal straight elastic column, centric load; theoretical K: 1, 2, 0.5, 0.6992 (fixed-pinned, from tan x = x); valid only below the proportional limit: outside that it says what is wrong instead of returning a number.

### Failure theories (2)

<a id="brittle_failure_mohr"></a>

#### Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr) — `brittle_failure_mohr`

Factors of safety of a brittle material under plane stress by the maximum-normal-stress, brittle Coulomb-Mohr and modified-Mohr criteria (proportional loading).

$$
\frac{\sigma_1}{S_{ut}} - \frac{\sigma_3}{S_{uc}} = \frac1n,\quad \frac{(S_{uc}-S_{ut})\sigma_A}{S_{uc}S_{ut}} - \frac{\sigma_B}{S_{uc}} = \frac1n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sA` | in | first in-plane principal stress | Pa |
| `sB` | in | second in-plane principal stress | Pa |
| `Sut` | in | ultimate tensile strength | Pa |
| `Suc` | in | ultimate compressive strength (positive number) | Pa |
| `n_max_normal` | out | safety factor, maximum normal stress theory | — |
| `n_coulomb_mohr` | out | safety factor, brittle Coulomb-Mohr | — |
| `n_modified_mohr` | out | safety factor, modified Mohr | — |

**Applies when:** plane stress (third principal stress zero); Sut, Suc > 0 (Suc given as a magnitude); proportional loading.

```shell
$ mechanical-engineering solve brittle_failure_mohr "sA=90 MPa" "sB=-50 MPa" "Sut=210 MPa" "Suc=750 MPa"
n_max_normal = 2.333333333
n_coulomb_mohr = 2.019230769
n_modified_mohr = 2.333333333
  by: Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr)  (BCM: 1/n = s1/Sut - s3/Suc (with s1 = max(sA, sB, 0), s3 = min(sA, sB, 0)); MM: 1/n = (Suc - Sut) sA/(Suc Sut) - sB/Suc when sA > 0 > sB and |sB| > sA; MNS: n = min(Sut/s1, Suc/|s3|))
  sA = 90000000 Pa
  sB = -50000000 Pa
  Sut = 210000000 Pa
  Suc = 750000000 Pa
  assumes: plane stress (third principal stress zero); Sut, Suc > 0 (Suc given as a magnitude); proportional loading
```

**What the worked example means:** This calculation determines the factor of safety for a brittle material under plane stress using three distinct failure theories. It uses the principal stresses 90 MPa and -50 MPa alongside the ultimate tensile strength 210 MPa and ultimate compressive strength 750 MPa. The resulting values, such as 2.01923, indicate how much the load can increase before failure is predicted by each specific criterion.

<a id="yield_criteria_safety_factor"></a>

#### Von Mises and Tresca equivalent stresses and safety factors — `yield_criteria_safety_factor`

Equivalent stresses by the distortion-energy (von Mises) and maximum-shear (Tresca) criteria and the corresponding factors of safety against yielding.

$$
\sigma_{vm} = \sqrt{\tfrac12\big[(\sigma_1-\sigma_2)^2+(\sigma_2-\sigma_3)^2+(\sigma_3-\sigma_1)^2\big]},\quad n = \frac{S_y}{\sigma_{eq}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sy` | in | yield strength | Pa |
| `sx` | in | sigma_x | Pa |
| `sy` | in | sigma_y (default 0) (optional) | Pa |
| `sz` | in | sigma_z (default 0) (optional) | Pa |
| `txy` | in | tau_xy (default 0) (optional) | Pa |
| `tyz` | in | tau_yz (default 0) (optional) | Pa |
| `tzx` | in | tau_zx (default 0) (optional) | Pa |
| `von_mises_MPa` | out | von Mises stress, MPa | — |
| `tresca_MPa` | out | Tresca equivalent stress (s1 - s3), MPa | — |
| `n_von_mises` | out | safety factor by distortion energy | — |
| `n_tresca` | out | safety factor by maximum shear | — |

**Applies when:** Sy > 0; not a purely hydrostatic stress; ductile, isotropic material.

```shell
$ mechanical-engineering solve yield_criteria_safety_factor "Sy=350 MPa" "sx=120 MPa" "sy=-40 MPa" "txy=60 MPa"
von_mises_MPa = 177.7638883
tresca_MPa = 200
n_von_mises = 1.968903827
n_tresca = 1.75
  by: Von Mises and Tresca equivalent stresses and safety factors  (sigma_vm = sqrt(((sx-sy)^2 + (sy-sz)^2 + (sz-sx)^2)/2 + 3(txy^2+tyz^2+tzx^2)), sigma_T = s1 - s3, n = Sy/sigma_eq)
  Sy = 350000000 Pa
  sx = 120000000 Pa
  sy = -40000000 Pa
  txy = 60000000 Pa
  assumes: Sy > 0; not a purely hydrostatic stress; ductile, isotropic material
```

**What the worked example means:** This calculation determines the equivalent stresses for a ductile material under a complex stress state using both the distortion energy theory and the maximum shear stress theory. The von Mises equivalent stress, computed as 177.764, represents the scalar value that compares the multiaxial loading to uniaxial yield conditions. Similarly, the Tresca equivalent stress, found to be 200, is derived from the difference between the principal stresses. These values are then used to calculate the factor of safety against yielding by dividing the material yield strength by each respective equivalent stress, resulting in safety factors of 1.9689 for von Mises and 1.75 for Tresca.

### Fatigue (14)

<a id="marin_endurance_limit"></a>

#### Endurance limit with the Marin factors — `marin_endurance_limit`

The corrected endurance limit Se = ka kb kc kd ke kf Se' of a steel part from its tensile strength, surface finish, size, loading and reliability (Se' = 0.5 Sut up to 1400 MPa, else 700 MPa).

$$
S_e = k_a k_b k_c k_d k_e k_f\,S_e',\quad k_a = aS_{ut}^b,\quad k_e = 1 - 0.08\,z_a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sut` | in | ultimate tensile strength | Pa |
| `surface` | in | surface finish (ground, machined, cold-drawn, hot-rolled, as-forged) | — |
| `load` | in | loading (bending, axial, torsion) | — |
| `d` | in | diameter (needed for bending and torsion, 2.79-254 mm) (optional) | m |
| `R` | in | reliability (default 0.5; written as 0.99 or 99%) (optional) | — |
| `kd` | in | temperature factor (default 1) (optional) | — |
| `kf` | in | miscellaneous-effects factor (default 1) (optional) | — |
| `ka` | out | surface factor a Sut^b | — |
| `kb` | out | size factor | — |
| `kc` | out | load factor | — |
| `ke` | out | reliability factor 1 - 0.08 z_a | — |
| `Se_prime_MPa` | out | uncorrected endurance limit, MPa | — |
| `Se_MPa` | out | corrected endurance limit, MPa | — |
| `Se` | out | corrected endurance limit | Pa |

**Applies when:** steels (Se' = 0.5 Sut, at most 700 MPa); size factor for 2.79 <= d <= 254 mm (rotating round sections); 0.5 <= R < 1.

```shell
$ mechanical-engineering solve marin_endurance_limit "Sut=690 MPa" surface=machined load=bending "d=32 mm" R=0.99
ka = 0.7977770394
kb = 0.8557967591
kc = 1
ke = 0.8138921701
Se_prime_MPa = 345
Se_MPa = 191.7070727
Se = 191707072.7 Pa
  by: Endurance limit with the Marin factors  (ka = a Sut^b (a, b by surface, Sut in MPa); kb = 1.24 d^-0.107 (d <= 51 mm) or 1.51 d^-0.157; kc = 1, 0.85, 0.59; ke = 1 - 0.08 z_a)
  Sut = 690000000 Pa
  d = 0.032 m
  R = 0.99
  surface = machined
  load = bending
  assumes: steels (Se' = 0.5 Sut, at most 700 MPa); size factor for 2.79 <= d <= 254 mm (rotating round sections); 0.5 <= R < 1
```

**What the worked example means:** This calculation determines the endurance limit for a steel component by applying Marin modification factors to the laboratory baseline. The process begins with the ultimate tensile strength of 690 MPa, which establishes the uncorrected endurance limit 345. Subsequent factors account for specific conditions: the surface factor 0.797777 reflects the finish quality, the size factor 0.855797 adjusts for the diameter 32 mm, and the load factor 1 considers the type of stress applied. Reliability is handled by the factor 0.813892, which decreases as the required reliability 0.99 increases. Multiplying these together yields the final corrected endurance limit 1.91707e+08 Pa, representing the maximum stress amplitude the part can withstand indefinitely without failure under the specified conditions.

<a id="rainflow_fatigue_damage"></a>

#### Fatigue damage of a stress history (rainflow counting + Miner's rule) — `rainflow_fatigue_damage`

The Miner damage of one pass of a stress history: rainflow cycles (ASTM, half cycles) on a single-slope S-N curve N = Nc (Sc/S)^m, and the passes to failure.

$$
D = \sum_i \frac{n_i}{N_c}\Big(\frac{S_i}{S_c}\Big)^m
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `history` | in | the stress history in MPa, point by point (a list) | — |
| `Sc` | in | reference stress range at Nc cycles, MPa | — |
| `m` | in | S-N slope | — |
| `Nc` | in | cycles at Sc (default 2e6) (optional) | — |
| `damage` | out | damage of one pass of the history | — |
| `repeats_to_failure` | out | passes until D = 1 | — |
| `cycles` | out | cycles counted per pass | — |

**Applies when:** ASTM E1049-85 rainflow counting, half cycles weighted 0.5; no mean-stress correction; single-slope S-N curve, no endurance limit.

```shell
$ mechanical-engineering solve rainflow_fatigue_damage "history=[0, 90, 0]" Sc=90 m=3
damage = 5e-07
repeats_to_failure = 2000000
cycles = 1
  by: Fatigue damage of a stress history (rainflow counting + Miner's rule)  (rainflow ranges S_i with counts n_i; D = sum n_i S_i^m/(Nc Sc^m))
  Sc = 90
  m = 3
  assumes: ASTM E1049-85 rainflow counting, half cycles weighted 0.5; no mean-stress correction; single-slope S-N curve, no endurance limit
```

**What the worked example means:** The calculated fatigue damage for one pass of the stress history is 5e-07, which implies that 2e+06 repeats are needed to reach failure.

<a id="eurocode_fatigue_damage"></a>

#### Fatigue damage on the trilinear (Eurocode 3) S-N curve — `eurocode_fatigue_damage`

The Miner damage of stress-range blocks on the trilinear S-N curve of a detail category (slope 3 to 5e6 cycles, slope 5 to 1e8, cut-off below), with the constant-amplitude and cut-off limits.

$$
N = 2\cdot10^6\Big(\frac{\Delta\sigma_C}{\Delta\sigma}\Big)^3\ (\Delta\sigma\ge\Delta\sigma_D),\quad N = 5\cdot10^6\Big(\frac{\Delta\sigma_D}{\Delta\sigma}\Big)^5
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ranges` | in | stress ranges in MPa (a list) | — |
| `counts` | in | cycles of each range (a list) | — |
| `dsc` | in | detail category: stress range at 2e6 cycles, MPa | — |
| `damage` | out | Miner damage sum | — |
| `delta_sigma_D` | out | constant-amplitude fatigue limit (5e6 cycles), MPa | — |
| `delta_sigma_L` | out | cut-off limit (1e8 cycles), MPa | — |
| `repeats_to_failure` | out | repeats of the spectrum until D = 1 | — |

**Applies when:** partial safety factors not applied (gamma_Mf = gamma_Ff = 1); at least one range above the cut-off.

```shell
$ mechanical-engineering solve eurocode_fatigue_damage "ranges=[90]" "counts=[1000000.0]" dsc=90
damage = 0.5
delta_sigma_D = 66.31256698
delta_sigma_L = 36.4241848
repeats_to_failure = 2
  by: Fatigue damage on the trilinear (Eurocode 3) S-N curve  (N = 2e6 (dsc/S)^3 for S >= sD = 0.737 dsc; N = 5e6 (sD/S)^5 for sL <= S < sD with sL = 0.549 sD; no damage below sL)
  dsc = 90
  assumes: partial safety factors not applied (gamma_Mf = gamma_Ff = 1); at least one range above the cut-off
```

**What the worked example means:** This calculation determines cumulative fatigue damage using the Eurocode 3 trilinear S-N curve. It sums the damage fractions from stress ranges in the input list against their cycle counts, applying a slope of three above the constant-amplitude limit and a slope of five between that limit and the cut-off. The resulting total damage is 0.5. If this value exceeds one, failure occurs within the current spectrum; otherwise, the structure can withstand 2 repetitions of this load block before reaching the damage limit. The computed thresholds are 66.3126 for the constant-amplitude fatigue limit and 36.4242 for the cut-off below which no damage accumulates.

<a id="sn_fatigue_life"></a>

#### Fatigue life from the S-N line (10^3 to 10^6 cycles) — `sn_fatigue_life`

Cycles to failure of a fully reversed amplitude on the straight log-log S-N line through f*Sut at 10^3 cycles and Se at 10^6 cycles, with the line's constants a and b.

$$
a = \frac{(fS_{ut})^2}{S_e},\quad b = -\frac13\log_{10}\frac{fS_{ut}}{S_e},\quad N = \Big(\frac{\sigma_a}{a}\Big)^{1/b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sut` | in | ultimate tensile strength | Pa |
| `Se` | in | endurance limit | Pa |
| `f` | in | fatigue strength fraction at 10^3 cycles (e.g. 0.9) | — |
| `sa` | in | fully reversed stress amplitude | Pa |
| `a_MPa` | out | S-N constant a, MPa | — |
| `b` | out | S-N exponent b | — |
| `N` | out | cycles to failure | — |
| `log10_N` | out | log10 of the cycles to failure | — |

**Applies when:** Se < sa < f Sut; 0 < f < 1; fully reversed loading.

```shell
$ mechanical-engineering solve sn_fatigue_life "Sut=700 MPa" "Se=300 MPa" f=0.85 "sa=400 MPa"
a_MPa = 1180.083333
b = -0.09913190367
N = 54912.57012
log10_N = 4.739671771
  by: Fatigue life from the S-N line (10^3 to 10^6 cycles)  (a = (f Sut)^2/Se, b = -log10(f Sut/Se)/3, N = (sa/a)^(1/b))
  Sut = 700000000 Pa
  Se = 300000000 Pa
  f = 0.85
  sa = 400000000 Pa
  assumes: Se < sa < f Sut; 0 < f < 1; fully reversed loading
```

**What the worked example means:** This calculation determines the number of cycles to failure, 54912.6, using the S-N line method. It relies on the ultimate tensile strength 700 MPa, endurance limit 300 MPa, and fatigue strength fraction 0.85 to define the linear relationship in log-log space. The resulting exponent -0.0991319 and constant 1180.08 characterize this specific material's finite-life region.

<a id="fatigue_life_mean_stress_correction"></a>

#### Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law — `fatigue_life_mean_stress_correction`

The equivalent fully reversed stress amplitude of a cycle with a mean stress, by a chosen correction, and the cycles to failure from Basquin's law sigma_ar = sf' (2N)^b.

$$
\sigma_{ar} = \frac{\sigma_a}{1-\sigma_m/\sigma_f'},\quad \sigma_{ar} = \sqrt{\sigma_{max}\sigma_a},\quad N_f = \tfrac12\Big(\frac{\sigma_{ar}}{\sigma_f'}\Big)^{1/b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sa` | in | stress amplitude | Pa |
| `sm` | in | mean stress | Pa |
| `sf` | in | fatigue strength coefficient sf' (Morrow and Goodman use it as the strength) | Pa |
| `b` | in | fatigue strength exponent (negative) | — |
| `method` | in | mean-stress correction | — |
| `gamma` | in | Walker exponent (walker only) (optional) | — |
| `sigma_ar_MPa` | out | equivalent fully reversed amplitude, MPa | — |
| `Nf` | out | cycles to failure | — |
| `log10_Nf` | out | log10 of the cycles to failure | — |

**Applies when:** -0.2 < b < 0; Morrow/Goodman: sm < sf'; SWT/Walker: sigma_max > 0; goodman here uses sf' as the strength (Morrow form).

```shell
$ mechanical-engineering solve fatigue_life_mean_stress_correction "sa=300 MPa" "sm=100 MPa" "sf=1200 MPa" b=-0.09 method=swt
sigma_ar_MPa = 346.4101615
Nf = 494790.5213
log10_Nf = 5.694421371
  by: Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law  (Morrow: sigma_ar = sa/(1 - sm/sf'); SWT: sigma_ar = sqrt(sigma_max sa); Walker: sigma_max^(1-gamma) sa^gamma; Nf = (sigma_ar/sf')^(1/b)/2)
  sa = 300000000 Pa
  sm = 100000000 Pa
  sf = 1200000000 Pa
  b = -0.09
  method = swt
  assumes: -0.2 < b < 0; Morrow/Goodman: sm < sf'; SWT/Walker: sigma_max > 0; goodman here uses sf' as the strength (Morrow form)
```

**What the worked example means:** This calculation determines the fatigue life of a component subjected to a cyclic load with a non-zero mean stress. By applying a selected mean-stress correction method such as Morrow, Goodman, Smith-Watson-Topper, or Walker, the alternating stress amplitude 300 MPa and mean stress 100 MPa are combined into an equivalent fully reversed amplitude 346.41. This equivalent stress is then used in Basquin's law with the fatigue strength coefficient 1200 MPa and exponent -0.09 to predict the number of reversals to failure 494791, providing a critical metric for assessing structural durability under asymmetric loading conditions.

<a id="asme_elliptic_fatigue_safety_factor"></a>

#### Fatigue safety factor by the ASME-elliptic criterion — `asme_elliptic_fatigue_safety_factor`

The fatigue factor of safety along a proportional load line by the ASME ellipse (sa/Se)^2 + (sm/Sy)^2 = 1, and the equivalent fully reversed amplitude.

$$
\left(\frac{n\sigma_a}{S_e}\right)^2 + \left(\frac{n\sigma_m}{S_y}\right)^2 = 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sa` | in | alternating stress amplitude | Pa |
| `sm` | in | mean stress (>= 0) | Pa |
| `Se` | in | fully corrected endurance limit | Pa |
| `Sy` | in | yield strength | Pa |
| `n_fatigue` | out | fatigue safety factor (ASME-elliptic, proportional loading) | — |
| `sigma_ar_MPa` | out | equivalent fully reversed amplitude sa/sqrt(1 - (sm/Sy)^2), MPa | — |

**Applies when:** sm >= 0; sm < Sy; proportional loading.

```shell
$ mechanical-engineering solve asme_elliptic_fatigue_safety_factor "sa=100 MPa" "sm=60 MPa" "Se=220 MPa" "Sy=450 MPa"
n_fatigue = 2.11105176
sigma_ar_MPa = 100.9009191
  by: Fatigue safety factor by the ASME-elliptic criterion  ((n sa/Se)^2 + (n sm/Sy)^2 = 1: n = 1/sqrt((sa/Se)^2 + (sm/Sy)^2))
  sa = 100000000 Pa
  sm = 60000000 Pa
  Se = 220000000 Pa
  Sy = 450000000 Pa
  assumes: sm >= 0; sm < Sy; proportional loading
```

**What the worked example means:** This calculation determines the fatigue safety factor using the ASME elliptic criterion, which accounts for both alternating stress 100 MPa and mean stress 60 MPa relative to the endurance limit 220 MPa and yield strength 450 MPa. The resulting safety factor 2.11105 indicates how close the component is to failure under proportional loading. Additionally, the equivalent fully reversed amplitude 100.901 represents the alternating stress that would cause the same damage if the mean stress were zero, providing a useful comparison for design against standard S-N curves.

<a id="gerber_fatigue_safety_factor"></a>

#### Fatigue safety factor by the Gerber criterion — `gerber_fatigue_safety_factor`

The fatigue factor of safety along a proportional load line by the Gerber parabola sa/Se + (sm/Sut)^2 = 1, and the equivalent fully reversed amplitude.

$$
\frac{n\sigma_a}{S_e} + \left(\frac{n\sigma_m}{S_{ut}}\right)^2 = 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sa` | in | alternating stress amplitude | Pa |
| `sm` | in | mean stress (>= 0) | Pa |
| `Se` | in | fully corrected endurance limit | Pa |
| `Sut` | in | ultimate tensile strength | Pa |
| `n_fatigue` | out | fatigue safety factor (Gerber, proportional loading) | — |
| `sigma_ar_MPa` | out | equivalent fully reversed amplitude sa/(1 - (sm/Sut)^2), MPa | — |

**Applies when:** sm >= 0 (compressive means not covered); sm < Sut; proportional loading.

```shell
$ mechanical-engineering solve gerber_fatigue_safety_factor "sa=120 MPa" "sm=80 MPa" "Se=250 MPa" "Sut=700 MPa"
n_fatigue = 1.976980862
sigma_ar_MPa = 121.5880893
  by: Fatigue safety factor by the Gerber criterion  (n sa/Se + (n sm/Sut)^2 = 1: n = (1/2)(Sut/sm)^2 (sa/Se) [-1 + sqrt(1 + (2 sm Se/(Sut sa))^2)])
  sa = 120000000 Pa
  sm = 80000000 Pa
  Se = 250000000 Pa
  Sut = 700000000 Pa
  assumes: sm >= 0 (compressive means not covered); sm < Sut; proportional loading
```

**What the worked example means:** This calculation determines the fatigue safety factor using the Gerber criterion for a component under proportional loading. It takes the alternating stress, mean stress, endurance limit, and ultimate tensile strength to find how many times the current stresses can be scaled before failure. The result, 1.97698, indicates the margin of safety against high-cycle fatigue, while 121.588 represents the equivalent alternating stress if the mean stress were removed.

<a id="goodman_fatigue_safety_factor"></a>

#### Fatigue safety factor by the modified Goodman criterion — `goodman_fatigue_safety_factor`

The fatigue factor of safety along a proportional load line by the modified Goodman line, the equivalent fully reversed amplitude, and (with Sy) Langer's first-cycle yield factor.

$$
\frac{\sigma_a}{S_e} + \frac{\sigma_m}{S_{ut}} = \frac1n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sa` | in | alternating stress amplitude | Pa |
| `sm` | in | mean stress (>= 0) | Pa |
| `Se` | in | fully corrected endurance limit | Pa |
| `Sut` | in | ultimate tensile strength | Pa |
| `Sy` | in | yield strength (optional: first-cycle yield check) (optional) | Pa |
| `n_fatigue` | out | fatigue safety factor (Goodman, proportional loading) | — |
| `sigma_ar_MPa` | out | equivalent fully reversed amplitude sa/(1 - sm/Sut), MPa | — |
| `n_yield` | out | Langer first-cycle yield factor Sy/(sa + sm) (with Sy) | — |

**Applies when:** sm >= 0 (compressive means not covered); sm < Sut; proportional loading (the load line through the origin).

```shell
$ mechanical-engineering solve goodman_fatigue_safety_factor "sa=120 MPa" "sm=80 MPa" "Se=250 MPa" "Sut=700 MPa"
n_fatigue = 1.682692308
sigma_ar_MPa = 135.483871
  by: Fatigue safety factor by the modified Goodman criterion  (1/n = sa/Se + sm/Sut; sigma_ar = sa/(1 - sm/Sut); n_y = Sy/(sa + sm))
  sa = 120000000 Pa
  sm = 80000000 Pa
  Se = 250000000 Pa
  Sut = 700000000 Pa
  assumes: sm >= 0 (compressive means not covered); sm < Sut; proportional loading (the load line through the origin)
```

In the worked example, sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa go through the equation above and give n_fatigue = 1.682692308, sigma_ar_MPa = 135.483871. It applies only when sm >= 0 (compressive means not covered); sm < Sut; proportional loading (the load line through the origin): outside that it says what is wrong instead of returning a number.

<a id="soderberg_fatigue_safety_factor"></a>

#### Fatigue safety factor by the Soderberg criterion — `soderberg_fatigue_safety_factor`

The conservative Soderberg fatigue factor of safety (the line from Se to the yield strength) along a proportional load line, and the equivalent amplitude.

$$
\frac{\sigma_a}{S_e} + \frac{\sigma_m}{S_y} = \frac1n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sa` | in | alternating stress amplitude | Pa |
| `sm` | in | mean stress (>= 0) | Pa |
| `Se` | in | fully corrected endurance limit | Pa |
| `Sy` | in | yield strength | Pa |
| `n_fatigue` | out | fatigue safety factor (Soderberg) | — |
| `sigma_ar_MPa` | out | equivalent fully reversed amplitude sa/(1 - sm/Sy), MPa | — |

**Applies when:** sm >= 0; sm < Sy; proportional loading.

```shell
$ mechanical-engineering solve soderberg_fatigue_safety_factor "sa=100 MPa" "sm=60 MPa" "Se=220 MPa" "Sy=450 MPa"
n_fatigue = 1.701030928
sigma_ar_MPa = 115.3846154
  by: Fatigue safety factor by the Soderberg criterion  (1/n = sa/Se + sm/Sy)
  sa = 100000000 Pa
  sm = 60000000 Pa
  Se = 220000000 Pa
  Sy = 450000000 Pa
  assumes: sm >= 0; sm < Sy; proportional loading
```

**What the worked example means:** This calculation determines the conservative Soderberg fatigue safety factor, which connects the endurance limit to the yield strength. Using the alternating stress of 100 MPa Pa and mean stress of 60 MPa Pa against the corrected endurance limit of 220 MPa Pa and yield strength of 450 MPa Pa, the resulting safety factor is 1.70103. The method also computes an equivalent fully reversed amplitude of 115.385 MPa to represent the combined loading effect on a ductile material under proportional loading conditions.

<a id="sn_fatigue_strength"></a>

#### Fatigue strength at a given number of cycles (S-N line) — `sn_fatigue_strength`

The fully reversed fatigue strength at N cycles (10^3 to 10^6) on the straight log-log S-N line through f*Sut at 10^3 and Se at 10^6 cycles.

$$
S_f = aN^b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Sut` | in | ultimate tensile strength | Pa |
| `Se` | in | endurance limit | Pa |
| `f` | in | fatigue strength fraction at 10^3 cycles | — |
| `N` | in | number of cycles (10^3 to 10^6) | — |
| `Sf_MPa` | out | fatigue strength at N cycles, MPa | — |

**Applies when:** 10^3 <= N <= 10^6; 0 < f < 1, f Sut > Se.

```shell
$ mechanical-engineering solve sn_fatigue_strength "Sut=700 MPa" "Se=300 MPa" f=0.85 N=1000000
Sf_MPa = 300
  by: Fatigue strength at a given number of cycles (S-N line)  (Sf = a N^b with a = (f Sut)^2/Se, b = -log10(f Sut/Se)/3)
  Sut = 700000000 Pa
  Se = 300000000 Pa
  f = 0.85
  N = 1000000
  assumes: 10^3 <= N <= 10^6; 0 < f < 1, f Sut > Se
```

**What the worked example means:** This calculation determines the fully reversed stress amplitude that a material can withstand for 1e+06 cycles. It uses a straight-line approximation on a log-log plot, defined by the ultimate tensile strength of 700 MPa, the endurance limit of 300 MPa, and the fatigue strength fraction f of 0.85. The resulting fatigue strength is 300, which represents the maximum allowable alternating stress for this specific life target within the high-cycle range.

<a id="miner_damage_sn_curve"></a>

#### Miner's rule damage with a single-slope S-N curve — `miner_damage_sn_curve`

The Palmgren-Miner damage sum of stress-range blocks on an S-N curve N = Nc (Sc/S)^m, and how many repeats of the spectrum the part survives.

$$
D = \sum_i\frac{n_i}{N_i},\quad N_i = N_c\Big(\frac{S_c}{S_i}\Big)^m
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ranges` | in | stress ranges in MPa (a list) | — |
| `counts` | in | number of cycles of each range (a list) | — |
| `Sc` | in | reference stress range of the curve at Nc cycles, MPa | — |
| `m` | in | S-N slope (e.g. 3 for welded details) | — |
| `Nc` | in | cycles at Sc (default 2e6) (optional) | — |
| `damage` | out | Miner damage sum D | — |
| `repeats_to_failure` | out | repeats of the spectrum until D = 1 | — |

**Applies when:** linear damage accumulation (Palmgren-Miner); a single-slope curve without endurance limit.

```shell
$ mechanical-engineering solve miner_damage_sn_curve "ranges=[90]" "counts=[1000000.0]" Sc=90 m=3
damage = 0.5
repeats_to_failure = 2
  by: Miner's rule damage with a single-slope S-N curve  (D = sum n_i / N_i, N_i = Nc (Sc/S_i)^m)
  Sc = 90
  m = 3
  assumes: linear damage accumulation (Palmgren-Miner); a single-slope curve without endurance limit
```

**What the worked example means:** This calculation determines cumulative fatigue damage using Palmgren-Miner linear accumulation. It processes a list of stress ranges and their corresponding cycle counts against a single-slope S-N curve defined by reference stress range 90 and slope 3. The resulting damage sum 0.5 indicates the fraction of life consumed, while 2 shows how many times the entire spectrum can be repeated until the damage reaches unity.

<a id="rainflow_counting"></a>

#### Rainflow cycle counting of a load history (ASTM E1049) — `rainflow_counting`

The cycles in a load or stress history by rainflow counting (three-point method, half cycles for the residue), as distinct ranges with their counts.

$$
|x_3-x_2| \ge |x_2-x_1| \Rightarrow \text{count } |x_2-x_1|
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `history` | in | the load or stress history, point by point (a list; any one unit) | — |
| `ranges` | out | distinct ranges, largest first (in the history's unit) | — |
| `counts` | out | cycles of each range (half cycles count 0.5) | — |
| `total_cycles` | out | total number of cycles | — |
| `max_range` | out | largest range | — |

**Applies when:** at least 3 points, not constant; ASTM E1049-85: the first and last points are reversals, the residue counts as half cycles; ranges equal to 12 significant digits are merged.

```shell
$ mechanical-engineering solve rainflow_counting "history=[0, 5, -3, 4, -2, 6, -4, 1, 0]"
ranges = [10.0,9.0,8.0,6.0,5.0,1.0]
counts = [0.5,0.5,0.5,1.0,1.0,0.5]
total_cycles = 4
max_range = 10
  by: Rainflow cycle counting of a load history (ASTM E1049)  (reversals (peaks and valleys), then the three-point rule: when a range Y is not larger than the next range X, count Y (a half cycle if it contains the start) and remove its points)
  assumes: at least 3 points, not constant; ASTM E1049-85: the first and last points are reversals, the residue counts as half cycles; ranges equal to 12 significant digits are merged
```

**What the worked example means:** The algorithm processes the load history [0, 5, -3, 4, -2, 6, -4, 1, 0] to identify stress reversals and apply the three-point rule for fatigue analysis. It extracts distinct ranges, such as the largest one at 10, and assigns counts to each, where half cycles contribute zero point five. The result is a set of ranges in descending order with their corresponding counts, summing to 4 total cycles, which serves as input for damage accumulation models.

<a id="shaft_diameter_de_goodman"></a>

#### Shaft diameter for a design factor (DE-Goodman) — `shaft_diameter_de_goodman`

The smallest solid-shaft diameter giving a required fatigue design factor under alternating and mean bending and torsion (distortion-energy Goodman).

$$
d = \Big(\frac{16n}{\pi}\Big\{\frac{\sqrt{4(K_fM_a)^2+3(K_{fs}T_a)^2}}{S_e} + \frac{\sqrt{4(K_fM_m)^2+3(K_{fs}T_m)^2}}{S_{ut}}\Big\}\Big)^{1/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | required fatigue design factor (>= 1) | — |
| `Ma` | in | alternating bending moment | N*m |
| `Mm` | in | mean bending moment | N*m |
| `Ta` | in | alternating torque | N*m |
| `Tm` | in | mean torque | N*m |
| `Kf` | in | fatigue stress-concentration factor, bending | — |
| `Kfs` | in | fatigue stress-concentration factor, torsion | — |
| `Se` | in | fully corrected endurance limit | Pa |
| `Sut` | in | ultimate tensile strength | Pa |
| `d_mm` | out | required diameter, mm | — |

**Applies when:** solid round shaft; the stress-concentration factors for the final size are assumed known; n >= 1.

```shell
$ mechanical-engineering solve shaft_diameter_de_goodman n=1.5 "Ma=150 N*m" "Mm=0 N*m" "Ta=0 N*m" "Tm=200 N*m" Kf=1.6 Kfs=1.4 "Se=200 MPa" "Sut=700 MPa"
d_mm = 28.69494213
  by: Shaft diameter for a design factor (DE-Goodman)  (d = (16n/pi [sqrt(4(Kf Ma)^2 + 3(Kfs Ta)^2)/Se + sqrt(4(Kf Mm)^2 + 3(Kfs Tm)^2)/Sut])^(1/3))
  n = 1.5
  Ma = 150 N*m
  Mm = 0 N*m
  Ta = 0 N*m
  Tm = 200 N*m
  Kf = 1.6
  Kfs = 1.4
  Se = 200000000 Pa
  Sut = 700000000 Pa
  assumes: solid round shaft; the stress-concentration factors for the final size are assumed known; n >= 1
```

**What the worked example means:** This calculation determines the minimum solid round shaft diameter needed to achieve a specific fatigue design factor under combined alternating and mean bending and torsion loads. Using the distortion-energy Goodman criterion, the method incorporates stress concentration factors for both bending and torsion alongside the material's endurance limit and ultimate tensile strength. The resulting diameter of 28.6949 ensures that the equivalent von Mises stresses remain within safe limits relative to the failure envelope defined by the Goodman line.

<a id="shaft_fatigue_safety_factor"></a>

#### Shaft fatigue safety factor (DE-Goodman) — `shaft_fatigue_safety_factor`

The fatigue factor of safety of a rotating solid shaft under alternating and mean bending and torsion by the distortion-energy Goodman criterion, with the von Mises stresses and (with Sy) the first-cycle yield factor.

$$
\frac1n = \frac{16}{\pi d^3}\Big\{\frac{\sqrt{4(K_fM_a)^2+3(K_{fs}T_a)^2}}{S_e} + \frac{\sqrt{4(K_fM_m)^2+3(K_{fs}T_m)^2}}{S_{ut}}\Big\}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | shaft diameter | m |
| `Ma` | in | alternating bending moment | N*m |
| `Mm` | in | mean bending moment | N*m |
| `Ta` | in | alternating torque | N*m |
| `Tm` | in | mean torque | N*m |
| `Kf` | in | fatigue stress-concentration factor, bending | — |
| `Kfs` | in | fatigue stress-concentration factor, torsion | — |
| `Se` | in | fully corrected endurance limit | Pa |
| `Sut` | in | ultimate tensile strength | Pa |
| `Sy` | in | yield strength (optional) (optional) | Pa |
| `n_fatigue` | out | fatigue safety factor (DE-Goodman) | — |
| `sigma_a_vm_MPa` | out | alternating von Mises stress, MPa | — |
| `sigma_m_vm_MPa` | out | mean von Mises stress, MPa | — |
| `n_yield` | out | first-cycle yield factor (with Sy) | — |

**Applies when:** solid round shaft, no axial load; Kf, Kfs >= 1; an alternating moment or torque present.

```shell
$ mechanical-engineering solve shaft_fatigue_safety_factor "d=40 mm" "Ma=150 N*m" "Mm=0 N*m" "Ta=0 N*m" "Tm=200 N*m" Kf=1.6 Kfs=1.4 "Se=200 MPa" "Sut=700 MPa"
n_fatigue = 4.06307813
sigma_a_vm_MPa = 38.19718634
sigma_m_vm_MPa = 38.59302268
  by: Shaft fatigue safety factor (DE-Goodman)  (sigma_a' = sqrt((32 Kf Ma/(pi d^3))^2 + 3(16 Kfs Ta/(pi d^3))^2), likewise sigma_m'; 1/n = sigma_a'/Se + sigma_m'/Sut)
  d = 0.04 m
  Ma = 150 N*m
  Mm = 0 N*m
  Ta = 0 N*m
  Tm = 200 N*m
  Kf = 1.6
  Kfs = 1.4
  Se = 200000000 Pa
  Sut = 700000000 Pa
  assumes: solid round shaft, no axial load; Kf, Kfs >= 1; an alternating moment or torque present
```

In the worked example, d = 40 mm, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa go through the equation above and give n_fatigue = 4.06307813, sigma_a_vm_MPa = 38.19718634, sigma_m_vm_MPa = 38.59302268. It applies only when solid round shaft, no axial load; Kf, Kfs >= 1; an alternating moment or torque present: outside that it says what is wrong instead of returning a number.

### Machine elements (15)

<a id="belt_friction_capstan"></a>

#### Belt or rope friction on a drum (capstan equation) — `belt_friction_capstan`

The largest tight-side tension a flat belt, rope or V-belt can hold for a given slack-side tension, friction coefficient and wrap angle.

$$
\frac{T_1}{T_2} = e^{\mu\theta}\quad\Big(e^{\mu\theta/\sin(\beta/2)}\ \text{for a V-belt}\Big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T2` | in | slack-side tension | N |
| `mu` | in | coefficient of friction | — |
| `theta` | in | angle of wrap | rad |
| `beta` | in | included groove angle of a V-belt (optional; flat belt when absent) (optional) | rad |
| `T1` | out | tight-side tension at impending slip | N |
| `ratio` | out | tension ratio T1/T2 | — |
| `friction_force` | out | net friction force T1 - T2 | N |

**Applies when:** impending slip, no centrifugal effect; mu > 0, wrap > 0.

```shell
$ mechanical-engineering solve belt_friction_capstan "T2=200 N" mu=0.3 "theta=180 deg"
T1 = 513.266479 N
ratio = 2.566332395
friction_force = 313.266479 N
  by: Belt or rope friction on a drum (capstan equation)  (T1/T2 = exp(mu theta), V-belt exp(mu theta / sin(beta/2)))
  T2 = 200 N
  mu = 0.3
  theta = 3.141592654 rad
  assumes: impending slip, no centrifugal effect; mu > 0, wrap > 0
```

**What the worked example means:** This calculation determines the maximum tight-side tension a belt can support before slipping, given a slack-side tension of 200 N and a wrap angle of 180 deg. Using a friction coefficient of 0.3, the resulting tension ratio is 2.56633, which means the tight side holds 513.266 N. The net friction force available for power transmission is 313.266 N.

<a id="bolt_preload_torque"></a>

#### Bolt tightening torque and preload (T = K F d) — `bolt_preload_torque`

The tightening torque T = K F d that produces a bolt preload F with nut factor K and nominal diameter d, or the preload produced by a torque.

$$
T = K F_i\,d
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | nut (torque) factor (about 0.2 for unlubricated steel) | — |
| `d` | in | nominal bolt diameter | m |
| `F` | in | preload (give F or T) (optional) | N |
| `T` | in | tightening torque (give F or T) (optional) | N*m |
| `T` | out | tightening torque | N*m |
| `F` | out | bolt preload | N |
| `F_kN` | out | bolt preload, kN | — |

**Applies when:** 0.05 <= K <= 0.5; exactly one of F and T.

```shell
$ mechanical-engineering solve bolt_preload_torque K=0.2 "d=12 mm" "F=25 kN"
T = 60 N*m
F = 25000 N
F_kN = 25
  by: Bolt tightening torque and preload (T = K F d)  (T = K F d; F = T/(K d))
  K = 0.2
  d = 0.012 m
  F = 25000 N
  assumes: 0.05 <= K <= 0.5; exactly one of F and T
```

**What the worked example means:** This calculation determines the relationship between the applied tightening torque and the resulting bolt preload using the nut factor method. Given a nut factor of 0.2 and a nominal diameter of 12 mm, the required torque to achieve a specific preload or the preload generated by a given torque is computed. The result, 60 N*m, indicates the force needed at the wrench, while 25000 N represents the clamping force holding the joint together.

<a id="eccentric_bolt_group"></a>

#### Eccentrically loaded bolt group (elastic method) — `eccentric_bolt_group`

The direct shear per bolt, the largest resultant bolt force and which bolt carries it, for a group of identical bolts under a vertical load applied with a horizontal eccentricity.

$$
\mathbf F_i = -\frac{P}{n}\hat{\mathbf y} + \frac{Pe}{\sum r_j^2}\,(y_i-\bar y,\; -(x_i-\bar x))
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `bolts` | in | bolt positions, one row 'x y' per bolt, in mm | — |
| `P` | in | vertical load (downward) | N |
| `e` | in | horizontal distance of the load line from the bolt-group centroid (positive to the right) | m |
| `direct_kN` | out | direct shear per bolt, kN | — |
| `max_bolt_force_kN` | out | largest resultant bolt force, kN | — |
| `critical_bolt` | out | number of the most loaded bolt (1-based, the first if several are equal) | — |
| `polar_sum_mm2` | out | sum of r^2 about the centroid, mm^2 | — |

**Applies when:** identical bolts, rigid plate, elastic method; 2 to 40 bolts, not all at one point.

```shell
$ mechanical-engineering solve eccentric_bolt_group "bolts=[[0, 0], [0, 100], [100, 0], [100, 100]]" "P=20 kN" "e=250 mm"
direct_kN = 5
max_bolt_force_kN = 21.50581317
critical_bolt = 3
polar_sum_mm2 = 20000
  by: Eccentrically loaded bolt group (elastic method)  (direct F/n plus torsional M r_i / sum r^2 perpendicular to r_i, with M = P e about the bolt centroid; vector sum per bolt)
  P = 20000 N
  e = 0.25 m
  assumes: identical bolts, rigid plate, elastic method; 2 to 40 bolts, not all at one point
```

**What the worked example means:** This calculation determines the forces in a group of identical bolts subjected to a vertical load applied with a horizontal eccentricity. The total load is split into a direct shear component shared equally by all bolts and a torsional component caused by the moment about the centroid. For the given inputs, the direct shear on each bolt is 5. The geometric property sum of squared distances from the centroid is 20000. By vectorially adding the direct and torsional forces for each bolt, the most critical bolt is identified as number 3, which experiences the largest resultant force of 21.5058. This result helps engineers assess if the selected bolt pattern can safely support the applied load under elastic conditions.

<a id="four_bar_linkage_position"></a>

#### Four-bar linkage position analysis — `four_bar_linkage_position`

The coupler and output-rocker angles and the transmission angle of a four-bar linkage at a given crank angle, on the open or the crossed branch.

$$
K_1\cos\theta_4 - K_2\cos\theta_2 + K_3 = \cos(\theta_2-\theta_4),\quad \theta_4 = 2\arctan\frac{-B\mp\sqrt{B^2-4AC}}{2A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | input crank length | m |
| `b` | in | coupler length | m |
| `c` | in | output rocker length | m |
| `d` | in | ground link length | m |
| `theta2` | in | crank angle from the ground link | rad |
| `branch` | in | assembly branch | — |
| `theta3_deg` | out | coupler angle, degrees in [-180, 180) | — |
| `theta4_deg` | out | output rocker angle, degrees in [-180, 180) | — |
| `transmission_angle_deg` | out | transmission angle (acute), degrees | — |

**Applies when:** ground pivots at (0, 0) and (d, 0); open branch: the rocker pivot lies to the right of the directed coupler; refused when the linkage cannot close.

```shell
$ mechanical-engineering solve four_bar_linkage_position "a=2 m" "b=7 m" "c=9 m" "d=6 m" "theta2=30 deg" branch=open
theta3_deg = 88.8372413
theta4_deg = 117.2860679
transmission_angle_deg = 28.44882656
  by: Four-bar linkage position analysis  (Freudenstein's equation: K1 cos th4 - K2 cos th2 + K3 = cos(th2 - th4) solved by the half-angle substitution, th4 = 2 atan2(-B -+ sqrt(B^2 - 4AC), 2A); likewise th3)
  a = 2 m
  b = 7 m
  c = 9 m
  d = 6 m
  theta2 = 0.5235987756 rad
  branch = open
  assumes: ground pivots at (0, 0) and (d, 0); open branch: the rocker pivot lies to the right of the directed coupler; refused when the linkage cannot close
```

**What the worked example means:** This calculation determines the angular positions of the coupler and output links in a four-bar linkage. Given the input crank length, coupler length, output rocker length, ground link length, and current crank angle, it solves for the resulting geometry on the specified assembly branch. The computed coupler angle of 88.8372 and output rocker angle of 117.286 describe the orientation of these links relative to the ground. Additionally, the transmission angle of 28.4488 indicates how effectively force is transferred from the input to the output at this configuration.

<a id="helical_compression_spring"></a>

#### Helical compression spring: rate, deflection and shear stress — `helical_compression_spring`

The spring index, rate, deflection under a load and the wire shear stress (torsion alone, and with the direct-shear factor Ks = 1 + 0.5/C) of a round-wire helical spring.

$$
k = \frac{Gd^4}{8D^3N_a},\quad \tau = K_s\frac{8FD}{\pi d^3},\quad K_s = 1 + \frac{0.5}{C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | wire diameter | m |
| `D` | in | mean coil diameter | m |
| `Na` | in | number of active coils | — |
| `G` | in | shear modulus of the wire | Pa |
| `F` | in | axial load | N |
| `index` | out | spring index C = D/d | — |
| `k` | out | spring rate | N/m |
| `deflection_mm` | out | deflection under F, mm | — |
| `tau_torsion_MPa` | out | torsional shear stress 8FD/(pi d^3), MPa | — |
| `tau_Ks_MPa` | out | shear stress with the direct-shear factor Ks, MPa | — |

**Applies when:** C > 2; close-coiled (small helix angle); curvature (Wahl) correction not included.

```shell
$ mechanical-engineering solve helical_compression_spring "d=3 mm" "D=24 mm" Na=10 "G=79 GPa" "F=100 N"
index = 8
k = 5786.132812 N/m
deflection_mm = 17.28270042
tau_torsion_MPa = 226.3536968
tau_Ks_MPa = 240.5008029
  by: Helical compression spring: rate, deflection and shear stress  (k = G d^4/(8 D^3 Na), delta = F/k, tau = Ks 8FD/(pi d^3) with Ks = 1 + 0.5/C)
  d = 0.003 m
  D = 0.024 m
  Na = 10
  G = 7.9e+10 Pa
  F = 100 N
  assumes: C > 2; close-coiled (small helix angle); curvature (Wahl) correction not included
```

**What the worked example means:** This calculation determines the mechanical behavior of a helical compression spring based on its geometry and material properties. Given the wire diameter, mean coil diameter, active coils, shear modulus, and axial load, it computes the spring index, stiffness, resulting deflection, and shear stresses. The output shows a spring rate of 5786.13 N/m N/m, which leads to a deflection of 17.2827 mm under the applied load. The torsional shear stress is 226.354 MPa, while the total shear stress including the direct shear factor is 240.501 MPa.

<a id="lewis_gear_bending_stress"></a>

#### Lewis bending stress of a gear tooth — `lewis_gear_bending_stress`

The bending stress at the root of a spur-gear tooth by the Lewis equation sigma = Kv Wt/(F m Y) (SI, module m, form factor Y based on the module).

$$
\sigma = \frac{K_v W_t}{F\,m\,Y}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Wt` | in | transmitted (tangential) load | N |
| `F` | in | face width | m |
| `m` | in | module | m |
| `Y` | in | Lewis form factor (from the tooth count and pressure angle, based on the module) | — |
| `Kv` | in | velocity (dynamic) factor, default 1 (optional) | — |
| `sigma` | out | tooth bending stress | Pa |
| `sigma_MPa` | out | tooth bending stress, MPa | — |

**Applies when:** Wt, F, m > 0; 0 < Y <= 1; Kv >= 1; load at the tip, one tooth in contact.

```shell
$ mechanical-engineering solve lewis_gear_bending_stress "Wt=2.5 kN" "F=40 mm" "m=4 mm" Y=0.322
sigma = 48524844.72 Pa
sigma_MPa = 48.52484472
  by: Lewis bending stress of a gear tooth  (sigma = Kv Wt/(F m Y))
  Wt = 2500 N
  F = 0.04 m
  m = 0.004 m
  Y = 0.322
  assumes: Wt, F, m > 0; 0 < Y <= 1; Kv >= 1; load at the tip, one tooth in contact
```

In the worked example, Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322 go through the equation above and give sigma = 48524844.72 Pa, sigma_MPa = 48.52484472. It applies only when Wt, F, m > 0; 0 < Y <= 1; Kv >= 1; load at the tip, one tooth in contact: outside that it says what is wrong instead of returning a number.

<a id="belt_drive_power"></a>

#### Power of a belt drive with centrifugal tension — `belt_drive_power`

The centrifugal tension, slack-side tension and transmitted power of a flat or V-belt running at the allowable tight-side tension and impending slip.

$$
\frac{T_1 - mv^2}{T_2 - mv^2} = e^{\mu\theta},\quad P = (T_1-T_2)v
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T1` | in | allowable tight-side tension | N |
| `m` | in | belt mass per unit length | kg/m |
| `v` | in | belt speed | m/s |
| `mu` | in | coefficient of friction | — |
| `theta` | in | wrap angle on the smaller pulley | rad |
| `beta` | in | included groove angle for a V-belt (optional) (optional) | rad |
| `Tc` | out | centrifugal tension m v^2 | N |
| `T2` | out | slack-side tension | N |
| `P` | out | transmitted power | W |
| `P_kW` | out | transmitted power in kW | — |

**Applies when:** impending slip on the smaller pulley; m v^2 < T1.

```shell
$ mechanical-engineering solve belt_drive_power "T1=1.2 kN" "m=0.4 kg/m" "v=20 m/s" mu=0.3 "theta=160 deg"
Tc = 160 N
T2 = 609.986666 N
P = 11800.26668 W
P_kW = 11.80026668
  by: Power of a belt drive with centrifugal tension  ((T1 - Tc)/(T2 - Tc) = exp(mu theta) (mu/sin(beta/2) for a V-belt), Tc = m v^2, P = (T1 - T2) v)
  T1 = 1200 N
  m = 0.4 kg/m
  v = 20 m/s
  mu = 0.3
  theta = 2.792526803 rad
  assumes: impending slip on the smaller pulley; m v^2 < T1
```

**What the worked example means:** This calculation determines the centrifugal tension, slack-side tension, and transmitted power for a belt drive operating at impending slip. Given an allowable tight-side tension of 1.2 kN, a belt mass per unit length of 0.4 kg/m, and a speed of 20 m/s, the centrifugal tension is found to be 160 N. Using the coefficient of friction 0.3 and wrap angle 160 deg, the resulting slack-side tension is 609.987 N. The total power transmitted by the belt is calculated as 11800.3 W, which corresponds to 11.8003 in kilowatts.

<a id="power_screw_torque"></a>

#### Power screw: raising and lowering torque, efficiency, self-locking — `power_screw_torque`

The torque to raise and to lower a load with a square or Acme power screw (thread and collar friction), the raising efficiency and whether the screw is self-locking.

$$
T_R = \frac{Fd_m}{2}\,\frac{l + \pi fd_m\sec\alpha}{\pi d_m - fl\sec\alpha} + \frac{Ff_cd_c}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | axial load | N |
| `dm` | in | mean thread diameter | m |
| `l` | in | lead | m |
| `f` | in | thread friction coefficient | — |
| `alpha` | in | thread half-angle (0 for square, 14.5 deg for Acme; default 0) (optional) | rad |
| `fc` | in | collar friction coefficient (default 0) (optional) | — |
| `dc` | in | collar mean diameter (needed with fc) (optional) | m |
| `T_raise` | out | torque to raise the load | N*m |
| `T_lower` | out | torque to lower the load (negative: the load lowers itself) | N*m |
| `efficiency` | out | efficiency when raising | — |
| `self_locking` | out | yes if the thread alone holds the load | — |

**Applies when:** alpha < 45 degrees; thread friction taken as f/cos(alpha); single thrust collar.

```shell
$ mechanical-engineering solve power_screw_torque "F=6.4 kN" "dm=30 mm" "l=12 mm" f=0.08 fc=0.08 "dc=40 mm"
T_raise = 30.34791719 N*m
T_lower = 5.742709396 N*m
efficiency = 0.4027656841
self_locking = no
  by: Power screw: raising and lowering torque, efficiency, self-locking  (T_R = F dm/2 (l + pi f dm sec a)/(pi dm - f l sec a) + F fc dc/2; T_L = F dm/2 (pi f dm sec a - l)/(pi dm + f l sec a) + F fc dc/2; e = F l/(2 pi T_R))
  F = 6400 N
  dm = 0.03 m
  l = 0.012 m
  f = 0.08
  fc = 0.08
  dc = 0.04 m
  assumes: alpha < 45 degrees; thread friction taken as f/cos(alpha); single thrust collar
```

**What the worked example means:** This calculation determines the torque required to raise a load using a power screw, resulting in 30.3479 N*m, and the torque needed to lower it, which is 5.74271 N*m. The mechanical efficiency during the lifting process is found to be 0.402766. These values depend on the axial load, thread geometry, and friction coefficients for both the threads and the collar.

<a id="bearing_catalog_rating"></a>

#### Required catalogue rating of a rolling bearing — `bearing_catalog_rating`

The basic dynamic load rating C10 a ball or roller bearing needs for a design load, life and speed at a required reliability (three-parameter Weibull, application factor).

$$
C_{10} = a_fF_D\Big[\frac{x_D}{x_0 + (\theta-x_0)(\ln 1/R_D)^{1/b}}\Big]^{1/a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `FD` | in | design (equivalent radial) load | N |
| `LD` | in | design life (hours) | s |
| `n` | in | speed (rpm) | rad/s |
| `R` | in | required reliability | — |
| `kind` | in | ball (a = 3) or roller (a = 10/3) | — |
| `af` | in | application (load) factor (default 1) (optional) | — |
| `LR` | in | rating life of the catalogue, revolutions (default 1e6) (optional) | — |
| `x0` | in | Weibull guaranteed life multiple (default 0.02) (optional) | — |
| `theta` | in | Weibull characteristic life multiple (default 4.459) (optional) | — |
| `b` | in | Weibull shape (default 1.483) (optional) | — |
| `C10` | out | required catalogue rating | N |
| `C10_kN` | out | required rating in kN | — |
| `design_life_multiple` | out | design life as a multiple of LR | — |

**Applies when:** 0 < R < 1; Weibull defaults x0 = 0.02, theta = 4.459, b = 1.483 and LR = 1e6 revolutions unless given.

```shell
$ mechanical-engineering solve bearing_catalog_rating "FD=1.8 kN" "LD=30000 h" "n=500 rpm" R=0.99 kind=ball af=1.2
C10 = 34567.18494 N
C10_kN = 34.56718494
design_life_multiple = 900
  by: Required catalogue rating of a rolling bearing  (C10 = af FD [xD/(x0 + (theta - x0)(ln 1/R)^(1/b))]^(1/a), xD = LD n 60/LR)
  FD = 1800 N
  LD = 108000000 s
  n = 52.35987756 rad/s
  R = 0.99
  af = 1.2
  kind = ball
  assumes: 0 < R < 1; Weibull defaults x0 = 0.02, theta = 4.459, b = 1.483 and LR = 1e6 revolutions unless given
```

**What the worked example means:** This calculation determines the minimum basic dynamic load rating a rolling bearing must have to survive the specified design life and reliability. It uses the design load, speed, and Weibull distribution parameters to adjust the standard catalogue rating. The result indicates that a bearing with a rating of at least 34567.2 N is needed for this application.

<a id="bearing_life_reliability"></a>

#### Rolling-bearing life at a given reliability — `bearing_life_reliability`

The rating (L10) life of a ball or roller bearing from its catalogue rating and load, and the life at another reliability from the three-parameter Weibull distribution, in revolutions and hours.

$$
L_{10} = L_R\Big(\frac{C}{F}\Big)^a,\quad x = x_0 + (\theta-x_0)\Big(\ln\frac1R\Big)^{1/b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | catalogue (basic dynamic) load rating | N |
| `F` | in | equivalent radial load | N |
| `n` | in | speed (rpm) | rad/s |
| `R` | in | required reliability (e.g. 0.95 or 95%) | — |
| `kind` | in | ball (a = 3) or roller (a = 10/3) bearing | — |
| `LR` | in | rating life of the catalogue, revolutions (default 1e6) (optional) | — |
| `x0` | in | Weibull guaranteed life multiple (default 0.02) (optional) | — |
| `theta` | in | Weibull characteristic life multiple (default 4.459) (optional) | — |
| `b` | in | Weibull shape (default 1.483) (optional) | — |
| `L10_rev` | out | rating life (90 % reliability), revolutions | — |
| `L10_hours` | out | rating life in hours | — |
| `life_multiple` | out | life at R as a multiple of the rating life | — |
| `L_rev` | out | life at reliability R, revolutions | — |
| `L_hours` | out | life at reliability R, hours | — |

**Applies when:** 0 < R < 1; Weibull parameters of the catalogue (defaults: x0 = 0.02, theta = 4.459, b = 1.483); constant load and speed.

```shell
$ mechanical-engineering solve bearing_life_reliability "C=30 kN" "F=4 kN" "n=1500 rpm" R=0.95 kind=ball
L10_rev = 421875000
L10_hours = 4687.5
life_multiple = 0.6190562567
L_rev = 261164358.3
L_hours = 2901.826203
  by: Rolling-bearing life at a given reliability  (L10 = LR (C/F)^a; x = x0 + (theta - x0) (ln 1/R)^(1/b); L = x L10; hours = revolutions / (60 n_rpm))
  C = 30000 N
  F = 4000 N
  n = 157.0796327 rad/s
  R = 0.95
  kind = ball
  assumes: 0 < R < 1; Weibull parameters of the catalogue (defaults: x0 = 0.02, theta = 4.459, b = 1.483); constant load and speed
```

**What the worked example means:** This calculation determines the expected service life of a rolling bearing based on its dynamic load capacity and applied load. First, the basic rating life is computed as 4.21875e+08 revolutions, equivalent to 4687.5 hours at the operating speed. Then, using Weibull distribution parameters for the desired reliability of 0.95, the life is adjusted to 2.61164e+08 revolutions, or 2901.83 hours. The ratio between these lives is captured by the life multiple 0.619056, indicating how much shorter or longer the component lasts compared to the standard ninety percent reliability benchmark.

<a id="slider_crank_kinematics"></a>

#### Slider-crank mechanism: slider position, velocity and acceleration — `slider_crank_kinematics`

The slider position, velocity and acceleration and the connecting-rod angle of an in-line slider-crank mechanism at a crank angle, the crank turning at constant speed.

$$
x = r\cos\theta + \sqrt{l^2 - r^2\sin^2\theta},\quad v = \omega\frac{dx}{d\theta},\quad a = \omega^2\frac{d^2x}{d\theta^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | crank radius | m |
| `l` | in | connecting rod length | m |
| `theta` | in | crank angle from the line of stroke (top dead centre at 0) | rad |
| `omega` | in | constant crank speed (rad/s or rpm) | rad/s |
| `x` | out | distance of the slider pin from the crank centre | m |
| `v` | out | slider velocity (positive away from the crank) | m/s |
| `a` | out | slider acceleration | m/s^2 |
| `rod_angle_deg` | out | connecting-rod angle to the line of stroke, degrees | — |

**Applies when:** in-line (zero offset); l > r; constant crank speed.

```shell
$ mechanical-engineering solve slider_crank_kinematics "r=50 mm" "l=200 mm" "theta=0 deg" "omega=100 rad/s"
x = 0.25 m
v = 0 m/s
a = -625 m/s^2
rod_angle_deg = 0
  by: Slider-crank mechanism: slider position, velocity and acceleration  (x = r cos(theta) + sqrt(l^2 - r^2 sin^2 theta); v = dx/dt and a = d^2x/dt^2 with dtheta/dt = omega)
  r = 0.05 m
  l = 0.2 m
  theta = 0 rad
  omega = 100 rad/s
  assumes: in-line (zero offset); l > r; constant crank speed
```

**What the worked example means:** This calculation determines the kinematic state of an in-line slider-crank mechanism given a crank radius of 50 mm, connecting rod length of 200 mm, crank angle of 0 deg, and constant angular speed of 100 rad/s. The resulting slider distance from the crank center is 0.25 m, its velocity is 0 m/s, its acceleration is -625 m/s^2, and the connecting rod angle relative to the line of stroke is 0. These values describe the instantaneous motion characteristics essential for analyzing engine or compressor performance.

<a id="planetary_gear_train"></a>

#### Speeds of a simple planetary (epicyclic) gear train — `planetary_gear_train`

The third speed of a sun-planet-ring-carrier train from the other two (any member may be held: speed 0), with the planets' absolute speed and tooth number.

$$
\frac{\omega_s - \omega_c}{\omega_r - \omega_c} = -\frac{N_r}{N_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ns` | in | teeth on the sun | — |
| `Nr` | in | teeth on the ring (internal gear) | — |
| `ws` | in | sun speed (signed; rpm or rad/s) (optional) | rad/s |
| `wr` | in | ring speed (signed) (optional) | rad/s |
| `wc` | in | carrier (arm) speed (signed) (optional) | rad/s |
| `ws_rpm` | out | sun speed, rpm | — |
| `wr_rpm` | out | ring speed, rpm | — |
| `wc_rpm` | out | carrier speed, rpm | — |
| `wp_rpm` | out | absolute speed of each planet, rpm | — |
| `Np` | out | teeth on each planet | — |

**Applies when:** exactly two of the three speeds given; Nr > Ns, Nr - Ns even; same sign convention for every speed.

```shell
$ mechanical-engineering solve planetary_gear_train Ns=20 Nr=80 "ws=1200 rpm" "wr=0 rpm"
ws_rpm = 1200
wr_rpm = 0
wc_rpm = 240
wp_rpm = -400
Np = 30
  by: Speeds of a simple planetary (epicyclic) gear train  (Willis: (ws - wc)/(wr - wc) = -Nr/Ns; planet: wp = wc - (ws - wc) Ns/Np, Np = (Nr - Ns)/2)
  ws = 125.6637061 rad/s
  wr = 0 rad/s
  assumes: exactly two of the three speeds given; Nr > Ns, Nr - Ns even; same sign convention for every speed
```

**What the worked example means:** This calculation determines the rotational speeds of the sun, ring, and carrier in a simple epicyclic gear train using the Willis method. Given the tooth counts and two known speeds, it solves for the third speed while ensuring consistent sign conventions. The result shows that the sun rotates at 1200, the ring at 0, and the carrier at 240. Additionally, it computes the absolute speed of each planet as -400 and the number of teeth on each planet as 30. These values describe how motion is distributed among the gears, allowing any member to be fixed or used as an input or output depending on the application.

<a id="spur_gear_contact_ratio"></a>

#### Spur gear mesh geometry and contact ratio — `spur_gear_contact_ratio`

Pitch diameters, centre distance, length of the path of contact, base pitch and contact ratio of a pair of standard full-depth spur gears.

$$
Z = \sqrt{r_{a1}^2-r_{b1}^2} + \sqrt{r_{a2}^2-r_{b2}^2} - C\sin\phi,\quad m_c = \frac{Z}{\pi m\cos\phi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N1` | in | teeth on the pinion | — |
| `N2` | in | teeth on the gear | — |
| `m` | in | module (e.g. 4 mm) | m |
| `phi` | in | pressure angle | rad |
| `d1_mm` | out | pinion pitch diameter, mm | — |
| `d2_mm` | out | gear pitch diameter, mm | — |
| `C_mm` | out | centre distance, mm | — |
| `Z_mm` | out | length of the path of contact, mm | — |
| `pb_mm` | out | base pitch, mm | — |
| `contact_ratio` | out | contact ratio Z / p_b | — |

**Applies when:** standard full-depth teeth: addendum = 1 module; standard centre distance; no interference (refused otherwise); 10 <= pressure angle <= 30 degrees.

```shell
$ mechanical-engineering solve spur_gear_contact_ratio N1=20 N2=50 "m=4 mm" "phi=20 deg"
d1_mm = 80
d2_mm = 200
C_mm = 140
Z_mm = 19.55203491
pb_mm = 11.80852574
contact_ratio = 1.655755794
  by: Spur gear mesh geometry and contact ratio  (Z = sqrt(ra1^2 - rb1^2) + sqrt(ra2^2 - rb2^2) - C sin(phi), rb = r cos(phi), ra = r + m, pb = pi m cos(phi), m_c = Z/pb)
  m = 0.004 m
  phi = 0.3490658504 rad
  assumes: standard full-depth teeth: addendum = 1 module; standard centre distance; no interference (refused otherwise); 10 <= pressure angle <= 30 degrees
```

**What the worked example means:** This calculation determines the geometric mesh characteristics for a pair of standard full-depth spur gears. Using the pinion tooth count 20, gear tooth count 50, module 4 mm, and pressure angle 20 deg, it computes the pitch diameters as 80 and 200, along with the center distance 140. The resulting length of the path of contact is 19.552, which, when divided by the base pitch 11.8085, yields a contact ratio of 1.65576. A higher contact ratio indicates smoother operation because more teeth share the load at any given moment.

<a id="disc_clutch_torque"></a>

#### Torque capacity of a disc clutch or brake — `disc_clutch_torque`

The friction torque and the largest contact pressure of a plate (disc) clutch or brake with an annular friction face, by the uniform-wear or uniform-pressure assumption.

$$
T_{wear} = \frac{Ff(D+d)}{4},\quad T_{press} = \frac{Ff}{3}\,\frac{D^3-d^3}{D^2-d^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | axial actuating force | N |
| `f` | in | coefficient of friction | — |
| `D` | in | outer diameter of the friction face | m |
| `d` | in | inner diameter of the friction face | m |
| `model` | in | uniform wear (worn-in) or uniform pressure (new) | — |
| `pairs` | in | number of friction-surface pairs (default 1) (optional) | — |
| `T` | out | torque capacity | N*m |
| `p_max_MPa` | out | largest contact pressure, MPa | — |

**Applies when:** d < D; the same force on every pair.

```shell
$ mechanical-engineering solve disc_clutch_torque "F=4 kN" f=0.3 "D=200 mm" "d=100 mm" "model=uniform wear"
T = 90 N*m
p_max_MPa = 0.2546479089
  by: Torque capacity of a disc clutch or brake  (uniform wear: T = F f (D + d)/4, p_max = 2F/(pi d (D - d)); uniform pressure: T = F f (D^3 - d^3)/(3(D^2 - d^2)), p = 4F/(pi (D^2 - d^2)); times the number of pairs)
  F = 4000 N
  f = 0.3
  D = 0.2 m
  d = 0.1 m
  model = uniform wear
  assumes: d < D; the same force on every pair
```

**What the worked example means:** This result represents the friction torque of 90 N*m that a disc clutch or brake can transmit, derived from an axial force of 4 kN, a coefficient of friction of 0.3, and friction face diameters of 200 mm and 100 mm. The corresponding peak interface pressure is 0.254648, which indicates how tightly the surfaces are pressed together under the selected uniform wear or uniform pressure model.

<a id="wahl_factor"></a>

#### Wahl curvature factor of a helical spring — `wahl_factor`

The Wahl stress-correction factor K_W = (4C - 1)/(4C - 4) + 0.615/C of a round-wire helical spring (with the Bergstrasser and direct-shear factors), and with a load the corrected maximum shear stress.

$$
K_W = \frac{4C-1}{4C-4} + \frac{0.615}{C},\quad \tau = K_W\frac{8FD}{\pi d^3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | wire diameter | m |
| `D` | in | mean coil diameter | m |
| `F` | in | axial load (optional) (optional) | N |
| `index` | out | spring index C = D/d | — |
| `K_wahl` | out | Wahl factor | — |
| `K_bergstrasser` | out | Bergstrasser factor (4C + 2)/(4C - 3) | — |
| `K_s` | out | direct-shear factor 1 + 0.5/C | — |
| `tau_MPa` | out | maximum shear stress K_W 8FD/(pi d^3), MPa (with F) | — |

**Applies when:** C = D/d > 2; round wire, close-coiled.

```shell
$ mechanical-engineering solve wahl_factor "d=3 mm" "D=24 mm"
index = 8
K_wahl = 1.184017857
K_bergstrasser = 1.172413793
K_s = 1.0625
  by: Wahl curvature factor of a helical spring  (K_W = (4C - 1)/(4C - 4) + 0.615/C; tau = K_W 8 F D/(pi d^3))
  d = 0.003 m
  D = 0.024 m
  assumes: C = D/d > 2; round wire, close-coiled
```

In the worked example, d = 3 mm, D = 24 mm go through the equation above and give index = 8, K_wahl = 1.184017857, K_bergstrasser = 1.172413793, K_s = 1.0625. It applies only when C = D/d > 2; round wire, close-coiled: outside that it says what is wrong instead of returning a number.

### Pressure vessels (4)

<a id="shrink_fit_pressure"></a>

#### Shrink (press) fit: interface pressure and hoop stresses — `shrink_fit_pressure`

The contact pressure of an interference fit between a hub and a solid or hollow shaft of the same material, and the hoop stresses it causes at the interface.

$$
p = \frac{E\delta}{2R^3}\,\frac{(r_o^2-R^2)(R^2-r_i^2)}{r_o^2-r_i^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `delta` | in | radial interference (shaft radius minus hub bore radius) | m |
| `R` | in | nominal interface radius | m |
| `ro` | in | outer radius of the hub | m |
| `E` | in | Young's modulus (both parts) | Pa |
| `ri` | in | bore radius of a hollow shaft (default 0: solid) (optional) | m |
| `p_MPa` | out | interface pressure, MPa | — |
| `hub_hoop_MPa` | out | hoop stress in the hub at the interface, MPa | — |
| `shaft_hoop_MPa` | out | hoop stress in the shaft at the interface, MPa | — |

**Applies when:** ri < R < ro; both parts of the same material; plane stress, elastic, long hub.

```shell
$ mechanical-engineering solve shrink_fit_pressure "delta=0.02 mm" "R=25 mm" "ro=50 mm" "E=207 GPa"
p_MPa = 62.1
hub_hoop_MPa = 103.5
shaft_hoop_MPa = -62.1
  by: Shrink (press) fit: interface pressure and hoop stresses  (p = E delta/(2R^3) (ro^2 - R^2)(R^2 - ri^2)/(ro^2 - ri^2); hub sigma_t = p (ro^2 + R^2)/(ro^2 - R^2); shaft sigma_t = -p (R^2 + ri^2)/(R^2 - ri^2))
  delta = 2e-05 m
  R = 0.025 m
  ro = 0.05 m
  E = 2.07e+11 Pa
  assumes: ri < R < ro; both parts of the same material; plane stress, elastic, long hub
```

**What the worked example means:** This calculation determines the contact pressure generated by a radial interference fit between a hub and a shaft. Using an interference of 0.02 mm at a nominal radius of 25 mm, with a hub outer radius of 50 mm and material stiffness defined by Young's modulus 207 GPa, the resulting interface pressure is 62.1. This pressure induces a tensile hoop stress in the hub of 103.5 and a compressive hoop stress in the shaft of -62.1, values critical for assessing joint integrity and preventing material failure.

<a id="rotating_disk_stresses"></a>

#### Stresses in a rotating disk (solid or with a central hole) — `rotating_disk_stresses`

The largest hoop and radial stresses, the radius of the largest radial stress and the hoop stress at the rim of a thin disk of uniform thickness spinning at constant speed.

$$
\sigma_\theta(r_i) = \frac{3+\nu}{4}\rho\omega^2\Big(r_o^2 + \frac{1-\nu}{3+\nu}r_i^2\Big),\quad \sigma_{r,max} = \frac{3+\nu}{8}\rho\omega^2(r_o-r_i)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density | kg/m^3 |
| `omega` | in | angular speed (rpm or rad/s) | rad/s |
| `nu` | in | Poisson's ratio | — |
| `ro` | in | outer radius | m |
| `ri` | in | radius of the central hole (default 0: solid) (optional) | m |
| `hoop_max_MPa` | out | largest hoop stress (at the bore, or the centre of a solid disk), MPa | — |
| `radial_max_MPa` | out | largest radial stress, MPa | — |
| `r_radial_max` | out | radius of the largest radial stress (sqrt(ri ro); 0 for a solid disk) | m |
| `hoop_rim_MPa` | out | hoop stress at the outer rim, MPa | — |

**Applies when:** thin disk of uniform thickness, plane stress; free inner and outer edges; 0 <= nu < 0.5.

```shell
$ mechanical-engineering solve rotating_disk_stresses "rho=7800 kg/m^3" "omega=3000 rpm" nu=0.3 "ro=250 mm" "ri=50 mm"
hoop_max_MPa = 40.03111545
radial_max_MPa = 12.70218086
r_radial_max = 0.1118033989 m
hoop_rim_MPa = 10.00777886
  by: Stresses in a rotating disk (solid or with a central hole)  (solid: sigma_r = sigma_t = (3 + nu)/8 rho w^2 ro^2 at the centre; annular: sigma_t(ri) = (3 + nu)/4 rho w^2 (ro^2 + (1 - nu)/(3 + nu) ri^2), sigma_r,max = (3 + nu)/8 rho w^2 (ro - ri)^2 at sqrt(ri ro))
  rho = 7800 kg/m^3
  omega = 314.1592654 rad/s
  nu = 0.3
  ro = 0.25 m
  ri = 0.05 m
  assumes: thin disk of uniform thickness, plane stress; free inner and outer edges; 0 <= nu < 0.5
```

**What the worked example means:** This calculation determines the critical internal stresses within a thin, uniform disk spinning at a constant angular velocity. It computes the largest hoop stress, which appears at the inner edge for disks with holes or at the geometric centre for solid disks, resulting in a value of 40.0311. Additionally, it identifies the maximum radial stress of 12.7022 and specifies that this peak occurs at a radius of 0.111803 m. These values are derived from the material density, Poisson's ratio, rotational speed, and disk geometry, providing essential data for assessing structural integrity under centrifugal loads.

<a id="thick_walled_cylinder"></a>

#### Thick-walled cylinder under pressure (Lame equations) — `thick_walled_cylinder`

Radial and hoop stresses at a radius, the hoop stresses at both surfaces, the maximum shear at the bore and the closed-end axial stress of a thick cylinder under internal and external pressure.

$$
\sigma_{r,\theta} = \frac{p_ia^2 - p_ob^2}{b^2-a^2} \mp \frac{(p_i-p_o)a^2b^2}{(b^2-a^2)r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pi` | in | internal pressure | Pa |
| `po` | in | external pressure | Pa |
| `a` | in | inner radius | m |
| `b` | in | outer radius | m |
| `r` | in | radius at which to evaluate (default: the bore, r = a) (optional) | m |
| `sr_MPa` | out | radial stress at r, MPa | — |
| `st_MPa` | out | hoop stress at r, MPa | — |
| `st_inner_MPa` | out | hoop stress at the bore, MPa | — |
| `st_outer_MPa` | out | hoop stress at the outer surface, MPa | — |
| `tau_max_inner_MPa` | out | maximum shear stress at the bore, MPa | — |
| `sz_closed_MPa` | out | axial stress with closed ends, MPa | — |

**Applies when:** a < b; pressures >= 0, not both zero; a <= r <= b; linear elastic.

```shell
$ mechanical-engineering solve thick_walled_cylinder "pi=100 MPa" "po=0 MPa" "a=50 mm" "b=100 mm"
sr_MPa = -100
st_MPa = 166.6666667
st_inner_MPa = 166.6666667
st_outer_MPa = 66.66666667
tau_max_inner_MPa = 133.3333333
sz_closed_MPa = 33.33333333
  by: Thick-walled cylinder under pressure (Lame equations)  (sigma_r = A - B/r^2, sigma_t = A + B/r^2 with A = (pi a^2 - po b^2)/(b^2 - a^2), B = (pi - po) a^2 b^2/(b^2 - a^2); closed ends sigma_z = A)
  pi = 100000000 Pa
  po = 0 Pa
  a = 0.05 m
  b = 0.1 m
  assumes: a < b; pressures >= 0, not both zero; a <= r <= b; linear elastic
```

**What the worked example means:** This calculation determines the stress state within a thick-walled cylinder subjected to internal and external pressures. It computes the radial and tangential stresses at a specified radius, as well as the maximum shear stress at the inner surface. The results indicate that for an internal pressure of 100 MPa acting on a cylinder with an inner radius of 50 mm and outer radius of 100 mm, the hoop stress at the bore reaches 166.667. This value represents the primary tensile load the material must withstand, occurring where the wall is thinnest relative to the radius.

<a id="thin_walled_pressure_vessel"></a>

#### Thin-walled pressure vessel stresses (cylinder or sphere) — `thin_walled_pressure_vessel`

Hoop and longitudinal stresses and the in-plane and absolute maximum shear stresses in the wall of a thin cylindrical or spherical pressure vessel.

$$
\sigma_h = \frac{pr}{t},\quad \sigma_l = \frac{pr}{2t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | internal gauge pressure | Pa |
| `r` | in | inner radius | m |
| `t` | in | wall thickness | m |
| `shape` | in | vessel shape | — |
| `hoop_MPa` | out | hoop (circumferential) stress, MPa | — |
| `longitudinal_MPa` | out | longitudinal (axial) stress, MPa | — |
| `tau_inplane_MPa` | out | maximum in-plane shear stress, MPa | — |
| `tau_abs_MPa` | out | absolute maximum shear stress at the outer surface, MPa | — |

**Applies when:** r/t >= 10 (thin wall); closed ends, internal pressure only; r is the inner radius.

```shell
$ mechanical-engineering solve thin_walled_pressure_vessel "p=2 MPa" "r=500 mm" "t=10 mm" shape=cylinder
hoop_MPa = 100
longitudinal_MPa = 50
tau_inplane_MPa = 25
tau_abs_MPa = 50
  by: Thin-walled pressure vessel stresses (cylinder or sphere)  (cylinder: sigma_h = pr/t, sigma_l = pr/(2t); sphere: sigma = pr/(2t); tau_abs = sigma_h/2 (radial stress 0 at the outer surface))
  p = 2000000 Pa
  r = 0.5 m
  t = 0.01 m
  shape = cylinder
  assumes: r/t >= 10 (thin wall); closed ends, internal pressure only; r is the inner radius
```

**What the worked example means:** This calculation determines the stresses in a thin-walled pressure vessel using internal gauge pressure 2 MPa, inner radius 500 mm, and wall thickness 10 mm. For a cylindrical shape, the hoop stress is found to be 100 and the longitudinal stress is 50. The maximum in-plane shear stress is 25, while the absolute maximum shear stress at the outer surface is 50. These values help assess if the vessel wall can withstand the operating pressure without failure.

### Section properties (10)

<a id="composite_section_properties"></a>

#### Composite section of rectangles (parallel-axis theorem) — `composite_section_properties`

Area, centroid, centroidal second moments, product of inertia and principal second moments of a section built from non-overlapping rectangles.

$$
I_x = \sum_i\Big(\frac{b_ih_i^3}{12} + A_i(y_i-\bar y)^2\Big),\quad I_{xy} = \sum_i A_i(x_i-\bar x)(y_i-\bar y)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rects` | in | one row per rectangle: width b, height h, centre x, centre y, all in mm | — |
| `A` | out | area in mm^2 | — |
| `x_bar` | out | centroid x in mm | — |
| `y_bar` | out | centroid y in mm | — |
| `Ix` | out | centroidal second moment about x in mm^4 | — |
| `Iy` | out | centroidal second moment about y in mm^4 | — |
| `Ixy` | out | centroidal product of inertia in mm^4 | — |
| `I1` | out | major principal second moment in mm^4 | — |
| `I2` | out | minor principal second moment in mm^4 | — |

**Applies when:** rectangles do not overlap (touching is allowed); 1 to 20 rectangles, all dimensions in mm.

```shell
$ mechanical-engineering solve composite_section_properties "rects=[[200, 20, 0, 10], [20, 300, 0, 170]]"
A = 10000
x_bar = 0
y_bar = 106
Ix = 106573333.3
Iy = 13533333.33
Ixy = 0
I1 = 106573333.3
I2 = 13533333.33
  by: Composite section of rectangles (parallel-axis theorem)  (parallel-axis theorem: x_bar = sum A x / sum A, Ix = sum(bh^3/12 + A (y - y_bar)^2), Ixy = sum A (x - x_bar)(y - y_bar))
  assumes: rectangles do not overlap (touching is allowed); 1 to 20 rectangles, all dimensions in mm
```

**What the worked example means:** This calculation determines the geometric properties of a shape formed by combining non-overlapping rectangles. Using the parallel-axis theorem, it computes the total area as 10000, the centroid coordinates as 0 and 106, and the centroidal second moments about the horizontal and vertical axes as 1.06573e+08 and 1.35333e+07. It also calculates the product of inertia as 0 to find the principal axes, resulting in major and minor principal second moments of 1.06573e+08 and 1.35333e+07, which indicate the section's resistance to bending.

<a id="channel_section_properties"></a>

#### Section properties of a channel (C) section — `channel_section_properties`

Area, centroid distance from the back of the web, second moments, section moduli and radii of gyration of a channel with square corners.

$$
\bar x = \frac{\sum A_i x_i}{\sum A_i},\quad I_y = \sum\Big(\frac{h_ib_i^3}{12} + A_i(x_i-\bar x)^2\Big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | flange width (including the web) | m |
| `h` | in | overall depth | m |
| `tf` | in | flange thickness | m |
| `tw` | in | web thickness | m |
| `A` | out | area | m^2 |
| `x_bar` | out | distance of the centroid from the back of the web | m |
| `Ix` | out | second moment about the horizontal axis of symmetry | m^4 |
| `Iy` | out | second moment about the vertical centroidal axis | m^4 |
| `Zx` | out | section modulus about x | m^3 |
| `Zy_min` | out | smaller section modulus about y (flange tips) | m^3 |
| `rx` | out | radius of gyration about x | m |
| `ry` | out | radius of gyration about y | m |

**Applies when:** 2 tf < h; tw < b; square corners, no taper.

```shell
$ mechanical-engineering solve channel_section_properties "b=80 mm" "h=200 mm" "tf=10 mm" "tw=7 mm"
A = 0.00286 m^2
x_bar = 0.02391958042 m
Ix = 1.785533333e-05 m^4
Iy = 1.797574837e-06 m^4
Zx = 0.0001785533333 m^3
Zy_min = 3.205351975e-05 m^3
rx = 0.079013439 m
ry = 0.02507035441 m
  by: Section properties of a channel (C) section  (three rectangles (two flanges b x tf, web tw x (h - 2tf)) with the parallel-axis theorem)
  b = 0.08 m
  h = 0.2 m
  tf = 0.01 m
  tw = 0.007 m
  assumes: 2 tf < h; tw < b; square corners, no taper
```

**What the worked example means:** This calculation determines the geometric properties of a C-shaped channel by treating it as three rectangles: two flanges and one web. The total cross-sectional area is found to be 0.00286 m^2, while the centroid position, measured from the back face of the web, is 0.0239196 m. This offset is critical because it indicates the shear center's location relative to the web. The second moments of area are 1.78553e-05 m^4 about the horizontal symmetry axis and 1.79757e-06 m^4 about the vertical centroidal axis. These values allow engineers to calculate bending stresses and deflections for beams oriented in either direction.

<a id="hollow_circular_section_properties"></a>

#### Section properties of a hollow circle (tube) — `hollow_circular_section_properties`

Area, second moment of area, polar moment, elastic and plastic section moduli and radius of gyration of a circular tube.

$$
I = \frac{\pi(D^4-d^4)}{64},\quad J = \frac{\pi(D^4-d^4)}{32}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | outer diameter | m |
| `d` | in | inner diameter | m |
| `A` | out | area | m^2 |
| `I` | out | second moment of area about a diameter | m^4 |
| `J` | out | polar second moment of area (torsion constant) | m^4 |
| `Z` | out | elastic section modulus | m^3 |
| `Zp` | out | plastic section modulus | m^3 |
| `r` | out | radius of gyration | m |

**Applies when:** D > d >= 0.

```shell
$ mechanical-engineering solve hollow_circular_section_properties "D=60 mm" "d=40 mm"
A = 0.001570796327 m^2
I = 5.105088062e-07 m^4
J = 1.021017612e-06 m^4
Z = 1.701696021e-05 m^3
Zp = 2.533333333e-05 m^3
r = 0.01802775638 m
  by: Section properties of a hollow circle (tube)  (A = pi(D^2 - d^2)/4, I = pi(D^4 - d^4)/64, J = 2I, Z = 2I/D, Zp = (D^3 - d^3)/6, r = sqrt(D^2 + d^2)/4)
  D = 0.06 m
  d = 0.04 m
  assumes: D > d >= 0
```

**What the worked example means:** For a hollow circular section defined by an outer diameter of 60 mm and an inner diameter of 40 mm, the geometric properties are calculated. The cross-sectional area is 0.0015708 m^2, representing the material available to carry axial loads. The resistance to bending about a diameter is quantified by the second moment of area, 5.10509e-07 m^4, while the torsional stiffness is given by the polar moment of inertia, 1.02102e-06 m^4. The elastic section modulus, 1.7017e-05 m^3, indicates the stress at the extreme fiber under bending, and the plastic section modulus, 2.53333e-05 m^3, reflects the full capacity after yielding. Finally, the radius of gyration, 0.0180278 m, characterizes the distribution of mass relative to the centroid for buckling analysis.

<a id="hollow_rectangular_section_properties"></a>

#### Section properties of a hollow rectangle (box) — `hollow_rectangular_section_properties`

Area, second moments, elastic section moduli, radii of gyration and the plastic modulus about x of a rectangular hollow section of uniform wall thickness.

$$
I_x = \frac{BH^3 - bh^3}{12},\quad Z_{p,x} = \frac{BH^2 - bh^2}{4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | outer width | m |
| `H` | in | outer depth | m |
| `t` | in | wall thickness | m |
| `A` | out | cross-sectional area | m^2 |
| `Ix` | out | second moment of area about the horizontal centroidal axis | m^4 |
| `Iy` | out | second moment of area about the vertical centroidal axis | m^4 |
| `Zx` | out | elastic section modulus about x (Ix / distance to the extreme fibre) | m^3 |
| `Zy` | out | elastic section modulus about y | m^3 |
| `rx` | out | radius of gyration about x | m |
| `ry` | out | radius of gyration about y | m |
| `Zpx` | out | plastic section modulus about x | m^3 |

**Applies when:** B, H, t > 0; 2t < min(B, H); square corners (no corner radii).

```shell
$ mechanical-engineering solve hollow_rectangular_section_properties "B=100 mm" "H=200 mm" "t=10 mm"
A = 0.0056 m^2
Ix = 2.778666667e-05 m^4
Iy = 8.986666667e-06 m^4
Zx = 0.0002778666667 m^3
Zy = 0.0001797333333 m^3
rx = 0.07044078905 m
ry = 0.04005947959 m
Zpx = 0.000352 m^3
  by: Section properties of a hollow rectangle (box)  (outer rectangle minus inner rectangle: Ix = (BH^3 - bh^3)/12 with b = B - 2t, h = H - 2t; Zpx = (BH^2 - bh^2)/4)
  B = 0.1 m
  H = 0.2 m
  t = 0.01 m
  assumes: B, H, t > 0; 2t < min(B, H); square corners (no corner radii)
```

**What the worked example means:** The calculation determines geometric properties for a rectangular hollow section by subtracting the inner void from the outer boundary. Using an outer width of 100 mm, depth of 200 mm, and wall thickness of 10 mm, it yields a cross-sectional area of 0.0056 m^2. The second moment of area about the horizontal axis is 2.77867e-05 m^4, while the elastic section moduli are 0.000277867 m^3 and 0.000179733 m^3. These values indicate the beam's resistance to bending and its capacity to carry load before yielding.

<a id="polygon_section_properties"></a>

#### Section properties of a polygonal section — `polygon_section_properties`

Area, centroid, centroidal second moments, product of inertia and principal second moments of any simple polygon given by its vertices.

$$
A = \tfrac12\sum c_i,\quad I_x = \tfrac1{12}\sum c_i\,(y_i^2+y_iy_{i+1}+y_{i+1}^2) - A\bar y^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `vertices` | in | vertices in order (either sense), one row 'x y' per vertex, in mm | — |
| `A` | out | area in mm^2 | — |
| `x_bar` | out | centroid x in mm | — |
| `y_bar` | out | centroid y in mm | — |
| `Ix` | out | centroidal second moment about x in mm^4 | — |
| `Iy` | out | centroidal second moment about y in mm^4 | — |
| `Ixy` | out | centroidal product of inertia in mm^4 | — |
| `I1` | out | major principal second moment in mm^4 | — |
| `I2` | out | minor principal second moment in mm^4 | — |

**Applies when:** 3 to 60 distinct vertices; a simple polygon (edges do not cross); no holes.

```shell
$ mechanical-engineering solve polygon_section_properties "vertices=[[0, 0], [100, 0], [100, 200], [0, 200]]"
A = 20000
x_bar = 50
y_bar = 100
Ix = 66666666.67
Iy = 16666666.67
Ixy = 0
I1 = 66666666.67
I2 = 16666666.67
  by: Section properties of a polygonal section  (Green's theorem over the edges: A = 1/2 sum c_i, Ix = 1/12 sum c_i (y_i^2 + y_i y_i+1 + y_i+1^2) with c_i = x_i y_i+1 - x_i+1 y_i; then shifted to the centroid)
  assumes: 3 to 60 distinct vertices; a simple polygon (edges do not cross); no holes
```

**What the worked example means:** This calculation determines the geometric properties of the provided polygonal cross-section using Green's theorem over its edges. The computed area is 20000, while the centroid coordinates are located at x equal to 50 and y equal to 100. The centroidal second moments of area, which indicate resistance to bending about the horizontal and vertical axes through the center, are 6.66667e+07 and 1.66667e+07 respectively. The product of inertia, measuring the asymmetry of the shape relative to these axes, is 0. Finally, the major and minor principal second moments, representing the maximum and minimum bending stiffness orientations, are 6.66667e+07 and 1.66667e+07.

<a id="circular_section_properties"></a>

#### Section properties of a solid circle — `circular_section_properties`

Area, second moment of area, polar moment, elastic and plastic section moduli and radius of gyration of a solid circular section.

$$
I = \frac{\pi d^4}{64},\quad J = \frac{\pi d^4}{32},\quad Z = \frac{\pi d^3}{32}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | diameter | m |
| `A` | out | area | m^2 |
| `I` | out | second moment of area about a diameter | m^4 |
| `J` | out | polar second moment of area (torsion constant of a circle) | m^4 |
| `Z` | out | elastic section modulus | m^3 |
| `Zp` | out | plastic section modulus | m^3 |
| `r` | out | radius of gyration | m |

**Applies when:** d > 0.

```shell
$ mechanical-engineering solve circular_section_properties "d=50 mm"
A = 0.001963495408 m^2
I = 3.067961576e-07 m^4
J = 6.135923152e-07 m^4
Z = 1.22718463e-05 m^3
Zp = 2.083333333e-05 m^3
r = 0.0125 m
  by: Section properties of a solid circle  (A = pi d^2/4, I = pi d^4/64, J = pi d^4/32, Z = pi d^3/32, Zp = d^3/6, r = d/4)
  d = 0.05 m
  assumes: d > 0
```

**What the worked example means:** This calculation determines the geometric properties of a solid circular cross-section based on its diameter. Given a diameter of 50 mm, the area is computed as 0.0019635 m^2, which represents the total surface enclosed by the circle. The second moment of area, or moment of inertia, is 3.06796e-07 m^4, indicating resistance to bending. The polar moment of area is 6.13592e-07 m^4, relevant for torsional stiffness. The elastic section modulus is 1.22718e-05 m^3, used for stress calculations in the linear range, while the plastic section modulus is 2.08333e-05 m^3, applicable when material yields. Finally, the radius of gyration is 0.0125 m, a measure of how spread out the area is from the centroid.

<a id="rectangular_section_properties"></a>

#### Section properties of a solid rectangle — `rectangular_section_properties`

Area, second moments of area, elastic section moduli, radii of gyration and polar moment of a solid rectangle b wide and h deep.

$$
I_x = \frac{bh^3}{12},\quad Z_x = \frac{bh^2}{6},\quad r_x = \sqrt{I_x/A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | width (horizontal) | m |
| `h` | in | depth (vertical) | m |
| `A` | out | cross-sectional area | m^2 |
| `Ix` | out | second moment of area about the horizontal centroidal axis | m^4 |
| `Iy` | out | second moment of area about the vertical centroidal axis | m^4 |
| `Zx` | out | elastic section modulus about x (Ix / distance to the extreme fibre) | m^3 |
| `Zy` | out | elastic section modulus about y | m^3 |
| `rx` | out | radius of gyration about x | m |
| `ry` | out | radius of gyration about y | m |
| `J_polar` | out | polar second moment about the centroid (Ix + Iy; not the torsion constant) | m^4 |

**Applies when:** b, h > 0.

```shell
$ mechanical-engineering solve rectangular_section_properties "b=100 mm" "h=200 mm"
A = 0.02 m^2
Ix = 6.666666667e-05 m^4
Iy = 1.666666667e-05 m^4
Zx = 0.0006666666667 m^3
Zy = 0.0003333333333 m^3
rx = 0.05773502692 m
ry = 0.02886751346 m
J_polar = 8.333333333e-05 m^4
  by: Section properties of a solid rectangle  (A = bh, Ix = bh^3/12, Iy = hb^3/12, Zx = bh^2/6, Zy = hb^2/6, r = sqrt(I/A))
  b = 0.1 m
  h = 0.2 m
  assumes: b, h > 0
```

**What the worked example means:** This calculation determines the geometric properties for a solid rectangle defined by width 100 mm and depth 200 mm. The resulting area is 0.02 m^2, which quantifies the material present in the section. The second moment of area about the horizontal centroidal axis is 6.66667e-05 m^4, indicating resistance to bending about that axis, while the vertical counterpart is 1.66667e-05 m^4. The elastic section moduli are 0.000666667 m^3 horizontally and 0.000333333 m^3 vertically, representing the efficiency of the shape in resisting bending stress. Finally, the radii of gyration are 0.057735 m and 0.0288675 m, which describe how the area is distributed relative to the axes.

<a id="i_section_properties"></a>

#### Section properties of a symmetric I-section — `i_section_properties`

Area, second moments, elastic and plastic section moduli, radii of gyration and the shape factor of a doubly symmetric I-section (flanges b x tf, web tw, overall depth h, no fillets).

$$
I_x = \frac{bh^3 - (b-t_w)(h-2t_f)^3}{12},\quad Z_{p,x} = bt_f(h-t_f) + \frac{t_w(h-2t_f)^2}{4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | flange width | m |
| `h` | in | overall depth | m |
| `tf` | in | flange thickness | m |
| `tw` | in | web thickness | m |
| `A` | out | cross-sectional area | m^2 |
| `Ix` | out | second moment of area about the horizontal centroidal axis | m^4 |
| `Iy` | out | second moment of area about the vertical centroidal axis | m^4 |
| `Zx` | out | elastic section modulus about x (Ix / distance to the extreme fibre) | m^3 |
| `Zy` | out | elastic section modulus about y | m^3 |
| `rx` | out | radius of gyration about x | m |
| `ry` | out | radius of gyration about y | m |
| `Zpx` | out | plastic section modulus about x | m^3 |
| `Zpy` | out | plastic section modulus about y | m^3 |
| `shape_factor_x` | out | shape factor Zpx / Zx | — |

**Applies when:** 2 tf < h; tw < b; no root fillets.

```shell
$ mechanical-engineering solve i_section_properties "b=150 mm" "h=300 mm" "tf=12 mm" "tw=8 mm"
A = 0.005808 m^2
Ix = 8.8709184e-05 m^4
Iy = 6.761776e-06 m^4
Zx = 0.00059139456 m^3
Zy = 9.015701333e-05 m^3
rx = 0.1235864873 m
ry = 0.03412063351 m
Zpx = 0.000670752 m^3
Zpy = 0.000139416 m^3
shape_factor_x = 1.134186963
  by: Section properties of a symmetric I-section  (three rectangles with the parallel-axis theorem: Ix = sum(bh^3/12 + A d^2); plastic moduli about the axes of symmetry: Zpx = b tf (h - tf) + tw (h - 2tf)^2/4, Zpy = tf b^2/2 + (h - 2tf) tw^2/4)
  b = 0.15 m
  h = 0.3 m
  tf = 0.012 m
  tw = 0.008 m
  assumes: 2 tf < h; tw < b; no root fillets
```

In the worked example, b = 150 mm, h = 300 mm, tf = 12 mm, tw = 8 mm go through the equation above and give A = 0.005808 m^2, Ix = 8.8709184e-05 m^4, Iy = 6.761776e-06 m^4, Zx = 0.00059139456 m^3. It applies only when 2 tf < h; tw < b; no root fillets: outside that it says what is wrong instead of returning a number.

<a id="t_section_properties"></a>

#### Section properties of a T-section — `t_section_properties`

Area, centroid height, second moments, top and bottom section moduli and radii of gyration of a T-section (flange on top).

$$
\bar y = \frac{\sum A_i y_i}{\sum A_i},\quad I_x = \sum\Big(\frac{b_ih_i^3}{12} + A_i(y_i - \bar y)^2\Big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `bf` | in | flange width | m |
| `tf` | in | flange thickness | m |
| `h` | in | overall depth | m |
| `tw` | in | stem (web) thickness | m |
| `A` | out | area | m^2 |
| `y_bar` | out | height of the centroid above the bottom of the stem | m |
| `Ix` | out | second moment about the horizontal centroidal axis | m^4 |
| `Iy` | out | second moment about the vertical axis of symmetry | m^4 |
| `Z_top` | out | section modulus for the top fibre | m^3 |
| `Z_bottom` | out | section modulus for the bottom fibre | m^3 |
| `rx` | out | radius of gyration about x | m |
| `ry` | out | radius of gyration about y | m |

**Applies when:** tf < h; tw <= bf; no fillets.

```shell
$ mechanical-engineering solve t_section_properties "bf=120 mm" "tf=20 mm" "h=150 mm" "tw=15 mm"
A = 0.00435 m^2
y_bar = 0.1063793103 m
Ix = 8.877974138e-06 m^4
Iy = 2.9165625e-06 m^4
Z_top = 0.0002035266798 m^3
Z_bottom = 8.345583468e-05 m^3
rx = 0.0451764717 m
ry = 0.02589351536 m
  by: Section properties of a T-section  (two rectangles: y_bar = sum(A y)/sum(A); Ix = sum(bh^3/12 + A (y - y_bar)^2); Z = Ix / distance to the fibre)
  bf = 0.12 m
  tf = 0.02 m
  h = 0.15 m
  tw = 0.015 m
  assumes: tf < h; tw <= bf; no fillets
```

In the worked example, bf = 120 mm, tf = 20 mm, h = 150 mm, tw = 15 mm go through the equation above and give A = 0.00435 m^2, y_bar = 0.1063793103 m, Ix = 8.877974138e-06 m^4, Iy = 2.9165625e-06 m^4. It applies only when tf < h; tw <= bf; no fillets: outside that it says what is wrong instead of returning a number.

<a id="angle_section_properties"></a>

#### Section properties of an angle (L) section, with principal axes — `angle_section_properties`

Area, centroid, centroidal second moments and product of inertia, principal second moments, the principal angle and the least radius of gyration of an equal or unequal angle.

$$
I_{1,2} = \frac{I_x+I_y}{2} \pm \sqrt{\Big(\frac{I_x-I_y}{2}\Big)^2 + I_{xy}^2},\quad \tan2\varphi = \frac{-2I_{xy}}{I_x-I_y}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | horizontal leg length | m |
| `h` | in | vertical leg length | m |
| `t` | in | thickness (both legs) | m |
| `A` | out | area | m^2 |
| `x_bar` | out | centroid distance from the back of the vertical leg | m |
| `y_bar` | out | centroid distance from the bottom of the horizontal leg | m |
| `Ix` | out | centroidal second moment about x | m^4 |
| `Iy` | out | centroidal second moment about y | m^4 |
| `Ixy` | out | centroidal product of inertia (integral of x y dA) | m^4 |
| `I1` | out | major principal second moment | m^4 |
| `I2` | out | minor principal second moment | m^4 |
| `principal_angle_deg` | out | angle of the major principal axis from x, degrees, in (-90, 90] | — |
| `r_min` | out | least radius of gyration | m |

**Applies when:** t < b and t < h; square corners, no root radius.

```shell
$ mechanical-engineering solve angle_section_properties "b=100 mm" "h=100 mm" "t=10 mm"
A = 0.0019 m^2
x_bar = 0.02868421053 m
y_bar = 0.02868421053 m
Ix = 1.80004386e-06 m^4
Iy = 1.80004386e-06 m^4
Ixy = -1.065789474e-06 m^4
I1 = 2.865833333e-06 m^4
I2 = 7.34254386e-07 m^4
principal_angle_deg = 45
r_min = 0.01965832335 m
  by: Section properties of an angle (L) section, with principal axes  (two rectangles with the parallel-axis theorem (also for Ixy); I1,2 = (Ix + Iy)/2 +- sqrt(((Ix - Iy)/2)^2 + Ixy^2), tan(2 phi) = -2 Ixy/(Ix - Iy))
  b = 0.1 m
  h = 0.1 m
  t = 0.01 m
  assumes: t < b and t < h; square corners, no root radius
```

**What the worked example means:** This calculation determines the geometric properties of an angle section defined by leg lengths 100 mm and 100 mm with thickness 10 mm. It computes the total area 0.0019 m^2 and locates the centroid relative to the outer edges at distances 0.0286842 m and 0.0286842 m. Using these values, it derives the second moments of area about the horizontal and vertical axes (1.80004e-06 m^4 and 1.80004e-06 m^4) and the product of inertia -1.06579e-06 m^4. From these, the major and minor principal moments of inertia (2.86583e-06 m^4 and 7.34254e-07 m^4) are found, along with the orientation of the major principal axis given by 45 degrees. The least radius of gyration, a key indicator for buckling resistance, is reported as 0.0196583 m.

### Statics (3)

<a id="moment_of_a_force"></a>

#### Moment of a force about a point (2D) — `moment_of_a_force`

The moment of a force (Fx, Fy) applied at (x, y) about the point (x0, y0), counterclockwise positive, and the perpendicular distance to its line of action.

$$
M_O = (x - x_0)F_y - (y - y_0)F_x,\quad d = \frac{|M_O|}{|\mathbf F|}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Fx` | in | x component of the force | N |
| `Fy` | in | y component of the force | N |
| `x` | in | x coordinate of the point of application | m |
| `y` | in | y coordinate of the point of application | m |
| `x0` | in | x coordinate of the moment centre (default 0) (optional) | m |
| `y0` | in | y coordinate of the moment centre (default 0) (optional) | m |
| `M` | out | moment about the point (counterclockwise positive) | N*m |
| `d` | out | perpendicular distance from the point to the line of action | m |

**Applies when:** the force is not zero.

```shell
$ mechanical-engineering solve moment_of_a_force "Fx=100 N" "Fy=200 N" "x=2 m" "y=1 m"
M = 300 N*m
d = 1.341640786 m
  by: Moment of a force about a point (2D)  (M = (x - x0) Fy - (y - y0) Fx, d = |M| / |F|)
  Fx = 100 N
  Fy = 200 N
  x = 2 m
  y = 1 m
  assumes: the force is not zero
```

**What the worked example means:** The moment is 300 N*m and the perpendicular distance is 1.34164 m.

<a id="resultant_of_concurrent_forces"></a>

#### Resultant of concurrent coplanar forces — `resultant_of_concurrent_forces`

The magnitude, components and direction of the resultant of forces acting through one point in a plane.

$$
R = \sqrt{\Big(\sum F_i\cos\theta_i\Big)^2 + \Big(\sum F_i\sin\theta_i\Big)^2},\quad \theta_R = \operatorname{atan2}(R_y, R_x)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | force magnitudes in N (a list) | — |
| `theta` | in | direction of each force in degrees, counterclockwise from +x (a list) | — |
| `R` | out | magnitude of the resultant | N |
| `Rx` | out | x component of the resultant | N |
| `Ry` | out | y component of the resultant | N |
| `direction_deg` | out | direction of the resultant in degrees, counterclockwise from +x, in (-180, 180] | — |

**Applies when:** F and theta have the same length; magnitudes >= 0, at least one > 0; a zero resultant is reported with direction 0.

```shell
$ mechanical-engineering solve resultant_of_concurrent_forces "F=[3, 4]" "theta=[0, 90]"
R = 5 N
Rx = 3 N
Ry = 4 N
direction_deg = 53.13010235
  by: Resultant of concurrent coplanar forces  (Rx = sum F_i cos(theta_i), Ry = sum F_i sin(theta_i), R = sqrt(Rx^2 + Ry^2), direction = atan2(Ry, Rx))
  assumes: F and theta have the same length; magnitudes >= 0, at least one > 0; a zero resultant is reported with direction 0
```

**What the worked example means:** This calculation determines the single equivalent force that replaces multiple concurrent coplanar forces. It sums the horizontal components to find 3 N and the vertical components to find 4 N. The final magnitude is computed as 5 N, while the direction is reported as 53.1301 degrees measured counterclockwise from the positive horizontal axis. This simplifies complex loading systems into a manageable net force for structural analysis.

<a id="two_cable_equilibrium"></a>

#### Tensions in two cables holding a weight — `two_cable_equilibrium`

The tensions in two cables, one rising to the left at angle alpha and one rising to the right at angle beta above the horizontal, that hold a weight W at rest.

$$
T_1 = \frac{W\cos\beta}{\sin(\alpha+\beta)},\quad T_2 = \frac{W\cos\alpha}{\sin(\alpha+\beta)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight held | N |
| `alpha` | in | angle of the left cable above the horizontal | rad |
| `beta` | in | angle of the right cable above the horizontal | rad |
| `T1` | out | tension in the left cable | N |
| `T2` | out | tension in the right cable | N |

**Applies when:** W > 0; 0 < alpha, beta < 90 degrees.

```shell
$ mechanical-engineering solve two_cable_equilibrium "W=500 N" "alpha=30 deg" "beta=45 deg"
T1 = 366.0254038 N
T2 = 448.2877361 N
  by: Tensions in two cables holding a weight  (Lami's theorem: T1 = W cos(beta)/sin(alpha + beta), T2 = W cos(alpha)/sin(alpha + beta))
  W = 500 N
  alpha = 0.5235987756 rad
  beta = 0.7853981634 rad
  assumes: W > 0; 0 < alpha, beta < 90 degrees
```

**What the worked example means:** This calculation determines the forces needed to keep a weight stationary using two angled cables. Applying Lami's theorem shows that the left cable bears a tension of 366.025 N, while the right cable carries 448.288 N. These values ensure static equilibrium for the suspended load of 500 N, given the specific angles of the cables relative to the horizontal.

### Stress analysis (5)

<a id="combined_axial_and_bending_stress"></a>

#### Combined axial force and biaxial bending of a rectangular section — `combined_axial_and_bending_stress`

Normal stresses at the four corners of a rectangular section carrying an axial force and bending moments about both axes, with the extreme values.

$$
\sigma = \frac{N}{A} + \frac{M_xy}{I_x} + \frac{M_yx}{I_y}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | width (along x) | m |
| `h` | in | depth (along y) | m |
| `N` | in | axial force (tension positive) | N |
| `Mx` | in | moment about x (positive: tension at +y) | N*m |
| `My` | in | moment about y (positive: tension at +x) | N*m |
| `sigma_corners_MPa` | out | stresses at the corners (+x,+y), (-x,+y), (-x,-y), (+x,-y), MPa | — |
| `sigma_max_MPa` | out | largest (most tensile) stress, MPa | — |
| `sigma_min_MPa` | out | smallest (most compressive) stress, MPa | — |
| `sigma_axial_MPa` | out | uniform axial part N/A, MPa | — |

**Applies when:** linear elastic, plane sections; moments about the centroidal axes.

```shell
$ mechanical-engineering solve combined_axial_and_bending_stress "b=100 mm" "h=200 mm" "N=-50 kN" "Mx=8 kN*m" "My=2 kN*m"
sigma_corners_MPa = [15.5,3.5,-20.5,-8.5]
sigma_max_MPa = 15.5
sigma_min_MPa = -20.5
sigma_axial_MPa = -2.5
  by: Combined axial force and biaxial bending of a rectangular section  (sigma(x, y) = N/A + Mx y/Ix + My x/Iy at the corners (x = +-b/2, y = +-h/2))
  b = 0.1 m
  h = 0.2 m
  N = -50000 N
  Mx = 8000 N*m
  My = 2000 N*m
  assumes: linear elastic, plane sections; moments about the centroidal axes
```

**What the worked example means:** This calculation determines the normal stresses at the four corners of a rectangular section subjected to an axial force and bending moments about both principal axes. The uniform axial stress, computed as the applied force divided by the cross-sectional area, equals -2.5. By superposing this uniform stress with the linearly varying bending stresses derived from the moments and section properties, the extreme values are found. The most tensile stress occurs at one corner and measures 15.5, whereas the most compressive stress at another corner is -20.5. These results allow for the assessment of structural integrity under combined loading conditions.

<a id="plane_stress_transformation"></a>

#### Plane stress transformation to rotated axes — `plane_stress_transformation`

The normal and shear stresses on axes rotated counterclockwise by theta from x, for a plane state of stress.

$$
\sigma_{x'} = \frac{\sigma_x+\sigma_y}{2} + \frac{\sigma_x-\sigma_y}{2}\cos2\theta + \tau_{xy}\sin2\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sx` | in | normal stress sigma_x | Pa |
| `sy` | in | normal stress sigma_y | Pa |
| `txy` | in | shear stress tau_xy | Pa |
| `theta` | in | rotation of the axes, counterclockwise from x | rad |
| `sx_t_MPa` | out | sigma_x' in MPa | — |
| `sy_t_MPa` | out | sigma_y' in MPa | — |
| `txy_t_MPa` | out | tau_x'y' in MPa | — |

**Applies when:** plane stress; tension positive, theta counterclockwise.

```shell
$ mechanical-engineering solve plane_stress_transformation "sx=80 MPa" "sy=-40 MPa" "txy=25 MPa" "theta=30 deg"
sx_t_MPa = 71.65063509
sy_t_MPa = -31.65063509
txy_t_MPa = -39.46152423
  by: Plane stress transformation to rotated axes  (sigma_x' = (sx + sy)/2 + (sx - sy)/2 cos 2theta + txy sin 2theta; tau_x'y' = -(sx - sy)/2 sin 2theta + txy cos 2theta)
  sx = 80000000 Pa
  sy = -40000000 Pa
  txy = 25000000 Pa
  theta = 0.5235987756 rad
  assumes: plane stress; tension positive, theta counterclockwise
```

**What the worked example means:** This calculation determines the state of stress on a plane oriented at an angle theta relative to the original axes. Applying the transformation equations shows that the normal stress in the new x direction is 71.6506, while the normal stress in the new y direction is -31.6506. The shear stress acting along this plane is calculated as -39.4615. These values allow engineers to assess potential failure modes on inclined surfaces such as welds or bolt interfaces.

<a id="principal_stresses_2d"></a>

#### Principal stresses and maximum shear (Mohr's circle, plane stress) — `principal_stresses_2d`

Principal stresses, their direction, the maximum in-plane and absolute shear stresses and the von Mises stress of a plane state of stress.

$$
\sigma_{1,2} = \frac{\sigma_x+\sigma_y}{2} \pm \sqrt{\Big(\frac{\sigma_x-\sigma_y}{2}\Big)^2 + \tau_{xy}^2},\quad \tan2\theta_p = \frac{2\tau_{xy}}{\sigma_x-\sigma_y}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sx` | in | normal stress sigma_x | Pa |
| `sy` | in | normal stress sigma_y | Pa |
| `txy` | in | shear stress tau_xy | Pa |
| `s1_MPa` | out | larger in-plane principal stress, MPa | — |
| `s2_MPa` | out | smaller in-plane principal stress, MPa | — |
| `tau_max_inplane_MPa` | out | maximum in-plane shear stress (radius of Mohr's circle), MPa | — |
| `theta_p_deg` | out | angle from x to the sigma_1 direction, degrees in (-90, 90] (0 when the circle is a point) | — |
| `tau_abs_max_MPa` | out | absolute maximum shear stress including sigma_z = 0, MPa | — |
| `von_mises_MPa` | out | von Mises equivalent stress, MPa | — |

**Applies when:** plane stress (sigma_z = 0); tension positive.

```shell
$ mechanical-engineering solve principal_stresses_2d "sx=50 MPa" "sy=-10 MPa" "txy=40 MPa"
s1_MPa = 70
s2_MPa = -30
tau_max_inplane_MPa = 50
theta_p_deg = 26.56505118
tau_abs_max_MPa = 50
von_mises_MPa = 88.88194417
  by: Principal stresses and maximum shear (Mohr's circle, plane stress)  (sigma_1,2 = (sx + sy)/2 +- R, R = sqrt(((sx - sy)/2)^2 + txy^2); tan 2theta_p = 2 txy/(sx - sy); tau_abs = max(s1 - s2, |s1|, |s2|)/2)
  sx = 50000000 Pa
  sy = -10000000 Pa
  txy = 40000000 Pa
  assumes: plane stress (sigma_z = 0); tension positive
```

**What the worked example means:** This calculation determines the principal stresses, their orientation, and maximum shear stresses for a plane stress state defined by normal stresses 50 MPa and -10 MPa and shear stress 40 MPa. The larger principal stress is 70 and the smaller is -30. The angle to the major principal plane is 26.5651. The radius of Mohr's circle gives the maximum in-plane shear stress of 50. Including the zero out-of-plane stress, the absolute maximum shear is 50. The von Mises equivalent stress, used to predict yielding in ductile materials, is 88.8819.

<a id="principal_stresses_3d"></a>

#### Principal stresses of a three-dimensional stress state — `principal_stresses_3d`

The three principal stresses, the absolute maximum shear, von Mises, hydrostatic and octahedral shear stresses of a general 3D state of stress.

$$
\det(\boldsymbol\sigma - \sigma\mathbf I) = 0,\quad \tau_{max} = \frac{\sigma_1-\sigma_3}{2},\quad \tau_{oct} = \frac{\sqrt2}{3}\sigma_{vm}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sx` | in | sigma_x | Pa |
| `sy` | in | sigma_y | Pa |
| `sz` | in | sigma_z | Pa |
| `txy` | in | tau_xy | Pa |
| `tyz` | in | tau_yz | Pa |
| `tzx` | in | tau_zx | Pa |
| `s1_MPa` | out | largest principal stress, MPa | — |
| `s2_MPa` | out | intermediate principal stress, MPa | — |
| `s3_MPa` | out | smallest principal stress, MPa | — |
| `tau_max_MPa` | out | absolute maximum shear stress (s1 - s3)/2, MPa | — |
| `von_mises_MPa` | out | von Mises stress, MPa | — |
| `hydrostatic_MPa` | out | mean (hydrostatic) stress, MPa | — |
| `tau_oct_MPa` | out | octahedral shear stress, MPa | — |

**Applies when:** symmetric stress tensor; tension positive.

```shell
$ mechanical-engineering solve principal_stresses_3d "sx=100 MPa" "sy=0 MPa" "sz=0 MPa" "txy=0 MPa" "tyz=0 MPa" "tzx=0 MPa"
s1_MPa = 100
s2_MPa = 0
s3_MPa = 0
tau_max_MPa = 50
von_mises_MPa = 100
hydrostatic_MPa = 33.33333333
tau_oct_MPa = 47.14045208
  by: Principal stresses of a three-dimensional stress state  (eigenvalues of the stress tensor by Jacobi rotations; sigma_vm = sqrt(((sx-sy)^2 + (sy-sz)^2 + (sz-sx)^2)/2 + 3(txy^2 + tyz^2 + tzx^2)); tau_oct = sqrt(2)/3 sigma_vm)
  sx = 100000000 Pa
  sy = 0 Pa
  sz = 0 Pa
  txy = 0 Pa
  tyz = 0 Pa
  tzx = 0 Pa
  assumes: symmetric stress tensor; tension positive
```

**What the worked example means:** This calculation determines the principal stresses, von Mises stress, hydrostatic stress, and octahedral shear stress for a general three-dimensional state of stress defined by normal and shear components. The principal stresses, reported as 100, 0, and 0, represent the magnitudes of normal stress on planes where shear stress is zero. These values are derived from the eigenvalues of the symmetric stress tensor. The absolute maximum shear stress is calculated as half the difference between the largest and smallest principal stresses, yielding 50. The von Mises equivalent stress, which predicts yielding under complex loading, is 100. The mean or hydrostatic stress, representing the volumetric component, is 33.3333, while the octahedral shear stress is 47.1405. Together, these metrics provide a complete description of the stress state's intensity and nature.

<a id="strain_rosette"></a>

#### Strain rosette: strains, principal strains and principal stresses — `strain_rosette`

From the readings of a three-gauge rosette (rectangular 0/45/90 or delta 0/60/120 degrees): the strain components, the principal strains and their direction, and the principal stresses (plane stress).

$$
\varepsilon_{1,2} = \frac{\varepsilon_x+\varepsilon_y}{2} \pm \sqrt{\Big(\frac{\varepsilon_x-\varepsilon_y}{2}\Big)^2 + \Big(\frac{\gamma_{xy}}{2}\Big)^2},\quad \sigma_1 = \frac{E(\varepsilon_1+\nu\varepsilon_2)}{1-\nu^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ea` | in | strain of gauge a (at 0 degrees), microstrain | — |
| `eb` | in | strain of gauge b (45 or 60 degrees), microstrain | — |
| `ec` | in | strain of gauge c (90 or 120 degrees), microstrain | — |
| `E` | in | Young's modulus | Pa |
| `nu` | in | Poisson's ratio | — |
| `layout` | in | rosette layout: rectangular (0/45/90) or delta (0/60/120) | — |
| `ex` | out | strain eps_x, microstrain | — |
| `ey` | out | strain eps_y, microstrain | — |
| `gxy` | out | shear strain gamma_xy, microstrain | — |
| `e1` | out | larger principal strain, microstrain | — |
| `e2` | out | smaller principal strain, microstrain | — |
| `theta_p_deg` | out | angle from gauge a to the eps_1 direction, degrees in (-90, 90] | — |
| `s1_MPa` | out | larger principal stress, MPa | — |
| `s2_MPa` | out | smaller principal stress, MPa | — |

**Applies when:** gauge angles measured counterclockwise from gauge a; plane stress, linear elastic isotropic; 0 <= nu < 0.5.

```shell
$ mechanical-engineering solve strain_rosette ea=400 eb=300 ec=-100 "E=200 GPa" nu=0.3 layout=rectangular
ex = 400
ey = -100
gxy = 300
e1 = 441.5475947
e2 = -141.5475947
theta_p_deg = 15.48187827
s1_MPa = 87.71061897
s2_MPa = -1.996333257
  by: Strain rosette: strains, principal strains and principal stresses  (rectangular: ex = ea, ey = ec, gxy = 2eb - ea - ec; delta: ex = ea, ey = (2eb + 2ec - ea)/3, gxy = 2(eb - ec)/sqrt(3); e1,2 = (ex+ey)/2 +- sqrt(((ex-ey)/2)^2 + (gxy/2)^2); s1 = E(e1 + nu e2)/(1 - nu^2))
  ea = 400
  eb = 300
  ec = -100
  E = 2e+11 Pa
  nu = 0.3
  layout = rectangular
  assumes: gauge angles measured counterclockwise from gauge a; plane stress, linear elastic isotropic; 0 <= nu < 0.5
```

**What the worked example means:** This calculation converts the measured surface strains from the three-gauge rosette into the state of strain at the point. Using the specific layout geometry, it determines the normal and shear strain components, which are then used to find the principal strains and their orientation. Finally, applying Hooke's law for plane stress, these strains are converted into principal stresses. The resulting larger principal strain is 441.548 microstrain, oriented at 15.4819 degrees from gauge a, corresponding to a principal stress of 87.7106 MPa.

### Stress and strain (3)

<a id="bar_fixed_at_both_ends"></a>

#### Bar fixed at both ends with an axial load at a step — `bar_fixed_at_both_ends`

Reactions, stresses in both parts and the displacement of the loaded joint of a two-part bar held between two rigid supports, loaded axially at the joint.

$$
R_A = P\frac{k_1}{k_1+k_2},\quad k_i = \frac{A_iE_i}{L_i},\quad \delta = \frac{P}{k_1+k_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | axial load at the joint (towards end B) | N |
| `L1` | in | length of part 1 (end A to the joint) | m |
| `A1` | in | area of part 1 | m^2 |
| `E1` | in | modulus of part 1 | Pa |
| `L2` | in | length of part 2 | m |
| `A2` | in | area of part 2 | m^2 |
| `E2` | in | modulus of part 2 | Pa |
| `RA` | out | reaction at A | N |
| `RB` | out | reaction at B | N |
| `stress1_MPa` | out | stress in part 1 in MPa (tension positive) | — |
| `stress2_MPa` | out | stress in part 2 in MPa (compression negative) | — |
| `displacement_mm` | out | displacement of the joint in mm | — |

**Applies when:** rigid supports, no gap; linear elastic, no buckling of the compressed part.

```shell
$ mechanical-engineering solve bar_fixed_at_both_ends "P=60 kN" "L1=0.4 m" "A1=500 mm^2" "E1=200 GPa" "L2=0.4 m" "A2=500 mm^2" "E2=200 GPa"
RA = 30000 N
RB = 30000 N
stress1_MPa = 60
stress2_MPa = -60
displacement_mm = 0.12
  by: Bar fixed at both ends with an axial load at a step  (compatibility: the joint moves the same amount in both parts; RA = P k1/(k1 + k2), k = AE/L)
  P = 60000 N
  L1 = 0.4 m
  A1 = 0.0005 m^2
  E1 = 2e+11 Pa
  L2 = 0.4 m
  A2 = 0.0005 m^2
  E2 = 2e+11 Pa
  assumes: rigid supports, no gap; linear elastic, no buckling of the compressed part
```

**What the worked example means:** This calculation determines the internal forces in a two-part bar fixed between rigid supports and loaded axially at the junction. By enforcing compatibility, the stiffness of each segment dictates how the applied load 60 kN is shared. The resulting reaction at support A is 30000 N, while the stress in the first segment is 60. The total displacement of the joint relative to its original position is 0.12, reflecting the combined elongation or contraction of both parts under equilibrium.

<a id="generalized_hookes_law"></a>

#### Generalized Hooke's law: strains from a 3D stress state — `generalized_hookes_law`

The six strain components and the volumetric strain of an isotropic linear-elastic material under a general three-dimensional stress state.

$$
\varepsilon_x = \frac{1}{E}\big[\sigma_x - \nu(\sigma_y+\sigma_z)\big],\quad \gamma_{xy} = \frac{\tau_{xy}}{G},\quad G = \frac{E}{2(1+\nu)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sx` | in | sigma_x | Pa |
| `sy` | in | sigma_y | Pa |
| `sz` | in | sigma_z | Pa |
| `txy` | in | tau_xy | Pa |
| `tyz` | in | tau_yz | Pa |
| `tzx` | in | tau_zx | Pa |
| `E` | in | Young's modulus | Pa |
| `nu` | in | Poisson's ratio | — |
| `ex` | out | eps_x, microstrain | — |
| `ey` | out | eps_y, microstrain | — |
| `ez` | out | eps_z, microstrain | — |
| `gxy` | out | gamma_xy, microstrain | — |
| `gyz` | out | gamma_yz, microstrain | — |
| `gzx` | out | gamma_zx, microstrain | — |
| `volumetric` | out | volumetric strain eps_x + eps_y + eps_z, microstrain | — |

**Applies when:** isotropic, linear elastic; -1 < nu < 0.5; small strains.

```shell
$ mechanical-engineering solve generalized_hookes_law "sx=100 MPa" "sy=50 MPa" "sz=0 MPa" "txy=30 MPa" "tyz=0 MPa" "tzx=0 MPa" "E=200 GPa" nu=0.3
ex = 425
ey = 100
ez = -225
gxy = 390
gyz = 0
gzx = 0
volumetric = 300
  by: Generalized Hooke's law: strains from a 3D stress state  (eps_x = (sx - nu(sy + sz))/E (and cyclic), gamma = tau/G with G = E/(2(1 + nu)))
  sx = 100000000 Pa
  sy = 50000000 Pa
  sz = 0 Pa
  txy = 30000000 Pa
  tyz = 0 Pa
  tzx = 0 Pa
  E = 2e+11 Pa
  nu = 0.3
  assumes: isotropic, linear elastic; -1 < nu < 0.5; small strains
```

**What the worked example means:** This calculation determines the deformation of an isotropic linear elastic material under a general three-dimensional stress state. Using the input stresses and material properties, it computes the normal strains in each direction and the corresponding shear strains. The resulting normal strain in the x direction is 425, while the shear strain in the xy plane is 390. These values represent the fractional change in length and angle distortion respectively, allowing engineers to predict how the structure will deform under load.

<a id="axial_bar_segments"></a>

#### Stepped bar under axial loads at its joints — `axial_bar_segments`

Internal axial force, normal stress of every segment and the total elongation of a bar fixed at its left end, made of segments in series, with axial loads at the joints.

$$
\delta = \sum_i \frac{N_iL_i}{A_iE},\quad \sigma_i = \frac{N_i}{A_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lengths` | in | segment lengths in mm, from the fixed end (a list) | — |
| `areas` | in | segment cross-sectional areas in mm^2 (a list) | — |
| `loads` | in | axial load at the far end of each segment in kN, positive pulling away from the support (a list) | — |
| `E` | in | Young's modulus (all segments) | Pa |
| `elongation_mm` | out | total elongation of the bar in mm (negative = shortening) | — |
| `N_kN` | out | internal axial force of each segment in kN (tension positive) | — |
| `stress_MPa` | out | normal stress of each segment in MPa | — |
| `max_abs_stress_MPa` | out | largest stress magnitude in MPa | — |

**Applies when:** equal-length lists; left end fixed, right end free; linear elastic, no buckling.

```shell
$ mechanical-engineering solve axial_bar_segments "lengths=[300, 500]" "areas=[400, 200]" "loads=[-20, 30]" "E=200 GPa"
elongation_mm = 0.4125
N_kN = [10.0,30.0]
stress_MPa = [25.0,150.0]
max_abs_stress_MPa = 150
  by: Stepped bar under axial loads at its joints  (N_i = sum of the loads beyond segment i; sigma_i = N_i/A_i; delta = sum N_i L_i/(A_i E))
  E = 2e+11 Pa
  assumes: equal-length lists; left end fixed, right end free; linear elastic, no buckling
```

**What the worked example means:** This calculation determines the total deformation and internal forces for a stepped bar fixed at one end. Using segment lengths, areas, and joint loads, it computes the axial force in each section as [10, 30] and the corresponding normal stress as [25, 150]. The resulting total elongation is 0.4125, while the highest magnitude stress observed is 150. These values indicate how much the structure stretches or shortens under the applied tension or compression.

### Structural analysis (2)

<a id="continuous_beam_three_moment"></a>

#### Continuous beam under uniform loads (three-moment equation) — `continuous_beam_three_moment`

The bending moments over the supports and the support reactions of a continuous beam of equal stiffness over 2 to 6 simply supported spans, each with its own uniform load.

$$
M_{i-1}L_i + 2M_i(L_i+L_{i+1}) + M_{i+1}L_{i+1} = -\frac{w_iL_i^3 + w_{i+1}L_{i+1}^3}{4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `spans` | in | span lengths in m, left to right (a list) | — |
| `w` | in | uniform load on each span in kN/m (a list) | — |
| `support_moments_kNm` | out | bending moment at each support in kN*m, end supports first and last (hogging negative) | — |
| `reactions_kN` | out | reaction at each support in kN | — |

**Applies when:** constant EI along the beam; simple supports, no settlement; end supports carry no moment.

```shell
$ mechanical-engineering solve continuous_beam_three_moment "spans=[6, 6]" "w=[10, 10]"
support_moments_kNm = [0.0,-45.0,0.0]
reactions_kN = [22.5,75.0,22.5]
  by: Continuous beam under uniform loads (three-moment equation)  (Clapeyron: M_(i-1) L_i + 2 M_i (L_i + L_(i+1)) + M_(i+1) L_(i+1) = -(w_i L_i^3 + w_(i+1) L_(i+1)^3)/4; reactions from the span shears)
  assumes: constant EI along the beam; simple supports, no settlement; end supports carry no moment
```

**What the worked example means:** This calculation determines the bending moments at each support and the vertical reaction forces for a continuous beam with constant stiffness. Using Clapeyron's three-moment equation, it processes the span lengths [6, 6] and uniform loads [10, 10] to produce the internal forces. The resulting support moments [0, -45, 0] indicate the hogging or sagging behavior over the piers, while the reactions [22.5, 75, 22.5] show how the load is transferred to the supports.

<a id="truss_member_forces"></a>

#### Member forces of a plane truss (method of joints) — `truss_member_forces`

The axial force in every member and the support reactions of a statically determinate plane pin-jointed truss on a pin and a roller, under joint loads.

$$
\sum_{\text{members at } j} F_m\,\hat{\mathbf e}_{m} + \mathbf R_j + \mathbf P_j = \mathbf 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nodes` | in | node coordinates, one row 'x y' per node in m (node 1 is the first row) | — |
| `members` | in | members as pairs of node numbers, e.g. 1-2, 2-3 | — |
| `pin` | in | node number of the pin support | — |
| `roller` | in | node number of the roller support (vertical reaction) | — |
| `loads` | in | joint loads, one row 'node Fx Fy' in kN (up and right positive) | — |
| `forces_kN` | out | member forces in kN in the order given (tension positive) | — |
| `pin_Rx_kN` | out | horizontal reaction at the pin, kN | — |
| `pin_Ry_kN` | out | vertical reaction at the pin, kN | — |
| `roller_Ry_kN` | out | vertical reaction at the roller, kN | — |
| `max_tension_kN` | out | largest tension, kN | — |
| `max_compression_kN` | out | largest compression (magnitude), kN | — |

**Applies when:** pin-jointed, loads at the joints; statically determinate: members + 3 = 2 x nodes, geometrically stable; roller on a horizontal surface.

```shell
$ mechanical-engineering solve truss_member_forces "nodes=0 0; 4 0; 2 3" "members=1-2, 2-3, 3-1" pin=1 roller=2 "loads=3 0 -10"
forces_kN = [3.33333333333,-6.00925212577,-6.00925212577]
pin_Rx_kN = 0
pin_Ry_kN = 5
roller_Ry_kN = 5
max_tension_kN = 3.333333333
max_compression_kN = 6.009252126
  by: Member forces of a plane truss (method of joints)  (equilibrium of every joint (sum Fx = 0, sum Fy = 0) as one linear system in the member forces and three reactions, solved by Gaussian elimination)
  assumes: pin-jointed, loads at the joints; statically determinate: members + 3 = 2 x nodes, geometrically stable; roller on a horizontal surface
```

**What the worked example means:** This calculation determines the axial force in every member and the support reactions for a statically determinate plane truss. By applying equilibrium conditions at each joint, the system solves for unknowns such as the pin horizontal reaction 0, pin vertical reaction 5, and roller vertical reaction 5. The resulting member forces, listed in the order provided, indicate whether each component is in tension or compression, which is critical for structural integrity.

### Torsion (5)

<a id="stepped_shaft_twist"></a>

#### Angle of twist of a stepped shaft — `stepped_shaft_twist`

Total angle of twist and the largest shear stress of a shaft made of solid circular segments in series carrying one torque.

$$
\varphi = \sum_i\frac{TL_i}{GJ_i},\quad J_i = \frac{\pi d_i^4}{32}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | torque carried by every segment | N*m |
| `lengths` | in | segment lengths in mm (a list) | — |
| `diameters` | in | segment diameters in mm (a list, same order) | — |
| `G` | in | shear modulus | Pa |
| `twist_rad` | out | total angle of twist | rad |
| `twist_deg` | out | total angle of twist in degrees | — |
| `tau_max_MPa` | out | largest shear stress (smallest segment) in MPa | — |

**Applies when:** equal-length lists; the same torque in every segment; stress concentrations at the steps not included.

```shell
$ mechanical-engineering solve stepped_shaft_twist "T=500 N*m" "lengths=[400, 600]" "diameters=[40, 30]" "G=80 GPa"
twist_rad = 0.05710420412 rad
twist_deg = 3.271829888
tau_max_MPa = 94.31404035
  by: Angle of twist of a stepped shaft  (phi = sum T L_i/(G J_i), J_i = pi d_i^4/32, tau_max = max 16T/(pi d_i^3))
  T = 500 N*m
  G = 8e+10 Pa
  assumes: equal-length lists; the same torque in every segment; stress concentrations at the steps not included
```

**What the worked example means:** This calculation determines the deformation and internal stress of a stepped shaft subjected to a constant torque. By summing the angular displacement contributed by each segment based on its length and diameter, the model finds that the shaft twists by 0.0571042 rad radians, which equals 3.27183 degrees. The analysis also identifies the location of highest risk for failure; because shear stress increases as the shaft diameter decreases, the largest stress occurs in the smallest segment, reaching a value of 94.314 megapascals. This result assumes the material behaves linearly elastically and that the torque remains unchanged throughout the entire length of the shaft.

<a id="statically_indeterminate_shaft"></a>

#### Shaft fixed at both ends with a torque between them — `statically_indeterminate_shaft`

The reaction torques at both fixed ends, the shear stress in each part and the rotation at the loaded section of a shaft built in at both ends.

$$
T_A = T_0\frac{b/J_2}{a/J_1 + b/J_2},\quad T_B = T_0 - T_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T0` | in | torque applied between the ends | N*m |
| `a` | in | length of part 1 (from end A to the load) | m |
| `b` | in | length of part 2 (from the load to end B) | m |
| `d1` | in | diameter of part 1 | m |
| `d2` | in | diameter of part 2 | m |
| `G` | in | shear modulus | Pa |
| `TA` | out | reaction torque at A | N*m |
| `TB` | out | reaction torque at B | N*m |
| `tau1_MPa` | out | maximum shear stress in part 1, MPa | — |
| `tau2_MPa` | out | maximum shear stress in part 2, MPa | — |
| `twist_at_load_deg` | out | rotation of the loaded section in degrees | — |

**Applies when:** both ends fully fixed; solid circular parts of one material.

```shell
$ mechanical-engineering solve statically_indeterminate_shaft "T0=1 kN*m" "a=0.5 m" "b=1.5 m" "d1=50 mm" "d2=50 mm" "G=80 GPa"
TA = 750 N*m
TB = 250 N*m
tau1_MPa = 30.55774907
tau2_MPa = 10.18591636
twist_at_load_deg = 0.4377075133
  by: Shaft fixed at both ends with a torque between them  (compatibility: TA a/(G J1) = TB b/(G J2), equilibrium TA + TB = T0)
  T0 = 1000 N*m
  a = 0.5 m
  b = 1.5 m
  d1 = 0.05 m
  d2 = 0.05 m
  G = 8e+10 Pa
  assumes: both ends fully fixed; solid circular parts of one material
```

**What the worked example means:** This calculation determines the internal forces in a shaft fixed at both ends with an applied torque between them. Using compatibility of deformation and equilibrium, the system solves for the reaction torques at each support. For the given inputs, the reaction torque at end A is 750 N*m and at end B is 250 N*m. The maximum shear stress in the first segment is 30.5577 and in the second segment is 10.1859. The resulting angular twist at the point of load application is 0.437708. These values indicate how the external torque distributes based on the relative stiffness of each shaft segment.

<a id="shaft_power_transmission"></a>

#### Torque and minimum diameter of a power-transmission shaft — `shaft_power_transmission`

The torque a solid shaft transmits at a given power and speed, and the smallest diameter that keeps the torsional shear stress within an allowable value.

$$
T = \frac{P}{\omega},\quad d_{min} = \Big(\frac{16T}{\pi\tau_{allow}}\Big)^{1/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | transmitted power | W |
| `n` | in | rotational speed (rpm or rad/s) | rad/s |
| `tau_allow` | in | allowable shear stress | Pa |
| `T` | out | transmitted torque | N*m |
| `d_min_mm` | out | minimum solid-shaft diameter in mm | — |
| `frequency_Hz` | out | rotational frequency in Hz | — |

**Applies when:** P, n, tau_allow > 0; pure torsion (no bending), solid circular shaft.

```shell
$ mechanical-engineering solve shaft_power_transmission "P=50 kW" "n=1500 rpm" "tau_allow=40 MPa"
T = 318.3098862 N*m
d_min_mm = 34.34947329
frequency_Hz = 25
  by: Torque and minimum diameter of a power-transmission shaft  (T = P/omega, d_min = (16T/(pi tau_allow))^(1/3))
  P = 50000 W
  n = 157.0796327 rad/s
  tau_allow = 40000000 Pa
  assumes: P, n, tau_allow > 0; pure torsion (no bending), solid circular shaft
```

**What the worked example means:** This calculation determines the torque transmitted by a solid circular shaft given the power and rotational speed, then finds the smallest diameter that keeps shear stress below the allowable limit. The resulting torque is 318.31 N*m, which dictates the required minimum shaft diameter of 34.3495 to ensure structural integrity under pure torsion.

<a id="hollow_shaft_torsion"></a>

#### Torsion of a hollow circular shaft — `hollow_shaft_torsion`

Shear stresses at the outer and inner surfaces, polar moment and angle of twist of a hollow circular shaft under a torque.

$$
\tau_{max} = \frac{16TD}{\pi(D^4-d^4)},\quad \varphi = \frac{TL}{GJ}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | applied torque | N*m |
| `D` | in | outer diameter | m |
| `d` | in | inner diameter | m |
| `L` | in | length | m |
| `G` | in | shear modulus | Pa |
| `tau_max_MPa` | out | maximum shear stress (outer surface) in MPa | — |
| `tau_inner_MPa` | out | shear stress at the inner surface in MPa (0 for a solid shaft) | — |
| `twist_rad` | out | angle of twist | rad |
| `twist_deg` | out | angle of twist in degrees | — |
| `J` | out | polar second moment of area | m^4 |

**Applies when:** D > d > 0; linear elastic.

```shell
$ mechanical-engineering solve hollow_shaft_torsion "T=2 kN*m" "D=60 mm" "d=40 mm" "L=2 m" "G=80 GPa"
tau_max_MPa = 58.76490206
tau_inner_MPa = 39.17660138
twist_rad = 0.04897075172 rad
twist_deg = 2.805817393
J = 1.021017612e-06 m^4
  by: Torsion of a hollow circular shaft  (J = pi (D^4 - d^4)/32, tau_max = T (D/2)/J, tau_inner = T (d/2)/J, phi = TL/(GJ))
  T = 2000 N*m
  D = 0.06 m
  d = 0.04 m
  L = 2 m
  G = 8e+10 Pa
  assumes: D > d > 0; linear elastic
```

**What the worked example means:** This calculation determines the mechanical response of a hollow circular shaft under torsion. Using the applied torque, dimensions, and material properties, it computes the polar second moment of area as 1.02102e-06 m^4, which quantifies geometric resistance to twisting. The resulting maximum shear stress at the outer surface is 58.7649, while the stress at the inner surface is 39.1766. The total rotation over the shaft length is expressed as an angle of twist of 2.80582 degrees.

<a id="solid_shaft_torsion"></a>

#### Torsion of a solid circular shaft — `solid_shaft_torsion`

Maximum shear stress, polar moment and angle of twist of a solid circular shaft under a torque.

$$
\tau_{max} = \frac{16T}{\pi d^3},\quad \varphi = \frac{TL}{GJ},\quad J = \frac{\pi d^4}{32}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | applied torque | N*m |
| `d` | in | diameter | m |
| `L` | in | length | m |
| `G` | in | shear modulus | Pa |
| `tau_max_MPa` | out | maximum shear stress (outer surface) in MPa | — |
| `tau_inner_MPa` | out | shear stress at the inner surface in MPa (0 for a solid shaft) | — |
| `twist_rad` | out | angle of twist | rad |
| `twist_deg` | out | angle of twist in degrees | — |
| `J` | out | polar second moment of area | m^4 |

**Applies when:** T, d, L, G > 0; linear elastic, circular section, uniform torque.

```shell
$ mechanical-engineering solve solid_shaft_torsion "T=1.2 kN*m" "d=50 mm" "L=1.5 m" "G=80 GPa"
tau_max_MPa = 48.89239852
tau_inner_MPa = 0
twist_rad = 0.03666929889 rad
twist_deg = 2.100996064
J = 6.135923152e-07 m^4
  by: Torsion of a solid circular shaft  (J = pi d^4/32, tau_max = T (d/2)/J = 16T/(pi d^3), phi = TL/(GJ))
  T = 1200 N*m
  d = 0.05 m
  L = 1.5 m
  G = 8e+10 Pa
  assumes: T, d, L, G > 0; linear elastic, circular section, uniform torque
```

**What the worked example means:** This calculation determines the structural response of a solid circular shaft under torsion. Using an applied torque of 1.2 kN*m, a diameter of 50 mm, a length of 1.5 m, and a shear modulus of 80 GPa, it computes the polar second moment of area as 6.13592e-07 m^4. The resulting maximum shear stress on the outer surface is 48.8924, while the shear stress at the center is 0. The total angular deformation, or angle of twist, is calculated to be 0.0366693 rad radians, which equals 2.101 degrees.

### Vibrations (14)

<a id="shaft_whirling_speeds"></a>

#### All critical (whirling) speeds of a shaft with several disks — `shaft_whirling_speeds`

The exact critical speeds of a simply supported massless shaft carrying point-mass disks, from the eigenvalues of its flexibility matrix.

$$
\det\Big(\mathbf A\mathbf M - \frac{1}{\omega^2}\mathbf I\Big) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `masses` | in | disk masses in kg (a list) | — |
| `positions` | in | disk positions from the left bearing, in m (a list) | — |
| `L` | in | bearing span | m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area of the shaft | m^4 |
| `omega` | out | critical speeds in rad/s, ascending | — |
| `N_rpm` | out | critical speeds in rpm | — |

**Applies when:** simply supported uniform massless shaft, point-mass disks (no rotary inertia or gyroscopics); distinct positions.

```shell
$ mechanical-engineering solve shaft_whirling_speeds "masses=[50]" "positions=[0.5]" "L=1 m" "E=200 GPa" "I=2e-07 m^4"
omega = [195.959179423]
N_rpm = [1871.27232296]
  by: All critical (whirling) speeds of a shaft with several disks  (det(A M - (1/omega^2) I) = 0 with the influence coefficients a_ij; eigenvalues of the symmetric M^(1/2) A M^(1/2) by Jacobi rotations)
  L = 1 m
  E = 2e+11 Pa
  I = 2e-07 m^4
  assumes: simply supported uniform massless shaft, point-mass disks (no rotary inertia or gyroscopics); distinct positions
```

**What the worked example means:** This calculation determines the exact natural frequencies of lateral vibration for a simply supported massless shaft carrying discrete point-mass disks. By constructing the flexibility matrix based on the disk positions [0.5], span 1 m, modulus 200 GPa, and inertia 2e-07 m^4, and combining it with the diagonal mass matrix from [50], the method solves an eigenvalue problem. The resulting values [195.959] represent the angular velocities in radians per second where resonance occurs, while [1871.27] converts these into revolutions per minute for practical operational assessment.

<a id="shaft_critical_speed_single_disk"></a>

#### Critical speed of a shaft carrying one disk — `shaft_critical_speed_single_disk`

The lateral stiffness at the disk, the first critical (whirling) speed and the static deflection of a simply supported massless shaft carrying one disk.

$$
k = \frac{3EIL}{a^2b^2},\quad \omega_{cr} = \sqrt{\frac{k}{m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | disk mass | kg |
| `E` | in | Young's modulus of the shaft | Pa |
| `I` | in | second moment of area of the shaft | m^4 |
| `L` | in | bearing span | m |
| `a` | in | distance of the disk from the left bearing | m |
| `k` | out | lateral stiffness of the shaft at the disk | N/m |
| `omega_cr` | out | critical speed | rad/s |
| `N_cr_rpm` | out | critical speed in rpm | — |
| `static_deflection_mm` | out | static deflection under the disk's weight, mm | — |

**Applies when:** simple (short) bearings, uniform massless shaft; gyroscopic effects neglected; 0 < a < L.

```shell
$ mechanical-engineering solve shaft_critical_speed_single_disk "m=50 kg" "E=200 GPa" "I=2e-07 m^4" "L=1 m" "a=0.5 m"
k = 1920000 N/m
omega_cr = 195.9591794 rad/s
N_cr_rpm = 1871.272323
static_deflection_mm = 0.2553815104
  by: Critical speed of a shaft carrying one disk  (k = 3 E I L/(a^2 b^2), omega_cr = sqrt(k/m))
  m = 50 kg
  E = 2e+11 Pa
  I = 2e-07 m^4
  L = 1 m
  a = 0.5 m
  assumes: simple (short) bearings, uniform massless shaft; gyroscopic effects neglected; 0 < a < L
```

**What the worked example means:** This calculation determines the first lateral natural frequency of a simply supported shaft carrying a single disk. Using the disk mass 50 kg, shaft Young's modulus 200 GPa, second moment of area 2e-07 m^4, bearing span 1 m, and disk position 0.5 m, it computes the effective lateral stiffness at the disk as 1.92e+06 N/m. The resulting critical angular velocity is 195.959 rad/s rad/s, which converts to 1871.27 revolutions per minute. The static deflection under the disk's own weight is calculated as 0.255382 millimeters.

<a id="damped_sdof_parameters"></a>

#### Damping ratio, damped frequency and logarithmic decrement — `damped_sdof_parameters`

The natural frequency, damping ratio, critical damping, damped natural frequency and logarithmic decrement of a viscously damped spring-mass system, and whether it is under- or overdamped.

$$
\zeta = \frac{c}{2\sqrt{km}},\quad \omega_d = \omega_n\sqrt{1-\zeta^2},\quad \delta = \frac{2\pi\zeta}{\sqrt{1-\zeta^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `c` | in | viscous damping coefficient (N*s/m) | kg/s |
| `k` | in | stiffness | N/m |
| `omega_n` | out | undamped natural frequency | rad/s |
| `zeta` | out | damping ratio c/c_c | — |
| `omega_d` | out | damped natural frequency (0 if not underdamped) | rad/s |
| `f_d` | out | damped natural frequency in Hz | Hz |
| `c_critical` | out | critical damping 2 sqrt(k m) | kg/s |
| `log_decrement` | out | logarithmic decrement (0 if not underdamped) | — |
| `regime` | out | underdamped, critically damped or overdamped | — |

**Applies when:** m, k > 0, c >= 0; linear viscous damping.

```shell
$ mechanical-engineering solve damped_sdof_parameters "m=10 kg" "c=40 N*s/m" "k=4000 N/m"
omega_n = 20 rad/s
zeta = 0.1
omega_d = 19.89974874 rad/s
f_d = 3.167143379 Hz
c_critical = 400 kg/s
log_decrement = 0.6314838834
regime = underdamped
  by: Damping ratio, damped frequency and logarithmic decrement  (zeta = c/(2 sqrt(k m)), omega_d = omega_n sqrt(1 - zeta^2), delta = 2 pi zeta/sqrt(1 - zeta^2))
  m = 10 kg
  c = 40 kg/s
  k = 4000 N/m
  assumes: m, k > 0, c >= 0; linear viscous damping
```

**What the worked example means:** This calculation determines the vibrational characteristics of a viscously damped spring-mass system. Using the mass 10 kg, stiffness 4000 N/m, and damping coefficient 40 N*s/m gives the critical damping value 400 kg/s and the resulting damping ratio 0.1. The undamped natural frequency is found to be 20 rad/s, while the actual oscillation frequency under these conditions is 19.8997 rad/s. If the system is underdamped, the logarithmic decrement 0.631484 quantifies how rapidly the amplitude decays with each cycle. These values classify the motion as underdamped, indicating whether the system will oscillate or return to equilibrium without overshooting.

<a id="dunkerley_critical_speed"></a>

#### First critical speed of a shaft with several disks (Dunkerley) — `dunkerley_critical_speed`

Dunkerley's lower-bound estimate of the first critical speed of a simply supported massless shaft carrying several disks.

$$
\frac{1}{\omega_{cr}^2} = \sum_i m_ia_{ii},\quad a_{ii} = \frac{a_i^2b_i^2}{3EIL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `masses` | in | disk masses in kg (a list) | — |
| `positions` | in | disk positions from the left bearing, in m (a list) | — |
| `L` | in | bearing span | m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area of the shaft | m^4 |
| `omega_cr` | out | first critical speed (Dunkerley, a lower bound) | rad/s |
| `N_cr_rpm` | out | critical speed in rpm | — |

**Applies when:** simply supported uniform massless shaft; disks at distinct positions inside the span.

```shell
$ mechanical-engineering solve dunkerley_critical_speed "masses=[50]" "positions=[0.5]" "L=1 m" "E=200 GPa" "I=2e-07 m^4"
omega_cr = 195.9591794 rad/s
N_cr_rpm = 1871.272323
  by: First critical speed of a shaft with several disks (Dunkerley)  (1/omega^2 = sum m_i a_ii with a_ii = b_i^2 a_i^2/(3 E I L))
  L = 1 m
  E = 2e+11 Pa
  I = 2e-07 m^4
  assumes: simply supported uniform massless shaft; disks at distinct positions inside the span
```

**What the worked example means:** This calculation determines the first critical speed of a simply supported shaft carrying multiple disks using Dunkerley's method. It combines the individual flexibility contributions of each disk based on their masses and positions along the span defined by length L, modulus E, and moment of area I. The resulting angular frequency 195.959 rad/s represents a lower-bound estimate for the natural frequency where resonance occurs, which converts to rotational speed 1871.27 in revolutions per minute.

<a id="rayleigh_critical_speed"></a>

#### First critical speed of a shaft with several disks (Rayleigh) — `rayleigh_critical_speed`

Rayleigh's estimate of the first critical speed of a simply supported massless shaft carrying several disks, from the static deflections under the disks' weights.

$$
\omega_{cr}^2 = \frac{g\sum m_iy_i}{\sum m_iy_i^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `masses` | in | disk masses in kg (a list) | — |
| `positions` | in | disk positions from the left bearing, in m (a list) | — |
| `L` | in | bearing span | m |
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area of the shaft | m^4 |
| `omega_cr` | out | first critical speed (Rayleigh, an upper bound) | rad/s |
| `N_cr_rpm` | out | critical speed in rpm | — |
| `deflections_mm` | out | static deflection at each disk, mm | — |

**Applies when:** simply supported uniform massless shaft; disks at distinct positions inside the span; g = 9.80665 m/s^2.

```shell
$ mechanical-engineering solve rayleigh_critical_speed "masses=[50]" "positions=[0.5]" "L=1 m" "E=200 GPa" "I=2e-07 m^4"
omega_cr = 195.9591794 rad/s
N_cr_rpm = 1871.272323
deflections_mm = [0.255381510417]
  by: First critical speed of a shaft with several disks (Rayleigh)  (y_i = sum_j a_ij m_j g (influence coefficients of the simply supported beam); omega^2 = g sum m_i y_i / sum m_i y_i^2)
  L = 1 m
  E = 2e+11 Pa
  I = 2e-07 m^4
  assumes: simply supported uniform massless shaft; disks at distinct positions inside the span; g = 9.80665 m/s^2
```

**What the worked example means:** This calculation determines the first critical speed of a simply supported shaft using Rayleigh's method. It computes static deflections at each disk position based on beam theory and input stiffness properties. These deflections are used to approximate the fundamental mode shape, resulting in an estimated angular velocity of 195.959 rad/s rad/s and a rotational speed of 1871.27 rpm. The individual static deflections contributing to this result are [0.255382].

<a id="vibration_transmissibility"></a>

#### Force (or base-motion) transmissibility of a vibration isolator — `vibration_transmissibility`

The ratio of transmitted to applied force amplitude (equally, of mass to base displacement amplitude) for a damped isolator at a frequency ratio, and the isolation achieved.

$$
TR = \sqrt{\frac{1 + (2\zeta r)^2}{(1-r^2)^2 + (2\zeta r)^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `zeta` | in | damping ratio | — |
| `r` | in | frequency ratio omega/omega_n | — |
| `TR` | out | transmissibility | — |
| `isolation_percent` | out | isolation (1 - TR) in percent (0 when TR >= 1) | — |

**Applies when:** zeta >= 0, r > 0; not undamped resonance (r = 1 with zeta = 0).

```shell
$ mechanical-engineering solve vibration_transmissibility zeta=0 r=2
TR = 0.3333333333
isolation_percent = 66.66666667
  by: Force (or base-motion) transmissibility of a vibration isolator  (TR = sqrt((1 + (2 zeta r)^2)/((1 - r^2)^2 + (2 zeta r)^2)))
  zeta = 0
  r = 2
  assumes: zeta >= 0, r > 0; not undamped resonance (r = 1 with zeta = 0)
```

**What the worked example means:** This calculation determines the force transmissibility of a vibration isolator using the damping ratio 0 and frequency ratio 2. The resulting transmissibility value of 0.333333 represents the ratio of transmitted force to applied force. From this, the isolation percentage is calculated as 66.6667, indicating how effectively the system reduces vibration transmission relative to the input.

<a id="sdof_free_response"></a>

#### Free vibration response of a damped spring-mass system — `sdof_free_response`

The displacement and velocity at time t of a damped single-degree-of-freedom system released from an initial displacement and velocity (under-, critically or overdamped).

$$
x(t) = e^{-\zeta\omega_nt}\Big(x_0\cos\omega_dt + \frac{v_0+\zeta\omega_nx_0}{\omega_d}\sin\omega_dt\Big)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `c` | in | damping coefficient | kg/s |
| `k` | in | stiffness | N/m |
| `x0` | in | initial displacement | m |
| `v0` | in | initial velocity | m/s |
| `t` | in | time | s |
| `x` | out | displacement at t | m |
| `x_mm` | out | displacement at t in mm | — |
| `v` | out | velocity at t | m/s |

**Applies when:** linear viscous damping; no external force; zeta within 1e-9 of 1 is treated as critical.

```shell
$ mechanical-engineering solve sdof_free_response "m=1 kg" "c=20 N*s/m" "k=100 N/m" "x0=10 mm" "v0=0 m/s" "t=0.1 s"
x = 0.007357588823 m
x_mm = 7.357588823
v = -0.03678794412 m/s
  by: Free vibration response of a damped spring-mass system  (underdamped x = e^(-zeta wn t)(x0 cos wd t + (v0 + zeta wn x0)/wd sin wd t); critical x = (x0 + (v0 + wn x0) t) e^(-wn t); overdamped sum of two exponentials)
  m = 1 kg
  c = 20 kg/s
  k = 100 N/m
  x0 = 0.01 m
  v0 = 0 m/s
  t = 0.1 s
  assumes: linear viscous damping; no external force; zeta within 1e-9 of 1 is treated as critical
```

**What the worked example means:** This calculation determines the position and speed of a mass attached to a spring at time 0.1 s, starting from an initial displacement of 10 mm and velocity of 0 m/s. The system has a mass of 1 kg, stiffness of 100 N/m, and damping coefficient of 20 N*s/m. The resulting displacement is 0.00735759 m and the velocity is -0.0367879 m/s. This response reflects how the stored energy dissipates over time due to damping.

<a id="mdof_spring_mass_chain"></a>

#### Natural frequencies and first mode of a spring-mass chain — `mdof_spring_mass_chain`

All natural frequencies and the first mode shape of a chain of masses joined by springs, fixed to a wall at the left end (and optionally at the right end).

$$
(\mathbf K - \omega^2\mathbf M)\boldsymbol\phi = \mathbf 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `masses` | in | masses in kg, left to right (a list) | — |
| `springs` | in | spring stiffnesses in N/m, the wall spring first (a list) | — |
| `ends` | in | fixed-free (n springs) or fixed-fixed (n + 1 springs) | — |
| `omega` | out | natural circular frequencies in rad/s, ascending | — |
| `f_Hz` | out | natural frequencies in Hz | — |
| `mode1` | out | first mode shape, largest entry +1 | — |

**Applies when:** 1 to 12 masses; linear springs, no damping; springs: n (fixed-free) or n + 1 (fixed-fixed).

```shell
$ mechanical-engineering solve mdof_spring_mass_chain "masses=[2, 1]" "springs=[2000, 1000]" ends=fixed-free
omega = [22.360679775,44.72135955]
f_Hz = [3.55881271709,7.11762543417]
mode1 = [0.5,1.0]
  by: Natural frequencies and first mode of a spring-mass chain  (K phi = w^2 M phi with tridiagonal K; solved as the symmetric M^(-1/2) K M^(-1/2) by Jacobi rotations)
  ends = fixed-free
  assumes: 1 to 12 masses; linear springs, no damping; springs: n (fixed-free) or n + 1 (fixed-fixed)
```

**What the worked example means:** This calculation determines the natural circular frequencies in radians per second, listed as [22.3607, 44.7214], and converts them to Hertz as [3.55881, 7.11763] for a chain of masses with stiffnesses [2000, 1000] and inertias [2, 1]. The system configuration is defined by the boundary condition fixed-free. The resulting first mode shape, normalized so its largest entry is positive one, is given as [0.5, 1]. These values describe how the structure vibrates freely without external forces.

<a id="beam_natural_frequency"></a>

#### Natural frequencies of a uniform beam (Euler-Bernoulli) — `beam_natural_frequency`

The n-th bending natural frequency of a uniform Euler-Bernoulli beam with simply supported, cantilever, fixed-fixed, fixed-pinned or free-free ends.

$$
\omega_n = (\beta_nL)^2\sqrt{\frac{EI}{\rho AL^4}},\quad \cos\beta L\cosh\beta L = -1\ \text{(cantilever)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | Young's modulus | Pa |
| `I` | in | second moment of area | m^4 |
| `rho` | in | density | kg/m^3 |
| `A` | in | cross-sectional area | m^2 |
| `L` | in | length | m |
| `ends` | in | end conditions | — |
| `n` | in | mode number (default 1; free-free counts the elastic modes) (optional) | — |
| `beta_L` | out | eigenvalue beta L | — |
| `omega` | out | natural circular frequency | rad/s |
| `f` | out | natural frequency | Hz |

**Applies when:** slender uniform beam, no shear deformation or rotary inertia; mode 1 to 6.

```shell
$ mechanical-engineering solve beam_natural_frequency "E=200 GPa" "I=1e-08 m^4" "rho=7850 kg/m^3" "A=1000 mm^2" "L=2 m" ends=simply-supported
beta_L = 3.141592654
omega = 39.38400914 rad/s
f = 6.268159733 Hz
  by: Natural frequencies of a uniform beam (Euler-Bernoulli)  (omega_n = (beta_n L)^2 sqrt(E I/(rho A L^4)); beta L from sin = 0 (SS), cos cosh = -1 (cantilever), cos cosh = 1 (fixed-fixed, free-free), tan = tanh (fixed-pinned))
  E = 2e+11 Pa
  I = 1e-08 m^4
  rho = 7850 kg/m^3
  A = 0.001 m^2
  L = 2 m
  ends = simply-supported
  assumes: slender uniform beam, no shear deformation or rotary inertia; mode 1 to 6
```

**What the worked example means:** This calculation determines the bending natural frequency of a uniform beam using Euler-Bernoulli theory. By applying the specific transcendental equation for the given end conditions, the dimensionless eigenvalue 3.14159 is found. This value is then scaled by the material stiffness and geometric properties to yield the circular frequency 39.384 rad/s rad/s and the corresponding frequency in Hertz 6.26816 Hz Hz.

<a id="sdof_natural_frequency"></a>

#### Natural frequency of a spring-mass system — `sdof_natural_frequency`

The natural circular frequency, frequency, period and static deflection (under gravity) of an undamped single-degree-of-freedom spring-mass system.

$$
\omega_n = \sqrt{\frac{k}{m}},\quad f_n = \frac{1}{2\pi}\sqrt{\frac{k}{m}} = \frac{1}{2\pi}\sqrt{\frac{g}{\delta_{st}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `k` | in | spring stiffness | N/m |
| `omega_n` | out | natural circular frequency | rad/s |
| `f_n` | out | natural frequency | Hz |
| `period` | out | period | s |
| `static_deflection_mm` | out | static deflection m g/k in mm (g = 9.80665 m/s^2) | — |

**Applies when:** m, k > 0; linear spring, no damping.

```shell
$ mechanical-engineering solve sdof_natural_frequency "m=20 kg" "k=8 kN/m"
omega_n = 20 rad/s
f_n = 3.183098862 Hz
period = 0.3141592654 s
static_deflection_mm = 24.516625
  by: Natural frequency of a spring-mass system  (omega_n = sqrt(k/m), f_n = omega_n/(2 pi), T = 1/f_n, delta_st = m g/k)
  m = 20 kg
  k = 8000 N/m
  assumes: m, k > 0; linear spring, no damping
```

**What the worked example means:** This calculation determines the dynamic properties of an undamped single-degree-of-freedom spring-mass system. Using a mass of 20 kg and a spring stiffness of 8 kN/m, it computes the natural circular frequency as 20 rad/s radians per second, the natural frequency as 3.1831 Hz hertz, and the period as 0.314159 s seconds. It also finds the static deflection under gravity to be 24.5166, which represents how far the spring stretches when the mass hangs at rest.

<a id="forced_vibration_amplitude"></a>

#### Steady-state response to a harmonic force — `forced_vibration_amplitude`

The steady-state amplitude, phase lag and dynamic magnification factor of a damped spring-mass system driven by a harmonic force F0 cos(omega t).

$$
X = \frac{F_0/k}{\sqrt{(1-r^2)^2 + (2\zeta r)^2}},\quad \tan\phi = \frac{2\zeta r}{1-r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `c` | in | damping coefficient | kg/s |
| `k` | in | stiffness | N/m |
| `F0` | in | force amplitude | N |
| `omega` | in | forcing frequency (rad/s or rpm) | rad/s |
| `X` | out | steady-state amplitude | m |
| `X_mm` | out | amplitude in mm | — |
| `phase_deg` | out | phase lag of the displacement behind the force, degrees (0 to 180) | — |
| `magnification` | out | dynamic magnification X k/F0 | — |
| `r` | out | frequency ratio omega/omega_n | — |

**Applies when:** steady state (transient decayed); not undamped resonance.

```shell
$ mechanical-engineering solve forced_vibration_amplitude "m=10 kg" "c=40 N*s/m" "k=4000 N/m" "F0=100 N" "omega=20 rad/s"
X = 0.125 m
X_mm = 125
phase_deg = 90
magnification = 5
r = 1
  by: Steady-state response to a harmonic force  (X = F0/sqrt((k - m w^2)^2 + (c w)^2), phi = atan2(c w, k - m w^2))
  m = 10 kg
  c = 40 kg/s
  k = 4000 N/m
  F0 = 100 N
  omega = 20 rad/s
  assumes: steady state (transient decayed); not undamped resonance
```

**What the worked example means:** This calculation determines the long-term vibration response of a damped mass-spring system under harmonic forcing. Using the input mass, damping coefficient, stiffness, force amplitude, and driving frequency, it computes the resulting displacement amplitude 0.125 m, which equals 125 in millimeters. The output also includes the phase angle 90, indicating how much the motion lags behind the applied force, and the dynamic magnification factor 5. These values help predict whether the machine will vibrate excessively or operate safely near its natural frequency.

<a id="vibration_isolator_design"></a>

#### Stiffness of an undamped isolator for a target transmissibility — `vibration_isolator_design`

The mount stiffness, natural frequency, frequency ratio and static deflection an undamped isolator needs so that a machine running at omega transmits only a fraction TR of its force.

$$
TR = \frac{1}{r^2-1},\quad k = m\Big(\frac{\omega}{r}\Big)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | isolated mass | kg |
| `omega` | in | forcing frequency (rad/s or rpm) | rad/s |
| `TR` | in | target transmissibility (0 < TR < 1, e.g. 0.1 or 10%) | — |
| `k` | out | required total stiffness | N/m |
| `f_n` | out | natural frequency of the mounted system | Hz |
| `r` | out | frequency ratio omega/omega_n | — |
| `static_deflection_mm` | out | static deflection of the mounts, mm | — |

**Applies when:** undamped isolator; 0 < TR < 1 (isolation region r > sqrt(2)).

```shell
$ mechanical-engineering solve vibration_isolator_design "m=200 kg" "omega=1500 rpm" TR=0.1
k = 448618.3819 N/m
f_n = 7.537783614 Hz
r = 3.31662479
static_deflection_mm = 4.371934096
  by: Stiffness of an undamped isolator for a target transmissibility  (TR = 1/(r^2 - 1) -> r = sqrt(1 + 1/TR), k = m (omega/r)^2)
  m = 200 kg
  omega = 157.0796327 rad/s
  TR = 0.1
  assumes: undamped isolator; 0 < TR < 1 (isolation region r > sqrt(2))
```

**What the worked example means:** To achieve the target transmissibility of 0.1 for a mass of 200 kg operating at a forcing frequency of 1500 rpm, the isolator requires a total stiffness of 448618 N/m. This configuration results in a system natural frequency of 7.53778 Hz and a frequency ratio of 3.31662. The corresponding static deflection under the machine's weight is 4.37193, ensuring the vibration isolation criteria are met within the undamped assumption.

<a id="two_rotor_torsional_vibration"></a>

#### Torsional natural frequency of a two-rotor system — `two_rotor_torsional_vibration`

The torsional natural frequency of two rotors joined by an elastic shaft, the ratio of their vibration amplitudes and (with the shaft length) where the node lies.

$$
\omega = \sqrt{\frac{k_t(J_1+J_2)}{J_1J_2}},\quad l_1 = \frac{J_2L}{J_1+J_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `J1` | in | mass moment of inertia of rotor 1 | kg*m^2 |
| `J2` | in | mass moment of inertia of rotor 2 | kg*m^2 |
| `kt` | in | torsional stiffness of the shaft (N*m/rad) | N*m/rad |
| `L` | in | shaft length (optional: gives the node position) (optional) | m |
| `omega` | out | torsional natural frequency | rad/s |
| `f` | out | natural frequency in Hz | Hz |
| `amplitude_ratio` | out | amplitude of rotor 2 / rotor 1 (negative: opposite phase) | — |
| `node_from_rotor1` | out | distance of the node from rotor 1 (with L) | m |

**Applies when:** shaft inertia neglected; free-free system (no fixed end).

```shell
$ mechanical-engineering solve two_rotor_torsional_vibration "J1=2 kg*m^2" "J2=6 kg*m^2" "kt=30 kN*m/rad" "L=1.2 m"
omega = 141.4213562 rad/s
f = 22.5079079 Hz
amplitude_ratio = -0.3333333333
node_from_rotor1 = 0.9 m
  by: Torsional natural frequency of a two-rotor system  (omega^2 = kt (J1 + J2)/(J1 J2); amplitude ratio -J1/J2; node at L J2/(J1 + J2) from rotor 1)
  J1 = 2 kg*m^2
  J2 = 6 kg*m^2
  kt = 30000 N*m/rad
  L = 1.2 m
  assumes: shaft inertia neglected; free-free system (no fixed end)
```

**What the worked example means:** This calculation determines the fundamental torsional vibration characteristics of a free-free two-rotor system connected by an elastic shaft. Given the mass moments of inertia for both rotors and the shaft's torsional stiffness, the model computes the natural angular frequency as 141.421 rad/s rad/s and its cyclic equivalent in Hz. It also establishes that the amplitude ratio between the rotors is -0.333333, indicating their relative motion magnitude and phase opposition, while pinpointing the nodal point at a distance of 0.9 m from the first rotor.

<a id="rotating_unbalance_response"></a>

#### Vibration due to rotating unbalance — `rotating_unbalance_response`

The steady-state amplitude of a machine on springs and dampers excited by a rotating unbalance, the unbalance force and the force transmitted to the foundation.

$$
X = \frac{m_ue\omega^2}{\sqrt{(k-M\omega^2)^2 + (c\omega)^2}},\quad F_T = X\sqrt{k^2+(c\omega)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | total mass of the machine | kg |
| `mu` | in | unbalanced mass | kg |
| `e` | in | eccentricity of the unbalanced mass | m |
| `k` | in | total stiffness of the mounts | N/m |
| `c` | in | damping coefficient | kg/s |
| `omega` | in | rotational speed | rad/s |
| `X` | out | amplitude of the machine | m |
| `X_mm` | out | amplitude in mm | — |
| `F_transmitted` | out | force amplitude transmitted to the foundation | N |
| `F_unbalance` | out | rotating unbalance force mu e omega^2 | N |

**Applies when:** vertical motion only, steady state; mu < M.

```shell
$ mechanical-engineering solve rotating_unbalance_response "M=100 kg" "mu=0.5 kg" "e=50 mm" "k=400 kN/m" "c=0 N*s/m" "omega=100 rad/s"
X = 0.0004166666667 m
X_mm = 0.4166666667
F_transmitted = 166.6666667 N
F_unbalance = 250 N
  by: Vibration due to rotating unbalance  (X = mu e w^2/sqrt((k - M w^2)^2 + (c w)^2), F_T = X sqrt(k^2 + (c w)^2))
  M = 100 kg
  mu = 0.5 kg
  e = 0.05 m
  k = 400000 N/m
  c = 0 kg/s
  omega = 100 rad/s
  assumes: vertical motion only, steady state; mu < M
```

**What the worked example means:** This calculation determines the vibration response of a machine with rotating unbalance. Given the total mass, unbalanced mass, eccentricity, stiffness, damping, and rotational speed, it computes the resulting displacement amplitude and the dynamic force passed to the support structure. The output shows that for the current operating conditions, the machine vibrates with an amplitude of 0.000416667 m, which corresponds to 0.416667 when converted to millimeters. The rotating unbalance generates a centrifugal force of 250 N, and the isolation system transmits a force of 166.667 N to the foundation.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
