//! mechanical_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/mechanical_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "angle_section_properties",
    "asme_elliptic_fatigue_safety_factor",
    "axial_bar_segments",
    "bar_fixed_at_both_ends",
    "beam_natural_frequency",
    "bearing_catalog_rating",
    "bearing_life_reliability",
    "belt_drive_power",
    "belt_friction_capstan",
    "bolt_preload_torque",
    "brittle_failure_mohr",
    "cantilever_point_load",
    "cantilever_triangular_load",
    "cantilever_udl",
    "channel_section_properties",
    "circular_section_properties",
    "combined_axial_and_bending_stress",
    "composite_section_properties",
    "continuous_beam_three_moment",
    "damped_sdof_parameters",
    "disc_clutch_torque",
    "dunkerley_critical_speed",
    "eccentric_bolt_group",
    "euler_buckling",
    "eurocode_fatigue_damage",
    "fatigue_life_mean_stress_correction",
    "fixed_fixed_beam_point_load",
    "fixed_fixed_beam_udl",
    "forced_vibration_amplitude",
    "four_bar_linkage_position",
    "generalized_hookes_law",
    "gerber_fatigue_safety_factor",
    "goodman_fatigue_safety_factor",
    "helical_compression_spring",
    "hollow_circular_section_properties",
    "hollow_rectangular_section_properties",
    "hollow_shaft_torsion",
    "i_section_properties",
    "johnson_column",
    "lewis_gear_bending_stress",
    "marin_endurance_limit",
    "mdof_spring_mass_chain",
    "miner_damage_sn_curve",
    "moment_of_a_force",
    "plane_stress_transformation",
    "planetary_gear_train",
    "polygon_section_properties",
    "power_screw_torque",
    "principal_stresses_2d",
    "principal_stresses_3d",
    "propped_cantilever_point_load",
    "propped_cantilever_udl",
    "rainflow_counting",
    "rainflow_fatigue_damage",
    "rayleigh_critical_speed",
    "rectangular_section_properties",
    "resultant_of_concurrent_forces",
    "rotating_disk_stresses",
    "rotating_unbalance_response",
    "sdof_free_response",
    "sdof_natural_frequency",
    "secant_column",
    "shaft_critical_speed_single_disk",
    "shaft_diameter_de_goodman",
    "shaft_fatigue_safety_factor",
    "shaft_power_transmission",
    "shaft_whirling_speeds",
    "shrink_fit_pressure",
    "simply_supported_beam_point_load",
    "simply_supported_beam_point_loads",
    "simply_supported_beam_udl",
    "slider_crank_kinematics",
    "sn_fatigue_life",
    "sn_fatigue_strength",
    "soderberg_fatigue_safety_factor",
    "solid_shaft_torsion",
    "spur_gear_contact_ratio",
    "statically_indeterminate_shaft",
    "stepped_shaft_twist",
    "strain_rosette",
    "t_section_properties",
    "thick_walled_cylinder",
    "thin_walled_pressure_vessel",
    "truss_member_forces",
    "two_cable_equilibrium",
    "two_rotor_torsional_vibration",
    "vibration_isolator_design",
    "vibration_transmissibility",
    "wahl_factor",
    "yield_criteria_safety_factor",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "angle_section_properties" => {
            let r = f_angle_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_37(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "asme_elliptic_fatigue_safety_factor" => {
            let r = f_asme_elliptic(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_81(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "axial_bar_segments" => {
            let r = f_axial_bar(match v.get("lengths") { Some(x) => py::from_val(x), None => { return None } }, match v.get("areas") { Some(x) => py::from_val(x), None => { return None } }, match v.get("loads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_142(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bar_fixed_at_both_ends" => {
            let r = f_bar_fixed_ends(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beam_natural_frequency" => {
            let r = f_beam_frequency(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_236(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bearing_catalog_rating" => {
            let r = f_bearing_rating(match v.get("FD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("af") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("LR") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000000.0)) })().ok()? } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bearing_life_reliability" => {
            let r = f_bearing_life(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LR") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000000.0)) })().ok()? } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_346(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "belt_drive_power" => {
            let r = f_belt_power(match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_395(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "belt_friction_capstan" => {
            let r = f_capstan(match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_442(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bolt_preload_torque" => {
            let r = f_bolt_torque(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_491(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "brittle_failure_mohr" => {
            let r = f_brittle_mohr(match v.get("sA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Suc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_546(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cantilever_point_load" => {
            let r = f_cantilever_point_load(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_588(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cantilever_triangular_load" => {
            let r = f_cantilever_triangular(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_625(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cantilever_udl" => {
            let r = f_cantilever_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_662(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "channel_section_properties" => {
            let r = f_channel_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circular_section_properties" => {
            let r = f_circle_section(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_756(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_axial_and_bending_stress" => {
            let r = f_combined_rect(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("My") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_801(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "composite_section_properties" => {
            let r = f_composite_rects(match v.get("rects") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_884(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "continuous_beam_three_moment" => {
            let r = f_continuous_beam(match v.get("spans") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_965(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "damped_sdof_parameters" => {
            let r = f_damped_sdof(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1012(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "disc_clutch_torque" => {
            let r = f_disc_clutch(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pairs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1056(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dunkerley_critical_speed" => {
            let r = f_dunkerley_shaft(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1110(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eccentric_bolt_group" => {
            let r = f_bolt_group(match v.get("bolts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1171(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "euler_buckling" => {
            let r = f_euler_column(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eurocode_fatigue_damage" => {
            let r = f_eurocode_damage(match v.get("ranges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dsc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1271(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fatigue_life_mean_stress_correction" => {
            let r = f_mean_stress_life(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1327(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fixed_fixed_beam_point_load" => {
            let r = f_ff_point(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1368(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fixed_fixed_beam_udl" => {
            let r = f_ff_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forced_vibration_amplitude" => {
            let r = f_forced_response(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1453(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "four_bar_linkage_position" => {
            let r = f_four_bar(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("branch") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1505(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "generalized_hookes_law" => {
            let r = f_hooke_3d(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tyz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tzx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1550(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gerber_fatigue_safety_factor" => {
            let r = f_gerber(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1596(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "goodman_fatigue_safety_factor" => {
            let r = f_goodman(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1643(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "helical_compression_spring" => {
            let r = f_helical_spring(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1682(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hollow_circular_section_properties" => {
            let r = f_tube_section(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hollow_rectangular_section_properties" => {
            let r = f_box_section(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1766(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hollow_shaft_torsion" => {
            let r = s_1790(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1811(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "i_section_properties" => {
            let r = f_i_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1868(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "johnson_column" => {
            let r = f_johnson_column(match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1922(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lewis_gear_bending_stress" => {
            let r = f_lewis_stress(match v.get("Wt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kv") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1964(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "marin_endurance_limit" => {
            let r = f_marin(match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface") { Some(x) => py::from_val(x), None => { return None } }, match v.get("load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.5)) })().ok()? } }, match v.get("kd") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("kf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2026(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mdof_spring_mass_chain" => {
            let r = f_mdof_chain(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("springs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2135(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "miner_damage_sn_curve" => {
            let r = f_miner_linear(match v.get("ranges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(2000000.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2178(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moment_of_a_force" => {
            let r = f_moment_of_force(match v.get("Fx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Fy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("y0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2217(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_stress_transformation" => {
            let r = f_plane_transform(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2256(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "planetary_gear_train" => {
            let r = f_planetary(match v.get("Ns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ws") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("wr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("wc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2312(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_section_properties" => {
            let r = f_polygon_section(match v.get("vertices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2429(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_screw_torque" => {
            let r = f_power_screw(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("fc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("dc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "principal_stresses_2d" => {
            let r = f_principal_2d(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2526(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "principal_stresses_3d" => {
            let r = f_principal_3d(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tyz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tzx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2589(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "propped_cantilever_point_load" => {
            let r = f_pc_point(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2629(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "propped_cantilever_udl" => {
            let r = f_pc_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2666(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rainflow_counting" => {
            let r = f_rainflow_count(match v.get("history") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rainflow_fatigue_damage" => {
            let r = f_rainflow_damage(match v.get("history") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(2000000.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2794(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rayleigh_critical_speed" => {
            let r = f_rayleigh_shaft(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2855(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangular_section_properties" => {
            let r = f_rect_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "resultant_of_concurrent_forces" => {
            let r = f_resultant_forces(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2942(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotating_disk_stresses" => {
            let r = f_rotating_disk(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ri") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2992(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotating_unbalance_response" => {
            let r = f_unbalance(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3038(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sdof_free_response" => {
            let r = f_sdof_free(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3089(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sdof_natural_frequency" => {
            let r = f_sdof_natural(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3126(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "secant_column" => {
            let r = f_secant_column(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3170(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_critical_speed_single_disk" => {
            let r = f_single_disk(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3210(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_diameter_de_goodman" => {
            let r = f_shaft_de_goodman_d(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kfs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3264(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_fatigue_safety_factor" => {
            let r = f_shaft_de_goodman_n(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kfs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3322(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_power_transmission" => {
            let r = f_shaft_power(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_allow") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_whirling_speeds" => {
            let r = f_exact_whirl(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3454(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shrink_fit_pressure" => {
            let r = f_shrink_fit(match v.get("delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ri") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3499(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simply_supported_beam_point_load" => {
            let r = f_ss_point_load(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3542(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simply_supported_beam_point_loads" => {
            let r = f_ss_point_loads(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3600(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simply_supported_beam_udl" => {
            let r = f_ss_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3637(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "slider_crank_kinematics" => {
            let r = f_slider_crank(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3681(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sn_fatigue_life" => {
            let r = f_sn_life(match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3727(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sn_fatigue_strength" => {
            let r = f_sn_strength(match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3773(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "soderberg_fatigue_safety_factor" => {
            let r = f_soderberg(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solid_shaft_torsion" => {
            let r = s_3841(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3862(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spur_gear_contact_ratio" => {
            let r = f_spur_contact(match v.get("N1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3916(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "statically_indeterminate_shaft" => {
            let r = f_fixed_shaft(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3955(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stepped_shaft_twist" => {
            let r = f_stepped_shaft(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lengths") { Some(x) => py::from_val(x), None => { return None } }, match v.get("diameters") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3998(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "strain_rosette" => {
            let r = f_rosette(match v.get("ea") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ec") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("layout") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "t_section_properties" => {
            let r = f_t_section(match v.get("bf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thick_walled_cylinder" => {
            let r = f_lame(match v.get("pi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4165(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thin_walled_pressure_vessel" => {
            let r = f_thin_vessel(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("shape") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4208(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "truss_member_forces" => {
            let r = f_truss(match v.get("nodes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("members") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("roller") { Some(x) => py::from_val(x), None => { return None } }, match v.get("loads") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4342(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_cable_equilibrium" => {
            let r = f_two_cable_tension(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4384(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_rotor_torsional_vibration" => {
            let r = f_two_rotor(match v.get("J1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4424(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vibration_isolator_design" => {
            let r = f_isolator_design(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4464(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vibration_transmissibility" => {
            let r = f_transmissibility(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4510(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wahl_factor" => {
            let r = f_wahl_factor(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4552(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "yield_criteria_safety_factor" => {
            let r = f_yield_safety(match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("sz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("txy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("tyz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("tzx") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4623(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MARIN_LOAD() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("bending"), Py::Float(1.0)), (Py::str("axial"), Py::Float(0.85)), (Py::str("torsion"), Py::Float(0.59))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MARIN_SURFACE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("ground"), Py::tuple(vec![Py::Float(1.58), Py::Float(-0.085)])), (Py::str("machined"), Py::tuple(vec![Py::Float(4.51), Py::Float(-0.265)])), (Py::str("cold-drawn"), Py::tuple(vec![Py::Float(4.51), Py::Float(-0.265)])), (Py::str("hot-rolled"), Py::tuple(vec![Py::Float(57.7), Py::Float(-0.718)])), (Py::str("as-forged"), Py::tuple(vec![Py::Float(272.0), Py::Float(-0.995)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RPM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.549296585513721)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__WEIBULL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("x0"), Py::Float(0.02)), (Py::str("theta"), Py::Float(4.459)), (Py::str("b"), Py::Float(1.483))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta_L(mut v_ends: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    if ({ let t213 = v_ends.clone(); Py::Bool({ let t214 = Py::str("simply-supported"); if py::eq(&t213, &t214) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?);
    }
    if ({ let t215 = v_ends.clone(); Py::Bool({ let t216 = Py::str("cantilever"); if py::eq(&t215, &t216) { true } else { false } }) }).truthy() {
        let t217 = (Py::tuple(vec![{ let v_ends = v_ends.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::math1("cos", &v_x.clone())?, &py::truediv(&Py::Int(1i64), &py::math1("cosh", &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::truediv(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?])).items()?;
        if t217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t217[0].clone();
        v_c = t217[1].clone();
    } else {
        if ({ let t218 = v_ends.clone(); Py::Bool({ let t219 = Py::tuple(vec![Py::str("fixed-fixed"), Py::str("free-free")]); if py::contains(&t219, &t218)? { true } else { false } }) }).truthy() {
            let t220 = (Py::tuple(vec![{ let v_c = v_c.clone(); let v_ends = v_ends.clone(); let v_f = v_f.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("cos", &v_x.clone())?, &py::truediv(&Py::Int(1i64), &py::math1("cosh", &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::truediv(&py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?])).items()?;
            if t220.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t220[0].clone();
            v_c = t220[1].clone();
        } else {
            let t221 = (Py::tuple(vec![{ let v_c = v_c.clone(); let v_ends = v_ends.clone(); let v_f = v_f.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("sin", &v_x.clone())?, &py::mul(&py::math1("cos", &v_x.clone())?, &py::math1("tanh", &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::truediv(&py::mul(&py::add(&py::mul(&Py::Int(4i64), &v_n.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(4i64))?])).items()?;
            if t221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t221[0].clone();
            v_c = t221[1].clone();
        }
    }
    let t222 = (Py::tuple(vec![py::sub(&v_c.clone(), &Py::Float(0.5))?, py::add(&v_c.clone(), &Py::Float(0.5))?])).items()?;
    if t222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t222[0].clone();
    v_hi = t222[1].clone();
    if ({ let t223 = py::mul(&py::call(&v_f.clone(), &[v_lo.clone()])?, &py::call(&v_f.clone(), &[v_hi.clone()])?)?; Py::Bool({ let t224 = Py::Int(0i64); if py::gt(&t223, &t224)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no root bracketed"))));
    }
    for t225 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t225;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t226 = py::mul(&py::call(&v_f.clone(), &[v_lo.clone()])?, &py::call(&v_f.clone(), &[v_mid.clone()])?)?; Py::Bool({ let t227 = Py::Int(0i64); if py::le(&t226, &t227)? { true } else { false } }) }).truthy() {
            v_hi = v_mid.clone();
        } else {
            v_lo = v_mid.clone();
        }
    }
    return Ok(py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chain_K(mut v_springs: Py, mut v_n: Py, mut v_ends: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_i: Py = Py::None;
    v_K = { let mut out = vec![]; for t2063 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t2063;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t2064 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2064;
        let t2065 = py::getitem(&v_K.clone(), &v_i.clone())?; let t2066 = v_i.clone();
        py::setitem(&t2065, &t2066, py::iadd(&py::getitem(&t2065, &t2066)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
        if ({ let t2067 = v_i.clone(); Py::Bool({ let t2068 = Py::Int(0i64); if py::gt(&t2067, &t2068)? { true } else { false } }) }).truthy() {
            let t2069 = py::getitem(&v_K.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2070 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t2069, &t2070, py::iadd(&py::getitem(&t2069, &t2070)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
            let t2071 = py::getitem(&v_K.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2072 = v_i.clone();
            py::setitem(&t2071, &t2072, py::sub(&py::getitem(&t2071, &t2072)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
            let t2073 = py::getitem(&v_K.clone(), &v_i.clone())?; let t2074 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t2073, &t2074, py::sub(&py::getitem(&t2073, &t2074)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
        }
    }
    if ({ let t2075 = v_ends.clone(); Py::Bool({ let t2076 = Py::str("fixed-fixed"); if py::eq(&t2075, &t2076) { true } else { false } }) }).truthy() {
        let t2077 = py::getitem(&v_K.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?; let t2078 = py::sub(&v_n.clone(), &Py::Int(1i64))?;
        py::setitem(&t2077, &t2078, py::iadd(&py::getitem(&t2077, &t2078)?, &py::getitem(&v_springs.clone(), &v_n.clone())?)?)?;
    }
    return Ok(v_K.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cosd(mut v_a: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = py::modulo(&v_a.clone(), &Py::Float(360.0))?;
    return Ok(py::method(&Py::dict(vec![(Py::Float(0.0), Py::Float(1.0)), (Py::Float(90.0), Py::Float(0.0)), (Py::Float(180.0), py::neg(&Py::Float(1.0))?), (Py::Float(270.0), Py::Float(0.0))]), "get", &[v_r.clone(), py::math1("cos", &f__deg(v_a.clone())?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__counts_check(mut v_ranges: Py, mut v_counts: Py) -> R {
    let mut v_v: Py = Py::None;
    if ({ let t2159 = py::len(&v_ranges.clone())?; Py::Bool({ let t2160 = py::len(&v_counts.clone())?; if py::ne(&t2159, &t2160) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("ranges and counts must have the same number of entries"))));
    }
    for t2161 in py::iter(&(v_ranges.clone()))? {
        v_v = t2161;
        let _ = f__pos(Py::str("every stress range"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t2167 in py::iter(&(v_counts.clone()))? {
        v_v = t2167;
        let _ = f__pos(Py::str("every cycle count"), Py::tuple(vec![v_v.clone()]))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__de_check(mut v_Ma: Py, mut v_Mm: Py, mut v_Ta: Py, mut v_Tm: Py, mut v_Kf: Py, mut v_Kfs: Py, mut v_Se: Py, mut v_Sut: Py) -> R {
    let _ = f__nonneg(Py::str("the moments and torques"), Py::tuple(vec![v_Ma.clone(), v_Mm.clone(), v_Ta.clone(), v_Tm.clone()]))?;
    if ({ let t3302 = { let t3298 = v_Ma.clone(); Py::Bool({ let t3299 = Py::Int(0i64); if py::eq(&t3298, &t3299) { true } else { false } }) }; if !t3302.truthy() { t3302 } else { { let t3300 = v_Ta.clone(); Py::Bool({ let t3301 = Py::Int(0i64); if py::eq(&t3300, &t3301) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is no alternating load: a static check, not a fatigue one"))));
    }
    if ({ let t3307 = { let t3303 = v_Kf.clone(); Py::Bool({ let t3304 = Py::Int(1i64); if py::lt(&t3303, &t3304)? { true } else { false } }) }; if t3307.truthy() { t3307 } else { { let t3305 = v_Kfs.clone(); Py::Bool({ let t3306 = Py::Int(1i64); if py::lt(&t3305, &t3306)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("fatigue stress-concentration factors are >= 1"))));
    }
    let _ = f__pos(Py::str("Se and Sut"), Py::tuple(vec![v_Se.clone(), v_Sut.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__de_stresses(mut v_d: Py, mut v_Ma: Py, mut v_Mm: Py, mut v_Ta: Py, mut v_Tm: Py, mut v_Kf: Py, mut v_Kfs: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_sa: Py = Py::None;
    let mut v_sm: Py = Py::None;
    v_c = py::truediv(&Py::Int(1i64), &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?)?;
    v_sa = py::math1("sqrt", &py::add(&py::pow(&py::mul(&py::mul(&py::mul(&Py::Int(32i64), &v_Kf.clone())?, &v_Ma.clone())?, &v_c.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(3i64), &py::pow(&py::mul(&py::mul(&py::mul(&Py::Int(16i64), &v_Kfs.clone())?, &v_Ta.clone())?, &v_c.clone())?, &Py::Int(2i64))?)?)?)?;
    v_sm = py::math1("sqrt", &py::add(&py::pow(&py::mul(&py::mul(&py::mul(&Py::Int(32i64), &v_Kf.clone())?, &v_Mm.clone())?, &v_c.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(3i64), &py::pow(&py::mul(&py::mul(&py::mul(&Py::Int(16i64), &v_Kfs.clone())?, &v_Tm.clone())?, &v_c.clone())?, &Py::Int(2i64))?)?)?)?;
    return Ok(Py::tuple(vec![v_sa.clone(), v_sm.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__defl_check(mut v_E: Py, mut v_I: Py) -> R {
    let _ = f__pos(Py::str("E and I"), Py::tuple(vec![v_E.clone(), v_I.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__deg(mut v_x: Py) -> R {
    return Ok(py::truediv(&py::mul(&v_x.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Float(180.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fat_check(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_S: Py) -> R {
    let _ = f__pos(Py::str("the alternating stress, Se and the strength"), Py::tuple(vec![v_sa.clone(), v_Se.clone(), v_S.clone()]))?;
    let _ = f__nonneg(Py::str("the mean stress (compressive means are not covered)"), Py::tuple(vec![v_sm.clone()]))?;
    if ({ let t3807 = v_sm.clone(); Py::Bool({ let t3808 = v_S.clone(); if py::ge(&t3807, &t3808)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mean stress must be below the strength"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4581 in py::iter(&(v_vals.clone()))? {
        v_v = t4581;
        if (py::not(&{ let t4582 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4582.truthy() { t4582 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__flex(mut v_positions: Py, mut v_L: Py, mut v_EI: Py) -> R {
    return Ok({ let mut out = vec![]; for t3401 in py::iter(&(v_positions.clone()))? { let mut v_xi: Py; 
v_xi = t3401;
out.push({ let mut out = vec![]; for t3402 in py::iter(&(v_positions.clone()))? { let mut v_aj: Py; 
v_aj = t3402;
out.push(py::truediv(&f__ss_influence(v_xi.clone(), v_aj.clone(), v_L.clone())?, &v_EI.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss_solve(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t4304 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t4305 = (t4304).items()?;
if t4305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t4305[0].clone();
v_row = t4305[1].clone();
out.push(py::add(&py::slice(&v_row.clone(), None, None, None)?, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t4308 = py::minmax(&[{ let mut out = vec![]; for t4306 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t4306;
for t4307 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t4307;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?; if t4308.truthy() { t4308 } else { Py::Float(1.0) } };
    for t4309 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_col = t4309;
        v_piv = py::minmax_key(&[Py::list((py::range_iter(&[(v_col.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_col = v_col.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t4310 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_piv.clone())?, &v_col.clone())?)?; Py::Bool({ let t4311 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t4310, &t4311)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the structure is unstable or not statically determinate (singular equilibrium equations)"))));
        }
        let t4312 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_piv.clone())?, py::getitem(&v_M.clone(), &v_col.clone())?])).items()?;
        if t4312.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_col.clone(), t4312[0].clone())?;
        py::setitem(&v_M.clone(), &v_piv.clone(), t4312[1].clone())?;
        for t4313 in py::range_iter(&[(py::add(&v_col.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t4313;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_col.clone())?, &v_col.clone())?)?;
            if (v_f.clone()).truthy() {
                for t4314 in py::range_iter(&[(v_col.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_c = t4314;
                    let t4315 = py::getitem(&v_M.clone(), &v_r.clone())?; let t4316 = v_c.clone();
                    py::setitem(&t4315, &t4316, py::sub(&py::getitem(&t4315, &t4316)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_col.clone())?, &v_c.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t4317 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_r = t4317;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::fsum(&{ let mut out = vec![]; for t4318 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t4318;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&v_x.clone(), &v_c.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hist_check(mut v_history: Py) -> R {
    if ({ let t2750 = py::len(&v_history.clone())?; Py::Bool({ let t2751 = Py::Int(3i64); if py::lt(&t2750, &t2751)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least 3 points of the load history"))));
    }
    let _ = f__fin(Py::str("every point"), { let mut v = vec![]; v.extend((v_history.clone()).items()?); Py::tuple(v) })?;
    if ({ let t2754 = py::minmax(&[v_history.clone()], true)?; Py::Bool({ let t2755 = py::minmax(&[v_history.clone()], false)?; if py::eq(&t2754, &t2755) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the history is constant: there are no cycles"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hollow_shaft(mut v_T: Py, mut v_D: Py, mut v_d: Py, mut v_L: Py, mut v_G: Py) -> R {
    let _ = f__pos(Py::str("the inner diameter d (use the solid shaft for d = 0)"), Py::tuple(vec![v_d.clone()]))?;
    return Ok(f_shaft_torsion(v_T.clone(), v_D.clone(), v_L.clone(), v_G.clone(), v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__i_check(mut v_b: Py, mut v_h: Py, mut v_tf: Py, mut v_tw: Py) -> R {
    let _ = f__pos(Py::str("b, h, tf and tw"), Py::tuple(vec![v_b.clone(), v_h.clone(), v_tf.clone(), v_tw.clone()]))?;
    if ({ let t1840 = py::mul(&Py::Int(2i64), &v_tf.clone())?; Py::Bool({ let t1841 = v_h.clone(); if py::ge(&t1840, &t1841)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the flanges (2 tf) must be thinner than the overall depth h"))));
    }
    if ({ let t1842 = v_tw.clone(); Py::Bool({ let t1843 = v_b.clone(); if py::ge(&t1842, &t1843)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the web thickness must be smaller than the flange width"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jacobi3(mut v_S: Py) -> R {
    let mut v__sweep: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_akp: Py = Py::None;
    let mut v_akq: Py = Py::None;
    let mut v_apk: Py = Py::None;
    let mut v_aqk: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nrm: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_a = { let mut out = vec![]; for t4584 in py::iter(&(v_S.clone()))? { let mut v_row: Py; 
v_row = t4584;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    for t4585 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
        v__sweep = t4585;
        v_off = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t4586 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4586;
for t4587 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t4587;
if ({ let t4588 = v_i.clone(); Py::Bool({ let t4589 = v_j.clone(); if py::ne(&t4588, &t4589) { true } else { false } }) }).truthy() { out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }}} Py::list(out) }, None)?)?;
        v_nrm = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t4590 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4590;
for t4591 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t4591;
out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }} Py::list(out) }, None)?)?;
        if ({ let t4596 = { let t4592 = v_off.clone(); Py::Bool({ let t4593 = py::mul(&Py::Float(1e-17), &v_nrm.clone())?; if py::le(&t4592, &t4593)? { true } else { false } }) }; if t4596.truthy() { t4596 } else { { let t4594 = v_off.clone(); Py::Bool({ let t4595 = Py::Int(0i64); if py::eq(&t4594, &t4595) { true } else { false } }) } } }).truthy() {
            break;
        }
        for t4597 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])])))? {
            let t4606 = (t4597).items()?;
            if t4606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p = t4606[0].clone();
            v_q = t4606[1].clone();
            if ({ let t4598 = py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?; Py::Bool({ let t4599 = Py::Int(0i64); if py::eq(&t4598, &t4599) { true } else { false } }) }).truthy() {
                continue;
            }
            v_th = py::truediv(&py::sub(&py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_q.clone())?, &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_p.clone())?)?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?)?)?;
            v_t = py::truediv(&py::copysign(&Py::Float(1.0), &v_th.clone())?, &py::add(&py::abs(&v_th.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_th.clone(), &v_th.clone())?, &Py::Int(1i64))?)?)?)?;
            v_c = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::mul(&v_t.clone(), &v_t.clone())?, &Py::Int(1i64))?)?)?;
            v_s = py::mul(&v_t.clone(), &v_c.clone())?;
            for t4600 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
                v_k = t4600;
                let t4601 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                if t4601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_akp = t4601[0].clone();
                v_akq = t4601[1].clone();
                let t4602 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_akp.clone())?, &py::mul(&v_s.clone(), &v_akq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_akp.clone())?, &py::mul(&v_c.clone(), &v_akq.clone())?)?])).items()?;
                if t4602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone(), t4602[0].clone())?;
                py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone(), t4602[1].clone())?;
            }
            for t4603 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
                v_k = t4603;
                let t4604 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone())?])).items()?;
                if t4604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_apk = t4604[0].clone();
                v_aqk = t4604[1].clone();
                let t4605 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_apk.clone())?, &py::mul(&v_s.clone(), &v_aqk.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_apk.clone())?, &py::mul(&v_c.clone(), &v_aqk.clone())?)?])).items()?;
                if t4605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone(), t4605[0].clone())?;
                py::setitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone(), t4605[1].clone())?;
            }
        }
    }
    return Ok(py::sorted(&{ let mut out = vec![]; for t4607 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t4607;
out.push(py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jacobi_sym(mut v_A: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v__sweep: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_akp: Py = Py::None;
    let mut v_akq: Py = Py::None;
    let mut v_apk: Py = Py::None;
    let mut v_aqk: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nrm: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_vkp: Py = Py::None;
    let mut v_vkq: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_a = { let mut out = vec![]; for t3408 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t3408;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    v_V = { let mut out = vec![]; for t3409 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3409;
out.push({ let mut out = vec![]; for t3410 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3410;
out.push((if ({ let t3411 = v_i.clone(); Py::Bool({ let t3412 = v_j.clone(); if py::eq(&t3411, &t3412) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    for t3413 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__sweep = t3413;
        v_off = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t3414 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3414;
for t3415 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3415;
if ({ let t3416 = v_i.clone(); Py::Bool({ let t3417 = v_j.clone(); if py::ne(&t3416, &t3417) { true } else { false } }) }).truthy() { out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }}} Py::list(out) }, None)?)?;
        v_nrm = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t3418 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3418;
for t3419 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3419;
out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }} Py::list(out) }, None)?)?;
        if ({ let t3424 = { let t3420 = v_off.clone(); Py::Bool({ let t3421 = py::mul(&Py::Float(1e-17), &v_nrm.clone())?; if py::le(&t3420, &t3421)? { true } else { false } }) }; if t3424.truthy() { t3424 } else { { let t3422 = v_off.clone(); Py::Bool({ let t3423 = Py::Int(0i64); if py::eq(&t3422, &t3423) { true } else { false } }) } } }).truthy() {
            break;
        }
        for t3425 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_p = t3425;
            for t3426 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_q = t3426;
                if ({ let t3427 = py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?; Py::Bool({ let t3428 = Py::Int(0i64); if py::eq(&t3427, &t3428) { true } else { false } }) }).truthy() {
                    continue;
                }
                v_th = py::truediv(&py::sub(&py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_q.clone())?, &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_p.clone())?)?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?)?)?;
                v_t = py::truediv(&py::copysign(&Py::Float(1.0), &v_th.clone())?, &py::add(&py::abs(&v_th.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_th.clone(), &v_th.clone())?, &Py::Int(1i64))?)?)?)?;
                v_c = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::mul(&v_t.clone(), &v_t.clone())?, &Py::Int(1i64))?)?)?;
                v_s = py::mul(&v_t.clone(), &v_c.clone())?;
                for t3429 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t3429;
                    let t3430 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                    if t3430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_akp = t3430[0].clone();
                    v_akq = t3430[1].clone();
                    let t3431 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_akp.clone())?, &py::mul(&v_s.clone(), &v_akq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_akp.clone())?, &py::mul(&v_c.clone(), &v_akq.clone())?)?])).items()?;
                    if t3431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone(), t3431[0].clone())?;
                    py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone(), t3431[1].clone())?;
                }
                for t3432 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t3432;
                    let t3433 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone())?])).items()?;
                    if t3433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_apk = t3433[0].clone();
                    v_aqk = t3433[1].clone();
                    let t3434 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_apk.clone())?, &py::mul(&v_s.clone(), &v_aqk.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_apk.clone())?, &py::mul(&v_c.clone(), &v_aqk.clone())?)?])).items()?;
                    if t3434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone(), t3434[0].clone())?;
                    py::setitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone(), t3434[1].clone())?;
                }
                for t3435 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t3435;
                    let t3436 = (Py::tuple(vec![py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                    if t3436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_vkp = t3436[0].clone();
                    v_vkq = t3436[1].clone();
                    let t3437 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_vkp.clone())?, &py::mul(&v_s.clone(), &v_vkq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_vkp.clone())?, &py::mul(&v_c.clone(), &v_vkq.clone())?)?])).items()?;
                    if t3437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_p.clone(), t3437[0].clone())?;
                    py::setitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_q.clone(), t3437[1].clone())?;
                }
            }
        }
    }
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_A = v_A.clone(); let v_V = v_V.clone(); let v__sweep = v__sweep.clone(); let v_a = v_a.clone(); let v_akp = v_akp.clone(); let v_akq = v_akq.clone(); let v_apk = v_apk.clone(); let v_aqk = v_aqk.clone(); let v_c = v_c.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_nrm = v_nrm.clone(); let v_off = v_off.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); let v_th = v_th.clone(); let v_vkp = v_vkp.clone(); let v_vkq = v_vkq.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_i.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t3438 in py::iter(&(v_order.clone()))? { let mut v_i: Py; 
v_i = t3438;
out.push(py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3439 in py::iter(&(v_order.clone()))? { let mut v_i: Py; 
v_i = t3439;
out.push({ let mut out = vec![]; for t3440 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3440;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_r.clone())?, &v_i.clone())?); } Py::list(out) }); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__k_factor(mut v_ends: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    if ({ let t1897 = v_ends.clone(); Py::Bool({ let t1898 = Py::str("pinned-pinned"); if py::eq(&t1897, &t1898) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t1899 = v_ends.clone(); Py::Bool({ let t1900 = Py::str("fixed-free"); if py::eq(&t1899, &t1900) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(2.0));
    }
    if ({ let t1901 = v_ends.clone(); Py::Bool({ let t1902 = Py::str("fixed-fixed"); if py::eq(&t1901, &t1902) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.5));
    }
    let t1903 = (Py::tuple(vec![py::add(&Py::Float(std::f64::consts::PI), &Py::Float(1e-09))?, py::sub(&py::mul(&Py::Float(1.5), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-09))?])).items()?;
    if t1903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1903[0].clone();
    v_hi = t1903[1].clone();
    for t1904 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t1904;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t1905 = py::sub(&py::math1("tan", &v_mid.clone())?, &v_mid.clone())?; Py::Bool({ let t1906 = Py::Int(0i64); if py::gt(&t1905, &t1906)? { true } else { false } }) }).truthy() {
            v_hi = v_mid.clone();
        } else {
            v_lo = v_mid.clone();
        }
    }
    return Ok(py::truediv(&Py::Float(std::f64::consts::PI), &py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4488 in py::iter(&(v_vals.clone()))? {
        v_v = t4488;
        if (py::not(&{ let t4492 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4492.truthy() { t4492 } else { { let t4491 = py::math1("isfinite", &v_v.clone())?; if !t4491.truthy() { t4491 } else { { let t4489 = v_v.clone(); Py::Bool({ let t4490 = Py::Int(0i64); if py::ge(&t4489, &t4490)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm_mode(mut v_v: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_piv: Py = Py::None;
    v_big = py::minmax(&[{ let mut out = vec![]; for t2117 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2117;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?;
    v_piv = (match ('g2118: { let mut out = vec![]; for t2119 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2119;
if ({ let t2120 = py::abs(&v_x.clone())?; Py::Bool({ let t2121 = py::mul(&v_big.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::ge(&t2120, &t2121)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); break 'g2118 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    return Ok({ let mut out = vec![]; for t2122 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2122;
out.push(f__snap(py::truediv(&v_x.clone(), &v_piv.clone())?, Py::Float(1.0))?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t4612 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4613 = (t4612).items()?;
if t4613.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4613[0].clone();
v_v = t4613[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4576 in py::iter(&(v_vals.clone()))? {
        v_v = t4576;
        if (py::not(&{ let t4580 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4580.truthy() { t4580 } else { { let t4579 = py::math1("isfinite", &v_v.clone())?; if !t4579.truthy() { t4579 } else { { let t4577 = v_v.clone(); Py::Bool({ let t4578 = Py::Int(0i64); if py::gt(&t4577, &t4578)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__principal(mut v_Ix: Py, mut v_Iy: Py, mut v_Ixy: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let t2415 = (Py::tuple(vec![py::truediv(&py::add(&v_Ix.clone(), &v_Iy.clone())?, &Py::Int(2i64))?, py::hypot(&[py::truediv(&py::sub(&v_Ix.clone(), &v_Iy.clone())?, &Py::Int(2i64))?, v_Ixy.clone()])?])).items()?;
    if t2415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2415[0].clone();
    v_rr = t2415[1].clone();
    if ({ let t2416 = v_rr.clone(); Py::Bool({ let t2417 = py::mul(&Py::Float(1e-12), &v_c.clone())?; if py::le(&t2416, &t2417)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![v_c.clone(), v_c.clone(), Py::Float(0.0)]));
    }
    v_phi = py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&py::mul(&py::neg(&Py::Int(2i64))?, &v_Ixy.clone())?, &py::sub(&v_Ix.clone(), &v_Iy.clone())?)?)?)?;
    if ({ let t2418 = v_phi.clone(); Py::Bool({ let t2419 = py::neg(&Py::Int(90i64))?; if py::le(&t2418, &t2419)? { true } else { false } }) }).truthy() {
        v_phi = py::iadd(&v_phi, &Py::Int(180i64))?;
    }
    return Ok(Py::tuple(vec![py::add(&v_c.clone(), &v_rr.clone())?, py::sub(&v_c.clone(), &v_rr.clone())?, v_phi.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if (Py::Bool(matches!(v_x.clone(), Py::Bool(_) | Py::Str(_) | Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4614 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4614;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t4617 = { let t4615 = v_x.clone(); Py::Bool({ let t4616 = Py::Int(0i64); if py::eq(&t4615, &t4616) { true } else { false } }) }; if t4617.truthy() { t4617 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4618 = v_x.clone(); Py::Bool({ let t4619 = Py::Int(0i64); if py::eq(&t4618, &t4619) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rainflow(mut v_series: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_cyc: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let t2761 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2761.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t2761[0].clone();
    v_cyc = t2761[1].clone();
    for t2769 in py::iter(&(f__reversals(v_series.clone())?))? {
        v_p = t2769;
        let _ = py::method(&v_pts.clone(), "append", &[v_p.clone()])?;
        while ({ let t2775 = py::len(&v_pts.clone())?; Py::Bool({ let t2776 = Py::Int(3i64); if py::ge(&t2775, &t2776)? { true } else { false } }) }).truthy() {
            let t2770 = (Py::tuple(vec![py::abs(&py::sub(&py::getitem(&v_pts.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_pts.clone(), &py::neg(&Py::Int(2i64))?)?)?)?, py::abs(&py::sub(&py::getitem(&v_pts.clone(), &py::neg(&Py::Int(2i64))?)?, &py::getitem(&v_pts.clone(), &py::neg(&Py::Int(3i64))?)?)?)?])).items()?;
            if t2770.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_X = t2770[0].clone();
            v_Y = t2770[1].clone();
            if ({ let t2771 = v_X.clone(); Py::Bool({ let t2772 = v_Y.clone(); if py::lt(&t2771, &t2772)? { true } else { false } }) }).truthy() {
                break;
            }
            if ({ let t2773 = py::len(&v_pts.clone())?; Py::Bool({ let t2774 = Py::Int(3i64); if py::eq(&t2773, &t2774) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_cyc.clone(), "append", &[Py::tuple(vec![v_Y.clone(), py::truediv(&py::add(&py::getitem(&v_pts.clone(), &Py::Int(0i64))?, &py::getitem(&v_pts.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?, Py::Float(0.5)])])?;
                let _ = py::method(&v_pts.clone(), "pop", &[Py::Int(0i64)])?;
            } else {
                let _ = py::method(&v_cyc.clone(), "append", &[Py::tuple(vec![v_Y.clone(), py::truediv(&py::add(&py::getitem(&v_pts.clone(), &py::neg(&Py::Int(3i64))?)?, &py::getitem(&v_pts.clone(), &py::neg(&Py::Int(2i64))?)?)?, &Py::Int(2i64))?, Py::Float(1.0)])])?;
                v_last = py::method(&v_pts.clone(), "pop", &[])?;
                let _ = py::method(&v_pts.clone(), "pop", &[])?;
                let _ = py::method(&v_pts.clone(), "pop", &[])?;
                let _ = py::method(&v_pts.clone(), "append", &[v_last.clone()])?;
            }
        }
    }
    for t2777 in py::range_iter(&[(py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t2777;
        let _ = py::method(&v_cyc.clone(), "append", &[Py::tuple(vec![py::abs(&py::sub(&py::getitem(&v_pts.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_pts.clone(), &v_i.clone())?)?)?, py::truediv(&py::add(&py::getitem(&v_pts.clone(), &v_i.clone())?, &py::getitem(&v_pts.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?, Py::Float(0.5)])])?;
    }
    return Ok(v_cyc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rect_set(mut v_rects: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Ixy: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let mut v_yc: Py = Py::None;
    v_A = py::fsum(&{ let mut out = vec![]; for t4086 in py::iter(&(v_rects.clone()))? { let mut v__x: Py; let mut v__y: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; 
let t4087 = (t4086).items()?;
if t4087.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4087[0].clone();
v_b = t4087[1].clone();
v_h = t4087[2].clone();
v__x = t4087[3].clone();
v__y = t4087[4].clone();
out.push(py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?); } Py::list(out) })?;
    if ({ let t4088 = v_A.clone(); Py::Bool({ let t4089 = Py::Int(0i64); if py::le(&t4088, &t4089)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the section has no positive area"))));
    }
    v_xc = py::truediv(&py::fsum(&{ let mut out = vec![]; for t4090 in py::iter(&(v_rects.clone()))? { let mut v__y: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_x: Py; 
let t4091 = (t4090).items()?;
if t4091.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4091[0].clone();
v_b = t4091[1].clone();
v_h = t4091[2].clone();
v_x = t4091[3].clone();
v__y = t4091[4].clone();
out.push(py::mul(&py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?, &v_x.clone())?); } Py::list(out) })?, &v_A.clone())?;
    v_yc = py::truediv(&py::fsum(&{ let mut out = vec![]; for t4092 in py::iter(&(v_rects.clone()))? { let mut v__x: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_y: Py; 
let t4093 = (t4092).items()?;
if t4093.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4093[0].clone();
v_b = t4093[1].clone();
v_h = t4093[2].clone();
v__x = t4093[3].clone();
v_y = t4093[4].clone();
out.push(py::mul(&py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?, &v_y.clone())?); } Py::list(out) })?, &v_A.clone())?;
    v_Ix = py::fsum(&{ let mut out = vec![]; for t4094 in py::iter(&(v_rects.clone()))? { let mut v__x: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_y: Py; 
let t4095 = (t4094).items()?;
if t4095.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4095[0].clone();
v_b = t4095[1].clone();
v_h = t4095[2].clone();
v__x = t4095[3].clone();
v_y = t4095[4].clone();
out.push(py::mul(&v_s.clone(), &py::add(&py::truediv(&py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, &py::mul(&py::mul(&v_b.clone(), &v_h.clone())?, &py::pow(&py::sub(&v_y.clone(), &v_yc.clone())?, &Py::Int(2i64))?)?)?)?); } Py::list(out) })?;
    v_Iy = py::fsum(&{ let mut out = vec![]; for t4096 in py::iter(&(v_rects.clone()))? { let mut v__y: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_x: Py; 
let t4097 = (t4096).items()?;
if t4097.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4097[0].clone();
v_b = t4097[1].clone();
v_h = t4097[2].clone();
v_x = t4097[3].clone();
v__y = t4097[4].clone();
out.push(py::mul(&v_s.clone(), &py::add(&py::truediv(&py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, &py::mul(&py::mul(&v_b.clone(), &v_h.clone())?, &py::pow(&py::sub(&v_x.clone(), &v_xc.clone())?, &Py::Int(2i64))?)?)?)?); } Py::list(out) })?;
    v_Ixy = py::fsum(&{ let mut out = vec![]; for t4098 in py::iter(&(v_rects.clone()))? { let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_x: Py; let mut v_y: Py; 
let t4099 = (t4098).items()?;
if t4099.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4099[0].clone();
v_b = t4099[1].clone();
v_h = t4099[2].clone();
v_x = t4099[3].clone();
v_y = t4099[4].clone();
out.push(py::mul(&py::mul(&py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?, &py::sub(&v_x.clone(), &v_xc.clone())?)?, &py::sub(&v_y.clone(), &v_yc.clone())?)?); } Py::list(out) })?;
    return Ok(Py::tuple(vec![v_A.clone(), v_xc.clone(), v_yc.clone(), v_Ix.clone(), v_Iy.clone(), v_Ixy.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t4629 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4629.truthy() { t4629 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4630 = v_a.clone(); Py::Bool({ let t4631 = v_b.clone(); if py::eq(&t4630, &t4631) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    if ({ let t4632 = Py::Bool(matches!(v_a.clone(), Py::List(_))); if t4632.truthy() { t4632 } else { Py::Bool(matches!(v_b.clone(), Py::List(_))) } }).truthy() {
        if ({ let t4636 = py::not(&{ let t4633 = Py::Bool(matches!(v_a.clone(), Py::List(_))); if !t4633.truthy() { t4633 } else { Py::Bool(matches!(v_b.clone(), Py::List(_))) } }); if t4636.truthy() { t4636 } else { { let t4634 = py::len(&v_a.clone())?; Py::Bool({ let t4635 = py::len(&v_b.clone())?; if py::ne(&t4634, &t4635) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[{ let t4639 = { let mut out = vec![]; for t4637 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4638 = (t4637).items()?;
if t4638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4638[0].clone();
v_y = t4638[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t4639.truthy() { t4639 } else { Py::list(vec![Py::Float(0.0)]) } }], true)?);
    }
    let t4641: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4640 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4640[0].clone();
            v_b = t4640[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4641 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4642 = v_d.clone(); Py::Bool({ let t4643 = Py::Int(0i64); if py::eq(&t4642, &t4643) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t4644 = v_s.clone(); Py::Bool({ let t4645 = Py::Int(0i64); if py::gt(&t4644, &t4645)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__reversals(mut v_series: Py) -> R {
    let mut v_d_last: Py = Py::None;
    let mut v_d_next: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_x_next: Py = Py::None;
    v_out = Py::list(vec![py::getitem(&v_series.clone(), &Py::Int(0i64))?]);
    let t2762 = (Py::tuple(vec![py::getitem(&v_series.clone(), &Py::Int(1i64))?, py::sub(&py::getitem(&v_series.clone(), &Py::Int(1i64))?, &py::getitem(&v_series.clone(), &Py::Int(0i64))?)?])).items()?;
    if t2762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2762[0].clone();
    v_d_last = t2762[1].clone();
    for t2763 in py::iter(&(py::slice(&v_series.clone(), Some((Py::Int(2i64)).i()?), None, None)?))? {
        v_x_next = t2763;
        if ({ let t2764 = v_x_next.clone(); Py::Bool({ let t2765 = v_x.clone(); if py::eq(&t2764, &t2765) { true } else { false } }) }).truthy() {
            continue;
        }
        v_d_next = py::sub(&v_x_next.clone(), &v_x.clone())?;
        if ({ let t2766 = py::mul(&v_d_last.clone(), &v_d_next.clone())?; Py::Bool({ let t2767 = Py::Int(0i64); if py::lt(&t2766, &t2767)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_x.clone()])?;
        }
        let t2768 = (Py::tuple(vec![v_x_next.clone(), v_d_next.clone()])).items()?;
        if t2768.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t2768[0].clone();
        v_d_last = t2768[1].clone();
    }
    let _ = py::method(&v_out.clone(), "append", &[py::getitem(&v_series.clone(), &py::neg(&Py::Int(1i64))?)?])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__segments_cross(mut v_p1: Py, mut v_p2: Py, mut v_p3: Py, mut v_p4: Py) -> R {
    let mut v_o1: Py = Py::None;
    let mut v_o2: Py = Py::None;
    let mut v_o3: Py = Py::None;
    let mut v_o4: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_orient: Py = Py::None;
    v_orient = { let v_orient = v_orient.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_p3 = v_p3.clone(); let v_p4 = v_p4.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_v: Py = Py::None;
            v_v = py::sub(&py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?)?)?;
            return Ok((if ({ let t2356 = v_v.clone(); Py::Bool({ let t2357 = Py::Int(0i64); if py::eq(&t2356, &t2357) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { (if ({ let t2358 = v_v.clone(); Py::Bool({ let t2359 = Py::Int(0i64); if py::gt(&t2358, &t2359)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2360 = (Py::tuple(vec![py::call(&v_orient.clone(), &[v_p1.clone(), v_p2.clone(), v_p3.clone()])?, py::call(&v_orient.clone(), &[v_p1.clone(), v_p2.clone(), v_p4.clone()])?, py::call(&v_orient.clone(), &[v_p3.clone(), v_p4.clone(), v_p1.clone()])?, py::call(&v_orient.clone(), &[v_p3.clone(), v_p4.clone(), v_p2.clone()])?])).items()?;
    if t2360.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_o1 = t2360[0].clone();
    v_o2 = t2360[1].clone();
    v_o3 = t2360[2].clone();
    v_o4 = t2360[3].clone();
    if ({ let t2365 = { let t2361 = v_o1.clone(); Py::Bool({ let t2362 = v_o2.clone(); if py::ne(&t2361, &t2362) { true } else { false } }) }; if !t2365.truthy() { t2365 } else { { let t2363 = v_o3.clone(); Py::Bool({ let t2364 = v_o4.clone(); if py::ne(&t2363, &t2364) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(true));
    }
    v_on = { let v_o1 = v_o1.clone(); let v_o2 = v_o2.clone(); let v_o3 = v_o3.clone(); let v_o4 = v_o4.clone(); let v_orient = v_orient.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_p3 = v_p3.clone(); let v_p4 = v_p4.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok({ let t2372 = { let t2366 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], false)?; Py::Bool({ let t2367 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; if py::le(&t2366, &t2367)? { { let t2368 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], true)?; if py::le(&t2367, &t2368)? { true } else { false } } } else { false } }) }; if !t2372.truthy() { t2372 } else { { let t2369 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], false)?; Py::Bool({ let t2370 = py::getitem(&v_c.clone(), &Py::Int(1i64))?; if py::le(&t2369, &t2370)? { { let t2371 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], true)?; if py::le(&t2370, &t2371)? { true } else { false } } } else { false } }) } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok({ let t2387 = { let t2375 = { let t2373 = v_o1.clone(); Py::Bool({ let t2374 = Py::Int(0i64); if py::eq(&t2373, &t2374) { true } else { false } }) }; if !t2375.truthy() { t2375 } else { py::call(&v_on.clone(), &[v_p1.clone(), v_p2.clone(), v_p3.clone()])? } }; if t2387.truthy() { t2387 } else { { let t2386 = { let t2378 = { let t2376 = v_o2.clone(); Py::Bool({ let t2377 = Py::Int(0i64); if py::eq(&t2376, &t2377) { true } else { false } }) }; if !t2378.truthy() { t2378 } else { py::call(&v_on.clone(), &[v_p1.clone(), v_p2.clone(), v_p4.clone()])? } }; if t2386.truthy() { t2386 } else { { let t2385 = { let t2381 = { let t2379 = v_o3.clone(); Py::Bool({ let t2380 = Py::Int(0i64); if py::eq(&t2379, &t2380) { true } else { false } }) }; if !t2381.truthy() { t2381 } else { py::call(&v_on.clone(), &[v_p3.clone(), v_p4.clone(), v_p1.clone()])? } }; if t2385.truthy() { t2385 } else { { let t2384 = { let t2382 = v_o4.clone(); Py::Bool({ let t2383 = Py::Int(0i64); if py::eq(&t2382, &t2383) { true } else { false } }) }; if !t2384.truthy() { t2384 } else { py::call(&v_on.clone(), &[v_p3.clone(), v_p4.clone(), v_p2.clone()])? } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shaft_check(mut v_masses: Py, mut v_positions: Py, mut v_L: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("L, E and I"), Py::tuple(vec![v_L.clone(), v_E.clone(), v_I.clone()]))?;
    if ({ let t3393 = { let t3388 = py::len(&v_masses.clone())?; Py::Bool({ let t3389 = py::len(&v_positions.clone())?; if py::ne(&t3388, &t3389) { true } else { false } }) }; if t3393.truthy() { t3393 } else { py::not(&{ let t3390 = Py::Int(1i64); Py::Bool({ let t3391 = py::len(&v_masses.clone())?; if py::le(&t3390, &t3391)? { { let t3392 = Py::Int(10i64); if py::le(&t3391, &t3392)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("masses and positions must be lists of the same length (1 to 10 entries)"))));
    }
    for t3394 in py::iter(&(v_masses.clone()))? {
        v_v = t3394;
        let _ = f__pos(Py::str("every mass"), Py::tuple(vec![v_v.clone()]))?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t3395 in py::iter(&(v_positions.clone()))? { let mut v_p: Py; 
v_p = t3395;
out.push(py::not(&{ let t3396 = Py::Int(0i64); Py::Bool({ let t3397 = v_p.clone(); if py::lt(&t3396, &t3397)? { { let t3398 = v_L.clone(); if py::lt(&t3397, &t3398)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every disk must sit between the bearings (0 < position < L)"))));
    }
    if ({ let t3399 = py::len(&py::set(&v_positions.clone())?)?; Py::Bool({ let t3400 = py::len(&v_positions.clone())?; if py::ne(&t3399, &t3400) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("two disks at one position: combine them"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sind(mut v_a: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = py::modulo(&v_a.clone(), &Py::Float(360.0))?;
    return Ok(py::method(&Py::dict(vec![(Py::Float(0.0), Py::Float(0.0)), (Py::Float(90.0), Py::Float(1.0)), (Py::Float(180.0), Py::Float(0.0)), (Py::Float(270.0), py::neg(&Py::Float(1.0))?)]), "get", &[v_r.clone(), py::math1("sin", &f__deg(v_a.clone())?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_x: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t4609 = py::abs(&v_x.clone())?; Py::Bool({ let t4610 = py::mul(&Py::Float(1e-12), &py::abs(&v_scale.clone())?)?; if py::le(&t4609, &t4610)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ss_influence(mut v_x: Py, mut v_a: Py, mut v_L: Py) -> R {
    let mut v_b: Py = Py::None;
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    if ({ let t3403 = v_x.clone(); Py::Bool({ let t3404 = v_a.clone(); if py::le(&t3403, &t3404)? { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::mul(&py::mul(&v_b.clone(), &v_x.clone())?, &py::sub(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_x.clone(), &v_x.clone())?)?)?, &py::mul(&Py::Int(6i64), &v_L.clone())?)?);
    }
    return Ok(py::truediv(&py::mul(&py::mul(&v_a.clone(), &py::sub(&v_L.clone(), &v_x.clone())?)?, &py::sub(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::pow(&py::sub(&v_L.clone(), &v_x.clone())?, &Py::Int(2i64))?)?)?, &py::mul(&Py::Int(6i64), &v_L.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stress_scale(mut v_v: Py) -> R {
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4583 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t4583;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(1e-300)]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__truss_read(mut v_nodes: Py, mut v_members: Py, mut v_pin: Py, mut v_roller: Py, mut v_loads: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_mem: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_row: Py = Py::None;
    v_pts = { let mut out = vec![]; for t4232 in py::iter(&(v_nodes.clone()))? { let mut v_r: Py; 
v_r = t4232;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) };
    if ({ let t4239 = Py::Bool(({ let mut out = vec![]; for t4233 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t4233;
out.push({ let t4234 = py::len(&v_p.clone())?; Py::Bool({ let t4235 = Py::Int(2i64); if py::ne(&t4234, &t4235) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t4239.truthy() { t4239 } else { py::not(&{ let t4236 = Py::Int(3i64); Py::Bool({ let t4237 = py::len(&v_pts.clone())?; if py::le(&t4236, &t4237)? { { let t4238 = Py::Int(30i64); if py::le(&t4237, &t4238)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 3 to 30 nodes as rows 'x y' (m)"))));
    }
    v_n = py::len(&v_pts.clone())?;
    v_mem = Py::list(vec![]);
    for t4240 in py::iter(&(v_members.clone()))? {
        v_e = t4240;
        if ({ let t4241 = py::len(&v_e.clone())?; Py::Bool({ let t4242 = Py::Int(2i64); if py::ne(&t4241, &t4242) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("members are pairs of node numbers, e.g. 1-2"))));
        }
        let t4244: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                let t4243 = (Py::tuple(vec![py::int(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)?])).items()?;
                if t4243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_i = t4243[0].clone();
                v_j = t4243[1].clone();
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4244 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                return Err(py::value_error(py::to_str(&Py::str("members are pairs of node numbers, e.g. 1-2"))));
            }
            Err(e) => return Err(e),
        }
        if ({ let t4254 = py::not(&{ let t4251 = { let t4245 = Py::Int(1i64); Py::Bool({ let t4246 = v_i.clone(); if py::le(&t4245, &t4246)? { { let t4247 = v_n.clone(); if py::le(&t4246, &t4247)? { true } else { false } } } else { false } }) }; if !t4251.truthy() { t4251 } else { { let t4248 = Py::Int(1i64); Py::Bool({ let t4249 = v_j.clone(); if py::le(&t4248, &t4249)? { { let t4250 = v_n.clone(); if py::le(&t4249, &t4250)? { true } else { false } } } else { false } }) } } }); if t4254.truthy() { t4254 } else { { let t4252 = v_i.clone(); Py::Bool({ let t4253 = v_j.clone(); if py::eq(&t4252, &t4253) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["member ".to_string(), py::format(&py::getitem(&v_e.clone(), &Py::Int(0i64))?, "")?, "-".to_string(), py::format(&py::getitem(&v_e.clone(), &Py::Int(1i64))?, "")?, " does not join two different nodes".to_string()].concat()))));
        }
        if ({ let t4255 = py::getitem(&v_pts.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t4256 = py::getitem(&v_pts.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; if py::eq(&t4255, &t4256) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a member has zero length"))));
        }
        let _ = py::method(&v_mem.clone(), "append", &[Py::tuple(vec![py::sub(&v_i.clone(), &Py::Int(1i64))?, py::sub(&v_j.clone(), &Py::Int(1i64))?])])?;
    }
    if ({ let t4257 = py::len(&py::set(&{ let mut out = vec![]; for t4259 in py::iter(&(v_mem.clone()))? { let mut v_m: Py; 
v_m = t4259;
out.push(Py::tuple((py::sorted(&v_m.clone(), (Py::Bool(false)).truthy())?).items()?)); } Py::list(out) })?)?; Py::Bool({ let t4258 = py::len(&v_mem.clone())?; if py::ne(&t4257, &t4258) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a member is listed twice"))));
    }
    if ({ let t4269 = py::not(&{ let t4266 = { let t4260 = Py::Int(1i64); Py::Bool({ let t4261 = v_pin.clone(); if py::le(&t4260, &t4261)? { { let t4262 = v_n.clone(); if py::le(&t4261, &t4262)? { true } else { false } } } else { false } }) }; if !t4266.truthy() { t4266 } else { { let t4263 = Py::Int(1i64); Py::Bool({ let t4264 = v_roller.clone(); if py::le(&t4263, &t4264)? { { let t4265 = v_n.clone(); if py::le(&t4264, &t4265)? { true } else { false } } } else { false } }) } } }); if t4269.truthy() { t4269 } else { { let t4267 = v_pin.clone(); Py::Bool({ let t4268 = v_roller.clone(); if py::eq(&t4267, &t4268) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pin and the roller must be two different nodes"))));
    }
    v_ld = Py::list(vec![]);
    for t4270 in py::iter(&(v_loads.clone()))? {
        v_row = t4270;
        if ({ let t4279 = { let t4271 = py::len(&v_row.clone())?; Py::Bool({ let t4272 = Py::Int(3i64); if py::ne(&t4271, &t4272) { true } else { false } }) }; if t4279.truthy() { t4279 } else { { let t4278 = { let t4273 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t4274 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::ne(&t4273, &t4274) { true } else { false } }) }; if t4278.truthy() { t4278 } else { py::not(&{ let t4275 = Py::Int(1i64); Py::Bool({ let t4276 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::le(&t4275, &t4276)? { { let t4277 = v_n.clone(); if py::le(&t4276, &t4277)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each load is a row 'node Fx Fy' (kN)"))));
        }
        let _ = py::method(&v_ld.clone(), "append", &[Py::tuple(vec![py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?, py::getitem(&v_row.clone(), &Py::Int(1i64))?, py::getitem(&v_row.clone(), &Py::Int(2i64))?])])?;
    }
    if ({ let t4280 = py::add(&py::len(&v_mem.clone())?, &Py::Int(3i64))?; Py::Bool({ let t4281 = py::mul(&Py::Int(2i64), &v_n.clone())?; if py::ne(&t4280, &t4281) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["not statically determinate: ".to_string(), py::format(&py::len(&v_mem.clone())?, "")?, " members + 3 reactions must equal 2 x ".to_string(), py::format(&v_n.clone(), "")?, " joint equations".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![v_pts.clone(), v_mem.clone(), py::sub(&v_pin.clone(), &Py::Int(1i64))?, py::sub(&v_roller.clone(), &Py::Int(1i64))?, v_ld.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__weib(mut v_x0: Py, mut v_theta: Py, mut v_b: Py) -> R {
    v_x0 = (if ({ let t323 = v_x0.clone(); Py::Bool({ let t324 = Py::None; if py::is(&t323, &t324) { true } else { false } }) }).truthy() { py::getitem(&c__WEIBULL()?, &Py::str("x0"))? } else { v_x0.clone() });
    v_theta = (if ({ let t325 = v_theta.clone(); Py::Bool({ let t326 = Py::None; if py::is(&t325, &t326) { true } else { false } }) }).truthy() { py::getitem(&c__WEIBULL()?, &Py::str("theta"))? } else { v_theta.clone() });
    v_b = (if ({ let t327 = v_b.clone(); Py::Bool({ let t328 = Py::None; if py::is(&t327, &t328) { true } else { false } }) }).truthy() { py::getitem(&c__WEIBULL()?, &Py::str("b"))? } else { v_b.clone() });
    if (py::not(&{ let t334 = { let t329 = Py::Int(0i64); Py::Bool({ let t330 = v_x0.clone(); if py::le(&t329, &t330)? { { let t331 = v_theta.clone(); if py::lt(&t330, &t331)? { true } else { false } } } else { false } }) }; if !t334.truthy() { t334 } else { { let t332 = v_b.clone(); Py::Bool({ let t333 = Py::Int(0i64); if py::gt(&t332, &t333)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Weibull parameters need 0 <= x0 < theta and b > 0"))));
    }
    return Ok(Py::tuple(vec![v_x0.clone(), v_theta.clone(), v_b.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wrap_factor(mut v_mu: Py, mut v_theta: Py, mut v_beta: Py) -> R {
    let mut v_mu_e: Py = Py::None;
    let _ = f__pos(Py::str("mu and the wrap angle"), Py::tuple(vec![v_mu.clone(), v_theta.clone()]))?;
    if ({ let t429 = { let t424 = v_beta.clone(); Py::Bool({ let t425 = Py::None; if !py::is(&t424, &t425) { true } else { false } }) }; if !t429.truthy() { t429 } else { py::not(&{ let t426 = Py::Int(0i64); Py::Bool({ let t427 = v_beta.clone(); if py::lt(&t426, &t427)? { { let t428 = Py::Float(std::f64::consts::PI); if py::lt(&t427, &t428)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the groove angle must lie between 0 and 180 degrees"))));
    }
    v_mu_e = (if ({ let t430 = v_beta.clone(); Py::Bool({ let t431 = Py::None; if !py::is(&t430, &t431) { true } else { false } }) }).truthy() { py::truediv(&v_mu.clone(), &py::math1("sin", &py::truediv(&v_beta.clone(), &Py::Int(2i64))?)?)? } else { v_mu.clone() });
    if ({ let t432 = py::mul(&v_mu_e.clone(), &v_theta.clone())?; Py::Bool({ let t433 = Py::Int(50i64); if py::gt(&t432, &t433)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("mu*theta is unrealistically large (> 50)"))));
    }
    return Ok(v_mu_e.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_section(mut v_b: Py, mut v_h: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Ixy: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let mut v_yc: Py = Py::None;
    let _ = f__pos(Py::str("b, h and t"), Py::tuple(vec![v_b.clone(), v_h.clone(), v_t.clone()]))?;
    if ({ let t6 = v_t.clone(); Py::Bool({ let t7 = py::minmax(&[v_b.clone(), v_h.clone()], false)?; if py::ge(&t6, &t7)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the thickness must be less than both leg lengths"))));
    }
    let t22 = (f__rect_set(Py::list(vec![Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_t.clone(), py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&v_t.clone(), &Py::Int(2i64))?]), Py::tuple(vec![Py::Int(1i64), v_t.clone(), py::sub(&v_h.clone(), &v_t.clone())?, py::truediv(&v_t.clone(), &Py::Int(2i64))?, py::add(&v_t.clone(), &py::truediv(&py::sub(&v_h.clone(), &v_t.clone())?, &Py::Int(2i64))?)?])]))?).items()?;
    if t22.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t22[0].clone();
    v_xc = t22[1].clone();
    v_yc = t22[2].clone();
    v_Ix = t22[3].clone();
    v_Iy = t22[4].clone();
    v_Ixy = t22[5].clone();
    let t28 = (f__principal(v_Ix.clone(), v_Iy.clone(), v_Ixy.clone())?).items()?;
    if t28.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I1 = t28[0].clone();
    v_I2 = t28[1].clone();
    v_phi = t28[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("x_bar"), v_xc.clone()), (Py::str("y_bar"), v_yc.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Ixy"), v_Ixy.clone()), (Py::str("I1"), v_I1.clone()), (Py::str("I2"), v_I2.clone()), (Py::str("principal_angle_deg"), v_phi.clone()), (Py::str("r_min"), py::math1("sqrt", &py::truediv(&v_I2.clone(), &v_A.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_asme_elliptic(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_Sy: Py) -> R {
    let _ = f__fat_check(v_sa.clone(), v_sm.clone(), v_Se.clone(), v_Sy.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("n_fatigue"), py::truediv(&Py::Float(1.0), &py::hypot(&[py::truediv(&v_sa.clone(), &v_Se.clone())?, py::truediv(&v_sm.clone(), &v_Sy.clone())?])?)?), (Py::str("sigma_ar_MPa"), py::truediv(&py::truediv(&v_sa.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_sm.clone(), &v_Sy.clone())?, &Py::Int(2i64))?)?)?)?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_axial_bar(mut v_lengths: Py, mut v_areas: Py, mut v_loads: Py, mut v_E: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_stress: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("E"), Py::tuple(vec![v_E.clone()]))?;
    v_n = py::len(&v_lengths.clone())?;
    if (py::not(&{ let t114 = { let t110 = py::len(&v_areas.clone())?; Py::Bool({ let t111 = v_n.clone(); if py::eq(&t110, &t111) { true } else { false } }) }; if !t114.truthy() { t114 } else { { let t112 = py::len(&v_loads.clone())?; Py::Bool({ let t113 = v_n.clone(); if py::eq(&t112, &t113) { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("lengths, areas and loads must have the same number of entries (one per segment)"))));
    }
    for t115 in py::iter(&(py::add(&Py::list((v_lengths.clone()).items()?), &Py::list((v_areas.clone()).items()?))?))? {
        v_v = t115;
        let _ = f__pos(Py::str("every length and area"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__fin(Py::str("every load"), { let mut v = vec![]; v.extend((v_loads.clone()).items()?); Py::tuple(v) })?;
    v_N = { let mut out = vec![]; for t118 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t118;
out.push(py::fsum(&py::slice(&v_loads.clone(), Some((v_i.clone()).i()?), None, None)?)?); } Py::list(out) };
    v_scale = { let t120 = py::minmax(&[{ let mut out = vec![]; for t119 in py::iter(&(v_loads.clone()))? { let mut v_v: Py; 
v_v = t119;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?; if t120.truthy() { t120 } else { Py::Float(1.0) } };
    v_N = { let mut out = vec![]; for t121 in py::iter(&(v_N.clone()))? { let mut v_v: Py; 
v_v = t121;
out.push(f__snap(v_v.clone(), v_scale.clone())?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t124 in py::iter(&(v_N.clone()))? { let mut v_v: Py; 
v_v = t124;
out.push({ let t125 = v_v.clone(); Py::Bool({ let t126 = Py::Int(0i64); if py::eq(&t125, &t126) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the loads balance to zero in every segment"))));
    }
    v_stress = { let mut out = vec![]; for t127 in py::iter(&(py::zip(&[v_N.clone(), v_areas.clone()])?))? { let mut v_Ai: Py; let mut v_Ni: Py; 
let t128 = (t127).items()?;
if t128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_Ni = t128[0].clone();
v_Ai = t128[1].clone();
out.push(py::truediv(&py::mul(&v_Ni.clone(), &Py::Float(1000.0))?, &v_Ai.clone())?); } Py::list(out) };
    v_delta = py::fsum(&{ let mut out = vec![]; for t129 in py::iter(&(py::zip(&[v_N.clone(), v_lengths.clone(), v_areas.clone()])?))? { let mut v_Ai: Py; let mut v_Li: Py; let mut v_Ni: Py; 
let t130 = (t129).items()?;
if t130.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_Ni = t130[0].clone();
v_Li = t130[1].clone();
v_Ai = t130[2].clone();
out.push(py::truediv(&py::mul(&py::mul(&v_Ni.clone(), &Py::Float(1000.0))?, &v_Li.clone())?, &py::mul(&py::mul(&v_Ai.clone(), &v_E.clone())?, &Py::Float(1e-06))?)?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("elongation_mm"), f__snap(v_delta.clone(), py::sum(&{ let mut out = vec![]; for t139 in py::iter(&(py::zip(&[v_N.clone(), v_lengths.clone(), v_areas.clone()])?))? { let mut v_Ai: Py; let mut v_Li: Py; let mut v_Ni: Py; 
let t140 = (t139).items()?;
if t140.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_Ni = t140[0].clone();
v_Li = t140[1].clone();
v_Ai = t140[2].clone();
out.push(py::truediv(&py::mul(&py::mul(&py::abs(&v_Ni.clone())?, &Py::Float(1000.0))?, &v_Li.clone())?, &py::mul(&py::mul(&v_Ai.clone(), &v_E.clone())?, &Py::Float(1e-06))?)?); } Py::list(out) }, None)?)?), (Py::str("N_kN"), v_N.clone()), (Py::str("stress_MPa"), v_stress.clone()), (Py::str("max_abs_stress_MPa"), py::minmax(&[{ let mut out = vec![]; for t141 in py::iter(&(v_stress.clone()))? { let mut v_v: Py; 
v_v = t141;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bar_fixed_ends(mut v_P: Py, mut v_L1: Py, mut v_A1: Py, mut v_E1: Py, mut v_L2: Py, mut v_A2: Py, mut v_E2: Py) -> R {
    let mut v_RA: Py = Py::None;
    let mut v_RB: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let _ = f__pos(Py::str("P, L1, A1, E1, L2, A2 and E2"), Py::tuple(vec![v_P.clone(), v_L1.clone(), v_A1.clone(), v_E1.clone(), v_L2.clone(), v_A2.clone(), v_E2.clone()]))?;
    let t171 = (Py::tuple(vec![py::truediv(&py::mul(&v_A1.clone(), &v_E1.clone())?, &v_L1.clone())?, py::truediv(&py::mul(&v_A2.clone(), &v_E2.clone())?, &v_L2.clone())?])).items()?;
    if t171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k1 = t171[0].clone();
    v_k2 = t171[1].clone();
    v_RA = py::truediv(&py::mul(&v_P.clone(), &v_k1.clone())?, &py::add(&v_k1.clone(), &v_k2.clone())?)?;
    v_RB = py::sub(&v_P.clone(), &v_RA.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), v_RA.clone()), (Py::str("RB"), v_RB.clone()), (Py::str("stress1_MPa"), py::truediv(&py::truediv(&v_RA.clone(), &v_A1.clone())?, &Py::Float(1000000.0))?), (Py::str("stress2_MPa"), py::truediv(&py::truediv(&py::neg(&v_RB.clone())?, &v_A2.clone())?, &Py::Float(1000000.0))?), (Py::str("displacement_mm"), py::mul(&py::truediv(&v_P.clone(), &py::add(&v_k1.clone(), &v_k2.clone())?)?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beam_frequency(mut v_E: Py, mut v_I: Py, mut v_rho: Py, mut v_A: Py, mut v_L: Py, mut v_ends: Py, mut v_n: Py) -> R {
    let mut v_bl: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("E, I, rho, A and L"), Py::tuple(vec![v_E.clone(), v_I.clone(), v_rho.clone(), v_A.clone(), v_L.clone()]))?;
    if (py::not(&{ let t212 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t212.truthy() { t212 } else { { let t209 = Py::Int(1i64); Py::Bool({ let t210 = v_n.clone(); if py::le(&t209, &t210)? { { let t211 = Py::Int(6i64); if py::le(&t210, &t211)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mode number must be 1 to 6"))));
    }
    v_bl = f__beta_L(v_ends.clone(), v_n.clone())?;
    v_w = py::mul(&py::mul(&v_bl.clone(), &v_bl.clone())?, &py::math1("sqrt", &py::truediv(&py::mul(&v_E.clone(), &v_I.clone())?, &py::mul(&py::mul(&v_rho.clone(), &v_A.clone())?, &py::pow(&v_L.clone(), &Py::Int(4i64))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta_L"), v_bl.clone()), (Py::str("omega"), v_w.clone()), (Py::str("f"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bearing_life(mut v_C: Py, mut v_F: Py, mut v_n: Py, mut v_R: Py, mut v_kind: Py, mut v_LR: Py, mut v_x0: Py, mut v_theta: Py, mut v_b: Py) -> R {
    let mut v_L10: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_rev_per_h: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("C, F and n"), Py::tuple(vec![v_C.clone(), v_F.clone(), v_n.clone()]))?;
    if (py::not(&{ let t320 = Py::Int(0i64); Py::Bool({ let t321 = v_R.clone(); if py::lt(&t320, &t321)? { { let t322 = Py::Int(1i64); if py::lt(&t321, &t322)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reliability must lie strictly between 0 and 1"))));
    }
    let t335 = (f__weib(v_x0.clone(), v_theta.clone(), v_b.clone())?).items()?;
    if t335.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t335[0].clone();
    v_theta = t335[1].clone();
    v_b = t335[2].clone();
    v_a = (if ({ let t336 = v_kind.clone(); Py::Bool({ let t337 = Py::str("ball"); if py::eq(&t336, &t337) { true } else { false } }) }).truthy() { Py::Float(3.0) } else { py::truediv(&Py::Int(10i64), &Py::Int(3i64))? });
    v_L10 = py::mul(&v_LR.clone(), &py::pow(&py::truediv(&v_C.clone(), &v_F.clone())?, &v_a.clone())?)?;
    v_x = py::add(&v_x0.clone(), &py::mul(&py::sub(&v_theta.clone(), &v_x0.clone())?, &py::pow(&py::math1("log", &py::truediv(&Py::Int(1i64), &v_R.clone())?)?, &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?)?;
    v_rev_per_h = py::truediv(&py::mul(&v_n.clone(), &Py::Int(3600i64))?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L10_rev"), v_L10.clone()), (Py::str("L10_hours"), py::truediv(&v_L10.clone(), &v_rev_per_h.clone())?), (Py::str("life_multiple"), v_x.clone()), (Py::str("L_rev"), py::mul(&v_L10.clone(), &v_x.clone())?), (Py::str("L_hours"), py::truediv(&py::mul(&v_L10.clone(), &v_x.clone())?, &v_rev_per_h.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bearing_rating(mut v_FD: Py, mut v_LD: Py, mut v_n: Py, mut v_R: Py, mut v_kind: Py, mut v_af: Py, mut v_LR: Py, mut v_x0: Py, mut v_theta: Py, mut v_b: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_xD: Py = Py::None;
    let _ = f__pos(Py::str("FD, LD, n and af"), Py::tuple(vec![v_FD.clone(), v_LD.clone(), v_n.clone(), v_af.clone()]))?;
    if (py::not(&{ let t265 = Py::Int(0i64); Py::Bool({ let t266 = v_R.clone(); if py::lt(&t265, &t266)? { { let t267 = Py::Int(1i64); if py::lt(&t266, &t267)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reliability must lie strictly between 0 and 1"))));
    }
    let t280 = (f__weib(v_x0.clone(), v_theta.clone(), v_b.clone())?).items()?;
    if t280.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t280[0].clone();
    v_theta = t280[1].clone();
    v_b = t280[2].clone();
    v_a = (if ({ let t281 = v_kind.clone(); Py::Bool({ let t282 = Py::str("ball"); if py::eq(&t281, &t282) { true } else { false } }) }).truthy() { Py::Float(3.0) } else { py::truediv(&Py::Int(10i64), &Py::Int(3i64))? });
    v_xD = py::truediv(&py::truediv(&py::mul(&v_LD.clone(), &v_n.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &v_LR.clone())?;
    v_C = py::mul(&py::mul(&v_af.clone(), &v_FD.clone())?, &py::pow(&py::truediv(&v_xD.clone(), &py::add(&v_x0.clone(), &py::mul(&py::sub(&v_theta.clone(), &v_x0.clone())?, &py::pow(&py::math1("log", &py::truediv(&Py::Int(1i64), &v_R.clone())?)?, &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?)?)?, &py::truediv(&Py::Int(1i64), &v_a.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C10"), v_C.clone()), (Py::str("C10_kN"), py::truediv(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("design_life_multiple"), v_xD.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_belt_power(mut v_T1: Py, mut v_m: Py, mut v_v: Py, mut v_mu: Py, mut v_theta: Py, mut v_beta: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_Tc: Py = Py::None;
    let mut v_mu_e: Py = Py::None;
    let _ = f__pos(Py::str("T1, m and v"), Py::tuple(vec![v_T1.clone(), v_m.clone(), v_v.clone()]))?;
    v_mu_e = f__wrap_factor(v_mu.clone(), v_theta.clone(), v_beta.clone())?;
    v_Tc = py::mul(&py::mul(&v_m.clone(), &v_v.clone())?, &v_v.clone())?;
    if ({ let t385 = v_Tc.clone(); Py::Bool({ let t386 = v_T1.clone(); if py::ge(&t385, &t386)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the centrifugal tension m v^2 reaches the allowable tension: the belt transmits nothing"))));
    }
    v_T2 = py::add(&v_Tc.clone(), &py::truediv(&py::sub(&v_T1.clone(), &v_Tc.clone())?, &py::math1("exp", &py::mul(&v_mu_e.clone(), &v_theta.clone())?)?)?)?;
    v_P = py::mul(&py::sub(&v_T1.clone(), &v_T2.clone())?, &v_v.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Tc"), v_Tc.clone()), (Py::str("T2"), v_T2.clone()), (Py::str("P"), v_P.clone()), (Py::str("P_kW"), py::truediv(&v_P.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bolt_group(mut v_bolts: Py, mut v_P: Py, mut v_e: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_em: Py = Py::None;
    let mut v_fmax: Py = Py::None;
    let mut v_forces: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_yc: Py = Py::None;
    v_pts = { let mut out = vec![]; for t1134 in py::iter(&(v_bolts.clone()))? { let mut v_r: Py; 
v_r = t1134;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) };
    if ({ let t1141 = Py::Bool(({ let mut out = vec![]; for t1135 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1135;
out.push({ let t1136 = py::len(&v_p.clone())?; Py::Bool({ let t1137 = Py::Int(2i64); if py::ne(&t1136, &t1137) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1141.truthy() { t1141 } else { py::not(&{ let t1138 = Py::Int(2i64); Py::Bool({ let t1139 = py::len(&v_pts.clone())?; if py::le(&t1138, &t1139)? { { let t1140 = Py::Int(40i64); if py::le(&t1139, &t1140)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 40 bolts as rows 'x y' (mm)"))));
    }
    let _ = f__pos(Py::str("the load P"), Py::tuple(vec![v_P.clone()]))?;
    let _ = f__fin(Py::str("the eccentricity e"), Py::tuple(vec![v_e.clone()]))?;
    v_n = py::len(&v_pts.clone())?;
    let t1151 = (Py::tuple(vec![py::truediv(&py::fsum(&{ let mut out = vec![]; for t1149 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1149;
out.push(py::getitem(&v_p.clone(), &Py::Int(0i64))?); } Py::list(out) })?, &v_n.clone())?, py::truediv(&py::fsum(&{ let mut out = vec![]; for t1150 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1150;
out.push(py::getitem(&v_p.clone(), &Py::Int(1i64))?); } Py::list(out) })?, &v_n.clone())?])).items()?;
    if t1151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xc = t1151[0].clone();
    v_yc = t1151[1].clone();
    v_J = py::fsum(&{ let mut out = vec![]; for t1152 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1153 = (t1152).items()?;
if t1153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1153[0].clone();
v_y = t1153[1].clone();
out.push(py::add(&py::pow(&py::sub(&v_x.clone(), &v_xc.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_y.clone(), &v_yc.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    if ({ let t1154 = v_J.clone(); Py::Bool({ let t1155 = Py::Int(0i64); if py::le(&t1154, &t1155)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the bolts must not all coincide"))));
    }
    v_em = py::mul(&v_e.clone(), &Py::Float(1000.0))?;
    v_M = py::mul(&v_P.clone(), &v_em.clone())?;
    v_forces = Py::list(vec![]);
    for t1156 in py::iter(&(v_pts.clone()))? {
        let t1157 = (t1156).items()?;
        if t1157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1157[0].clone();
        v_y = t1157[1].clone();
        v_fx = py::truediv(&py::mul(&v_M.clone(), &py::sub(&v_y.clone(), &v_yc.clone())?)?, &v_J.clone())?;
        v_fy = py::sub(&py::truediv(&py::neg(&v_P.clone())?, &v_n.clone())?, &py::truediv(&py::mul(&v_M.clone(), &py::sub(&v_x.clone(), &v_xc.clone())?)?, &v_J.clone())?)?;
        let _ = py::method(&v_forces.clone(), "append", &[py::hypot(&[v_fx.clone(), v_fy.clone()])?])?;
    }
    v_fmax = py::minmax(&[v_forces.clone()], true)?;
    v_idx = py::add(&(match ('g1158: { let mut out = vec![]; for t1159 in py::iter(&(py::enumerate(&v_forces.clone(), 0)?))? { let mut v_f: Py; let mut v_i: Py; 
let t1160 = (t1159).items()?;
if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1160[0].clone();
v_f = t1160[1].clone();
if ({ let t1161 = v_f.clone(); Py::Bool({ let t1162 = py::mul(&v_fmax.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::ge(&t1161, &t1162)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g1158 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }), &Py::Int(1i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("direct_kN"), py::truediv(&py::truediv(&v_P.clone(), &v_n.clone())?, &Py::Float(1000.0))?), (Py::str("max_bolt_force_kN"), py::truediv(&v_fmax.clone(), &Py::Float(1000.0))?), (Py::str("critical_bolt"), v_idx.clone()), (Py::str("polar_sum_mm2"), v_J.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bolt_torque(mut v_K: Py, mut v_d: Py, mut v_F: Py, mut v_T: Py) -> R {
    if (py::not(&{ let t469 = py::math1("isfinite", &v_K.clone())?; if !t469.truthy() { t469 } else { { let t466 = Py::Float(0.05); Py::Bool({ let t467 = v_K.clone(); if py::le(&t466, &t467)? { { let t468 = Py::Float(0.5); if py::le(&t467, &t468)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the nut (torque) factor K must lie between 0.05 and 0.5 (typically 0.2)"))));
    }
    let _ = f__pos(Py::str("d"), Py::tuple(vec![v_d.clone()]))?;
    if ({ let t475 = { let t477 = v_F.clone(); Py::Bool({ let t478 = Py::None; if py::is(&t477, &t478) { true } else { false } }) }; Py::Bool({ let t476 = { let t479 = v_T.clone(); Py::Bool({ let t480 = Py::None; if py::is(&t479, &t480) { true } else { false } }) }; if py::eq(&t475, &t476) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the preload F and the torque T"))));
    }
    if ({ let t481 = v_F.clone(); Py::Bool({ let t482 = Py::None; if !py::is(&t481, &t482) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("F"), Py::tuple(vec![v_F.clone()]))?;
        v_T = py::mul(&py::mul(&v_K.clone(), &v_F.clone())?, &v_d.clone())?;
    } else {
        let _ = f__pos(Py::str("T"), Py::tuple(vec![v_T.clone()]))?;
        v_F = py::truediv(&v_T.clone(), &py::mul(&v_K.clone(), &v_d.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("F"), v_F.clone()), (Py::str("F_kN"), py::truediv(&v_F.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_box_section(mut v_B: Py, mut v_H: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v_Zpx: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::str("B, H and t"), Py::tuple(vec![v_B.clone(), v_H.clone(), v_t.clone()]))?;
    if ({ let t1755 = py::mul(&Py::Int(2i64), &v_t.clone())?; Py::Bool({ let t1756 = py::minmax(&[v_B.clone(), v_H.clone()], false)?; if py::ge(&t1755, &t1756)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wall thickness must be less than half the smaller outer dimension"))));
    }
    let t1757 = (Py::tuple(vec![py::sub(&v_B.clone(), &py::mul(&Py::Int(2i64), &v_t.clone())?)?, py::sub(&v_H.clone(), &py::mul(&Py::Int(2i64), &v_t.clone())?)?])).items()?;
    if t1757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1757[0].clone();
    v_h = t1757[1].clone();
    v_A = py::sub(&py::mul(&v_B.clone(), &v_H.clone())?, &py::mul(&v_b.clone(), &v_h.clone())?)?;
    v_Ix = py::truediv(&py::sub(&py::mul(&v_B.clone(), &py::pow(&v_H.clone(), &Py::Int(3i64))?)?, &py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?)?, &Py::Int(12i64))?;
    v_Iy = py::truediv(&py::sub(&py::mul(&v_H.clone(), &py::pow(&v_B.clone(), &Py::Int(3i64))?)?, &py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?)?, &Py::Int(12i64))?;
    v_Zpx = py::truediv(&py::sub(&py::mul(&py::mul(&v_B.clone(), &v_H.clone())?, &v_H.clone())?, &py::mul(&py::mul(&v_b.clone(), &v_h.clone())?, &v_h.clone())?)?, &Py::Int(4i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Zx"), py::truediv(&v_Ix.clone(), &py::truediv(&v_H.clone(), &Py::Int(2i64))?)?), (Py::str("Zy"), py::truediv(&v_Iy.clone(), &py::truediv(&v_B.clone(), &Py::Int(2i64))?)?), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?), (Py::str("Zpx"), v_Zpx.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_brittle_mohr(mut v_sA: Py, mut v_sB: Py, mut v_Sut: Py, mut v_Suc: Py) -> R {
    let mut v_bcm: Py = Py::None;
    let mut v_mm: Py = Py::None;
    let mut v_mns: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s3: Py = Py::None;
    let _ = f__pos(Py::str("Sut and Suc"), Py::tuple(vec![v_Sut.clone(), v_Suc.clone()]))?;
    let _ = f__fin(Py::str("sA and sB"), Py::tuple(vec![v_sA.clone(), v_sB.clone()]))?;
    let t522 = (Py::tuple(vec![py::minmax(&[v_sA.clone(), v_sB.clone()], true)?, py::minmax(&[v_sA.clone(), v_sB.clone()], false)?])).items()?;
    if t522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t522[0].clone();
    v_s3 = t522[1].clone();
    if ({ let t527 = { let t523 = v_s1.clone(); Py::Bool({ let t524 = Py::Int(0i64); if py::eq(&t523, &t524) { true } else { false } }) }; if !t527.truthy() { t527 } else { { let t525 = v_s3.clone(); Py::Bool({ let t526 = Py::Int(0i64); if py::eq(&t525, &t526) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stress state is zero"))));
    }
    v_mns = py::minmax(&[(if ({ let t528 = v_s1.clone(); Py::Bool({ let t529 = Py::Int(0i64); if py::gt(&t528, &t529)? { true } else { false } }) }).truthy() { py::truediv(&v_Sut.clone(), &v_s1.clone())? } else { Py::Float(f64::INFINITY) }), (if ({ let t530 = v_s3.clone(); Py::Bool({ let t531 = Py::Int(0i64); if py::lt(&t530, &t531)? { true } else { false } }) }).truthy() { py::truediv(&v_Suc.clone(), &py::neg(&v_s3.clone())?)? } else { Py::Float(f64::INFINITY) })], false)?;
    v_bcm = py::truediv(&Py::Float(1.0), &py::sub(&py::truediv(&py::minmax(&[v_s1.clone(), Py::Float(0.0)], true)?, &v_Sut.clone())?, &py::truediv(&py::minmax(&[v_s3.clone(), Py::Float(0.0)], false)?, &v_Suc.clone())?)?)?;
    if ({ let t532 = v_s3.clone(); Py::Bool({ let t533 = Py::Int(0i64); if py::ge(&t532, &t533)? { true } else { false } }) }).truthy() {
        v_mm = py::truediv(&v_Sut.clone(), &v_s1.clone())?;
    } else {
        if ({ let t534 = v_s1.clone(); Py::Bool({ let t535 = Py::Int(0i64); if py::le(&t534, &t535)? { true } else { false } }) }).truthy() {
            v_mm = py::truediv(&v_Suc.clone(), &py::neg(&v_s3.clone())?)?;
        } else {
            if ({ let t536 = py::neg(&v_s3.clone())?; Py::Bool({ let t537 = v_s1.clone(); if py::le(&t536, &t537)? { true } else { false } }) }).truthy() {
                v_mm = py::truediv(&v_Sut.clone(), &v_s1.clone())?;
            } else {
                v_mm = py::truediv(&Py::Float(1.0), &py::sub(&py::truediv(&py::mul(&py::sub(&v_Suc.clone(), &v_Sut.clone())?, &v_s1.clone())?, &py::mul(&v_Suc.clone(), &v_Sut.clone())?)?, &py::truediv(&v_s3.clone(), &v_Suc.clone())?)?)?;
            }
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("n_max_normal"), v_mns.clone()), (Py::str("n_coulomb_mohr"), v_bcm.clone()), (Py::str("n_modified_mohr"), v_mm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cantilever_point_load(mut v_L: Py, mut v_P: Py, mut v_E: Py, mut v_I: Py, mut v_a: Py) -> R {
    let mut v_EI: Py = Py::None;
    let mut v_d_load: Py = Py::None;
    let mut v_d_tip: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("L and P"), Py::tuple(vec![v_L.clone(), v_P.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    v_a = (if ({ let t575 = v_a.clone(); Py::Bool({ let t576 = Py::None; if py::is(&t575, &t576) { true } else { false } }) }).truthy() { v_L.clone() } else { v_a.clone() });
    if (py::not(&{ let t577 = Py::Int(0i64); Py::Bool({ let t578 = v_a.clone(); if py::lt(&t577, &t578)? { { let t579 = v_L.clone(); if py::le(&t578, &t579)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load position a (from the fixed end) must satisfy 0 < a <= L"))));
    }
    v_EI = py::mul(&v_E.clone(), &v_I.clone())?;
    v_d_load = py::truediv(&py::mul(&v_P.clone(), &py::pow(&v_a.clone(), &Py::Int(3i64))?)?, &py::mul(&Py::Int(3i64), &v_EI.clone())?)?;
    v_th = py::truediv(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_a.clone())?, &py::mul(&Py::Int(2i64), &v_EI.clone())?)?;
    v_d_tip = py::add(&v_d_load.clone(), &py::mul(&v_th.clone(), &py::sub(&v_L.clone(), &v_a.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_P.clone()), (Py::str("M_fixed"), py::mul(&v_P.clone(), &v_a.clone())?), (Py::str("deflection_at_load_mm"), py::mul(&v_d_load.clone(), &Py::Float(1000.0))?), (Py::str("tip_deflection_mm"), py::mul(&v_d_tip.clone(), &Py::Float(1000.0))?), (Py::str("tip_slope_deg"), py::math1("degrees", &v_th.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cantilever_triangular(mut v_L: Py, mut v_w: Py, mut v_E: Py, mut v_I: Py) -> R {
    let _ = f__pos(Py::str("L and w"), Py::tuple(vec![v_L.clone(), v_w.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?), (Py::str("M_fixed"), py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &v_L.clone())?, &Py::Int(6i64))?), (Py::str("tip_deflection_mm"), py::mul(&py::truediv(&py::mul(&v_w.clone(), &py::pow(&v_L.clone(), &Py::Int(4i64))?)?, &py::mul(&py::mul(&Py::Int(30i64), &v_E.clone())?, &v_I.clone())?)?, &Py::Float(1000.0))?), (Py::str("tip_slope_deg"), py::math1("degrees", &py::truediv(&py::mul(&v_w.clone(), &py::pow(&v_L.clone(), &Py::Int(3i64))?)?, &py::mul(&py::mul(&Py::Int(24i64), &v_E.clone())?, &v_I.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cantilever_udl(mut v_L: Py, mut v_w: Py, mut v_E: Py, mut v_I: Py) -> R {
    let _ = f__pos(Py::str("L and w"), Py::tuple(vec![v_L.clone(), v_w.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::mul(&v_w.clone(), &v_L.clone())?), (Py::str("M_fixed"), py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &v_L.clone())?, &Py::Int(2i64))?), (Py::str("tip_deflection_mm"), py::mul(&py::truediv(&py::mul(&v_w.clone(), &py::pow(&v_L.clone(), &Py::Int(4i64))?)?, &py::mul(&py::mul(&Py::Int(8i64), &v_E.clone())?, &v_I.clone())?)?, &Py::Float(1000.0))?), (Py::str("tip_slope_deg"), py::math1("degrees", &py::truediv(&py::mul(&v_w.clone(), &py::pow(&v_L.clone(), &Py::Int(3i64))?)?, &py::mul(&py::mul(&Py::Int(6i64), &v_E.clone())?, &v_I.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capstan(mut v_T2: Py, mut v_mu: Py, mut v_theta: Py, mut v_beta: Py) -> R {
    let mut v_mu_e: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the slack-side tension T2"), Py::tuple(vec![v_T2.clone()]))?;
    v_mu_e = f__wrap_factor(v_mu.clone(), v_theta.clone(), v_beta.clone())?;
    v_r = py::math1("exp", &py::mul(&v_mu_e.clone(), &v_theta.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T1"), py::mul(&v_T2.clone(), &v_r.clone())?), (Py::str("ratio"), v_r.clone()), (Py::str("friction_force"), py::mul(&v_T2.clone(), &py::sub(&v_r.clone(), &Py::Int(1i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_channel_section(mut v_b: Py, mut v_h: Py, mut v_tf: Py, mut v_tw: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v__yc: Py = Py::None;
    let mut v_hw: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let _ = f__pos(Py::str("b, h, tf and tw"), Py::tuple(vec![v_b.clone(), v_h.clone(), v_tf.clone(), v_tw.clone()]))?;
    if ({ let t695 = { let t691 = py::mul(&Py::Int(2i64), &v_tf.clone())?; Py::Bool({ let t692 = v_h.clone(); if py::ge(&t691, &t692)? { true } else { false } }) }; if t695.truthy() { t695 } else { { let t693 = v_tw.clone(); Py::Bool({ let t694 = v_b.clone(); if py::ge(&t693, &t694)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 2 tf < h and tw < b"))));
    }
    v_hw = py::sub(&v_h.clone(), &py::mul(&Py::Int(2i64), &v_tf.clone())?)?;
    let t710 = (f__rect_set(Py::list(vec![Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_tf.clone(), py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::sub(&v_h.clone(), &py::truediv(&v_tf.clone(), &Py::Int(2i64))?)?]), Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_tf.clone(), py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&v_tf.clone(), &Py::Int(2i64))?]), Py::tuple(vec![Py::Int(1i64), v_tw.clone(), v_hw.clone(), py::truediv(&v_tw.clone(), &Py::Int(2i64))?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])]))?).items()?;
    if t710.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t710[0].clone();
    v_xb = t710[1].clone();
    v__yc = t710[2].clone();
    v_Ix = t710[3].clone();
    v_Iy = t710[4].clone();
    v__ = t710[5].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("x_bar"), v_xb.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Zx"), py::truediv(&v_Ix.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?), (Py::str("Zy_min"), py::truediv(&v_Iy.clone(), &py::sub(&v_b.clone(), &v_xb.clone())?)?), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circle_section(mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I: Py = Py::None;
    let _ = f__pos(Py::str("the diameter d"), Py::tuple(vec![v_d.clone()]))?;
    v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?, &v_d.clone())?, &Py::Int(4i64))?;
    v_I = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d.clone(), &Py::Int(4i64))?)?, &Py::Int(64i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("I"), v_I.clone()), (Py::str("J"), py::mul(&Py::Int(2i64), &v_I.clone())?), (Py::str("Z"), py::truediv(&v_I.clone(), &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?), (Py::str("Zp"), py::truediv(&py::pow(&v_d.clone(), &Py::Int(3i64))?, &Py::Int(6i64))?), (Py::str("r"), py::truediv(&v_d.clone(), &Py::Int(4i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combined_rect(mut v_b: Py, mut v_h: Py, mut v_N: Py, mut v_Mx: Py, mut v_My: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v_corners: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let _ = f__pos(Py::str("b and h"), Py::tuple(vec![v_b.clone(), v_h.clone()]))?;
    let _ = f__fin(Py::str("N, Mx and My"), Py::tuple(vec![v_N.clone(), v_Mx.clone(), v_My.clone()]))?;
    let t787 = (Py::tuple(vec![py::mul(&v_b.clone(), &v_h.clone())?, py::truediv(&py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, py::truediv(&py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?])).items()?;
    if t787.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t787[0].clone();
    v_Ix = t787[1].clone();
    v_Iy = t787[2].clone();
    v_sig = { let v_A = v_A.clone(); let v_Ix = v_Ix.clone(); let v_Iy = v_Iy.clone(); let v_Mx = v_Mx.clone(); let v_My = v_My.clone(); let v_N = v_N.clone(); let v_b = v_b.clone(); let v_h = v_h.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&py::truediv(&v_N.clone(), &v_A.clone())?, &py::truediv(&py::mul(&v_Mx.clone(), &v_y.clone())?, &v_Ix.clone())?)?, &py::truediv(&py::mul(&v_My.clone(), &v_x.clone())?, &v_Iy.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_sc = py::add(&py::add(&py::truediv(&py::abs(&v_N.clone())?, &v_A.clone())?, &py::truediv(&py::truediv(&py::mul(&py::abs(&v_Mx.clone())?, &v_h.clone())?, &Py::Int(2i64))?, &v_Ix.clone())?)?, &py::truediv(&py::truediv(&py::mul(&py::abs(&v_My.clone())?, &v_b.clone())?, &Py::Int(2i64))?, &v_Iy.clone())?)?;
    if ({ let t788 = v_sc.clone(); Py::Bool({ let t789 = Py::Int(0i64); if py::eq(&t788, &t789) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no load is given"))));
    }
    v_corners = Py::list(vec![py::call(&v_sig.clone(), &[py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])?, py::call(&v_sig.clone(), &[py::truediv(&py::neg(&v_b.clone())?, &Py::Int(2i64))?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])?, py::call(&v_sig.clone(), &[py::truediv(&py::neg(&v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::neg(&v_h.clone())?, &Py::Int(2i64))?])?, py::call(&v_sig.clone(), &[py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&py::neg(&v_h.clone())?, &Py::Int(2i64))?])?]);
    v_corners = { let mut out = vec![]; for t790 in py::iter(&(v_corners.clone()))? { let mut v_v: Py; 
v_v = t790;
out.push(py::truediv(&f__snap(v_v.clone(), v_sc.clone())?, &Py::Float(1000000.0))?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_corners_MPa"), v_corners.clone()), (Py::str("sigma_max_MPa"), py::minmax(&[v_corners.clone()], true)?), (Py::str("sigma_min_MPa"), py::minmax(&[v_corners.clone()], false)?), (Py::str("sigma_axial_MPa"), py::truediv(&py::truediv(&v_N.clone(), &v_A.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_composite_rects(mut v_rects: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Ixy: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v__x: Py = Py::None;
    let mut v__y: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_h1: Py = Py::None;
    let mut v_h2: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_ox: Py = Py::None;
    let mut v_oy: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_size: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let mut v_x2: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let mut v_y1: Py = Py::None;
    let mut v_y2: Py = Py::None;
    let mut v_yc: Py = Py::None;
    v_rows = { let mut out = vec![]; for t825 in py::iter(&(v_rects.clone()))? { let mut v_r: Py; 
v_r = t825;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t826 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t826;
out.push({ let t827 = py::len(&v_r.clone())?; Py::Bool({ let t828 = Py::Int(4i64); if py::ne(&t827, &t828) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each rectangle is a row 'b h xc yc' (width, height, centre x, centre y in mm)"))));
    }
    if ({ let t831 = py::not(&v_rows.clone()); if t831.truthy() { t831 } else { { let t829 = py::len(&v_rows.clone())?; Py::Bool({ let t830 = Py::Int(20i64); if py::gt(&t829, &t830)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 to 20 rectangles"))));
    }
    for t832 in py::iter(&(v_rows.clone()))? {
        let t838 = (t832).items()?;
        if t838.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_b = t838[0].clone();
        v_h = t838[1].clone();
        v__x = t838[2].clone();
        v__y = t838[3].clone();
        let _ = f__pos(Py::str("each rectangle's b and h"), Py::tuple(vec![v_b.clone(), v_h.clone()]))?;
    }
    for t839 in py::range_iter(&[(py::len(&v_rows.clone())?).i()?]) {
        v_i = t839;
        for t840 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_rows.clone())?).i()?]) {
            v_j = t840;
            let t841 = (py::getitem(&v_rows.clone(), &v_i.clone())?).items()?;
            if t841.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_b1 = t841[0].clone();
            v_h1 = t841[1].clone();
            v_x1 = t841[2].clone();
            v_y1 = t841[3].clone();
            let t842 = (py::getitem(&v_rows.clone(), &v_j.clone())?).items()?;
            if t842.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_b2 = t842[0].clone();
            v_h2 = t842[1].clone();
            v_x2 = t842[2].clone();
            v_y2 = t842[3].clone();
            v_ox = py::sub(&py::minmax(&[py::add(&v_x1.clone(), &py::truediv(&v_b1.clone(), &Py::Int(2i64))?)?, py::add(&v_x2.clone(), &py::truediv(&v_b2.clone(), &Py::Int(2i64))?)?], false)?, &py::minmax(&[py::sub(&v_x1.clone(), &py::truediv(&v_b1.clone(), &Py::Int(2i64))?)?, py::sub(&v_x2.clone(), &py::truediv(&v_b2.clone(), &Py::Int(2i64))?)?], true)?)?;
            v_oy = py::sub(&py::minmax(&[py::add(&v_y1.clone(), &py::truediv(&v_h1.clone(), &Py::Int(2i64))?)?, py::add(&v_y2.clone(), &py::truediv(&v_h2.clone(), &Py::Int(2i64))?)?], false)?, &py::minmax(&[py::sub(&v_y1.clone(), &py::truediv(&v_h1.clone(), &Py::Int(2i64))?)?, py::sub(&v_y2.clone(), &py::truediv(&v_h2.clone(), &Py::Int(2i64))?)?], true)?)?;
            if ({ let t847 = { let t843 = v_ox.clone(); Py::Bool({ let t844 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_b1.clone(), v_b2.clone()], true)?)?; if py::gt(&t843, &t844)? { true } else { false } }) }; if !t847.truthy() { t847 } else { { let t845 = v_oy.clone(); Py::Bool({ let t846 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_h1.clone(), v_h2.clone()], true)?)?; if py::gt(&t845, &t846)? { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["rectangles ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?, " and ".to_string(), py::format(&py::add(&v_j.clone(), &Py::Int(1i64))?, "")?, " overlap: the parts of a composite section must not overlap".to_string()].concat()))));
            }
        }
    }
    let t864 = (f__rect_set({ let mut out = vec![]; for t862 in py::iter(&(v_rows.clone()))? { let mut v_b: Py; let mut v_h: Py; let mut v_x: Py; let mut v_y: Py; 
let t863 = (t862).items()?;
if t863.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_b = t863[0].clone();
v_h = t863[1].clone();
v_x = t863[2].clone();
v_y = t863[3].clone();
out.push(Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_h.clone(), v_x.clone(), v_y.clone()])); } Py::list(out) })?).items()?;
    if t864.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t864[0].clone();
    v_xc = t864[1].clone();
    v_yc = t864[2].clone();
    v_Ix = t864[3].clone();
    v_Iy = t864[4].clone();
    v_Ixy = t864[5].clone();
    v_size = py::minmax(&[{ let mut out = vec![]; for t865 in py::iter(&(v_rows.clone()))? { let mut v_b: Py; let mut v_h: Py; let mut v_x: Py; let mut v_y: Py; 
let t866 = (t865).items()?;
if t866.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_b = t866[0].clone();
v_h = t866[1].clone();
v_x = t866[2].clone();
v_y = t866[3].clone();
out.push(py::add(&py::add(&py::add(&py::abs(&v_x.clone())?, &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?, &py::abs(&v_y.clone())?)?, &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?); } Py::list(out) }], true)?;
    let t869 = (Py::tuple(vec![f__snap(v_xc.clone(), v_size.clone())?, f__snap(v_yc.clone(), v_size.clone())?])).items()?;
    if t869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xc = t869[0].clone();
    v_yc = t869[1].clone();
    v_sc = py::minmax(&[v_Ix.clone(), v_Iy.clone()], true)?;
    v_Ixy = f__snap(v_Ixy.clone(), v_sc.clone())?;
    let t875 = (f__principal(v_Ix.clone(), v_Iy.clone(), v_Ixy.clone())?).items()?;
    if t875.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I1 = t875[0].clone();
    v_I2 = t875[1].clone();
    v_phi = t875[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("x_bar"), v_xc.clone()), (Py::str("y_bar"), v_yc.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Ixy"), v_Ixy.clone()), (Py::str("I1"), v_I1.clone()), (Py::str("I2"), v_I2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_continuous_beam(mut v_spans: Py, mut v_w: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_L1: Py = Py::None;
    let mut v_L2: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Mi: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_VL: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_spans.clone())?;
    if ({ let t913 = { let t908 = py::len(&v_w.clone())?; Py::Bool({ let t909 = v_n.clone(); if py::ne(&t908, &t909) { true } else { false } }) }; if t913.truthy() { t913 } else { py::not(&{ let t910 = Py::Int(2i64); Py::Bool({ let t911 = v_n.clone(); if py::le(&t910, &t911)? { { let t912 = Py::Int(6i64); if py::le(&t911, &t912)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 6 spans and one uniform load per span"))));
    }
    for t914 in py::iter(&(v_spans.clone()))? {
        v_v = t914;
        let _ = f__pos(Py::str("every span"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t920 in py::iter(&(v_w.clone()))? {
        v_v = t920;
        let _ = f__nonneg(Py::str("every load"), Py::tuple(vec![v_v.clone()]))?;
    }
    if (py::not(&Py::Bool((v_w.clone()).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no load"))));
    }
    v_k = py::sub(&v_n.clone(), &Py::Int(1i64))?;
    v_A = { let mut out = vec![]; for t926 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v__: Py; 
v__ = t926;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?); } Py::list(out) };
    v_b = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?;
    for t927 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_i = t927;
        let t928 = (Py::tuple(vec![py::getitem(&v_spans.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_spans.clone(), &v_i.clone())?])).items()?;
        if t928.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_L1 = t928[0].clone();
        v_L2 = t928[1].clone();
        v_r = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_r.clone(), py::mul(&Py::Int(2i64), &py::add(&v_L1.clone(), &v_L2.clone())?)?)?;
        if ({ let t929 = v_r.clone(); Py::Bool({ let t930 = Py::Int(0i64); if py::gt(&t929, &t930)? { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &py::sub(&v_r.clone(), &Py::Int(1i64))?, v_L1.clone())?;
        }
        if ({ let t931 = v_r.clone(); Py::Bool({ let t932 = py::sub(&v_k.clone(), &Py::Int(1i64))?; if py::lt(&t931, &t932)? { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &py::add(&v_r.clone(), &Py::Int(1i64))?, v_L2.clone())?;
        }
        py::setitem(&v_b.clone(), &v_r.clone(), py::truediv(&py::neg(&py::add(&py::mul(&py::getitem(&v_w.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::pow(&v_L1.clone(), &Py::Int(3i64))?)?, &py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::pow(&v_L2.clone(), &Py::Int(3i64))?)?)?)?, &Py::Int(4i64))?)?;
    }
    v_Mi = f__gauss_solve(v_A.clone(), v_b.clone())?;
    v_M = py::add(&py::add(&Py::list(vec![Py::Float(0.0)]), &v_Mi.clone())?, &Py::list(vec![Py::Float(0.0)]))?;
    v_R = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    for t948 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t948;
        v_L = py::getitem(&v_spans.clone(), &v_i.clone())?;
        v_VL = py::add(&py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &v_L.clone())?, &Py::Int(2i64))?, &py::truediv(&py::sub(&py::getitem(&v_M.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_M.clone(), &v_i.clone())?)?, &v_L.clone())?)?;
        let t949 = v_R.clone(); let t950 = v_i.clone();
        py::setitem(&t949, &t950, py::iadd(&py::getitem(&t949, &t950)?, &v_VL.clone())?)?;
        let t951 = v_R.clone(); let t952 = py::add(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t951, &t952, py::iadd(&py::getitem(&t951, &t952)?, &py::sub(&py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &v_L.clone())?, &v_VL.clone())?)?)?;
    }
    v_sc = py::mul(&py::minmax(&[v_w.clone()], true)?, &py::pow(&py::minmax(&[v_spans.clone()], true)?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("support_moments_kNm"), { let mut out = vec![]; for t961 in py::iter(&(v_M.clone()))? { let mut v_v: Py; 
v_v = t961;
out.push(f__snap(v_v.clone(), v_sc.clone())?); } Py::list(out) }), (Py::str("reactions_kN"), { let mut out = vec![]; for t964 in py::iter(&(v_R.clone()))? { let mut v_v: Py; 
v_v = t964;
out.push(f__snap(v_v.clone(), py::truediv(&v_sc.clone(), &py::minmax(&[v_spans.clone()], true)?)?)?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_damped_sdof(mut v_m: Py, mut v_c: Py, mut v_k: Py) -> R {
    let mut v_cc: Py = Py::None;
    let mut v_dec: Py = Py::None;
    let mut v_regime: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("m and k"), Py::tuple(vec![v_m.clone(), v_k.clone()]))?;
    let _ = f__nonneg(Py::str("c"), Py::tuple(vec![v_c.clone()]))?;
    v_wn = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    v_cc = py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&v_k.clone(), &v_m.clone())?)?)?;
    v_z = py::truediv(&v_c.clone(), &v_cc.clone())?;
    if ({ let t999 = v_z.clone(); Py::Bool({ let t1000 = Py::Int(1i64); if py::lt(&t999, &t1000)? { true } else { false } }) }).truthy() {
        v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?;
        v_dec = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_z.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?;
        v_regime = Py::str("underdamped");
    } else {
        let t1003 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), (if ({ let t1001 = v_z.clone(); Py::Bool({ let t1002 = Py::Int(1i64); if py::eq(&t1001, &t1002) { true } else { false } }) }).truthy() { Py::str("critically damped") } else { Py::str("overdamped") })])).items()?;
        if t1003.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_wd = t1003[0].clone();
        v_dec = t1003[1].clone();
        v_regime = t1003[2].clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("omega_n"), v_wn.clone()), (Py::str("zeta"), v_z.clone()), (Py::str("omega_d"), v_wd.clone()), (Py::str("f_d"), py::truediv(&v_wd.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("c_critical"), v_cc.clone()), (Py::str("log_decrement"), v_dec.clone()), (Py::str("regime"), v_regime.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_disc_clutch(mut v_F: Py, mut v_f: Py, mut v_D: Py, mut v_d: Py, mut v_model: Py, mut v_pairs: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__pos(Py::str("F, f, D and d"), Py::tuple(vec![v_F.clone(), v_f.clone(), v_D.clone(), v_d.clone()]))?;
    if ({ let t1041 = v_d.clone(); Py::Bool({ let t1042 = v_D.clone(); if py::ge(&t1041, &t1042)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner diameter must be smaller than the outer diameter"))));
    }
    if (py::not(&{ let t1045 = Py::Bool(matches!(v_pairs.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1045.truthy() { t1045 } else { { let t1043 = v_pairs.clone(); Py::Bool({ let t1044 = Py::Int(1i64); if py::ge(&t1043, &t1044)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of friction pairs must be a whole number >= 1"))));
    }
    if ({ let t1046 = v_model.clone(); Py::Bool({ let t1047 = Py::str("uniform wear"); if py::eq(&t1046, &t1047) { true } else { false } }) }).truthy() {
        v_T = py::truediv(&py::mul(&py::mul(&v_F.clone(), &v_f.clone())?, &py::add(&v_D.clone(), &v_d.clone())?)?, &Py::Int(4i64))?;
        v_p = py::truediv(&py::mul(&Py::Int(2i64), &v_F.clone())?, &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?, &py::sub(&v_D.clone(), &v_d.clone())?)?)?;
    } else {
        v_T = py::truediv(&py::mul(&py::mul(&v_F.clone(), &v_f.clone())?, &py::sub(&py::pow(&v_D.clone(), &Py::Int(3i64))?, &py::pow(&v_d.clone(), &Py::Int(3i64))?)?)?, &py::mul(&Py::Int(3i64), &py::sub(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&v_d.clone(), &v_d.clone())?)?)?)?;
        v_p = py::truediv(&py::mul(&Py::Int(4i64), &v_F.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&v_d.clone(), &v_d.clone())?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::mul(&v_T.clone(), &v_pairs.clone())?), (Py::str("p_max_MPa"), py::truediv(&v_p.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dunkerley_shaft(mut v_masses: Py, mut v_positions: Py, mut v_L: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__shaft_check(v_masses.clone(), v_positions.clone(), v_L.clone(), v_E.clone(), v_I.clone())?;
    v_s = py::fsum(&{ let mut out = vec![]; for t1098 in py::iter(&(py::zip(&[v_masses.clone(), v_positions.clone()])?))? { let mut v_m: Py; let mut v_p: Py; 
let t1099 = (t1098).items()?;
if t1099.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t1099[0].clone();
v_p = t1099[1].clone();
out.push(py::truediv(&py::mul(&v_m.clone(), &f__ss_influence(v_p.clone(), v_p.clone(), v_L.clone())?)?, &py::mul(&v_E.clone(), &v_I.clone())?)?); } Py::list(out) })?;
    v_w = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_s.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_cr"), v_w.clone()), (Py::str("N_cr_rpm"), py::mul(&v_w.clone(), &c__RPM()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_euler_column(mut v_E: Py, mut v_I: Py, mut v_L: Py, mut v_ends: Py, mut v_A: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Le: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("E, I and L"), Py::tuple(vec![v_E.clone(), v_I.clone(), v_L.clone()]))?;
    v_K = f__k_factor(v_ends.clone())?;
    v_Le = py::mul(&v_K.clone(), &v_L.clone())?;
    v_P = py::truediv(&py::mul(&py::mul(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_E.clone())?, &v_I.clone())?, &py::pow(&v_Le.clone(), &Py::Int(2i64))?)?;
    v_out = Py::dict(vec![(Py::str("P_cr"), v_P.clone()), (Py::str("P_cr_kN"), py::truediv(&v_P.clone(), &Py::Float(1000.0))?), (Py::str("K"), v_K.clone()), (Py::str("Le"), v_Le.clone())]);
    if ({ let t1210 = v_A.clone(); Py::Bool({ let t1211 = Py::None; if !py::is(&t1210, &t1211) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("A"), Py::tuple(vec![v_A.clone()]))?;
        let _ = { let t1212 = v_out.clone(); py::setitem(&t1212, &Py::str("sigma_cr_MPa"), py::truediv(&py::truediv(&v_P.clone(), &v_A.clone())?, &Py::Float(1000000.0))?)?; py::setitem(&t1212, &Py::str("slenderness"), py::truediv(&v_Le.clone(), &py::math1("sqrt", &py::truediv(&v_I.clone(), &v_A.clone())?)?)?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eurocode_damage(mut v_ranges: Py, mut v_counts: Py, mut v_dsc: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sD: Py = Py::None;
    let mut v_sL: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let _ = f__counts_check(v_ranges.clone(), v_counts.clone())?;
    let _ = f__pos(Py::str("the detail category"), Py::tuple(vec![v_dsc.clone()]))?;
    v_sD = py::mul(&v_dsc.clone(), &py::pow(&py::truediv(&Py::Int(2i64), &Py::Int(5i64))?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?;
    v_sL = py::mul(&v_sD.clone(), &py::pow(&py::truediv(&Py::Float(5000000.0), &Py::Float(100000000.0))?, &py::truediv(&Py::Int(1i64), &Py::Int(5i64))?)?)?;
    v_D = Py::Float(0.0);
    v_terms = Py::list(vec![]);
    for t1255 in py::iter(&(py::zip(&[v_ranges.clone(), v_counts.clone()])?))? {
        let t1260 = (t1255).items()?;
        if t1260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t1260[0].clone();
        v_n = t1260[1].clone();
        if ({ let t1256 = v_s.clone(); Py::Bool({ let t1257 = v_sD.clone(); if py::ge(&t1256, &t1257)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_terms.clone(), "append", &[py::truediv(&v_n.clone(), &py::mul(&Py::Float(2000000.0), &py::pow(&py::truediv(&v_dsc.clone(), &v_s.clone())?, &Py::Int(3i64))?)?)?])?;
        } else {
            if ({ let t1258 = v_s.clone(); Py::Bool({ let t1259 = v_sL.clone(); if py::ge(&t1258, &t1259)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_terms.clone(), "append", &[py::truediv(&v_n.clone(), &py::mul(&Py::Float(5000000.0), &py::pow(&py::truediv(&v_sD.clone(), &v_s.clone())?, &Py::Int(5i64))?)?)?])?;
            }
        }
    }
    v_D = py::fsum(&v_terms.clone())?;
    if ({ let t1261 = v_D.clone(); Py::Bool({ let t1262 = Py::Int(0i64); if py::eq(&t1261, &t1262) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every range is below the cut-off limit: no damage"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("damage"), v_D.clone()), (Py::str("delta_sigma_D"), v_sD.clone()), (Py::str("delta_sigma_L"), v_sL.clone()), (Py::str("repeats_to_failure"), py::truediv(&Py::Int(1i64), &v_D.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exact_whirl(mut v_masses: Py, mut v_positions: Py, mut v_L: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v__v: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__shaft_check(v_masses.clone(), v_positions.clone(), v_L.clone(), v_E.clone(), v_I.clone())?;
    v_A = f__flex(v_positions.clone(), v_L.clone(), py::mul(&v_E.clone(), &v_I.clone())?)?;
    v_s = { let mut out = vec![]; for t3405 in py::iter(&(v_masses.clone()))? { let mut v_m: Py; 
v_m = t3405;
out.push(py::math1("sqrt", &v_m.clone())?); } Py::list(out) };
    v_n = py::len(&v_masses.clone())?;
    v_B = { let mut out = vec![]; for t3406 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3406;
out.push({ let mut out = vec![]; for t3407 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3407;
out.push(py::mul(&py::mul(&py::getitem(&v_s.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_s.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    let t3441 = (f__jacobi_sym(v_B.clone())?).items()?;
    if t3441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t3441[0].clone();
    v__v = t3441[1].clone();
    v_w = py::sorted(&{ let mut out = vec![]; for t3442 in py::iter(&(v_lam.clone()))? { let mut v_l_: Py; 
v_l_ = t3442;
if ({ let t3443 = v_l_.clone(); Py::Bool({ let t3444 = Py::Int(0i64); if py::gt(&t3443, &t3444)? { true } else { false } }) }).truthy() { out.push(py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_l_.clone())?)?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("N_rpm"), { let mut out = vec![]; for t3453 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t3453;
out.push(py::mul(&v_x.clone(), &c__RPM()?)?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ff_point(mut v_L: Py, mut v_P: Py, mut v_a: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_L2: Py = Py::None;
    let mut v_L3: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("L and P"), Py::tuple(vec![v_L.clone(), v_P.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    if (py::not(&{ let t1356 = Py::Int(0i64); Py::Bool({ let t1357 = v_a.clone(); if py::lt(&t1356, &t1357)? { { let t1358 = v_L.clone(); if py::lt(&t1357, &t1358)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load position a must lie strictly between the ends (0 < a < L)"))));
    }
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    let t1359 = (Py::tuple(vec![py::mul(&v_L.clone(), &v_L.clone())?, py::pow(&v_L.clone(), &Py::Int(3i64))?])).items()?;
    if t1359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L2 = t1359[0].clone();
    v_L3 = t1359[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_b.clone())?, &v_b.clone())?, &py::add(&py::mul(&Py::Int(3i64), &v_a.clone())?, &v_b.clone())?)?, &v_L3.clone())?), (Py::str("RB"), py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_a.clone())?, &py::add(&v_a.clone(), &py::mul(&Py::Int(3i64), &v_b.clone())?)?)?, &v_L3.clone())?), (Py::str("MA"), py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_b.clone())?, &v_b.clone())?, &v_L2.clone())?), (Py::str("MB"), py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_a.clone())?, &v_b.clone())?, &v_L2.clone())?), (Py::str("M_load"), py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_P.clone())?, &v_a.clone())?, &v_a.clone())?, &v_b.clone())?, &v_b.clone())?, &v_L3.clone())?), (Py::str("deflection_at_load_mm"), py::mul(&py::truediv(&py::mul(&py::mul(&v_P.clone(), &py::pow(&v_a.clone(), &Py::Int(3i64))?)?, &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_E.clone())?, &v_I.clone())?, &v_L3.clone())?)?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ff_udl(mut v_L: Py, mut v_w: Py, mut v_E: Py, mut v_I: Py) -> R {
    let _ = f__pos(Py::str("L and w"), Py::tuple(vec![v_L.clone(), v_w.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?), (Py::str("RB"), py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?), (Py::str("M_end"), py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &v_L.clone())?, &Py::Int(12i64))?), (Py::str("M_mid"), py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &v_L.clone())?, &Py::Int(24i64))?), (Py::str("max_deflection_mm"), py::mul(&py::truediv(&py::mul(&v_w.clone(), &py::pow(&v_L.clone(), &Py::Int(4i64))?)?, &py::mul(&py::mul(&Py::Int(384i64), &v_E.clone())?, &v_I.clone())?)?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fixed_shaft(mut v_T0: Py, mut v_a: Py, mut v_b: Py, mut v_d1: Py, mut v_d2: Py, mut v_G: Py) -> R {
    let mut v_J1: Py = Py::None;
    let mut v_J2: Py = Py::None;
    let mut v_TA: Py = Py::None;
    let mut v_TB: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let _ = f__pos(Py::str("T0, a, b, d1, d2 and G"), Py::tuple(vec![v_T0.clone(), v_a.clone(), v_b.clone(), v_d1.clone(), v_d2.clone(), v_G.clone()]))?;
    let t3945 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d1.clone(), &Py::Int(4i64))?)?, &Py::Int(32i64))?, py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d2.clone(), &Py::Int(4i64))?)?, &Py::Int(32i64))?])).items()?;
    if t3945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_J1 = t3945[0].clone();
    v_J2 = t3945[1].clone();
    let t3946 = (Py::tuple(vec![py::truediv(&v_a.clone(), &py::mul(&v_G.clone(), &v_J1.clone())?)?, py::truediv(&v_b.clone(), &py::mul(&v_G.clone(), &v_J2.clone())?)?])).items()?;
    if t3946.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t3946[0].clone();
    v_f2 = t3946[1].clone();
    v_TA = py::truediv(&py::mul(&v_T0.clone(), &v_f2.clone())?, &py::add(&v_f1.clone(), &v_f2.clone())?)?;
    v_TB = py::sub(&v_T0.clone(), &v_TA.clone())?;
    v_phi = py::mul(&v_TA.clone(), &v_f1.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("TA"), v_TA.clone()), (Py::str("TB"), v_TB.clone()), (Py::str("tau1_MPa"), py::truediv(&py::truediv(&py::truediv(&py::mul(&v_TA.clone(), &v_d1.clone())?, &Py::Int(2i64))?, &v_J1.clone())?, &Py::Float(1000000.0))?), (Py::str("tau2_MPa"), py::truediv(&py::truediv(&py::truediv(&py::mul(&v_TB.clone(), &v_d2.clone())?, &Py::Int(2i64))?, &v_J2.clone())?, &Py::Float(1000000.0))?), (Py::str("twist_at_load_deg"), py::math1("degrees", &v_phi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forced_response(mut v_m: Py, mut v_c: Py, mut v_k: Py, mut v_F0: Py, mut v_omega: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let _ = f__pos(Py::str("m, k, F0 and omega"), Py::tuple(vec![v_m.clone(), v_k.clone(), v_F0.clone(), v_omega.clone()]))?;
    let _ = f__nonneg(Py::str("c"), Py::tuple(vec![v_c.clone()]))?;
    let t1439 = (Py::tuple(vec![py::sub(&v_k.clone(), &py::mul(&v_m.clone(), &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?)?, py::mul(&v_c.clone(), &v_omega.clone())?])).items()?;
    if t1439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t1439[0].clone();
    v_im = t1439[1].clone();
    if ({ let t1444 = { let t1440 = v_re_.clone(); Py::Bool({ let t1441 = Py::Int(0i64); if py::eq(&t1440, &t1441) { true } else { false } }) }; if !t1444.truthy() { t1444 } else { { let t1442 = v_im.clone(); Py::Bool({ let t1443 = Py::Int(0i64); if py::eq(&t1442, &t1443) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("undamped resonance: the amplitude grows without bound"))));
    }
    v_X = py::truediv(&v_F0.clone(), &py::hypot(&[v_re_.clone(), v_im.clone()])?)?;
    v_wn = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), v_X.clone()), (Py::str("X_mm"), py::mul(&v_X.clone(), &Py::Float(1000.0))?), (Py::str("phase_deg"), py::math1("degrees", &py::atan2(&v_im.clone(), &v_re_.clone())?)?), (Py::str("magnification"), py::truediv(&py::mul(&v_k.clone(), &v_X.clone())?, &v_F0.clone())?), (Py::str("r"), py::truediv(&v_omega.clone(), &v_wn.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_four_bar(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_theta2: Py, mut v_branch: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_K1: Py = Py::None;
    let mut v_K2: Py = Py::None;
    let mut v_K3: Py = Py::None;
    let mut v_K4: Py = Py::None;
    let mut v_K5: Py = Py::None;
    let mut v_ct: Py = Py::None;
    let mut v_deg: Py = Py::None;
    let mut v_disc3: Py = Py::None;
    let mut v_disc4: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_t3: Py = Py::None;
    let mut v_t4: Py = Py::None;
    let _ = f__pos(Py::str("a, b, c and d"), Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]))?;
    let _ = f__fin(Py::str("theta2"), Py::tuple(vec![v_theta2.clone()]))?;
    let t1484 = (Py::tuple(vec![py::truediv(&v_d.clone(), &v_a.clone())?, py::truediv(&v_d.clone(), &v_c.clone())?, py::truediv(&py::add(&py::add(&py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&v_d.clone(), &v_d.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_c.clone())?)?])).items()?;
    if t1484.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K1 = t1484[0].clone();
    v_K2 = t1484[1].clone();
    v_K3 = t1484[2].clone();
    let t1485 = (Py::tuple(vec![py::truediv(&v_d.clone(), &v_b.clone())?, py::truediv(&py::sub(&py::sub(&py::sub(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&v_d.clone(), &v_d.clone())?)?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?)?])).items()?;
    if t1485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_K4 = t1485[0].clone();
    v_K5 = t1485[1].clone();
    let t1486 = (Py::tuple(vec![py::math1("cos", &v_theta2.clone())?, py::math1("sin", &v_theta2.clone())?])).items()?;
    if t1486.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ct = t1486[0].clone();
    v_st = t1486[1].clone();
    v_A = py::add(&py::sub(&py::sub(&v_ct.clone(), &v_K1.clone())?, &py::mul(&v_K2.clone(), &v_ct.clone())?)?, &v_K3.clone())?;
    v_B = py::mul(&py::neg(&Py::Int(2i64))?, &v_st.clone())?;
    v_C = py::add(&py::sub(&v_K1.clone(), &py::mul(&py::add(&v_K2.clone(), &Py::Int(1i64))?, &v_ct.clone())?)?, &v_K3.clone())?;
    v_D = py::add(&py::add(&py::sub(&v_ct.clone(), &v_K1.clone())?, &py::mul(&v_K4.clone(), &v_ct.clone())?)?, &v_K5.clone())?;
    v_E = py::mul(&py::neg(&Py::Int(2i64))?, &v_st.clone())?;
    v_F = py::add(&py::add(&v_K1.clone(), &py::mul(&py::sub(&v_K4.clone(), &Py::Int(1i64))?, &v_ct.clone())?)?, &v_K5.clone())?;
    let t1487 = (Py::tuple(vec![py::sub(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_C.clone())?)?, py::sub(&py::mul(&v_E.clone(), &v_E.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_D.clone())?, &v_F.clone())?)?])).items()?;
    if t1487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_disc4 = t1487[0].clone();
    v_disc3 = t1487[1].clone();
    if ({ let t1492 = { let t1488 = v_disc4.clone(); Py::Bool({ let t1489 = Py::Int(0i64); if py::lt(&t1488, &t1489)? { true } else { false } }) }; if t1492.truthy() { t1492 } else { { let t1490 = v_disc3.clone(); Py::Bool({ let t1491 = Py::Int(0i64); if py::lt(&t1490, &t1491)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the linkage cannot be assembled at this input angle"))));
    }
    v_sg = (if ({ let t1493 = v_branch.clone(); Py::Bool({ let t1494 = Py::str("open"); if py::eq(&t1493, &t1494) { true } else { false } }) }).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) });
    v_t4 = py::mul(&Py::Int(2i64), &py::atan2(&py::add(&py::neg(&v_B.clone())?, &py::mul(&v_sg.clone(), &py::math1("sqrt", &v_disc4.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?)?;
    v_t3 = py::mul(&Py::Int(2i64), &py::atan2(&py::add(&py::neg(&v_E.clone())?, &py::mul(&v_sg.clone(), &py::math1("sqrt", &v_disc3.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_D.clone())?)?)?;
    v_deg = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_D = v_D.clone(); let v_E = v_E.clone(); let v_F = v_F.clone(); let v_K1 = v_K1.clone(); let v_K2 = v_K2.clone(); let v_K3 = v_K3.clone(); let v_K4 = v_K4.clone(); let v_K5 = v_K5.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_branch = v_branch.clone(); let v_c = v_c.clone(); let v_ct = v_ct.clone(); let v_d = v_d.clone(); let v_disc3 = v_disc3.clone(); let v_disc4 = v_disc4.clone(); let v_sg = v_sg.clone(); let v_st = v_st.clone(); let v_t3 = v_t3.clone(); let v_t4 = v_t4.clone(); let v_theta2 = v_theta2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::modulo(&py::add(&py::math1("degrees", &v_t.clone())?, &Py::Float(180.0))?, &Py::Float(360.0))?, &Py::Float(180.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_mu = py::abs(&py::call(&v_deg.clone(), &[py::sub(&v_t4.clone(), &v_t3.clone())?])?)?;
    v_mu = (if ({ let t1495 = v_mu.clone(); Py::Bool({ let t1496 = Py::Int(90i64); if py::gt(&t1495, &t1496)? { true } else { false } }) }).truthy() { py::sub(&Py::Int(180i64), &v_mu.clone())? } else { v_mu.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("theta3_deg"), py::call(&v_deg.clone(), &[v_t3.clone()])?), (Py::str("theta4_deg"), py::call(&v_deg.clone(), &[v_t4.clone()])?), (Py::str("transmission_angle_deg"), v_mu.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gerber(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_Sut: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__fat_check(v_sa.clone(), v_sm.clone(), v_Se.clone(), v_Sut.clone())?;
    if ({ let t1586 = v_sm.clone(); Py::Bool({ let t1587 = Py::Int(0i64); if py::eq(&t1586, &t1587) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("n_fatigue"), py::truediv(&v_Se.clone(), &v_sa.clone())?), (Py::str("sigma_ar_MPa"), py::truediv(&v_sa.clone(), &Py::Float(1000000.0))?)]))?);
    }
    v_r = py::truediv(&v_sa.clone(), &v_sm.clone())?;
    v_x = py::truediv(&py::mul(&Py::Int(2i64), &v_Se.clone())?, &py::mul(&v_Sut.clone(), &v_r.clone())?)?;
    v_n = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &py::pow(&py::truediv(&v_Sut.clone(), &v_sm.clone())?, &Py::Int(2i64))?)?, &py::truediv(&v_sa.clone(), &v_Se.clone())?)?, &v_x.clone())?, &v_x.clone())?, &py::add(&Py::Int(1i64), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n_fatigue"), v_n.clone()), (Py::str("sigma_ar_MPa"), py::truediv(&py::truediv(&v_sa.clone(), &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_sm.clone(), &v_Sut.clone())?, &Py::Int(2i64))?)?)?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_goodman(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_Sut: Py, mut v_Sy: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__fat_check(v_sa.clone(), v_sm.clone(), v_Se.clone(), v_Sut.clone())?;
    v_n = py::truediv(&Py::Float(1.0), &py::add(&py::truediv(&v_sa.clone(), &v_Se.clone())?, &py::truediv(&v_sm.clone(), &v_Sut.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("n_fatigue"), v_n.clone()), (Py::str("sigma_ar_MPa"), py::truediv(&py::truediv(&v_sa.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&v_sm.clone(), &v_Sut.clone())?)?)?, &Py::Float(1000000.0))?)]);
    if ({ let t1632 = v_Sy.clone(); Py::Bool({ let t1633 = Py::None; if !py::is(&t1632, &t1633) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("Sy"), Py::tuple(vec![v_Sy.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("n_yield"), py::truediv(&v_Sy.clone(), &py::add(&v_sa.clone(), &v_sm.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_helical_spring(mut v_d: Py, mut v_D: Py, mut v_Na: Py, mut v_G: Py, mut v_F: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_tau0: Py = Py::None;
    let _ = f__pos(Py::str("d, D, Na, G and F"), Py::tuple(vec![v_d.clone(), v_D.clone(), v_Na.clone(), v_G.clone(), v_F.clone()]))?;
    v_C = py::truediv(&v_D.clone(), &v_d.clone())?;
    if ({ let t1672 = v_C.clone(); Py::Bool({ let t1673 = Py::Int(2i64); if py::le(&t1672, &t1673)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the spring index D/d must exceed 2"))));
    }
    v_k = py::truediv(&py::mul(&v_G.clone(), &py::pow(&v_d.clone(), &Py::Int(4i64))?)?, &py::mul(&py::mul(&Py::Int(8i64), &py::pow(&v_D.clone(), &Py::Int(3i64))?)?, &v_Na.clone())?)?;
    v_tau0 = py::truediv(&py::mul(&py::mul(&Py::Int(8i64), &v_F.clone())?, &v_D.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("index"), v_C.clone()), (Py::str("k"), v_k.clone()), (Py::str("deflection_mm"), py::mul(&py::truediv(&v_F.clone(), &v_k.clone())?, &Py::Float(1000.0))?), (Py::str("tau_torsion_MPa"), py::truediv(&v_tau0.clone(), &Py::Float(1000000.0))?), (Py::str("tau_Ks_MPa"), py::truediv(&py::mul(&v_tau0.clone(), &py::add(&Py::Int(1i64), &py::truediv(&Py::Float(0.5), &v_C.clone())?)?)?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hooke_3d(mut v_sx: Py, mut v_sy: Py, mut v_sz: Py, mut v_txy: Py, mut v_tyz: Py, mut v_tzx: Py, mut v_E: Py, mut v_nu: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_ez: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_ue: Py = Py::None;
    let _ = f__fin(Py::str("every stress"), Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_sz.clone(), v_txy.clone(), v_tyz.clone(), v_tzx.clone()]))?;
    let _ = f__pos(Py::str("E"), Py::tuple(vec![v_E.clone()]))?;
    if (py::not(&{ let t1536 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1537 = v_nu.clone(); if py::lt(&t1536, &t1537)? { { let t1538 = Py::Float(0.5); if py::lt(&t1537, &t1538)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie between -1 and 0.5"))));
    }
    v_G = py::truediv(&v_E.clone(), &py::mul(&Py::Int(2i64), &py::add(&Py::Int(1i64), &v_nu.clone())?)?)?;
    v_sc = py::mul(&py::truediv(&f__stress_scale(Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_sz.clone(), v_txy.clone(), v_tyz.clone(), v_tzx.clone()]))?, &v_E.clone())?, &Py::Float(1000000.0))?;
    v_ue = { let v_E = v_E.clone(); let v_G = v_G.clone(); let v_nu = v_nu.clone(); let v_sc = v_sc.clone(); let v_sx = v_sx.clone(); let v_sy = v_sy.clone(); let v_sz = v_sz.clone(); let v_txy = v_txy.clone(); let v_tyz = v_tyz.clone(); let v_tzx = v_tzx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__snap(py::mul(&v_v.clone(), &Py::Float(1000000.0))?, v_sc.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_ex = py::truediv(&py::sub(&v_sx.clone(), &py::mul(&v_nu.clone(), &py::add(&v_sy.clone(), &v_sz.clone())?)?)?, &v_E.clone())?;
    v_ey = py::truediv(&py::sub(&v_sy.clone(), &py::mul(&v_nu.clone(), &py::add(&v_sz.clone(), &v_sx.clone())?)?)?, &v_E.clone())?;
    v_ez = py::truediv(&py::sub(&v_sz.clone(), &py::mul(&v_nu.clone(), &py::add(&v_sx.clone(), &v_sy.clone())?)?)?, &v_E.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("ex"), py::call(&v_ue.clone(), &[v_ex.clone()])?), (Py::str("ey"), py::call(&v_ue.clone(), &[v_ey.clone()])?), (Py::str("ez"), py::call(&v_ue.clone(), &[v_ez.clone()])?), (Py::str("gxy"), py::call(&v_ue.clone(), &[py::truediv(&v_txy.clone(), &v_G.clone())?])?), (Py::str("gyz"), py::call(&v_ue.clone(), &[py::truediv(&v_tyz.clone(), &v_G.clone())?])?), (Py::str("gzx"), py::call(&v_ue.clone(), &[py::truediv(&v_tzx.clone(), &v_G.clone())?])?), (Py::str("volumetric"), py::call(&v_ue.clone(), &[py::add(&py::add(&v_ex.clone(), &v_ey.clone())?, &v_ez.clone())?])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_i_section(mut v_b: Py, mut v_h: Py, mut v_tf: Py, mut v_tw: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v_Zpx: Py = Py::None;
    let mut v_Zpy: Py = Py::None;
    let mut v_Zx: Py = Py::None;
    let mut v_Zy: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v__xc: Py = Py::None;
    let mut v__yc: Py = Py::None;
    let mut v_hw: Py = Py::None;
    let mut v_rects: Py = Py::None;
    let _ = f__i_check(v_b.clone(), v_h.clone(), v_tf.clone(), v_tw.clone())?;
    v_hw = py::sub(&v_h.clone(), &py::mul(&Py::Int(2i64), &v_tf.clone())?)?;
    v_rects = Py::list(vec![Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_tf.clone(), Py::Float(0.0), py::truediv(&py::sub(&v_h.clone(), &v_tf.clone())?, &Py::Int(2i64))?]), Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_tf.clone(), Py::Float(0.0), py::truediv(&py::neg(&py::sub(&v_h.clone(), &v_tf.clone())?)?, &Py::Int(2i64))?]), Py::tuple(vec![Py::Int(1i64), v_tw.clone(), v_hw.clone(), Py::Float(0.0), Py::Float(0.0)])]);
    let t1858 = (f__rect_set(v_rects.clone())?).items()?;
    if t1858.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1858[0].clone();
    v__xc = t1858[1].clone();
    v__yc = t1858[2].clone();
    v_Ix = t1858[3].clone();
    v_Iy = t1858[4].clone();
    v__ = t1858[5].clone();
    let t1859 = (Py::tuple(vec![py::truediv(&v_Ix.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::truediv(&v_Iy.clone(), &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?])).items()?;
    if t1859.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Zx = t1859[0].clone();
    v_Zy = t1859[1].clone();
    v_Zpx = py::add(&py::mul(&py::mul(&v_b.clone(), &v_tf.clone())?, &py::sub(&v_h.clone(), &v_tf.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_tw.clone(), &v_hw.clone())?, &v_hw.clone())?, &Py::Int(4i64))?)?;
    v_Zpy = py::add(&py::truediv(&py::mul(&py::mul(&v_tf.clone(), &v_b.clone())?, &v_b.clone())?, &Py::Int(2i64))?, &py::truediv(&py::mul(&py::mul(&v_hw.clone(), &v_tw.clone())?, &v_tw.clone())?, &Py::Int(4i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Zx"), v_Zx.clone()), (Py::str("Zy"), v_Zy.clone()), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?), (Py::str("Zpx"), v_Zpx.clone()), (Py::str("Zpy"), v_Zpy.clone()), (Py::str("shape_factor_x"), py::truediv(&v_Zpx.clone(), &v_Zx.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isolator_design(mut v_m: Py, mut v_omega: Py, mut v_TR: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let _ = f__pos(Py::str("m and omega"), Py::tuple(vec![v_m.clone(), v_omega.clone()]))?;
    if (py::not(&{ let t4453 = Py::Int(0i64); Py::Bool({ let t4454 = v_TR.clone(); if py::lt(&t4453, &t4454)? { { let t4455 = Py::Int(1i64); if py::lt(&t4454, &t4455)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the target transmissibility must lie strictly between 0 and 1"))));
    }
    v_r = py::math1("sqrt", &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_TR.clone())?)?)?;
    v_wn = py::truediv(&v_omega.clone(), &v_r.clone())?;
    v_k = py::mul(&v_m.clone(), &py::pow(&v_wn.clone(), &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("f_n"), py::truediv(&v_wn.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("r"), v_r.clone()), (Py::str("static_deflection_mm"), py::mul(&py::truediv(&py::mul(&v_m.clone(), &c_G0()?)?, &v_k.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_johnson_column(mut v_Sy: Py, mut v_E: Py, mut v_L: Py, mut v_r: Py, mut v_ends: Py, mut v_A: Py) -> R {
    let mut v_Cc: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_regime: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let _ = f__pos(Py::str("Sy, E, L and r"), Py::tuple(vec![v_Sy.clone(), v_E.clone(), v_L.clone(), v_r.clone()]))?;
    v_K = f__k_factor(v_ends.clone())?;
    v_lam = py::truediv(&py::mul(&v_K.clone(), &v_L.clone())?, &v_r.clone())?;
    v_Cc = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_E.clone())?, &v_Sy.clone())?)?;
    if ({ let t1907 = v_lam.clone(); Py::Bool({ let t1908 = v_Cc.clone(); if py::lt(&t1907, &t1908)? { true } else { false } }) }).truthy() {
        let t1909 = (Py::tuple(vec![py::sub(&v_Sy.clone(), &py::truediv(&py::pow(&py::truediv(&py::mul(&v_Sy.clone(), &v_lam.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &Py::Int(2i64))?, &v_E.clone())?)?, Py::str("Johnson")])).items()?;
        if t1909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sc = t1909[0].clone();
        v_regime = t1909[1].clone();
    } else {
        let t1910 = (Py::tuple(vec![py::truediv(&py::mul(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_E.clone())?, &py::pow(&v_lam.clone(), &Py::Int(2i64))?)?, Py::str("Euler")])).items()?;
        if t1910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sc = t1910[0].clone();
        v_regime = t1910[1].clone();
    }
    v_out = Py::dict(vec![(Py::str("slenderness"), v_lam.clone()), (Py::str("transition_slenderness"), v_Cc.clone()), (Py::str("regime"), v_regime.clone()), (Py::str("sigma_cr_MPa"), py::truediv(&v_sc.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t1911 = v_A.clone(); Py::Bool({ let t1912 = Py::None; if !py::is(&t1911, &t1912) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("A"), Py::tuple(vec![v_A.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("P_cr_kN"), py::truediv(&py::mul(&v_sc.clone(), &v_A.clone())?, &Py::Float(1000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lame(mut v_pi: Py, mut v_po: Py, mut v_a: Py, mut v_b: Py, mut v_r: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sr: Py = Py::None;
    let mut v_st: Py = Py::None;
    let _ = f__nonneg(Py::str("the pressures"), Py::tuple(vec![v_pi.clone(), v_po.clone()]))?;
    let _ = f__pos(Py::str("a and b"), Py::tuple(vec![v_a.clone(), v_b.clone()]))?;
    if ({ let t4143 = v_a.clone(); Py::Bool({ let t4144 = v_b.clone(); if py::ge(&t4143, &t4144)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner radius a must be smaller than the outer radius b"))));
    }
    if ({ let t4149 = { let t4145 = v_pi.clone(); Py::Bool({ let t4146 = Py::Int(0i64); if py::eq(&t4145, &t4146) { true } else { false } }) }; if !t4149.truthy() { t4149 } else { { let t4147 = v_po.clone(); Py::Bool({ let t4148 = Py::Int(0i64); if py::eq(&t4147, &t4148) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one pressure must be positive"))));
    }
    v_r = (if ({ let t4150 = v_r.clone(); Py::Bool({ let t4151 = Py::None; if py::is(&t4150, &t4151) { true } else { false } }) }).truthy() { v_a.clone() } else { v_r.clone() });
    if (py::not(&{ let t4152 = v_a.clone(); Py::Bool({ let t4153 = v_r.clone(); if py::le(&t4152, &t4153)? { { let t4154 = v_b.clone(); if py::le(&t4153, &t4154)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the radius r must lie in the wall (a <= r <= b)"))));
    }
    v_A = py::truediv(&py::sub(&py::mul(&py::mul(&v_pi.clone(), &v_a.clone())?, &v_a.clone())?, &py::mul(&py::mul(&v_po.clone(), &v_b.clone())?, &v_b.clone())?)?, &py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?)?;
    v_B = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::sub(&v_pi.clone(), &v_po.clone())?, &v_a.clone())?, &v_a.clone())?, &v_b.clone())?, &v_b.clone())?, &py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?)?;
    v_sr = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_pi = v_pi.clone(); let v_po = v_po.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&v_A.clone(), &py::truediv(&v_B.clone(), &py::mul(&v_x.clone(), &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_st = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_pi = v_pi.clone(); let v_po = v_po.clone(); let v_r = v_r.clone(); let v_sr = v_sr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&v_A.clone(), &py::truediv(&v_B.clone(), &py::mul(&v_x.clone(), &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_sc = py::minmax(&[v_pi.clone(), v_po.clone()], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sr_MPa"), py::truediv(&f__snap(py::call(&v_sr.clone(), &[v_r.clone()])?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("st_MPa"), py::truediv(&f__snap(py::call(&v_st.clone(), &[v_r.clone()])?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("st_inner_MPa"), py::truediv(&f__snap(py::call(&v_st.clone(), &[v_a.clone()])?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("st_outer_MPa"), py::truediv(&f__snap(py::call(&v_st.clone(), &[v_b.clone()])?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("tau_max_inner_MPa"), py::truediv(&f__snap(py::truediv(&py::sub(&py::call(&v_st.clone(), &[v_a.clone()])?, &py::call(&v_sr.clone(), &[v_a.clone()])?)?, &Py::Int(2i64))?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("sz_closed_MPa"), py::truediv(&f__snap(v_A.clone(), v_sc.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lewis_stress(mut v_Wt: Py, mut v_F: Py, mut v_m: Py, mut v_Y: Py, mut v_Kv: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("Wt, F, m and Y"), Py::tuple(vec![v_Wt.clone(), v_F.clone(), v_m.clone(), v_Y.clone()]))?;
    if (py::not(&{ let t1953 = py::math1("isfinite", &v_Kv.clone())?; if !t1953.truthy() { t1953 } else { { let t1951 = v_Kv.clone(); Py::Bool({ let t1952 = Py::Int(1i64); if py::ge(&t1951, &t1952)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the velocity factor Kv must be at least 1"))));
    }
    if ({ let t1954 = v_Y.clone(); Py::Bool({ let t1955 = Py::Int(1i64); if py::gt(&t1954, &t1955)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Lewis form factor Y must be at most 1 (based on the module)"))));
    }
    v_s = py::truediv(&py::mul(&v_Kv.clone(), &v_Wt.clone())?, &py::mul(&py::mul(&v_F.clone(), &v_m.clone())?, &v_Y.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma"), v_s.clone()), (Py::str("sigma_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_marin(mut v_Sut: Py, mut v_surface: Py, mut v_load: Py, mut v_d: Py, mut v_R: Py, mut v_kd: Py, mut v_kf: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_Sep: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dm: Py = Py::None;
    let mut v_ka: Py = Py::None;
    let mut v_kb: Py = Py::None;
    let mut v_kc: Py = Py::None;
    let mut v_ke: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("Sut"), Py::tuple(vec![v_Sut.clone()]))?;
    if ({ let t1993 = v_surface.clone(); Py::Bool({ let t1994 = c__MARIN_SURFACE()?; if !py::contains(&t1994, &t1993)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("surface must be ground, machined, cold-drawn, hot-rolled or as-forged"))));
    }
    if ({ let t1995 = v_load.clone(); Py::Bool({ let t1996 = c__MARIN_LOAD()?; if !py::contains(&t1996, &t1995)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("load must be bending, axial or torsion"))));
    }
    if (py::not(&{ let t1997 = Py::Float(0.5); Py::Bool({ let t1998 = v_R.clone(); if py::le(&t1997, &t1998)? { { let t1999 = Py::Int(1i64); if py::lt(&t1998, &t1999)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reliability must lie in [0.5, 1) (e.g. 0.99 or 99%)"))));
    }
    for t2000 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("kd"), v_kd.clone()]), Py::tuple(vec![Py::str("kf"), v_kf.clone()])])))? {
        let t2005 = (t2000).items()?;
        if t2005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t2005[0].clone();
        v_v = t2005[1].clone();
        if (py::not(&{ let t2004 = py::math1("isfinite", &v_v.clone())?; if !t2004.truthy() { t2004 } else { { let t2001 = Py::Int(0i64); Py::Bool({ let t2002 = v_v.clone(); if py::lt(&t2001, &t2002)? { { let t2003 = Py::Float(1.5); if py::le(&t2002, &t2003)? { true } else { false } } } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must lie in (0, 1.5]".to_string()].concat()))));
        }
    }
    v_S = py::truediv(&v_Sut.clone(), &Py::Float(1000000.0))?;
    let t2006 = (py::getitem(&c__MARIN_SURFACE()?, &v_surface.clone())?).items()?;
    if t2006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2006[0].clone();
    v_b = t2006[1].clone();
    v_ka = py::mul(&v_a.clone(), &py::pow(&v_S.clone(), &v_b.clone())?)?;
    if ({ let t2007 = v_load.clone(); Py::Bool({ let t2008 = Py::str("axial"); if py::eq(&t2007, &t2008) { true } else { false } }) }).truthy() {
        v_kb = Py::Float(1.0);
    } else {
        if ({ let t2009 = v_d.clone(); Py::Bool({ let t2010 = Py::None; if py::is(&t2009, &t2010) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the diameter d is needed for bending and torsion (size factor)"))));
        }
        v_dm = py::mul(&v_d.clone(), &Py::Float(1000.0))?;
        if (py::not(&{ let t2011 = Py::Float(2.79); Py::Bool({ let t2012 = v_dm.clone(); if py::le(&t2011, &t2012)? { { let t2013 = Py::Int(254i64); if py::le(&t2012, &t2013)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the size factor covers 2.79 mm <= d <= 254 mm"))));
        }
        v_kb = (if ({ let t2014 = v_dm.clone(); Py::Bool({ let t2015 = Py::Int(51i64); if py::le(&t2014, &t2015)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(1.24), &py::pow(&v_dm.clone(), &py::neg(&Py::Float(0.107))?)?)? } else { py::mul(&Py::Float(1.51), &py::pow(&v_dm.clone(), &py::neg(&Py::Float(0.157))?)?)? });
    }
    v_kc = py::getitem(&c__MARIN_LOAD()?, &v_load.clone())?;
    v_ke = py::sub(&Py::Int(1i64), &py::mul(&Py::Float(0.08), &py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_R.clone()])?)?)?;
    v_Sep = (if ({ let t2016 = v_S.clone(); Py::Bool({ let t2017 = Py::Int(1400i64); if py::le(&t2016, &t2017)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(0.5), &v_S.clone())? } else { Py::Float(700.0) });
    return Ok(f__out(Py::dict(vec![(Py::str("ka"), v_ka.clone()), (Py::str("kb"), v_kb.clone()), (Py::str("kc"), v_kc.clone()), (Py::str("ke"), v_ke.clone()), (Py::str("Se_prime_MPa"), v_Sep.clone()), (Py::str("Se_MPa"), py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&v_ka.clone(), &v_kb.clone())?, &v_kc.clone())?, &v_kd.clone())?, &v_ke.clone())?, &v_kf.clone())?, &v_Sep.clone())?), (Py::str("Se"), py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&v_ka.clone(), &v_kb.clone())?, &v_kc.clone())?, &v_kd.clone())?, &v_ke.clone())?, &v_kf.clone())?, &v_Sep.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mdof_chain(mut v_masses: Py, mut v_springs: Py, mut v_ends: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mode1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_need: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vecs: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_n = py::len(&v_masses.clone())?;
    if (py::not(&{ let t2050 = Py::Int(1i64); Py::Bool({ let t2051 = v_n.clone(); if py::le(&t2050, &t2051)? { { let t2052 = Py::Int(12i64); if py::le(&t2051, &t2052)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 to 12 masses"))));
    }
    v_need = (if ({ let t2053 = v_ends.clone(); Py::Bool({ let t2054 = Py::str("fixed-free"); if py::eq(&t2053, &t2054) { true } else { false } }) }).truthy() { v_n.clone() } else { py::add(&v_n.clone(), &Py::Int(1i64))? });
    if ({ let t2055 = py::len(&v_springs.clone())?; Py::Bool({ let t2056 = v_need.clone(); if py::ne(&t2055, &t2056) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["a ".to_string(), py::format(&v_ends.clone(), "")?, " chain of ".to_string(), py::format(&v_n.clone(), "")?, " masses needs ".to_string(), py::format(&v_need.clone(), "")?, " springs (wall spring first)".to_string()].concat()))));
    }
    for t2057 in py::iter(&(py::add(&Py::list((v_masses.clone()).items()?), &Py::list((v_springs.clone()).items()?))?))? {
        v_v = t2057;
        let _ = f__pos(Py::str("every mass and spring"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_K = f__chain_K(Py::list((v_springs.clone()).items()?), v_n.clone(), v_ends.clone())?;
    v_h = { let mut out = vec![]; for t2079 in py::iter(&(v_masses.clone()))? { let mut v_mi: Py; 
v_mi = t2079;
out.push(py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_mi.clone())?)?); } Py::list(out) };
    v_A = { let mut out = vec![]; for t2080 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2080;
out.push({ let mut out = vec![]; for t2081 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2081;
out.push(py::mul(&py::mul(&py::getitem(&py::getitem(&v_K.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_h.clone(), &v_i.clone())?)?, &py::getitem(&v_h.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    let t2115 = (f__jacobi_sym(v_A.clone())?).items()?;
    if t2115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2115[0].clone();
    v_vecs = t2115[1].clone();
    v_w = { let mut out = vec![]; for t2116 in py::iter(&(v_lam.clone()))? { let mut v_l_: Py; 
v_l_ = t2116;
out.push(py::math1("sqrt", &py::minmax(&[v_l_.clone(), Py::Float(0.0)], true)?)?); } Py::list(out) };
    v_mode1 = f__norm_mode({ let mut out = vec![]; for t2125 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2125;
out.push(py::mul(&py::getitem(&py::getitem(&v_vecs.clone(), &Py::Int(0i64))?, &v_i.clone())?, &py::getitem(&v_h.clone(), &v_i.clone())?)?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("f_Hz"), { let mut out = vec![]; for t2134 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t2134;
out.push(py::truediv(&v_x.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?); } Py::list(out) }), (Py::str("mode1"), v_mode1.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_stress_life(mut v_sa: Py, mut v_sm: Py, mut v_sf: Py, mut v_b: Py, mut v_method: Py, mut v_gamma: Py) -> R {
    let mut v_Nf: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_sar: Py = Py::None;
    let mut v_smax: Py = Py::None;
    let _ = f__pos(Py::str("sa and sf"), Py::tuple(vec![v_sa.clone(), v_sf.clone()]))?;
    let _ = f__fin(Py::str("sm"), Py::tuple(vec![v_sm.clone()]))?;
    if (py::not(&{ let t1302 = py::neg(&Py::Float(0.2))?; Py::Bool({ let t1303 = v_b.clone(); if py::lt(&t1302, &t1303)? { { let t1304 = Py::Int(0i64); if py::lt(&t1303, &t1304)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the fatigue strength exponent b must be negative (typically -0.05 to -0.15)"))));
    }
    v_smax = py::add(&v_sm.clone(), &v_sa.clone())?;
    if ({ let t1305 = v_method.clone(); Py::Bool({ let t1306 = Py::tuple(vec![Py::str("goodman"), Py::str("morrow")]); if py::contains(&t1306, &t1305)? { true } else { false } }) }).truthy() {
        if ({ let t1307 = v_sm.clone(); Py::Bool({ let t1308 = v_sf.clone(); if py::ge(&t1307, &t1308)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the mean stress must be below the strength used by the correction"))));
        }
        v_sar = py::truediv(&v_sa.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&v_sm.clone(), &v_sf.clone())?)?)?;
    } else {
        if ({ let t1309 = v_smax.clone(); Py::Bool({ let t1310 = Py::Int(0i64); if py::le(&t1309, &t1310)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("SWT and Walker are not defined for a maximum stress <= 0"))));
        }
        v_g = (if ({ let t1311 = v_method.clone(); Py::Bool({ let t1312 = Py::str("swt"); if py::eq(&t1311, &t1312) { true } else { false } }) }).truthy() { Py::Float(0.5) } else { v_gamma.clone() });
        if ({ let t1318 = { let t1313 = v_g.clone(); Py::Bool({ let t1314 = Py::None; if py::is(&t1313, &t1314) { true } else { false } }) }; if t1318.truthy() { t1318 } else { py::not(&{ let t1315 = Py::Int(0i64); Py::Bool({ let t1316 = v_g.clone(); if py::lt(&t1315, &t1316)? { { let t1317 = Py::Int(1i64); if py::le(&t1316, &t1317)? { true } else { false } } } else { false } }) }) } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the Walker exponent gamma must lie in (0, 1]"))));
        }
        v_sar = py::mul(&py::pow(&v_smax.clone(), &py::sub(&Py::Int(1i64), &v_g.clone())?)?, &py::pow(&v_sa.clone(), &v_g.clone())?)?;
    }
    v_Nf = py::mul(&Py::Float(0.5), &py::pow(&py::truediv(&v_sar.clone(), &v_sf.clone())?, &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_ar_MPa"), py::truediv(&v_sar.clone(), &Py::Float(1000000.0))?), (Py::str("Nf"), v_Nf.clone()), (Py::str("log10_Nf"), py::math1("log10", &v_Nf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_miner_linear(mut v_ranges: Py, mut v_counts: Py, mut v_Sc: Py, mut v_m: Py, mut v_Nc: Py) -> R {
    let mut v_D: Py = Py::None;
    let _ = f__counts_check(v_ranges.clone(), v_counts.clone())?;
    let _ = f__pos(Py::str("Sc, m and Nc"), Py::tuple(vec![v_Sc.clone(), v_m.clone(), v_Nc.clone()]))?;
    v_D = py::fsum(&{ let mut out = vec![]; for t2168 in py::iter(&(py::zip(&[v_ranges.clone(), v_counts.clone()])?))? { let mut v_n: Py; let mut v_s: Py; 
let t2169 = (t2168).items()?;
if t2169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t2169[0].clone();
v_n = t2169[1].clone();
out.push(py::truediv(&v_n.clone(), &py::mul(&v_Nc.clone(), &py::pow(&py::truediv(&v_Sc.clone(), &v_s.clone())?, &v_m.clone())?)?)?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("damage"), v_D.clone()), (Py::str("repeats_to_failure"), py::truediv(&Py::Int(1i64), &v_D.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moment_of_force(mut v_Fx: Py, mut v_Fy: Py, mut v_x: Py, mut v_y: Py, mut v_x0: Py, mut v_y0: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_rx: Py = Py::None;
    let mut v_ry: Py = Py::None;
    let _ = f__fin(Py::str("every component and coordinate"), Py::tuple(vec![v_Fx.clone(), v_Fy.clone(), v_x.clone(), v_y.clone(), v_x0.clone(), v_y0.clone()]))?;
    v_F = py::hypot(&[v_Fx.clone(), v_Fy.clone()])?;
    if ({ let t2204 = v_F.clone(); Py::Bool({ let t2205 = Py::Int(0i64); if py::eq(&t2204, &t2205) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the force must not be zero"))));
    }
    let t2206 = (Py::tuple(vec![py::sub(&v_x.clone(), &v_x0.clone())?, py::sub(&v_y.clone(), &v_y0.clone())?])).items()?;
    if t2206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t2206[0].clone();
    v_ry = t2206[1].clone();
    v_M = py::sub(&py::mul(&v_rx.clone(), &v_Fy.clone())?, &py::mul(&v_ry.clone(), &v_Fx.clone())?)?;
    v_M = f__snap(v_M.clone(), py::mul(&v_F.clone(), &py::hypot(&[v_rx.clone(), v_ry.clone()])?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("d"), py::truediv(&py::abs(&v_M.clone())?, &v_F.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pc_point(mut v_L: Py, mut v_P: Py, mut v_a: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_EI: Py = Py::None;
    let mut v_MA: Py = Py::None;
    let mut v_Mload: Py = Py::None;
    let mut v_RA: Py = Py::None;
    let mut v_RB: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("L and P"), Py::tuple(vec![v_L.clone(), v_P.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    if (py::not(&{ let t2618 = Py::Int(0i64); Py::Bool({ let t2619 = v_a.clone(); if py::lt(&t2618, &t2619)? { { let t2620 = v_L.clone(); if py::lt(&t2619, &t2620)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load position a must lie strictly between the fixed end and the prop (0 < a < L)"))));
    }
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    v_RB = py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_a.clone())?, &py::sub(&py::mul(&Py::Int(3i64), &v_L.clone())?, &v_a.clone())?)?, &py::mul(&Py::Int(2i64), &py::pow(&v_L.clone(), &Py::Int(3i64))?)?)?;
    v_RA = py::sub(&v_P.clone(), &v_RB.clone())?;
    v_MA = py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_b.clone())?, &py::add(&v_L.clone(), &v_b.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_L.clone())?, &v_L.clone())?)?;
    v_Mload = py::mul(&v_RB.clone(), &v_b.clone())?;
    v_EI = py::mul(&v_E.clone(), &v_I.clone())?;
    v_d = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_P.clone(), &py::pow(&v_a.clone(), &Py::Int(3i64))?)?, &v_b.clone())?, &v_b.clone())?, &py::add(&py::mul(&Py::Int(3i64), &v_L.clone())?, &v_b.clone())?)?, &py::mul(&py::mul(&Py::Int(12i64), &v_EI.clone())?, &py::pow(&v_L.clone(), &Py::Int(3i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), v_RA.clone()), (Py::str("RB"), v_RB.clone()), (Py::str("MA"), v_MA.clone()), (Py::str("M_load"), v_Mload.clone()), (Py::str("deflection_at_load_mm"), py::mul(&v_d.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pc_udl(mut v_L: Py, mut v_w: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_dmax: Py = Py::None;
    let mut v_xm: Py = Py::None;
    let _ = f__pos(Py::str("L and w"), Py::tuple(vec![v_L.clone(), v_w.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    v_xm = py::truediv(&py::mul(&v_L.clone(), &py::sub(&Py::Int(15i64), &py::math1("sqrt", &Py::Int(33i64))?)?)?, &Py::Int(16i64))?;
    v_dmax = py::truediv(&py::mul(&py::mul(&py::truediv(&py::add(&Py::Int(39i64), &py::mul(&Py::Int(55i64), &py::math1("sqrt", &Py::Int(33i64))?)?)?, &Py::Int(65536i64))?, &v_w.clone())?, &py::pow(&v_L.clone(), &Py::Int(4i64))?)?, &py::mul(&v_E.clone(), &v_I.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), py::truediv(&py::mul(&py::mul(&Py::Int(5i64), &v_w.clone())?, &v_L.clone())?, &Py::Int(8i64))?), (Py::str("RB"), py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &v_w.clone())?, &v_L.clone())?, &Py::Int(8i64))?), (Py::str("MA"), py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &v_L.clone())?, &Py::Int(8i64))?), (Py::str("M_span"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(9i64), &v_w.clone())?, &v_L.clone())?, &v_L.clone())?, &Py::Int(128i64))?), (Py::str("x_M_span"), py::truediv(&py::mul(&Py::Int(5i64), &v_L.clone())?, &Py::Int(8i64))?), (Py::str("max_deflection_mm"), py::mul(&v_dmax.clone(), &Py::Float(1000.0))?), (Py::str("x_max"), v_xm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plane_transform(mut v_sx: Py, mut v_sy: Py, mut v_txy: Py, mut v_theta: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__fin(Py::str("every stress and the angle"), Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_txy.clone(), v_theta.clone()]))?;
    let t2243 = (Py::tuple(vec![py::math1("cos", &py::mul(&Py::Int(2i64), &v_theta.clone())?)?, py::math1("sin", &py::mul(&Py::Int(2i64), &v_theta.clone())?)?])).items()?;
    if t2243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c2 = t2243[0].clone();
    v_s2 = t2243[1].clone();
    let t2244 = (Py::tuple(vec![py::truediv(&py::add(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?])).items()?;
    if t2244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2244[0].clone();
    v_d = t2244[1].clone();
    v_a = py::add(&py::add(&v_m.clone(), &py::mul(&v_d.clone(), &v_c2.clone())?)?, &py::mul(&v_txy.clone(), &v_s2.clone())?)?;
    v_b = py::sub(&py::sub(&v_m.clone(), &py::mul(&v_d.clone(), &v_c2.clone())?)?, &py::mul(&v_txy.clone(), &v_s2.clone())?)?;
    v_t = py::add(&py::mul(&py::neg(&v_d.clone())?, &v_s2.clone())?, &py::mul(&v_txy.clone(), &v_c2.clone())?)?;
    v_sc = f__stress_scale(Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_txy.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("sx_t_MPa"), py::truediv(&f__snap(v_a.clone(), v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("sy_t_MPa"), py::truediv(&f__snap(v_b.clone(), v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("txy_t_MPa"), py::truediv(&f__snap(v_t.clone(), v_sc.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_planetary(mut v_Ns: Py, mut v_Nr: Py, mut v_ws: Py, mut v_wr: Py, mut v_wc: Py) -> R {
    let mut v_Np: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_wp: Py = Py::None;
    if (py::not(&{ let t2287 = { let t2280 = v_Ns.clone(); Py::Bool({ let t2281 = Py::Int(0i64); if py::gt(&t2280, &t2281)? { true } else { false } }) }; if !t2287.truthy() { t2287 } else { { let t2286 = { let t2282 = v_Nr.clone(); Py::Bool({ let t2283 = v_Ns.clone(); if py::gt(&t2282, &t2283)? { true } else { false } }) }; if !t2286.truthy() { t2286 } else { { let t2284 = py::modulo(&py::sub(&v_Nr.clone(), &v_Ns.clone())?, &Py::Int(2i64))?; Py::Bool({ let t2285 = Py::Int(0i64); if py::eq(&t2284, &t2285) { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need Nr > Ns > 0 with Nr - Ns even (the planets have (Nr - Ns)/2 teeth)"))));
    }
    v_given = { let mut out = vec![]; for t2288 in py::iter(&(Py::tuple(vec![v_ws.clone(), v_wr.clone(), v_wc.clone()])))? { let mut v_v: Py; 
v_v = t2288;
out.push({ let t2289 = v_v.clone(); Py::Bool({ let t2290 = Py::None; if !py::is(&t2289, &t2290) { true } else { false } }) }); } Py::list(out) };
    if ({ let t2291 = py::sum(&v_given.clone(), None)?; Py::Bool({ let t2292 = Py::Int(2i64); if py::ne(&t2291, &t2292) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly two of the three speeds ws, wr, wc"))));
    }
    for t2293 in py::iter(&(Py::tuple(vec![v_ws.clone(), v_wr.clone(), v_wc.clone()])))? {
        v_v = t2293;
        if ({ let t2294 = v_v.clone(); Py::Bool({ let t2295 = Py::None; if !py::is(&t2294, &t2295) { true } else { false } }) }).truthy() {
            let _ = f__fin(Py::str("every speed"), Py::tuple(vec![v_v.clone()]))?;
        }
    }
    v_k = py::truediv(&v_Nr.clone(), &v_Ns.clone())?;
    if ({ let t2298 = v_ws.clone(); Py::Bool({ let t2299 = Py::None; if py::is(&t2298, &t2299) { true } else { false } }) }).truthy() {
        v_ws = py::sub(&v_wc.clone(), &py::mul(&v_k.clone(), &py::sub(&v_wr.clone(), &v_wc.clone())?)?)?;
    } else {
        if ({ let t2300 = v_wr.clone(); Py::Bool({ let t2301 = Py::None; if py::is(&t2300, &t2301) { true } else { false } }) }).truthy() {
            v_wr = py::sub(&v_wc.clone(), &py::truediv(&py::sub(&v_ws.clone(), &v_wc.clone())?, &v_k.clone())?)?;
        } else {
            v_wc = py::truediv(&py::add(&v_ws.clone(), &py::mul(&v_k.clone(), &v_wr.clone())?)?, &py::add(&Py::Int(1i64), &v_k.clone())?)?;
        }
    }
    v_Np = py::floordiv(&py::sub(&v_Nr.clone(), &v_Ns.clone())?, &Py::Int(2i64))?;
    v_wp = py::sub(&v_wc.clone(), &py::truediv(&py::mul(&py::sub(&v_ws.clone(), &v_wc.clone())?, &v_Ns.clone())?, &v_Np.clone())?)?;
    v_sc = py::minmax(&[py::abs(&v_ws.clone())?, py::abs(&v_wr.clone())?, py::abs(&v_wc.clone())?], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("ws_rpm"), py::mul(&f__snap(v_ws.clone(), v_sc.clone())?, &c__RPM()?)?), (Py::str("wr_rpm"), py::mul(&f__snap(v_wr.clone(), v_sc.clone())?, &c__RPM()?)?), (Py::str("wc_rpm"), py::mul(&f__snap(v_wc.clone(), v_sc.clone())?, &c__RPM()?)?), (Py::str("wp_rpm"), py::mul(&f__snap(v_wp.clone(), v_sc.clone())?, &c__RPM()?)?), (Py::str("Np"), v_Np.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polygon_section(mut v_vertices: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Ixy: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_ixx: Py = Py::None;
    let mut v_ixy: Py = Py::None;
    let mut v_iyy: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    let mut v_size: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let mut v_y1: Py = Py::None;
    v_pts = { let mut out = vec![]; for t2336 in py::iter(&(v_vertices.clone()))? { let mut v_r: Py; 
v_r = t2336;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) };
    if ({ let t2343 = Py::Bool(({ let mut out = vec![]; for t2337 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2337;
out.push({ let t2338 = py::len(&v_p.clone())?; Py::Bool({ let t2339 = Py::Int(2i64); if py::ne(&t2338, &t2339) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2343.truthy() { t2343 } else { py::not(&{ let t2340 = Py::Int(3i64); Py::Bool({ let t2341 = py::len(&v_pts.clone())?; if py::le(&t2340, &t2341)? { { let t2342 = Py::Int(60i64); if py::le(&t2341, &t2342)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 3 to 60 vertices as rows 'x y' (mm)"))));
    }
    v_n = py::len(&v_pts.clone())?;
    if ({ let t2344 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t2345 = v_n.clone(); if py::ne(&t2344, &t2345) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("vertices must be distinct"))));
    }
    for t2346 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2346;
        for t2347 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t2347;
            if ({ let t2355 = { let t2348 = v_j.clone(); Py::Bool({ let t2349 = py::add(&v_i.clone(), &Py::Int(1i64))?; if py::eq(&t2348, &t2349) { true } else { false } }) }; if t2355.truthy() { t2355 } else { { let t2354 = { let t2350 = v_i.clone(); Py::Bool({ let t2351 = Py::Int(0i64); if py::eq(&t2350, &t2351) { true } else { false } }) }; if !t2354.truthy() { t2354 } else { { let t2352 = v_j.clone(); Py::Bool({ let t2353 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t2352, &t2353) { true } else { false } }) } } } } }).truthy() {
                continue;
            }
            if (f__segments_cross(py::getitem(&v_pts.clone(), &v_i.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_pts.clone(), &v_j.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the polygon must be simple (its edges must not cross)"))));
            }
        }
    }
    let t2388 = Py::Float(0.0);
    v_a = t2388.clone();
    v_cx = t2388.clone();
    v_cy = t2388.clone();
    v_ixx = t2388.clone();
    v_iyy = t2388.clone();
    v_ixy = t2388.clone();
    v_terms = Py::list(vec![]);
    for t2389 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2389;
        let t2390 = (py::getitem(&v_pts.clone(), &v_i.clone())?).items()?;
        if t2390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x0 = t2390[0].clone();
        v_y0 = t2390[1].clone();
        let t2391 = (py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?).items()?;
        if t2391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x1 = t2391[0].clone();
        v_y1 = t2391[1].clone();
        v_c = py::sub(&py::mul(&v_x0.clone(), &v_y1.clone())?, &py::mul(&v_x1.clone(), &v_y0.clone())?)?;
        let _ = py::method(&v_terms.clone(), "append", &[Py::tuple(vec![v_c.clone(), v_x0.clone(), v_y0.clone(), v_x1.clone(), v_y1.clone()])])?;
    }
    v_a = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2392 in py::iter(&(v_terms.clone()))? { let mut v__: Py; let mut v_c: Py; 
let t2393 = (t2392).items()?;
if t2393.len() < 1 { return Err(PyErr::Value("unpack".into())); }
v_c = t2393[0].clone();
v__ = Py::list(t2393[1..t2393.len() - 0].to_vec());
out.push(v_c.clone()); } Py::list(out) })?, &Py::Int(2i64))?;
    if ({ let t2394 = py::abs(&v_a.clone())?; Py::Bool({ let t2395 = py::mul(&Py::Float(1e-12), &py::pow(&py::minmax(&[{ let mut out = vec![]; for t2396 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2396;
for t2397 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2397;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?, &Py::Int(2i64))?)?; if py::le(&t2394, &t2395)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polygon has no area"))));
    }
    v_cx = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2398 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2399 = (t2398).items()?;
if t2399.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2399[0].clone();
v_x0 = t2399[1].clone();
v_y0 = t2399[2].clone();
v_x1 = t2399[3].clone();
v_y1 = t2399[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&v_x0.clone(), &v_x1.clone())?)?); } Py::list(out) })?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?;
    v_cy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2400 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2401 = (t2400).items()?;
if t2401.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2401[0].clone();
v_x0 = t2401[1].clone();
v_y0 = t2401[2].clone();
v_x1 = t2401[3].clone();
v_y1 = t2401[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&v_y0.clone(), &v_y1.clone())?)?); } Py::list(out) })?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?;
    v_ixx = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2402 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2403 = (t2402).items()?;
if t2403.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2403[0].clone();
v_x0 = t2403[1].clone();
v_y0 = t2403[2].clone();
v_x1 = t2403[3].clone();
v_y1 = t2403[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::add(&py::mul(&v_y0.clone(), &v_y0.clone())?, &py::mul(&v_y0.clone(), &v_y1.clone())?)?, &py::mul(&v_y1.clone(), &v_y1.clone())?)?)?); } Py::list(out) })?, &Py::Int(12i64))?;
    v_iyy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2404 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2405 = (t2404).items()?;
if t2405.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2405[0].clone();
v_x0 = t2405[1].clone();
v_y0 = t2405[2].clone();
v_x1 = t2405[3].clone();
v_y1 = t2405[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::add(&py::mul(&v_x0.clone(), &v_x0.clone())?, &py::mul(&v_x0.clone(), &v_x1.clone())?)?, &py::mul(&v_x1.clone(), &v_x1.clone())?)?)?); } Py::list(out) })?, &Py::Int(12i64))?;
    v_ixy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2406 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2407 = (t2406).items()?;
if t2407.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2407[0].clone();
v_x0 = t2407[1].clone();
v_y0 = t2407[2].clone();
v_x1 = t2407[3].clone();
v_y1 = t2407[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::add(&py::add(&py::mul(&v_x0.clone(), &v_y1.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_x0.clone())?, &v_y0.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_x1.clone())?, &v_y1.clone())?)?, &py::mul(&v_x1.clone(), &v_y0.clone())?)?)?); } Py::list(out) })?, &Py::Int(24i64))?;
    v_sgn = (if ({ let t2408 = v_a.clone(); Py::Bool({ let t2409 = Py::Int(0i64); if py::gt(&t2408, &t2409)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_A = py::abs(&v_a.clone())?;
    v_Ix = py::sub(&py::mul(&v_sgn.clone(), &v_ixx.clone())?, &py::mul(&py::mul(&v_A.clone(), &v_cy.clone())?, &v_cy.clone())?)?;
    v_Iy = py::sub(&py::mul(&v_sgn.clone(), &v_iyy.clone())?, &py::mul(&py::mul(&v_A.clone(), &v_cx.clone())?, &v_cx.clone())?)?;
    v_size = py::minmax(&[{ let mut out = vec![]; for t2410 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2410;
for t2411 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2411;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?;
    let t2414 = (Py::tuple(vec![f__snap(v_cx.clone(), v_size.clone())?, f__snap(v_cy.clone(), v_size.clone())?])).items()?;
    if t2414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2414[0].clone();
    v_cy = t2414[1].clone();
    v_Ixy = f__snap(py::sub(&py::mul(&v_sgn.clone(), &v_ixy.clone())?, &py::mul(&py::mul(&v_A.clone(), &v_cx.clone())?, &v_cy.clone())?)?, py::minmax(&[v_Ix.clone(), v_Iy.clone()], true)?)?;
    let t2420 = (f__principal(v_Ix.clone(), v_Iy.clone(), v_Ixy.clone())?).items()?;
    if t2420.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I1 = t2420[0].clone();
    v_I2 = t2420[1].clone();
    v_phi = t2420[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("x_bar"), v_cx.clone()), (Py::str("y_bar"), v_cy.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Ixy"), v_Ixy.clone()), (Py::str("I1"), v_I1.clone()), (Py::str("I2"), v_I2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_screw(mut v_F: Py, mut v_dm: Py, mut v_l: Py, mut v_f: Py, mut v_alpha: Py, mut v_fc: Py, mut v_dc: Py) -> R {
    let mut v_TL_thread: Py = Py::None;
    let mut v_TR: Py = Py::None;
    let mut v_collar: Py = Py::None;
    let mut v_sec: Py = Py::None;
    let _ = f__pos(Py::str("F, dm, l and f"), Py::tuple(vec![v_F.clone(), v_dm.clone(), v_l.clone(), v_f.clone()]))?;
    let _ = f__nonneg(Py::str("alpha and fc"), Py::tuple(vec![v_alpha.clone(), v_fc.clone()]))?;
    if ({ let t2463 = v_alpha.clone(); Py::Bool({ let t2464 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?; if py::ge(&t2463, &t2464)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the thread half-angle must be below 45 degrees"))));
    }
    if ({ let t2469 = { let t2465 = v_fc.clone(); Py::Bool({ let t2466 = Py::Int(0i64); if py::gt(&t2465, &t2466)? { true } else { false } }) }; if !t2469.truthy() { t2469 } else { { let t2467 = v_dc.clone(); Py::Bool({ let t2468 = Py::None; if py::is(&t2467, &t2468) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a collar friction coefficient needs the collar mean diameter dc"))));
    }
    v_sec = py::truediv(&Py::Int(1i64), &py::math1("cos", &v_alpha.clone())?)?;
    v_collar = py::truediv(&py::mul(&py::mul(&v_F.clone(), &v_fc.clone())?, &{ let t2470 = v_dc.clone(); if t2470.truthy() { t2470 } else { Py::Float(0.0) } })?, &Py::Int(2i64))?;
    v_TR = py::add(&py::truediv(&py::mul(&py::truediv(&py::mul(&v_F.clone(), &v_dm.clone())?, &Py::Int(2i64))?, &py::add(&v_l.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_f.clone())?, &v_dm.clone())?, &v_sec.clone())?)?)?, &py::sub(&py::mul(&Py::Float(std::f64::consts::PI), &v_dm.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_l.clone())?, &v_sec.clone())?)?)?, &v_collar.clone())?;
    if ({ let t2471 = py::sub(&py::mul(&Py::Float(std::f64::consts::PI), &v_dm.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_l.clone())?, &v_sec.clone())?)?; Py::Bool({ let t2472 = Py::Int(0i64); if py::le(&t2471, &t2472)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the lead is too steep for this friction: the screw cannot raise the load"))));
    }
    v_TL_thread = py::truediv(&py::mul(&py::truediv(&py::mul(&v_F.clone(), &v_dm.clone())?, &Py::Int(2i64))?, &py::sub(&py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_f.clone())?, &v_dm.clone())?, &v_sec.clone())?, &v_l.clone())?)?, &py::add(&py::mul(&Py::Float(std::f64::consts::PI), &v_dm.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_l.clone())?, &v_sec.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_raise"), v_TR.clone()), (Py::str("T_lower"), py::add(&v_TL_thread.clone(), &v_collar.clone())?), (Py::str("efficiency"), py::truediv(&py::mul(&v_F.clone(), &v_l.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_TR.clone())?)?), (Py::str("self_locking"), (if ({ let t2481 = v_TL_thread.clone(); Py::Bool({ let t2482 = Py::Int(0i64); if py::gt(&t2481, &t2482)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_principal_2d(mut v_sx: Py, mut v_sy: Py, mut v_txy: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_tabs: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_vm: Py = Py::None;
    let _ = f__fin(Py::str("every stress"), Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_txy.clone()]))?;
    v_sc = f__stress_scale(Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_txy.clone()]))?;
    let t2510 = (Py::tuple(vec![py::truediv(&py::add(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, py::hypot(&[py::truediv(&py::sub(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, v_txy.clone()])?])).items()?;
    if t2510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2510[0].clone();
    v_R = t2510[1].clone();
    let t2513 = (Py::tuple(vec![f__snap(py::add(&v_c.clone(), &v_R.clone())?, v_sc.clone())?, f__snap(py::sub(&v_c.clone(), &v_R.clone())?, v_sc.clone())?])).items()?;
    if t2513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t2513[0].clone();
    v_s2 = t2513[1].clone();
    v_th = (if ({ let t2514 = v_R.clone(); Py::Bool({ let t2515 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t2514, &t2515)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&py::mul(&Py::Int(2i64), &v_txy.clone())?, &py::sub(&v_sx.clone(), &v_sy.clone())?)?)?)? });
    if ({ let t2516 = v_th.clone(); Py::Bool({ let t2517 = py::neg(&Py::Int(90i64))?; if py::le(&t2516, &t2517)? { true } else { false } }) }).truthy() {
        v_th = py::iadd(&v_th, &Py::Int(180i64))?;
    }
    v_tabs = py::truediv(&py::minmax(&[py::sub(&v_s1.clone(), &v_s2.clone())?, py::abs(&v_s1.clone())?, py::abs(&v_s2.clone())?], true)?, &Py::Int(2i64))?;
    v_vm = py::math1("sqrt", &py::minmax(&[py::add(&py::sub(&py::mul(&v_s1.clone(), &v_s1.clone())?, &py::mul(&v_s1.clone(), &v_s2.clone())?)?, &py::mul(&v_s2.clone(), &v_s2.clone())?)?, Py::Float(0.0)], true)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("s1_MPa"), py::truediv(&v_s1.clone(), &Py::Float(1000000.0))?), (Py::str("s2_MPa"), py::truediv(&v_s2.clone(), &Py::Float(1000000.0))?), (Py::str("tau_max_inplane_MPa"), py::truediv(&f__snap(v_R.clone(), v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("theta_p_deg"), v_th.clone()), (Py::str("tau_abs_max_MPa"), py::truediv(&v_tabs.clone(), &Py::Float(1000000.0))?), (Py::str("von_mises_MPa"), py::truediv(&f__snap(v_vm.clone(), v_sc.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_principal_3d(mut v_sx: Py, mut v_sy: Py, mut v_sz: Py, mut v_txy: Py, mut v_tyz: Py, mut v_tzx: Py) -> R {
    let mut v_I1: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_s3: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_vm: Py = Py::None;
    let _ = f__fin(Py::str("every stress component"), Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_sz.clone(), v_txy.clone(), v_tyz.clone(), v_tzx.clone()]))?;
    v_sc = f__stress_scale(Py::tuple(vec![v_sx.clone(), v_sy.clone(), v_sz.clone(), v_txy.clone(), v_tyz.clone(), v_tzx.clone()]))?;
    v_S = Py::list(vec![Py::list(vec![v_sx.clone(), v_txy.clone(), v_tzx.clone()]), Py::list(vec![v_txy.clone(), v_sy.clone(), v_tyz.clone()]), Py::list(vec![v_tzx.clone(), v_tyz.clone(), v_sz.clone()])]);
    let t4611 = ({ let mut out = vec![]; for t4608 in py::iter(&(f__jacobi3(v_S.clone())?))? { let mut v_v: Py; 
v_v = t4608;
out.push(f__snap(v_v.clone(), v_sc.clone())?); } Py::list(out) }).items()?;
    if t4611.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s3 = t4611[0].clone();
    v_s2 = t4611[1].clone();
    v_s1 = t4611[2].clone();
    v_vm = py::math1("sqrt", &py::minmax(&[py::add(&py::mul(&Py::Float(0.5), &py::add(&py::add(&py::pow(&py::sub(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_sy.clone(), &v_sz.clone())?, &Py::Int(2i64))?)?, &py::pow(&py::sub(&v_sz.clone(), &v_sx.clone())?, &Py::Int(2i64))?)?)?, &py::mul(&Py::Int(3i64), &py::add(&py::add(&py::mul(&v_txy.clone(), &v_txy.clone())?, &py::mul(&v_tyz.clone(), &v_tyz.clone())?)?, &py::mul(&v_tzx.clone(), &v_tzx.clone())?)?)?)?, Py::Float(0.0)], true)?)?;
    v_I1 = py::add(&py::add(&v_sx.clone(), &v_sy.clone())?, &v_sz.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("s1_MPa"), py::truediv(&v_s1.clone(), &Py::Float(1000000.0))?), (Py::str("s2_MPa"), py::truediv(&v_s2.clone(), &Py::Float(1000000.0))?), (Py::str("s3_MPa"), py::truediv(&v_s3.clone(), &Py::Float(1000000.0))?), (Py::str("tau_max_MPa"), py::truediv(&f__snap(py::truediv(&py::sub(&v_s1.clone(), &v_s3.clone())?, &Py::Int(2i64))?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("von_mises_MPa"), py::truediv(&f__snap(v_vm.clone(), v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("hydrostatic_MPa"), py::truediv(&f__snap(py::truediv(&v_I1.clone(), &Py::Int(3i64))?, v_sc.clone())?, &Py::Float(1000000.0))?), (Py::str("tau_oct_MPa"), py::truediv(&f__snap(py::mul(&py::truediv(&py::math1("sqrt", &Py::Int(2i64))?, &Py::Int(3i64))?, &v_vm.clone())?, v_sc.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rainflow_count(mut v_history: Py) -> R {
    let mut v__m: Py = Py::None;
    let mut v_agg: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_keys: Py = Py::None;
    let mut v_rng_: Py = Py::None;
    let _ = f__hist_check(v_history.clone())?;
    v_agg = Py::dict(vec![]);
    for t2713 in py::iter(&(f__rainflow(Py::list((v_history.clone()).items()?))?))? {
        let t2722 = (t2713).items()?;
        if t2722.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_rng_ = t2722[0].clone();
        v__m = t2722[1].clone();
        v_c = t2722[2].clone();
        if ({ let t2714 = v_rng_.clone(); Py::Bool({ let t2715 = Py::Int(0i64); if py::gt(&t2714, &t2715)? { true } else { false } }) }).truthy() {
            v_k = f__r(v_rng_.clone())?;
            py::setitem(&v_agg.clone(), &v_k.clone(), py::add(&py::method(&v_agg.clone(), "get", &[v_k.clone(), Py::Float(0.0)])?, &v_c.clone())?)?;
        }
    }
    v_keys = py::sorted(&v_agg.clone(), (Py::Bool(true)).truthy())?;
    return Ok(f__out(Py::dict(vec![(Py::str("ranges"), v_keys.clone()), (Py::str("counts"), { let mut out = vec![]; for t2725 in py::iter(&(v_keys.clone()))? { let mut v_k: Py; 
v_k = t2725;
out.push(py::getitem(&v_agg.clone(), &v_k.clone())?); } Py::list(out) }), (Py::str("total_cycles"), py::sum(&py::method(&v_agg.clone(), "values", &[])?, None)?), (Py::str("max_range"), py::getitem(&v_keys.clone(), &Py::Int(0i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rainflow_damage(mut v_history: Py, mut v_Sc: Py, mut v_m: Py, mut v_Nc: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_cyc: Py = Py::None;
    let _ = f__hist_check(v_history.clone())?;
    let _ = f__pos(Py::str("Sc, m and Nc"), Py::tuple(vec![v_Sc.clone(), v_m.clone(), v_Nc.clone()]))?;
    v_cyc = { let mut out = vec![]; for t2778 in py::iter(&(f__rainflow(Py::list((v_history.clone()).items()?))?))? { let mut v__m: Py; let mut v_c: Py; let mut v_r_: Py; 
let t2779 = (t2778).items()?;
if t2779.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_r_ = t2779[0].clone();
v__m = t2779[1].clone();
v_c = t2779[2].clone();
if ({ let t2780 = v_r_.clone(); Py::Bool({ let t2781 = Py::Int(0i64); if py::gt(&t2780, &t2781)? { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_r_.clone(), v_c.clone()])); }} Py::list(out) };
    v_D = py::fsum(&{ let mut out = vec![]; for t2782 in py::iter(&(v_cyc.clone()))? { let mut v_c: Py; let mut v_r_: Py; 
let t2783 = (t2782).items()?;
if t2783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r_ = t2783[0].clone();
v_c = t2783[1].clone();
out.push(py::truediv(&py::mul(&v_c.clone(), &py::pow(&py::truediv(&v_r_.clone(), &v_Sc.clone())?, &v_m.clone())?)?, &v_Nc.clone())?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("damage"), v_D.clone()), (Py::str("repeats_to_failure"), py::truediv(&Py::Int(1i64), &v_D.clone())?), (Py::str("cycles"), py::sum(&{ let mut out = vec![]; for t2792 in py::iter(&(v_cyc.clone()))? { let mut v__r_: Py; let mut v_c: Py; 
let t2793 = (t2792).items()?;
if t2793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__r_ = t2793[0].clone();
v_c = t2793[1].clone();
out.push(v_c.clone()); } Py::list(out) }, None)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rayleigh_shaft(mut v_masses: Py, mut v_positions: Py, mut v_L: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__shaft_check(v_masses.clone(), v_positions.clone(), v_L.clone(), v_E.clone(), v_I.clone())?;
    v_A = f__flex(v_positions.clone(), v_L.clone(), py::mul(&v_E.clone(), &v_I.clone())?)?;
    v_y = { let mut out = vec![]; for t2840 in py::range_iter(&[(py::len(&v_masses.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2840;
out.push(py::mul(&c_G0()?, &py::fsum(&{ let mut out = vec![]; for t2841 in py::range_iter(&[(py::len(&v_masses.clone())?).i()?]) { let mut v_j: Py; 
v_j = t2841;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_masses.clone(), &v_j.clone())?)?); } Py::list(out) })?)?); } Py::list(out) };
    v_w = py::math1("sqrt", &py::truediv(&py::mul(&c_G0()?, &py::fsum(&{ let mut out = vec![]; for t2842 in py::iter(&(py::zip(&[v_masses.clone(), v_y.clone()])?))? { let mut v_m: Py; let mut v_yi: Py; 
let t2843 = (t2842).items()?;
if t2843.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t2843[0].clone();
v_yi = t2843[1].clone();
out.push(py::mul(&v_m.clone(), &v_yi.clone())?); } Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t2844 in py::iter(&(py::zip(&[v_masses.clone(), v_y.clone()])?))? { let mut v_m: Py; let mut v_yi: Py; 
let t2845 = (t2844).items()?;
if t2845.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t2845[0].clone();
v_yi = t2845[1].clone();
out.push(py::mul(&py::mul(&v_m.clone(), &v_yi.clone())?, &v_yi.clone())?); } Py::list(out) })?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_cr"), v_w.clone()), (Py::str("N_cr_rpm"), py::mul(&v_w.clone(), &c__RPM()?)?), (Py::str("deflections_mm"), { let mut out = vec![]; for t2854 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t2854;
out.push(py::mul(&v_v.clone(), &Py::Float(1000.0))?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rect_section(mut v_b: Py, mut v_h: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let _ = f__pos(Py::str("b and h"), Py::tuple(vec![v_b.clone(), v_h.clone()]))?;
    v_A = py::mul(&v_b.clone(), &v_h.clone())?;
    let t2884 = (Py::tuple(vec![py::truediv(&py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, py::truediv(&py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?])).items()?;
    if t2884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Ix = t2884[0].clone();
    v_Iy = t2884[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Zx"), py::truediv(&v_Ix.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?), (Py::str("Zy"), py::truediv(&v_Iy.clone(), &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?), (Py::str("J_polar"), py::add(&v_Ix.clone(), &v_Iy.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_resultant_forces(mut v_F: Py, mut v_theta: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_Rx: Py = Py::None;
    let mut v_Ry: Py = Py::None;
    let mut v_scale: Py = Py::None;
    if ({ let t2917 = py::len(&v_F.clone())?; Py::Bool({ let t2918 = py::len(&v_theta.clone())?; if py::ne(&t2917, &t2918) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F and theta must have the same number of entries"))));
    }
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("each force and angle")); v.extend((v_F.clone()).items()?); v.extend((v_theta.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_fin() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_fin() got an unexpected keyword argument".into())) } f__fin(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    if (Py::Bool(({ let mut out = vec![]; for t2921 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t2921;
out.push({ let t2922 = v_f.clone(); Py::Bool({ let t2923 = Py::Int(0i64); if py::lt(&t2922, &t2923)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("force magnitudes must be zero or positive (give the direction by the angle)"))));
    }
    v_scale = py::sum(&v_F.clone(), None)?;
    if ({ let t2924 = v_scale.clone(); Py::Bool({ let t2925 = Py::Int(0i64); if py::le(&t2924, &t2925)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one force must be non-zero"))));
    }
    v_Rx = f__snap(py::fsum(&{ let mut out = vec![]; for t2928 in py::iter(&(py::zip(&[v_F.clone(), v_theta.clone()])?))? { let mut v_f: Py; let mut v_t: Py; 
let t2929 = (t2928).items()?;
if t2929.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t2929[0].clone();
v_t = t2929[1].clone();
out.push(py::mul(&v_f.clone(), &f__cosd(v_t.clone())?)?); } Py::list(out) })?, v_scale.clone())?;
    v_Ry = f__snap(py::fsum(&{ let mut out = vec![]; for t2930 in py::iter(&(py::zip(&[v_F.clone(), v_theta.clone()])?))? { let mut v_f: Py; let mut v_t: Py; 
let t2931 = (t2930).items()?;
if t2931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t2931[0].clone();
v_t = t2931[1].clone();
out.push(py::mul(&v_f.clone(), &f__sind(v_t.clone())?)?); } Py::list(out) })?, v_scale.clone())?;
    v_R = py::hypot(&[v_Rx.clone(), v_Ry.clone()])?;
    if ({ let t2932 = v_R.clone(); Py::Bool({ let t2933 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2932, &t2933)? { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("R"), Py::Float(0.0)), (Py::str("Rx"), Py::Float(0.0)), (Py::str("Ry"), Py::Float(0.0)), (Py::str("direction_deg"), Py::Float(0.0))]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("Rx"), v_Rx.clone()), (Py::str("Ry"), v_Ry.clone()), (Py::str("direction_deg"), py::math1("degrees", &py::atan2(&v_Ry.clone(), &v_Rx.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rosette(mut v_ea: Py, mut v_eb: Py, mut v_ec: Py, mut v_E: Py, mut v_nu: Py, mut v_layout: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_gxy: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__fin(Py::str("the three gauge strains"), Py::tuple(vec![v_ea.clone(), v_eb.clone(), v_ec.clone()]))?;
    let _ = f__pos(Py::str("E"), Py::tuple(vec![v_E.clone()]))?;
    if (py::not(&{ let t4029 = Py::Int(0i64); Py::Bool({ let t4030 = v_nu.clone(); if py::le(&t4029, &t4030)? { { let t4031 = Py::Float(0.5); if py::lt(&t4030, &t4031)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie in [0, 0.5)"))));
    }
    if ({ let t4032 = v_layout.clone(); Py::Bool({ let t4033 = Py::str("rectangular"); if py::eq(&t4032, &t4033) { true } else { false } }) }).truthy() {
        let t4034 = (Py::tuple(vec![v_ea.clone(), v_ec.clone(), py::sub(&py::sub(&py::mul(&Py::Int(2i64), &v_eb.clone())?, &v_ea.clone())?, &v_ec.clone())?])).items()?;
        if t4034.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t4034[0].clone();
        v_ey = t4034[1].clone();
        v_gxy = t4034[2].clone();
    } else {
        let t4035 = (Py::tuple(vec![v_ea.clone(), py::truediv(&py::sub(&py::add(&py::mul(&Py::Int(2i64), &v_eb.clone())?, &py::mul(&Py::Int(2i64), &v_ec.clone())?)?, &v_ea.clone())?, &Py::Int(3i64))?, py::truediv(&py::mul(&Py::Int(2i64), &py::sub(&v_eb.clone(), &v_ec.clone())?)?, &py::math1("sqrt", &Py::Int(3i64))?)?])).items()?;
        if t4035.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t4035[0].clone();
        v_ey = t4035[1].clone();
        v_gxy = t4035[2].clone();
    }
    v_sc = { let t4036 = py::minmax(&[py::abs(&v_ea.clone())?, py::abs(&v_eb.clone())?, py::abs(&v_ec.clone())?], true)?; if t4036.truthy() { t4036 } else { Py::Float(1.0) } };
    let t4037 = (Py::tuple(vec![py::truediv(&py::add(&v_ex.clone(), &v_ey.clone())?, &Py::Int(2i64))?, py::hypot(&[py::truediv(&py::sub(&v_ex.clone(), &v_ey.clone())?, &Py::Int(2i64))?, py::truediv(&v_gxy.clone(), &Py::Int(2i64))?])?])).items()?;
    if t4037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t4037[0].clone();
    v_R = t4037[1].clone();
    let t4040 = (Py::tuple(vec![f__snap(py::add(&v_c.clone(), &v_R.clone())?, v_sc.clone())?, f__snap(py::sub(&v_c.clone(), &v_R.clone())?, v_sc.clone())?])).items()?;
    if t4040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e1 = t4040[0].clone();
    v_e2 = t4040[1].clone();
    v_th = (if ({ let t4041 = v_R.clone(); Py::Bool({ let t4042 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t4041, &t4042)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&v_gxy.clone(), &py::sub(&v_ex.clone(), &v_ey.clone())?)?)?)? });
    if ({ let t4043 = v_th.clone(); Py::Bool({ let t4044 = py::neg(&Py::Int(90i64))?; if py::le(&t4043, &t4044)? { true } else { false } }) }).truthy() {
        v_th = py::iadd(&v_th, &Py::Int(180i64))?;
    }
    v_f = py::truediv(&py::mul(&py::truediv(&v_E.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_nu.clone(), &v_nu.clone())?)?)?, &Py::Float(1e-06))?, &Py::Float(1000000.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ex"), f__snap(v_ex.clone(), v_sc.clone())?), (Py::str("ey"), f__snap(v_ey.clone(), v_sc.clone())?), (Py::str("gxy"), f__snap(v_gxy.clone(), v_sc.clone())?), (Py::str("e1"), v_e1.clone()), (Py::str("e2"), v_e2.clone()), (Py::str("theta_p_deg"), v_th.clone()), (Py::str("s1_MPa"), f__snap(py::mul(&v_f.clone(), &py::add(&v_e1.clone(), &py::mul(&v_nu.clone(), &v_e2.clone())?)?)?, py::mul(&v_sc.clone(), &v_f.clone())?)?), (Py::str("s2_MPa"), f__snap(py::mul(&v_f.clone(), &py::add(&v_e2.clone(), &py::mul(&v_nu.clone(), &v_e1.clone())?)?)?, py::mul(&v_sc.clone(), &v_f.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rotating_disk(mut v_rho: Py, mut v_omega: Py, mut v_nu: Py, mut v_ro: Py, mut v_ri: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_r_sr: Py = Py::None;
    let mut v_sr_max: Py = Py::None;
    let mut v_st_max: Py = Py::None;
    let mut v_st_rim: Py = Py::None;
    let _ = f__pos(Py::str("rho, omega and ro"), Py::tuple(vec![v_rho.clone(), v_omega.clone(), v_ro.clone()]))?;
    let _ = f__nonneg(Py::str("ri"), Py::tuple(vec![v_ri.clone()]))?;
    if (py::not(&{ let t2976 = Py::Int(0i64); Py::Bool({ let t2977 = v_nu.clone(); if py::le(&t2976, &t2977)? { { let t2978 = Py::Float(0.5); if py::lt(&t2977, &t2978)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie in [0, 0.5)"))));
    }
    if ({ let t2979 = v_ri.clone(); Py::Bool({ let t2980 = v_ro.clone(); if py::ge(&t2979, &t2980)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the bore radius must be smaller than the outer radius"))));
    }
    v_k = py::mul(&py::mul(&py::truediv(&py::add(&Py::Int(3i64), &v_nu.clone())?, &Py::Int(8i64))?, &v_rho.clone())?, &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?;
    if ({ let t2981 = v_ri.clone(); Py::Bool({ let t2982 = Py::Int(0i64); if py::eq(&t2981, &t2982) { true } else { false } }) }).truthy() {
        let t2983 = py::mul(&py::mul(&v_k.clone(), &v_ro.clone())?, &v_ro.clone())?;
        v_st_max = t2983.clone();
        v_sr_max = t2983.clone();
        v_r_sr = Py::Float(0.0);
    } else {
        v_st_max = py::mul(&py::mul(&py::mul(&py::truediv(&py::add(&Py::Int(3i64), &v_nu.clone())?, &Py::Int(4i64))?, &v_rho.clone())?, &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?, &py::add(&py::mul(&v_ro.clone(), &v_ro.clone())?, &py::mul(&py::mul(&py::truediv(&py::sub(&Py::Int(1i64), &v_nu.clone())?, &py::add(&Py::Int(3i64), &v_nu.clone())?)?, &v_ri.clone())?, &v_ri.clone())?)?)?;
        v_sr_max = py::mul(&v_k.clone(), &py::pow(&py::sub(&v_ro.clone(), &v_ri.clone())?, &Py::Int(2i64))?)?;
        v_r_sr = py::math1("sqrt", &py::mul(&v_ri.clone(), &v_ro.clone())?)?;
    }
    v_st_rim = py::mul(&py::mul(&py::mul(&py::truediv(&py::add(&Py::Int(3i64), &v_nu.clone())?, &Py::Int(4i64))?, &v_rho.clone())?, &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?, &py::add(&py::mul(&v_ri.clone(), &v_ri.clone())?, &py::mul(&py::mul(&py::truediv(&py::sub(&Py::Int(1i64), &v_nu.clone())?, &py::add(&Py::Int(3i64), &v_nu.clone())?)?, &v_ro.clone())?, &v_ro.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("hoop_max_MPa"), py::truediv(&v_st_max.clone(), &Py::Float(1000000.0))?), (Py::str("radial_max_MPa"), py::truediv(&v_sr_max.clone(), &Py::Float(1000000.0))?), (Py::str("r_radial_max"), v_r_sr.clone()), (Py::str("hoop_rim_MPa"), py::truediv(&v_st_rim.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sdof_free(mut v_m: Py, mut v_c: Py, mut v_k: Py, mut v_x0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("m and k"), Py::tuple(vec![v_m.clone(), v_k.clone()]))?;
    let _ = f__nonneg(Py::str("c and t"), Py::tuple(vec![v_c.clone(), v_t.clone()]))?;
    let _ = f__fin(Py::str("x0 and v0"), Py::tuple(vec![v_x0.clone(), v_v0.clone()]))?;
    v_wn = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    v_z = py::truediv(&v_c.clone(), &py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&v_k.clone(), &v_m.clone())?)?)?)?;
    if ({ let t3074 = py::abs(&py::sub(&v_z.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t3075 = Py::Float(1e-09); if py::lt(&t3074, &t3075)? { true } else { false } }) }).truthy() {
        v_e = py::math1("exp", &py::mul(&py::neg(&v_wn.clone())?, &v_t.clone())?)?;
        v_x = py::mul(&py::add(&v_x0.clone(), &py::mul(&py::add(&v_v0.clone(), &py::mul(&v_wn.clone(), &v_x0.clone())?)?, &v_t.clone())?)?, &v_e.clone())?;
        v_v = py::mul(&py::sub(&v_v0.clone(), &py::mul(&py::mul(&v_wn.clone(), &py::add(&v_v0.clone(), &py::mul(&v_wn.clone(), &v_x0.clone())?)?)?, &v_t.clone())?)?, &v_e.clone())?;
    } else {
        if ({ let t3076 = v_z.clone(); Py::Bool({ let t3077 = Py::Int(1i64); if py::lt(&t3076, &t3077)? { true } else { false } }) }).truthy() {
            v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?;
            v_B = py::truediv(&py::add(&v_v0.clone(), &py::mul(&py::mul(&v_z.clone(), &v_wn.clone())?, &v_x0.clone())?)?, &v_wd.clone())?;
            v_e = py::math1("exp", &py::mul(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &v_t.clone())?)?;
            let t3078 = (Py::tuple(vec![py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?])).items()?;
            if t3078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cs = t3078[0].clone();
            v_sn = t3078[1].clone();
            v_x = py::mul(&v_e.clone(), &py::add(&py::mul(&v_x0.clone(), &v_cs.clone())?, &py::mul(&v_B.clone(), &v_sn.clone())?)?)?;
            v_v = py::mul(&v_e.clone(), &py::add(&py::mul(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &py::add(&py::mul(&v_x0.clone(), &v_cs.clone())?, &py::mul(&v_B.clone(), &v_sn.clone())?)?)?, &py::mul(&v_wd.clone(), &py::add(&py::mul(&py::neg(&v_x0.clone())?, &v_sn.clone())?, &py::mul(&v_B.clone(), &v_cs.clone())?)?)?)?)?;
        } else {
            v_q = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&py::mul(&v_z.clone(), &v_z.clone())?, &Py::Int(1i64))?)?)?;
            let t3079 = (Py::tuple(vec![py::add(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &v_q.clone())?, py::sub(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &v_q.clone())?])).items()?;
            if t3079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s1 = t3079[0].clone();
            v_s2 = t3079[1].clone();
            let t3080 = (Py::tuple(vec![py::truediv(&py::sub(&v_v0.clone(), &py::mul(&v_s2.clone(), &v_x0.clone())?)?, &py::sub(&v_s1.clone(), &v_s2.clone())?)?, py::truediv(&py::sub(&py::mul(&v_s1.clone(), &v_x0.clone())?, &v_v0.clone())?, &py::sub(&v_s1.clone(), &v_s2.clone())?)?])).items()?;
            if t3080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_A1 = t3080[0].clone();
            v_A2 = t3080[1].clone();
            v_x = py::add(&py::mul(&v_A1.clone(), &py::math1("exp", &py::mul(&v_s1.clone(), &v_t.clone())?)?)?, &py::mul(&v_A2.clone(), &py::math1("exp", &py::mul(&v_s2.clone(), &v_t.clone())?)?)?)?;
            v_v = py::add(&py::mul(&py::mul(&v_A1.clone(), &v_s1.clone())?, &py::math1("exp", &py::mul(&v_s1.clone(), &v_t.clone())?)?)?, &py::mul(&py::mul(&v_A2.clone(), &v_s2.clone())?, &py::math1("exp", &py::mul(&v_s2.clone(), &v_t.clone())?)?)?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("x_mm"), py::mul(&v_x.clone(), &Py::Float(1000.0))?), (Py::str("v"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sdof_natural(mut v_m: Py, mut v_k: Py) -> R {
    let mut v_wn: Py = Py::None;
    let _ = f__pos(Py::str("m and k"), Py::tuple(vec![v_m.clone(), v_k.clone()]))?;
    v_wn = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_n"), v_wn.clone()), (Py::str("f_n"), py::truediv(&v_wn.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("period"), py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_wn.clone())?), (Py::str("static_deflection_mm"), py::mul(&py::truediv(&py::mul(&v_m.clone(), &c_G0()?)?, &v_k.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_secant_column(mut v_P: Py, mut v_e: Py, mut v_L: Py, mut v_A: Py, mut v_I: Py, mut v_c: Py, mut v_E: Py) -> R {
    let mut v_Pcr: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_sec: Py = Py::None;
    let mut v_smax: Py = Py::None;
    let _ = f__pos(Py::str("P, L, A, I, c and E"), Py::tuple(vec![v_P.clone(), v_L.clone(), v_A.clone(), v_I.clone(), v_c.clone(), v_E.clone()]))?;
    let _ = f__nonneg(Py::str("the eccentricity e"), Py::tuple(vec![v_e.clone()]))?;
    v_Pcr = py::truediv(&py::mul(&py::mul(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_E.clone())?, &v_I.clone())?, &py::pow(&v_L.clone(), &Py::Int(2i64))?)?;
    if ({ let t3160 = v_P.clone(); Py::Bool({ let t3161 = v_Pcr.clone(); if py::ge(&t3160, &t3161)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load reaches the Euler load of the pinned column: no equilibrium"))));
    }
    v_k = py::math1("sqrt", &py::truediv(&v_P.clone(), &py::mul(&v_E.clone(), &v_I.clone())?)?)?;
    v_sec = py::truediv(&Py::Int(1i64), &py::math1("cos", &py::truediv(&py::mul(&v_k.clone(), &v_L.clone())?, &Py::Int(2i64))?)?)?;
    v_smax = py::mul(&py::truediv(&v_P.clone(), &v_A.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::truediv(&py::mul(&v_e.clone(), &v_c.clone())?, &py::truediv(&v_I.clone(), &v_A.clone())?)?, &v_sec.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_max_MPa"), py::truediv(&v_smax.clone(), &Py::Float(1000000.0))?), (Py::str("deflection_mm"), py::mul(&py::mul(&v_e.clone(), &py::sub(&v_sec.clone(), &Py::Int(1i64))?)?, &Py::Float(1000.0))?), (Py::str("M_max"), py::mul(&py::mul(&v_P.clone(), &v_e.clone())?, &v_sec.clone())?), (Py::str("P_over_Pcr"), py::truediv(&v_P.clone(), &v_Pcr.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shaft_de_goodman_d(mut v_n: Py, mut v_Ma: Py, mut v_Mm: Py, mut v_Ta: Py, mut v_Tm: Py, mut v_Kf: Py, mut v_Kfs: Py, mut v_Se: Py, mut v_Sut: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__de_check(v_Ma.clone(), v_Mm.clone(), v_Ta.clone(), v_Tm.clone(), v_Kf.clone(), v_Kfs.clone(), v_Se.clone(), v_Sut.clone())?;
    if (py::not(&{ let t3254 = v_n.clone(); Py::Bool({ let t3255 = Py::Int(1i64); if py::ge(&t3254, &t3255)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the design factor n must be at least 1"))));
    }
    v_A = py::math1("sqrt", &py::add(&py::mul(&Py::Int(4i64), &py::pow(&py::mul(&v_Kf.clone(), &v_Ma.clone())?, &Py::Int(2i64))?)?, &py::mul(&Py::Int(3i64), &py::pow(&py::mul(&v_Kfs.clone(), &v_Ta.clone())?, &Py::Int(2i64))?)?)?)?;
    v_B = py::math1("sqrt", &py::add(&py::mul(&Py::Int(4i64), &py::pow(&py::mul(&v_Kf.clone(), &v_Mm.clone())?, &Py::Int(2i64))?)?, &py::mul(&Py::Int(3i64), &py::pow(&py::mul(&v_Kfs.clone(), &v_Tm.clone())?, &Py::Int(2i64))?)?)?)?;
    v_d = py::pow(&py::mul(&py::truediv(&py::mul(&Py::Int(16i64), &v_n.clone())?, &Py::Float(std::f64::consts::PI))?, &py::add(&py::truediv(&v_A.clone(), &v_Se.clone())?, &py::truediv(&v_B.clone(), &v_Sut.clone())?)?)?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d_mm"), py::mul(&v_d.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shaft_de_goodman_n(mut v_d: Py, mut v_Ma: Py, mut v_Mm: Py, mut v_Ta: Py, mut v_Tm: Py, mut v_Kf: Py, mut v_Kfs: Py, mut v_Se: Py, mut v_Sut: Py, mut v_Sy: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_sa: Py = Py::None;
    let mut v_sm: Py = Py::None;
    let mut v_smax: Py = Py::None;
    let _ = f__pos(Py::str("d"), Py::tuple(vec![v_d.clone()]))?;
    let _ = f__de_check(v_Ma.clone(), v_Mm.clone(), v_Ta.clone(), v_Tm.clone(), v_Kf.clone(), v_Kfs.clone(), v_Se.clone(), v_Sut.clone())?;
    let t3308 = (f__de_stresses(v_d.clone(), v_Ma.clone(), v_Mm.clone(), v_Ta.clone(), v_Tm.clone(), v_Kf.clone(), v_Kfs.clone())?).items()?;
    if t3308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sa = t3308[0].clone();
    v_sm = t3308[1].clone();
    if ({ let t3309 = v_sm.clone(); Py::Bool({ let t3310 = v_Sut.clone(); if py::ge(&t3309, &t3310)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mean von Mises stress reaches Sut"))));
    }
    v_out = Py::dict(vec![(Py::str("n_fatigue"), py::truediv(&Py::Int(1i64), &py::add(&py::truediv(&v_sa.clone(), &v_Se.clone())?, &py::truediv(&v_sm.clone(), &v_Sut.clone())?)?)?), (Py::str("sigma_a_vm_MPa"), py::truediv(&v_sa.clone(), &Py::Float(1000000.0))?), (Py::str("sigma_m_vm_MPa"), py::truediv(&v_sm.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t3311 = v_Sy.clone(); Py::Bool({ let t3312 = Py::None; if !py::is(&t3311, &t3312) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("Sy"), Py::tuple(vec![v_Sy.clone()]))?;
        v_smax = py::math1("sqrt", &py::add(&py::pow(&py::truediv(&py::mul(&py::mul(&Py::Int(32i64), &v_Kf.clone())?, &py::add(&v_Mm.clone(), &v_Ma.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?)?, &Py::Int(2i64))?, &py::mul(&Py::Int(3i64), &py::pow(&py::truediv(&py::mul(&py::mul(&Py::Int(16i64), &v_Kfs.clone())?, &py::add(&v_Tm.clone(), &v_Ta.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?)?, &Py::Int(2i64))?)?)?)?;
        py::setitem(&v_out.clone(), &Py::str("n_yield"), py::truediv(&v_Sy.clone(), &v_smax.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shaft_power(mut v_P: Py, mut v_n: Py, mut v_tau_allow: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("P, n and the allowable shear stress"), Py::tuple(vec![v_P.clone(), v_n.clone(), v_tau_allow.clone()]))?;
    v_T = py::truediv(&v_P.clone(), &v_n.clone())?;
    v_d = py::pow(&py::truediv(&py::mul(&Py::Int(16i64), &v_T.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &v_tau_allow.clone())?)?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("d_min_mm"), py::mul(&v_d.clone(), &Py::Float(1000.0))?), (Py::str("frequency_Hz"), py::truediv(&v_n.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shaft_torsion(mut v_T: Py, mut v_d: Py, mut v_L: Py, mut v_G: Py, mut v_d_i: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("T, d, L and G"), Py::tuple(vec![v_T.clone(), v_d.clone(), v_L.clone(), v_G.clone()]))?;
    let _ = f__nonneg(Py::str("the inner diameter"), Py::tuple(vec![v_d_i.clone()]))?;
    if ({ let t3852 = v_d_i.clone(); Py::Bool({ let t3853 = v_d.clone(); if py::ge(&t3852, &t3853)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner diameter must be smaller than the outer diameter"))));
    }
    v_J = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&py::pow(&v_d.clone(), &Py::Int(4i64))?, &py::pow(&v_d_i.clone(), &Py::Int(4i64))?)?)?, &Py::Int(32i64))?;
    v_tau = py::truediv(&py::mul(&v_T.clone(), &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?, &v_J.clone())?;
    v_phi = py::truediv(&py::mul(&v_T.clone(), &v_L.clone())?, &py::mul(&v_G.clone(), &v_J.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("tau_max_MPa"), py::truediv(&v_tau.clone(), &Py::Float(1000000.0))?), (Py::str("tau_inner_MPa"), py::truediv(&py::truediv(&py::mul(&v_tau.clone(), &v_d_i.clone())?, &v_d.clone())?, &Py::Float(1000000.0))?), (Py::str("twist_rad"), v_phi.clone()), (Py::str("twist_deg"), py::math1("degrees", &v_phi.clone())?), (Py::str("J"), v_J.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shrink_fit(mut v_delta: Py, mut v_R: Py, mut v_ro: Py, mut v_E: Py, mut v_ri: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_st_hub: Py = Py::None;
    let mut v_st_shaft: Py = Py::None;
    let _ = f__pos(Py::str("delta, R, ro and E"), Py::tuple(vec![v_delta.clone(), v_R.clone(), v_ro.clone(), v_E.clone()]))?;
    let _ = f__nonneg(Py::str("ri"), Py::tuple(vec![v_ri.clone()]))?;
    if (py::not(&{ let t3488 = v_ri.clone(); Py::Bool({ let t3489 = v_R.clone(); if py::lt(&t3488, &t3489)? { { let t3490 = v_ro.clone(); if py::lt(&t3489, &t3490)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need ri < R < ro (shaft bore < interface < hub outer radius)"))));
    }
    v_p = py::truediv(&py::mul(&py::mul(&py::truediv(&py::mul(&v_E.clone(), &v_delta.clone())?, &py::mul(&Py::Int(2i64), &py::pow(&v_R.clone(), &Py::Int(3i64))?)?)?, &py::sub(&py::mul(&v_ro.clone(), &v_ro.clone())?, &py::mul(&v_R.clone(), &v_R.clone())?)?)?, &py::sub(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_ri.clone(), &v_ri.clone())?)?)?, &py::sub(&py::mul(&v_ro.clone(), &v_ro.clone())?, &py::mul(&v_ri.clone(), &v_ri.clone())?)?)?;
    v_st_hub = py::truediv(&py::mul(&v_p.clone(), &py::add(&py::mul(&v_ro.clone(), &v_ro.clone())?, &py::mul(&v_R.clone(), &v_R.clone())?)?)?, &py::sub(&py::mul(&v_ro.clone(), &v_ro.clone())?, &py::mul(&v_R.clone(), &v_R.clone())?)?)?;
    v_st_shaft = py::truediv(&py::mul(&py::neg(&v_p.clone())?, &py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_ri.clone(), &v_ri.clone())?)?)?, &py::sub(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_ri.clone(), &v_ri.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("p_MPa"), py::truediv(&v_p.clone(), &Py::Float(1000000.0))?), (Py::str("hub_hoop_MPa"), py::truediv(&v_st_hub.clone(), &Py::Float(1000000.0))?), (Py::str("shaft_hoop_MPa"), py::truediv(&v_st_shaft.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_disk(mut v_m: Py, mut v_E: Py, mut v_I: Py, mut v_L: Py, mut v_a: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("m, E, I and L"), Py::tuple(vec![v_m.clone(), v_E.clone(), v_I.clone(), v_L.clone()]))?;
    if (py::not(&{ let t3199 = Py::Int(0i64); Py::Bool({ let t3200 = v_a.clone(); if py::lt(&t3199, &t3200)? { { let t3201 = v_L.clone(); if py::lt(&t3200, &t3201)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the disk must sit between the bearings (0 < a < L)"))));
    }
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    v_k = py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(3i64), &v_E.clone())?, &v_I.clone())?, &v_L.clone())?, &py::mul(&py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &v_b.clone())?, &v_b.clone())?)?;
    v_w = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("omega_cr"), v_w.clone()), (Py::str("N_cr_rpm"), py::mul(&v_w.clone(), &c__RPM()?)?), (Py::str("static_deflection_mm"), py::mul(&py::truediv(&py::mul(&v_m.clone(), &c_G0()?)?, &v_k.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_slider_crank(mut v_r: Py, mut v_l: Py, mut v_theta: Py, mut v_omega: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d2x: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_root: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("r and l"), Py::tuple(vec![v_r.clone(), v_l.clone()]))?;
    let _ = f__fin(Py::str("theta and omega"), Py::tuple(vec![v_theta.clone(), v_omega.clone()]))?;
    if ({ let t3668 = v_l.clone(); Py::Bool({ let t3669 = v_r.clone(); if py::le(&t3668, &t3669)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the connecting rod must be longer than the crank"))));
    }
    let t3670 = (Py::tuple(vec![py::math1("sin", &v_theta.clone())?, py::math1("cos", &v_theta.clone())?])).items()?;
    if t3670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3670[0].clone();
    v_c = t3670[1].clone();
    v_root = py::math1("sqrt", &py::sub(&py::mul(&v_l.clone(), &v_l.clone())?, &py::mul(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_s.clone())?, &v_s.clone())?)?)?;
    v_x = py::add(&py::mul(&v_r.clone(), &v_c.clone())?, &v_root.clone())?;
    v_dx = py::sub(&py::mul(&py::neg(&v_r.clone())?, &v_s.clone())?, &py::truediv(&py::mul(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_s.clone())?, &v_c.clone())?, &v_root.clone())?)?;
    v_d2x = py::sub(&py::sub(&py::mul(&py::neg(&v_r.clone())?, &v_c.clone())?, &py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &py::sub(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&v_s.clone(), &v_s.clone())?)?)?, &v_root.clone())?)?, &py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::pow(&v_r.clone(), &Py::Int(4i64))?, &v_s.clone())?, &v_s.clone())?, &v_c.clone())?, &v_c.clone())?, &py::pow(&v_root.clone(), &Py::Int(3i64))?)?)?;
    v_sc = py::mul(&v_r.clone(), &py::pow(&py::minmax(&[py::abs(&v_omega.clone())?, Py::Float(1e-300)], true)?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("v"), f__snap(py::mul(&v_dx.clone(), &v_omega.clone())?, py::mul(&v_r.clone(), &py::abs(&v_omega.clone())?)?)?), (Py::str("a"), f__snap(py::mul(&py::mul(&v_d2x.clone(), &v_omega.clone())?, &v_omega.clone())?, v_sc.clone())?), (Py::str("rod_angle_deg"), f__snap(py::math1("degrees", &py::math1("asin", &py::truediv(&py::mul(&v_r.clone(), &v_s.clone())?, &v_l.clone())?)?)?, Py::Float(90.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sn_life(mut v_Sut: Py, mut v_Se: Py, mut v_f: Py, mut v_sa: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("Sut, Se and sa"), Py::tuple(vec![v_Sut.clone(), v_Se.clone(), v_sa.clone()]))?;
    if ({ let t3715 = py::not(&{ let t3710 = Py::Int(0i64); Py::Bool({ let t3711 = v_f.clone(); if py::lt(&t3710, &t3711)? { { let t3712 = Py::Int(1i64); if py::lt(&t3711, &t3712)? { true } else { false } } } else { false } }) }); if t3715.truthy() { t3715 } else { { let t3713 = py::mul(&v_f.clone(), &v_Sut.clone())?; Py::Bool({ let t3714 = v_Se.clone(); if py::le(&t3713, &t3714)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 0 < f < 1 and f*Sut > Se"))));
    }
    if (py::not(&{ let t3716 = v_Se.clone(); Py::Bool({ let t3717 = v_sa.clone(); if py::lt(&t3716, &t3717)? { { let t3718 = py::mul(&v_f.clone(), &v_Sut.clone())?; if py::lt(&t3717, &t3718)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amplitude must lie between Se (infinite life) and f*Sut (low-cycle range below 1000 cycles)"))));
    }
    v_a = py::truediv(&py::pow(&py::mul(&v_f.clone(), &v_Sut.clone())?, &Py::Int(2i64))?, &v_Se.clone())?;
    v_b = py::truediv(&py::neg(&py::math1("log10", &py::truediv(&py::mul(&v_f.clone(), &v_Sut.clone())?, &v_Se.clone())?)?)?, &Py::Int(3i64))?;
    v_N = py::pow(&py::truediv(&v_sa.clone(), &v_a.clone())?, &py::truediv(&Py::Int(1i64), &v_b.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_MPa"), py::truediv(&v_a.clone(), &Py::Float(1000000.0))?), (Py::str("b"), v_b.clone()), (Py::str("N"), v_N.clone()), (Py::str("log10_N"), py::math1("log10", &v_N.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sn_strength(mut v_Sut: Py, mut v_Se: Py, mut v_f: Py, mut v_N: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("Sut and Se"), Py::tuple(vec![v_Sut.clone(), v_Se.clone()]))?;
    if ({ let t3761 = py::not(&{ let t3756 = Py::Int(0i64); Py::Bool({ let t3757 = v_f.clone(); if py::lt(&t3756, &t3757)? { { let t3758 = Py::Int(1i64); if py::lt(&t3757, &t3758)? { true } else { false } } } else { false } }) }); if t3761.truthy() { t3761 } else { { let t3759 = py::mul(&v_f.clone(), &v_Sut.clone())?; Py::Bool({ let t3760 = v_Se.clone(); if py::le(&t3759, &t3760)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 0 < f < 1 and f*Sut > Se"))));
    }
    if (py::not(&{ let t3762 = Py::Float(1000.0); Py::Bool({ let t3763 = v_N.clone(); if py::le(&t3762, &t3763)? { { let t3764 = Py::Float(1000000.0); if py::le(&t3763, &t3764)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("N must lie between 10^3 and 10^6 cycles"))));
    }
    v_a = py::truediv(&py::pow(&py::mul(&v_f.clone(), &v_Sut.clone())?, &Py::Int(2i64))?, &v_Se.clone())?;
    v_b = py::truediv(&py::neg(&py::math1("log10", &py::truediv(&py::mul(&v_f.clone(), &v_Sut.clone())?, &v_Se.clone())?)?)?, &Py::Int(3i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Sf_MPa"), py::truediv(&py::mul(&v_a.clone(), &py::pow(&v_N.clone(), &v_b.clone())?)?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_soderberg(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_Sy: Py) -> R {
    let _ = f__fat_check(v_sa.clone(), v_sm.clone(), v_Se.clone(), v_Sy.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("n_fatigue"), py::truediv(&Py::Float(1.0), &py::add(&py::truediv(&v_sa.clone(), &v_Se.clone())?, &py::truediv(&v_sm.clone(), &v_Sy.clone())?)?)?), (Py::str("sigma_ar_MPa"), py::truediv(&py::truediv(&v_sa.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&v_sm.clone(), &v_Sy.clone())?)?)?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spur_contact(mut v_N1: Py, mut v_N2: Py, mut v_m: Py, mut v_phi: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_pb: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_ra1: Py = Py::None;
    let mut v_ra2: Py = Py::None;
    let mut v_rb1: Py = Py::None;
    let mut v_rb2: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let mut v_u2: Py = Py::None;
    if ({ let t3890 = { let t3886 = v_N1.clone(); Py::Bool({ let t3887 = Py::Int(1i64); if py::lt(&t3886, &t3887)? { true } else { false } }) }; if t3890.truthy() { t3890 } else { { let t3888 = v_N2.clone(); Py::Bool({ let t3889 = Py::Int(1i64); if py::lt(&t3888, &t3889)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("tooth numbers must be positive"))));
    }
    let _ = f__pos(Py::str("the module"), Py::tuple(vec![v_m.clone()]))?;
    if (py::not(&{ let t3896 = f__deg(Py::Int(10i64))?; Py::Bool({ let t3897 = v_phi.clone(); if py::le(&t3896, &t3897)? { { let t3898 = f__deg(Py::Int(30i64))?; if py::le(&t3897, &t3898)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pressure angle must lie between 10 and 30 degrees"))));
    }
    let t3899 = (Py::tuple(vec![py::truediv(&py::mul(&v_N1.clone(), &v_m.clone())?, &Py::Int(2i64))?, py::truediv(&py::mul(&v_N2.clone(), &v_m.clone())?, &Py::Int(2i64))?])).items()?;
    if t3899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t3899[0].clone();
    v_r2 = t3899[1].clone();
    let t3900 = (Py::tuple(vec![py::add(&v_r1.clone(), &v_m.clone())?, py::add(&v_r2.clone(), &v_m.clone())?])).items()?;
    if t3900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ra1 = t3900[0].clone();
    v_ra2 = t3900[1].clone();
    let t3901 = (Py::tuple(vec![py::mul(&v_r1.clone(), &py::math1("cos", &v_phi.clone())?)?, py::mul(&v_r2.clone(), &py::math1("cos", &v_phi.clone())?)?])).items()?;
    if t3901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rb1 = t3901[0].clone();
    v_rb2 = t3901[1].clone();
    v_C = py::add(&v_r1.clone(), &v_r2.clone())?;
    let t3902 = (Py::tuple(vec![py::math1("sqrt", &py::sub(&py::pow(&v_ra1.clone(), &Py::Int(2i64))?, &py::pow(&v_rb1.clone(), &Py::Int(2i64))?)?)?, py::math1("sqrt", &py::sub(&py::pow(&v_ra2.clone(), &Py::Int(2i64))?, &py::pow(&v_rb2.clone(), &Py::Int(2i64))?)?)?])).items()?;
    if t3902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u1 = t3902[0].clone();
    v_u2 = t3902[1].clone();
    if ({ let t3907 = { let t3903 = v_u1.clone(); Py::Bool({ let t3904 = py::mul(&v_C.clone(), &py::math1("sin", &v_phi.clone())?)?; if py::gt(&t3903, &t3904)? { true } else { false } }) }; if t3907.truthy() { t3907 } else { { let t3905 = v_u2.clone(); Py::Bool({ let t3906 = py::mul(&v_C.clone(), &py::math1("sin", &v_phi.clone())?)?; if py::gt(&t3905, &t3906)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("involute interference: an addendum passes the other gear's base-circle tangency (too few teeth)"))));
    }
    v_Z = py::sub(&py::add(&v_u1.clone(), &v_u2.clone())?, &py::mul(&v_C.clone(), &py::math1("sin", &v_phi.clone())?)?)?;
    v_pb = py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_m.clone())?, &py::math1("cos", &v_phi.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d1_mm"), py::mul(&py::mul(&Py::Int(2i64), &v_r1.clone())?, &Py::Float(1000.0))?), (Py::str("d2_mm"), py::mul(&py::mul(&Py::Int(2i64), &v_r2.clone())?, &Py::Float(1000.0))?), (Py::str("C_mm"), py::mul(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("Z_mm"), py::mul(&v_Z.clone(), &Py::Float(1000.0))?), (Py::str("pb_mm"), py::mul(&v_pb.clone(), &Py::Float(1000.0))?), (Py::str("contact_ratio"), py::truediv(&v_Z.clone(), &v_pb.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_point_load(mut v_L: Py, mut v_P: Py, mut v_a: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_EI: Py = Py::None;
    let mut v_Mmax: Py = Py::None;
    let mut v_RA: Py = Py::None;
    let mut v_RB: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d_load: Py = Py::None;
    let mut v_dmax: Py = Py::None;
    let mut v_xm: Py = Py::None;
    let _ = f__pos(Py::str("L and P"), Py::tuple(vec![v_L.clone(), v_P.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    if (py::not(&{ let t3528 = Py::Int(0i64); Py::Bool({ let t3529 = v_a.clone(); if py::lt(&t3528, &t3529)? { { let t3530 = v_L.clone(); if py::lt(&t3529, &t3530)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load position a must lie strictly between the supports (0 < a < L)"))));
    }
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    v_EI = py::mul(&v_E.clone(), &v_I.clone())?;
    let t3531 = (Py::tuple(vec![py::truediv(&py::mul(&v_P.clone(), &v_b.clone())?, &v_L.clone())?, py::truediv(&py::mul(&v_P.clone(), &v_a.clone())?, &v_L.clone())?])).items()?;
    if t3531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_RA = t3531[0].clone();
    v_RB = t3531[1].clone();
    v_Mmax = py::truediv(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_b.clone())?, &v_L.clone())?;
    v_d_load = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_a.clone())?, &v_b.clone())?, &v_b.clone())?, &py::mul(&py::mul(&Py::Int(3i64), &v_EI.clone())?, &v_L.clone())?)?;
    if ({ let t3532 = v_a.clone(); Py::Bool({ let t3533 = v_b.clone(); if py::ge(&t3532, &t3533)? { true } else { false } }) }).truthy() {
        v_xm = py::math1("sqrt", &py::truediv(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &Py::Int(3i64))?)?;
        v_dmax = py::truediv(&py::mul(&py::mul(&v_P.clone(), &v_b.clone())?, &py::pow(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &Py::Float(1.5))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(9i64), &py::math1("sqrt", &Py::Int(3i64))?)?, &v_EI.clone())?, &v_L.clone())?)?;
    } else {
        v_xm = py::sub(&v_L.clone(), &py::math1("sqrt", &py::truediv(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &Py::Int(3i64))?)?)?;
        v_dmax = py::truediv(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &py::pow(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &Py::Float(1.5))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(9i64), &py::math1("sqrt", &Py::Int(3i64))?)?, &v_EI.clone())?, &v_L.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), v_RA.clone()), (Py::str("RB"), v_RB.clone()), (Py::str("M_max"), v_Mmax.clone()), (Py::str("V_max"), py::minmax(&[v_RA.clone(), v_RB.clone()], true)?), (Py::str("deflection_at_load_mm"), py::mul(&v_d_load.clone(), &Py::Float(1000.0))?), (Py::str("max_deflection_mm"), py::mul(&v_dmax.clone(), &Py::Float(1000.0))?), (Py::str("x_max"), v_xm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_point_loads(mut v_L: Py, mut v_x: Py, mut v_P: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_RA: Py = Py::None;
    let mut v_RB: Py = Py::None;
    let mut v_best_M: Py = Py::None;
    let mut v_best_x: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_xv: Py = Py::None;
    let _ = f__pos(Py::str("the span L"), Py::tuple(vec![v_L.clone()]))?;
    if ({ let t3571 = py::len(&v_x.clone())?; Py::Bool({ let t3572 = py::len(&v_P.clone())?; if py::ne(&t3571, &t3572) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and P must have the same number of entries"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3573 in py::iter(&(v_x.clone()))? { let mut v_xi: Py; 
v_xi = t3573;
out.push(py::not(&{ let t3574 = Py::Int(0i64); Py::Bool({ let t3575 = v_xi.clone(); if py::le(&t3574, &t3575)? { { let t3576 = v_L.clone(); if py::le(&t3575, &t3576)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every load position must lie on the span (0 <= x <= L)"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3577 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3577;
out.push(py::not(&{ let t3580 = py::math1("isfinite", &v_p.clone())?; if !t3580.truthy() { t3580 } else { { let t3578 = v_p.clone(); Py::Bool({ let t3579 = Py::Int(0i64); if py::gt(&t3578, &t3579)? { true } else { false } }) } } })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every load must be positive (downward)"))));
    }
    v_tot = py::fsum(&v_P.clone())?;
    v_RB = py::truediv(&py::fsum(&{ let mut out = vec![]; for t3581 in py::iter(&(py::zip(&[v_P.clone(), v_x.clone()])?))? { let mut v_p: Py; let mut v_xi: Py; 
let t3582 = (t3581).items()?;
if t3582.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t3582[0].clone();
v_xi = t3582[1].clone();
out.push(py::mul(&v_p.clone(), &v_xi.clone())?); } Py::list(out) })?, &v_L.clone())?;
    v_RA = py::sub(&v_tot.clone(), &v_RB.clone())?;
    v_pts = py::sorted(&py::set(&v_x.clone())?, (Py::Bool(false)).truthy())?;
    let t3583 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3583.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best_M = t3583[0].clone();
    v_best_x = t3583[1].clone();
    for t3584 in py::iter(&(v_pts.clone()))? {
        v_xv = t3584;
        v_M = py::sub(&py::mul(&v_RA.clone(), &v_xv.clone())?, &py::fsum(&{ let mut out = vec![]; for t3585 in py::iter(&(py::zip(&[v_P.clone(), v_x.clone()])?))? { let mut v_p: Py; let mut v_xi: Py; 
let t3586 = (t3585).items()?;
if t3586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t3586[0].clone();
v_xi = t3586[1].clone();
if ({ let t3587 = v_xi.clone(); Py::Bool({ let t3588 = v_xv.clone(); if py::lt(&t3587, &t3588)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_p.clone(), &py::sub(&v_xv.clone(), &v_xi.clone())?)?); }} Py::list(out) })?)?;
        if ({ let t3589 = v_M.clone(); Py::Bool({ let t3590 = py::mul(&v_best_M.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t3589, &t3590)? { true } else { false } }) }).truthy() {
            let t3591 = (Py::tuple(vec![v_M.clone(), v_xv.clone()])).items()?;
            if t3591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_best_M = t3591[0].clone();
            v_best_x = t3591[1].clone();
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), v_RA.clone()), (Py::str("RB"), v_RB.clone()), (Py::str("M_max"), v_best_M.clone()), (Py::str("x_Mmax"), v_best_x.clone()), (Py::str("V_max"), py::minmax(&[py::abs(&v_RA.clone())?, py::abs(&v_RB.clone())?], true)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_udl(mut v_L: Py, mut v_w: Py, mut v_E: Py, mut v_I: Py) -> R {
    let _ = f__pos(Py::str("L and w"), Py::tuple(vec![v_L.clone(), v_w.clone()]))?;
    let _ = f__defl_check(v_E.clone(), v_I.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("RA"), py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?), (Py::str("RB"), py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?), (Py::str("M_max"), py::truediv(&py::mul(&py::mul(&v_w.clone(), &v_L.clone())?, &v_L.clone())?, &Py::Int(8i64))?), (Py::str("V_max"), py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &Py::Int(2i64))?), (Py::str("max_deflection_mm"), py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(5i64), &v_w.clone())?, &py::pow(&v_L.clone(), &Py::Int(4i64))?)?, &py::mul(&py::mul(&Py::Int(384i64), &v_E.clone())?, &v_I.clone())?)?, &Py::Float(1000.0))?), (Py::str("end_slope_deg"), py::math1("degrees", &py::truediv(&py::mul(&v_w.clone(), &py::pow(&v_L.clone(), &Py::Int(3i64))?)?, &py::mul(&py::mul(&Py::Int(24i64), &v_E.clone())?, &v_I.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stepped_shaft(mut v_T: Py, mut v_lengths: Py, mut v_diameters: Py, mut v_G: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_Li: Py = Py::None;
    let mut v_di: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_tw: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("T and G"), Py::tuple(vec![v_T.clone(), v_G.clone()]))?;
    if ({ let t3984 = py::len(&v_lengths.clone())?; Py::Bool({ let t3985 = py::len(&v_diameters.clone())?; if py::ne(&t3984, &t3985) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("lengths and diameters must have the same number of entries"))));
    }
    for t3986 in py::iter(&(py::add(&Py::list((v_lengths.clone()).items()?), &Py::list((v_diameters.clone()).items()?))?))? {
        v_v = t3986;
        let _ = f__pos(Py::str("every length and diameter"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t3987 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tw = t3987[0].clone();
    v_tau = t3987[1].clone();
    for t3988 in py::iter(&(py::zip(&[v_lengths.clone(), v_diameters.clone()])?))? {
        let t3989 = (t3988).items()?;
        if t3989.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Li = t3989[0].clone();
        v_di = t3989[1].clone();
        v_J = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::mul(&v_di.clone(), &Py::Float(0.001))?, &Py::Int(4i64))?)?, &Py::Int(32i64))?;
        v_tw = py::iadd(&v_tw, &py::truediv(&py::mul(&py::mul(&v_T.clone(), &v_Li.clone())?, &Py::Float(0.001))?, &py::mul(&v_G.clone(), &v_J.clone())?)?)?;
        v_tau = py::minmax(&[v_tau.clone(), py::truediv(&py::truediv(&py::mul(&py::mul(&v_T.clone(), &v_di.clone())?, &Py::Float(0.001))?, &Py::Int(2i64))?, &v_J.clone())?], true)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("twist_rad"), v_tw.clone()), (Py::str("twist_deg"), py::math1("degrees", &v_tw.clone())?), (Py::str("tau_max_MPa"), py::truediv(&v_tau.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_t_section(mut v_bf: Py, mut v_tf: Py, mut v_h: Py, mut v_tw: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ix: Py = Py::None;
    let mut v_Iy: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v__xc: Py = Py::None;
    let mut v_hs: Py = Py::None;
    let mut v_yb: Py = Py::None;
    let _ = f__pos(Py::str("bf, tf, h and tw"), Py::tuple(vec![v_bf.clone(), v_tf.clone(), v_h.clone(), v_tw.clone()]))?;
    if ({ let t4082 = v_tf.clone(); Py::Bool({ let t4083 = v_h.clone(); if py::ge(&t4082, &t4083)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the flange thickness must be less than the overall depth"))));
    }
    if ({ let t4084 = v_tw.clone(); Py::Bool({ let t4085 = v_bf.clone(); if py::gt(&t4084, &t4085)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stem must not be wider than the flange"))));
    }
    v_hs = py::sub(&v_h.clone(), &v_tf.clone())?;
    let t4100 = (f__rect_set(Py::list(vec![Py::tuple(vec![Py::Int(1i64), v_bf.clone(), v_tf.clone(), Py::Float(0.0), py::sub(&v_h.clone(), &py::truediv(&v_tf.clone(), &Py::Int(2i64))?)?]), Py::tuple(vec![Py::Int(1i64), v_tw.clone(), v_hs.clone(), Py::Float(0.0), py::truediv(&v_hs.clone(), &Py::Int(2i64))?])]))?).items()?;
    if t4100.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4100[0].clone();
    v__xc = t4100[1].clone();
    v_yb = t4100[2].clone();
    v_Ix = t4100[3].clone();
    v_Iy = t4100[4].clone();
    v__ = t4100[5].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("y_bar"), v_yb.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Z_top"), py::truediv(&v_Ix.clone(), &py::sub(&v_h.clone(), &v_yb.clone())?)?), (Py::str("Z_bottom"), py::truediv(&v_Ix.clone(), &v_yb.clone())?), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thin_vessel(mut v_p: Py, mut v_r: Py, mut v_t: Py, mut v_shape: Py) -> R {
    let mut v_sh: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let _ = f__pos(Py::str("p, r and t"), Py::tuple(vec![v_p.clone(), v_r.clone(), v_t.clone()]))?;
    if ({ let t4194 = py::truediv(&v_r.clone(), &v_t.clone())?; Py::Bool({ let t4195 = Py::Int(10i64); if py::lt(&t4194, &t4195)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("r/t is below 10: the wall is not thin (use the thick-walled cylinder)"))));
    }
    if ({ let t4196 = v_shape.clone(); Py::Bool({ let t4197 = Py::str("cylinder"); if py::eq(&t4196, &t4197) { true } else { false } }) }).truthy() {
        let t4198 = (Py::tuple(vec![py::truediv(&py::mul(&v_p.clone(), &v_r.clone())?, &v_t.clone())?, py::truediv(&py::mul(&v_p.clone(), &v_r.clone())?, &py::mul(&Py::Int(2i64), &v_t.clone())?)?])).items()?;
        if t4198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sh = t4198[0].clone();
        v_sl = t4198[1].clone();
    } else {
        let t4199 = py::truediv(&py::mul(&v_p.clone(), &v_r.clone())?, &py::mul(&Py::Int(2i64), &v_t.clone())?)?;
        v_sh = t4199.clone();
        v_sl = t4199.clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("hoop_MPa"), py::truediv(&v_sh.clone(), &Py::Float(1000000.0))?), (Py::str("longitudinal_MPa"), py::truediv(&v_sl.clone(), &Py::Float(1000000.0))?), (Py::str("tau_inplane_MPa"), py::truediv(&py::truediv(&py::sub(&v_sh.clone(), &v_sl.clone())?, &Py::Int(2i64))?, &Py::Float(1000000.0))?), (Py::str("tau_abs_MPa"), py::truediv(&py::truediv(&v_sh.clone(), &Py::Int(2i64))?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transmissibility(mut v_zeta: Py, mut v_r: Py) -> R {
    let mut v_TR: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_num: Py = Py::None;
    let _ = f__nonneg(Py::str("zeta"), Py::tuple(vec![v_zeta.clone()]))?;
    let _ = f__pos(Py::str("the frequency ratio r"), Py::tuple(vec![v_r.clone()]))?;
    v_num = py::add(&Py::Int(1i64), &py::pow(&py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &v_r.clone())?, &Py::Int(2i64))?)?;
    v_den = py::add(&py::pow(&py::sub(&Py::Int(1i64), &py::mul(&v_r.clone(), &v_r.clone())?)?, &Py::Int(2i64))?, &py::pow(&py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &v_r.clone())?, &Py::Int(2i64))?)?;
    if ({ let t4498 = v_den.clone(); Py::Bool({ let t4499 = Py::Int(0i64); if py::eq(&t4498, &t4499) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("undamped resonance: the transmissibility is unbounded"))));
    }
    v_TR = py::math1("sqrt", &py::truediv(&v_num.clone(), &v_den.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("TR"), v_TR.clone()), (Py::str("isolation_percent"), (if ({ let t4508 = v_TR.clone(); Py::Bool({ let t4509 = Py::Int(1i64); if py::lt(&t4508, &t4509)? { true } else { false } }) }).truthy() { py::mul(&py::sub(&Py::Int(1i64), &v_TR.clone())?, &Py::Int(100i64))? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_truss(mut v_nodes: Py, mut v_members: Py, mut v_pin: Py, mut v_roller: Py, mut v_loads: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_forces: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mem: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_node: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_rol: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t4282 = (f__truss_read(v_nodes.clone(), v_members.clone(), v_pin.clone(), v_roller.clone(), v_loads.clone())?).items()?;
    if t4282.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t4282[0].clone();
    v_mem = t4282[1].clone();
    v_pin = t4282[2].clone();
    v_rol = t4282[3].clone();
    v_ld = t4282[4].clone();
    let t4283 = (Py::tuple(vec![py::len(&v_pts.clone())?, py::len(&v_mem.clone())?])).items()?;
    if t4283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4283[0].clone();
    v_m = t4283[1].clone();
    v_A = { let mut out = vec![]; for t4284 in py::range_iter(&[(py::mul(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v__: Py; 
v__ = t4284;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_m.clone(), &Py::Int(3i64))?)?); } Py::list(out) };
    v_b = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::mul(&Py::Int(2i64), &v_n.clone())?)?;
    for t4285 in py::iter(&(py::enumerate(&v_mem.clone(), 0)?))? {
        let t4296 = (t4285).items()?;
        if t4296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4296[0].clone();
        let t4297 = (t4296[1].clone()).items()?;
        if t4297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4297[0].clone();
        v_j = t4297[1].clone();
        let t4286 = (Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_j.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_j.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(1i64))?)?])).items()?;
        if t4286.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dx = t4286[0].clone();
        v_dy = t4286[1].clone();
        v_L = py::hypot(&[v_dx.clone(), v_dy.clone()])?;
        let t4287 = (Py::tuple(vec![py::truediv(&v_dx.clone(), &v_L.clone())?, py::truediv(&v_dy.clone(), &v_L.clone())?])).items()?;
        if t4287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cx = t4287[0].clone();
        v_cy = t4287[1].clone();
        let t4288 = py::getitem(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_i.clone())?)?; let t4289 = v_k.clone();
        py::setitem(&t4288, &t4289, py::iadd(&py::getitem(&t4288, &t4289)?, &v_cx.clone())?)?;
        let t4290 = py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_i.clone())?, &Py::Int(1i64))?)?; let t4291 = v_k.clone();
        py::setitem(&t4290, &t4291, py::iadd(&py::getitem(&t4290, &t4291)?, &v_cy.clone())?)?;
        let t4292 = py::getitem(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_j.clone())?)?; let t4293 = v_k.clone();
        py::setitem(&t4292, &t4293, py::sub(&py::getitem(&t4292, &t4293)?, &v_cx.clone())?)?;
        let t4294 = py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_j.clone())?, &Py::Int(1i64))?)?; let t4295 = v_k.clone();
        py::setitem(&t4294, &t4295, py::sub(&py::getitem(&t4294, &t4295)?, &v_cy.clone())?)?;
    }
    py::setitem(&py::getitem(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_pin.clone())?)?, &v_m.clone(), Py::Float(1.0))?;
    py::setitem(&py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_pin.clone())?, &Py::Int(1i64))?)?, &py::add(&v_m.clone(), &Py::Int(1i64))?, Py::Float(1.0))?;
    py::setitem(&py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_rol.clone())?, &Py::Int(1i64))?)?, &py::add(&v_m.clone(), &Py::Int(2i64))?, Py::Float(1.0))?;
    for t4298 in py::iter(&(v_ld.clone()))? {
        let t4303 = (t4298).items()?;
        if t4303.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_node = t4303[0].clone();
        v_fx = t4303[1].clone();
        v_fy = t4303[2].clone();
        let t4299 = v_b.clone(); let t4300 = py::mul(&Py::Int(2i64), &v_node.clone())?;
        py::setitem(&t4299, &t4300, py::sub(&py::getitem(&t4299, &t4300)?, &v_fx.clone())?)?;
        let t4301 = v_b.clone(); let t4302 = py::add(&py::mul(&Py::Int(2i64), &v_node.clone())?, &Py::Int(1i64))?;
        py::setitem(&t4301, &t4302, py::sub(&py::getitem(&t4301, &t4302)?, &v_fy.clone())?)?;
    }
    v_x = f__gauss_solve(v_A.clone(), v_b.clone())?;
    v_scale = py::minmax(&[py::add(&py::add(&{ let mut out = vec![]; for t4319 in py::iter(&(v_ld.clone()))? { let mut v__n: Py; let mut v_f: Py; let mut v_g: Py; 
let t4320 = (t4319).items()?;
if t4320.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__n = t4320[0].clone();
v_f = t4320[1].clone();
v_g = t4320[2].clone();
out.push(py::abs(&v_f.clone())?); } Py::list(out) }, &{ let mut out = vec![]; for t4321 in py::iter(&(v_ld.clone()))? { let mut v__n: Py; let mut v_f: Py; let mut v_g: Py; 
let t4322 = (t4321).items()?;
if t4322.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__n = t4322[0].clone();
v_f = t4322[1].clone();
v_g = t4322[2].clone();
out.push(py::abs(&v_g.clone())?); } Py::list(out) })?, &Py::list(vec![Py::Float(1e-300)]))?], true)?;
    v_x = { let mut out = vec![]; for t4323 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4323;
out.push(f__snap(v_v.clone(), v_scale.clone())?); } Py::list(out) };
    v_forces = py::slice(&v_x.clone(), None, Some((v_m.clone()).i()?), None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("forces_kN"), v_forces.clone()), (Py::str("pin_Rx_kN"), py::getitem(&v_x.clone(), &v_m.clone())?), (Py::str("pin_Ry_kN"), py::getitem(&v_x.clone(), &py::add(&v_m.clone(), &Py::Int(1i64))?)?), (Py::str("roller_Ry_kN"), py::getitem(&v_x.clone(), &py::add(&v_m.clone(), &Py::Int(2i64))?)?), (Py::str("max_tension_kN"), py::minmax(&[{ let t4337 = { let mut out = vec![]; for t4334 in py::iter(&(v_forces.clone()))? { let mut v_f: Py; 
v_f = t4334;
if ({ let t4335 = v_f.clone(); Py::Bool({ let t4336 = Py::Int(0i64); if py::gt(&t4335, &t4336)? { true } else { false } }) }).truthy() { out.push(v_f.clone()); }} Py::list(out) }; if t4337.truthy() { t4337 } else { Py::list(vec![Py::Float(0.0)]) } }], true)?), (Py::str("max_compression_kN"), py::neg(&py::minmax(&[{ let t4341 = { let mut out = vec![]; for t4338 in py::iter(&(v_forces.clone()))? { let mut v_f: Py; 
v_f = t4338;
if ({ let t4339 = v_f.clone(); Py::Bool({ let t4340 = Py::Int(0i64); if py::lt(&t4339, &t4340)? { true } else { false } }) }).truthy() { out.push(v_f.clone()); }} Py::list(out) }; if t4341.truthy() { t4341 } else { Py::list(vec![Py::Float(0.0)]) } }], false)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tube_section(mut v_D: Py, mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I: Py = Py::None;
    let _ = f__pos(Py::str("the outer diameter D"), Py::tuple(vec![v_D.clone()]))?;
    let _ = f__nonneg(Py::str("the inner diameter d"), Py::tuple(vec![v_d.clone()]))?;
    if ({ let t1716 = v_d.clone(); Py::Bool({ let t1717 = v_D.clone(); if py::ge(&t1716, &t1717)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner diameter must be smaller than the outer diameter"))));
    }
    v_A = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&v_d.clone(), &v_d.clone())?)?)?, &Py::Int(4i64))?;
    v_I = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&py::pow(&v_D.clone(), &Py::Int(4i64))?, &py::pow(&v_d.clone(), &Py::Int(4i64))?)?)?, &Py::Int(64i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("I"), v_I.clone()), (Py::str("J"), py::mul(&Py::Int(2i64), &v_I.clone())?), (Py::str("Z"), py::truediv(&v_I.clone(), &py::truediv(&v_D.clone(), &Py::Int(2i64))?)?), (Py::str("Zp"), py::truediv(&py::sub(&py::pow(&v_D.clone(), &Py::Int(3i64))?, &py::pow(&v_d.clone(), &Py::Int(3i64))?)?, &Py::Int(6i64))?), (Py::str("r"), py::truediv(&py::math1("sqrt", &py::add(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&v_d.clone(), &v_d.clone())?)?)?, &Py::Int(4i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_cable_tension(mut v_W: Py, mut v_alpha: Py, mut v_beta: Py) -> R {
    let mut v_T1: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the weight W"), Py::tuple(vec![v_W.clone()]))?;
    for t4371 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("alpha"), v_alpha.clone()]), Py::tuple(vec![Py::str("beta"), v_beta.clone()])])))? {
        let t4375 = (t4371).items()?;
        if t4375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t4375[0].clone();
        v_v = t4375[1].clone();
        if (py::not(&{ let t4372 = Py::Int(0i64); Py::Bool({ let t4373 = v_v.clone(); if py::lt(&t4372, &t4373)? { { let t4374 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t4373, &t4374)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_nm.clone(), "")?, " must lie strictly between 0 and 90 degrees (measured from the horizontal)".to_string()].concat()))));
        }
    }
    v_s = py::math1("sin", &py::add(&v_alpha.clone(), &v_beta.clone())?)?;
    v_T1 = py::truediv(&py::mul(&v_W.clone(), &py::math1("cos", &v_beta.clone())?)?, &v_s.clone())?;
    v_T2 = py::truediv(&py::mul(&v_W.clone(), &py::math1("cos", &v_alpha.clone())?)?, &v_s.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_rotor(mut v_J1: Py, mut v_J2: Py, mut v_kt: Py, mut v_L: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("J1, J2 and kt"), Py::tuple(vec![v_J1.clone(), v_J2.clone(), v_kt.clone()]))?;
    v_w = py::math1("sqrt", &py::truediv(&py::mul(&v_kt.clone(), &py::add(&v_J1.clone(), &v_J2.clone())?)?, &py::mul(&v_J1.clone(), &v_J2.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("f"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?), (Py::str("amplitude_ratio"), py::truediv(&py::neg(&v_J1.clone())?, &v_J2.clone())?)]);
    if ({ let t4413 = v_L.clone(); Py::Bool({ let t4414 = Py::None; if !py::is(&t4413, &t4414) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("L"), Py::tuple(vec![v_L.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("node_from_rotor1"), py::truediv(&py::mul(&v_L.clone(), &v_J2.clone())?, &py::add(&v_J1.clone(), &v_J2.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unbalance(mut v_M: Py, mut v_mu: Py, mut v_e: Py, mut v_k: Py, mut v_c: Py, mut v_omega: Py) -> R {
    let mut v_FT: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_den: Py = Py::None;
    let _ = f__pos(Py::str("M, mu, e, k and omega"), Py::tuple(vec![v_M.clone(), v_mu.clone(), v_e.clone(), v_k.clone(), v_omega.clone()]))?;
    let _ = f__nonneg(Py::str("c"), Py::tuple(vec![v_c.clone()]))?;
    if ({ let t3026 = v_mu.clone(); Py::Bool({ let t3027 = v_M.clone(); if py::ge(&t3026, &t3027)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the unbalanced mass must be part of the total mass M"))));
    }
    v_den = py::hypot(&[py::sub(&v_k.clone(), &py::mul(&v_M.clone(), &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?)?, py::mul(&v_c.clone(), &v_omega.clone())?])?;
    if ({ let t3028 = v_den.clone(); Py::Bool({ let t3029 = Py::Int(0i64); if py::eq(&t3028, &t3029) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("undamped resonance"))));
    }
    v_X = py::truediv(&py::mul(&py::mul(&v_mu.clone(), &v_e.clone())?, &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?, &v_den.clone())?;
    v_FT = py::mul(&v_X.clone(), &py::hypot(&[v_k.clone(), py::mul(&v_c.clone(), &v_omega.clone())?])?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), v_X.clone()), (Py::str("X_mm"), py::mul(&v_X.clone(), &Py::Float(1000.0))?), (Py::str("F_transmitted"), v_FT.clone()), (Py::str("F_unbalance"), py::mul(&py::mul(&v_mu.clone(), &v_e.clone())?, &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wahl_factor(mut v_d: Py, mut v_D: Py, mut v_F: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_KW: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("d and D"), Py::tuple(vec![v_d.clone(), v_D.clone()]))?;
    v_C = py::truediv(&v_D.clone(), &v_d.clone())?;
    if ({ let t4539 = v_C.clone(); Py::Bool({ let t4540 = Py::Int(2i64); if py::le(&t4539, &t4540)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the spring index D/d must exceed 2"))));
    }
    v_KW = py::add(&py::truediv(&py::sub(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(1i64))?, &py::sub(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(4i64))?)?, &py::truediv(&Py::Float(0.615), &v_C.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("index"), v_C.clone()), (Py::str("K_wahl"), v_KW.clone()), (Py::str("K_bergstrasser"), py::truediv(&py::add(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(2i64))?, &py::sub(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(3i64))?)?), (Py::str("K_s"), py::add(&Py::Int(1i64), &py::truediv(&Py::Float(0.5), &v_C.clone())?)?)])?;
    if ({ let t4541 = v_F.clone(); Py::Bool({ let t4542 = Py::None; if !py::is(&t4541, &t4542) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("F"), Py::tuple(vec![v_F.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("tau_MPa"), py::truediv(&py::truediv(&py::mul(&py::mul(&py::mul(&v_KW.clone(), &Py::Int(8i64))?, &v_F.clone())?, &v_D.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d.clone(), &Py::Int(3i64))?)?)?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_yield_safety(mut v_Sy: Py, mut v_sx: Py, mut v_sy: Py, mut v_sz: Py, mut v_txy: Py, mut v_tyz: Py, mut v_tzx: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_vm: Py = Py::None;
    let _ = f__pos(Py::str("the yield strength Sy"), Py::tuple(vec![v_Sy.clone()]))?;
    v_r = f_principal_3d(v_sx.clone(), v_sy.clone(), v_sz.clone(), v_txy.clone(), v_tyz.clone(), v_tzx.clone())?;
    let t4620 = (Py::tuple(vec![py::mul(&py::getitem(&v_r.clone(), &Py::str("von_mises_MPa"))?, &Py::Float(1000000.0))?, py::mul(&py::sub(&py::getitem(&v_r.clone(), &Py::str("s1_MPa"))?, &py::getitem(&v_r.clone(), &Py::str("s3_MPa"))?)?, &Py::Float(1000000.0))?])).items()?;
    if t4620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vm = t4620[0].clone();
    v_tr = t4620[1].clone();
    if ({ let t4621 = v_vm.clone(); Py::Bool({ let t4622 = Py::Int(0i64); if py::le(&t4621, &t4622)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stress state is purely hydrostatic: neither criterion predicts yielding"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("von_mises_MPa"), py::truediv(&v_vm.clone(), &Py::Float(1000000.0))?), (Py::str("tresca_MPa"), py::truediv(&v_tr.clone(), &Py::Float(1000000.0))?), (Py::str("n_von_mises"), py::truediv(&v_Sy.clone(), &v_vm.clone())?), (Py::str("n_tresca"), py::truediv(&v_Sy.clone(), &v_tr.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1012(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1014: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1013 = v_inputs.clone(); f_damped_sdof(py::getitem(&t1013, &Py::str("m"))?, py::getitem(&t1013, &Py::str("c"))?, py::getitem(&t1013, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1014 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1015 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1035 = (t1015).items()?;
        if t1035.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1035[0].clone();
        v_v = t1035[1].clone();
        if ({ let t1016 = v_k.clone(); Py::Bool({ let t1017 = v_outputs.clone(); if !py::contains(&t1017, &t1016)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1056(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1058: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1057 = v_inputs.clone(); f_disc_clutch(py::getitem(&t1057, &Py::str("F"))?, py::getitem(&t1057, &Py::str("f"))?, py::getitem(&t1057, &Py::str("D"))?, py::getitem(&t1057, &Py::str("d"))?, py::getitem(&t1057, &Py::str("model"))?, py::method(&t1057, "get", &[Py::str("pairs"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1058 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1059 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1079 = (t1059).items()?;
        if t1079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1079[0].clone();
        v_v = t1079[1].clone();
        if ({ let t1060 = v_k.clone(); Py::Bool({ let t1061 = v_outputs.clone(); if !py::contains(&t1061, &t1060)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1110(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1112: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1111 = v_inputs.clone(); f_dunkerley_shaft(py::getitem(&t1111, &Py::str("masses"))?, py::getitem(&t1111, &Py::str("positions"))?, py::getitem(&t1111, &Py::str("L"))?, py::getitem(&t1111, &Py::str("E"))?, py::getitem(&t1111, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1112 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1113 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1133 = (t1113).items()?;
        if t1133.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1133[0].clone();
        v_v = t1133[1].clone();
        if ({ let t1114 = v_k.clone(); Py::Bool({ let t1115 = v_outputs.clone(); if !py::contains(&t1115, &t1114)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1171(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1173: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1172 = v_inputs.clone(); f_bolt_group(py::getitem(&t1172, &Py::str("bolts"))?, py::getitem(&t1172, &Py::str("P"))?, py::getitem(&t1172, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1173 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1174 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1194 = (t1174).items()?;
        if t1194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1194[0].clone();
        v_v = t1194[1].clone();
        if ({ let t1175 = v_k.clone(); Py::Bool({ let t1176 = v_outputs.clone(); if !py::contains(&t1176, &t1175)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1223 = v_inputs.clone(); f_euler_column(py::getitem(&t1223, &Py::str("E"))?, py::getitem(&t1223, &Py::str("I"))?, py::getitem(&t1223, &Py::str("L"))?, py::getitem(&t1223, &Py::str("ends"))?, py::method(&t1223, "get", &[Py::str("A"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1245 = (t1225).items()?;
        if t1245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1245[0].clone();
        v_v = t1245[1].clone();
        if ({ let t1226 = v_k.clone(); Py::Bool({ let t1227 = v_outputs.clone(); if !py::contains(&t1227, &t1226)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1271(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1273: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1272 = v_inputs.clone(); f_eurocode_damage(py::getitem(&t1272, &Py::str("ranges"))?, py::getitem(&t1272, &Py::str("counts"))?, py::getitem(&t1272, &Py::str("dsc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1273 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1274 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1294 = (t1274).items()?;
        if t1294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1294[0].clone();
        v_v = t1294[1].clone();
        if ({ let t1275 = v_k.clone(); Py::Bool({ let t1276 = v_outputs.clone(); if !py::contains(&t1276, &t1275)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1327(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1329: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1328 = v_inputs.clone(); f_mean_stress_life(py::getitem(&t1328, &Py::str("sa"))?, py::getitem(&t1328, &Py::str("sm"))?, py::getitem(&t1328, &Py::str("sf"))?, py::getitem(&t1328, &Py::str("b"))?, py::getitem(&t1328, &Py::str("method"))?, py::method(&t1328, "get", &[Py::str("gamma"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1329 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1330 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1350 = (t1330).items()?;
        if t1350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1350[0].clone();
        v_v = t1350[1].clone();
        if ({ let t1331 = v_k.clone(); Py::Bool({ let t1332 = v_outputs.clone(); if !py::contains(&t1332, &t1331)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1368(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1370: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1369 = v_inputs.clone(); f_ff_point(py::getitem(&t1369, &Py::str("L"))?, py::getitem(&t1369, &Py::str("P"))?, py::getitem(&t1369, &Py::str("a"))?, py::getitem(&t1369, &Py::str("E"))?, py::getitem(&t1369, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1370 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1371 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1391 = (t1371).items()?;
        if t1391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1391[0].clone();
        v_v = t1391[1].clone();
        if ({ let t1372 = v_k.clone(); Py::Bool({ let t1373 = v_outputs.clone(); if !py::contains(&t1373, &t1372)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1407: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1406 = v_inputs.clone(); f_ff_udl(py::getitem(&t1406, &Py::str("L"))?, py::getitem(&t1406, &Py::str("w"))?, py::getitem(&t1406, &Py::str("E"))?, py::getitem(&t1406, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1407 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1408 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1428 = (t1408).items()?;
        if t1428.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1428[0].clone();
        v_v = t1428[1].clone();
        if ({ let t1409 = v_k.clone(); Py::Bool({ let t1410 = v_outputs.clone(); if !py::contains(&t1410, &t1409)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_142(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t144: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t143 = v_inputs.clone(); f_axial_bar(py::getitem(&t143, &Py::str("lengths"))?, py::getitem(&t143, &Py::str("areas"))?, py::getitem(&t143, &Py::str("loads"))?, py::getitem(&t143, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t144 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t145 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t165 = (t145).items()?;
        if t165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t165[0].clone();
        v_v = t165[1].clone();
        if ({ let t146 = v_k.clone(); Py::Bool({ let t147 = v_outputs.clone(); if !py::contains(&t147, &t146)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1453(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1455: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1454 = v_inputs.clone(); f_forced_response(py::getitem(&t1454, &Py::str("m"))?, py::getitem(&t1454, &Py::str("c"))?, py::getitem(&t1454, &Py::str("k"))?, py::getitem(&t1454, &Py::str("F0"))?, py::getitem(&t1454, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1455 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1456 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1476 = (t1456).items()?;
        if t1476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1476[0].clone();
        v_v = t1476[1].clone();
        if ({ let t1457 = v_k.clone(); Py::Bool({ let t1458 = v_outputs.clone(); if !py::contains(&t1458, &t1457)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1505(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1507: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1506 = v_inputs.clone(); f_four_bar(py::getitem(&t1506, &Py::str("a"))?, py::getitem(&t1506, &Py::str("b"))?, py::getitem(&t1506, &Py::str("c"))?, py::getitem(&t1506, &Py::str("d"))?, py::getitem(&t1506, &Py::str("theta2"))?, py::getitem(&t1506, &Py::str("branch"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1507 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1508 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1528 = (t1508).items()?;
        if t1528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1528[0].clone();
        v_v = t1528[1].clone();
        if ({ let t1509 = v_k.clone(); Py::Bool({ let t1510 = v_outputs.clone(); if !py::contains(&t1510, &t1509)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1550(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1552: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1551 = v_inputs.clone(); f_hooke_3d(py::getitem(&t1551, &Py::str("sx"))?, py::getitem(&t1551, &Py::str("sy"))?, py::getitem(&t1551, &Py::str("sz"))?, py::getitem(&t1551, &Py::str("txy"))?, py::getitem(&t1551, &Py::str("tyz"))?, py::getitem(&t1551, &Py::str("tzx"))?, py::getitem(&t1551, &Py::str("E"))?, py::getitem(&t1551, &Py::str("nu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1552 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1553 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1573 = (t1553).items()?;
        if t1573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1573[0].clone();
        v_v = t1573[1].clone();
        if ({ let t1554 = v_k.clone(); Py::Bool({ let t1555 = v_outputs.clone(); if !py::contains(&t1555, &t1554)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1596(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1598: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1597 = v_inputs.clone(); f_gerber(py::getitem(&t1597, &Py::str("sa"))?, py::getitem(&t1597, &Py::str("sm"))?, py::getitem(&t1597, &Py::str("Se"))?, py::getitem(&t1597, &Py::str("Sut"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1598 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1599 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1619 = (t1599).items()?;
        if t1619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1619[0].clone();
        v_v = t1619[1].clone();
        if ({ let t1600 = v_k.clone(); Py::Bool({ let t1601 = v_outputs.clone(); if !py::contains(&t1601, &t1600)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1643(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1645: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1644 = v_inputs.clone(); f_goodman(py::getitem(&t1644, &Py::str("sa"))?, py::getitem(&t1644, &Py::str("sm"))?, py::getitem(&t1644, &Py::str("Se"))?, py::getitem(&t1644, &Py::str("Sut"))?, py::method(&t1644, "get", &[Py::str("Sy"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1645 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1646 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1666 = (t1646).items()?;
        if t1666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1666[0].clone();
        v_v = t1666[1].clone();
        if ({ let t1647 = v_k.clone(); Py::Bool({ let t1648 = v_outputs.clone(); if !py::contains(&t1648, &t1647)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1682(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1684: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1683 = v_inputs.clone(); f_helical_spring(py::getitem(&t1683, &Py::str("d"))?, py::getitem(&t1683, &Py::str("D"))?, py::getitem(&t1683, &Py::str("Na"))?, py::getitem(&t1683, &Py::str("G"))?, py::getitem(&t1683, &Py::str("F"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1684 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1685 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1705 = (t1685).items()?;
        if t1705.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1705[0].clone();
        v_v = t1705[1].clone();
        if ({ let t1686 = v_k.clone(); Py::Bool({ let t1687 = v_outputs.clone(); if !py::contains(&t1687, &t1686)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1728: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1727 = v_inputs.clone(); f_tube_section(py::getitem(&t1727, &Py::str("D"))?, py::getitem(&t1727, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1728 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1729 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1749 = (t1729).items()?;
        if t1749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1749[0].clone();
        v_v = t1749[1].clone();
        if ({ let t1730 = v_k.clone(); Py::Bool({ let t1731 = v_outputs.clone(); if !py::contains(&t1731, &t1730)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1766(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1768: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1767 = v_inputs.clone(); f_box_section(py::getitem(&t1767, &Py::str("B"))?, py::getitem(&t1767, &Py::str("H"))?, py::getitem(&t1767, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1768 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1769 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1789 = (t1769).items()?;
        if t1789.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1789[0].clone();
        v_v = t1789[1].clone();
        if ({ let t1770 = v_k.clone(); Py::Bool({ let t1771 = v_outputs.clone(); if !py::contains(&t1771, &t1770)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1790(mut v_T: Py, mut v_D: Py, mut v_d: Py, mut v_L: Py, mut v_G: Py) -> R {
    return Ok(f__hollow_shaft(v_T.clone(), v_D.clone(), v_d.clone(), v_L.clone(), v_G.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t182: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t181 = v_inputs.clone(); f_bar_fixed_ends(py::getitem(&t181, &Py::str("P"))?, py::getitem(&t181, &Py::str("L1"))?, py::getitem(&t181, &Py::str("A1"))?, py::getitem(&t181, &Py::str("E1"))?, py::getitem(&t181, &Py::str("L2"))?, py::getitem(&t181, &Py::str("A2"))?, py::getitem(&t181, &Py::str("E2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t182 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t183 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t203 = (t183).items()?;
        if t203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t203[0].clone();
        v_v = t203[1].clone();
        if ({ let t184 = v_k.clone(); Py::Bool({ let t185 = v_outputs.clone(); if !py::contains(&t185, &t184)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1811(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1813: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1812 = v_inputs.clone(); s_1790(py::getitem(&t1812, &Py::str("T"))?, py::getitem(&t1812, &Py::str("D"))?, py::getitem(&t1812, &Py::str("d"))?, py::getitem(&t1812, &Py::str("L"))?, py::getitem(&t1812, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1813 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1814 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1834 = (t1814).items()?;
        if t1834.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1834[0].clone();
        v_v = t1834[1].clone();
        if ({ let t1815 = v_k.clone(); Py::Bool({ let t1816 = v_outputs.clone(); if !py::contains(&t1816, &t1815)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1868(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1870: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1869 = v_inputs.clone(); f_i_section(py::getitem(&t1869, &Py::str("b"))?, py::getitem(&t1869, &Py::str("h"))?, py::getitem(&t1869, &Py::str("tf"))?, py::getitem(&t1869, &Py::str("tw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1870 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1871 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1891 = (t1871).items()?;
        if t1891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1891[0].clone();
        v_v = t1891[1].clone();
        if ({ let t1872 = v_k.clone(); Py::Bool({ let t1873 = v_outputs.clone(); if !py::contains(&t1873, &t1872)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1922(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1924: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1923 = v_inputs.clone(); f_johnson_column(py::getitem(&t1923, &Py::str("Sy"))?, py::getitem(&t1923, &Py::str("E"))?, py::getitem(&t1923, &Py::str("L"))?, py::getitem(&t1923, &Py::str("r"))?, py::getitem(&t1923, &Py::str("ends"))?, py::method(&t1923, "get", &[Py::str("A"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1924 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1925 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1945 = (t1925).items()?;
        if t1945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1945[0].clone();
        v_v = t1945[1].clone();
        if ({ let t1926 = v_k.clone(); Py::Bool({ let t1927 = v_outputs.clone(); if !py::contains(&t1927, &t1926)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1964(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1966: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1965 = v_inputs.clone(); f_lewis_stress(py::getitem(&t1965, &Py::str("Wt"))?, py::getitem(&t1965, &Py::str("F"))?, py::getitem(&t1965, &Py::str("m"))?, py::getitem(&t1965, &Py::str("Y"))?, py::method(&t1965, "get", &[Py::str("Kv"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1966 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1967 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1987 = (t1967).items()?;
        if t1987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1987[0].clone();
        v_v = t1987[1].clone();
        if ({ let t1968 = v_k.clone(); Py::Bool({ let t1969 = v_outputs.clone(); if !py::contains(&t1969, &t1968)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2026(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2028: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2027 = v_inputs.clone(); f_marin(py::getitem(&t2027, &Py::str("Sut"))?, py::getitem(&t2027, &Py::str("surface"))?, py::getitem(&t2027, &Py::str("load"))?, py::method(&t2027, "get", &[Py::str("d"), Py::None])?, py::method(&t2027, "get", &[Py::str("R"), Py::Float(0.5)])?, py::method(&t2027, "get", &[Py::str("kd"), Py::Float(1.0)])?, py::method(&t2027, "get", &[Py::str("kf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2028 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2029 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2049 = (t2029).items()?;
        if t2049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2049[0].clone();
        v_v = t2049[1].clone();
        if ({ let t2030 = v_k.clone(); Py::Bool({ let t2031 = v_outputs.clone(); if !py::contains(&t2031, &t2030)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2135(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2137: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2136 = v_inputs.clone(); f_mdof_chain(py::getitem(&t2136, &Py::str("masses"))?, py::getitem(&t2136, &Py::str("springs"))?, py::getitem(&t2136, &Py::str("ends"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2137 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2138 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2158 = (t2138).items()?;
        if t2158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2158[0].clone();
        v_v = t2158[1].clone();
        if ({ let t2139 = v_k.clone(); Py::Bool({ let t2140 = v_outputs.clone(); if !py::contains(&t2140, &t2139)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2178(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2180: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2179 = v_inputs.clone(); f_miner_linear(py::getitem(&t2179, &Py::str("ranges"))?, py::getitem(&t2179, &Py::str("counts"))?, py::getitem(&t2179, &Py::str("Sc"))?, py::getitem(&t2179, &Py::str("m"))?, py::method(&t2179, "get", &[Py::str("Nc"), Py::Float(2000000.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2180 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2181 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2201 = (t2181).items()?;
        if t2201.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2201[0].clone();
        v_v = t2201[1].clone();
        if ({ let t2182 = v_k.clone(); Py::Bool({ let t2183 = v_outputs.clone(); if !py::contains(&t2183, &t2182)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2217(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2219: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2218 = v_inputs.clone(); f_moment_of_force(py::getitem(&t2218, &Py::str("Fx"))?, py::getitem(&t2218, &Py::str("Fy"))?, py::getitem(&t2218, &Py::str("x"))?, py::getitem(&t2218, &Py::str("y"))?, py::method(&t2218, "get", &[Py::str("x0"), Py::Float(0.0)])?, py::method(&t2218, "get", &[Py::str("y0"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2219 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2220 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2240 = (t2220).items()?;
        if t2240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2240[0].clone();
        v_v = t2240[1].clone();
        if ({ let t2221 = v_k.clone(); Py::Bool({ let t2222 = v_outputs.clone(); if !py::contains(&t2222, &t2221)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2256(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2258: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2257 = v_inputs.clone(); f_plane_transform(py::getitem(&t2257, &Py::str("sx"))?, py::getitem(&t2257, &Py::str("sy"))?, py::getitem(&t2257, &Py::str("txy"))?, py::getitem(&t2257, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2258 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2259 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2279 = (t2259).items()?;
        if t2279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2279[0].clone();
        v_v = t2279[1].clone();
        if ({ let t2260 = v_k.clone(); Py::Bool({ let t2261 = v_outputs.clone(); if !py::contains(&t2261, &t2260)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2312(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2314: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2313 = v_inputs.clone(); f_planetary(py::getitem(&t2313, &Py::str("Ns"))?, py::getitem(&t2313, &Py::str("Nr"))?, py::method(&t2313, "get", &[Py::str("ws"), Py::None])?, py::method(&t2313, "get", &[Py::str("wr"), Py::None])?, py::method(&t2313, "get", &[Py::str("wc"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2314 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2315 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2335 = (t2315).items()?;
        if t2335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2335[0].clone();
        v_v = t2335[1].clone();
        if ({ let t2316 = v_k.clone(); Py::Bool({ let t2317 = v_outputs.clone(); if !py::contains(&t2317, &t2316)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_236(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t238: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t237 = v_inputs.clone(); f_beam_frequency(py::getitem(&t237, &Py::str("E"))?, py::getitem(&t237, &Py::str("I"))?, py::getitem(&t237, &Py::str("rho"))?, py::getitem(&t237, &Py::str("A"))?, py::getitem(&t237, &Py::str("L"))?, py::getitem(&t237, &Py::str("ends"))?, py::method(&t237, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t238 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t239 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t259 = (t239).items()?;
        if t259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t259[0].clone();
        v_v = t259[1].clone();
        if ({ let t240 = v_k.clone(); Py::Bool({ let t241 = v_outputs.clone(); if !py::contains(&t241, &t240)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2429(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2431: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2430 = v_inputs.clone(); f_polygon_section(py::getitem(&t2430, &Py::str("vertices"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2431 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2432 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2452 = (t2432).items()?;
        if t2452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2452[0].clone();
        v_v = t2452[1].clone();
        if ({ let t2433 = v_k.clone(); Py::Bool({ let t2434 = v_outputs.clone(); if !py::contains(&t2434, &t2433)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2485: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2484 = v_inputs.clone(); f_power_screw(py::getitem(&t2484, &Py::str("F"))?, py::getitem(&t2484, &Py::str("dm"))?, py::getitem(&t2484, &Py::str("l"))?, py::getitem(&t2484, &Py::str("f"))?, py::method(&t2484, "get", &[Py::str("alpha"), Py::Float(0.0)])?, py::method(&t2484, "get", &[Py::str("fc"), Py::Float(0.0)])?, py::method(&t2484, "get", &[Py::str("dc"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2485 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2486 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2506 = (t2486).items()?;
        if t2506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2506[0].clone();
        v_v = t2506[1].clone();
        if ({ let t2487 = v_k.clone(); Py::Bool({ let t2488 = v_outputs.clone(); if !py::contains(&t2488, &t2487)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2526(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2527 = v_inputs.clone(); f_principal_2d(py::getitem(&t2527, &Py::str("sx"))?, py::getitem(&t2527, &Py::str("sy"))?, py::getitem(&t2527, &Py::str("txy"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2529 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2549 = (t2529).items()?;
        if t2549.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2549[0].clone();
        v_v = t2549[1].clone();
        if ({ let t2530 = v_k.clone(); Py::Bool({ let t2531 = v_outputs.clone(); if !py::contains(&t2531, &t2530)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2589(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2591: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2590 = v_inputs.clone(); f_principal_3d(py::getitem(&t2590, &Py::str("sx"))?, py::getitem(&t2590, &Py::str("sy"))?, py::getitem(&t2590, &Py::str("sz"))?, py::getitem(&t2590, &Py::str("txy"))?, py::getitem(&t2590, &Py::str("tyz"))?, py::getitem(&t2590, &Py::str("tzx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2591 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2592 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2612 = (t2592).items()?;
        if t2612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2612[0].clone();
        v_v = t2612[1].clone();
        if ({ let t2593 = v_k.clone(); Py::Bool({ let t2594 = v_outputs.clone(); if !py::contains(&t2594, &t2593)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2629(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2631: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2630 = v_inputs.clone(); f_pc_point(py::getitem(&t2630, &Py::str("L"))?, py::getitem(&t2630, &Py::str("P"))?, py::getitem(&t2630, &Py::str("a"))?, py::getitem(&t2630, &Py::str("E"))?, py::getitem(&t2630, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2631 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2632 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2652 = (t2632).items()?;
        if t2652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2652[0].clone();
        v_v = t2652[1].clone();
        if ({ let t2633 = v_k.clone(); Py::Bool({ let t2634 = v_outputs.clone(); if !py::contains(&t2634, &t2633)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2666(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2668: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2667 = v_inputs.clone(); f_pc_udl(py::getitem(&t2667, &Py::str("L"))?, py::getitem(&t2667, &Py::str("w"))?, py::getitem(&t2667, &Py::str("E"))?, py::getitem(&t2667, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2668 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2669 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2689 = (t2669).items()?;
        if t2689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2689[0].clone();
        v_v = t2689[1].clone();
        if ({ let t2670 = v_k.clone(); Py::Bool({ let t2671 = v_outputs.clone(); if !py::contains(&t2671, &t2670)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2728: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2727 = v_inputs.clone(); f_rainflow_count(py::getitem(&t2727, &Py::str("history"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2728 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2729 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2749 = (t2729).items()?;
        if t2749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2749[0].clone();
        v_v = t2749[1].clone();
        if ({ let t2730 = v_k.clone(); Py::Bool({ let t2731 = v_outputs.clone(); if !py::contains(&t2731, &t2730)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2794(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2796: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2795 = v_inputs.clone(); f_rainflow_damage(py::getitem(&t2795, &Py::str("history"))?, py::getitem(&t2795, &Py::str("Sc"))?, py::getitem(&t2795, &Py::str("m"))?, py::method(&t2795, "get", &[Py::str("Nc"), Py::Float(2000000.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2796 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2797 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2817 = (t2797).items()?;
        if t2817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2817[0].clone();
        v_v = t2817[1].clone();
        if ({ let t2798 = v_k.clone(); Py::Bool({ let t2799 = v_outputs.clone(); if !py::contains(&t2799, &t2798)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2855(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2857: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2856 = v_inputs.clone(); f_rayleigh_shaft(py::getitem(&t2856, &Py::str("masses"))?, py::getitem(&t2856, &Py::str("positions"))?, py::getitem(&t2856, &Py::str("L"))?, py::getitem(&t2856, &Py::str("E"))?, py::getitem(&t2856, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2857 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2858 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2878 = (t2858).items()?;
        if t2878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2878[0].clone();
        v_v = t2878[1].clone();
        if ({ let t2859 = v_k.clone(); Py::Bool({ let t2860 = v_outputs.clone(); if !py::contains(&t2860, &t2859)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2893(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2894 = v_inputs.clone(); f_rect_section(py::getitem(&t2894, &Py::str("b"))?, py::getitem(&t2894, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2896 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2916 = (t2896).items()?;
        if t2916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2916[0].clone();
        v_v = t2916[1].clone();
        if ({ let t2897 = v_k.clone(); Py::Bool({ let t2898 = v_outputs.clone(); if !py::contains(&t2898, &t2897)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_291(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t293: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t292 = v_inputs.clone(); f_bearing_rating(py::getitem(&t292, &Py::str("FD"))?, py::getitem(&t292, &Py::str("LD"))?, py::getitem(&t292, &Py::str("n"))?, py::getitem(&t292, &Py::str("R"))?, py::getitem(&t292, &Py::str("kind"))?, py::method(&t292, "get", &[Py::str("af"), Py::Float(1.0)])?, py::method(&t292, "get", &[Py::str("LR"), Py::Float(1000000.0)])?, py::method(&t292, "get", &[Py::str("x0"), Py::None])?, py::method(&t292, "get", &[Py::str("theta"), Py::None])?, py::method(&t292, "get", &[Py::str("b"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t293 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t294 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t314 = (t294).items()?;
        if t314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t314[0].clone();
        v_v = t314[1].clone();
        if ({ let t295 = v_k.clone(); Py::Bool({ let t296 = v_outputs.clone(); if !py::contains(&t296, &t295)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2942(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2944: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2943 = v_inputs.clone(); f_resultant_forces(py::getitem(&t2943, &Py::str("F"))?, py::getitem(&t2943, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2944 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2945 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2965 = (t2945).items()?;
        if t2965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2965[0].clone();
        v_v = t2965[1].clone();
        if ({ let t2946 = v_k.clone(); Py::Bool({ let t2947 = v_outputs.clone(); if !py::contains(&t2947, &t2946)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2992(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2994: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2993 = v_inputs.clone(); f_rotating_disk(py::getitem(&t2993, &Py::str("rho"))?, py::getitem(&t2993, &Py::str("omega"))?, py::getitem(&t2993, &Py::str("nu"))?, py::getitem(&t2993, &Py::str("ro"))?, py::method(&t2993, "get", &[Py::str("ri"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2994 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2995 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3015 = (t2995).items()?;
        if t3015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3015[0].clone();
        v_v = t3015[1].clone();
        if ({ let t2996 = v_k.clone(); Py::Bool({ let t2997 = v_outputs.clone(); if !py::contains(&t2997, &t2996)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3038(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3040: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3039 = v_inputs.clone(); f_unbalance(py::getitem(&t3039, &Py::str("M"))?, py::getitem(&t3039, &Py::str("mu"))?, py::getitem(&t3039, &Py::str("e"))?, py::getitem(&t3039, &Py::str("k"))?, py::getitem(&t3039, &Py::str("c"))?, py::getitem(&t3039, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3040 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3041 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3061 = (t3041).items()?;
        if t3061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3061[0].clone();
        v_v = t3061[1].clone();
        if ({ let t3042 = v_k.clone(); Py::Bool({ let t3043 = v_outputs.clone(); if !py::contains(&t3043, &t3042)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3089(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3091: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3090 = v_inputs.clone(); f_sdof_free(py::getitem(&t3090, &Py::str("m"))?, py::getitem(&t3090, &Py::str("c"))?, py::getitem(&t3090, &Py::str("k"))?, py::getitem(&t3090, &Py::str("x0"))?, py::getitem(&t3090, &Py::str("v0"))?, py::getitem(&t3090, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3091 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3092 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3112 = (t3092).items()?;
        if t3112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3112[0].clone();
        v_v = t3112[1].clone();
        if ({ let t3093 = v_k.clone(); Py::Bool({ let t3094 = v_outputs.clone(); if !py::contains(&t3094, &t3093)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3126(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3128: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3127 = v_inputs.clone(); f_sdof_natural(py::getitem(&t3127, &Py::str("m"))?, py::getitem(&t3127, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3128 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3129 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3149 = (t3129).items()?;
        if t3149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3149[0].clone();
        v_v = t3149[1].clone();
        if ({ let t3130 = v_k.clone(); Py::Bool({ let t3131 = v_outputs.clone(); if !py::contains(&t3131, &t3130)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3170(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3172: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3171 = v_inputs.clone(); f_secant_column(py::getitem(&t3171, &Py::str("P"))?, py::getitem(&t3171, &Py::str("e"))?, py::getitem(&t3171, &Py::str("L"))?, py::getitem(&t3171, &Py::str("A"))?, py::getitem(&t3171, &Py::str("I"))?, py::getitem(&t3171, &Py::str("c"))?, py::getitem(&t3171, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3172 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3173 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3193 = (t3173).items()?;
        if t3193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3193[0].clone();
        v_v = t3193[1].clone();
        if ({ let t3174 = v_k.clone(); Py::Bool({ let t3175 = v_outputs.clone(); if !py::contains(&t3175, &t3174)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3210(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3212: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3211 = v_inputs.clone(); f_single_disk(py::getitem(&t3211, &Py::str("m"))?, py::getitem(&t3211, &Py::str("E"))?, py::getitem(&t3211, &Py::str("I"))?, py::getitem(&t3211, &Py::str("L"))?, py::getitem(&t3211, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3212 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3213 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3233 = (t3213).items()?;
        if t3233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3233[0].clone();
        v_v = t3233[1].clone();
        if ({ let t3214 = v_k.clone(); Py::Bool({ let t3215 = v_outputs.clone(); if !py::contains(&t3215, &t3214)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3264(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3266: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3265 = v_inputs.clone(); f_shaft_de_goodman_d(py::getitem(&t3265, &Py::str("n"))?, py::getitem(&t3265, &Py::str("Ma"))?, py::getitem(&t3265, &Py::str("Mm"))?, py::getitem(&t3265, &Py::str("Ta"))?, py::getitem(&t3265, &Py::str("Tm"))?, py::getitem(&t3265, &Py::str("Kf"))?, py::getitem(&t3265, &Py::str("Kfs"))?, py::getitem(&t3265, &Py::str("Se"))?, py::getitem(&t3265, &Py::str("Sut"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3266 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3267 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3287 = (t3267).items()?;
        if t3287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3287[0].clone();
        v_v = t3287[1].clone();
        if ({ let t3268 = v_k.clone(); Py::Bool({ let t3269 = v_outputs.clone(); if !py::contains(&t3269, &t3268)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3322(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3324: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3323 = v_inputs.clone(); f_shaft_de_goodman_n(py::getitem(&t3323, &Py::str("d"))?, py::getitem(&t3323, &Py::str("Ma"))?, py::getitem(&t3323, &Py::str("Mm"))?, py::getitem(&t3323, &Py::str("Ta"))?, py::getitem(&t3323, &Py::str("Tm"))?, py::getitem(&t3323, &Py::str("Kf"))?, py::getitem(&t3323, &Py::str("Kfs"))?, py::getitem(&t3323, &Py::str("Se"))?, py::getitem(&t3323, &Py::str("Sut"))?, py::method(&t3323, "get", &[Py::str("Sy"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3324 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3325 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3345 = (t3325).items()?;
        if t3345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3345[0].clone();
        v_v = t3345[1].clone();
        if ({ let t3326 = v_k.clone(); Py::Bool({ let t3327 = v_outputs.clone(); if !py::contains(&t3327, &t3326)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3361: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3360 = v_inputs.clone(); f_shaft_power(py::getitem(&t3360, &Py::str("P"))?, py::getitem(&t3360, &Py::str("n"))?, py::getitem(&t3360, &Py::str("tau_allow"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3361 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3362 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3382 = (t3362).items()?;
        if t3382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3382[0].clone();
        v_v = t3382[1].clone();
        if ({ let t3363 = v_k.clone(); Py::Bool({ let t3364 = v_outputs.clone(); if !py::contains(&t3364, &t3363)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3454(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3456: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3455 = v_inputs.clone(); f_exact_whirl(py::getitem(&t3455, &Py::str("masses"))?, py::getitem(&t3455, &Py::str("positions"))?, py::getitem(&t3455, &Py::str("L"))?, py::getitem(&t3455, &Py::str("E"))?, py::getitem(&t3455, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3456 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3457 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3477 = (t3457).items()?;
        if t3477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3477[0].clone();
        v_v = t3477[1].clone();
        if ({ let t3458 = v_k.clone(); Py::Bool({ let t3459 = v_outputs.clone(); if !py::contains(&t3459, &t3458)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_346(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t348: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t347 = v_inputs.clone(); f_bearing_life(py::getitem(&t347, &Py::str("C"))?, py::getitem(&t347, &Py::str("F"))?, py::getitem(&t347, &Py::str("n"))?, py::getitem(&t347, &Py::str("R"))?, py::getitem(&t347, &Py::str("kind"))?, py::method(&t347, "get", &[Py::str("LR"), Py::Float(1000000.0)])?, py::method(&t347, "get", &[Py::str("x0"), Py::None])?, py::method(&t347, "get", &[Py::str("theta"), Py::None])?, py::method(&t347, "get", &[Py::str("b"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t348 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t349 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t369 = (t349).items()?;
        if t369.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t369[0].clone();
        v_v = t369[1].clone();
        if ({ let t350 = v_k.clone(); Py::Bool({ let t351 = v_outputs.clone(); if !py::contains(&t351, &t350)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3499(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3501: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3500 = v_inputs.clone(); f_shrink_fit(py::getitem(&t3500, &Py::str("delta"))?, py::getitem(&t3500, &Py::str("R"))?, py::getitem(&t3500, &Py::str("ro"))?, py::getitem(&t3500, &Py::str("E"))?, py::method(&t3500, "get", &[Py::str("ri"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3501 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3502 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3522 = (t3502).items()?;
        if t3522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3522[0].clone();
        v_v = t3522[1].clone();
        if ({ let t3503 = v_k.clone(); Py::Bool({ let t3504 = v_outputs.clone(); if !py::contains(&t3504, &t3503)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3542(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3544: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3543 = v_inputs.clone(); f_ss_point_load(py::getitem(&t3543, &Py::str("L"))?, py::getitem(&t3543, &Py::str("P"))?, py::getitem(&t3543, &Py::str("a"))?, py::getitem(&t3543, &Py::str("E"))?, py::getitem(&t3543, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3544 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3545 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3565 = (t3545).items()?;
        if t3565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3565[0].clone();
        v_v = t3565[1].clone();
        if ({ let t3546 = v_k.clone(); Py::Bool({ let t3547 = v_outputs.clone(); if !py::contains(&t3547, &t3546)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3600(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3602: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3601 = v_inputs.clone(); f_ss_point_loads(py::getitem(&t3601, &Py::str("L"))?, py::getitem(&t3601, &Py::str("x"))?, py::getitem(&t3601, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3602 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3603 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3623 = (t3603).items()?;
        if t3623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3623[0].clone();
        v_v = t3623[1].clone();
        if ({ let t3604 = v_k.clone(); Py::Bool({ let t3605 = v_outputs.clone(); if !py::contains(&t3605, &t3604)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3637(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3639: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3638 = v_inputs.clone(); f_ss_udl(py::getitem(&t3638, &Py::str("L"))?, py::getitem(&t3638, &Py::str("w"))?, py::getitem(&t3638, &Py::str("E"))?, py::getitem(&t3638, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3639 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3640 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3660 = (t3640).items()?;
        if t3660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3660[0].clone();
        v_v = t3660[1].clone();
        if ({ let t3641 = v_k.clone(); Py::Bool({ let t3642 = v_outputs.clone(); if !py::contains(&t3642, &t3641)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3681(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3683: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3682 = v_inputs.clone(); f_slider_crank(py::getitem(&t3682, &Py::str("r"))?, py::getitem(&t3682, &Py::str("l"))?, py::getitem(&t3682, &Py::str("theta"))?, py::getitem(&t3682, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3683 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3684 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3704 = (t3684).items()?;
        if t3704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3704[0].clone();
        v_v = t3704[1].clone();
        if ({ let t3685 = v_k.clone(); Py::Bool({ let t3686 = v_outputs.clone(); if !py::contains(&t3686, &t3685)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_37(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t39: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t38 = v_inputs.clone(); f_angle_section(py::getitem(&t38, &Py::str("b"))?, py::getitem(&t38, &Py::str("h"))?, py::getitem(&t38, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t39 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t40 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t60 = (t40).items()?;
        if t60.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t60[0].clone();
        v_v = t60[1].clone();
        if ({ let t41 = v_k.clone(); Py::Bool({ let t42 = v_outputs.clone(); if !py::contains(&t42, &t41)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3727(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3729: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3728 = v_inputs.clone(); f_sn_life(py::getitem(&t3728, &Py::str("Sut"))?, py::getitem(&t3728, &Py::str("Se"))?, py::getitem(&t3728, &Py::str("f"))?, py::getitem(&t3728, &Py::str("sa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3729 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3730 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3750 = (t3730).items()?;
        if t3750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3750[0].clone();
        v_v = t3750[1].clone();
        if ({ let t3731 = v_k.clone(); Py::Bool({ let t3732 = v_outputs.clone(); if !py::contains(&t3732, &t3731)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3773(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3775: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3774 = v_inputs.clone(); f_sn_strength(py::getitem(&t3774, &Py::str("Sut"))?, py::getitem(&t3774, &Py::str("Se"))?, py::getitem(&t3774, &Py::str("f"))?, py::getitem(&t3774, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3775 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3776 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3796 = (t3776).items()?;
        if t3796.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3796[0].clone();
        v_v = t3796[1].clone();
        if ({ let t3777 = v_k.clone(); Py::Bool({ let t3778 = v_outputs.clone(); if !py::contains(&t3778, &t3777)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3817(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3819: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3818 = v_inputs.clone(); f_soderberg(py::getitem(&t3818, &Py::str("sa"))?, py::getitem(&t3818, &Py::str("sm"))?, py::getitem(&t3818, &Py::str("Se"))?, py::getitem(&t3818, &Py::str("Sy"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3819 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3820 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3840 = (t3820).items()?;
        if t3840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3840[0].clone();
        v_v = t3840[1].clone();
        if ({ let t3821 = v_k.clone(); Py::Bool({ let t3822 = v_outputs.clone(); if !py::contains(&t3822, &t3821)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3841(mut v_T: Py, mut v_d: Py, mut v_L: Py, mut v_G: Py) -> R {
    return Ok(f_shaft_torsion(v_T.clone(), v_d.clone(), v_L.clone(), v_G.clone(), Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3862(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3864: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3863 = v_inputs.clone(); s_3841(py::getitem(&t3863, &Py::str("T"))?, py::getitem(&t3863, &Py::str("d"))?, py::getitem(&t3863, &Py::str("L"))?, py::getitem(&t3863, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3864 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3865 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3885 = (t3865).items()?;
        if t3885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3885[0].clone();
        v_v = t3885[1].clone();
        if ({ let t3866 = v_k.clone(); Py::Bool({ let t3867 = v_outputs.clone(); if !py::contains(&t3867, &t3866)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3916(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3918: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3917 = v_inputs.clone(); f_spur_contact(py::getitem(&t3917, &Py::str("N1"))?, py::getitem(&t3917, &Py::str("N2"))?, py::getitem(&t3917, &Py::str("m"))?, py::getitem(&t3917, &Py::str("phi"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3918 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3919 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3939 = (t3919).items()?;
        if t3939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3939[0].clone();
        v_v = t3939[1].clone();
        if ({ let t3920 = v_k.clone(); Py::Bool({ let t3921 = v_outputs.clone(); if !py::contains(&t3921, &t3920)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_395(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t397: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t396 = v_inputs.clone(); f_belt_power(py::getitem(&t396, &Py::str("T1"))?, py::getitem(&t396, &Py::str("m"))?, py::getitem(&t396, &Py::str("v"))?, py::getitem(&t396, &Py::str("mu"))?, py::getitem(&t396, &Py::str("theta"))?, py::method(&t396, "get", &[Py::str("beta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t397 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t398 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t418 = (t398).items()?;
        if t418.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t418[0].clone();
        v_v = t418[1].clone();
        if ({ let t399 = v_k.clone(); Py::Bool({ let t400 = v_outputs.clone(); if !py::contains(&t400, &t399)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3955(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3957: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3956 = v_inputs.clone(); f_fixed_shaft(py::getitem(&t3956, &Py::str("T0"))?, py::getitem(&t3956, &Py::str("a"))?, py::getitem(&t3956, &Py::str("b"))?, py::getitem(&t3956, &Py::str("d1"))?, py::getitem(&t3956, &Py::str("d2"))?, py::getitem(&t3956, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3957 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3958 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3978 = (t3958).items()?;
        if t3978.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3978[0].clone();
        v_v = t3978[1].clone();
        if ({ let t3959 = v_k.clone(); Py::Bool({ let t3960 = v_outputs.clone(); if !py::contains(&t3960, &t3959)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3998(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4000: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3999 = v_inputs.clone(); f_stepped_shaft(py::getitem(&t3999, &Py::str("T"))?, py::getitem(&t3999, &Py::str("lengths"))?, py::getitem(&t3999, &Py::str("diameters"))?, py::getitem(&t3999, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4000 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4001 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4021 = (t4001).items()?;
        if t4021.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4021[0].clone();
        v_v = t4021[1].clone();
        if ({ let t4002 = v_k.clone(); Py::Bool({ let t4003 = v_outputs.clone(); if !py::contains(&t4003, &t4002)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4053(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4055: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4054 = v_inputs.clone(); f_rosette(py::getitem(&t4054, &Py::str("ea"))?, py::getitem(&t4054, &Py::str("eb"))?, py::getitem(&t4054, &Py::str("ec"))?, py::getitem(&t4054, &Py::str("E"))?, py::getitem(&t4054, &Py::str("nu"))?, py::getitem(&t4054, &Py::str("layout"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4055 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4056 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4076 = (t4056).items()?;
        if t4076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4076[0].clone();
        v_v = t4076[1].clone();
        if ({ let t4057 = v_k.clone(); Py::Bool({ let t4058 = v_outputs.clone(); if !py::contains(&t4058, &t4057)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4111: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4110 = v_inputs.clone(); f_t_section(py::getitem(&t4110, &Py::str("bf"))?, py::getitem(&t4110, &Py::str("tf"))?, py::getitem(&t4110, &Py::str("h"))?, py::getitem(&t4110, &Py::str("tw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4111 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4112 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4132 = (t4112).items()?;
        if t4132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4132[0].clone();
        v_v = t4132[1].clone();
        if ({ let t4113 = v_k.clone(); Py::Bool({ let t4114 = v_outputs.clone(); if !py::contains(&t4114, &t4113)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4165(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4167: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4166 = v_inputs.clone(); f_lame(py::getitem(&t4166, &Py::str("pi"))?, py::getitem(&t4166, &Py::str("po"))?, py::getitem(&t4166, &Py::str("a"))?, py::getitem(&t4166, &Py::str("b"))?, py::method(&t4166, "get", &[Py::str("r"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4167 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4168 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4188 = (t4168).items()?;
        if t4188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4188[0].clone();
        v_v = t4188[1].clone();
        if ({ let t4169 = v_k.clone(); Py::Bool({ let t4170 = v_outputs.clone(); if !py::contains(&t4170, &t4169)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4208(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4210: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4209 = v_inputs.clone(); f_thin_vessel(py::getitem(&t4209, &Py::str("p"))?, py::getitem(&t4209, &Py::str("r"))?, py::getitem(&t4209, &Py::str("t"))?, py::getitem(&t4209, &Py::str("shape"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4210 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4211 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4231 = (t4211).items()?;
        if t4231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4231[0].clone();
        v_v = t4231[1].clone();
        if ({ let t4212 = v_k.clone(); Py::Bool({ let t4213 = v_outputs.clone(); if !py::contains(&t4213, &t4212)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4342(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4344: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4343 = v_inputs.clone(); f_truss(py::getitem(&t4343, &Py::str("nodes"))?, py::getitem(&t4343, &Py::str("members"))?, py::getitem(&t4343, &Py::str("pin"))?, py::getitem(&t4343, &Py::str("roller"))?, py::getitem(&t4343, &Py::str("loads"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4344 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4345 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4365 = (t4345).items()?;
        if t4365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4365[0].clone();
        v_v = t4365[1].clone();
        if ({ let t4346 = v_k.clone(); Py::Bool({ let t4347 = v_outputs.clone(); if !py::contains(&t4347, &t4346)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4384(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4386: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4385 = v_inputs.clone(); f_two_cable_tension(py::getitem(&t4385, &Py::str("W"))?, py::getitem(&t4385, &Py::str("alpha"))?, py::getitem(&t4385, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4386 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4387 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4407 = (t4387).items()?;
        if t4407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4407[0].clone();
        v_v = t4407[1].clone();
        if ({ let t4388 = v_k.clone(); Py::Bool({ let t4389 = v_outputs.clone(); if !py::contains(&t4389, &t4388)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_442(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t444: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t443 = v_inputs.clone(); f_capstan(py::getitem(&t443, &Py::str("T2"))?, py::getitem(&t443, &Py::str("mu"))?, py::getitem(&t443, &Py::str("theta"))?, py::method(&t443, "get", &[Py::str("beta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t444 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t445 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t465 = (t445).items()?;
        if t465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t465[0].clone();
        v_v = t465[1].clone();
        if ({ let t446 = v_k.clone(); Py::Bool({ let t447 = v_outputs.clone(); if !py::contains(&t447, &t446)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4424(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4426: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4425 = v_inputs.clone(); f_two_rotor(py::getitem(&t4425, &Py::str("J1"))?, py::getitem(&t4425, &Py::str("J2"))?, py::getitem(&t4425, &Py::str("kt"))?, py::method(&t4425, "get", &[Py::str("L"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4426 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4427 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4447 = (t4427).items()?;
        if t4447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4447[0].clone();
        v_v = t4447[1].clone();
        if ({ let t4428 = v_k.clone(); Py::Bool({ let t4429 = v_outputs.clone(); if !py::contains(&t4429, &t4428)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4464(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4466: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4465 = v_inputs.clone(); f_isolator_design(py::getitem(&t4465, &Py::str("m"))?, py::getitem(&t4465, &Py::str("omega"))?, py::getitem(&t4465, &Py::str("TR"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4466 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4467 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4487 = (t4467).items()?;
        if t4487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4487[0].clone();
        v_v = t4487[1].clone();
        if ({ let t4468 = v_k.clone(); Py::Bool({ let t4469 = v_outputs.clone(); if !py::contains(&t4469, &t4468)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4510(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4512: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4511 = v_inputs.clone(); f_transmissibility(py::getitem(&t4511, &Py::str("zeta"))?, py::getitem(&t4511, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4512 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4513 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4533 = (t4513).items()?;
        if t4533.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4533[0].clone();
        v_v = t4533[1].clone();
        if ({ let t4514 = v_k.clone(); Py::Bool({ let t4515 = v_outputs.clone(); if !py::contains(&t4515, &t4514)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4552(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4554: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4553 = v_inputs.clone(); f_wahl_factor(py::getitem(&t4553, &Py::str("d"))?, py::getitem(&t4553, &Py::str("D"))?, py::method(&t4553, "get", &[Py::str("F"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4554 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4555 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4575 = (t4555).items()?;
        if t4575.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4575[0].clone();
        v_v = t4575[1].clone();
        if ({ let t4556 = v_k.clone(); Py::Bool({ let t4557 = v_outputs.clone(); if !py::contains(&t4557, &t4556)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4623(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4625: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4624 = v_inputs.clone(); f_yield_safety(py::getitem(&t4624, &Py::str("Sy"))?, py::getitem(&t4624, &Py::str("sx"))?, py::method(&t4624, "get", &[Py::str("sy"), Py::Float(0.0)])?, py::method(&t4624, "get", &[Py::str("sz"), Py::Float(0.0)])?, py::method(&t4624, "get", &[Py::str("txy"), Py::Float(0.0)])?, py::method(&t4624, "get", &[Py::str("tyz"), Py::Float(0.0)])?, py::method(&t4624, "get", &[Py::str("tzx"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4625 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4626 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4646 = (t4626).items()?;
        if t4646.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4646[0].clone();
        v_v = t4646[1].clone();
        if ({ let t4627 = v_k.clone(); Py::Bool({ let t4628 = v_outputs.clone(); if !py::contains(&t4628, &t4627)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_491(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t493: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t492 = v_inputs.clone(); f_bolt_torque(py::getitem(&t492, &Py::str("K"))?, py::getitem(&t492, &Py::str("d"))?, py::method(&t492, "get", &[Py::str("F"), Py::None])?, py::method(&t492, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t493 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t494 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t514 = (t494).items()?;
        if t514.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t514[0].clone();
        v_v = t514[1].clone();
        if ({ let t495 = v_k.clone(); Py::Bool({ let t496 = v_outputs.clone(); if !py::contains(&t496, &t495)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_546(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t548: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t547 = v_inputs.clone(); f_brittle_mohr(py::getitem(&t547, &Py::str("sA"))?, py::getitem(&t547, &Py::str("sB"))?, py::getitem(&t547, &Py::str("Sut"))?, py::getitem(&t547, &Py::str("Suc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t548 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t549 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t569 = (t549).items()?;
        if t569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t569[0].clone();
        v_v = t569[1].clone();
        if ({ let t550 = v_k.clone(); Py::Bool({ let t551 = v_outputs.clone(); if !py::contains(&t551, &t550)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_588(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t590: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t589 = v_inputs.clone(); f_cantilever_point_load(py::getitem(&t589, &Py::str("L"))?, py::getitem(&t589, &Py::str("P"))?, py::getitem(&t589, &Py::str("E"))?, py::getitem(&t589, &Py::str("I"))?, py::method(&t589, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t590 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t591 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t611 = (t591).items()?;
        if t611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t611[0].clone();
        v_v = t611[1].clone();
        if ({ let t592 = v_k.clone(); Py::Bool({ let t593 = v_outputs.clone(); if !py::contains(&t593, &t592)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_625(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t627: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t626 = v_inputs.clone(); f_cantilever_triangular(py::getitem(&t626, &Py::str("L"))?, py::getitem(&t626, &Py::str("w"))?, py::getitem(&t626, &Py::str("E"))?, py::getitem(&t626, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t627 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t628 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t648 = (t628).items()?;
        if t648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t648[0].clone();
        v_v = t648[1].clone();
        if ({ let t629 = v_k.clone(); Py::Bool({ let t630 = v_outputs.clone(); if !py::contains(&t630, &t629)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_662(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t664: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t663 = v_inputs.clone(); f_cantilever_udl(py::getitem(&t663, &Py::str("L"))?, py::getitem(&t663, &Py::str("w"))?, py::getitem(&t663, &Py::str("E"))?, py::getitem(&t663, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t664 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t665 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t685 = (t665).items()?;
        if t685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t685[0].clone();
        v_v = t685[1].clone();
        if ({ let t666 = v_k.clone(); Py::Bool({ let t667 = v_outputs.clone(); if !py::contains(&t667, &t666)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t721: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t720 = v_inputs.clone(); f_channel_section(py::getitem(&t720, &Py::str("b"))?, py::getitem(&t720, &Py::str("h"))?, py::getitem(&t720, &Py::str("tf"))?, py::getitem(&t720, &Py::str("tw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t721 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t722 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t742 = (t722).items()?;
        if t742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t742[0].clone();
        v_v = t742[1].clone();
        if ({ let t723 = v_k.clone(); Py::Bool({ let t724 = v_outputs.clone(); if !py::contains(&t724, &t723)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_756(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t758: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t757 = v_inputs.clone(); f_circle_section(py::getitem(&t757, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t758 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t759 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t779 = (t759).items()?;
        if t779.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t779[0].clone();
        v_v = t779[1].clone();
        if ({ let t760 = v_k.clone(); Py::Bool({ let t761 = v_outputs.clone(); if !py::contains(&t761, &t760)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_801(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t803: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t802 = v_inputs.clone(); f_combined_rect(py::getitem(&t802, &Py::str("b"))?, py::getitem(&t802, &Py::str("h"))?, py::getitem(&t802, &Py::str("N"))?, py::getitem(&t802, &Py::str("Mx"))?, py::getitem(&t802, &Py::str("My"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t803 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t804 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t824 = (t804).items()?;
        if t824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t824[0].clone();
        v_v = t824[1].clone();
        if ({ let t805 = v_k.clone(); Py::Bool({ let t806 = v_outputs.clone(); if !py::contains(&t806, &t805)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_81(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t83: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t82 = v_inputs.clone(); f_asme_elliptic(py::getitem(&t82, &Py::str("sa"))?, py::getitem(&t82, &Py::str("sm"))?, py::getitem(&t82, &Py::str("Se"))?, py::getitem(&t82, &Py::str("Sy"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t83 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t84 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t104 = (t84).items()?;
        if t104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t104[0].clone();
        v_v = t104[1].clone();
        if ({ let t85 = v_k.clone(); Py::Bool({ let t86 = v_outputs.clone(); if !py::contains(&t86, &t85)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_884(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t886: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t885 = v_inputs.clone(); f_composite_rects(py::getitem(&t885, &Py::str("rects"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t886 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t887 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t907 = (t887).items()?;
        if t907.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t907[0].clone();
        v_v = t907[1].clone();
        if ({ let t888 = v_k.clone(); Py::Bool({ let t889 = v_outputs.clone(); if !py::contains(&t889, &t888)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_965(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t967: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t966 = v_inputs.clone(); f_continuous_beam(py::getitem(&t966, &Py::str("spans"))?, py::getitem(&t966, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t967 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t968 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t988 = (t968).items()?;
        if t988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t988[0].clone();
        v_v = t988[1].clone();
        if ({ let t969 = v_k.clone(); Py::Bool({ let t970 = v_outputs.clone(); if !py::contains(&t970, &t969)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
