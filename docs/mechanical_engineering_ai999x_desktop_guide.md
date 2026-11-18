# Mechanical Engineering: the desktop app, user guide

**Mechanical Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Mechanical Engineering covers statics, stress and strain, beams and frames, section properties, torsion, columns, fatigue, machine elements, vibrations and structural analysis. Its desktop app holds all 97 calculations in 14 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-mechanical-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-mechanical-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-mechanical-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-mechanical-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-mechanical-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-mechanical-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-mechanical-engineering-1.6.5-linux-x86_64/ai999x-mechanical-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Mechanical Engineering app, opened on Simply supported beam with a point load with its worked example.](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/hero.webp)

*The Mechanical Engineering app, opened on Simply supported beam with a point load with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Beams › Simply supported beam with a point load (or type part of its name in the search box). It opens with a worked example filled in: “Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4” The answer: RA = 13333.33333 N, RB = 6666.666667 N, M_max = 26666.66667 N*m, V_max = 13333.33333 N.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Simply supported beam with a point load: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-1.webp)

*Simply supported beam with a point load: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Simply supported beam with a point load
RA = 13333.33333 N
RB = 6666.666667 N
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Principal stresses and maximum shear (Mohr's circle, plane stress)](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-2.webp)

*Principal stresses and maximum shear (Mohr's circle, plane stress)*

![Fatigue safety factor by the modified Goodman criterion](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-3.webp)

*Fatigue safety factor by the modified Goodman criterion*

![Euler buckling load of a column with given end conditions](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-4.webp)

*Euler buckling load of a column with given end conditions*

![Required catalogue rating of a rolling bearing](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-5.webp)

*Required catalogue rating of a rolling bearing*

![Stresses in a rotating disk (solid or with a central hole)](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-6.webp)

*Stresses in a rotating disk (solid or with a central hole)*

![Composite section of rectangles (parallel-axis theorem)](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-7.webp)

*Composite section of rectangles (parallel-axis theorem)*

![Equivalent force-couple system and single resultant of a force system](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/scene-8.webp)

*Equivalent force-couple system and single resultant of a force system*

![Simply supported beam with a point load: a chart, enlarged.](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/chart-1.webp)

*Simply supported beam with a point load: a chart, enlarged.*

![Principal stresses and maximum shear (Mohr's circle, plane stress): a chart, enlarged.](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/chart-2.webp)

*Principal stresses and maximum shear (Mohr's circle, plane stress): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Mechanical Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=mechanical-engineering-ai999x/hero-dark.webp)

*The Mechanical Engineering app in the dark theme.*

## 7. Every calculation

97 calculations in 14 topics; each has its own entry on the model's page (https://www.ai999x.com/models/mechanical-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Beams | Cantilever with a point load, Cantilever with a triangular (linearly varying) load, Cantilever with a uniformly distributed load, Fixed-fixed (built-in) beam with a point load, Fixed-fixed (built-in) beam with a uniformly distributed load, Propped cantilever with a point load, Propped cantilever with a uniformly distributed load, Simply supported beam with a point load, Simply supported beam with a uniformly distributed load, Simply supported beam with several point loads |
| Columns | Critical stress of an intermediate column (Johnson parabola / Euler), Eccentrically loaded column (secant formula), Euler buckling load of a column with given end conditions |
| Failure theories | Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr), Von Mises and Tresca equivalent stresses and safety factors |
| Fatigue | Endurance limit with the Marin factors, Fatigue damage of a stress history (rainflow counting + Miner's rule), Fatigue damage on the trilinear (Eurocode 3) S-N curve, Fatigue life from the S-N line (10^3 to 10^6 cycles), Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law, Fatigue safety factor by the ASME-elliptic criterion, Fatigue safety factor by the Gerber criterion, Fatigue safety factor by the modified Goodman criterion, Fatigue safety factor by the Soderberg criterion, Fatigue strength at a given number of cycles (S-N line), Miner's rule damage with a single-slope S-N curve, Rainflow cycle counting of a load history (ASTM E1049), Shaft diameter for a design factor (DE-Goodman), Shaft fatigue safety factor (DE-Goodman) |
| Machine elements | Belt or rope friction on a drum (capstan equation), Bolt tightening torque and preload (T = K F d), Eccentrically loaded bolt group (elastic method), Four-bar linkage position analysis, Helical compression spring: rate, deflection and shear stress, Lewis bending stress of a gear tooth, Power of a belt drive with centrifugal tension, Power screw: raising and lowering torque, efficiency, self-locking, Required catalogue rating of a rolling bearing, Rolling-bearing life at a given reliability, Simple and compound gear trains: ratio, output speed and torque, Slider-crank mechanism: slider position, velocity and acceleration, Speeds of a simple planetary (epicyclic) gear train, Spur gear mesh geometry and contact ratio, Torque capacity of a disc clutch or brake, Wahl curvature factor of a helical spring |
| Pressure vessels | Shrink (press) fit: interface pressure and hoop stresses, Stresses in a rotating disk (solid or with a central hole), Thick-walled cylinder under pressure (Lame equations), Thin-walled pressure vessel stresses (cylinder or sphere) |
| Section properties | Composite section of rectangles (parallel-axis theorem), Section properties of a channel (C) section, Section properties of a hollow circle (tube), Section properties of a hollow rectangle (box), Section properties of a polygonal section, Section properties of a solid circle, Section properties of a solid rectangle, Section properties of a symmetric I-section, Section properties of a T-section, Section properties of an angle (L) section, with principal axes |
| Statics | Equivalent force-couple system and single resultant of a force system, Moment of a force about a point (2D), Moment of a force about a point and about an axis (3D), Resultant of concurrent coplanar forces, Tensions in two cables holding a weight |
| Stress analysis | Combined axial force and biaxial bending of a rectangular section, Plane stress transformation to rotated axes, Principal stresses and maximum shear (Mohr's circle, plane stress), Principal stresses of a three-dimensional stress state, Strain rosette: strains, principal strains and principal stresses |
| Stress and strain | Bar fixed at both ends with an axial load at a step, Direct shear and bearing stress in pins, bolts and rivets, Generalized Hooke's law: strains from a 3D stress state, Stepped bar under axial loads at its joints, Thermal stress and force in a restrained bar (with an optional gap) |
| Stresses in beams | Bending stress at a point of a beam section (flexure formula), Transverse shear stress at a point of a beam section (V Q / I b) |
| Structural analysis | Continuous beam under uniform loads (three-moment equation), Member forces of a plane truss (method of joints) |
| Torsion | Angle of twist of a stepped shaft, Shaft fixed at both ends with a torque between them, Torque and minimum diameter of a power-transmission shaft, Torsion of a hollow circular shaft, Torsion of a solid circular shaft |
| Vibrations | All critical (whirling) speeds of a shaft with several disks, Critical speed of a shaft carrying one disk, Damping ratio, damped frequency and logarithmic decrement, First critical speed of a shaft with several disks (Dunkerley), First critical speed of a shaft with several disks (Rayleigh), Force (or base-motion) transmissibility of a vibration isolator, Free vibration response of a damped spring-mass system, Natural frequencies and first mode of a spring-mass chain, Natural frequencies of a uniform beam (Euler-Bernoulli), Natural frequency of a spring-mass system, Steady-state response to a harmonic force, Stiffness of an undamped isolator for a target transmissibility, Torsional natural frequency of a two-rotor system, Vibration due to rotating unbalance |

## 8. Other ways to use it

The same engine runs on the command line (`mechanical-engineering`, its own guide), in Python (`pip install mechanical-engineering-ai999x`, `import mechanical_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/mechanical-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/mechanical-engineering-ai999x/
