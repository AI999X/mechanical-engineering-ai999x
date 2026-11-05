//! Every Mechanical Engineering capability's own live figures (src/figures*.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas; an input the model refuses draws nothing.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use mechanical_engineering_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/mechanical-engineering-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them, drawn by the model's own figures.
const KINDS: &[(&str, &[&str])] = &[
    ("angle_section_properties", &["section"]),
    ("asme_elliptic_fatigue_safety_factor", &["fatigue"]),
    ("axial_bar_segments", &["bar"]),
    ("bar_fixed_at_both_ends", &["bar"]),
    ("belt_drive_power", &["fbd"]),
    ("belt_friction_capstan", &["fbd"]),
    ("brittle_failure_mohr", &["envelope"]),
    ("cantilever_point_load", &["beam", "beam-2"]),
    ("cantilever_triangular_load", &["beam"]),
    ("cantilever_udl", &["beam"]),
    ("channel_section_properties", &["section"]),
    ("circular_section_properties", &["section"]),
    ("combined_axial_and_bending_stress", &["section"]),
    ("composite_section_properties", &["section"]),
    ("continuous_beam_three_moment", &["beam"]),
    ("damped_sdof_parameters", &["decision", "response"]),
    ("dunkerley_critical_speed", &["schematic"]),
    ("eccentric_bolt_group", &["schematic"]),
    ("euler_buckling", &["column"]),
    ("fixed_fixed_beam_point_load", &["beam"]),
    ("fixed_fixed_beam_udl", &["beam"]),
    ("forced_vibration_amplitude", &["response"]),
    ("four_bar_linkage_position", &["mechanism"]),
    ("gerber_fatigue_safety_factor", &["fatigue"]),
    ("goodman_fatigue_safety_factor", &["fatigue", "fatigue-2"]),
    ("hollow_circular_section_properties", &["section"]),
    ("hollow_rectangular_section_properties", &["section"]),
    ("hollow_shaft_torsion", &["profile"]),
    ("i_section_properties", &["section"]),
    ("mdof_spring_mass_chain", &["schematic"]),
    ("moment_of_a_force", &["fbd"]),
    ("plane_stress_transformation", &["mohr"]),
    ("planetary_gear_train", &["gears"]),
    ("polygon_section_properties", &["section"]),
    ("principal_stresses_2d", &["element", "mohr"]),
    ("principal_stresses_3d", &["mohr"]),
    ("propped_cantilever_point_load", &["beam"]),
    ("propped_cantilever_udl", &["beam"]),
    ("rainflow_counting", &["histogram"]),
    ("rayleigh_critical_speed", &["schematic"]),
    ("rectangular_section_properties", &["section"]),
    ("resultant_of_concurrent_forces", &["fbd"]),
    ("sdof_free_response", &["response"]),
    ("sdof_natural_frequency", &["schematic"]),
    ("secant_column", &["curve-2"]),
    ("shaft_critical_speed_single_disk", &["schematic"]),
    ("shaft_whirling_speeds", &["schematic"]),
    ("simply_supported_beam_point_load", &["beam"]),
    ("simply_supported_beam_point_loads", &["beam", "beam-2"]),
    ("simply_supported_beam_udl", &["beam"]),
    ("slider_crank_kinematics", &["mechanism"]),
    ("sn_fatigue_life", &["sn"]),
    ("soderberg_fatigue_safety_factor", &["fatigue"]),
    ("solid_shaft_torsion", &["profile"]),
    ("spur_gear_contact_ratio", &["gears"]),
    ("statically_indeterminate_shaft", &["shaft"]),
    ("stepped_shaft_twist", &["shaft"]),
    ("strain_rosette", &["mohr"]),
    ("t_section_properties", &["section"]),
    ("thick_walled_cylinder", &["profile"]),
    ("thin_walled_pressure_vessel", &["element"]),
    ("truss_member_forces", &["truss"]),
    ("two_cable_equilibrium", &["fbd"]),
    ("two_rotor_torsional_vibration", &["mode"]),
    ("wahl_factor", &["field"]),
    ("yield_criteria_safety_factor", &["envelope"]),
];

/// Published kinds the generic engine's figures already draw (a verdict sweep), checked by kind only.
const GENERIC: &[(&str, &str)] = &[("johnson_column", "decision"), ("power_screw_torque", "decision")];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("axial_bar_segments", "loads", "-10, 40"),
    ("continuous_beam_three_moment", "w", "10, 20"),
    ("composite_section_properties", "rects", "200 20 0 10; 20 300 0 170; 100 20 0 330"),
    ("polygon_section_properties", "vertices", "0 0; 100 0; 100 200; 0 120"),
    ("eccentric_bolt_group", "bolts", "0 0; 0 100; 100 0; 100 100; 50 150"),
    ("mdof_spring_mass_chain", "masses", "2, 3"),
    ("rainflow_counting", "history", "0, 7, -3, 4, -6, 6, -4, 1, 0"),
    ("resultant_of_concurrent_forces", "theta", "0, 120"),
    ("simply_supported_beam_point_loads", "P", "10, 30"),
    ("stepped_shaft_twist", "diameters", "40, 25"),
    ("truss_member_forces", "loads", "3 4 -10"),
    ("dunkerley_critical_speed", "positions", "0.3"),
    ("rayleigh_critical_speed", "positions", "0.3"),
    ("shaft_whirling_speeds", "positions", "0.3"),
    ("planetary_gear_train", "Nr", "100"),
    ("spur_gear_contact_ratio", "N1", "24"),
    // stresses written as 5e+07 Pa, which scaling the digits would break
    ("asme_elliptic_fatigue_safety_factor", "sa", "130000000 Pa"),
    ("gerber_fatigue_safety_factor", "sa", "150000000 Pa"),
    ("goodman_fatigue_safety_factor", "sa", "150000000 Pa"),
    ("soderberg_fatigue_safety_factor", "sa", "80000000 Pa"),
    ("brittle_failure_mohr", "sA", "120000000 Pa"),
    ("principal_stresses_2d", "txy", "20000000 Pa"),
    ("principal_stresses_3d", "sy", "40000000 Pa"),
    ("yield_criteria_safety_factor", "sx", "150000000 Pa"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

/// No public-text slips in a figure's words: no first person plural, no em dash.
fn check_words(f: &Figure) {
    let text = format!("{} {}", f.title, f.caption).to_lowercase();
    for w in [" we ", " our ", " us ", "—"] {
        assert!(!text.contains(w), "{}: '{w}' in its words", f.slug);
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                    check_words(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    for (key, kind) in GENERIC {
        let figs = figures(p, key, &example(key), None, None);
        if !figs.iter().any(|f| f.kind == *kind) {
            missing.push(format!("{key}: {kind} (generic)"));
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

fn given(pairs: &[(&str, &str)]) -> Vec<(String, Input)> {
    pairs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect()
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    let p = pack();
    // a linkage that cannot be assembled: refused by the model, so no figure at all
    let g = given(&[("a", "2 m"), ("b", "1 m"), ("c", "1 m"), ("d", "10 m"), ("theta2", "0.5 rad"), ("branch", "open")]);
    let a = ax_engine::solve(p, "four_bar_linkage_position", &g, None);
    assert!(!a.solved(), "an impossible linkage was solved");
    assert!(figures(p, "four_bar_linkage_position", &g, None, None).is_empty());
    // a load placed beyond the span: refused, nothing drawn
    let g = given(&[("L", "6 m"), ("P", "20000 N"), ("a", "9 m"), ("E", "2e11 Pa"), ("I", "8e-5 m^4")]);
    let a = ax_engine::solve(p, "simply_supported_beam_point_load", &g, None);
    let figs = figures(p, "simply_supported_beam_point_load", &g, None, None);
    if !a.solved() {
        assert!(figs.is_empty());
    }
    // a cantilever loaded away from its tip: the reciprocal shape is not drawn, the diagrams are
    let g = given(&[("L", "2 m"), ("P", "5000 N"), ("E", "2e11 Pa"), ("I", "8e-6 m^4"), ("a", "1 m")]);
    let figs = figures(p, "cantilever_point_load", &g, None, None);
    assert!(figs.iter().all(|f| f.kind != "beam"));
    assert!(figs.iter().any(|f| f.kind == "beam-2"));
}

/// When FIG_DUMP names a folder, write every own figure of the worked examples there (for looking at them).
#[test]
fn dump_for_inspection() {
    let Ok(dir) = std::env::var("FIG_DUMP") else { return };
    let p = pack();
    std::fs::create_dir_all(&dir).unwrap();
    for (key, _) in KINDS {
        for f in figures(p, key, &example(key), None, None).iter().filter(|f| !f.slug.contains("-live-")) {
            std::fs::write(format!("{dir}/{}.svg", f.slug), &f.svg).unwrap();
        }
    }
}
