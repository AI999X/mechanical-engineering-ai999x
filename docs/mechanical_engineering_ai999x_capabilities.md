# mechanical-engineering-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `mechanical_engineering_ai999x.about()` for the same answer.

Every capability mechanical-engineering-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import mechanical_engineering_ai999x

result = mechanical_engineering_ai999x.solve('resultant_of_concurrent_forces', F='3, 4', theta='0, 90')
result.show()
# R = 5 N, Rx = 3 N, Ry = 4 N, direction_deg = 53.1301
```

## By topic

| topic | capabilities |
|---|---|
| Beams / cantilevers | [Cantilever with a point load](#cantilever_point_load), [Cantilever with a uniformly distributed load](#cantilever_udl), [Cantilever with a triangular (linearly varying) load](#cantilever_triangular_load) |
| Beams / simply supported beams | [Simply supported beam with a point load](#simply_supported_beam_point_load), [Simply supported beam with a uniformly distributed load](#simply_supported_beam_udl), [Simply supported beam with several point loads](#simply_supported_beam_point_loads) |
| Beams / statically indeterminate beams | [Fixed-fixed (built-in) beam with a uniformly distributed load](#fixed_fixed_beam_udl), [Fixed-fixed (built-in) beam with a point load](#fixed_fixed_beam_point_load), [Propped cantilever with a uniformly distributed load](#propped_cantilever_udl), [Propped cantilever with a point load](#propped_cantilever_point_load) |
| Columns / eccentric loading | [Eccentrically loaded column (secant formula)](#secant_column) |
| Columns / elastic buckling | [Euler buckling load of a column with given end conditions](#euler_buckling) |
| Columns / inelastic buckling | [Critical stress of an intermediate column (Johnson parabola / Euler)](#johnson_column) |
| Failure theories / brittle materials | [Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr)](#brittle_failure_mohr) |
| Failure theories / ductile materials | [Von Mises and Tresca equivalent stresses and safety factors](#yield_criteria_safety_factor) |
| Fatigue / cumulative damage | [Miner's rule damage with a single-slope S-N curve](#miner_damage_sn_curve), [Fatigue damage on the trilinear (Eurocode 3) S-N curve](#eurocode_fatigue_damage), [Fatigue damage of a stress history (rainflow counting + Miner's rule)](#rainflow_fatigue_damage) |
| Fatigue / cycle counting | [Rainflow cycle counting of a load history (ASTM E1049)](#rainflow_counting) |
| Fatigue / endurance limit | [Endurance limit with the Marin factors](#marin_endurance_limit) |
| Fatigue / mean-stress criteria | [Fatigue safety factor by the modified Goodman criterion](#goodman_fatigue_safety_factor), [Fatigue safety factor by the Soderberg criterion](#soderberg_fatigue_safety_factor), [Fatigue safety factor by the Gerber criterion](#gerber_fatigue_safety_factor), [Fatigue safety factor by the ASME-elliptic criterion](#asme_elliptic_fatigue_safety_factor) |
| Fatigue / shaft design | [Shaft fatigue safety factor (DE-Goodman)](#shaft_fatigue_safety_factor), [Shaft diameter for a design factor (DE-Goodman)](#shaft_diameter_de_goodman) |
| Fatigue / stress-life | [Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law](#fatigue_life_mean_stress_correction), [Fatigue life from the S-N line (10^3 to 10^6 cycles)](#sn_fatigue_life), [Fatigue strength at a given number of cycles (S-N line)](#sn_fatigue_strength) |
| Machine elements / belts and band brakes | [Belt or rope friction on a drum (capstan equation)](#belt_friction_capstan), [Power of a belt drive with centrifugal tension](#belt_drive_power) |
| Machine elements / bolted joints | [Eccentrically loaded bolt group (elastic method)](#eccentric_bolt_group), [Bolt tightening torque and preload (T = K F d)](#bolt_preload_torque) |
| Machine elements / clutches and brakes | [Torque capacity of a disc clutch or brake](#disc_clutch_torque) |
| Machine elements / gear trains | [Speeds of a simple planetary (epicyclic) gear train](#planetary_gear_train) |
| Machine elements / gears | [Spur gear mesh geometry and contact ratio](#spur_gear_contact_ratio), [Lewis bending stress of a gear tooth](#lewis_gear_bending_stress) |
| Machine elements / mechanisms | [Slider-crank mechanism: slider position, velocity and acceleration](#slider_crank_kinematics), [Four-bar linkage position analysis](#four_bar_linkage_position) |
| Machine elements / power screws | [Power screw: raising and lowering torque, efficiency, self-locking](#power_screw_torque) |
| Machine elements / rolling bearings | [Rolling-bearing life at a given reliability](#bearing_life_reliability), [Required catalogue rating of a rolling bearing](#bearing_catalog_rating) |
| Machine elements / springs | [Helical compression spring: rate, deflection and shear stress](#helical_compression_spring), [Wahl curvature factor of a helical spring](#wahl_factor) |
| Pressure vessels / interference fits | [Shrink (press) fit: interface pressure and hoop stresses](#shrink_fit_pressure) |
| Pressure vessels / rotating disks | [Stresses in a rotating disk (solid or with a central hole)](#rotating_disk_stresses) |
| Pressure vessels / thick-walled cylinders | [Thick-walled cylinder under pressure (Lame equations)](#thick_walled_cylinder) |
| Pressure vessels / thin-walled vessels | [Thin-walled pressure vessel stresses (cylinder or sphere)](#thin_walled_pressure_vessel) |
| Section properties / arbitrary sections | [Section properties of a polygonal section](#polygon_section_properties) |
| Section properties / composite sections | [Composite section of rectangles (parallel-axis theorem)](#composite_section_properties) |
| Section properties / standard shapes | [Section properties of a solid rectangle](#rectangular_section_properties), [Section properties of a solid circle](#circular_section_properties), [Section properties of a hollow circle (tube)](#hollow_circular_section_properties), [Section properties of a hollow rectangle (box)](#hollow_rectangular_section_properties), [Section properties of a symmetric I-section](#i_section_properties), [Section properties of a T-section](#t_section_properties), [Section properties of a channel (C) section](#channel_section_properties) |
| Section properties / unsymmetric sections | [Section properties of an angle (L) section, with principal axes](#angle_section_properties) |
| Statics / equilibrium of a particle | [Tensions in two cables holding a weight](#two_cable_equilibrium) |
| Statics / force systems | [Resultant of concurrent coplanar forces](#resultant_of_concurrent_forces) |
| Statics / moments | [Moment of a force about a point (2D)](#moment_of_a_force) |
| Stress analysis / combined loading | [Combined axial force and biaxial bending of a rectangular section](#combined_axial_and_bending_stress) |
| Stress analysis / principal stresses | [Principal stresses and maximum shear (Mohr's circle, plane stress)](#principal_stresses_2d), [Principal stresses of a three-dimensional stress state](#principal_stresses_3d) |
| Stress analysis / strain measurement | [Strain rosette: strains, principal strains and principal stresses](#strain_rosette) |
| Stress analysis / stress transformation | [Plane stress transformation to rotated axes](#plane_stress_transformation) |
| Stress and strain / axial loading | [Stepped bar under axial loads at its joints](#axial_bar_segments) |
| Stress and strain / constitutive law | [Generalized Hooke's law: strains from a 3D stress state](#generalized_hookes_law) |
| Stress and strain / statically indeterminate members | [Bar fixed at both ends with an axial load at a step](#bar_fixed_at_both_ends) |
| Structural analysis / continuous beams | [Continuous beam under uniform loads (three-moment equation)](#continuous_beam_three_moment) |
| Structural analysis / trusses | [Member forces of a plane truss (method of joints)](#truss_member_forces) |
| Torsion / circular shafts | [Torsion of a solid circular shaft](#solid_shaft_torsion), [Torsion of a hollow circular shaft](#hollow_shaft_torsion) |
| Torsion / compound shafts | [Angle of twist of a stepped shaft](#stepped_shaft_twist) |
| Torsion / power transmission | [Torque and minimum diameter of a power-transmission shaft](#shaft_power_transmission) |
| Torsion / statically indeterminate shafts | [Shaft fixed at both ends with a torque between them](#statically_indeterminate_shaft) |
| Vibrations / continuous systems | [Natural frequencies of a uniform beam (Euler-Bernoulli)](#beam_natural_frequency) |
| Vibrations / critical speeds of shafts | [Critical speed of a shaft carrying one disk](#shaft_critical_speed_single_disk), [First critical speed of a shaft with several disks (Rayleigh)](#rayleigh_critical_speed), [First critical speed of a shaft with several disks (Dunkerley)](#dunkerley_critical_speed), [All critical (whirling) speeds of a shaft with several disks](#shaft_whirling_speeds) |
| Vibrations / damped vibration | [Damping ratio, damped frequency and logarithmic decrement](#damped_sdof_parameters), [Free vibration response of a damped spring-mass system](#sdof_free_response) |
| Vibrations / forced vibration | [Steady-state response to a harmonic force](#forced_vibration_amplitude), [Vibration due to rotating unbalance](#rotating_unbalance_response) |
| Vibrations / free vibration | [Natural frequency of a spring-mass system](#sdof_natural_frequency) |
| Vibrations / multi-degree-of-freedom systems | [Natural frequencies and first mode of a spring-mass chain](#mdof_spring_mass_chain) |
| Vibrations / torsional vibration | [Torsional natural frequency of a two-rotor system](#two_rotor_torsional_vibration) |
| Vibrations / vibration isolation | [Force (or base-motion) transmissibility of a vibration isolator](#vibration_transmissibility), [Stiffness of an undamped isolator for a target transmissibility](#vibration_isolator_design) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| resultant_of_concurrent_forces | Resultant of concurrent coplanar forces | F, theta | R, Rx, Ry, direction_deg |
| two_cable_equilibrium | Tensions in two cables holding a weight | W, alpha, beta | T1, T2 |
| moment_of_a_force | Moment of a force about a point (2D) | Fx, Fy, x, y, x0, y0 | M, d |
| simply_supported_beam_point_load | Simply supported beam with a point load | L, P, a, E, I | RA, RB, M_max, V_max, deflection_at_load_mm, max_deflection_mm, x_max |
| simply_supported_beam_udl | Simply supported beam with a uniformly distributed load | L, w, E, I | RA, RB, M_max, V_max, max_deflection_mm, end_slope_deg |
| simply_supported_beam_point_loads | Simply supported beam with several point loads | L, x, P | RA, RB, M_max, x_Mmax, V_max |
| cantilever_point_load | Cantilever with a point load | L, P, E, I, a | R, M_fixed, deflection_at_load_mm, tip_deflection_mm, tip_slope_deg |
| cantilever_udl | Cantilever with a uniformly distributed load | L, w, E, I | R, M_fixed, tip_deflection_mm, tip_slope_deg |
| cantilever_triangular_load | Cantilever with a triangular (linearly varying) load | L, w, E, I | R, M_fixed, tip_deflection_mm, tip_slope_deg |
| fixed_fixed_beam_udl | Fixed-fixed (built-in) beam with a uniformly distributed load | L, w, E, I | RA, RB, M_end, M_mid, max_deflection_mm |
| fixed_fixed_beam_point_load | Fixed-fixed (built-in) beam with a point load | L, P, a, E, I | RA, RB, MA, MB, M_load, deflection_at_load_mm |
| propped_cantilever_udl | Propped cantilever with a uniformly distributed load | L, w, E, I | RA, RB, MA, M_span, x_M_span, max_deflection_mm, x_max |
| propped_cantilever_point_load | Propped cantilever with a point load | L, P, a, E, I | RA, RB, MA, M_load, deflection_at_load_mm |
| rectangular_section_properties | Section properties of a solid rectangle | b, h | A, Ix, Iy, Zx, Zy, rx, ry, J_polar |
| circular_section_properties | Section properties of a solid circle | d | A, I, J, Z, Zp, r |
| hollow_circular_section_properties | Section properties of a hollow circle (tube) | D, d | A, I, J, Z, Zp, r |
| hollow_rectangular_section_properties | Section properties of a hollow rectangle (box) | B, H, t | A, Ix, Iy, Zx, Zy, rx, ry, Zpx |
| i_section_properties | Section properties of a symmetric I-section | b, h, tf, tw | A, Ix, Iy, Zx, Zy, rx, ry, Zpx, Zpy, shape_factor_x |
| t_section_properties | Section properties of a T-section | bf, tf, h, tw | A, y_bar, Ix, Iy, Z_top, Z_bottom, rx, ry |
| channel_section_properties | Section properties of a channel (C) section | b, h, tf, tw | A, x_bar, Ix, Iy, Zx, Zy_min, rx, ry |
| angle_section_properties | Section properties of an angle (L) section, with principal axes | b, h, t | A, x_bar, y_bar, Ix, Iy, Ixy, I1, I2, principal_angle_deg, r_min |
| composite_section_properties | Composite section of rectangles (parallel-axis theorem) | rects | A, x_bar, y_bar, Ix, Iy, Ixy, I1, I2 |
| polygon_section_properties | Section properties of a polygonal section | vertices | A, x_bar, y_bar, Ix, Iy, Ixy, I1, I2 |
| solid_shaft_torsion | Torsion of a solid circular shaft | T, d, L, G | tau_max_MPa, tau_inner_MPa, twist_rad, twist_deg, J |
| hollow_shaft_torsion | Torsion of a hollow circular shaft | T, D, d, L, G | tau_max_MPa, tau_inner_MPa, twist_rad, twist_deg, J |
| shaft_power_transmission | Torque and minimum diameter of a power-transmission shaft | P, n, tau_allow | T, d_min_mm, frequency_Hz |
| stepped_shaft_twist | Angle of twist of a stepped shaft | T, lengths, diameters, G | twist_rad, twist_deg, tau_max_MPa |
| statically_indeterminate_shaft | Shaft fixed at both ends with a torque between them | T0, a, b, d1, d2, G | TA, TB, tau1_MPa, tau2_MPa, twist_at_load_deg |
| axial_bar_segments | Stepped bar under axial loads at its joints | lengths, areas, loads, E | elongation_mm, N_kN, stress_MPa, max_abs_stress_MPa |
| bar_fixed_at_both_ends | Bar fixed at both ends with an axial load at a step | P, L1, A1, E1, L2, A2, E2 | RA, RB, stress1_MPa, stress2_MPa, displacement_mm |
| plane_stress_transformation | Plane stress transformation to rotated axes | sx, sy, txy, theta | sx_t_MPa, sy_t_MPa, txy_t_MPa |
| principal_stresses_2d | Principal stresses and maximum shear (Mohr's circle, plane stress) | sx, sy, txy | s1_MPa, s2_MPa, tau_max_inplane_MPa, theta_p_deg, tau_abs_max_MPa, von_mises_MPa |
| principal_stresses_3d | Principal stresses of a three-dimensional stress state | sx, sy, sz, txy, tyz, tzx | s1_MPa, s2_MPa, s3_MPa, tau_max_MPa, von_mises_MPa, hydrostatic_MPa, tau_oct_MPa |
| yield_criteria_safety_factor | Von Mises and Tresca equivalent stresses and safety factors | Sy, sx, sy, sz, txy, tyz, tzx | von_mises_MPa, tresca_MPa, n_von_mises, n_tresca |
| brittle_failure_mohr | Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr) | sA, sB, Sut, Suc | n_max_normal, n_coulomb_mohr, n_modified_mohr |
| strain_rosette | Strain rosette: strains, principal strains and principal stresses | ea, eb, ec, E, nu, layout | ex, ey, gxy, e1, e2, theta_p_deg, s1_MPa, s2_MPa |
| generalized_hookes_law | Generalized Hooke's law: strains from a 3D stress state | sx, sy, sz, txy, tyz, tzx, E, nu | ex, ey, ez, gxy, gyz, gzx, volumetric |
| thin_walled_pressure_vessel | Thin-walled pressure vessel stresses (cylinder or sphere) | p, r, t, shape | hoop_MPa, longitudinal_MPa, tau_inplane_MPa, tau_abs_MPa |
| thick_walled_cylinder | Thick-walled cylinder under pressure (Lame equations) | pi, po, a, b, r | sr_MPa, st_MPa, st_inner_MPa, st_outer_MPa, tau_max_inner_MPa, sz_closed_MPa |
| shrink_fit_pressure | Shrink (press) fit: interface pressure and hoop stresses | delta, R, ro, E, ri | p_MPa, hub_hoop_MPa, shaft_hoop_MPa |
| rotating_disk_stresses | Stresses in a rotating disk (solid or with a central hole) | rho, omega, nu, ro, ri | hoop_max_MPa, radial_max_MPa, r_radial_max, hoop_rim_MPa |
| euler_buckling | Euler buckling load of a column with given end conditions | E, I, L, ends, A | P_cr, P_cr_kN, K, Le, sigma_cr_MPa, slenderness |
| johnson_column | Critical stress of an intermediate column (Johnson parabola / Euler) | Sy, E, L, r, ends, A | slenderness, transition_slenderness, regime, sigma_cr_MPa, P_cr_kN |
| secant_column | Eccentrically loaded column (secant formula) | P, e, L, A, I, c, E | sigma_max_MPa, deflection_mm, M_max, P_over_Pcr |
| combined_axial_and_bending_stress | Combined axial force and biaxial bending of a rectangular section | b, h, N, Mx, My | sigma_corners_MPa, sigma_max_MPa, sigma_min_MPa, sigma_axial_MPa |
| eccentric_bolt_group | Eccentrically loaded bolt group (elastic method) | bolts, P, e | direct_kN, max_bolt_force_kN, critical_bolt, polar_sum_mm2 |
| goodman_fatigue_safety_factor | Fatigue safety factor by the modified Goodman criterion | sa, sm, Se, Sut, Sy | n_fatigue, sigma_ar_MPa, n_yield |
| soderberg_fatigue_safety_factor | Fatigue safety factor by the Soderberg criterion | sa, sm, Se, Sy | n_fatigue, sigma_ar_MPa |
| fatigue_life_mean_stress_correction | Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law | sa, sm, sf, b, method, gamma | sigma_ar_MPa, Nf, log10_Nf |
| sn_fatigue_life | Fatigue life from the S-N line (10^3 to 10^6 cycles) | Sut, Se, f, sa | a_MPa, b, N, log10_N |
| sn_fatigue_strength | Fatigue strength at a given number of cycles (S-N line) | Sut, Se, f, N | Sf_MPa |
| miner_damage_sn_curve | Miner's rule damage with a single-slope S-N curve | ranges, counts, Sc, m, Nc | damage, repeats_to_failure |
| eurocode_fatigue_damage | Fatigue damage on the trilinear (Eurocode 3) S-N curve | ranges, counts, dsc | damage, delta_sigma_D, delta_sigma_L, repeats_to_failure |
| rainflow_counting | Rainflow cycle counting of a load history (ASTM E1049) | history | ranges, counts, total_cycles, max_range |
| rainflow_fatigue_damage | Fatigue damage of a stress history (rainflow counting + Miner's rule) | history, Sc, m, Nc | damage, repeats_to_failure, cycles |
| shaft_fatigue_safety_factor | Shaft fatigue safety factor (DE-Goodman) | d, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut, Sy | n_fatigue, sigma_a_vm_MPa, sigma_m_vm_MPa, n_yield |
| shaft_diameter_de_goodman | Shaft diameter for a design factor (DE-Goodman) | n, Ma, Mm, Ta, Tm, Kf, Kfs, Se, Sut | d_mm |
| belt_friction_capstan | Belt or rope friction on a drum (capstan equation) | T2, mu, theta, beta | T1, ratio, friction_force |
| belt_drive_power | Power of a belt drive with centrifugal tension | T1, m, v, mu, theta, beta | Tc, T2, P, P_kW |
| power_screw_torque | Power screw: raising and lowering torque, efficiency, self-locking | F, dm, l, f, alpha, fc, dc | T_raise, T_lower, efficiency, self_locking |
| disc_clutch_torque | Torque capacity of a disc clutch or brake | F, f, D, d, model, pairs | T, p_max_MPa |
| planetary_gear_train | Speeds of a simple planetary (epicyclic) gear train | Ns, Nr, ws, wr, wc | ws_rpm, wr_rpm, wc_rpm, wp_rpm, Np |
| spur_gear_contact_ratio | Spur gear mesh geometry and contact ratio | N1, N2, m, phi | d1_mm, d2_mm, C_mm, Z_mm, pb_mm, contact_ratio |
| bearing_life_reliability | Rolling-bearing life at a given reliability | C, F, n, R, kind, LR, x0, theta, b | L10_rev, L10_hours, life_multiple, L_rev, L_hours |
| bearing_catalog_rating | Required catalogue rating of a rolling bearing | FD, LD, n, R, kind, af, LR, x0, theta, b | C10, C10_kN, design_life_multiple |
| helical_compression_spring | Helical compression spring: rate, deflection and shear stress | d, D, Na, G, F | index, k, deflection_mm, tau_torsion_MPa, tau_Ks_MPa |
| sdof_natural_frequency | Natural frequency of a spring-mass system | m, k | omega_n, f_n, period, static_deflection_mm |
| damped_sdof_parameters | Damping ratio, damped frequency and logarithmic decrement | m, c, k | omega_n, zeta, omega_d, f_d, c_critical, log_decrement, regime |
| sdof_free_response | Free vibration response of a damped spring-mass system | m, c, k, x0, v0, t | x, x_mm, v |
| forced_vibration_amplitude | Steady-state response to a harmonic force | m, c, k, F0, omega | X, X_mm, phase_deg, magnification, r |
| vibration_transmissibility | Force (or base-motion) transmissibility of a vibration isolator | zeta, r | TR, isolation_percent |
| rotating_unbalance_response | Vibration due to rotating unbalance | M, mu, e, k, c, omega | X, X_mm, F_transmitted, F_unbalance |
| vibration_isolator_design | Stiffness of an undamped isolator for a target transmissibility | m, omega, TR | k, f_n, r, static_deflection_mm |
| two_rotor_torsional_vibration | Torsional natural frequency of a two-rotor system | J1, J2, kt, L | omega, f, amplitude_ratio, node_from_rotor1 |
| mdof_spring_mass_chain | Natural frequencies and first mode of a spring-mass chain | masses, springs, ends | omega, f_Hz, mode1 |
| shaft_critical_speed_single_disk | Critical speed of a shaft carrying one disk | m, E, I, L, a | k, omega_cr, N_cr_rpm, static_deflection_mm |
| rayleigh_critical_speed | First critical speed of a shaft with several disks (Rayleigh) | masses, positions, L, E, I | omega_cr, N_cr_rpm, deflections_mm |
| dunkerley_critical_speed | First critical speed of a shaft with several disks (Dunkerley) | masses, positions, L, E, I | omega_cr, N_cr_rpm |
| shaft_whirling_speeds | All critical (whirling) speeds of a shaft with several disks | masses, positions, L, E, I | omega, N_rpm |
| beam_natural_frequency | Natural frequencies of a uniform beam (Euler-Bernoulli) | E, I, rho, A, L, ends, n | beta_L, omega, f |
| truss_member_forces | Member forces of a plane truss (method of joints) | nodes, members, pin, roller, loads | forces_kN, pin_Rx_kN, pin_Ry_kN, roller_Ry_kN, max_tension_kN, max_compression_kN |
| continuous_beam_three_moment | Continuous beam under uniform loads (three-moment equation) | spans, w | support_moments_kNm, reactions_kN |
| slider_crank_kinematics | Slider-crank mechanism: slider position, velocity and acceleration | r, l, theta, omega | x, v, a, rod_angle_deg |
| four_bar_linkage_position | Four-bar linkage position analysis | a, b, c, d, theta2, branch | theta3_deg, theta4_deg, transmission_angle_deg |
| gerber_fatigue_safety_factor | Fatigue safety factor by the Gerber criterion | sa, sm, Se, Sut | n_fatigue, sigma_ar_MPa |
| asme_elliptic_fatigue_safety_factor | Fatigue safety factor by the ASME-elliptic criterion | sa, sm, Se, Sy | n_fatigue, sigma_ar_MPa |
| lewis_gear_bending_stress | Lewis bending stress of a gear tooth | Wt, F, m, Y, Kv | sigma, sigma_MPa |
| wahl_factor | Wahl curvature factor of a helical spring | d, D, F | index, K_wahl, K_bergstrasser, K_s, tau_MPa |
| marin_endurance_limit | Endurance limit with the Marin factors | Sut, surface, load, d, R, kd, kf | ka, kb, kc, ke, Se_prime_MPa, Se_MPa, Se |
| bolt_preload_torque | Bolt tightening torque and preload (T = K F d) | K, d, F, T | T, F, F_kN |

<a id="resultant_of_concurrent_forces"></a>

### Resultant of concurrent coplanar forces — `resultant_of_concurrent_forces`

The magnitude, components and direction of the resultant of forces acting through one point in a plane.

$$
R = \sqrt{\Big(\sum F_i\cos\theta_i\Big)^2 + \Big(\sum F_i\sin\theta_i\Big)^2},\quad \theta_R = \operatorname{atan2}(R_y, R_x)
$$

| input | meaning | condition |
|---|---|---|
| `F` | force magnitudes in N (a list) |  |
| `theta` | direction of each force in degrees, counterclockwise from +x (a list) |  |

| output | meaning |
|---|---|
| `R` | magnitude of the resultant |
| `Rx` | x component of the resultant |
| `Ry` | y component of the resultant |
| `direction_deg` | direction of the resultant in degrees, counterclockwise from +x, in (-180, 180] |

- **Method:** Rx = sum F_i cos(theta_i), Ry = sum F_i sin(theta_i), $R = \sqrt{Rx^{2} + R^{2}}$, direction = atan2(Ry, Rx)
- **Assumes:** F and theta have the same length; magnitudes >= 0, at least one > 0; a zero resultant is reported with direction 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('resultant_of_concurrent_forces', F='3, 4', theta='0, 90')
# -> R = 5 N, Rx = 3 N, Ry = 4 N, direction_deg = 53.1301   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('resultant_of_concurrent_forces', F='10, 10', theta='0, 180')
# -> R = 0 N, Rx = 0 N, Ry = 0 N, direction_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Resultant of concurrent forces: F = [3, 4], theta = [0, 90] | SOLVED | R = 5 N, Rx = 3 N, Ry = 4 N, direction_deg = 53.1301 |
| Resultant of concurrent forces: F = [3, 4], theta = [0, 90, 45] | INVALID_INPUT | Resultant of concurrent coplanar forces: F and theta must have the same number of entries. |
| Resultant of concurrent forces: F = [3, 4] | NEEDS_INPUT | resultant_of_concurrent_forces needs theta (direction of each force in degrees, counterclockwise from +x (a list)). Exam |

<a id="two_cable_equilibrium"></a>

### Tensions in two cables holding a weight — `two_cable_equilibrium`

The tensions in two cables, one rising to the left at angle alpha and one rising to the right at angle beta above the horizontal, that hold a weight W at rest.

$$
T_1 = \frac{W\cos\beta}{\sin(\alpha+\beta)},\quad T_2 = \frac{W\cos\alpha}{\sin(\alpha+\beta)}
$$

| input | meaning | condition |
|---|---|---|
| `W` | weight held |  |
| `alpha` | angle of the left cable above the horizontal |  |
| `beta` | angle of the right cable above the horizontal |  |

| output | meaning |
|---|---|
| `T1` | tension in the left cable |
| `T2` | tension in the right cable |

- **Method:** Lami's theorem: $T_{1} = W$ cos(beta)/sin(alpha + beta), $T_{2} = W$ cos(alpha)/sin(alpha + beta)
- **Assumes:** $W > 0$; $0 < \alpha$, $\beta < 90$ degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('two_cable_equilibrium', W=500.0, alpha=0.5235987755982988, beta=0.7853981633974483)
# -> T1 = 366.025 N, T2 = 448.288 N   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('two_cable_equilibrium', W=100.0, alpha=0.7853981633974483, beta=0.7853981633974483)
# -> T1 = 70.7107 N, T2 = 70.7107 N   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Two cable tensions: W = 500 N, alpha = 30 deg, beta = 45 deg | SOLVED | T1 = 366.025 N, T2 = 448.288 N |
| Two cable tensions: W = 500 N, alpha = 0 deg, beta = 45 deg | INVALID_INPUT | Tensions in two cables holding a weight: alpha must lie strictly between 0 and 90 degrees (measured from the horizontal) |
| Two cable tensions: W = 500 N, alpha = 30 deg | NEEDS_INPUT | two_cable_equilibrium needs beta (angle of the right cable above the horizontal). Example: solve('two_cable_equilibrium' |

<a id="moment_of_a_force"></a>

### Moment of a force about a point (2D) — `moment_of_a_force`

The moment of a force (Fx, Fy) applied at (x, y) about the point (x0, y0), counterclockwise positive, and the perpendicular distance to its line of action.

$$
M_O = (x - x_0)F_y - (y - y_0)F_x,\quad d = \frac{|M_O|}{|\mathbf F|}
$$

| input | meaning | condition |
|---|---|---|
| `Fx` | x component of the force |  |
| `Fy` | y component of the force |  |
| `x` | x coordinate of the point of application |  |
| `y` | y coordinate of the point of application |  |
| `x0` | x coordinate of the moment centre (default 0) |  |
| `y0` | y coordinate of the moment centre (default 0) |  |

| output | meaning |
|---|---|
| `M` | moment about the point (counterclockwise positive) |
| `d` | perpendicular distance from the point to the line of action |

- **Method:** $M = (x - x_{0})$ $Fy - (y - y_{0})$ Fx, d = |M| / |F|
- **Assumes:** the force is not zero
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('moment_of_a_force', Fx=100.0, Fy=200.0, x=2.0, y=1.0)
# -> M = 300 N*m, d = 1.34164 m   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('moment_of_a_force', Fx=0.0, Fy=-50.0, x=3.0, y=0.0, x0=1.0, y0=0.0)
# -> M = -100 N*m, d = 2 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Moment of a force: Fx = 100 N, Fy = 200 N, x = 2 m, y = 1 m | SOLVED | M = 300 N*m, d = 1.34164 m |
| Moment of a force: Fx = 0 N, Fy = -50 N, x = 3 m, y = 0 m, x0 = 1 m, y0 = 0 m | SOLVED | M = -100 N*m, d = 2 m |
| Moment of a force: Fx = 0 N, Fy = 0 N, x = 3 m, y = 0 m | INVALID_INPUT | Moment of a force about a point (2D): the force must not be zero. |

<a id="simply_supported_beam_point_load"></a>

### Simply supported beam with a point load — `simply_supported_beam_point_load`

Reactions, maximum bending moment and shear, deflection under the load and the maximum deflection with its position, for a simply supported beam carrying one point load.

$$
M_{max} = \frac{Pab}{L},\quad \delta_{max} = \frac{Pb\,(L^2-b^2)^{3/2}}{9\sqrt3\,EIL}
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `P` | point load (downward) |  |
| `a` | distance of the load from the left support A |  |
| `E` | Young's modulus |  |
| `I` | second moment of area of the section |  |

| output | meaning |
|---|---|
| `RA` | reaction at the left support A |
| `RB` | reaction at the right support B |
| `M_max` | maximum bending moment (under the load) |
| `V_max` | maximum shear force |
| `deflection_at_load_mm` | deflection under the load in mm |
| `max_deflection_mm` | maximum deflection in mm |
| `x_max` | position of the maximum deflection from A |

- **Method:** $RA = Pb/L$, $RB = Pa/L$, M_max = Pab/L, deflection under the load Pa^2b^2/(3EIL), maximum deflection Pb($L^{2}-b^{2})^{3/2}/(9$ sqrt(3) EIL) at $x = sqrt$((L^2-b^2)/3) (a >= b; mirrored otherwise)
- **Assumes:** $0 < a < L$; Euler-Bernoulli beam, linear elastic, prismatic
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('simply_supported_beam_point_load', L=6.0, P=20000.0, a=2.0, E=200000000000.0, I=8e-05)
# -> RA = 13333.3 N, RB = 6666.67 N, M_max = 26666.7 N*m, V_max = 13333.3 N, deflection_at_load_mm = 4.44444, max_deflection_mm = 4.8385, x_max = 2.73401 m   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('simply_supported_beam_point_load', L=4.0, P=1000.0, a=2.0, E=1.0, I=1.0)
# -> RA = 500 N, RB = 500 N, M_max = 1000 N*m, V_max = 500 N, deflection_at_load_mm = 1.33333e+06, max_deflection_mm = 1.33333e+06, x_max = 2 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m, E = 200 GPa, I = 8e-5 m^4 | SOLVED | RA = 13333.3 N, RB = 6666.67 N, M_max = 26666.7 N*m, V_max = 13333.3 N, deflection_at_load_mm = 4.44444, max_deflection_mm = 4.8385, x_max = 2.73401 m |
| Simply supported beam with point load: L = 6 m, P = 20 kN, a = 7 m, E = 200 GPa, I = 8e-5 m^4 | INVALID_INPUT | Simply supported beam with a point load: the load position a must lie strictly between the supports (0 < a < L). |
| Simply supported beam with point load: L = 6 m, P = 20 kN, a = 2 m | NEEDS_INPUT | simply_supported_beam_point_load needs E, I (Young's modulus; second moment of area of the section). Example: solve('sim |

<a id="simply_supported_beam_udl"></a>

### Simply supported beam with a uniformly distributed load — `simply_supported_beam_udl`

Reactions, maximum bending moment and shear, maximum (midspan) deflection and end slope of a simply supported beam under a uniform load over its whole span.

$$
M_{max} = \frac{wL^2}{8},\quad \delta_{max} = \frac{5wL^4}{384EI}
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `w` | uniform load per unit length (downward) |  |
| `E` | Young's modulus |  |
| `I` | second moment of area of the section |  |

| output | meaning |
|---|---|
| `RA` | reaction at A |
| `RB` | reaction at B |
| `M_max` | maximum bending moment (midspan) |
| `V_max` | maximum shear force (at the supports) |
| `max_deflection_mm` | maximum deflection (midspan) in mm |
| `end_slope_deg` | slope at the supports in degrees |

- **Method:** $R = wL/2$, M_max = wL^2/8, delta_max = 5wL^4/(384EI), theta_end = wL^3/(24EI)
- **Assumes:** L, w, E, $I > 0$; Euler-Bernoulli beam, small deflections
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('simply_supported_beam_udl', L=5.0, w=10000.0, E=200000000000.0, I=4e-05)
# -> RA = 25000 N, RB = 25000 N, M_max = 31250 N*m, V_max = 25000 N, max_deflection_mm = 10.1725, end_slope_deg = 0.373019   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Simply supported beam with uniform load: L = 5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4 | SOLVED | RA = 25000 N, RB = 25000 N, M_max = 31250 N*m, V_max = 25000 N, max_deflection_mm = 10.1725, end_slope_deg = 0.373019 |
| Simply supported beam with UDL: L = 5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4 | SOLVED | RA = 25000 N, RB = 25000 N, M_max = 31250 N*m, V_max = 25000 N, max_deflection_mm = 10.1725, end_slope_deg = 0.373019 |
| Simply supported beam with uniform load: L = -5 m, w = 10 kN/m, E = 200 GPa, I = 4e-5 m^4 | INVALID_INPUT | Simply supported beam with a uniformly distributed load: L and w must be a positive number. |

<a id="simply_supported_beam_point_loads"></a>

### Simply supported beam with several point loads — `simply_supported_beam_point_loads`

Support reactions, the maximum bending moment and its position, and the maximum shear for a simply supported beam carrying several downward point loads.

$$
R_B = \frac{\sum P_i x_i}{L},\quad R_A = \sum P_i - R_B,\quad M(x) = R_A x - \sum_{x_i<x} P_i(x-x_i)
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `x` | load positions from the left support A, in m (a list) |  |
| `P` | point loads in kN, downward (a list, same order as x) |  |

| output | meaning |
|---|---|
| `RA` | reaction at A in kN |
| `RB` | reaction at B in kN |
| `M_max` | maximum bending moment in $kN m$ |
| `x_Mmax` | position of the maximum moment from A, in m (the first one if several are equal) |
| `V_max` | maximum shear force in kN |

- **Method:** moments about A: RB = sum P_i x_i / L, RA = sum P_i - RB; M(x) = RA x - sum P_i (x - x_i) for $x_i < x$, largest at a load point
- **Assumes:** $0  \leq  x_i  \leq  L$; $P_i > 0$ (downward); x and P of equal length
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('simply_supported_beam_point_loads', L=8.0, x='2, 5', P='10, 20')
# -> RA = 15, RB = 15, M_max = 45, x_Mmax = 5, V_max = 15   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('simply_supported_beam_point_loads', L=6.0, x='3', P='12')
# -> RA = 6, RB = 6, M_max = 18, x_Mmax = 3, V_max = 6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Simply supported beam with point loads: L = 8 m, x = [2, 5], P = [10, 20] | SOLVED | RA = 15, RB = 15, M_max = 45, x_Mmax = 5, V_max = 15 |
| Simply supported beam with point loads: L = 8 m, x = [2, 9], P = [10, 20] | INVALID_INPUT | Simply supported beam with several point loads: every load position must lie on the span (0 <= x <= L). |
| Simply supported beam with point loads: L = 8 m, x = [2, 5] | NEEDS_INPUT | simply_supported_beam_point_loads needs P (point loads in kN, downward (a list, same order as x)). Example: solve('simpl |

<a id="cantilever_point_load"></a>

### Cantilever with a point load — `cantilever_point_load`

Reaction, fixed-end moment, deflection under the load, tip deflection and tip slope of a cantilever carrying one point load at distance a from the fixed end (default: at the free end).

$$
\delta_{tip} = \frac{Pa^2(3L-a)}{6EI},\quad \theta_{tip} = \frac{Pa^2}{2EI}
$$

| input | meaning | condition |
|---|---|---|
| `L` | length of the cantilever |  |
| `P` | point load (downward) |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |
| `a` | distance of the load from the fixed end (default L: at the free end) |  |

| output | meaning |
|---|---|
| `R` | vertical reaction at the fixed end |
| `M_fixed` | hogging moment at the fixed end |
| `deflection_at_load_mm` | deflection under the load in mm |
| `tip_deflection_mm` | deflection of the free end in mm |
| `tip_slope_deg` | slope at the free end in degrees |

- **Method:** $M = Pa$, deflection under the load $Pa^{3}/(3EI)$, slope there $Pa^{2}/(2EI)$, tip deflection $Pa^{2}$(3L - a)/(6EI)
- **Assumes:** $0 < a  \leq  L$; Euler-Bernoulli beam, small deflections
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('cantilever_point_load', L=2.0, P=5000.0, E=200000000000.0, I=8e-06)
# -> R = 5000 N, M_fixed = 10000 N*m, deflection_at_load_mm = 8.33333, tip_deflection_mm = 8.33333, tip_slope_deg = 0.358099   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('cantilever_point_load', L=3.0, P=4000.0, a=2.0, E=70000000000.0, I=2e-05)
# -> R = 4000 N, M_fixed = 8000 N*m, deflection_at_load_mm = 7.61905, tip_deflection_mm = 13.3333, tip_slope_deg = 0.327404   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cantilever with end load: L = 2 m, P = 5 kN, E = 200 GPa, I = 8e-6 m^4 | SOLVED | R = 5000 N, M_fixed = 10000 N*m, deflection_at_load_mm = 8.33333, tip_deflection_mm = 8.33333, tip_slope_deg = 0.358099 |
| Cantilever with point load: L = 3 m, P = 4 kN, a = 2 m, E = 70 GPa, I = 2e-5 m^4 | SOLVED | R = 4000 N, M_fixed = 8000 N*m, deflection_at_load_mm = 7.61905, tip_deflection_mm = 13.3333, tip_slope_deg = 0.327404 |
| Cantilever with point load: L = 3 m, P = 4 kN, a = 4 m, E = 70 GPa, I = 2e-5 m^4 | INVALID_INPUT | Cantilever with a point load: the load position a (from the fixed end) must satisfy 0 < a <= L. |

<a id="cantilever_udl"></a>

### Cantilever with a uniformly distributed load — `cantilever_udl`

Reaction, fixed-end moment, tip deflection and tip slope of a cantilever under a uniform load over its whole length.

$$
\delta_{tip} = \frac{wL^4}{8EI},\quad \theta_{tip} = \frac{wL^3}{6EI}
$$

| input | meaning | condition |
|---|---|---|
| `L` | length of the cantilever |  |
| `w` | uniform load per unit length (downward) |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |

| output | meaning |
|---|---|
| `R` | vertical reaction at the fixed end |
| `M_fixed` | hogging moment at the fixed end |
| `tip_deflection_mm` | deflection of the free end in mm |
| `tip_slope_deg` | slope at the free end in degrees |

- **Method:** $R = wL$, $M = wL^{2}/2$, delta_tip = wL^4/(8EI), theta_tip = wL^3/(6EI)
- **Assumes:** L, w, E, $I > 0$; Euler-Bernoulli beam, small deflections
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('cantilever_udl', L=3.0, w=2000.0, E=200000000000.0, I=1e-05)
# -> R = 6000 N, M_fixed = 9000 N*m, tip_deflection_mm = 10.125, tip_slope_deg = 0.257831   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cantilever with uniform load: L = 3 m, w = 2 kN/m, E = 200 GPa, I = 1e-5 m^4 | SOLVED | R = 6000 N, M_fixed = 9000 N*m, tip_deflection_mm = 10.125, tip_slope_deg = 0.257831 |
| Cantilever with UDL: L = 3 m, w = 2 kN/m, E = 200 GPa, I = 0 m^4 | INVALID_INPUT | Cantilever with a uniformly distributed load: E and I must be a positive number. |

<a id="cantilever_triangular_load"></a>

### Cantilever with a triangular (linearly varying) load — `cantilever_triangular_load`

Reaction, fixed-end moment, tip deflection and tip slope of a cantilever whose load falls linearly from w at the fixed end to zero at the free end.

$$
\delta_{tip} = \frac{w_0L^4}{30EI},\quad \theta_{tip} = \frac{w_0L^3}{24EI}
$$

| input | meaning | condition |
|---|---|---|
| `L` | length of the cantilever |  |
| `w` | load intensity at the fixed end (downward) |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |

| output | meaning |
|---|---|
| `R` | vertical reaction at the fixed end |
| `M_fixed` | hogging moment at the fixed end |
| `tip_deflection_mm` | deflection of the free end in mm |
| `tip_slope_deg` | slope at the free end in degrees |

- **Method:** $R = wL/2$, $M = wL^{2}/6$, delta_tip = wL^4/(30EI), theta_tip = wL^3/(24EI)
- **Assumes:** L, w, E, $I > 0$; maximum intensity at the fixed end; Euler-Bernoulli beam
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('cantilever_triangular_load', L=3.0, w=6000.0, E=200000000000.0, I=2e-05)
# -> R = 9000 N, M_fixed = 9000 N*m, tip_deflection_mm = 4.05, tip_slope_deg = 0.0966866   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cantilever with triangular load: L = 3 m, w = 6 kN/m, E = 200 GPa, I = 2e-5 m^4 | SOLVED | R = 9000 N, M_fixed = 9000 N*m, tip_deflection_mm = 4.05, tip_slope_deg = 0.0966866 |
| Cantilever with triangular load: L = 3 m, w = 6 kN/m | NEEDS_INPUT | cantilever_triangular_load needs E, I (Young's modulus; second moment of area). Example: solve('cantilever_triangular_lo |

<a id="fixed_fixed_beam_udl"></a>

### Fixed-fixed (built-in) beam with a uniformly distributed load — `fixed_fixed_beam_udl`

Reactions, fixed-end (hogging) moments, midspan (sagging) moment and maximum deflection of a beam built in at both ends under a uniform load.

$$
M_{end} = \frac{wL^2}{12},\quad M_{mid} = \frac{wL^2}{24},\quad \delta_{max} = \frac{wL^4}{384EI}
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `w` | uniform load per unit length (downward) |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |

| output | meaning |
|---|---|
| `RA` | reaction at A |
| `RB` | reaction at B |
| `M_end` | hogging moment at each fixed end |
| `M_mid` | sagging moment at midspan |
| `max_deflection_mm` | maximum (midspan) deflection in mm |

- **Method:** $R = wL/2$, M_end = wL^2/12, M_mid = wL^2/24, delta_max = wL^4/(384EI)
- **Assumes:** L, w, E, $I > 0$; both ends fully fixed against rotation and deflection
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('fixed_fixed_beam_udl', L=6.0, w=12000.0, E=200000000000.0, I=6e-05)
# -> RA = 36000 N, RB = 36000 N, M_end = 36000 N*m, M_mid = 18000 N*m, max_deflection_mm = 3.375   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fixed-fixed beam with uniform load: L = 6 m, w = 12 kN/m, E = 200 GPa, I = 6e-5 m^4 | SOLVED | RA = 36000 N, RB = 36000 N, M_end = 36000 N*m, M_mid = 18000 N*m, max_deflection_mm = 3.375 |
| Built-in beam with UDL: L = 6 m, w = 12 kN/m, E = 200 GPa, I = 6e-5 m^4 | SOLVED | RA = 36000 N, RB = 36000 N, M_end = 36000 N*m, M_mid = 18000 N*m, max_deflection_mm = 3.375 |

<a id="fixed_fixed_beam_point_load"></a>

### Fixed-fixed (built-in) beam with a point load — `fixed_fixed_beam_point_load`

Reactions, fixed-end moments, the moment under the load and the deflection under the load of a beam built in at both ends carrying one point load.

$$
M_A = \frac{Pab^2}{L^2},\quad M_B = \frac{Pa^2b}{L^2},\quad \delta_P = \frac{Pa^3b^3}{3EIL^3}
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `P` | point load (downward) |  |
| `a` | distance of the load from end A |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |

| output | meaning |
|---|---|
| `RA` | reaction at A |
| `RB` | reaction at B |
| `MA` | hogging moment at A |
| `MB` | hogging moment at B |
| `M_load` | sagging moment under the load |
| `deflection_at_load_mm` | deflection under the load in mm |

- **Method:** MA = Pab^2/L^2, $MB = Pa^{2b}/L^{2}$, $RA = Pb^{2}$(3a + b)/L^3, $RB = Pa^{2}$(a + 3b)/L^3, M_load = 2Pa^2b^2/L^3, delta_load = Pa^3b^3/(3EIL^3)
- **Assumes:** $0 < a < L$; both ends fully fixed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('fixed_fixed_beam_point_load', L=6.0, P=30000.0, a=2.0, E=200000000000.0, I=5e-05)
# -> RA = 22222.2 N, RB = 7777.78 N, MA = 26666.7 N*m, MB = 13333.3 N*m, M_load = 17777.8 N*m, deflection_at_load_mm = 2.37037   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fixed-fixed beam with point load: L = 6 m, P = 30 kN, a = 2 m, E = 200 GPa, I = 5e-5 m^4 | SOLVED | RA = 22222.2 N, RB = 7777.78 N, MA = 26666.7 N*m, MB = 13333.3 N*m, M_load = 17777.8 N*m, deflection_at_load_mm = 2.37037 |
| Fixed-fixed beam with point load: L = 6 m, P = 30 kN, a = 0 m, E = 200 GPa, I = 5e-5 m^4 | INVALID_INPUT | Fixed-fixed (built-in) beam with a point load: the load position a must lie strictly between the ends (0 < a < L). |

<a id="propped_cantilever_udl"></a>

### Propped cantilever with a uniformly distributed load — `propped_cantilever_udl`

Reactions, fixed-end moment, maximum span moment and its position, and the maximum deflection with its position, for a beam fixed at A and simply supported at B under a uniform load.

$$
R_B = \frac{3wL}{8},\quad M_A = \frac{wL^2}{8},\quad \delta_{max} \approx \frac{wL^4}{185EI}
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `w` | uniform load per unit length (downward) |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |

| output | meaning |
|---|---|
| `RA` | reaction at the fixed end A |
| `RB` | reaction at the prop B |
| `MA` | hogging moment at A |
| `M_span` | largest sagging moment in the span |
| `x_M_span` | its position from A |
| `max_deflection_mm` | maximum deflection in mm |
| `x_max` | position of the maximum deflection from A |

- **Method:** $RB = 3wL/8$, $RA = 5wL/8$, $MA = wL^{2}/8$, M_span = 9wL^2/128 at $5L/8$, delta_max = (39 + 55 sqrt(33)) wL^4/(65536 EI) at $x = L$($15 - sqrt$(33))/16
- **Assumes:** L, w, E, $I > 0$; A fully fixed, B a simple support
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('propped_cantilever_udl', L=8.0, w=10000.0, E=200000000000.0, I=0.0001)
# -> RA = 50000 N, RB = 30000 N, MA = 80000 N*m, M_span = 45000 N*m, x_M_span = 5 m, max_deflection_mm = 11.0922, x_max = 4.62772 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Propped cantilever with uniform load: L = 8 m, w = 10 kN/m, E = 200 GPa, I = 1e-4 m^4 | SOLVED | RA = 50000 N, RB = 30000 N, MA = 80000 N*m, M_span = 45000 N*m, x_M_span = 5 m, max_deflection_mm = 11.0922, x_max = 4.62772 m |

<a id="propped_cantilever_point_load"></a>

### Propped cantilever with a point load — `propped_cantilever_point_load`

Reactions, fixed-end moment, moment under the load and deflection under the load of a beam fixed at A and simply supported at B with one point load.

$$
R_B = \frac{Pa^2(3L-a)}{2L^3},\quad M_A = \frac{Pab(L+b)}{2L^2}
$$

| input | meaning | condition |
|---|---|---|
| `L` | span |  |
| `P` | point load (downward) |  |
| `a` | distance of the load from the fixed end A |  |
| `E` | Young's modulus |  |
| `I` | second moment of area |  |

| output | meaning |
|---|---|
| `RA` | reaction at the fixed end A |
| `RB` | reaction at the prop B |
| `MA` | hogging moment at A |
| `M_load` | sagging moment under the load |
| `deflection_at_load_mm` | deflection under the load in mm |

- **Method:** $RB = Pa^{2}$(3L - a)/(2L^3), $RA = P - RB$, MA = Pab(L + b)/(2L^2), M_load = RB b, delta_load = Pa^3b^2(3L + b)/(12EIL^3)
- **Assumes:** $0 < a < L$; A fully fixed, B a simple support
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('propped_cantilever_point_load', L=4.0, P=16000.0, a=2.0, E=200000000000.0, I=2e-05)
# -> RA = 11000 N, RB = 5000 N, MA = 12000 N*m, M_load = 10000 N*m, deflection_at_load_mm = 2.33333   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Propped cantilever with point load: L = 4 m, P = 16 kN, a = 2 m, E = 200 GPa, I = 2e-5 m^4 | SOLVED | RA = 11000 N, RB = 5000 N, MA = 12000 N*m, M_load = 10000 N*m, deflection_at_load_mm = 2.33333 |

<a id="rectangular_section_properties"></a>

### Section properties of a solid rectangle — `rectangular_section_properties`

Area, second moments of area, elastic section moduli, radii of gyration and polar moment of a solid rectangle b wide and h deep.

$$
I_x = \frac{bh^3}{12},\quad Z_x = \frac{bh^2}{6},\quad r_x = \sqrt{I_x/A}
$$

| input | meaning | condition |
|---|---|---|
| `b` | width (horizontal) |  |
| `h` | depth (vertical) |  |

| output | meaning |
|---|---|
| `A` | cross-sectional area |
| `Ix` | second moment of area about the horizontal centroidal axis |
| `Iy` | second moment of area about the vertical centroidal axis |
| `Zx` | elastic section modulus about x (Ix / distance to the extreme fibre) |
| `Zy` | elastic section modulus about y |
| `rx` | radius of gyration about x |
| `ry` | radius of gyration about y |
| `J_polar` | polar second moment about the centroid (Ix + Iy; not the torsion constant) |

- **Method:** $A = bh$, $Ix = bh^{3}/12$, $Iy = hb^{3}/12$, $Zx = bh^{2}/6$, $Zy = hb^{2}/6$, $r = \sqrt{\frac{I}{A}}$
- **Assumes:** b, $h > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('rectangular_section_properties', b=0.1, h=0.2)
# -> A = 0.02 m^2, Ix = 6.66667e-05 m^4, Iy = 1.66667e-05 m^4, Zx = 0.000666667 m^3, Zy = 0.000333333 m^3, rx = 0.057735 m, ry = 0.0288675 m, J_polar = 8.33333e-05 m^4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rectangular section: b = 100 mm, h = 200 mm | SOLVED | A = 0.02 m^2, Ix = 6.66667e-05 m^4, Iy = 1.66667e-05 m^4, Zx = 0.000666667 m^3, Zy = 0.000333333 m^3, rx = 0.057735 m, ry = 0.0288675 m, J_polar = 8.33333e-05 m^4 |
| Rectangular section: b = 100 mm, h = 0 mm | INVALID_INPUT | Section properties of a solid rectangle: b and h must be a positive number. |

<a id="circular_section_properties"></a>

### Section properties of a solid circle — `circular_section_properties`

Area, second moment of area, polar moment, elastic and plastic section moduli and radius of gyration of a solid circular section.

$$
I = \frac{\pi d^4}{64},\quad J = \frac{\pi d^4}{32},\quad Z = \frac{\pi d^3}{32}
$$

| input | meaning | condition |
|---|---|---|
| `d` | diameter |  |

| output | meaning |
|---|---|
| `A` | area |
| `I` | second moment of area about a diameter |
| `J` | polar second moment of area (torsion constant of a circle) |
| `Z` | elastic section modulus |
| `Zp` | plastic section modulus |
| `r` | radius of gyration |

- **Method:** $A = pi$ $d^{2}/4$, $I = pi$ $d^{4}/64$, $J = pi$ $d^{4}/32$, $Z = pi$ $d^{3}/32$, $Zp = d^{3}/6$, $r = d/4$
- **Assumes:** $d > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('circular_section_properties', d=0.05)
# -> A = 0.0019635 m^2, I = 3.06796e-07 m^4, J = 6.13592e-07 m^4, Z = 1.22718e-05 m^3, Zp = 2.08333e-05 m^3, r = 0.0125 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Circular section: d = 50 mm | SOLVED | A = 0.0019635 m^2, I = 3.06796e-07 m^4, J = 6.13592e-07 m^4, Z = 1.22718e-05 m^3, Zp = 2.08333e-05 m^3, r = 0.0125 m |

<a id="hollow_circular_section_properties"></a>

### Section properties of a hollow circle (tube) — `hollow_circular_section_properties`

Area, second moment of area, polar moment, elastic and plastic section moduli and radius of gyration of a circular tube.

$$
I = \frac{\pi(D^4-d^4)}{64},\quad J = \frac{\pi(D^4-d^4)}{32}
$$

| input | meaning | condition |
|---|---|---|
| `D` | outer diameter |  |
| `d` | inner diameter |  |

| output | meaning |
|---|---|
| `A` | area |
| `I` | second moment of area about a diameter |
| `J` | polar second moment of area (torsion constant) |
| `Z` | elastic section modulus |
| `Zp` | plastic section modulus |
| `r` | radius of gyration |

- **Method:** $A = pi$(D^2 - d^2)/4, $I = pi$(D^4 - d^4)/64, $J = 2I$, $Z = 2I/D$, $Zp = (D^{3} - d^{3})/6$, $r = \frac{\sqrt{D^{2} + d^{2}}}{4}$
- **Assumes:** $D > d  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('hollow_circular_section_properties', D=0.06, d=0.04)
# -> A = 0.0015708 m^2, I = 5.10509e-07 m^4, J = 1.02102e-06 m^4, Z = 1.7017e-05 m^3, Zp = 2.53333e-05 m^3, r = 0.0180278 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hollow circular section: D = 60 mm, d = 40 mm | SOLVED | A = 0.0015708 m^2, I = 5.10509e-07 m^4, J = 1.02102e-06 m^4, Z = 1.7017e-05 m^3, Zp = 2.53333e-05 m^3, r = 0.0180278 m |
| Hollow circular section: D = 40 mm, d = 60 mm | INVALID_INPUT | Section properties of a hollow circle (tube): the inner diameter must be smaller than the outer diameter. |

<a id="hollow_rectangular_section_properties"></a>

### Section properties of a hollow rectangle (box) — `hollow_rectangular_section_properties`

Area, second moments, elastic section moduli, radii of gyration and the plastic modulus about x of a rectangular hollow section of uniform wall thickness.

$$
I_x = \frac{BH^3 - bh^3}{12},\quad Z_{p,x} = \frac{BH^2 - bh^2}{4}
$$

| input | meaning | condition |
|---|---|---|
| `B` | outer width |  |
| `H` | outer depth |  |
| `t` | wall thickness |  |

| output | meaning |
|---|---|
| `A` | cross-sectional area |
| `Ix` | second moment of area about the horizontal centroidal axis |
| `Iy` | second moment of area about the vertical centroidal axis |
| `Zx` | elastic section modulus about x (Ix / distance to the extreme fibre) |
| `Zy` | elastic section modulus about y |
| `rx` | radius of gyration about x |
| `ry` | radius of gyration about y |
| `Zpx` | plastic section modulus about x |

- **Method:** outer rectangle minus inner rectangle: $Ix = (BH^{3} - bh^{3})/12$ with $b = B - 2t$, $h = H - 2t$; Zpx = (BH^2 - bh^2)/4
- **Assumes:** B, H, $t > 0$; 2t < min(B, H); square corners (no corner radii)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('hollow_rectangular_section_properties', B=0.1, H=0.2, t=0.01)
# -> A = 0.0056 m^2, Ix = 2.77867e-05 m^4, Iy = 8.98667e-06 m^4, Zx = 0.000277867 m^3, Zy = 0.000179733 m^3, rx = 0.0704408 m, ry = 0.0400595 m, Zpx = 0.000352 m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Box section: B = 100 mm, H = 200 mm, t = 10 mm | SOLVED | A = 0.0056 m^2, Ix = 2.77867e-05 m^4, Iy = 8.98667e-06 m^4, Zx = 0.000277867 m^3, Zy = 0.000179733 m^3, rx = 0.0704408 m, ry = 0.0400595 m, Zpx = 0.000352 m^3 |
| Box section: B = 100 mm, H = 200 mm, t = 60 mm | INVALID_INPUT | Section properties of a hollow rectangle (box): the wall thickness must be less than half the smaller outer dimension. |

<a id="i_section_properties"></a>

### Section properties of a symmetric I-section — `i_section_properties`

Area, second moments, elastic and plastic section moduli, radii of gyration and the shape factor of a doubly symmetric I-section (flanges b x tf, web tw, overall depth h, no fillets).

$$
I_x = \frac{bh^3 - (b-t_w)(h-2t_f)^3}{12},\quad Z_{p,x} = bt_f(h-t_f) + \frac{t_w(h-2t_f)^2}{4}
$$

| input | meaning | condition |
|---|---|---|
| `b` | flange width |  |
| `h` | overall depth |  |
| `tf` | flange thickness |  |
| `tw` | web thickness |  |

| output | meaning |
|---|---|
| `A` | cross-sectional area |
| `Ix` | second moment of area about the horizontal centroidal axis |
| `Iy` | second moment of area about the vertical centroidal axis |
| `Zx` | elastic section modulus about x (Ix / distance to the extreme fibre) |
| `Zy` | elastic section modulus about y |
| `rx` | radius of gyration about x |
| `ry` | radius of gyration about y |
| `Zpx` | plastic section modulus about x |
| `Zpy` | plastic section modulus about y |
| `shape_factor_x` | shape factor Zpx / Zx |

- **Method:** three rectangles with the parallel-axis theorem: Ix = sum($bh^{3}/12 + A$ d^2); plastic moduli about the axes of symmetry: Zpx = b tf (h - tf) + tw $(h - 2tf)^{2}/4$, Zpy = tf $b^{2}/2 + (h - 2tf)$ $tw^{2}/4$
- **Assumes:** 2 $tf < h$; $tw < b$; no root fillets
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('i_section_properties', b=0.15, h=0.3, tf=0.012, tw=0.008)
# -> A = 0.005808 m^2, Ix = 8.87092e-05 m^4, Iy = 6.76178e-06 m^4, Zx = 0.000591395 m^3, Zy = 9.0157e-05 m^3, rx = 0.123586 m, ry = 0.0341206 m, Zpx = 0.000670752 m^3, Zpy = 0.000139416 m^3, shape_factor_x = 1.13419   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| I-section: b = 150 mm, h = 300 mm, tf = 12 mm, tw = 8 mm | SOLVED | A = 0.005808 m^2, Ix = 8.87092e-05 m^4, Iy = 6.76178e-06 m^4, Zx = 0.000591395 m^3, Zy = 9.0157e-05 m^3, rx = 0.123586 m, ry = 0.0341206 m, Zpx = 0.000670752 m^3, Zpy = 0.000139416 m^3, shape_factor_x = 1.13419 |
| I-section: b = 150 mm, h = 300 mm, tf = 160 mm, tw = 8 mm | INVALID_INPUT | Section properties of a symmetric I-section: the flanges (2 tf) must be thinner than the overall depth h. |

<a id="t_section_properties"></a>

### Section properties of a T-section — `t_section_properties`

Area, centroid height, second moments, top and bottom section moduli and radii of gyration of a T-section (flange on top).

$$
\bar y = \frac{\sum A_i y_i}{\sum A_i},\quad I_x = \sum\Big(\frac{b_ih_i^3}{12} + A_i(y_i - \bar y)^2\Big)
$$

| input | meaning | condition |
|---|---|---|
| `bf` | flange width |  |
| `tf` | flange thickness |  |
| `h` | overall depth |  |
| `tw` | stem (web) thickness |  |

| output | meaning |
|---|---|
| `A` | area |
| `y_bar` | height of the centroid above the bottom of the stem |
| `Ix` | second moment about the horizontal centroidal axis |
| `Iy` | second moment about the vertical axis of symmetry |
| `Z_top` | section modulus for the top fibre |
| `Z_bottom` | section modulus for the bottom fibre |
| `rx` | radius of gyration about x |
| `ry` | radius of gyration about y |

- **Method:** two rectangles: y_bar = sum(A y)/sum(A); Ix = sum($bh^{3}/12 + A$ (y - y_bar)^2); Z = Ix / distance to the fibre
- **Assumes:** $tf < h$; $tw  \leq  bf$; no fillets
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('t_section_properties', bf=0.12, tf=0.02, h=0.15, tw=0.015)
# -> A = 0.00435 m^2, y_bar = 0.106379 m, Ix = 8.87797e-06 m^4, Iy = 2.91656e-06 m^4, Z_top = 0.000203527 m^3, Z_bottom = 8.34558e-05 m^3, rx = 0.0451765 m, ry = 0.0258935 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| T-section: bf = 120 mm, tf = 20 mm, h = 150 mm, tw = 15 mm | SOLVED | A = 0.00435 m^2, y_bar = 0.106379 m, Ix = 8.87797e-06 m^4, Iy = 2.91656e-06 m^4, Z_top = 0.000203527 m^3, Z_bottom = 8.34558e-05 m^3, rx = 0.0451765 m, ry = 0.0258935 m |
| T-section: bf = 120 mm, tf = 200 mm, h = 150 mm, tw = 15 mm | INVALID_INPUT | Section properties of a T-section: the flange thickness must be less than the overall depth. |

<a id="channel_section_properties"></a>

### Section properties of a channel (C) section — `channel_section_properties`

Area, centroid distance from the back of the web, second moments, section moduli and radii of gyration of a channel with square corners.

$$
\bar x = \frac{\sum A_i x_i}{\sum A_i},\quad I_y = \sum\Big(\frac{h_ib_i^3}{12} + A_i(x_i-\bar x)^2\Big)
$$

| input | meaning | condition |
|---|---|---|
| `b` | flange width (including the web) |  |
| `h` | overall depth |  |
| `tf` | flange thickness |  |
| `tw` | web thickness |  |

| output | meaning |
|---|---|
| `A` | area |
| `x_bar` | distance of the centroid from the back of the web |
| `Ix` | second moment about the horizontal axis of symmetry |
| `Iy` | second moment about the vertical centroidal axis |
| `Zx` | section modulus about x |
| `Zy_min` | smaller section modulus about y (flange tips) |
| `rx` | radius of gyration about x |
| `ry` | radius of gyration about y |

- **Method:** three rectangles (two flanges b x tf, web tw x $(h - 2tf)$) with the parallel-axis theorem
- **Assumes:** 2 $tf < h$; $tw < b$; square corners, no taper
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('channel_section_properties', b=0.08, h=0.2, tf=0.01, tw=0.007)
# -> A = 0.00286 m^2, x_bar = 0.0239196 m, Ix = 1.78553e-05 m^4, Iy = 1.79757e-06 m^4, Zx = 0.000178553 m^3, Zy_min = 3.20535e-05 m^3, rx = 0.0790134 m, ry = 0.0250704 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Channel section: b = 80 mm, h = 200 mm, tf = 10 mm, tw = 7 mm | SOLVED | A = 0.00286 m^2, x_bar = 0.0239196 m, Ix = 1.78553e-05 m^4, Iy = 1.79757e-06 m^4, Zx = 0.000178553 m^3, Zy_min = 3.20535e-05 m^3, rx = 0.0790134 m, ry = 0.0250704 m |

<a id="angle_section_properties"></a>

### Section properties of an angle (L) section, with principal axes — `angle_section_properties`

Area, centroid, centroidal second moments and product of inertia, principal second moments, the principal angle and the least radius of gyration of an equal or unequal angle.

$$
I_{1,2} = \frac{I_x+I_y}{2} \pm \sqrt{\Big(\frac{I_x-I_y}{2}\Big)^2 + I_{xy}^2},\quad \tan2\varphi = \frac{-2I_{xy}}{I_x-I_y}
$$

| input | meaning | condition |
|---|---|---|
| `b` | horizontal leg length |  |
| `h` | vertical leg length |  |
| `t` | thickness (both legs) |  |

| output | meaning |
|---|---|
| `A` | area |
| `x_bar` | centroid distance from the back of the vertical leg |
| `y_bar` | centroid distance from the bottom of the horizontal leg |
| `Ix` | centroidal second moment about x |
| `Iy` | centroidal second moment about y |
| `Ixy` | centroidal product of inertia (integral of x y dA) |
| `I1` | major principal second moment |
| `I2` | minor principal second moment |
| `principal_angle_deg` | angle of the major principal axis from x, degrees, in (-90, 90] |
| `r_min` | least radius of gyration |

- **Method:** two rectangles with the parallel-axis theorem (also for Ixy); I1,$2 = (Ix + Iy)/2$ +- sqrt(((Ix - Iy)/2)^2 + Ixy^2), tan(2 phi) = -2 Ixy/(Ix - Iy)
- **Assumes:** $t < b$ and $t < h$; square corners, no root radius
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('angle_section_properties', b=0.1, h=0.1, t=0.01)
# -> A = 0.0019 m^2, x_bar = 0.0286842 m, y_bar = 0.0286842 m, Ix = 1.80004e-06 m^4, Iy = 1.80004e-06 m^4, Ixy = -1.06579e-06 m^4, I1 = 2.86583e-06 m^4, I2 = 7.34254e-07 m^4, principal_angle_deg = 45, r_min = 0.0196583 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Angle section: b = 100 mm, h = 100 mm, t = 10 mm | SOLVED | A = 0.0019 m^2, x_bar = 0.0286842 m, y_bar = 0.0286842 m, Ix = 1.80004e-06 m^4, Iy = 1.80004e-06 m^4, Ixy = -1.06579e-06 m^4, I1 = 2.86583e-06 m^4, I2 = 7.34254e-07 m^4, principal_angle_deg = 45, r_min = 0.0196583 m |
| Angle section: b = 100 mm, h = 150 mm, t = 200 mm | INVALID_INPUT | Section properties of an angle (L) section, with principal axes: the thickness must be less than both leg lengths. |

<a id="composite_section_properties"></a>

### Composite section of rectangles (parallel-axis theorem) — `composite_section_properties`

Area, centroid, centroidal second moments, product of inertia and principal second moments of a section built from non-overlapping rectangles.

$$
I_x = \sum_i\Big(\frac{b_ih_i^3}{12} + A_i(y_i-\bar y)^2\Big),\quad I_{xy} = \sum_i A_i(x_i-\bar x)(y_i-\bar y)
$$

| input | meaning | condition |
|---|---|---|
| `rects` | one row per rectangle: width b, height h, centre x, centre y, all in mm |  |

| output | meaning |
|---|---|
| `A` | area in $mm^{2}$ |
| `x_bar` | centroid x in mm |
| `y_bar` | centroid y in mm |
| `Ix` | centroidal second moment about x in $mm^{4}$ |
| `Iy` | centroidal second moment about y in $mm^{4}$ |
| `Ixy` | centroidal product of inertia in $mm^{4}$ |
| `I1` | major principal second moment in $mm^{4}$ |
| `I2` | minor principal second moment in $mm^{4}$ |

- **Method:** parallel-axis theorem: x_bar = sum A x / sum A, Ix = sum($bh^{3}/12 + A$ (y - y_bar)^2), Ixy = sum A (x - x_bar)(y - y_bar)
- **Assumes:** rectangles do not overlap (touching is allowed); 1 to 20 rectangles, all dimensions in mm
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('composite_section_properties', rects='200 20 0 10; 20 300 0 170')
# -> A = 10000, x_bar = 0, y_bar = 106, Ix = 1.06573e+08, Iy = 1.35333e+07, Ixy = 0, I1 = 1.06573e+08, I2 = 1.35333e+07   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Composite section: rects = [200 20 0 10; 20 300 0 170] | SOLVED | A = 10000, x_bar = 0, y_bar = 106, Ix = 1.06573e+08, Iy = 1.35333e+07, Ixy = 0, I1 = 1.06573e+08, I2 = 1.35333e+07 |
| Composite section: rects = [200 20 0 10; 20 300 0 15] | INVALID_INPUT | Composite section of rectangles (parallel-axis theorem): rectangles 1 and 2 overlap: the parts of a composite section mu |

<a id="polygon_section_properties"></a>

### Section properties of a polygonal section — `polygon_section_properties`

Area, centroid, centroidal second moments, product of inertia and principal second moments of any simple polygon given by its vertices.

$$
A = \tfrac12\sum c_i,\quad I_x = \tfrac1{12}\sum c_i\,(y_i^2+y_iy_{i+1}+y_{i+1}^2) - A\bar y^2
$$

| input | meaning | condition |
|---|---|---|
| `vertices` | vertices in order (either sense), one row 'x y' per vertex, in mm |  |

| output | meaning |
|---|---|
| `A` | area in $mm^{2}$ |
| `x_bar` | centroid x in mm |
| `y_bar` | centroid y in mm |
| `Ix` | centroidal second moment about x in $mm^{4}$ |
| `Iy` | centroidal second moment about y in $mm^{4}$ |
| `Ixy` | centroidal product of inertia in $mm^{4}$ |
| `I1` | major principal second moment in $mm^{4}$ |
| `I2` | minor principal second moment in $mm^{4}$ |

- **Method:** Green's theorem over the edges: $A = 1/2$ sum c_i, $Ix = 1/12$ sum c_i (y_i^2 + y_i y_i+1 + y_i+1^2) with $c_i = x_i$ $y_i+1 - x_i+1$ y_i; then shifted to the centroid
- **Assumes:** 3 to 60 distinct vertices; a simple polygon (edges do not cross); no holes
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('polygon_section_properties', vertices='0 0; 100 0; 100 200; 0 200')
# -> A = 20000, x_bar = 50, y_bar = 100, Ix = 6.66667e+07, Iy = 1.66667e+07, Ixy = 0, I1 = 6.66667e+07, I2 = 1.66667e+07   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('polygon_section_properties', vertices='0 0; 0 30; 40 0')
# -> A = 600, x_bar = 13.3333, y_bar = 10, Ix = 30000, Iy = 53333.3, Ixy = -20000, I1 = 64820.7, I2 = 18512.6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Polygon section: vertices = [0 0; 100 0; 100 200; 0 200] | SOLVED | A = 20000, x_bar = 50, y_bar = 100, Ix = 6.66667e+07, Iy = 1.66667e+07, Ixy = 0, I1 = 6.66667e+07, I2 = 1.66667e+07 |
| Polygon section: vertices = [0 0; 100 100; 100 0; 0 100] | INVALID_INPUT | Section properties of a polygonal section: the polygon must be simple (its edges must not cross). |

<a id="solid_shaft_torsion"></a>

### Torsion of a solid circular shaft — `solid_shaft_torsion`

Maximum shear stress, polar moment and angle of twist of a solid circular shaft under a torque.

$$
\tau_{max} = \frac{16T}{\pi d^3},\quad \varphi = \frac{TL}{GJ},\quad J = \frac{\pi d^4}{32}
$$

| input | meaning | condition |
|---|---|---|
| `T` | applied torque |  |
| `d` | diameter |  |
| `L` | length |  |
| `G` | shear modulus |  |

| output | meaning |
|---|---|
| `tau_max_MPa` | maximum shear stress (outer surface) in MPa |
| `tau_inner_MPa` | shear stress at the inner surface in MPa (0 for a solid shaft) |
| `twist_rad` | angle of twist |
| `twist_deg` | angle of twist in degrees |
| `J` | polar second moment of area |

- **Method:** $J = pi$ $d^{4}/32$, tau_max = T (d/2)/J = 16T/(pi d^3), $\phi = TL/(GJ)$
- **Assumes:** T, d, L, $G > 0$; linear elastic, circular section, uniform torque
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('solid_shaft_torsion', T=1200.0, d=0.05, L=1.5, G=80000000000.0)
# -> tau_max_MPa = 48.8924, tau_inner_MPa = 0, twist_rad = 0.0366693 rad, twist_deg = 2.101, J = 6.13592e-07 m^4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Solid shaft torsion: T = 1.2 kN*m, d = 50 mm, L = 1.5 m, G = 80 GPa | SOLVED | tau_max_MPa = 48.8924, tau_inner_MPa = 0, twist_rad = 0.0366693 rad, twist_deg = 2.101, J = 6.13592e-07 m^4 |
| Solid shaft torsion: T = 1.2 kN*m, d = 50 mm, L = 1.5 m | NEEDS_INPUT | solid_shaft_torsion needs G (shear modulus). Example: solve('solid_shaft_torsion', T='1200 N*m', d='0.05 m', L='1.5 m',  |

<a id="hollow_shaft_torsion"></a>

### Torsion of a hollow circular shaft — `hollow_shaft_torsion`

Shear stresses at the outer and inner surfaces, polar moment and angle of twist of a hollow circular shaft under a torque.

$$
\tau_{max} = \frac{16TD}{\pi(D^4-d^4)},\quad \varphi = \frac{TL}{GJ}
$$

| input | meaning | condition |
|---|---|---|
| `T` | applied torque |  |
| `D` | outer diameter |  |
| `d` | inner diameter |  |
| `L` | length |  |
| `G` | shear modulus |  |

| output | meaning |
|---|---|
| `tau_max_MPa` | maximum shear stress (outer surface) in MPa |
| `tau_inner_MPa` | shear stress at the inner surface in MPa (0 for a solid shaft) |
| `twist_rad` | angle of twist |
| `twist_deg` | angle of twist in degrees |
| `J` | polar second moment of area |

- **Method:** $J = pi$ $(D^{4} - d^{4})/32$, tau_max = T $(D/2)/J$, tau_inner = T $(d/2)/J$, $\phi = TL/(GJ)$
- **Assumes:** $D > d > 0$; linear elastic
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('hollow_shaft_torsion', T=2000.0, D=0.06, d=0.04, L=2.0, G=80000000000.0)
# -> tau_max_MPa = 58.7649, tau_inner_MPa = 39.1766, twist_rad = 0.0489708 rad, twist_deg = 2.80582, J = 1.02102e-06 m^4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hollow shaft torsion: T = 2 kN*m, D = 60 mm, d = 40 mm, L = 2 m, G = 80 GPa | SOLVED | tau_max_MPa = 58.7649, tau_inner_MPa = 39.1766, twist_rad = 0.0489708 rad, twist_deg = 2.80582, J = 1.02102e-06 m^4 |
| Hollow shaft torsion: T = 2 kN*m, D = 40 mm, d = 60 mm, L = 2 m, G = 80 GPa | INVALID_INPUT | Torsion of a hollow circular shaft: the inner diameter must be smaller than the outer diameter. |

<a id="shaft_power_transmission"></a>

### Torque and minimum diameter of a power-transmission shaft — `shaft_power_transmission`

The torque a solid shaft transmits at a given power and speed, and the smallest diameter that keeps the torsional shear stress within an allowable value.

$$
T = \frac{P}{\omega},\quad d_{min} = \Big(\frac{16T}{\pi\tau_{allow}}\Big)^{1/3}
$$

| input | meaning | condition |
|---|---|---|
| `P` | transmitted power |  |
| `n` | rotational speed (rpm or rad/s) |  |
| `tau_allow` | allowable shear stress |  |

| output | meaning |
|---|---|
| `T` | transmitted torque |
| `d_min_mm` | minimum solid-shaft diameter in mm |
| `frequency_Hz` | rotational frequency in Hz |

- **Method:** $T = P/\omega$, d_min = (16T/(pi tau_allow))^$(1/3)$
- **Assumes:** P, n, tau_allow > 0; pure torsion (no bending), solid circular shaft
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('shaft_power_transmission', P=50000.0, n=157.07963267948966, tau_allow=40000000.0)
# -> T = 318.31 N*m, d_min_mm = 34.3495, frequency_Hz = 25   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shaft power transmission: P = 50 kW, n = 1500 rpm, tau_allow = 40 MPa | SOLVED | T = 318.31 N*m, d_min_mm = 34.3495, frequency_Hz = 25 |
| Shaft power transmission: P = 50 kW, n = 1500 rpm, tau_allow = 0 MPa | INVALID_INPUT | Torque and minimum diameter of a power-transmission shaft: P, n and the allowable shear stress must be a positive number |

<a id="stepped_shaft_twist"></a>

### Angle of twist of a stepped shaft — `stepped_shaft_twist`

Total angle of twist and the largest shear stress of a shaft made of solid circular segments in series carrying one torque.

$$
\varphi = \sum_i\frac{TL_i}{GJ_i},\quad J_i = \frac{\pi d_i^4}{32}
$$

| input | meaning | condition |
|---|---|---|
| `T` | torque carried by every segment |  |
| `lengths` | segment lengths in mm (a list) |  |
| `diameters` | segment diameters in mm (a list, same order) |  |
| `G` | shear modulus |  |

| output | meaning |
|---|---|
| `twist_rad` | total angle of twist |
| `twist_deg` | total angle of twist in degrees |
| `tau_max_MPa` | largest shear stress (smallest segment) in MPa |

- **Method:** phi = sum T L_i/(G J_i), $J_i = pi$ $d_i^{4}/32$, tau_max = max 16T/(pi d_i^3)
- **Assumes:** equal-length lists; the same torque in every segment; stress concentrations at the steps not included
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('stepped_shaft_twist', T=500.0, lengths='400, 600', diameters='40, 30', G=80000000000.0)
# -> twist_rad = 0.0571042 rad, twist_deg = 3.27183, tau_max_MPa = 94.314   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Stepped shaft: T = 500 N*m, lengths = [400, 600], diameters = [40, 30], G = 80 GPa | SOLVED | twist_rad = 0.0571042 rad, twist_deg = 3.27183, tau_max_MPa = 94.314 |
| Stepped shaft: T = 500 N*m, lengths = [400, 600], diameters = [40], G = 80 GPa | INVALID_INPUT | Angle of twist of a stepped shaft: lengths and diameters must have the same number of entries. |

<a id="statically_indeterminate_shaft"></a>

### Shaft fixed at both ends with a torque between them — `statically_indeterminate_shaft`

The reaction torques at both fixed ends, the shear stress in each part and the rotation at the loaded section of a shaft built in at both ends.

$$
T_A = T_0\frac{b/J_2}{a/J_1 + b/J_2},\quad T_B = T_0 - T_A
$$

| input | meaning | condition |
|---|---|---|
| `T0` | torque applied between the ends |  |
| `a` | length of part 1 (from end A to the load) |  |
| `b` | length of part 2 (from the load to end B) |  |
| `d1` | diameter of part 1 |  |
| `d2` | diameter of part 2 |  |
| `G` | shear modulus |  |

| output | meaning |
|---|---|
| `TA` | reaction torque at A |
| `TB` | reaction torque at B |
| `tau1_MPa` | maximum shear stress in part 1, MPa |
| `tau2_MPa` | maximum shear stress in part 2, MPa |
| `twist_at_load_deg` | rotation of the loaded section in degrees |

- **Method:** compatibility: TA a/(G J1) = TB b/(G J2), equilibrium $TA + TB = T_{0}$
- **Assumes:** both ends fully fixed; solid circular parts of one material
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('statically_indeterminate_shaft', T0=1000.0, a=0.5, b=1.5, d1=0.05, d2=0.05, G=80000000000.0)
# -> TA = 750 N*m, TB = 250 N*m, tau1_MPa = 30.5577, tau2_MPa = 10.1859, twist_at_load_deg = 0.437708   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Statically indeterminate shaft: T0 = 1 kN*m, a = 0.5 m, b = 1.5 m, d1 = 50 mm, d2 = 50 mm, G = 80 GPa | SOLVED | TA = 750 N*m, TB = 250 N*m, tau1_MPa = 30.5577, tau2_MPa = 10.1859, twist_at_load_deg = 0.437708 |

<a id="axial_bar_segments"></a>

### Stepped bar under axial loads at its joints — `axial_bar_segments`

Internal axial force, normal stress of every segment and the total elongation of a bar fixed at its left end, made of segments in series, with axial loads at the joints.

$$
\delta = \sum_i \frac{N_iL_i}{A_iE},\quad \sigma_i = \frac{N_i}{A_i}
$$

| input | meaning | condition |
|---|---|---|
| `lengths` | segment lengths in mm, from the fixed end (a list) |  |
| `areas` | segment cross-sectional areas in $mm^{2}$ (a list) |  |
| `loads` | axial load at the far end of each segment in kN, positive pulling away from the support (a list) |  |
| `E` | Young's modulus (all segments) |  |

| output | meaning |
|---|---|
| `elongation_mm` | total elongation of the bar in mm (negative = shortening) |
| `N_kN` | internal axial force of each segment in kN (tension positive) |
| `stress_MPa` | normal stress of each segment in MPa |
| `max_abs_stress_MPa` | largest stress magnitude in MPa |

- **Method:** N_i = sum of the loads beyond segment i; $sigma_i = N_i/A_i$; delta = sum N_i L_i/(A_i E)
- **Assumes:** equal-length lists; left end fixed, right end free; linear elastic, no buckling
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('axial_bar_segments', lengths='300, 500', areas='400, 200', loads='-20, 30', E=200000000000.0)
# -> elongation_mm = 0.4125, N_kN = [10.0, 30.0], stress_MPa = [25.0, 150.0], max_abs_stress_MPa = 150   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Stepped bar: lengths = [300, 500], areas = [400, 200], loads = [-20, 30], E = 200 GPa | SOLVED | elongation_mm = 0.4125, N_kN = [10.0, 30.0], stress_MPa = [25.0, 150.0], max_abs_stress_MPa = 150 |
| Stepped bar: lengths = [300, 500], areas = [400], loads = [-20, 30], E = 200 GPa | INVALID_INPUT | Stepped bar under axial loads at its joints: lengths, areas and loads must have the same number of entries (one per segm |

<a id="bar_fixed_at_both_ends"></a>

### Bar fixed at both ends with an axial load at a step — `bar_fixed_at_both_ends`

Reactions, stresses in both parts and the displacement of the loaded joint of a two-part bar held between two rigid supports, loaded axially at the joint.

$$
R_A = P\frac{k_1}{k_1+k_2},\quad k_i = \frac{A_iE_i}{L_i},\quad \delta = \frac{P}{k_1+k_2}
$$

| input | meaning | condition |
|---|---|---|
| `P` | axial load at the joint (towards end B) |  |
| `L1` | length of part 1 (end A to the joint) |  |
| `A1` | area of part 1 |  |
| `E1` | modulus of part 1 |  |
| `L2` | length of part 2 |  |
| `A2` | area of part 2 |  |
| `E2` | modulus of part 2 |  |

| output | meaning |
|---|---|
| `RA` | reaction at A |
| `RB` | reaction at B |
| `stress1_MPa` | stress in part 1 in MPa (tension positive) |
| `stress2_MPa` | stress in part 2 in MPa (compression negative) |
| `displacement_mm` | displacement of the joint in mm |

- **Method:** compatibility: the joint moves the same amount in both parts; $RA = P$ $k_{1}/(k_{1} + k_{2})$, $k = AE/L$
- **Assumes:** rigid supports, no gap; linear elastic, no buckling of the compressed part
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('bar_fixed_at_both_ends', P=60000.0, L1=0.4, A1=0.0005, E1=200000000000.0, L2=0.4, A2=0.0005, E2=200000000000.0)
# -> RA = 30000 N, RB = 30000 N, stress1_MPa = 60, stress2_MPa = -60, displacement_mm = 0.12   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bar fixed at both ends: P = 60 kN, L1 = 0.4 m, A1 = 500 mm^2, E1 = 200 GPa, L2 = 0.4 m, A2 = 500 mm^2, E2 = 200 GPa | SOLVED | RA = 30000 N, RB = 30000 N, stress1_MPa = 60, stress2_MPa = -60, displacement_mm = 0.12 |

<a id="plane_stress_transformation"></a>

### Plane stress transformation to rotated axes — `plane_stress_transformation`

The normal and shear stresses on axes rotated counterclockwise by theta from x, for a plane state of stress.

$$
\sigma_{x'} = \frac{\sigma_x+\sigma_y}{2} + \frac{\sigma_x-\sigma_y}{2}\cos2\theta + \tau_{xy}\sin2\theta
$$

| input | meaning | condition |
|---|---|---|
| `sx` | normal stress sigma_x |  |
| `sy` | normal stress sigma_y |  |
| `txy` | shear stress tau_xy |  |
| `theta` | rotation of the axes, counterclockwise from x |  |

| output | meaning |
|---|---|
| `sx_t_MPa` | sigma_x' in MPa |
| `sy_t_MPa` | sigma_y' in MPa |
| `txy_t_MPa` | tau_x'y' in MPa |

- **Method:** sigma_x' = $(sx + sy)/2 + (sx - sy)/2$ cos 2theta + txy sin 2theta; tau_x'y' = -$(sx - sy)/2$ sin 2theta + txy cos 2theta
- **Assumes:** plane stress; tension positive, theta counterclockwise
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('plane_stress_transformation', sx=80000000.0, sy=-40000000.0, txy=25000000.0, theta=0.5235987755982988)
# -> sx_t_MPa = 71.6506, sy_t_MPa = -31.6506, txy_t_MPa = -39.4615   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Plane stress transformation: sx = 80 MPa, sy = -40 MPa, txy = 25 MPa, theta = 30 deg | SOLVED | sx_t_MPa = 71.6506, sy_t_MPa = -31.6506, txy_t_MPa = -39.4615 |
| Plane stress transformation: sx = 80 MPa, sy = -40 MPa, txy = 25 MPa | NEEDS_INPUT | plane_stress_transformation needs theta (rotation of the axes, counterclockwise from x). Example: solve('plane_stress_tr |

<a id="principal_stresses_2d"></a>

### Principal stresses and maximum shear (Mohr's circle, plane stress) — `principal_stresses_2d`

Principal stresses, their direction, the maximum in-plane and absolute shear stresses and the von Mises stress of a plane state of stress.

$$
\sigma_{1,2} = \frac{\sigma_x+\sigma_y}{2} \pm \sqrt{\Big(\frac{\sigma_x-\sigma_y}{2}\Big)^2 + \tau_{xy}^2},\quad \tan2\theta_p = \frac{2\tau_{xy}}{\sigma_x-\sigma_y}
$$

| input | meaning | condition |
|---|---|---|
| `sx` | normal stress sigma_x |  |
| `sy` | normal stress sigma_y |  |
| `txy` | shear stress tau_xy |  |

| output | meaning |
|---|---|
| `s1_MPa` | larger in-plane principal stress, MPa |
| `s2_MPa` | smaller in-plane principal stress, MPa |
| `tau_max_inplane_MPa` | maximum in-plane shear stress (radius of Mohr's circle), MPa |
| `theta_p_deg` | angle from x to the sigma_1 direction, degrees in (-90, 90] (0 when the circle is a point) |
| `tau_abs_max_MPa` | absolute maximum shear stress including $sigma_z = 0$, MPa |
| `von_mises_MPa` | von Mises equivalent stress, MPa |

- **Method:** sigma_1,$2 = (sx + sy)/2$ +- R, $R = \sqrt{\left(\frac{sx - sy}{2}\right)^{2} + txy^{2}}$; tan $2theta_p = 2$ txy/(sx - sy); tau_abs = max($s_{1} - s_{2}$, |s1|, |s2|)/2
- **Assumes:** plane stress $(sigma_z = 0)$; tension positive
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('principal_stresses_2d', sx=50000000.0, sy=-10000000.0, txy=40000000.0)
# -> s1_MPa = 70, s2_MPa = -30, tau_max_inplane_MPa = 50, theta_p_deg = 26.5651, tau_abs_max_MPa = 50, von_mises_MPa = 88.8819   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('principal_stresses_2d', sx=100000000.0, sy=0.0, txy=0.0)
# -> s1_MPa = 100, s2_MPa = 0, tau_max_inplane_MPa = 50, theta_p_deg = 0, tau_abs_max_MPa = 50, von_mises_MPa = 100   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Principal stresses: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa | SOLVED | s1_MPa = 70, s2_MPa = -30, tau_max_inplane_MPa = 50, theta_p_deg = 26.5651, tau_abs_max_MPa = 50, von_mises_MPa = 88.8819 |
| Mohr's circle: sx = 50 MPa, sy = -10 MPa, txy = 40 MPa | SOLVED | s1_MPa = 70, s2_MPa = -30, tau_max_inplane_MPa = 50, theta_p_deg = 26.5651, tau_abs_max_MPa = 50, von_mises_MPa = 88.8819 |

<a id="principal_stresses_3d"></a>

### Principal stresses of a three-dimensional stress state — `principal_stresses_3d`

The three principal stresses, the absolute maximum shear, von Mises, hydrostatic and octahedral shear stresses of a general 3D state of stress.

$$
\det(\boldsymbol\sigma - \sigma\mathbf I) = 0,\quad \tau_{max} = \frac{\sigma_1-\sigma_3}{2},\quad \tau_{oct} = \frac{\sqrt2}{3}\sigma_{vm}
$$

| input | meaning | condition |
|---|---|---|
| `sx` | sigma_x |  |
| `sy` | sigma_y |  |
| `sz` | sigma_z |  |
| `txy` | tau_xy |  |
| `tyz` | tau_yz |  |
| `tzx` | tau_zx |  |

| output | meaning |
|---|---|
| `s1_MPa` | largest principal stress, MPa |
| `s2_MPa` | intermediate principal stress, MPa |
| `s3_MPa` | smallest principal stress, MPa |
| `tau_max_MPa` | absolute maximum shear stress $(s_{1} - s_{3})/2$, MPa |
| `von_mises_MPa` | von Mises stress, MPa |
| `hydrostatic_MPa` | mean (hydrostatic) stress, MPa |
| `tau_oct_MPa` | octahedral shear stress, MPa |

- **Method:** eigenvalues of the stress tensor by Jacobi rotations; $sigma_vm = sqrt$(((sx-sy)^2 + (sy-sz)^2 + (sz-sx)^2)/2 + 3(txy^2 + tyz^2 + tzx^2)); tau_oct = sqrt(2)/3 sigma_vm
- **Assumes:** symmetric stress tensor; tension positive
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('principal_stresses_3d', sx=100000000.0, sy=0.0, sz=0.0, txy=0.0, tyz=0.0, tzx=0.0)
# -> s1_MPa = 100, s2_MPa = 0, s3_MPa = 0, tau_max_MPa = 50, von_mises_MPa = 100, hydrostatic_MPa = 33.3333, tau_oct_MPa = 47.1405   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('principal_stresses_3d', sx=0.0, sy=0.0, sz=0.0, txy=50000000.0, tyz=0.0, tzx=0.0)
# -> s1_MPa = 50, s2_MPa = 0, s3_MPa = -50, tau_max_MPa = 50, von_mises_MPa = 86.6025, hydrostatic_MPa = 0, tau_oct_MPa = 40.8248   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Principal stresses 3D: sx = 100 MPa, sy = 0 MPa, sz = 0 MPa, txy = 0 MPa, tyz = 0 MPa, tzx = 0 MPa | SOLVED | s1_MPa = 100, s2_MPa = 0, s3_MPa = 0, tau_max_MPa = 50, von_mises_MPa = 100, hydrostatic_MPa = 33.3333, tau_oct_MPa = 47.1405 |
| Principal stresses 3D: sx = 100 MPa, sy = 0 MPa, sz = 0 MPa | NEEDS_INPUT | principal_stresses_3d needs txy, tyz, tzx (tau_xy; tau_yz; tau_zx). Example: solve('principal_stresses_3d', sx='1e+08 Pa |

<a id="yield_criteria_safety_factor"></a>

### Von Mises and Tresca equivalent stresses and safety factors — `yield_criteria_safety_factor`

Equivalent stresses by the distortion-energy (von Mises) and maximum-shear (Tresca) criteria and the corresponding factors of safety against yielding.

$$
\sigma_{vm} = \sqrt{\tfrac12\big[(\sigma_1-\sigma_2)^2+(\sigma_2-\sigma_3)^2+(\sigma_3-\sigma_1)^2\big]},\quad n = \frac{S_y}{\sigma_{eq}}
$$

| input | meaning | condition |
|---|---|---|
| `Sy` | yield strength |  |
| `sx` | sigma_x |  |
| `sy` | sigma_y (default 0) |  |
| `sz` | sigma_z (default 0) |  |
| `txy` | tau_xy (default 0) |  |
| `tyz` | tau_yz (default 0) |  |
| `tzx` | tau_zx (default 0) |  |

| output | meaning |
|---|---|
| `von_mises_MPa` | von Mises stress, MPa |
| `tresca_MPa` | Tresca equivalent stress $(s_{1} - s_{3})$, MPa |
| `n_von_mises` | safety factor by distortion energy |
| `n_tresca` | safety factor by maximum shear |

- **Method:** $sigma_vm = sqrt$(((sx-sy)^2 + (sy-sz)^2 + (sz-sx)^2)/2 + 3(txy^2+tyz^2+tzx^2)), $sigma_T = s_{1} - s_{3}$, $n = Sy/sigma_eq$
- **Assumes:** $Sy > 0$; not a purely hydrostatic stress; ductile, isotropic material
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('yield_criteria_safety_factor', Sy=350000000.0, sx=120000000.0, sy=-40000000.0, txy=60000000.0)
# -> von_mises_MPa = 177.764, tresca_MPa = 200, n_von_mises = 1.9689, n_tresca = 1.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Yield safety factor: Sy = 350 MPa, sx = 120 MPa, sy = -40 MPa, txy = 60 MPa | SOLVED | von_mises_MPa = 177.764, tresca_MPa = 200, n_von_mises = 1.9689, n_tresca = 1.75 |
| Yield safety factor: Sy = 350 MPa, sx = 100 MPa, sy = 100 MPa, sz = 100 MPa | INVALID_INPUT | Von Mises and Tresca equivalent stresses and safety factors: the stress state is purely hydrostatic: neither criterion p |

<a id="brittle_failure_mohr"></a>

### Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr) — `brittle_failure_mohr`

Factors of safety of a brittle material under plane stress by the maximum-normal-stress, brittle Coulomb-Mohr and modified-Mohr criteria (proportional loading).

$$
\frac{\sigma_1}{S_{ut}} - \frac{\sigma_3}{S_{uc}} = \frac1n,\quad \frac{(S_{uc}-S_{ut})\sigma_A}{S_{uc}S_{ut}} - \frac{\sigma_B}{S_{uc}} = \frac1n
$$

| input | meaning | condition |
|---|---|---|
| `sA` | first in-plane principal stress |  |
| `sB` | second in-plane principal stress |  |
| `Sut` | ultimate tensile strength |  |
| `Suc` | ultimate compressive strength (positive number) |  |

| output | meaning |
|---|---|
| `n_max_normal` | safety factor, maximum normal stress theory |
| `n_coulomb_mohr` | safety factor, brittle Coulomb-Mohr |
| `n_modified_mohr` | safety factor, modified Mohr |

- **Method:** BCM: 1/n = s1/Sut - s3/Suc (with s1 = max(sA, sB, 0), s3 = min(sA, sB, 0)); MM: 1/n = (Suc - Sut) sA/(Suc Sut) - sB/Suc when $sA > 0 > sB$ and |sB| > sA; MNS: n = min(Sut/s1, Suc/|s3|)
- **Assumes:** plane stress (third principal stress zero); Sut, Suc > 0 (Suc given as a magnitude); proportional loading
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('brittle_failure_mohr', sA=90000000.0, sB=-50000000.0, Sut=210000000.0, Suc=750000000.0)
# -> n_max_normal = 2.33333, n_coulomb_mohr = 2.01923, n_modified_mohr = 2.33333   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('brittle_failure_mohr', sA=50000000.0, sB=-300000000.0, Sut=210000000.0, Suc=750000000.0)
# -> n_max_normal = 2.5, n_coulomb_mohr = 1.56716, n_modified_mohr = 1.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Brittle failure: sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = 750 MPa | SOLVED | n_max_normal = 2.33333, n_coulomb_mohr = 2.01923, n_modified_mohr = 2.33333 |
| Brittle failure: sA = 90 MPa, sB = -50 MPa, Sut = 210 MPa, Suc = -750 MPa | INVALID_INPUT | Safety factors of a brittle material (maximum normal stress, Coulomb-Mohr, modified Mohr): Sut and Suc must be a positiv |

<a id="strain_rosette"></a>

### Strain rosette: strains, principal strains and principal stresses — `strain_rosette`

From the readings of a three-gauge rosette (rectangular $0/45/90$ or delta $0/60/120$ degrees): the strain components, the principal strains and their direction, and the principal stresses (plane stress).

$$
\varepsilon_{1,2} = \frac{\varepsilon_x+\varepsilon_y}{2} \pm \sqrt{\Big(\frac{\varepsilon_x-\varepsilon_y}{2}\Big)^2 + \Big(\frac{\gamma_{xy}}{2}\Big)^2},\quad \sigma_1 = \frac{E(\varepsilon_1+\nu\varepsilon_2)}{1-\nu^2}
$$

| input | meaning | condition |
|---|---|---|
| `ea` | strain of gauge a (at 0 degrees), microstrain |  |
| `eb` | strain of gauge b (45 or 60 degrees), microstrain |  |
| `ec` | strain of gauge c (90 or 120 degrees), microstrain |  |
| `E` | Young's modulus |  |
| `nu` | Poisson's ratio |  |
| `layout` | rosette layout: rectangular $(0/45/90)$ or delta $(0/60/120)$ |  |

| output | meaning |
|---|---|
| `ex` | strain eps_x, microstrain |
| `ey` | strain eps_y, microstrain |
| `gxy` | shear strain gamma_xy, microstrain |
| `e1` | larger principal strain, microstrain |
| `e2` | smaller principal strain, microstrain |
| `theta_p_deg` | angle from gauge a to the eps_1 direction, degrees in (-90, 90] |
| `s1_MPa` | larger principal stress, MPa |
| `s2_MPa` | smaller principal stress, MPa |

- **Method:** rectangular: $ex = ea$, $ey = ec$, gxy = 2eb - ea - ec; delta: $ex = ea$, $ey = (2eb + 2ec - ea)/3$, gxy = 2(eb - ec)/sqrt(3); e1,$2 = (ex+ey)/2$ +- sqrt(((ex-ey)/2)^2 + (gxy/2)^2); $s_{1} = E$($e_{1} + \nu$ e2)/(1 - nu^2)
- **Assumes:** gauge angles measured counterclockwise from gauge a; plane stress, linear elastic isotropic; $0  \leq  \nu < 0.5$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('strain_rosette', ea=400.0, eb=300.0, ec=-100.0, E=200000000000.0, nu=0.3, layout='rectangular')
# -> ex = 400, ey = -100, gxy = 300, e1 = 441.548, e2 = -141.548, theta_p_deg = 15.4819, s1_MPa = 87.7106, s2_MPa = -1.99633   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Strain rosette: ea = 400, eb = 300, ec = -100, E = 200 GPa, nu = 0.3, layout = rectangular | SOLVED | ex = 400, ey = -100, gxy = 300, e1 = 441.548, e2 = -141.548, theta_p_deg = 15.4819, s1_MPa = 87.7106, s2_MPa = -1.99633 |
| Strain rosette: ea = 400, eb = 300, ec = -100, E = 200 GPa, nu = 0.3, layout = star | INVALID_INPUT | layout: one of rectangular, delta; got 'star'. |

<a id="generalized_hookes_law"></a>

### Generalized Hooke's law: strains from a 3D stress state — `generalized_hookes_law`

The six strain components and the volumetric strain of an isotropic linear-elastic material under a general three-dimensional stress state.

$$
\varepsilon_x = \frac{1}{E}\big[\sigma_x - \nu(\sigma_y+\sigma_z)\big],\quad \gamma_{xy} = \frac{\tau_{xy}}{G},\quad G = \frac{E}{2(1+\nu)}
$$

| input | meaning | condition |
|---|---|---|
| `sx` | sigma_x |  |
| `sy` | sigma_y |  |
| `sz` | sigma_z |  |
| `txy` | tau_xy |  |
| `tyz` | tau_yz |  |
| `tzx` | tau_zx |  |
| `E` | Young's modulus |  |
| `nu` | Poisson's ratio |  |

| output | meaning |
|---|---|
| `ex` | eps_x, microstrain |
| `ey` | eps_y, microstrain |
| `ez` | eps_z, microstrain |
| `gxy` | gamma_xy, microstrain |
| `gyz` | gamma_yz, microstrain |
| `gzx` | gamma_zx, microstrain |
| `volumetric` | volumetric strain eps_x + eps_y + eps_z, microstrain |

- **Method:** eps_x = (sx - nu(sy + sz))/E (and cyclic), $\gamma = \tau/G$ with G = E/(2(1 + nu))
- **Assumes:** isotropic, linear elastic; $-1 < \nu < 0.5$; small strains
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('generalized_hookes_law', sx=100000000.0, sy=50000000.0, sz=0.0, txy=30000000.0, tyz=0.0, tzx=0.0, E=200000000000.0, nu=0.3)
# -> ex = 425, ey = 100, ez = -225, gxy = 390, gyz = 0, gzx = 0, volumetric = 300   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Generalized Hooke's law: sx = 100 MPa, sy = 50 MPa, sz = 0 MPa, txy = 30 MPa, tyz = 0 MPa, tzx = 0 MPa, E = 200 GPa, nu = 0.3 | SOLVED | ex = 425, ey = 100, ez = -225, gxy = 390, gyz = 0, gzx = 0, volumetric = 300 |
| Generalized Hooke's law: sx = 100 MPa, sy = 50 MPa, sz = 0 MPa, txy = 30 MPa, tyz = 0 MPa, tzx = 0 MPa, E = 200 GPa, nu = 0.6 | INVALID_INPUT | Generalized Hooke's law: strains from a 3D stress state: Poisson's ratio must lie between -1 and 0.5. |

<a id="thin_walled_pressure_vessel"></a>

### Thin-walled pressure vessel stresses (cylinder or sphere) — `thin_walled_pressure_vessel`

Hoop and longitudinal stresses and the in-plane and absolute maximum shear stresses in the wall of a thin cylindrical or spherical pressure vessel.

$$
\sigma_h = \frac{pr}{t},\quad \sigma_l = \frac{pr}{2t}
$$

| input | meaning | condition |
|---|---|---|
| `p` | internal gauge pressure |  |
| `r` | inner radius |  |
| `t` | wall thickness |  |
| `shape` | vessel shape |  |

| output | meaning |
|---|---|
| `hoop_MPa` | hoop (circumferential) stress, MPa |
| `longitudinal_MPa` | longitudinal (axial) stress, MPa |
| `tau_inplane_MPa` | maximum in-plane shear stress, MPa |
| `tau_abs_MPa` | absolute maximum shear stress at the outer surface, MPa |

- **Method:** cylinder: $sigma_h = pr/t$, $sigma_l = pr/(2t)$; sphere: $\sigma = pr/(2t)$; $tau_abs = sigma_h/2$ (radial stress 0 at the outer surface)
- **Assumes:** $r/t  \geq  10$ (thin wall); closed ends, internal pressure only; r is the inner radius
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('thin_walled_pressure_vessel', p=2000000.0, r=0.5, t=0.01, shape='cylinder')
# -> hoop_MPa = 100, longitudinal_MPa = 50, tau_inplane_MPa = 25, tau_abs_MPa = 50   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('thin_walled_pressure_vessel', p=2000000.0, r=0.5, t=0.01, shape='sphere')
# -> hoop_MPa = 50, longitudinal_MPa = 50, tau_inplane_MPa = 0, tau_abs_MPa = 25   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Thin-walled pressure vessel: p = 2 MPa, r = 500 mm, t = 10 mm, shape = cylinder | SOLVED | hoop_MPa = 100, longitudinal_MPa = 50, tau_inplane_MPa = 25, tau_abs_MPa = 50 |
| Thin-walled pressure vessel: p = 2 MPa, r = 50 mm, t = 10 mm, shape = cylinder | INVALID_INPUT | Thin-walled pressure vessel stresses (cylinder or sphere): r/t is below 10: the wall is not thin (use the thick-walled c |

<a id="thick_walled_cylinder"></a>

### Thick-walled cylinder under pressure (Lame equations) — `thick_walled_cylinder`

Radial and hoop stresses at a radius, the hoop stresses at both surfaces, the maximum shear at the bore and the closed-end axial stress of a thick cylinder under internal and external pressure.

$$
\sigma_{r,\theta} = \frac{p_ia^2 - p_ob^2}{b^2-a^2} \mp \frac{(p_i-p_o)a^2b^2}{(b^2-a^2)r^2}
$$

| input | meaning | condition |
|---|---|---|
| `pi` | internal pressure |  |
| `po` | external pressure |  |
| `a` | inner radius |  |
| `b` | outer radius |  |
| `r` | radius at which to evaluate (default: the bore, r = a) |  |

| output | meaning |
|---|---|
| `sr_MPa` | radial stress at r, MPa |
| `st_MPa` | hoop stress at r, MPa |
| `st_inner_MPa` | hoop stress at the bore, MPa |
| `st_outer_MPa` | hoop stress at the outer surface, MPa |
| `tau_max_inner_MPa` | maximum shear stress at the bore, MPa |
| `sz_closed_MPa` | axial stress with closed ends, MPa |

- **Method:** $sigma_r = A - B/r^{2}$, $sigma_t = A + B/r^{2}$ with A = (pi $a^{2} - po$ b^2)/(b^2 - a^2), $B = (pi - po)$ $a^{2}$ $b^{2}/(b^{2} - a^{2})$; closed ends $sigma_z = A$
- **Assumes:** $a < b$; pressures >= 0, not both zero; $a  \leq  r  \leq  b$; linear elastic
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('thick_walled_cylinder', pi=100000000.0, po=0.0, a=0.05, b=0.1)
# -> sr_MPa = -100, st_MPa = 166.667, st_inner_MPa = 166.667, st_outer_MPa = 66.6667, tau_max_inner_MPa = 133.333, sz_closed_MPa = 33.3333   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Thick-walled cylinder: pi = 100 MPa, po = 0 MPa, a = 50 mm, b = 100 mm | SOLVED | sr_MPa = -100, st_MPa = 166.667, st_inner_MPa = 166.667, st_outer_MPa = 66.6667, tau_max_inner_MPa = 133.333, sz_closed_MPa = 33.3333 |
| Thick-walled cylinder: pi = 100 MPa, po = 0 MPa, a = 100 mm, b = 50 mm | INVALID_INPUT | Thick-walled cylinder under pressure (Lame equations): the inner radius a must be smaller than the outer radius b. |

<a id="shrink_fit_pressure"></a>

### Shrink (press) fit: interface pressure and hoop stresses — `shrink_fit_pressure`

The contact pressure of an interference fit between a hub and a solid or hollow shaft of the same material, and the hoop stresses it causes at the interface.

$$
p = \frac{E\delta}{2R^3}\,\frac{(r_o^2-R^2)(R^2-r_i^2)}{r_o^2-r_i^2}
$$

| input | meaning | condition |
|---|---|---|
| `delta` | radial interference (shaft radius minus hub bore radius) |  |
| `R` | nominal interface radius |  |
| `ro` | outer radius of the hub |  |
| `E` | Young's modulus (both parts) |  |
| `ri` | bore radius of a hollow shaft (default 0: solid) |  |

| output | meaning |
|---|---|
| `p_MPa` | interface pressure, MPa |
| `hub_hoop_MPa` | hoop stress in the hub at the interface, MPa |
| `shaft_hoop_MPa` | hoop stress in the shaft at the interface, MPa |

- **Method:** $p = E$ $\delta/(2R^{3})$ $(ro^{2} - R^{2})$$(R^{2} - ri^{2})/(ro^{2} - ri^{2})$; hub $sigma_t = p$ $(ro^{2} + R^{2})/(ro^{2} - R^{2})$; shaft $sigma_t = -p$ $(R^{2} + ri^{2})/(R^{2} - ri^{2})$
- **Assumes:** $ri < R < ro$; both parts of the same material; plane stress, elastic, long hub
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('shrink_fit_pressure', delta=2e-05, R=0.025, ro=0.05, E=207000000000.0)
# -> p_MPa = 62.1, hub_hoop_MPa = 103.5, shaft_hoop_MPa = -62.1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shrink fit: delta = 0.02 mm, R = 25 mm, ro = 50 mm, E = 207 GPa | SOLVED | p_MPa = 62.1, hub_hoop_MPa = 103.5, shaft_hoop_MPa = -62.1 |
| Shrink fit: delta = 0.02 mm, R = 60 mm, ro = 50 mm, E = 207 GPa | INVALID_INPUT | Shrink (press) fit: interface pressure and hoop stresses: need ri < R < ro (shaft bore < interface < hub outer radius). |

<a id="rotating_disk_stresses"></a>

### Stresses in a rotating disk (solid or with a central hole) — `rotating_disk_stresses`

The largest hoop and radial stresses, the radius of the largest radial stress and the hoop stress at the rim of a thin disk of uniform thickness spinning at constant speed.

$$
\sigma_\theta(r_i) = \frac{3+\nu}{4}\rho\omega^2\Big(r_o^2 + \frac{1-\nu}{3+\nu}r_i^2\Big),\quad \sigma_{r,max} = \frac{3+\nu}{8}\rho\omega^2(r_o-r_i)^2
$$

| input | meaning | condition |
|---|---|---|
| `rho` | density |  |
| `omega` | angular speed (rpm or rad/s) |  |
| `nu` | Poisson's ratio |  |
| `ro` | outer radius |  |
| `ri` | radius of the central hole (default 0: solid) |  |

| output | meaning |
|---|---|
| `hoop_max_MPa` | largest hoop stress (at the bore, or the centre of a solid disk), MPa |
| `radial_max_MPa` | largest radial stress, MPa |
| `r_radial_max` | radius of the largest radial stress (sqrt(ri ro); 0 for a solid disk) |
| `hoop_rim_MPa` | hoop stress at the outer rim, MPa |

- **Method:** solid: $sigma_r = sigma_t = (3 + \nu)/8$ rho $w^{2}$ $ro^{2}$ at the centre; annular: sigma_t(ri) = (3 + nu)/4 rho $w^{2}$ (ro^2 + (1 - nu)/(3 + nu) ri^2), sigma_r,max = (3 + nu)/8 rho $w^{2}$ $(ro - ri)^{2}$ at sqrt(ri ro)
- **Assumes:** thin disk of uniform thickness, plane stress; free inner and outer edges; $0  \leq  \nu < 0.5$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('rotating_disk_stresses', rho=7800.0, omega=314.1592653589793, nu=0.3, ro=0.25, ri=0.05)
# -> hoop_max_MPa = 40.0311, radial_max_MPa = 12.7022, r_radial_max = 0.111803 m, hoop_rim_MPa = 10.0078   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('rotating_disk_stresses', rho=7800.0, omega=314.1592653589793, nu=0.3, ro=0.25)
# -> hoop_max_MPa = 19.8472, radial_max_MPa = 19.8472, r_radial_max = 0 m, hoop_rim_MPa = 8.42001   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rotating disk: rho = 7800 kg/m^3, omega = 3000 rpm, nu = 0.3, ro = 250 mm, ri = 50 mm | SOLVED | hoop_max_MPa = 40.0311, radial_max_MPa = 12.7022, r_radial_max = 0.111803 m, hoop_rim_MPa = 10.0078 |
| Rotating disk: rho = 7800 kg/m^3, omega = 3000 rpm, nu = 0.3, ro = 250 mm, ri = 300 mm | INVALID_INPUT | Stresses in a rotating disk (solid or with a central hole): the bore radius must be smaller than the outer radius. |

<a id="euler_buckling"></a>

### Euler buckling load of a column with given end conditions — `euler_buckling`

The critical (Euler) buckling load of an ideal elastic column, its effective length factor and length, and with the area also the critical stress and slenderness ratio.

$$
P_{cr} = \frac{\pi^2EI}{(KL)^2}
$$

| input | meaning | condition |
|---|---|---|
| `E` | Young's modulus |  |
| `I` | least second moment of area |  |
| `L` | column length |  |
| `ends` | end conditions: pinned-pinned, fixed-free, fixed-fixed or fixed-pinned |  |
| `A` | cross-sectional area (optional: gives the stress and slenderness) |  |

| output | meaning |
|---|---|
| `P_cr` | critical buckling load |
| `P_cr_kN` | critical load in kN |
| `K` | effective length factor (theoretical) |
| `Le` | effective length K L |
| `sigma_cr_MPa` | critical stress P_cr/A, MPa (with A) |
| `slenderness` | slenderness ratio K L / r (with A) |

- **Method:** $P_cr = pi^{2}$ E I/(K L)^2
- **Assumes:** ideal straight elastic column, centric load; theoretical K: 1, 2, 0.5, 0.6992 (fixed-pinned, from tan x = x); valid only below the proportional limit
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('euler_buckling', E=200000000000.0, I=8e-06, L=4.0, ends='pinned-pinned')
# -> P_cr = 986960 N, P_cr_kN = 986.96, K = 1, Le = 4 m   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('euler_buckling', E=200000000000.0, I=8e-06, L=4.0, ends='fixed-pinned', A=0.005)
# -> P_cr = 2.01907e+06 N, P_cr_kN = 2019.07, K = 0.699156, Le = 2.79662 m, sigma_cr_MPa = 403.815, slenderness = 69.9156   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = pinned-pinned | SOLVED | P_cr = 986960 N, P_cr_kN = 986.96, K = 1, Le = 4 m |
| Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = fixed-fixed, A = 5000 mm^2 | SOLVED | P_cr = 3.94784e+06 N, P_cr_kN = 3947.84, K = 0.5, Le = 2 m, sigma_cr_MPa = 789.568, slenderness = 50 |
| Euler buckling: E = 200 GPa, I = 8e-6 m^4, L = 4 m, ends = hinged | INVALID_INPUT | ends: one of pinned-pinned, fixed-free, fixed-fixed, fixed-pinned; got 'hinged'. |

<a id="johnson_column"></a>

### Critical stress of an intermediate column (Johnson parabola / Euler) — `johnson_column`

The critical stress of a column by the Johnson parabola below the transition slenderness and by Euler above it, with the transition slenderness and (with the area) the critical load.

$$
\sigma_{cr} = S_y - \frac{1}{E}\Big(\frac{S_y}{2\pi}\frac{KL}{r}\Big)^2 \quad (KL/r < C_c),\quad C_c = \sqrt{\frac{2\pi^2E}{S_y}}
$$

| input | meaning | condition |
|---|---|---|
| `Sy` | yield strength |  |
| `E` | Young's modulus |  |
| `L` | column length |  |
| `r` | least radius of gyration |  |
| `ends` | end conditions (theoretical K) |  |
| `A` | cross-sectional area (optional: gives the load) |  |

| output | meaning |
|---|---|
| `slenderness` | slenderness ratio K L / r |
| `transition_slenderness` | transition slenderness $C_c = sqrt$(2 $pi^{2}$ E / Sy) |
| `regime` | Johnson or Euler |
| `sigma_cr_MPa` | critical stress, MPa |
| `P_cr_kN` | critical load in kN (with A) |

- **Method:** $C_c = sqrt$(2 $pi^{2}$ E/Sy); $KL/r < C_c$: sigma_cr = Sy - (Sy KL/(2 pi r))^$2/E$; otherwise $sigma_cr = pi^{2}$ $E/(KL/r)^{2}$
- **Assumes:** centric load; theoretical effective length factors
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('johnson_column', Sy=250000000.0, E=200000000000.0, L=1.5, r=0.02, ends='pinned-pinned')
# -> slenderness = 75, transition_slenderness = 125.664, regime = Johnson, sigma_cr_MPa = 205.474   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('johnson_column', Sy=250000000.0, E=200000000000.0, L=4.0, r=0.02, ends='pinned-pinned')
# -> slenderness = 200, transition_slenderness = 125.664, regime = Euler, sigma_cr_MPa = 49.348   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Johnson column: Sy = 250 MPa, E = 200 GPa, L = 1.5 m, r = 20 mm, ends = pinned-pinned | SOLVED | slenderness = 75, transition_slenderness = 125.664, regime = Johnson, sigma_cr_MPa = 205.474 |
| Johnson column: Sy = 250 MPa, E = 200 GPa, L = 1.5 m, ends = pinned-pinned | NEEDS_INPUT | johnson_column needs r (least radius of gyration). Example: solve('johnson_column', Sy='2.5e+08 Pa', E='2e+11 Pa', L='1. |

<a id="secant_column"></a>

### Eccentrically loaded column (secant formula) — `secant_column`

The maximum compressive stress, midspan deflection and largest bending moment of a pinned column under a load with equal end eccentricities.

$$
\sigma_{max} = \frac{P}{A}\Big[1 + \frac{ec}{r^2}\sec\Big(\frac{L}{2}\sqrt{\frac{P}{EI}}\Big)\Big]
$$

| input | meaning | condition |
|---|---|---|
| `P` | compressive load |  |
| `e` | eccentricity of the load |  |
| `L` | length (pinned ends) |  |
| `A` | area |  |
| `I` | second moment of area in the bending plane |  |
| `c` | distance from the centroid to the extreme fibre |  |
| `E` | Young's modulus |  |

| output | meaning |
|---|---|
| `sigma_max_MPa` | maximum compressive stress (magnitude), MPa |
| `deflection_mm` | midspan lateral deflection, mm |
| `M_max` | largest bending moment P (e + deflection) |
| `P_over_Pcr` | P / P_Euler |

- **Method:** sigma_max = P/A [1 + (e c/r^2) sec($L/2$ sqrt(P/(EI)))], deflection = e [sec(kL/2) - 1]
- **Assumes:** pinned ends, equal eccentricities on the same side; P below the Euler load; elastic
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('secant_column', P=200000.0, e=0.01, L=3.0, A=0.004, I=1.2e-05, c=0.075, E=200000000000.0)
# -> sigma_max_MPa = 63.771, deflection_mm = 1.01679, M_max = 2203.36 N*m, P_over_Pcr = 0.0759909   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Secant column: P = 200 kN, e = 10 mm, L = 3 m, A = 4000 mm^2, I = 1.2e-5 m^4, c = 75 mm, E = 200 GPa | SOLVED | sigma_max_MPa = 63.771, deflection_mm = 1.01679, M_max = 2203.36 N*m, P_over_Pcr = 0.0759909 |
| Secant column: P = 5000 kN, e = 10 mm, L = 3 m, A = 4000 mm^2, I = 1.2e-5 m^4, c = 75 mm, E = 200 GPa | INVALID_INPUT | Eccentrically loaded column (secant formula): the load reaches the Euler load of the pinned column: no equilibrium. |

<a id="combined_axial_and_bending_stress"></a>

### Combined axial force and biaxial bending of a rectangular section — `combined_axial_and_bending_stress`

Normal stresses at the four corners of a rectangular section carrying an axial force and bending moments about both axes, with the extreme values.

$$
\sigma = \frac{N}{A} + \frac{M_xy}{I_x} + \frac{M_yx}{I_y}
$$

| input | meaning | condition |
|---|---|---|
| `b` | width (along x) |  |
| `h` | depth (along y) |  |
| `N` | axial force (tension positive) |  |
| `Mx` | moment about x (positive: tension at +y) |  |
| `My` | moment about y (positive: tension at +x) |  |

| output | meaning |
|---|---|
| `sigma_corners_MPa` | stresses at the corners (+x,+y), (-x,+y), (-x,-y), (+x,-y), MPa |
| `sigma_max_MPa` | largest (most tensile) stress, MPa |
| `sigma_min_MPa` | smallest (most compressive) stress, MPa |
| `sigma_axial_MPa` | uniform axial part N/A, MPa |

- **Method:** sigma(x, y) = N/A + Mx y/Ix + My x/Iy at the corners (x = +$-b/2$, y = +-h/2)
- **Assumes:** linear elastic, plane sections; moments about the centroidal axes
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('combined_axial_and_bending_stress', b=0.1, h=0.2, N=-50000.0, Mx=8000.0, My=2000.0)
# -> sigma_corners_MPa = [15.5, 3.5, -20.5, -8.5], sigma_max_MPa = 15.5, sigma_min_MPa = -20.5, sigma_axial_MPa = -2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Combined axial and bending stress: b = 100 mm, h = 200 mm, N = -50 kN, Mx = 8 kN*m, My = 2 kN*m | SOLVED | sigma_corners_MPa = [15.5, 3.5, -20.5, -8.5], sigma_max_MPa = 15.5, sigma_min_MPa = -20.5, sigma_axial_MPa = -2.5 |
| Combined axial and bending stress: b = 100 mm, h = 200 mm, N = -50 kN, Mx = 8 kN*m | NEEDS_INPUT | combined_axial_and_bending_stress needs My (moment about y (positive: tension at +x)). Example: solve('combined_axial_an |

<a id="eccentric_bolt_group"></a>

### Eccentrically loaded bolt group (elastic method) — `eccentric_bolt_group`

The direct shear per bolt, the largest resultant bolt force and which bolt carries it, for a group of identical bolts under a vertical load applied with a horizontal eccentricity.

$$
\mathbf F_i = -\frac{P}{n}\hat{\mathbf y} + \frac{Pe}{\sum r_j^2}\,(y_i-\bar y,\; -(x_i-\bar x))
$$

| input | meaning | condition |
|---|---|---|
| `bolts` | bolt positions, one row 'x y' per bolt, in mm |  |
| `P` | vertical load (downward) |  |
| `e` | horizontal distance of the load line from the bolt-group centroid (positive to the right) |  |

| output | meaning |
|---|---|
| `direct_kN` | direct shear per bolt, kN |
| `max_bolt_force_kN` | largest resultant bolt force, kN |
| `critical_bolt` | number of the most loaded bolt (1-based, the first if several are equal) |
| `polar_sum_mm2` | sum of $r^{2}$ about the centroid, $mm^{2}$ |

- **Method:** direct F/n plus torsional M r_i / sum $r^{2}$ perpendicular to r_i, with $M = P$ e about the bolt centroid; vector sum per bolt
- **Assumes:** identical bolts, rigid plate, elastic method; 2 to 40 bolts, not all at one point
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('eccentric_bolt_group', bolts='0 0; 0 100; 100 0; 100 100', P=20000.0, e=0.25)
# -> direct_kN = 5, max_bolt_force_kN = 21.5058, critical_bolt = 3, polar_sum_mm2 = 20000   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bolt group: bolts = [0 0; 0 100; 100 0; 100 100], P = 20 kN, e = 250 mm | SOLVED | direct_kN = 5, max_bolt_force_kN = 21.5058, critical_bolt = 3, polar_sum_mm2 = 20000 |
| Bolt group: bolts = [0 0; 0 0], P = 20 kN, e = 250 mm | INVALID_INPUT | Eccentrically loaded bolt group (elastic method): the bolts must not all coincide. |

<a id="goodman_fatigue_safety_factor"></a>

### Fatigue safety factor by the modified Goodman criterion — `goodman_fatigue_safety_factor`

The fatigue factor of safety along a proportional load line by the modified Goodman line, the equivalent fully reversed amplitude, and (with Sy) Langer's first-cycle yield factor.

$$
\frac{\sigma_a}{S_e} + \frac{\sigma_m}{S_{ut}} = \frac1n
$$

| input | meaning | condition |
|---|---|---|
| `sa` | alternating stress amplitude |  |
| `sm` | mean stress (>= 0) |  |
| `Se` | fully corrected endurance limit |  |
| `Sut` | ultimate tensile strength |  |
| `Sy` | yield strength (optional: first-cycle yield check) |  |

| output | meaning |
|---|---|
| `n_fatigue` | fatigue safety factor (Goodman, proportional loading) |
| `sigma_ar_MPa` | equivalent fully reversed amplitude sa/(1 - sm/Sut), MPa |
| `n_yield` | Langer first-cycle yield factor Sy/(sa + sm) (with Sy) |

- **Method:** 1/n = sa/Se + sm/Sut; sigma_ar = sa/(1 - sm/Sut); $n_y = Sy/(sa + sm)$
- **Assumes:** $sm  \geq  0$ (compressive means not covered); sm < Sut; proportional loading (the load line through the origin)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('goodman_fatigue_safety_factor', sa=120000000.0, sm=80000000.0, Se=250000000.0, Sut=700000000.0, Sy=500000000.0)
# -> n_fatigue = 1.68269, sigma_ar_MPa = 135.484, n_yield = 2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Goodman safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa | SOLVED | n_fatigue = 1.68269, sigma_ar_MPa = 135.484 |
| Goodman safety factor: sa = 120 MPa, sm = -80 MPa, Se = 250 MPa, Sut = 700 MPa | INVALID_INPUT | Fatigue safety factor by the modified Goodman criterion: the mean stress (compressive means are not covered) must be zer |

<a id="soderberg_fatigue_safety_factor"></a>

### Fatigue safety factor by the Soderberg criterion — `soderberg_fatigue_safety_factor`

The conservative Soderberg fatigue factor of safety (the line from Se to the yield strength) along a proportional load line, and the equivalent amplitude.

$$
\frac{\sigma_a}{S_e} + \frac{\sigma_m}{S_y} = \frac1n
$$

| input | meaning | condition |
|---|---|---|
| `sa` | alternating stress amplitude |  |
| `sm` | mean stress (>= 0) |  |
| `Se` | fully corrected endurance limit |  |
| `Sy` | yield strength |  |

| output | meaning |
|---|---|
| `n_fatigue` | fatigue safety factor (Soderberg) |
| `sigma_ar_MPa` | equivalent fully reversed amplitude $sa/(1 - sm/Sy)$, MPa |

- **Method:** $1/n = sa/Se + sm/Sy$
- **Assumes:** $sm  \geq  0$; $sm < Sy$; proportional loading
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('soderberg_fatigue_safety_factor', sa=100000000.0, sm=60000000.0, Se=220000000.0, Sy=450000000.0)
# -> n_fatigue = 1.70103, sigma_ar_MPa = 115.385   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Soderberg safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, Sy = 450 MPa | SOLVED | n_fatigue = 1.70103, sigma_ar_MPa = 115.385 |
| Soderberg safety factor: sa = 100 MPa, sm = 500 MPa, Se = 220 MPa, Sy = 450 MPa | INVALID_INPUT | Fatigue safety factor by the Soderberg criterion: the mean stress must be below the strength. |

<a id="fatigue_life_mean_stress_correction"></a>

### Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law — `fatigue_life_mean_stress_correction`

The equivalent fully reversed stress amplitude of a cycle with a mean stress, by a chosen correction, and the cycles to failure from Basquin's law $sigma_ar = sf$' $(2N)^{b.}$

$$
\sigma_{ar} = \frac{\sigma_a}{1-\sigma_m/\sigma_f'},\quad \sigma_{ar} = \sqrt{\sigma_{max}\sigma_a},\quad N_f = \tfrac12\Big(\frac{\sigma_{ar}}{\sigma_f'}\Big)^{1/b}
$$

| input | meaning | condition |
|---|---|---|
| `sa` | stress amplitude |  |
| `sm` | mean stress |  |
| `sf` | fatigue strength coefficient sf' (Morrow and Goodman use it as the strength) |  |
| `b` | fatigue strength exponent (negative) |  |
| `method` | mean-stress correction |  |
| `gamma` | Walker exponent (walker only) |  |

| output | meaning |
|---|---|
| `sigma_ar_MPa` | equivalent fully reversed amplitude, MPa |
| `Nf` | cycles to failure |
| `log10_Nf` | log10 of the cycles to failure |

- **Method:** Morrow: sigma_ar = sa/(1 - sm/sf'); SWT: $sigma_ar = sqrt$(sigma_max sa); Walker: sigma_max^(1-gamma) $sa^{\gamma}$; Nf = (sigma_ar/sf')^$(1/b)/2$
- **Assumes:** $-0.2 < b < 0$; Morrow/Goodman: $sm < sf$'; SWT/Walker: sigma_max > 0; goodman here uses sf' as the strength (Morrow form)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('fatigue_life_mean_stress_correction', sa=300000000.0, sm=100000000.0, sf=1200000000.0, b=-0.09, method='swt')
# -> sigma_ar_MPa = 346.41, Nf = 494791, log10_Nf = 5.69442   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('fatigue_life_mean_stress_correction', sa=300000000.0, sm=100000000.0, sf=1200000000.0, b=-0.09, method='morrow')
# -> sigma_ar_MPa = 327.273, Nf = 930363, log10_Nf = 5.96865   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fatigue life with mean stress: sa = 300 MPa, sm = 100 MPa, sf = 1200 MPa, b = -0.09, method = swt | SOLVED | sigma_ar_MPa = 346.41, Nf = 494791, log10_Nf = 5.69442 |
| Fatigue life with mean stress: sa = 300 MPa, sm = 100 MPa, sf = 1200 MPa, b = -0.09, method = walker | INVALID_INPUT | Fatigue life with a mean-stress correction (Morrow, Goodman, SWT, Walker) and Basquin's law: the Walker exponent gamma m |

<a id="sn_fatigue_life"></a>

### Fatigue life from the S-N line (10^3 to 10^6 cycles) — `sn_fatigue_life`

Cycles to failure of a fully reversed amplitude on the straight log-log S-N line through f*Sut at $10^{3}$ cycles and Se at $10^{6}$ cycles, with the line's constants a and b.

$$
a = \frac{(fS_{ut})^2}{S_e},\quad b = -\frac13\log_{10}\frac{fS_{ut}}{S_e},\quad N = \Big(\frac{\sigma_a}{a}\Big)^{1/b}
$$

| input | meaning | condition |
|---|---|---|
| `Sut` | ultimate tensile strength |  |
| `Se` | endurance limit |  |
| `f` | fatigue strength fraction at $10^{3}$ cycles (e.g. 0.9) |  |
| `sa` | fully reversed stress amplitude |  |

| output | meaning |
|---|---|
| `a_MPa` | S-N constant a, MPa |
| `b` | S-N exponent b |
| `N` | cycles to failure |
| `log10_N` | log10 of the cycles to failure |

- **Method:** a = (f Sut)^2/Se, $b = -log10$(f Sut/Se)/3, $N = (sa/a)^{1/b}$
- **Assumes:** $Se < sa < f$ Sut; $0 < f < 1$; fully reversed loading
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('sn_fatigue_life', Sut=700000000.0, Se=300000000.0, f=0.85, sa=400000000.0)
# -> a_MPa = 1180.08, b = -0.0991319, N = 54912.6, log10_N = 4.73967   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| S-N fatigue life: Sut = 700 MPa, Se = 300 MPa, f = 0.85, sa = 400 MPa | SOLVED | a_MPa = 1180.08, b = -0.0991319, N = 54912.6, log10_N = 4.73967 |
| S-N fatigue life: Sut = 700 MPa, Se = 300 MPa, f = 0.85, sa = 250 MPa | INVALID_INPUT | Fatigue life from the S-N line (10^3 to 10^6 cycles): the amplitude must lie between Se (infinite life) and f*Sut (low-c |

<a id="sn_fatigue_strength"></a>

### Fatigue strength at a given number of cycles (S-N line) — `sn_fatigue_strength`

The fully reversed fatigue strength at N cycles (10^3 to 10^6) on the straight log-log S-N line through f*Sut at $10^{3}$ and Se at $10^{6}$ cycles.

$$
S_f = aN^b
$$

| input | meaning | condition |
|---|---|---|
| `Sut` | ultimate tensile strength |  |
| `Se` | endurance limit |  |
| `f` | fatigue strength fraction at $10^{3}$ cycles |  |
| `N` | number of cycles (10^3 to 10^6) |  |

| output | meaning |
|---|---|
| `Sf_MPa` | fatigue strength at N cycles, MPa |

- **Method:** $Sf = a$ $N^{b}$ with a = (f Sut)^2/Se, $b = -log10$(f Sut/Se)/3
- **Assumes:** $10^{3}  \leq  N  \leq  10^{6}$; $0 < f < 1$, f Sut > Se
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('sn_fatigue_strength', Sut=700000000.0, Se=300000000.0, f=0.85, N=1000.0)
# -> Sf_MPa = 595   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('sn_fatigue_strength', Sut=700000000.0, Se=300000000.0, f=0.85, N=1000000.0)
# -> Sf_MPa = 300   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| S-N fatigue strength: Sut = 700 MPa, Se = 300 MPa, f = 0.85, N = 1000000 | SOLVED | Sf_MPa = 300 |
| S-N fatigue strength: Sut = 700 MPa, Se = 300 MPa, f = 0.85, N = 100 | INVALID_INPUT | Fatigue strength at a given number of cycles (S-N line): N must lie between 10^3 and 10^6 cycles. |

<a id="miner_damage_sn_curve"></a>

### Miner's rule damage with a single-slope S-N curve — `miner_damage_sn_curve`

The Palmgren-Miner damage sum of stress-range blocks on an S-N curve $N = Nc$ $(Sc/S)^{m}$, and how many repeats of the spectrum the part survives.

$$
D = \sum_i\frac{n_i}{N_i},\quad N_i = N_c\Big(\frac{S_c}{S_i}\Big)^m
$$

| input | meaning | condition |
|---|---|---|
| `ranges` | stress ranges in MPa (a list) |  |
| `counts` | number of cycles of each range (a list) |  |
| `Sc` | reference stress range of the curve at Nc cycles, MPa |  |
| `m` | S-N slope (e.g. 3 for welded details) |  |
| `Nc` | cycles at Sc (default 2e6) |  |

| output | meaning |
|---|---|
| `damage` | Miner damage sum D |
| `repeats_to_failure` | repeats of the spectrum until $D = 1$ |

- **Method:** D = sum n_i / N_i, $N_i = Nc$ $(Sc/S_i)^{m}$
- **Assumes:** linear damage accumulation (Palmgren-Miner); a single-slope curve without endurance limit
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('miner_damage_sn_curve', ranges='90', counts='1000000', Sc=90.0, m=3.0)
# -> damage = 0.5, repeats_to_failure = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Miner's rule: ranges = [90], counts = [1000000], Sc = 90, m = 3 | SOLVED | damage = 0.5, repeats_to_failure = 2 |
| Miner's rule: ranges = [120, 80], counts = [20000], Sc = 90, m = 3 | INVALID_INPUT | Miner's rule damage with a single-slope S-N curve: ranges and counts must have the same number of entries. |

<a id="eurocode_fatigue_damage"></a>

### Fatigue damage on the trilinear (Eurocode 3) S-N curve — `eurocode_fatigue_damage`

The Miner damage of stress-range blocks on the trilinear S-N curve of a detail category (slope 3 to 5e6 cycles, slope 5 to 1e8, cut-off below), with the constant-amplitude and cut-off limits.

$$
N = 2\cdot10^6\Big(\frac{\Delta\sigma_C}{\Delta\sigma}\Big)^3\ (\Delta\sigma\ge\Delta\sigma_D),\quad N = 5\cdot10^6\Big(\frac{\Delta\sigma_D}{\Delta\sigma}\Big)^5
$$

| input | meaning | condition |
|---|---|---|
| `ranges` | stress ranges in MPa (a list) |  |
| `counts` | cycles of each range (a list) |  |
| `dsc` | detail category: stress range at 2e6 cycles, MPa |  |

| output | meaning |
|---|---|
| `damage` | Miner damage sum |
| `delta_sigma_D` | constant-amplitude fatigue limit (5e6 cycles), MPa |
| `delta_sigma_L` | cut-off limit (1e8 cycles), MPa |
| `repeats_to_failure` | repeats of the spectrum until $D = 1$ |

- **Method:** $N = 2e6$ (dsc/S)^3 for $S  \geq  sD = 0.737$ dsc; $N = 5e6$ $(sD/S)^{5}$ for $sL  \leq  S < sD$ with $sL = 0.549$ sD; no damage below sL
- **Assumes:** partial safety factors not applied $(gamma_Mf = gamma_Ff = 1)$; at least one range above the cut-off
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('eurocode_fatigue_damage', ranges='90', counts='1000000', dsc=90.0)
# -> damage = 0.5, delta_sigma_D = 66.3126, delta_sigma_L = 36.4242, repeats_to_failure = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Eurocode fatigue damage: ranges = [90], counts = [1000000], dsc = 90 | SOLVED | damage = 0.5, delta_sigma_D = 66.3126, delta_sigma_L = 36.4242, repeats_to_failure = 2 |
| Eurocode fatigue damage: ranges = [10], counts = [1000000], dsc = 90 | INVALID_INPUT | Fatigue damage on the trilinear (Eurocode 3) S-N curve: every range is below the cut-off limit: no damage. |

<a id="rainflow_counting"></a>

### Rainflow cycle counting of a load history (ASTM E1049) — `rainflow_counting`

The cycles in a load or stress history by rainflow counting (three-point method, half cycles for the residue), as distinct ranges with their counts.

$$
|x_3-x_2| \ge |x_2-x_1| \Rightarrow \text{count } |x_2-x_1|
$$

| input | meaning | condition |
|---|---|---|
| `history` | the load or stress history, point by point (a list; any one unit) |  |

| output | meaning |
|---|---|
| `ranges` | distinct ranges, largest first (in the history's unit) |
| `counts` | cycles of each range (half cycles count 0.5) |
| `total_cycles` | total number of cycles |
| `max_range` | largest range |

- **Method:** reversals (peaks and valleys), then the three-point rule: when a range Y is not larger than the next range X, count Y (a half cycle if it contains the start) and remove its points
- **Assumes:** at least 3 points, not constant; ASTM $E1049-85$: the first and last points are reversals, the residue counts as half cycles; ranges equal to 12 significant digits are merged
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('rainflow_counting', history='0, 5, -3, 4, -2, 6, -4, 1, 0')
# -> ranges = [10.0, 9.0, 8.0, 6.0, 5.0, 1.0], counts = [0.5, 0.5, 0.5, 1.0, 1.0, 0.5], total_cycles = 4, max_range = 10   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rainflow counting: history = [0, 5, -3, 4, -2, 6, -4, 1, 0] | SOLVED | ranges = [10.0, 9.0, 8.0, 6.0, 5.0, 1.0], counts = [0.5, 0.5, 0.5, 1.0, 1.0, 0.5], total_cycles = 4, max_range = 10 |
| Rainflow counting: history = [2, 2, 2] | INVALID_INPUT | Rainflow cycle counting of a load history (ASTM E1049): the history is constant: there are no cycles. |

<a id="rainflow_fatigue_damage"></a>

### Fatigue damage of a stress history (rainflow counting + Miner's rule) — `rainflow_fatigue_damage`

The Miner damage of one pass of a stress history: rainflow cycles (ASTM, half cycles) on a single-slope S-N curve $N = Nc$ $(Sc/S)^{m}$, and the passes to failure.

$$
D = \sum_i \frac{n_i}{N_c}\Big(\frac{S_i}{S_c}\Big)^m
$$

| input | meaning | condition |
|---|---|---|
| `history` | the stress history in MPa, point by point (a list) |  |
| `Sc` | reference stress range at Nc cycles, MPa |  |
| `m` | S-N slope |  |
| `Nc` | cycles at Sc (default 2e6) |  |

| output | meaning |
|---|---|
| `damage` | damage of one pass of the history |
| `repeats_to_failure` | passes until $D = 1$ |
| `cycles` | cycles counted per pass |

- **Method:** rainflow ranges S_i with counts n_i; D = sum n_i S_i^m/(Nc Sc^m)
- **Assumes:** ASTM $E1049-85$ rainflow counting, half cycles weighted 0.5; no mean-stress correction; single-slope S-N curve, no endurance limit
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('rainflow_fatigue_damage', history='0, 90, 0', Sc=90.0, m=3.0)
# -> damage = 5e-07, repeats_to_failure = 2e+06, cycles = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rainflow fatigue damage: history = [0, 90, 0], Sc = 90, m = 3 | SOLVED | damage = 5e-07, repeats_to_failure = 2e+06, cycles = 1 |
| Rainflow fatigue damage: history = [0, 90], Sc = 90, m = 3 | INVALID_INPUT | history: needs at least 3 entries, got 2. |

<a id="shaft_fatigue_safety_factor"></a>

### Shaft fatigue safety factor (DE-Goodman) — `shaft_fatigue_safety_factor`

The fatigue factor of safety of a rotating solid shaft under alternating and mean bending and torsion by the distortion-energy Goodman criterion, with the von Mises stresses and (with Sy) the first-cycle yield factor.

$$
\frac1n = \frac{16}{\pi d^3}\Big\{\frac{\sqrt{4(K_fM_a)^2+3(K_{fs}T_a)^2}}{S_e} + \frac{\sqrt{4(K_fM_m)^2+3(K_{fs}T_m)^2}}{S_{ut}}\Big\}
$$

| input | meaning | condition |
|---|---|---|
| `d` | shaft diameter |  |
| `Ma` | alternating bending moment |  |
| `Mm` | mean bending moment |  |
| `Ta` | alternating torque |  |
| `Tm` | mean torque |  |
| `Kf` | fatigue stress-concentration factor, bending |  |
| `Kfs` | fatigue stress-concentration factor, torsion |  |
| `Se` | fully corrected endurance limit |  |
| `Sut` | ultimate tensile strength |  |
| `Sy` | yield strength (optional) |  |

| output | meaning |
|---|---|
| `n_fatigue` | fatigue safety factor (DE-Goodman) |
| `sigma_a_vm_MPa` | alternating von Mises stress, MPa |
| `sigma_m_vm_MPa` | mean von Mises stress, MPa |
| `n_yield` | first-cycle yield factor (with Sy) |

- **Method:** sigma_a' = sqrt((32 Kf Ma/(pi d^3))^$2 + 3$(16 Kfs Ta/(pi d^3))^2), likewise sigma_m'; $1/n = sigma_a$'/Se + sigma_m'/Sut
- **Assumes:** solid round shaft, no axial load; Kf, Kfs >= 1; an alternating moment or torque present
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('shaft_fatigue_safety_factor', d=0.04, Ma=150.0, Mm=0.0, Ta=0.0, Tm=200.0, Kf=1.6, Kfs=1.4, Se=200000000.0, Sut=700000000.0)
# -> n_fatigue = 4.06308, sigma_a_vm_MPa = 38.1972, sigma_m_vm_MPa = 38.593   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shaft fatigue safety factor: d = 40 mm, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa | SOLVED | n_fatigue = 4.06308, sigma_a_vm_MPa = 38.1972, sigma_m_vm_MPa = 38.593 |
| Shaft fatigue safety factor: d = 40 mm, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 0.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa | INVALID_INPUT | Shaft fatigue safety factor (DE-Goodman): fatigue stress-concentration factors are >= 1. |

<a id="shaft_diameter_de_goodman"></a>

### Shaft diameter for a design factor (DE-Goodman) — `shaft_diameter_de_goodman`

The smallest solid-shaft diameter giving a required fatigue design factor under alternating and mean bending and torsion (distortion-energy Goodman).

$$
d = \Big(\frac{16n}{\pi}\Big\{\frac{\sqrt{4(K_fM_a)^2+3(K_{fs}T_a)^2}}{S_e} + \frac{\sqrt{4(K_fM_m)^2+3(K_{fs}T_m)^2}}{S_{ut}}\Big\}\Big)^{1/3}
$$

| input | meaning | condition |
|---|---|---|
| `n` | required fatigue design factor (>= 1) |  |
| `Ma` | alternating bending moment |  |
| `Mm` | mean bending moment |  |
| `Ta` | alternating torque |  |
| `Tm` | mean torque |  |
| `Kf` | fatigue stress-concentration factor, bending |  |
| `Kfs` | fatigue stress-concentration factor, torsion |  |
| `Se` | fully corrected endurance limit |  |
| `Sut` | ultimate tensile strength |  |

| output | meaning |
|---|---|
| `d_mm` | required diameter, mm |

- **Method:** d = (16n/pi [sqrt(4(Kf Ma)^2 + 3(Kfs Ta)^2)/Se + sqrt(4(Kf Mm)^2 + 3(Kfs Tm)^2)/Sut])^$(1/3)$
- **Assumes:** solid round shaft; the stress-concentration factors for the final size are assumed known; $n  \geq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('shaft_diameter_de_goodman', n=1.5, Ma=150.0, Mm=0.0, Ta=0.0, Tm=200.0, Kf=1.6, Kfs=1.4, Se=200000000.0, Sut=700000000.0)
# -> d_mm = 28.6949   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shaft diameter DE-Goodman: n = 1.5, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa | SOLVED | d_mm = 28.6949 |
| Shaft diameter DE-Goodman: n = 0.5, Ma = 150 N*m, Mm = 0 N*m, Ta = 0 N*m, Tm = 200 N*m, Kf = 1.6, Kfs = 1.4, Se = 200 MPa, Sut = 700 MPa | INVALID_INPUT | Shaft diameter for a design factor (DE-Goodman): the design factor n must be at least 1. |

<a id="belt_friction_capstan"></a>

### Belt or rope friction on a drum (capstan equation) — `belt_friction_capstan`

The largest tight-side tension a flat belt, rope or V-belt can hold for a given slack-side tension, friction coefficient and wrap angle.

$$
\frac{T_1}{T_2} = e^{\mu\theta}\quad\Big(e^{\mu\theta/\sin(\beta/2)}\ \text{for a V-belt}\Big)
$$

| input | meaning | condition |
|---|---|---|
| `T2` | slack-side tension |  |
| `mu` | coefficient of friction |  |
| `theta` | angle of wrap |  |
| `beta` | included groove angle of a V-belt (optional; flat belt when absent) |  |

| output | meaning |
|---|---|
| `T1` | tight-side tension at impending slip |
| `ratio` | tension ratio $T_{1}/T_{2}$ |
| `friction_force` | net friction force $T_{1} - T_{2}$ |

- **Method:** $T_{1}/T_{2} = exp$(mu theta), V-belt exp(mu theta / sin(beta/2))
- **Assumes:** impending slip, no centrifugal effect; $mu > 0$, wrap > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('belt_friction_capstan', T2=200.0, mu=0.3, theta=3.141592653589793)
# -> T1 = 513.266 N, ratio = 2.56633, friction_force = 313.266 N   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('belt_friction_capstan', T2=200.0, mu=0.3, theta=3.141592653589793, beta=0.6283185307179586)
# -> T1 = 4222.74 N, ratio = 21.1137, friction_force = 4022.74 N   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Capstan equation: T2 = 200 N, mu = 0.3, theta = 180 deg | SOLVED | T1 = 513.266 N, ratio = 2.56633, friction_force = 313.266 N |
| Capstan equation: T2 = 200 N, mu = 0, theta = 180 deg | INVALID_INPUT | Belt or rope friction on a drum (capstan equation): mu and the wrap angle must be a positive number. |

<a id="belt_drive_power"></a>

### Power of a belt drive with centrifugal tension — `belt_drive_power`

The centrifugal tension, slack-side tension and transmitted power of a flat or V-belt running at the allowable tight-side tension and impending slip.

$$
\frac{T_1 - mv^2}{T_2 - mv^2} = e^{\mu\theta},\quad P = (T_1-T_2)v
$$

| input | meaning | condition |
|---|---|---|
| `T1` | allowable tight-side tension |  |
| `m` | belt mass per unit length |  |
| `v` | belt speed |  |
| `mu` | coefficient of friction |  |
| `theta` | wrap angle on the smaller pulley |  |
| `beta` | included groove angle for a V-belt (optional) |  |

| output | meaning |
|---|---|
| `Tc` | centrifugal tension m $v^{2}$ |
| `T2` | slack-side tension |
| `P` | transmitted power |
| `P_kW` | transmitted power in kW |

- **Method:** $(T_{1} - Tc)/(T_{2} - Tc) = exp$(mu theta) (mu/sin(beta/2) for a V-belt), $Tc = m$ $v^{2}$, $P = (T_{1} - T_{2})$ v
- **Assumes:** impending slip on the smaller pulley; m $v^{2} < T_{1}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('belt_drive_power', T1=1200.0, m=0.4, v=20.0, mu=0.3, theta=2.792526803190927)
# -> Tc = 160 N, T2 = 609.987 N, P = 11800.3 W, P_kW = 11.8003   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Belt drive power: T1 = 1.2 kN, m = 0.4 kg/m, v = 20 m/s, mu = 0.3, theta = 160 deg | SOLVED | Tc = 160 N, T2 = 609.987 N, P = 11800.3 W, P_kW = 11.8003 |
| Belt drive power: T1 = 100 N, m = 0.4 kg/m, v = 20 m/s, mu = 0.3, theta = 160 deg | INVALID_INPUT | Power of a belt drive with centrifugal tension: the centrifugal tension m v^2 reaches the allowable tension: the belt tr |

<a id="power_screw_torque"></a>

### Power screw: raising and lowering torque, efficiency, self-locking — `power_screw_torque`

The torque to raise and to lower a load with a square or Acme power screw (thread and collar friction), the raising efficiency and whether the screw is self-locking.

$$
T_R = \frac{Fd_m}{2}\,\frac{l + \pi fd_m\sec\alpha}{\pi d_m - fl\sec\alpha} + \frac{Ff_cd_c}{2}
$$

| input | meaning | condition |
|---|---|---|
| `F` | axial load |  |
| `dm` | mean thread diameter |  |
| `l` | lead |  |
| `f` | thread friction coefficient |  |
| `alpha` | thread half-angle (0 for square, 14.5 deg for Acme; default 0) |  |
| `fc` | collar friction coefficient (default 0) |  |
| `dc` | collar mean diameter (needed with fc) |  |

| output | meaning |
|---|---|
| `T_raise` | torque to raise the load |
| `T_lower` | torque to lower the load (negative: the load lowers itself) |
| `efficiency` | efficiency when raising |
| `self_locking` | yes if the thread alone holds the load |

- **Method:** $T_R = F$ $dm/2$ (l + pi f dm sec a)/(pi dm - f l sec a) + F fc $dc/2$; $T_L = F$ $dm/2$ (pi f dm sec a - l)/(pi dm + f l sec a) + F fc $dc/2$; $e = F$ l/(2 pi T_R)
- **Assumes:** $\alpha < 45$ degrees; thread friction taken as f/cos(alpha); single thrust collar
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('power_screw_torque', F=6400.0, dm=0.03, l=0.012, f=0.08, fc=0.08, dc=0.04)
# -> T_raise = 30.3479 N*m, T_lower = 5.74271 N*m, efficiency = 0.402766, self_locking = no   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Power screw: F = 6.4 kN, dm = 30 mm, l = 12 mm, f = 0.08, fc = 0.08, dc = 40 mm | SOLVED | T_raise = 30.3479 N*m, T_lower = 5.74271 N*m, efficiency = 0.402766, self_locking = no |
| Power screw: F = 6.4 kN, dm = 30 mm, l = 12 mm, f = 0.08, fc = 0.08 | INVALID_INPUT | Power screw: raising and lowering torque, efficiency, self-locking: a collar friction coefficient needs the collar mean  |

<a id="disc_clutch_torque"></a>

### Torque capacity of a disc clutch or brake — `disc_clutch_torque`

The friction torque and the largest contact pressure of a plate (disc) clutch or brake with an annular friction face, by the uniform-wear or uniform-pressure assumption.

$$
T_{wear} = \frac{Ff(D+d)}{4},\quad T_{press} = \frac{Ff}{3}\,\frac{D^3-d^3}{D^2-d^2}
$$

| input | meaning | condition |
|---|---|---|
| `F` | axial actuating force |  |
| `f` | coefficient of friction |  |
| `D` | outer diameter of the friction face |  |
| `d` | inner diameter of the friction face |  |
| `model` | uniform wear (worn-in) or uniform pressure (new) |  |
| `pairs` | number of friction-surface pairs (default 1) |  |

| output | meaning |
|---|---|
| `T` | torque capacity |
| `p_max_MPa` | largest contact pressure, MPa |

- **Method:** uniform wear: $T = F$ f $(D + d)/4$, p_max = 2F/(pi d (D - d)); uniform pressure: $T = F$ f (D^3 - d^3)/(3(D^2 - d^2)), p = 4F/(pi $(D^{2} - d^{2})$); times the number of pairs
- **Assumes:** $d < D$; the same force on every pair
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('disc_clutch_torque', F=4000.0, f=0.3, D=0.2, d=0.1, model='uniform wear')
# -> T = 90 N*m, p_max_MPa = 0.254648   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('disc_clutch_torque', F=4000.0, f=0.3, D=0.2, d=0.1, model='uniform pressure', pairs=2)
# -> T = 186.667 N*m, p_max_MPa = 0.169765   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Disc clutch: F = 4 kN, f = 0.3, D = 200 mm, d = 100 mm, model = uniform wear | SOLVED | T = 90 N*m, p_max_MPa = 0.254648 |
| Disc clutch: F = 4 kN, f = 0.3, D = 200 mm, d = 100 mm, model = uniform pressure, pairs = 2 | SOLVED | T = 186.667 N*m, p_max_MPa = 0.169765 |
| Disc clutch: F = 4 kN, f = 0.3, D = 100 mm, d = 200 mm, model = uniform wear | INVALID_INPUT | Torque capacity of a disc clutch or brake: the inner diameter must be smaller than the outer diameter. |

<a id="planetary_gear_train"></a>

### Speeds of a simple planetary (epicyclic) gear train — `planetary_gear_train`

The third speed of a sun-planet-ring-carrier train from the other two (any member may be held: speed 0), with the planets' absolute speed and tooth number.

$$
\frac{\omega_s - \omega_c}{\omega_r - \omega_c} = -\frac{N_r}{N_s}
$$

| input | meaning | condition |
|---|---|---|
| `Ns` | teeth on the sun |  |
| `Nr` | teeth on the ring (internal gear) |  |
| `ws` | sun speed (signed; rpm or rad/s) |  |
| `wr` | ring speed (signed) |  |
| `wc` | carrier (arm) speed (signed) |  |

| output | meaning |
|---|---|
| `ws_rpm` | sun speed, rpm |
| `wr_rpm` | ring speed, rpm |
| `wc_rpm` | carrier speed, rpm |
| `wp_rpm` | absolute speed of each planet, rpm |
| `Np` | teeth on each planet |

- **Method:** Willis: $(ws - wc)/(wr - wc) = -Nr/Ns$; planet: $wp = wc - (ws - wc)$ Ns/Np, $Np = (Nr - Ns)/2$
- **Assumes:** exactly two of the three speeds given; $Nr > Ns$, Nr - Ns even; same sign convention for every speed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('planetary_gear_train', Ns=20, Nr=80, ws=125.66370614359171, wr=0.0)
# -> ws_rpm = 1200, wr_rpm = 0, wc_rpm = 240, wp_rpm = -400, Np = 30   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('planetary_gear_train', Ns=20, Nr=80, wc=10.471975511965976, wr=0.0)
# -> ws_rpm = 500, wr_rpm = 0, wc_rpm = 100, wp_rpm = -166.667, Np = 30   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Planetary gear train: Ns = 20, Nr = 80, ws = 1200 rpm, wr = 0 rpm | SOLVED | ws_rpm = 1200, wr_rpm = 0, wc_rpm = 240, wp_rpm = -400, Np = 30 |
| Planetary gear train: Ns = 20, Nr = 81, ws = 1200 rpm, wr = 0 rpm | INVALID_INPUT | Speeds of a simple planetary (epicyclic) gear train: need Nr > Ns > 0 with Nr - Ns even (the planets have (Nr - Ns)/2 te |
| Planetary gear train: Ns = 20, Nr = 80, ws = 1200 rpm, wr = 0 rpm, wc = 10 rpm | INVALID_INPUT | Speeds of a simple planetary (epicyclic) gear train: give exactly two of the three speeds ws, wr, wc. |

<a id="spur_gear_contact_ratio"></a>

### Spur gear mesh geometry and contact ratio — `spur_gear_contact_ratio`

Pitch diameters, centre distance, length of the path of contact, base pitch and contact ratio of a pair of standard full-depth spur gears.

$$
Z = \sqrt{r_{a1}^2-r_{b1}^2} + \sqrt{r_{a2}^2-r_{b2}^2} - C\sin\phi,\quad m_c = \frac{Z}{\pi m\cos\phi}
$$

| input | meaning | condition |
|---|---|---|
| `N1` | teeth on the pinion |  |
| `N2` | teeth on the gear |  |
| `m` | module (e.g. 4 mm) |  |
| `phi` | pressure angle |  |

| output | meaning |
|---|---|
| `d1_mm` | pinion pitch diameter, mm |
| `d2_mm` | gear pitch diameter, mm |
| `C_mm` | centre distance, mm |
| `Z_mm` | length of the path of contact, mm |
| `pb_mm` | base pitch, mm |
| `contact_ratio` | contact ratio Z / p_b |

- **Method:** $Z = sqrt$(ra1^2 - rb1^2) + sqrt(ra2^2 - rb2^2) - C sin(phi), $rb = r$ cos(phi), $ra = r + m$, $pb = pi$ m cos(phi), $m_c = Z/pb$
- **Assumes:** standard full-depth teeth: addendum = 1 module; standard centre distance; no interference (refused otherwise); 10 <= pressure angle <= 30 degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('spur_gear_contact_ratio', N1=20, N2=50, m=0.004, phi=0.3490658503988659)
# -> d1_mm = 80, d2_mm = 200, C_mm = 140, Z_mm = 19.552, pb_mm = 11.8085, contact_ratio = 1.65576   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Spur gear contact ratio: N1 = 20, N2 = 50, m = 4 mm, phi = 20 deg | SOLVED | d1_mm = 80, d2_mm = 200, C_mm = 140, Z_mm = 19.552, pb_mm = 11.8085, contact_ratio = 1.65576 |
| Spur gear contact ratio: N1 = 8, N2 = 50, m = 4 mm, phi = 20 deg | INVALID_INPUT | Spur gear mesh geometry and contact ratio: involute interference: an addendum passes the other gear's base-circle tangen |

<a id="bearing_life_reliability"></a>

### Rolling-bearing life at a given reliability — `bearing_life_reliability`

The rating (L10) life of a ball or roller bearing from its catalogue rating and load, and the life at another reliability from the three-parameter Weibull distribution, in revolutions and hours.

$$
L_{10} = L_R\Big(\frac{C}{F}\Big)^a,\quad x = x_0 + (\theta-x_0)\Big(\ln\frac1R\Big)^{1/b}
$$

| input | meaning | condition |
|---|---|---|
| `C` | catalogue (basic dynamic) load rating |  |
| `F` | equivalent radial load |  |
| `n` | speed (rpm) |  |
| `R` | required reliability (e.g. 0.95 or 95%) |  |
| `kind` | ball $(a = 3)$ or roller $(a = 10/3)$ bearing |  |
| `LR` | rating life of the catalogue, revolutions (default 1e6) |  |
| `x0` | Weibull guaranteed life multiple (default 0.02) |  |
| `theta` | Weibull characteristic life multiple (default 4.459) |  |
| `b` | Weibull shape (default 1.483) |  |

| output | meaning |
|---|---|
| `L10_rev` | rating life (90 % reliability), revolutions |
| `L10_hours` | rating life in hours |
| `life_multiple` | life at R as a multiple of the rating life |
| `L_rev` | life at reliability R, revolutions |
| `L_hours` | life at reliability R, hours |

- **Method:** $L10 = LR$ $(C/F)^{a}$; $x = x_{0} + (\theta - x_{0})$ (ln 1/R)^(1/b); $L = x$ L10; hours = revolutions / (60 n_rpm)
- **Assumes:** $0 < R < 1$; Weibull parameters of the catalogue (defaults: $x_{0} = 0.02$, $\theta = 4.459$, b = 1.483); constant load and speed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('bearing_life_reliability', C=30000.0, F=4000.0, n=157.07963267948963, R=0.9, kind='ball')
# -> L10_rev = 4.21875e+08, L10_hours = 4687.5, life_multiple = 0.993348, L_rev = 4.19069e+08, L_hours = 4656.32   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 0.95, kind = ball | SOLVED | L10_rev = 4.21875e+08, L10_hours = 4687.5, life_multiple = 0.619056, L_rev = 2.61164e+08, L_hours = 2901.83 |
| Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 95%, kind = ball | SOLVED | L10_rev = 4.21875e+08, L10_hours = 4687.5, life_multiple = 0.619056, L_rev = 2.61164e+08, L_hours = 2901.83 |
| Bearing life: C = 30 kN, F = 4 kN, n = 1500 rpm, R = 1.2, kind = ball | INVALID_INPUT | Rolling-bearing life at a given reliability: the reliability must lie strictly between 0 and 1. |

<a id="bearing_catalog_rating"></a>

### Required catalogue rating of a rolling bearing — `bearing_catalog_rating`

The basic dynamic load rating C10 a ball or roller bearing needs for a design load, life and speed at a required reliability (three-parameter Weibull, application factor).

$$
C_{10} = a_fF_D\Big[\frac{x_D}{x_0 + (\theta-x_0)(\ln 1/R_D)^{1/b}}\Big]^{1/a}
$$

| input | meaning | condition |
|---|---|---|
| `FD` | design (equivalent radial) load |  |
| `LD` | design life (hours) |  |
| `n` | speed (rpm) |  |
| `R` | required reliability |  |
| `kind` | ball $(a = 3)$ or roller $(a = 10/3)$ |  |
| `af` | application (load) factor (default 1) |  |
| `LR` | rating life of the catalogue, revolutions (default 1e6) |  |
| `x0` | Weibull guaranteed life multiple (default 0.02) |  |
| `theta` | Weibull characteristic life multiple (default 4.459) |  |
| `b` | Weibull shape (default 1.483) |  |

| output | meaning |
|---|---|
| `C10` | required catalogue rating |
| `C10_kN` | required rating in kN |
| `design_life_multiple` | design life as a multiple of LR |

- **Method:** $C10 = af$ FD [xD/(x0 + (theta - x0)(ln 1/R)^(1/b))]^$(1/a)$, $xD = LD$ n $60/LR$
- **Assumes:** $0 < R < 1$; Weibull defaults $x_{0} = 0.02$, $\theta = 4.459$, $b = 1.483$ and $LR = 1e6$ revolutions unless given
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('bearing_catalog_rating', FD=1800.0, LD=108000000.0, n=52.35987755982988, R=0.99, kind='ball', af=1.2)
# -> C10 = 34567.2 N, C10_kN = 34.5672, design_life_multiple = 900   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bearing catalogue rating: FD = 1.8 kN, LD = 30000 h, n = 500 rpm, R = 0.99, kind = ball, af = 1.2 | SOLVED | C10 = 34567.2 N, C10_kN = 34.5672, design_life_multiple = 900 |

<a id="helical_compression_spring"></a>

### Helical compression spring: rate, deflection and shear stress — `helical_compression_spring`

The spring index, rate, deflection under a load and the wire shear stress (torsion alone, and with the direct-shear factor Ks = 1 + 0.5/C) of a round-wire helical spring.

$$
k = \frac{Gd^4}{8D^3N_a},\quad \tau = K_s\frac{8FD}{\pi d^3},\quad K_s = 1 + \frac{0.5}{C}
$$

| input | meaning | condition |
|---|---|---|
| `d` | wire diameter |  |
| `D` | mean coil diameter |  |
| `Na` | number of active coils |  |
| `G` | shear modulus of the wire |  |
| `F` | axial load |  |

| output | meaning |
|---|---|
| `index` | spring index $C = D/d$ |
| `k` | spring rate |
| `deflection_mm` | deflection under F, mm |
| `tau_torsion_MPa` | torsional shear stress 8FD/(pi d^3), MPa |
| `tau_Ks_MPa` | shear stress with the direct-shear factor Ks, MPa |

- **Method:** $k = G$ d^4/(8 $D^{3}$ Na), $\delta = F/k$, $\tau = Ks$ 8FD/(pi d^3) with $Ks = 1 + 0.5/C$
- **Assumes:** $C > 2$; close-coiled (small helix angle); curvature (Wahl) correction not included
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('helical_compression_spring', d=0.003, D=0.024, Na=10.0, G=79000000000.0, F=100.0)
# -> index = 8, k = 5786.13 N/m, deflection_mm = 17.2827, tau_torsion_MPa = 226.354, tau_Ks_MPa = 240.501   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Helical spring: d = 3 mm, D = 24 mm, Na = 10, G = 79 GPa, F = 100 N | SOLVED | index = 8, k = 5786.13 N/m, deflection_mm = 17.2827, tau_torsion_MPa = 226.354, tau_Ks_MPa = 240.501 |
| Helical spring: d = 3 mm, D = 5 mm, Na = 10, G = 79 GPa, F = 100 N | INVALID_INPUT | Helical compression spring: rate, deflection and shear stress: the spring index D/d must exceed 2. |

<a id="sdof_natural_frequency"></a>

### Natural frequency of a spring-mass system — `sdof_natural_frequency`

The natural circular frequency, frequency, period and static deflection (under gravity) of an undamped single-degree-of-freedom spring-mass system.

$$
\omega_n = \sqrt{\frac{k}{m}},\quad f_n = \frac{1}{2\pi}\sqrt{\frac{k}{m}} = \frac{1}{2\pi}\sqrt{\frac{g}{\delta_{st}}}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass |  |
| `k` | spring stiffness |  |

| output | meaning |
|---|---|
| `omega_n` | natural circular frequency |
| `f_n` | natural frequency |
| `period` | period |
| `static_deflection_mm` | static deflection m g/k in mm (g = 9.80665 m/s^2) |

- **Method:** $\omega_{n} = \sqrt{\frac{k}{m}}$, f_n = omega_n/(2 pi), $T = 1/f_n$, $delta_st = m$ g/k
- **Assumes:** m, $k > 0$; linear spring, no damping
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('sdof_natural_frequency', m=20.0, k=8000.0)
# -> omega_n = 20 rad/s, f_n = 3.1831 Hz, period = 0.314159 s, static_deflection_mm = 24.5166   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SDOF natural frequency: m = 20 kg, k = 8 kN/m | SOLVED | omega_n = 20 rad/s, f_n = 3.1831 Hz, period = 0.314159 s, static_deflection_mm = 24.5166 |
| Natural frequency of a spring-mass system: m = 20 kg, k = 8000 N/m | SOLVED | omega_n = 20 rad/s, f_n = 3.1831 Hz, period = 0.314159 s, static_deflection_mm = 24.5166 |
| SDOF natural frequency: m = 20 kg | NEEDS_INPUT | sdof_natural_frequency needs k (spring stiffness). Example: solve('sdof_natural_frequency', m='20 kg', k='8000 N/m'). |

<a id="damped_sdof_parameters"></a>

### Damping ratio, damped frequency and logarithmic decrement — `damped_sdof_parameters`

The natural frequency, damping ratio, critical damping, damped natural frequency and logarithmic decrement of a viscously damped spring-mass system, and whether it is under- or overdamped.

$$
\zeta = \frac{c}{2\sqrt{km}},\quad \omega_d = \omega_n\sqrt{1-\zeta^2},\quad \delta = \frac{2\pi\zeta}{\sqrt{1-\zeta^2}}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass |  |
| `c` | viscous damping coefficient $(N s/m)$ |  |
| `k` | stiffness |  |

| output | meaning |
|---|---|
| `omega_n` | undamped natural frequency |
| `zeta` | damping ratio c/c_c |
| `omega_d` | damped natural frequency (0 if not underdamped) |
| `f_d` | damped natural frequency in Hz |
| `c_critical` | critical damping 2 sqrt(k m) |
| `log_decrement` | logarithmic decrement (0 if not underdamped) |
| `regime` | underdamped, critically damped or overdamped |

- **Method:** zeta = c/(2 sqrt(k m)), $omega_d = omega_n$ sqrt(1 - zeta^2), $\delta = 2$ pi zeta/sqrt(1 - zeta^2)
- **Assumes:** m, $k > 0$, $c  \geq  0$; linear viscous damping
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('damped_sdof_parameters', m=10.0, c=40.0, k=4000.0)
# -> omega_n = 20 rad/s, zeta = 0.1, omega_d = 19.8997 rad/s, f_d = 3.16714 Hz, c_critical = 400 kg/s, log_decrement = 0.631484, regime = underdamped   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Damped SDOF: m = 10 kg, c = 40 N*s/m, k = 4000 N/m | SOLVED | omega_n = 20 rad/s, zeta = 0.1, omega_d = 19.8997 rad/s, f_d = 3.16714 Hz, c_critical = 400 kg/s, log_decrement = 0.631484, regime = underdamped |
| Damping ratio: m = 10 kg, c = 800 N*s/m, k = 4000 N/m | SOLVED | omega_n = 20 rad/s, zeta = 2, omega_d = 0 rad/s, f_d = 0 Hz, c_critical = 400 kg/s, log_decrement = 0, regime = overdamped |

<a id="sdof_free_response"></a>

### Free vibration response of a damped spring-mass system — `sdof_free_response`

The displacement and velocity at time t of a damped single-degree-of-freedom system released from an initial displacement and velocity (under-, critically or overdamped).

$$
x(t) = e^{-\zeta\omega_nt}\Big(x_0\cos\omega_dt + \frac{v_0+\zeta\omega_nx_0}{\omega_d}\sin\omega_dt\Big)
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass |  |
| `c` | damping coefficient |  |
| `k` | stiffness |  |
| `x0` | initial displacement |  |
| `v0` | initial velocity |  |
| `t` | time |  |

| output | meaning |
|---|---|
| `x` | displacement at t |
| `x_mm` | displacement at t in mm |
| `v` | velocity at t |

- **Method:** underdamped x = e^(-zeta wn t)(x0 cos wd t + (v0 + zeta wn x0)/wd sin wd t); critical x = (x0 + (v0 + wn x0) t) e^(-wn t); overdamped sum of two exponentials
- **Assumes:** linear viscous damping; no external force; zeta within $1e-9$ of 1 is treated as critical
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('sdof_free_response', m=1.0, c=2.0, k=100.0, x0=0.01, v0=0.0, t=0.5)
# -> x = 0.000985507 m, x_mm = 0.985507, v = 0.0588697 m/s   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('sdof_free_response', m=1.0, c=20.0, k=100.0, x0=0.01, v0=0.0, t=0.1)
# -> x = 0.00735759 m, x_mm = 7.35759, v = -0.0367879 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SDOF free response: m = 1 kg, c = 20 N*s/m, k = 100 N/m, x0 = 10 mm, v0 = 0 m/s, t = 0.1 s | SOLVED | x = 0.00735759 m, x_mm = 7.35759, v = -0.0367879 m/s |
| SDOF free response: m = 1 kg, c = 20 N*s/m, k = 100 N/m, x0 = 10 mm, v0 = 0 m/s | NEEDS_INPUT | sdof_free_response needs t (time). Example: solve('sdof_free_response', m='1 kg', c='2 kg/s', k='100 N/m', x0='0.01 m',  |

<a id="forced_vibration_amplitude"></a>

### Steady-state response to a harmonic force — `forced_vibration_amplitude`

The steady-state amplitude, phase lag and dynamic magnification factor of a damped spring-mass system driven by a harmonic force F0 cos(omega t).

$$
X = \frac{F_0/k}{\sqrt{(1-r^2)^2 + (2\zeta r)^2}},\quad \tan\phi = \frac{2\zeta r}{1-r^2}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass |  |
| `c` | damping coefficient |  |
| `k` | stiffness |  |
| `F0` | force amplitude |  |
| `omega` | forcing frequency (rad/s or rpm) |  |

| output | meaning |
|---|---|
| `X` | steady-state amplitude |
| `X_mm` | amplitude in mm |
| `phase_deg` | phase lag of the displacement behind the force, degrees (0 to 180) |
| `magnification` | dynamic magnification X $k/F_{0}$ |
| `r` | frequency ratio omega/omega_n |

- **Method:** $X = F_{0}/sqrt$((k - m $w^{2})^{2} + (c$ w)^2), phi = atan2(c w, k - m w^2)
- **Assumes:** steady state (transient decayed); not undamped resonance
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('forced_vibration_amplitude', m=10.0, c=40.0, k=4000.0, F0=100.0, omega=20.0)
# -> X = 0.125 m, X_mm = 125, phase_deg = 90, magnification = 5, r = 1   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Forced vibration: m = 10 kg, c = 40 N*s/m, k = 4000 N/m, F0 = 100 N, omega = 20 rad/s | SOLVED | X = 0.125 m, X_mm = 125, phase_deg = 90, magnification = 5, r = 1 |
| Forced vibration: m = 10 kg, c = 0 N*s/m, k = 4000 N/m, F0 = 100 N, omega = 20 rad/s | INVALID_INPUT | Steady-state response to a harmonic force: undamped resonance: the amplitude grows without bound. |

<a id="vibration_transmissibility"></a>

### Force (or base-motion) transmissibility of a vibration isolator — `vibration_transmissibility`

The ratio of transmitted to applied force amplitude (equally, of mass to base displacement amplitude) for a damped isolator at a frequency ratio, and the isolation achieved.

$$
TR = \sqrt{\frac{1 + (2\zeta r)^2}{(1-r^2)^2 + (2\zeta r)^2}}
$$

| input | meaning | condition |
|---|---|---|
| `zeta` | damping ratio |  |
| `r` | frequency ratio omega/omega_n |  |

| output | meaning |
|---|---|
| `TR` | transmissibility |
| `isolation_percent` | isolation $(1 - TR)$ in percent (0 when TR >= 1) |

- **Method:** $TR = sqrt$((1 + (2 zeta r)^2)/((1 - r^2)^2 + (2 zeta r)^2))
- **Assumes:** $\zeta  \geq  0$, $r > 0$; not undamped resonance (r = 1 with zeta = 0)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('vibration_transmissibility', zeta=0.0, r=2.0)
# -> TR = 0.333333, isolation_percent = 66.6667   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('vibration_transmissibility', zeta=0.1, r=1.4142135623730951)
# -> TR = 1, isolation_percent = 4.44089e-14   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transmissibility: zeta = 0, r = 2 | SOLVED | TR = 0.333333, isolation_percent = 66.6667 |
| Transmissibility: zeta = 0, r = 1 | INVALID_INPUT | Force (or base-motion) transmissibility of a vibration isolator: undamped resonance: the transmissibility is unbounded. |

<a id="rotating_unbalance_response"></a>

### Vibration due to rotating unbalance — `rotating_unbalance_response`

The steady-state amplitude of a machine on springs and dampers excited by a rotating unbalance, the unbalance force and the force transmitted to the foundation.

$$
X = \frac{m_ue\omega^2}{\sqrt{(k-M\omega^2)^2 + (c\omega)^2}},\quad F_T = X\sqrt{k^2+(c\omega)^2}
$$

| input | meaning | condition |
|---|---|---|
| `M` | total mass of the machine |  |
| `mu` | unbalanced mass |  |
| `e` | eccentricity of the unbalanced mass |  |
| `k` | total stiffness of the mounts |  |
| `c` | damping coefficient |  |
| `omega` | rotational speed |  |

| output | meaning |
|---|---|
| `X` | amplitude of the machine |
| `X_mm` | amplitude in mm |
| `F_transmitted` | force amplitude transmitted to the foundation |
| `F_unbalance` | rotating unbalance force mu e $\omega^{2}$ |

- **Method:** $X = mu$ e $w^{2}/sqrt$((k - M $w^{2})^{2} + (c$ w)^2), $F_T = X$ sqrt(k^2 + (c w)^2)
- **Assumes:** vertical motion only, steady state; $mu < M$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('rotating_unbalance_response', M=100.0, mu=0.5, e=0.05, k=400000.0, c=0.0, omega=100.0)
# -> X = 0.000416667 m, X_mm = 0.416667, F_transmitted = 166.667 N, F_unbalance = 250 N   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rotating unbalance: M = 100 kg, mu = 0.5 kg, e = 50 mm, k = 400 kN/m, c = 0 N*s/m, omega = 100 rad/s | SOLVED | X = 0.000416667 m, X_mm = 0.416667, F_transmitted = 166.667 N, F_unbalance = 250 N |
| Rotating unbalance: M = 100 kg, mu = 200 kg, e = 50 mm, k = 400 kN/m, c = 0 N*s/m, omega = 100 rad/s | INVALID_INPUT | Vibration due to rotating unbalance: the unbalanced mass must be part of the total mass M. |

<a id="vibration_isolator_design"></a>

### Stiffness of an undamped isolator for a target transmissibility — `vibration_isolator_design`

The mount stiffness, natural frequency, frequency ratio and static deflection an undamped isolator needs so that a machine running at omega transmits only a fraction TR of its force.

$$
TR = \frac{1}{r^2-1},\quad k = m\Big(\frac{\omega}{r}\Big)^2
$$

| input | meaning | condition |
|---|---|---|
| `m` | isolated mass |  |
| `omega` | forcing frequency (rad/s or rpm) |  |
| `TR` | target transmissibility (0 < TR < 1, e.g. 0.1 or 10%) |  |

| output | meaning |
|---|---|
| `k` | required total stiffness |
| `f_n` | natural frequency of the mounted system |
| `r` | frequency ratio omega/omega_n |
| `static_deflection_mm` | static deflection of the mounts, mm |

- **Method:** $TR = 1/(r^{2} - 1)$ -> $r = \sqrt{1 + \frac{1}{TR}}$, $k = m$ $(\omega/r)^{2}$
- **Assumes:** undamped isolator; $0 < TR < 1$ (isolation region $r > sqrt$(2))
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('vibration_isolator_design', m=200.0, omega=157.07963267948966, TR=0.1)
# -> k = 448618 N/m, f_n = 7.53778 Hz, r = 3.31662, static_deflection_mm = 4.37193   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 0.1 | SOLVED | k = 448618 N/m, f_n = 7.53778 Hz, r = 3.31662, static_deflection_mm = 4.37193 |
| Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 10% | SOLVED | k = 448618 N/m, f_n = 7.53778 Hz, r = 3.31662, static_deflection_mm = 4.37193 |
| Vibration isolator design: m = 200 kg, omega = 1500 rpm, TR = 1.5 | INVALID_INPUT | Stiffness of an undamped isolator for a target transmissibility: the target transmissibility must lie strictly between 0 |

<a id="two_rotor_torsional_vibration"></a>

### Torsional natural frequency of a two-rotor system — `two_rotor_torsional_vibration`

The torsional natural frequency of two rotors joined by an elastic shaft, the ratio of their vibration amplitudes and (with the shaft length) where the node lies.

$$
\omega = \sqrt{\frac{k_t(J_1+J_2)}{J_1J_2}},\quad l_1 = \frac{J_2L}{J_1+J_2}
$$

| input | meaning | condition |
|---|---|---|
| `J1` | mass moment of inertia of rotor 1 |  |
| `J2` | mass moment of inertia of rotor 2 |  |
| `kt` | torsional stiffness of the shaft (N*m/rad) |  |
| `L` | shaft length (optional: gives the node position) |  |

| output | meaning |
|---|---|
| `omega` | torsional natural frequency |
| `f` | natural frequency in Hz |
| `amplitude_ratio` | amplitude of rotor 2 / rotor 1 (negative: opposite phase) |
| `node_from_rotor1` | distance of the node from rotor 1 (with L) |

- **Method:** $\omega^{2} = kt$ (J1 + J2)/(J1 J2); amplitude ratio -J1/J2; node at L $J_{2}/(J_{1} + J_{2})$ from rotor 1
- **Assumes:** shaft inertia neglected; free-free system (no fixed end)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('two_rotor_torsional_vibration', J1=2.0, J2=6.0, kt=30000.0, L=1.2)
# -> omega = 141.421 rad/s, f = 22.5079 Hz, amplitude_ratio = -0.333333, node_from_rotor1 = 0.9 m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Two-rotor torsional vibration: J1 = 2 kg*m^2, J2 = 6 kg*m^2, kt = 30 kN*m/rad, L = 1.2 m | SOLVED | omega = 141.421 rad/s, f = 22.5079 Hz, amplitude_ratio = -0.333333, node_from_rotor1 = 0.9 m |
| Two-rotor torsional vibration: J1 = 2 kg*m^2, J2 = 6 kg*m^2 | NEEDS_INPUT | two_rotor_torsional_vibration needs kt (torsional stiffness of the shaft (N*m/rad)). Example: solve('two_rotor_torsional |

<a id="mdof_spring_mass_chain"></a>

### Natural frequencies and first mode of a spring-mass chain — `mdof_spring_mass_chain`

All natural frequencies and the first mode shape of a chain of masses joined by springs, fixed to a wall at the left end (and optionally at the right end).

$$
(\mathbf K - \omega^2\mathbf M)\boldsymbol\phi = \mathbf 0
$$

| input | meaning | condition |
|---|---|---|
| `masses` | masses in kg, left to right (a list) |  |
| `springs` | spring stiffnesses in N/m, the wall spring first (a list) |  |
| `ends` | fixed-free (n springs) or fixed-fixed (n + 1 springs) |  |

| output | meaning |
|---|---|
| `omega` | natural circular frequencies in rad/s, ascending |
| `f_Hz` | natural frequencies in Hz |
| `mode1` | first mode shape, largest entry +1 |

- **Method:** K $\phi = w^{2}$ M phi with tridiagonal K; solved as the symmetric $M^{-1/2}$ K $M^{-1/2}$ by Jacobi rotations
- **Assumes:** 1 to 12 masses; linear springs, no damping; springs: n (fixed-free) or $n + 1$ (fixed-fixed)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('mdof_spring_mass_chain', masses='2, 1', springs='2000, 1000', ends='fixed-free')
# -> omega = [22.360679775, 44.72135955], f_Hz = [3.55881271709, 7.11762543417], mode1 = [0.5, 1.0]   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('mdof_spring_mass_chain', masses='1, 1', springs='100, 100, 100', ends='fixed-fixed')
# -> omega = [10.0, 17.3205080757], f_Hz = [1.59154943092, 2.75664447711], mode1 = [1.0, 1.0]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Spring-mass chain: masses = [2, 1], springs = [2000, 1000], ends = fixed-free | SOLVED | omega = [22.360679775, 44.72135955], f_Hz = [3.55881271709, 7.11762543417], mode1 = [0.5, 1.0] |
| Spring-mass chain: masses = [2, 1], springs = [2000], ends = fixed-free | INVALID_INPUT | Natural frequencies and first mode of a spring-mass chain: a fixed-free chain of 2 masses needs 2 springs (wall spring f |

<a id="shaft_critical_speed_single_disk"></a>

### Critical speed of a shaft carrying one disk — `shaft_critical_speed_single_disk`

The lateral stiffness at the disk, the first critical (whirling) speed and the static deflection of a simply supported massless shaft carrying one disk.

$$
k = \frac{3EIL}{a^2b^2},\quad \omega_{cr} = \sqrt{\frac{k}{m}}
$$

| input | meaning | condition |
|---|---|---|
| `m` | disk mass |  |
| `E` | Young's modulus of the shaft |  |
| `I` | second moment of area of the shaft |  |
| `L` | bearing span |  |
| `a` | distance of the disk from the left bearing |  |

| output | meaning |
|---|---|
| `k` | lateral stiffness of the shaft at the disk |
| `omega_cr` | critical speed |
| `N_cr_rpm` | critical speed in rpm |
| `static_deflection_mm` | static deflection under the disk's weight, mm |

- **Method:** $k = 3$ E I L/(a^2 b^2), $\omega_{cr} = \sqrt{\frac{k}{m}}$
- **Assumes:** simple (short) bearings, uniform massless shaft; gyroscopic effects neglected; $0 < a < L$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('shaft_critical_speed_single_disk', m=50.0, E=200000000000.0, I=2e-07, L=1.0, a=0.5)
# -> k = 1.92e+06 N/m, omega_cr = 195.959 rad/s, N_cr_rpm = 1871.27, static_deflection_mm = 0.255382   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shaft critical speed: m = 50 kg, E = 200 GPa, I = 2e-7 m^4, L = 1 m, a = 0.5 m | SOLVED | k = 1.92e+06 N/m, omega_cr = 195.959 rad/s, N_cr_rpm = 1871.27, static_deflection_mm = 0.255382 |
| Shaft critical speed: m = 50 kg, E = 200 GPa, I = 2e-7 m^4, L = 1 m, a = 1.5 m | INVALID_INPUT | Critical speed of a shaft carrying one disk: the disk must sit between the bearings (0 < a < L). |

<a id="rayleigh_critical_speed"></a>

### First critical speed of a shaft with several disks (Rayleigh) — `rayleigh_critical_speed`

Rayleigh's estimate of the first critical speed of a simply supported massless shaft carrying several disks, from the static deflections under the disks' weights.

$$
\omega_{cr}^2 = \frac{g\sum m_iy_i}{\sum m_iy_i^2}
$$

| input | meaning | condition |
|---|---|---|
| `masses` | disk masses in kg (a list) |  |
| `positions` | disk positions from the left bearing, in m (a list) |  |
| `L` | bearing span |  |
| `E` | Young's modulus |  |
| `I` | second moment of area of the shaft |  |

| output | meaning |
|---|---|
| `omega_cr` | first critical speed (Rayleigh, an upper bound) |
| `N_cr_rpm` | critical speed in rpm |
| `deflections_mm` | static deflection at each disk, mm |

- **Method:** y_i = sum_j a_ij m_j g (influence coefficients of the simply supported beam); $\omega^{2} = g$ sum m_i y_i / sum m_i $y_i^{2}$
- **Assumes:** simply supported uniform massless shaft; disks at distinct positions inside the span; $g = 9.80665$ $m/s^{2}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('rayleigh_critical_speed', masses='50', positions='0.5', L=1.0, E=200000000000.0, I=2e-07)
# -> omega_cr = 195.959 rad/s, N_cr_rpm = 1871.27, deflections_mm = [0.255381510417]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rayleigh critical speed: masses = [50], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4 | SOLVED | omega_cr = 195.959 rad/s, N_cr_rpm = 1871.27, deflections_mm = [0.255381510417] |
| Rayleigh critical speed: masses = [50, 20], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4 | INVALID_INPUT | First critical speed of a shaft with several disks (Rayleigh): masses and positions must be lists of the same length (1  |

<a id="dunkerley_critical_speed"></a>

### First critical speed of a shaft with several disks (Dunkerley) — `dunkerley_critical_speed`

Dunkerley's lower-bound estimate of the first critical speed of a simply supported massless shaft carrying several disks.

$$
\frac{1}{\omega_{cr}^2} = \sum_i m_ia_{ii},\quad a_{ii} = \frac{a_i^2b_i^2}{3EIL}
$$

| input | meaning | condition |
|---|---|---|
| `masses` | disk masses in kg (a list) |  |
| `positions` | disk positions from the left bearing, in m (a list) |  |
| `L` | bearing span |  |
| `E` | Young's modulus |  |
| `I` | second moment of area of the shaft |  |

| output | meaning |
|---|---|
| `omega_cr` | first critical speed (Dunkerley, a lower bound) |
| `N_cr_rpm` | critical speed in rpm |

- **Method:** 1/omega^2 = sum m_i a_ii with $a_ii = b_i^{2}$ a_i^2/(3 E I L)
- **Assumes:** simply supported uniform massless shaft; disks at distinct positions inside the span
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('dunkerley_critical_speed', masses='50', positions='0.5', L=1.0, E=200000000000.0, I=2e-07)
# -> omega_cr = 195.959 rad/s, N_cr_rpm = 1871.27   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Dunkerley critical speed: masses = [50], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4 | SOLVED | omega_cr = 195.959 rad/s, N_cr_rpm = 1871.27 |

<a id="shaft_whirling_speeds"></a>

### All critical (whirling) speeds of a shaft with several disks — `shaft_whirling_speeds`

The exact critical speeds of a simply supported massless shaft carrying point-mass disks, from the eigenvalues of its flexibility matrix.

$$
\det\Big(\mathbf A\mathbf M - \frac{1}{\omega^2}\mathbf I\Big) = 0
$$

| input | meaning | condition |
|---|---|---|
| `masses` | disk masses in kg (a list) |  |
| `positions` | disk positions from the left bearing, in m (a list) |  |
| `L` | bearing span |  |
| `E` | Young's modulus |  |
| `I` | second moment of area of the shaft |  |

| output | meaning |
|---|---|
| `omega` | critical speeds in rad/s, ascending |
| `N_rpm` | critical speeds in rpm |

- **Method:** det(A $M - (1/\omega^{2})$ I) = 0 with the influence coefficients a_ij; eigenvalues of the symmetric $M^{1/2}$ A $M^{1/2}$ by Jacobi rotations
- **Assumes:** simply supported uniform massless shaft, point-mass disks (no rotary inertia or gyroscopics); distinct positions
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('shaft_whirling_speeds', masses='50', positions='0.5', L=1.0, E=200000000000.0, I=2e-07)
# -> omega = [195.959179423], N_rpm = [1871.27232296]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shaft whirling speeds: masses = [50], positions = [0.5], L = 1 m, E = 200 GPa, I = 2e-7 m^4 | SOLVED | omega = [195.959179423], N_rpm = [1871.27232296] |

<a id="beam_natural_frequency"></a>

### Natural frequencies of a uniform beam (Euler-Bernoulli) — `beam_natural_frequency`

The n-th bending natural frequency of a uniform Euler-Bernoulli beam with simply supported, cantilever, fixed-fixed, fixed-pinned or free-free ends.

$$
\omega_n = (\beta_nL)^2\sqrt{\frac{EI}{\rho AL^4}},\quad \cos\beta L\cosh\beta L = -1\ \text{(cantilever)}
$$

| input | meaning | condition |
|---|---|---|
| `E` | Young's modulus |  |
| `I` | second moment of area |  |
| `rho` | density |  |
| `A` | cross-sectional area |  |
| `L` | length |  |
| `ends` | end conditions |  |
| `n` | mode number (default 1; free-free counts the elastic modes) |  |

| output | meaning |
|---|---|
| `beta_L` | eigenvalue beta L |
| `omega` | natural circular frequency |
| `f` | natural frequency |

- **Method:** omega_n = (beta_n L)^2 sqrt(E I/(rho A L^4)); beta L from $sin = 0$ (SS), cos cosh = -1 (cantilever), cos cosh = 1 (fixed-fixed, free-free), tan = tanh (fixed-pinned)
- **Assumes:** slender uniform beam, no shear deformation or rotary inertia; mode 1 to 6
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('beam_natural_frequency', E=200000000000.0, I=1e-08, rho=7850.0, A=0.001, L=2.0, ends='simply-supported')
# -> beta_L = 3.14159, omega = 39.384 rad/s, f = 6.26816 Hz   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('beam_natural_frequency', E=200000000000.0, I=1e-08, rho=7850.0, A=0.001, L=2.0, ends='cantilever')
# -> beta_L = 1.8751, omega = 14.0304 rad/s, f = 2.23301 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Beam natural frequency: E = 200 GPa, I = 1e-8 m^4, rho = 7850 kg/m^3, A = 1000 mm^2, L = 2 m, ends = simply-supported | SOLVED | beta_L = 3.14159, omega = 39.384 rad/s, f = 6.26816 Hz |
| Beam natural frequency: E = 200 GPa, I = 1e-8 m^4, rho = 7850 kg/m^3, A = 1000 mm^2, L = 2 m, ends = cantilever, n = 9 | INVALID_INPUT | n: must be at most 6, got 9. |

<a id="truss_member_forces"></a>

### Member forces of a plane truss (method of joints) — `truss_member_forces`

The axial force in every member and the support reactions of a statically determinate plane pin-jointed truss on a pin and a roller, under joint loads.

$$
\sum_{\text{members at } j} F_m\,\hat{\mathbf e}_{m} + \mathbf R_j + \mathbf P_j = \mathbf 0
$$

| input | meaning | condition |
|---|---|---|
| `nodes` | node coordinates, one row 'x y' per node in m (node 1 is the first row) |  |
| `members` | members as pairs of node numbers, e.g. $1-2$, $2-3$ |  |
| `pin` | node number of the pin support |  |
| `roller` | node number of the roller support (vertical reaction) |  |
| `loads` | joint loads, one row 'node Fx Fy' in kN (up and right positive) |  |

| output | meaning |
|---|---|
| `forces_kN` | member forces in kN in the order given (tension positive) |
| `pin_Rx_kN` | horizontal reaction at the pin, kN |
| `pin_Ry_kN` | vertical reaction at the pin, kN |
| `roller_Ry_kN` | vertical reaction at the roller, kN |
| `max_tension_kN` | largest tension, kN |
| `max_compression_kN` | largest compression (magnitude), kN |

- **Method:** equilibrium of every joint (sum $Fx = 0$, sum Fy = 0) as one linear system in the member forces and three reactions, solved by Gaussian elimination
- **Assumes:** pin-jointed, loads at the joints; statically determinate: members + 3 = 2 x nodes, geometrically stable; roller on a horizontal surface
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('truss_member_forces', nodes='0 0; 4 0; 2 3', members='1-2, 2-3, 3-1', pin=1, roller=2, loads='3 0 -10')
# -> forces_kN = [3.33333333333, -6.00925212577, -6.00925212577], pin_Rx_kN = 0, pin_Ry_kN = 5, roller_Ry_kN = 5, max_tension_kN = 3.33333, max_compression_kN = 6.00925   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Truss member forces: nodes = [0 0; 4 0; 2 3], members = [1-2, 2-3, 3-1], pin = 1, roller = 2, loads = [3 0 -10] | SOLVED | forces_kN = [3.33333333333, -6.00925212577, -6.00925212577], pin_Rx_kN = 0, pin_Ry_kN = 5, roller_Ry_kN = 5, max_tension_kN = 3.33333, max_compression_kN = 6.00925 |
| Truss member forces: nodes = [0 0; 4 0; 2 3], members = [1-2, 2-3], pin = 1, roller = 2, loads = [3 0 -10] | INVALID_INPUT | Member forces of a plane truss (method of joints): not statically determinate: 2 members + 3 reactions must equal 2 x 3  |

<a id="continuous_beam_three_moment"></a>

### Continuous beam under uniform loads (three-moment equation) — `continuous_beam_three_moment`

The bending moments over the supports and the support reactions of a continuous beam of equal stiffness over 2 to 6 simply supported spans, each with its own uniform load.

$$
M_{i-1}L_i + 2M_i(L_i+L_{i+1}) + M_{i+1}L_{i+1} = -\frac{w_iL_i^3 + w_{i+1}L_{i+1}^3}{4}
$$

| input | meaning | condition |
|---|---|---|
| `spans` | span lengths in m, left to right (a list) |  |
| `w` | uniform load on each span in kN/m (a list) |  |

| output | meaning |
|---|---|
| `support_moments_kNm` | bending moment at each support in $kN m$, end supports first and last (hogging negative) |
| `reactions_kN` | reaction at each support in kN |

- **Method:** Clapeyron: M_(i-1) $L_i + 2$ M_i (L_i + L_(i+1)) + M_(i+1) L_(i+1) = -(w_i $L_i^{3} + w_$(i+1) L_(i+1)^3)/4; reactions from the span shears
- **Assumes:** constant EI along the beam; simple supports, no settlement; end supports carry no moment
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('continuous_beam_three_moment', spans='6, 6', w='10, 10')
# -> support_moments_kNm = [0.0, -45.0, 0.0], reactions_kN = [22.5, 75.0, 22.5]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Continuous beam: spans = [6, 6], w = [10, 10] | SOLVED | support_moments_kNm = [0.0, -45.0, 0.0], reactions_kN = [22.5, 75.0, 22.5] |
| Continuous beam: spans = [6, 6], w = [10] | INVALID_INPUT | w: needs at least 2 entries, got 1. |

<a id="slider_crank_kinematics"></a>

### Slider-crank mechanism: slider position, velocity and acceleration — `slider_crank_kinematics`

The slider position, velocity and acceleration and the connecting-rod angle of an in-line slider-crank mechanism at a crank angle, the crank turning at constant speed.

$$
x = r\cos\theta + \sqrt{l^2 - r^2\sin^2\theta},\quad v = \omega\frac{dx}{d\theta},\quad a = \omega^2\frac{d^2x}{d\theta^2}
$$

| input | meaning | condition |
|---|---|---|
| `r` | crank radius |  |
| `l` | connecting rod length |  |
| `theta` | crank angle from the line of stroke (top dead centre at 0) |  |
| `omega` | constant crank speed (rad/s or rpm) |  |

| output | meaning |
|---|---|
| `x` | distance of the slider pin from the crank centre |
| `v` | slider velocity (positive away from the crank) |
| `a` | slider acceleration |
| `rod_angle_deg` | connecting-rod angle to the line of stroke, degrees |

- **Method:** $x = r$ cos(theta) + sqrt($l^{2} - r^{2}$ $sin^{2}$ theta); $v = dx/dt$ and $a = d^{2x}/dt^{2}$ with dtheta/dt = omega
- **Assumes:** in-line (zero offset); $l > r$; constant crank speed
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('slider_crank_kinematics', r=0.05, l=0.2, theta=0.0, omega=100.0)
# -> x = 0.25 m, v = 0 m/s, a = -625 m/s^2, rod_angle_deg = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Slider-crank: r = 50 mm, l = 200 mm, theta = 0 deg, omega = 100 rad/s | SOLVED | x = 0.25 m, v = 0 m/s, a = -625 m/s^2, rod_angle_deg = 0 |
| Slider-crank: r = 50 mm, l = 40 mm, theta = 60 deg, omega = 100 rad/s | INVALID_INPUT | Slider-crank mechanism: slider position, velocity and acceleration: the connecting rod must be longer than the crank. |

<a id="four_bar_linkage_position"></a>

### Four-bar linkage position analysis — `four_bar_linkage_position`

The coupler and output-rocker angles and the transmission angle of a four-bar linkage at a given crank angle, on the open or the crossed branch.

$$
K_1\cos\theta_4 - K_2\cos\theta_2 + K_3 = \cos(\theta_2-\theta_4),\quad \theta_4 = 2\arctan\frac{-B\mp\sqrt{B^2-4AC}}{2A}
$$

| input | meaning | condition |
|---|---|---|
| `a` | input crank length |  |
| `b` | coupler length |  |
| `c` | output rocker length |  |
| `d` | ground link length |  |
| `theta2` | crank angle from the ground link |  |
| `branch` | assembly branch |  |

| output | meaning |
|---|---|
| `theta3_deg` | coupler angle, degrees in [-180, 180) |
| `theta4_deg` | output rocker angle, degrees in [-180, 180) |
| `transmission_angle_deg` | transmission angle (acute), degrees |

- **Method:** Freudenstein's equation: K1 cos $th4 - K_{2}$ cos $th2 + K_{3} = cos$(th2 - th4) solved by the half-angle substitution, $th4 = 2$ atan2(-B -+ sqrt(B^2 - 4AC), 2A); likewise th3
- **Assumes:** ground pivots at (0, 0) and (d, 0); open branch: the rocker pivot lies to the right of the directed coupler; refused when the linkage cannot close
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('four_bar_linkage_position', a=2.0, b=7.0, c=9.0, d=6.0, theta2=0.5235987755982988, branch='open')
# -> theta3_deg = 88.8372, theta4_deg = 117.286, transmission_angle_deg = 28.4488   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('four_bar_linkage_position', a=2.0, b=7.0, c=9.0, d=6.0, theta2=0.5235987755982988, branch='crossed')
# -> theta3_deg = -115.211, theta4_deg = -143.66, transmission_angle_deg = 28.4488   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Four-bar linkage: a = 2 m, b = 7 m, c = 9 m, d = 6 m, theta2 = 30 deg, branch = open | SOLVED | theta3_deg = 88.8372, theta4_deg = 117.286, transmission_angle_deg = 28.4488 |
| Four-bar linkage: a = 1 m, b = 1 m, c = 1 m, d = 5 m, theta2 = 90 deg, branch = open | INVALID_INPUT | Four-bar linkage position analysis: the linkage cannot be assembled at this input angle. |

<a id="gerber_fatigue_safety_factor"></a>

### Fatigue safety factor by the Gerber criterion — `gerber_fatigue_safety_factor`

The fatigue factor of safety along a proportional load line by the Gerber parabola sa/Se + (sm/Sut)^2 = 1, and the equivalent fully reversed amplitude.

$$
\frac{n\sigma_a}{S_e} + \left(\frac{n\sigma_m}{S_{ut}}\right)^2 = 1
$$

| input | meaning | condition |
|---|---|---|
| `sa` | alternating stress amplitude |  |
| `sm` | mean stress (>= 0) |  |
| `Se` | fully corrected endurance limit |  |
| `Sut` | ultimate tensile strength |  |

| output | meaning |
|---|---|
| `n_fatigue` | fatigue safety factor (Gerber, proportional loading) |
| `sigma_ar_MPa` | equivalent fully reversed amplitude sa/(1 - (sm/Sut)^2), MPa |

- **Method:** n sa/Se + (n sm/Sut)^2 = 1: $n = (1/2)$(Sut/sm)^2 (sa/Se) [$-1 + sqrt$(1 + (2 sm Se/(Sut sa))^2)]
- **Assumes:** $sm  \geq  0$ (compressive means not covered); sm < Sut; proportional loading
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('gerber_fatigue_safety_factor', sa=120000000.0, sm=80000000.0, Se=250000000.0, Sut=700000000.0)
# -> n_fatigue = 1.97698, sigma_ar_MPa = 121.588   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('gerber_fatigue_safety_factor', sa=100000000.0, sm=0.0, Se=250000000.0, Sut=700000000.0)
# -> n_fatigue = 2.5, sigma_ar_MPa = 100   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gerber safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa, Sut = 700 MPa | SOLVED | n_fatigue = 1.97698, sigma_ar_MPa = 121.588 |
| Gerber safety factor: sa = 120 MPa, sm = 800 MPa, Se = 250 MPa, Sut = 700 MPa | INVALID_INPUT | Fatigue safety factor by the Gerber criterion: the mean stress must be below the strength. |
| Gerber safety factor: sa = 120 MPa, sm = 80 MPa, Se = 250 MPa | NEEDS_INPUT | gerber_fatigue_safety_factor needs Sut (ultimate tensile strength). Example: solve('gerber_fatigue_safety_factor', sa='1 |

<a id="asme_elliptic_fatigue_safety_factor"></a>

### Fatigue safety factor by the ASME-elliptic criterion — `asme_elliptic_fatigue_safety_factor`

The fatigue factor of safety along a proportional load line by the ASME ellipse $(sa/Se)^{2} + (sm/Sy)^{2} = 1$, and the equivalent fully reversed amplitude.

$$
\left(\frac{n\sigma_a}{S_e}\right)^2 + \left(\frac{n\sigma_m}{S_y}\right)^2 = 1
$$

| input | meaning | condition |
|---|---|---|
| `sa` | alternating stress amplitude |  |
| `sm` | mean stress (>= 0) |  |
| `Se` | fully corrected endurance limit |  |
| `Sy` | yield strength |  |

| output | meaning |
|---|---|
| `n_fatigue` | fatigue safety factor (ASME-elliptic, proportional loading) |
| `sigma_ar_MPa` | equivalent fully reversed amplitude sa/sqrt(1 - (sm/Sy)^2), MPa |

- **Method:** (n $sa/Se)^{2} + (n$ sm/Sy)^2 = 1: $n = \frac{1}{\sqrt{\left(\frac{sa}{Se}\right)^{2} + \left(\frac{sm}{Sy}\right)^{2}}}$
- **Assumes:** $sm  \geq  0$; $sm < Sy$; proportional loading
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('asme_elliptic_fatigue_safety_factor', sa=100000000.0, sm=60000000.0, Se=220000000.0, Sy=450000000.0)
# -> n_fatigue = 2.11105, sigma_ar_MPa = 100.901   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('asme_elliptic_fatigue_safety_factor', sa=30000000.0, sm=40000000.0, Se=100000000.0, Sy=100000000.0)
# -> n_fatigue = 2, sigma_ar_MPa = 32.7327   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| ASME-elliptic safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa, Sy = 450 MPa | SOLVED | n_fatigue = 2.11105, sigma_ar_MPa = 100.901 |
| ASME-elliptic safety factor: sa = 100 MPa, sm = 500 MPa, Se = 220 MPa, Sy = 450 MPa | INVALID_INPUT | Fatigue safety factor by the ASME-elliptic criterion: the mean stress must be below the strength. |
| ASME-elliptic safety factor: sa = 100 MPa, sm = 60 MPa, Se = 220 MPa | NEEDS_INPUT | asme_elliptic_fatigue_safety_factor needs Sy (yield strength). Example: solve('asme_elliptic_fatigue_safety_factor', sa= |

<a id="lewis_gear_bending_stress"></a>

### Lewis bending stress of a gear tooth — `lewis_gear_bending_stress`

The bending stress at the root of a spur-gear tooth by the Lewis equation $\sigma = Kv$ Wt/(F m Y) (SI, module m, form factor Y based on the module).

$$
\sigma = \frac{K_v W_t}{F\,m\,Y}
$$

| input | meaning | condition |
|---|---|---|
| `Wt` | transmitted (tangential) load |  |
| `F` | face width |  |
| `m` | module |  |
| `Y` | Lewis form factor (from the tooth count and pressure angle, based on the module) |  |
| `Kv` | velocity (dynamic) factor, default 1 |  |

| output | meaning |
|---|---|
| `sigma` | tooth bending stress |
| `sigma_MPa` | tooth bending stress, MPa |

- **Method:** $\sigma = Kv$ Wt/(F m Y)
- **Assumes:** Wt, F, $m > 0$; $0 < Y  \leq  1$; $Kv  \geq  1$; load at the tip, one tooth in contact
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('lewis_gear_bending_stress', Wt=2500.0, F=0.04, m=0.004, Y=0.322)
# -> sigma = 4.85248e+07 Pa, sigma_MPa = 48.5248   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('lewis_gear_bending_stress', Wt=1000.0, F=0.05, m=0.002, Y=0.25, Kv=1.5)
# -> sigma = 6e+07 Pa, sigma_MPa = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322 | SOLVED | sigma = 4.85248e+07 Pa, sigma_MPa = 48.5248 |
| Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm, Y = 0.322, Kv = 0.5 | INVALID_INPUT | Lewis bending stress of a gear tooth: the velocity factor Kv must be at least 1. |
| Lewis bending stress: Wt = 2.5 kN, F = 40 mm, m = 4 mm | NEEDS_INPUT | lewis_gear_bending_stress needs Y (Lewis form factor (from the tooth count and pressure angle, based on the module)). Ex |

<a id="wahl_factor"></a>

### Wahl curvature factor of a helical spring — `wahl_factor`

The Wahl stress-correction factor $K_W = (4C - 1)/(4C - 4) + 0.615/C$ of a round-wire helical spring (with the Bergstrasser and direct-shear factors), and with a load the corrected maximum shear stress.

$$
K_W = \frac{4C-1}{4C-4} + \frac{0.615}{C},\quad \tau = K_W\frac{8FD}{\pi d^3}
$$

| input | meaning | condition |
|---|---|---|
| `d` | wire diameter |  |
| `D` | mean coil diameter |  |
| `F` | axial load (optional) |  |

| output | meaning |
|---|---|
| `index` | spring index $C = D/d$ |
| `K_wahl` | Wahl factor |
| `K_bergstrasser` | Bergstrasser factor $(4C + 2)/(4C - 3)$ |
| `K_s` | direct-shear factor $1 + 0.5/C$ |
| `tau_MPa` | maximum shear stress K_W 8FD/(pi d^3), MPa (with F) |

- **Method:** $K_W = (4C - 1)/(4C - 4) + 0.615/C$; $\tau = K_W$ 8 F D/(pi d^3)
- **Assumes:** $C = D/d > 2$; round wire, close-coiled
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('wahl_factor', d=0.003, D=0.024)
# -> index = 8, K_wahl = 1.18402, K_bergstrasser = 1.17241, K_s = 1.0625   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('wahl_factor', d=0.002, D=0.012, F=50.0)
# -> index = 6, K_wahl = 1.2525, K_bergstrasser = 1.2381, K_s = 1.08333, tau_MPa = 239.21   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Wahl factor: d = 3 mm, D = 24 mm | SOLVED | index = 8, K_wahl = 1.18402, K_bergstrasser = 1.17241, K_s = 1.0625 |
| Wahl factor: d = 3 mm, D = 5 mm | INVALID_INPUT | Wahl curvature factor of a helical spring: the spring index D/d must exceed 2. |
| Wahl factor: d = 3 mm | NEEDS_INPUT | wahl_factor needs D (mean coil diameter). Example: solve('wahl_factor', d='0.003 m', D='0.024 m'). |

<a id="marin_endurance_limit"></a>

### Endurance limit with the Marin factors — `marin_endurance_limit`

The corrected endurance limit $Se = ka$ kb kc kd ke kf Se' of a steel part from its tensile strength, surface finish, size, loading and reliability (Se' = 0.5 Sut up to 1400 MPa, else 700 MPa).

$$
S_e = k_a k_b k_c k_d k_e k_f\,S_e',\quad k_a = aS_{ut}^b,\quad k_e = 1 - 0.08\,z_a
$$

| input | meaning | condition |
|---|---|---|
| `Sut` | ultimate tensile strength |  |
| `surface` | surface finish (ground, machined, cold-drawn, hot-rolled, as-forged) |  |
| `load` | loading (bending, axial, torsion) |  |
| `d` | diameter (needed for bending and torsion, $2.79-254$ mm) |  |
| `R` | reliability (default 0.5; written as 0.99 or 99%) |  |
| `kd` | temperature factor (default 1) |  |
| `kf` | miscellaneous-effects factor (default 1) |  |

| output | meaning |
|---|---|
| `ka` | surface factor a Sut^b |
| `kb` | size factor |
| `kc` | load factor |
| `ke` | reliability factor $1 - 0.08$ z_a |
| `Se_prime_MPa` | uncorrected endurance limit, MPa |
| `Se_MPa` | corrected endurance limit, MPa |
| `Se` | corrected endurance limit |

- **Method:** $ka = a$ Sut^b (a, b by surface, Sut in MPa); $kb = 1.24$ $d^{-0.107}$ (d <= 51 mm) or 1.51 $d^{-0.157}$; $kc = 1$, 0.85, 0.59; $ke = 1 - 0.08$ z_a
- **Assumes:** steels (Se' = 0.5 Sut, at most 700 MPa); size factor for $2.79  \leq  d  \leq  254$ mm (rotating round sections); $0.5  \leq  R < 1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('marin_endurance_limit', Sut=690000000.0, surface='machined', load='bending', d=0.032, R=0.99)
# -> ka = 0.797777, kb = 0.855797, kc = 1, ke = 0.813892, Se_prime_MPa = 345, Se_MPa = 191.707, Se = 1.91707e+08 Pa   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('marin_endurance_limit', Sut=1600000000.0, surface='ground', load='axial')
# -> ka = 0.843931, kb = 1, kc = 0.85, ke = 1, Se_prime_MPa = 700, Se_MPa = 502.139, Se = 5.02139e+08 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Marin factors: Sut = 690 MPa, surface = machined, load = bending, d = 32 mm, R = 99% | SOLVED | ka = 0.797777, kb = 0.855797, kc = 1, ke = 0.813892, Se_prime_MPa = 345, Se_MPa = 191.707, Se = 1.91707e+08 Pa |
| Marin factors: Sut = 1600 MPa, surface = ground, load = axial | SOLVED | ka = 0.843931, kb = 1, kc = 0.85, ke = 1, Se_prime_MPa = 700, Se_MPa = 502.139, Se = 5.02139e+08 Pa |
| Marin factors: Sut = 690 MPa, surface = machined, load = bending | INVALID_INPUT | Endurance limit with the Marin factors: the diameter d is needed for bending and torsion (size factor). |
| Marin factors: Sut = 690 MPa, surface = machined | NEEDS_INPUT | marin_endurance_limit needs load (loading (bending, axial, torsion)). Example: solve('marin_endurance_limit', Sut='6.9e+ |

<a id="bolt_preload_torque"></a>

### Bolt tightening torque and preload (T = K F d) — `bolt_preload_torque`

The tightening torque $T = K$ F d that produces a bolt preload F with nut factor K and nominal diameter d, or the preload produced by a torque.

$$
T = K F_i\,d
$$

| input | meaning | condition |
|---|---|---|
| `K` | nut (torque) factor (about 0.2 for unlubricated steel) |  |
| `d` | nominal bolt diameter |  |
| `F` | preload (give F or T) |  |
| `T` | tightening torque (give F or T) |  |

| output | meaning |
|---|---|
| `T` | tightening torque |
| `F` | bolt preload |
| `F_kN` | bolt preload, kN |

- **Method:** $T = K$ F d; F = T/(K d)
- **Assumes:** $0.05  \leq  K  \leq  0.5$; exactly one of F and T
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

```python
mechanical_engineering_ai999x.solve('bolt_preload_torque', K=0.2, d=0.012, F=25000.0)
# -> T = 60 N*m, F = 25000 N, F_kN = 25   (SOLVED)
```

```python
mechanical_engineering_ai999x.solve('bolt_preload_torque', K=0.2, d=0.012, T=60.0)
# -> T = 60 N*m, F = 25000 N, F_kN = 25   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bolt torque: K = 0.2, d = 12 mm, F = 25 kN | SOLVED | T = 60 N*m, F = 25000 N, F_kN = 25 |
| Bolt torque: K = 0.2, d = 12 mm, T = 60 N*m | SOLVED | T = 60 N*m, F = 25000 N, F_kN = 25 |
| Bolt torque: K = 0.2, d = 12 mm, F = 25 kN, T = 60 N*m | INVALID_INPUT | Bolt tightening torque and preload (T = K F d): give exactly one of the preload F and the torque T. |
| Bolt torque: K = 0.2, F = 25 kN | NEEDS_INPUT | bolt_preload_torque needs d (nominal bolt diameter). Example: solve('bolt_preload_torque', K=0.2, d='0.012 m', F='25000  |
