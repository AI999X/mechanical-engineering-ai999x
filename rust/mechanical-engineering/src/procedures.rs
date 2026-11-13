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
    "beam_bending_stress",
    "beam_natural_frequency",
    "beam_shear_stress",
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
    "force_system_resultant",
    "forced_vibration_amplitude",
    "four_bar_linkage_position",
    "gear_train_speed_ratio",
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
    "moment_of_force_3d",
    "pin_shear_and_bearing_stress",
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
    "thermal_stress_restrained_bar",
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
        "beam_bending_stress" => {
            let r = f_beam_bending_stress(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_224(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beam_natural_frequency" => {
            let r = f_beam_frequency(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_280(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beam_shear_stress" => {
            let r = f_beam_shear_stress(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_324(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bearing_catalog_rating" => {
            let r = f_bearing_rating(match v.get("FD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("af") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("LR") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000000.0)) })().ok()? } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_379(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bearing_life_reliability" => {
            let r = f_bearing_life(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kind") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LR") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1000000.0)) })().ok()? } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_434(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "belt_drive_power" => {
            let r = f_belt_power(match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "belt_friction_capstan" => {
            let r = f_capstan(match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_530(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bolt_preload_torque" => {
            let r = f_bolt_torque(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_579(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "brittle_failure_mohr" => {
            let r = f_brittle_mohr(match v.get("sA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Suc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_634(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cantilever_point_load" => {
            let r = f_cantilever_point_load(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_676(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cantilever_triangular_load" => {
            let r = f_cantilever_triangular(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_713(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cantilever_udl" => {
            let r = f_cantilever_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_750(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "channel_section_properties" => {
            let r = f_channel_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_807(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circular_section_properties" => {
            let r = f_circle_section(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_844(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_axial_and_bending_stress" => {
            let r = f_combined_rect(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("My") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_889(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "composite_section_properties" => {
            let r = f_composite_rects(match v.get("rects") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_972(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "continuous_beam_three_moment" => {
            let r = f_continuous_beam(match v.get("spans") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "damped_sdof_parameters" => {
            let r = f_damped_sdof(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1100(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "disc_clutch_torque" => {
            let r = f_disc_clutch(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pairs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1144(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dunkerley_critical_speed" => {
            let r = f_dunkerley_shaft(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1198(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eccentric_bolt_group" => {
            let r = f_bolt_group(match v.get("bolts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1259(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "euler_buckling" => {
            let r = f_euler_column(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1310(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eurocode_fatigue_damage" => {
            let r = f_eurocode_damage(match v.get("ranges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dsc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fatigue_life_mean_stress_correction" => {
            let r = f_mean_stress_life(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1415(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fixed_fixed_beam_point_load" => {
            let r = f_ff_point(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fixed_fixed_beam_udl" => {
            let r = f_ff_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1493(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "force_system_resultant" => {
            let r = f_force_system(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("O") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forced_vibration_amplitude" => {
            let r = f_forced_response(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1642(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "four_bar_linkage_position" => {
            let r = f_four_bar(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("branch") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1694(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gear_train_speed_ratio" => {
            let r = f_gear_train(match v.get("drivers") { Some(x) => py::from_val(x), None => { return None } }, match v.get("driven") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_in") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1763(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "generalized_hookes_law" => {
            let r = f_hooke_3d(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tyz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tzx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1808(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gerber_fatigue_safety_factor" => {
            let r = f_gerber(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1854(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "goodman_fatigue_safety_factor" => {
            let r = f_goodman(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1901(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "helical_compression_spring" => {
            let r = f_helical_spring(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1940(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hollow_circular_section_properties" => {
            let r = f_tube_section(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1984(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hollow_rectangular_section_properties" => {
            let r = f_box_section(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2024(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hollow_shaft_torsion" => {
            let r = s_2048(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2069(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "i_section_properties" => {
            let r = f_i_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2126(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "johnson_column" => {
            let r = f_johnson_column(match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lewis_gear_bending_stress" => {
            let r = f_lewis_stress(match v.get("Wt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kv") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "marin_endurance_limit" => {
            let r = f_marin(match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("surface") { Some(x) => py::from_val(x), None => { return None } }, match v.get("load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.5)) })().ok()? } }, match v.get("kd") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("kf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2284(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mdof_spring_mass_chain" => {
            let r = f_mdof_chain(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("springs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ends") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2393(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "miner_damage_sn_curve" => {
            let r = f_miner_linear(match v.get("ranges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(2000000.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moment_of_a_force" => {
            let r = f_moment_of_force(match v.get("Fx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Fy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("y0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moment_of_force_3d" => {
            let r = f_moment_3d(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("O") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("axis") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2522(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pin_shear_and_bearing_stress" => {
            let r = f_pin_shear_bearing(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("n_bolts") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("planes") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("tau_allow") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("sb_allow") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2584(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_stress_transformation" => {
            let r = f_plane_transform(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2623(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "planetary_gear_train" => {
            let r = f_planetary(match v.get("Ns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ws") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("wr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("wc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2679(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_section_properties" => {
            let r = f_polygon_section(match v.get("vertices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2796(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_screw_torque" => {
            let r = f_power_screw(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("fc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("dc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2850(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "principal_stresses_2d" => {
            let r = f_principal_2d(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "principal_stresses_3d" => {
            let r = f_principal_3d(match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("txy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tyz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tzx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "propped_cantilever_point_load" => {
            let r = f_pc_point(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2996(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "propped_cantilever_udl" => {
            let r = f_pc_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3033(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rainflow_counting" => {
            let r = f_rainflow_count(match v.get("history") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3093(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rainflow_fatigue_damage" => {
            let r = f_rainflow_damage(match v.get("history") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(2000000.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3161(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rayleigh_critical_speed" => {
            let r = f_rayleigh_shaft(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangular_section_properties" => {
            let r = f_rect_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "resultant_of_concurrent_forces" => {
            let r = f_resultant_forces(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3309(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotating_disk_stresses" => {
            let r = f_rotating_disk(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ri") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotating_unbalance_response" => {
            let r = f_unbalance(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sdof_free_response" => {
            let r = f_sdof_free(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sdof_natural_frequency" => {
            let r = f_sdof_natural(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3493(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "secant_column" => {
            let r = f_secant_column(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3537(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_critical_speed_single_disk" => {
            let r = f_single_disk(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3577(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_diameter_de_goodman" => {
            let r = f_shaft_de_goodman_d(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kfs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3631(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_fatigue_safety_factor" => {
            let r = f_shaft_de_goodman_n(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kfs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3689(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_power_transmission" => {
            let r = f_shaft_power(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_allow") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shaft_whirling_speeds" => {
            let r = f_exact_whirl(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3821(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shrink_fit_pressure" => {
            let r = f_shrink_fit(match v.get("delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ri") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3866(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simply_supported_beam_point_load" => {
            let r = f_ss_point_load(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3909(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simply_supported_beam_point_loads" => {
            let r = f_ss_point_loads(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3967(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simply_supported_beam_udl" => {
            let r = f_ss_udl(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4004(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "slider_crank_kinematics" => {
            let r = f_slider_crank(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4048(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sn_fatigue_life" => {
            let r = f_sn_life(match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4094(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sn_fatigue_strength" => {
            let r = f_sn_strength(match v.get("Sut") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4140(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "soderberg_fatigue_safety_factor" => {
            let r = f_soderberg(match v.get("sa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Se") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solid_shaft_torsion" => {
            let r = s_4208(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4229(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spur_gear_contact_ratio" => {
            let r = f_spur_contact(match v.get("N1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "statically_indeterminate_shaft" => {
            let r = f_fixed_shaft(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4322(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stepped_shaft_twist" => {
            let r = f_stepped_shaft(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lengths") { Some(x) => py::from_val(x), None => { return None } }, match v.get("diameters") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4365(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "strain_rosette" => {
            let r = f_rosette(match v.get("ea") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ec") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("layout") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4420(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "t_section_properties" => {
            let r = f_t_section(match v.get("bf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4476(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thermal_stress_restrained_bar" => {
            let r = f_restrained_bar_thermal(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("gap") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4538(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thick_walled_cylinder" => {
            let r = f_lame(match v.get("pi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thin_walled_pressure_vessel" => {
            let r = f_thin_vessel(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("shape") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4637(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "truss_member_forces" => {
            let r = f_truss(match v.get("nodes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("members") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("roller") { Some(x) => py::from_val(x), None => { return None } }, match v.get("loads") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4771(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_cable_equilibrium" => {
            let r = f_two_cable_tension(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4813(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_rotor_torsional_vibration" => {
            let r = f_two_rotor(match v.get("J1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4853(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vibration_isolator_design" => {
            let r = f_isolator_design(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vibration_transmissibility" => {
            let r = f_transmissibility(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4939(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wahl_factor" => {
            let r = f_wahl_factor(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "yield_criteria_safety_factor" => {
            let r = f_yield_safety(match v.get("Sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("sz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("txy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("tyz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("tzx") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5052(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t257 = v_ends.clone(); Py::Bool({ let t258 = Py::str("simply-supported"); if py::eq(&t257, &t258) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?);
    }
    if ({ let t259 = v_ends.clone(); Py::Bool({ let t260 = Py::str("cantilever"); if py::eq(&t259, &t260) { true } else { false } }) }).truthy() {
        let t261 = (Py::tuple(vec![{ let v_ends = v_ends.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::math1("cos", &v_x.clone())?, &py::truediv(&Py::Int(1i64), &py::math1("cosh", &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::truediv(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?])).items()?;
        if t261.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t261[0].clone();
        v_c = t261[1].clone();
    } else {
        if ({ let t262 = v_ends.clone(); Py::Bool({ let t263 = Py::tuple(vec![Py::str("fixed-fixed"), Py::str("free-free")]); if py::contains(&t263, &t262)? { true } else { false } }) }).truthy() {
            let t264 = (Py::tuple(vec![{ let v_c = v_c.clone(); let v_ends = v_ends.clone(); let v_f = v_f.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("cos", &v_x.clone())?, &py::truediv(&Py::Int(1i64), &py::math1("cosh", &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::truediv(&py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?])).items()?;
            if t264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t264[0].clone();
            v_c = t264[1].clone();
        } else {
            let t265 = (Py::tuple(vec![{ let v_c = v_c.clone(); let v_ends = v_ends.clone(); let v_f = v_f.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("sin", &v_x.clone())?, &py::mul(&py::math1("cos", &v_x.clone())?, &py::math1("tanh", &v_x.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::truediv(&py::mul(&py::add(&py::mul(&Py::Int(4i64), &v_n.clone())?, &Py::Int(1i64))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(4i64))?])).items()?;
            if t265.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t265[0].clone();
            v_c = t265[1].clone();
        }
    }
    let t266 = (Py::tuple(vec![py::sub(&v_c.clone(), &Py::Float(0.5))?, py::add(&v_c.clone(), &Py::Float(0.5))?])).items()?;
    if t266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t266[0].clone();
    v_hi = t266[1].clone();
    if ({ let t267 = py::mul(&py::call(&v_f.clone(), &[v_lo.clone()])?, &py::call(&v_f.clone(), &[v_hi.clone()])?)?; Py::Bool({ let t268 = Py::Int(0i64); if py::gt(&t267, &t268)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no root bracketed"))));
    }
    for t269 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t269;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t270 = py::mul(&py::call(&v_f.clone(), &[v_lo.clone()])?, &py::call(&v_f.clone(), &[v_mid.clone()])?)?; Py::Bool({ let t271 = Py::Int(0i64); if py::le(&t270, &t271)? { true } else { false } }) }).truthy() {
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
    v_K = { let mut out = vec![]; for t2321 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t2321;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t2322 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2322;
        let t2323 = py::getitem(&v_K.clone(), &v_i.clone())?; let t2324 = v_i.clone();
        py::setitem(&t2323, &t2324, py::iadd(&py::getitem(&t2323, &t2324)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
        if ({ let t2325 = v_i.clone(); Py::Bool({ let t2326 = Py::Int(0i64); if py::gt(&t2325, &t2326)? { true } else { false } }) }).truthy() {
            let t2327 = py::getitem(&v_K.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2328 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t2327, &t2328, py::iadd(&py::getitem(&t2327, &t2328)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
            let t2329 = py::getitem(&v_K.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; let t2330 = v_i.clone();
            py::setitem(&t2329, &t2330, py::sub(&py::getitem(&t2329, &t2330)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
            let t2331 = py::getitem(&v_K.clone(), &v_i.clone())?; let t2332 = py::sub(&v_i.clone(), &Py::Int(1i64))?;
            py::setitem(&t2331, &t2332, py::sub(&py::getitem(&t2331, &t2332)?, &py::getitem(&v_springs.clone(), &v_i.clone())?)?)?;
        }
    }
    if ({ let t2333 = v_ends.clone(); Py::Bool({ let t2334 = Py::str("fixed-fixed"); if py::eq(&t2333, &t2334) { true } else { false } }) }).truthy() {
        let t2335 = py::getitem(&v_K.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?; let t2336 = py::sub(&v_n.clone(), &Py::Int(1i64))?;
        py::setitem(&t2335, &t2336, py::iadd(&py::getitem(&t2335, &t2336)?, &py::getitem(&v_springs.clone(), &v_n.clone())?)?)?;
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
    if ({ let t2417 = py::len(&v_ranges.clone())?; Py::Bool({ let t2418 = py::len(&v_counts.clone())?; if py::ne(&t2417, &t2418) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("ranges and counts must have the same number of entries"))));
    }
    for t2419 in py::iter(&(v_ranges.clone()))? {
        v_v = t2419;
        let _ = f__pos(Py::str("every stress range"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t2425 in py::iter(&(v_counts.clone()))? {
        v_v = t2425;
        let _ = f__pos(Py::str("every cycle count"), Py::tuple(vec![v_v.clone()]))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cross3(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(Py::list(vec![py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__de_check(mut v_Ma: Py, mut v_Mm: Py, mut v_Ta: Py, mut v_Tm: Py, mut v_Kf: Py, mut v_Kfs: Py, mut v_Se: Py, mut v_Sut: Py) -> R {
    let _ = f__nonneg(Py::str("the moments and torques"), Py::tuple(vec![v_Ma.clone(), v_Mm.clone(), v_Ta.clone(), v_Tm.clone()]))?;
    if ({ let t3669 = { let t3665 = v_Ma.clone(); Py::Bool({ let t3666 = Py::Int(0i64); if py::eq(&t3665, &t3666) { true } else { false } }) }; if !t3669.truthy() { t3669 } else { { let t3667 = v_Ta.clone(); Py::Bool({ let t3668 = Py::Int(0i64); if py::eq(&t3667, &t3668) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is no alternating load: a static check, not a fatigue one"))));
    }
    if ({ let t3674 = { let t3670 = v_Kf.clone(); Py::Bool({ let t3671 = Py::Int(1i64); if py::lt(&t3670, &t3671)? { true } else { false } }) }; if t3674.truthy() { t3674 } else { { let t3672 = v_Kfs.clone(); Py::Bool({ let t3673 = Py::Int(1i64); if py::lt(&t3672, &t3673)? { true } else { false } }) } } }).truthy() {
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
fn f__dot3(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::add(&py::add(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fat_check(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_S: Py) -> R {
    let _ = f__pos(Py::str("the alternating stress, Se and the strength"), Py::tuple(vec![v_sa.clone(), v_Se.clone(), v_S.clone()]))?;
    let _ = f__nonneg(Py::str("the mean stress (compressive means are not covered)"), Py::tuple(vec![v_sm.clone()]))?;
    if ({ let t4174 = v_sm.clone(); Py::Bool({ let t4175 = v_S.clone(); if py::ge(&t4174, &t4175)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mean stress must be below the strength"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5010 in py::iter(&(v_vals.clone()))? {
        v_v = t5010;
        if (py::not(&{ let t5011 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5011.truthy() { t5011 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__flex(mut v_positions: Py, mut v_L: Py, mut v_EI: Py) -> R {
    return Ok({ let mut out = vec![]; for t3768 in py::iter(&(v_positions.clone()))? { let mut v_xi: Py; 
v_xi = t3768;
out.push({ let mut out = vec![]; for t3769 in py::iter(&(v_positions.clone()))? { let mut v_aj: Py; 
v_aj = t3769;
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
    v_M = { let mut out = vec![]; for t4733 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t4734 = (t4733).items()?;
if t4734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t4734[0].clone();
v_row = t4734[1].clone();
out.push(py::add(&py::slice(&v_row.clone(), None, None, None)?, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t4737 = py::minmax(&[{ let mut out = vec![]; for t4735 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t4735;
for t4736 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t4736;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?; if t4737.truthy() { t4737 } else { Py::Float(1.0) } };
    for t4738 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_col = t4738;
        v_piv = py::minmax_key(&[Py::list((py::range_iter(&[(v_col.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_col = v_col.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t4739 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_piv.clone())?, &v_col.clone())?)?; Py::Bool({ let t4740 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t4739, &t4740)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the structure is unstable or not statically determinate (singular equilibrium equations)"))));
        }
        let t4741 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_piv.clone())?, py::getitem(&v_M.clone(), &v_col.clone())?])).items()?;
        if t4741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_col.clone(), t4741[0].clone())?;
        py::setitem(&v_M.clone(), &v_piv.clone(), t4741[1].clone())?;
        for t4742 in py::range_iter(&[(py::add(&v_col.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t4742;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_col.clone())?, &v_col.clone())?)?;
            if (v_f.clone()).truthy() {
                for t4743 in py::range_iter(&[(v_col.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_c = t4743;
                    let t4744 = py::getitem(&v_M.clone(), &v_r.clone())?; let t4745 = v_c.clone();
                    py::setitem(&t4744, &t4745, py::sub(&py::getitem(&t4744, &t4745)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_col.clone())?, &v_c.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t4746 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_r = t4746;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::fsum(&{ let mut out = vec![]; for t4747 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t4747;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&v_x.clone(), &v_c.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hist_check(mut v_history: Py) -> R {
    if ({ let t3117 = py::len(&v_history.clone())?; Py::Bool({ let t3118 = Py::Int(3i64); if py::lt(&t3117, &t3118)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least 3 points of the load history"))));
    }
    let _ = f__fin(Py::str("every point"), { let mut v = vec![]; v.extend((v_history.clone()).items()?); Py::tuple(v) })?;
    if ({ let t3121 = py::minmax(&[v_history.clone()], true)?; Py::Bool({ let t3122 = py::minmax(&[v_history.clone()], false)?; if py::eq(&t3121, &t3122) { true } else { false } }) }).truthy() {
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
    if ({ let t2098 = py::mul(&Py::Int(2i64), &v_tf.clone())?; Py::Bool({ let t2099 = v_h.clone(); if py::ge(&t2098, &t2099)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the flanges (2 tf) must be thinner than the overall depth h"))));
    }
    if ({ let t2100 = v_tw.clone(); Py::Bool({ let t2101 = v_b.clone(); if py::ge(&t2100, &t2101)? { true } else { false } }) }).truthy() {
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
    v_a = { let mut out = vec![]; for t5013 in py::iter(&(v_S.clone()))? { let mut v_row: Py; 
v_row = t5013;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    for t5014 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
        v__sweep = t5014;
        v_off = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t5015 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t5015;
for t5016 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t5016;
if ({ let t5017 = v_i.clone(); Py::Bool({ let t5018 = v_j.clone(); if py::ne(&t5017, &t5018) { true } else { false } }) }).truthy() { out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }}} Py::list(out) }, None)?)?;
        v_nrm = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t5019 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t5019;
for t5020 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t5020;
out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }} Py::list(out) }, None)?)?;
        if ({ let t5025 = { let t5021 = v_off.clone(); Py::Bool({ let t5022 = py::mul(&Py::Float(1e-17), &v_nrm.clone())?; if py::le(&t5021, &t5022)? { true } else { false } }) }; if t5025.truthy() { t5025 } else { { let t5023 = v_off.clone(); Py::Bool({ let t5024 = Py::Int(0i64); if py::eq(&t5023, &t5024) { true } else { false } }) } } }).truthy() {
            break;
        }
        for t5026 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])])))? {
            let t5035 = (t5026).items()?;
            if t5035.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p = t5035[0].clone();
            v_q = t5035[1].clone();
            if ({ let t5027 = py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?; Py::Bool({ let t5028 = Py::Int(0i64); if py::eq(&t5027, &t5028) { true } else { false } }) }).truthy() {
                continue;
            }
            v_th = py::truediv(&py::sub(&py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_q.clone())?, &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_p.clone())?)?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?)?)?;
            v_t = py::truediv(&py::copysign(&Py::Float(1.0), &v_th.clone())?, &py::add(&py::abs(&v_th.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_th.clone(), &v_th.clone())?, &Py::Int(1i64))?)?)?)?;
            v_c = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::mul(&v_t.clone(), &v_t.clone())?, &Py::Int(1i64))?)?)?;
            v_s = py::mul(&v_t.clone(), &v_c.clone())?;
            for t5029 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
                v_k = t5029;
                let t5030 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                if t5030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_akp = t5030[0].clone();
                v_akq = t5030[1].clone();
                let t5031 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_akp.clone())?, &py::mul(&v_s.clone(), &v_akq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_akp.clone())?, &py::mul(&v_c.clone(), &v_akq.clone())?)?])).items()?;
                if t5031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone(), t5031[0].clone())?;
                py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone(), t5031[1].clone())?;
            }
            for t5032 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
                v_k = t5032;
                let t5033 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone())?])).items()?;
                if t5033.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_apk = t5033[0].clone();
                v_aqk = t5033[1].clone();
                let t5034 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_apk.clone())?, &py::mul(&v_s.clone(), &v_aqk.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_apk.clone())?, &py::mul(&v_c.clone(), &v_aqk.clone())?)?])).items()?;
                if t5034.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone(), t5034[0].clone())?;
                py::setitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone(), t5034[1].clone())?;
            }
        }
    }
    return Ok(py::sorted(&{ let mut out = vec![]; for t5036 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t5036;
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
    v_a = { let mut out = vec![]; for t3775 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t3775;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    v_V = { let mut out = vec![]; for t3776 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3776;
out.push({ let mut out = vec![]; for t3777 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3777;
out.push((if ({ let t3778 = v_i.clone(); Py::Bool({ let t3779 = v_j.clone(); if py::eq(&t3778, &t3779) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    for t3780 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__sweep = t3780;
        v_off = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t3781 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3781;
for t3782 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3782;
if ({ let t3783 = v_i.clone(); Py::Bool({ let t3784 = v_j.clone(); if py::ne(&t3783, &t3784) { true } else { false } }) }).truthy() { out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }}} Py::list(out) }, None)?)?;
        v_nrm = py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t3785 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3785;
for t3786 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3786;
out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }} Py::list(out) }, None)?)?;
        if ({ let t3791 = { let t3787 = v_off.clone(); Py::Bool({ let t3788 = py::mul(&Py::Float(1e-17), &v_nrm.clone())?; if py::le(&t3787, &t3788)? { true } else { false } }) }; if t3791.truthy() { t3791 } else { { let t3789 = v_off.clone(); Py::Bool({ let t3790 = Py::Int(0i64); if py::eq(&t3789, &t3790) { true } else { false } }) } } }).truthy() {
            break;
        }
        for t3792 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_p = t3792;
            for t3793 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_q = t3793;
                if ({ let t3794 = py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?; Py::Bool({ let t3795 = Py::Int(0i64); if py::eq(&t3794, &t3795) { true } else { false } }) }).truthy() {
                    continue;
                }
                v_th = py::truediv(&py::sub(&py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_q.clone())?, &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_p.clone())?)?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?)?)?;
                v_t = py::truediv(&py::copysign(&Py::Float(1.0), &v_th.clone())?, &py::add(&py::abs(&v_th.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_th.clone(), &v_th.clone())?, &Py::Int(1i64))?)?)?)?;
                v_c = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::mul(&v_t.clone(), &v_t.clone())?, &Py::Int(1i64))?)?)?;
                v_s = py::mul(&v_t.clone(), &v_c.clone())?;
                for t3796 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t3796;
                    let t3797 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                    if t3797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_akp = t3797[0].clone();
                    v_akq = t3797[1].clone();
                    let t3798 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_akp.clone())?, &py::mul(&v_s.clone(), &v_akq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_akp.clone())?, &py::mul(&v_c.clone(), &v_akq.clone())?)?])).items()?;
                    if t3798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone(), t3798[0].clone())?;
                    py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone(), t3798[1].clone())?;
                }
                for t3799 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t3799;
                    let t3800 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone())?])).items()?;
                    if t3800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_apk = t3800[0].clone();
                    v_aqk = t3800[1].clone();
                    let t3801 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_apk.clone())?, &py::mul(&v_s.clone(), &v_aqk.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_apk.clone())?, &py::mul(&v_c.clone(), &v_aqk.clone())?)?])).items()?;
                    if t3801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone(), t3801[0].clone())?;
                    py::setitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone(), t3801[1].clone())?;
                }
                for t3802 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t3802;
                    let t3803 = (Py::tuple(vec![py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                    if t3803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_vkp = t3803[0].clone();
                    v_vkq = t3803[1].clone();
                    let t3804 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_vkp.clone())?, &py::mul(&v_s.clone(), &v_vkq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_vkp.clone())?, &py::mul(&v_c.clone(), &v_vkq.clone())?)?])).items()?;
                    if t3804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_p.clone(), t3804[0].clone())?;
                    py::setitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_q.clone(), t3804[1].clone())?;
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
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t3805 in py::iter(&(v_order.clone()))? { let mut v_i: Py; 
v_i = t3805;
out.push(py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3806 in py::iter(&(v_order.clone()))? { let mut v_i: Py; 
v_i = t3806;
out.push({ let mut out = vec![]; for t3807 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3807;
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
    if ({ let t2155 = v_ends.clone(); Py::Bool({ let t2156 = Py::str("pinned-pinned"); if py::eq(&t2155, &t2156) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t2157 = v_ends.clone(); Py::Bool({ let t2158 = Py::str("fixed-free"); if py::eq(&t2157, &t2158) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(2.0));
    }
    if ({ let t2159 = v_ends.clone(); Py::Bool({ let t2160 = Py::str("fixed-fixed"); if py::eq(&t2159, &t2160) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.5));
    }
    let t2161 = (Py::tuple(vec![py::add(&Py::Float(std::f64::consts::PI), &Py::Float(1e-09))?, py::sub(&py::mul(&Py::Float(1.5), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-09))?])).items()?;
    if t2161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2161[0].clone();
    v_hi = t2161[1].clone();
    for t2162 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t2162;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t2163 = py::sub(&py::math1("tan", &v_mid.clone())?, &v_mid.clone())?; Py::Bool({ let t2164 = Py::Int(0i64); if py::gt(&t2163, &t2164)? { true } else { false } }) }).truthy() {
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
    for t4917 in py::iter(&(v_vals.clone()))? {
        v_v = t4917;
        if (py::not(&{ let t4921 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4921.truthy() { t4921 } else { { let t4920 = py::math1("isfinite", &v_v.clone())?; if !t4920.truthy() { t4920 } else { { let t4918 = v_v.clone(); Py::Bool({ let t4919 = Py::Int(0i64); if py::ge(&t4918, &t4919)? { true } else { false } }) } } } } })).truthy() {
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
    v_big = py::minmax(&[{ let mut out = vec![]; for t2375 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2375;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?;
    v_piv = (match ('g2376: { let mut out = vec![]; for t2377 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2377;
if ({ let t2378 = py::abs(&v_x.clone())?; Py::Bool({ let t2379 = py::mul(&v_big.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::ge(&t2378, &t2379)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); break 'g2376 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    return Ok({ let mut out = vec![]; for t2380 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2380;
out.push(f__snap(py::truediv(&v_x.clone(), &v_piv.clone())?, Py::Float(1.0))?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5041 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5042 = (t5041).items()?;
if t5042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5042[0].clone();
v_v = t5042[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5005 in py::iter(&(v_vals.clone()))? {
        v_v = t5005;
        if (py::not(&{ let t5009 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5009.truthy() { t5009 } else { { let t5008 = py::math1("isfinite", &v_v.clone())?; if !t5008.truthy() { t5008 } else { { let t5006 = v_v.clone(); Py::Bool({ let t5007 = Py::Int(0i64); if py::gt(&t5006, &t5007)? { true } else { false } }) } } } } })).truthy() {
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
    let t2782 = (Py::tuple(vec![py::truediv(&py::add(&v_Ix.clone(), &v_Iy.clone())?, &Py::Int(2i64))?, py::hypot(&[py::truediv(&py::sub(&v_Ix.clone(), &v_Iy.clone())?, &Py::Int(2i64))?, v_Ixy.clone()])?])).items()?;
    if t2782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2782[0].clone();
    v_rr = t2782[1].clone();
    if ({ let t2783 = v_rr.clone(); Py::Bool({ let t2784 = py::mul(&Py::Float(1e-12), &v_c.clone())?; if py::le(&t2783, &t2784)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![v_c.clone(), v_c.clone(), Py::Float(0.0)]));
    }
    v_phi = py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&py::mul(&py::neg(&Py::Int(2i64))?, &v_Ixy.clone())?, &py::sub(&v_Ix.clone(), &v_Iy.clone())?)?)?)?;
    if ({ let t2785 = v_phi.clone(); Py::Bool({ let t2786 = py::neg(&Py::Int(90i64))?; if py::le(&t2785, &t2786)? { true } else { false } }) }).truthy() {
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
        return Ok({ let mut out = vec![]; for t5043 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5043;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5046 = { let t5044 = v_x.clone(); Py::Bool({ let t5045 = Py::Int(0i64); if py::eq(&t5044, &t5045) { true } else { false } }) }; if t5046.truthy() { t5046 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5047 = v_x.clone(); Py::Bool({ let t5048 = Py::Int(0i64); if py::eq(&t5047, &t5048) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
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
    let t3128 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t3128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t3128[0].clone();
    v_cyc = t3128[1].clone();
    for t3136 in py::iter(&(f__reversals(v_series.clone())?))? {
        v_p = t3136;
        let _ = py::method(&v_pts.clone(), "append", &[v_p.clone()])?;
        while ({ let t3142 = py::len(&v_pts.clone())?; Py::Bool({ let t3143 = Py::Int(3i64); if py::ge(&t3142, &t3143)? { true } else { false } }) }).truthy() {
            let t3137 = (Py::tuple(vec![py::abs(&py::sub(&py::getitem(&v_pts.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_pts.clone(), &py::neg(&Py::Int(2i64))?)?)?)?, py::abs(&py::sub(&py::getitem(&v_pts.clone(), &py::neg(&Py::Int(2i64))?)?, &py::getitem(&v_pts.clone(), &py::neg(&Py::Int(3i64))?)?)?)?])).items()?;
            if t3137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_X = t3137[0].clone();
            v_Y = t3137[1].clone();
            if ({ let t3138 = v_X.clone(); Py::Bool({ let t3139 = v_Y.clone(); if py::lt(&t3138, &t3139)? { true } else { false } }) }).truthy() {
                break;
            }
            if ({ let t3140 = py::len(&v_pts.clone())?; Py::Bool({ let t3141 = Py::Int(3i64); if py::eq(&t3140, &t3141) { true } else { false } }) }).truthy() {
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
    for t3144 in py::range_iter(&[(py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t3144;
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
    v_A = py::fsum(&{ let mut out = vec![]; for t4453 in py::iter(&(v_rects.clone()))? { let mut v__x: Py; let mut v__y: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; 
let t4454 = (t4453).items()?;
if t4454.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4454[0].clone();
v_b = t4454[1].clone();
v_h = t4454[2].clone();
v__x = t4454[3].clone();
v__y = t4454[4].clone();
out.push(py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?); } Py::list(out) })?;
    if ({ let t4455 = v_A.clone(); Py::Bool({ let t4456 = Py::Int(0i64); if py::le(&t4455, &t4456)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the section has no positive area"))));
    }
    v_xc = py::truediv(&py::fsum(&{ let mut out = vec![]; for t4457 in py::iter(&(v_rects.clone()))? { let mut v__y: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_x: Py; 
let t4458 = (t4457).items()?;
if t4458.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4458[0].clone();
v_b = t4458[1].clone();
v_h = t4458[2].clone();
v_x = t4458[3].clone();
v__y = t4458[4].clone();
out.push(py::mul(&py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?, &v_x.clone())?); } Py::list(out) })?, &v_A.clone())?;
    v_yc = py::truediv(&py::fsum(&{ let mut out = vec![]; for t4459 in py::iter(&(v_rects.clone()))? { let mut v__x: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_y: Py; 
let t4460 = (t4459).items()?;
if t4460.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4460[0].clone();
v_b = t4460[1].clone();
v_h = t4460[2].clone();
v__x = t4460[3].clone();
v_y = t4460[4].clone();
out.push(py::mul(&py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?, &v_y.clone())?); } Py::list(out) })?, &v_A.clone())?;
    v_Ix = py::fsum(&{ let mut out = vec![]; for t4461 in py::iter(&(v_rects.clone()))? { let mut v__x: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_y: Py; 
let t4462 = (t4461).items()?;
if t4462.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4462[0].clone();
v_b = t4462[1].clone();
v_h = t4462[2].clone();
v__x = t4462[3].clone();
v_y = t4462[4].clone();
out.push(py::mul(&v_s.clone(), &py::add(&py::truediv(&py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, &py::mul(&py::mul(&v_b.clone(), &v_h.clone())?, &py::pow(&py::sub(&v_y.clone(), &v_yc.clone())?, &Py::Int(2i64))?)?)?)?); } Py::list(out) })?;
    v_Iy = py::fsum(&{ let mut out = vec![]; for t4463 in py::iter(&(v_rects.clone()))? { let mut v__y: Py; let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_x: Py; 
let t4464 = (t4463).items()?;
if t4464.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4464[0].clone();
v_b = t4464[1].clone();
v_h = t4464[2].clone();
v_x = t4464[3].clone();
v__y = t4464[4].clone();
out.push(py::mul(&v_s.clone(), &py::add(&py::truediv(&py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, &py::mul(&py::mul(&v_b.clone(), &v_h.clone())?, &py::pow(&py::sub(&v_x.clone(), &v_xc.clone())?, &Py::Int(2i64))?)?)?)?); } Py::list(out) })?;
    v_Ixy = py::fsum(&{ let mut out = vec![]; for t4465 in py::iter(&(v_rects.clone()))? { let mut v_b: Py; let mut v_h: Py; let mut v_s: Py; let mut v_x: Py; let mut v_y: Py; 
let t4466 = (t4465).items()?;
if t4466.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_s = t4466[0].clone();
v_b = t4466[1].clone();
v_h = t4466[2].clone();
v_x = t4466[3].clone();
v_y = t4466[4].clone();
out.push(py::mul(&py::mul(&py::mul(&py::mul(&v_s.clone(), &v_b.clone())?, &v_h.clone())?, &py::sub(&v_x.clone(), &v_xc.clone())?)?, &py::sub(&v_y.clone(), &v_yc.clone())?)?); } Py::list(out) })?;
    return Ok(Py::tuple(vec![v_A.clone(), v_xc.clone(), v_yc.clone(), v_Ix.clone(), v_Iy.clone(), v_Ixy.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t5058 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5058.truthy() { t5058 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5059 = v_a.clone(); Py::Bool({ let t5060 = v_b.clone(); if py::eq(&t5059, &t5060) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    if ({ let t5061 = Py::Bool(matches!(v_a.clone(), Py::List(_))); if t5061.truthy() { t5061 } else { Py::Bool(matches!(v_b.clone(), Py::List(_))) } }).truthy() {
        if ({ let t5065 = py::not(&{ let t5062 = Py::Bool(matches!(v_a.clone(), Py::List(_))); if !t5062.truthy() { t5062 } else { Py::Bool(matches!(v_b.clone(), Py::List(_))) } }); if t5065.truthy() { t5065 } else { { let t5063 = py::len(&v_a.clone())?; Py::Bool({ let t5064 = py::len(&v_b.clone())?; if py::ne(&t5063, &t5064) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[{ let t5068 = { let mut out = vec![]; for t5066 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5067 = (t5066).items()?;
if t5067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5067[0].clone();
v_y = t5067[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t5068.truthy() { t5068 } else { Py::list(vec![Py::Float(0.0)]) } }], true)?);
    }
    let t5070: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5069 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5069[0].clone();
            v_b = t5069[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5070 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5071 = v_d.clone(); Py::Bool({ let t5072 = Py::Int(0i64); if py::eq(&t5071, &t5072) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t5073 = v_s.clone(); Py::Bool({ let t5074 = Py::Int(0i64); if py::gt(&t5073, &t5074)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
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
    let t3129 = (Py::tuple(vec![py::getitem(&v_series.clone(), &Py::Int(1i64))?, py::sub(&py::getitem(&v_series.clone(), &Py::Int(1i64))?, &py::getitem(&v_series.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3129[0].clone();
    v_d_last = t3129[1].clone();
    for t3130 in py::iter(&(py::slice(&v_series.clone(), Some((Py::Int(2i64)).i()?), None, None)?))? {
        v_x_next = t3130;
        if ({ let t3131 = v_x_next.clone(); Py::Bool({ let t3132 = v_x.clone(); if py::eq(&t3131, &t3132) { true } else { false } }) }).truthy() {
            continue;
        }
        v_d_next = py::sub(&v_x_next.clone(), &v_x.clone())?;
        if ({ let t3133 = py::mul(&v_d_last.clone(), &v_d_next.clone())?; Py::Bool({ let t3134 = Py::Int(0i64); if py::lt(&t3133, &t3134)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_x.clone()])?;
        }
        let t3135 = (Py::tuple(vec![v_x_next.clone(), v_d_next.clone()])).items()?;
        if t3135.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t3135[0].clone();
        v_d_last = t3135[1].clone();
    }
    let _ = py::method(&v_out.clone(), "append", &[py::getitem(&v_series.clone(), &py::neg(&Py::Int(1i64))?)?])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rows3(mut v_name: Py, mut v_rows: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_row: Py = Py::None;
    v_out = Py::list(vec![]);
    for t1517 in py::iter(&(v_rows.clone()))? {
        v_row = t1517;
        let _ = py::method(&v_out.clone(), "append", &[f__vec3(v_name.clone(), v_row.clone())?])?;
    }
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
            return Ok((if ({ let t2723 = v_v.clone(); Py::Bool({ let t2724 = Py::Int(0i64); if py::eq(&t2723, &t2724) { true } else { false } }) }).truthy() { Py::Int(0i64) } else { (if ({ let t2725 = v_v.clone(); Py::Bool({ let t2726 = Py::Int(0i64); if py::gt(&t2725, &t2726)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2727 = (Py::tuple(vec![py::call(&v_orient.clone(), &[v_p1.clone(), v_p2.clone(), v_p3.clone()])?, py::call(&v_orient.clone(), &[v_p1.clone(), v_p2.clone(), v_p4.clone()])?, py::call(&v_orient.clone(), &[v_p3.clone(), v_p4.clone(), v_p1.clone()])?, py::call(&v_orient.clone(), &[v_p3.clone(), v_p4.clone(), v_p2.clone()])?])).items()?;
    if t2727.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_o1 = t2727[0].clone();
    v_o2 = t2727[1].clone();
    v_o3 = t2727[2].clone();
    v_o4 = t2727[3].clone();
    if ({ let t2732 = { let t2728 = v_o1.clone(); Py::Bool({ let t2729 = v_o2.clone(); if py::ne(&t2728, &t2729) { true } else { false } }) }; if !t2732.truthy() { t2732 } else { { let t2730 = v_o3.clone(); Py::Bool({ let t2731 = v_o4.clone(); if py::ne(&t2730, &t2731) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Bool(true));
    }
    v_on = { let v_o1 = v_o1.clone(); let v_o2 = v_o2.clone(); let v_o3 = v_o3.clone(); let v_o4 = v_o4.clone(); let v_orient = v_orient.clone(); let v_p1 = v_p1.clone(); let v_p2 = v_p2.clone(); let v_p3 = v_p3.clone(); let v_p4 = v_p4.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok({ let t2739 = { let t2733 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], false)?; Py::Bool({ let t2734 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; if py::le(&t2733, &t2734)? { { let t2735 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], true)?; if py::le(&t2734, &t2735)? { true } else { false } } } else { false } }) }; if !t2739.truthy() { t2739 } else { { let t2736 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], false)?; Py::Bool({ let t2737 = py::getitem(&v_c.clone(), &Py::Int(1i64))?; if py::le(&t2736, &t2737)? { { let t2738 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], true)?; if py::le(&t2737, &t2738)? { true } else { false } } } else { false } }) } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok({ let t2754 = { let t2742 = { let t2740 = v_o1.clone(); Py::Bool({ let t2741 = Py::Int(0i64); if py::eq(&t2740, &t2741) { true } else { false } }) }; if !t2742.truthy() { t2742 } else { py::call(&v_on.clone(), &[v_p1.clone(), v_p2.clone(), v_p3.clone()])? } }; if t2754.truthy() { t2754 } else { { let t2753 = { let t2745 = { let t2743 = v_o2.clone(); Py::Bool({ let t2744 = Py::Int(0i64); if py::eq(&t2743, &t2744) { true } else { false } }) }; if !t2745.truthy() { t2745 } else { py::call(&v_on.clone(), &[v_p1.clone(), v_p2.clone(), v_p4.clone()])? } }; if t2753.truthy() { t2753 } else { { let t2752 = { let t2748 = { let t2746 = v_o3.clone(); Py::Bool({ let t2747 = Py::Int(0i64); if py::eq(&t2746, &t2747) { true } else { false } }) }; if !t2748.truthy() { t2748 } else { py::call(&v_on.clone(), &[v_p3.clone(), v_p4.clone(), v_p1.clone()])? } }; if t2752.truthy() { t2752 } else { { let t2751 = { let t2749 = v_o4.clone(); Py::Bool({ let t2750 = Py::Int(0i64); if py::eq(&t2749, &t2750) { true } else { false } }) }; if !t2751.truthy() { t2751 } else { py::call(&v_on.clone(), &[v_p3.clone(), v_p4.clone(), v_p2.clone()])? } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shaft_check(mut v_masses: Py, mut v_positions: Py, mut v_L: Py, mut v_E: Py, mut v_I: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("L, E and I"), Py::tuple(vec![v_L.clone(), v_E.clone(), v_I.clone()]))?;
    if ({ let t3760 = { let t3755 = py::len(&v_masses.clone())?; Py::Bool({ let t3756 = py::len(&v_positions.clone())?; if py::ne(&t3755, &t3756) { true } else { false } }) }; if t3760.truthy() { t3760 } else { py::not(&{ let t3757 = Py::Int(1i64); Py::Bool({ let t3758 = py::len(&v_masses.clone())?; if py::le(&t3757, &t3758)? { { let t3759 = Py::Int(10i64); if py::le(&t3758, &t3759)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("masses and positions must be lists of the same length (1 to 10 entries)"))));
    }
    for t3761 in py::iter(&(v_masses.clone()))? {
        v_v = t3761;
        let _ = f__pos(Py::str("every mass"), Py::tuple(vec![v_v.clone()]))?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t3762 in py::iter(&(v_positions.clone()))? { let mut v_p: Py; 
v_p = t3762;
out.push(py::not(&{ let t3763 = Py::Int(0i64); Py::Bool({ let t3764 = v_p.clone(); if py::lt(&t3763, &t3764)? { { let t3765 = v_L.clone(); if py::lt(&t3764, &t3765)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every disk must sit between the bearings (0 < position < L)"))));
    }
    if ({ let t3766 = py::len(&py::set(&v_positions.clone())?)?; Py::Bool({ let t3767 = py::len(&v_positions.clone())?; if py::ne(&t3766, &t3767) { true } else { false } }) }).truthy() {
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
    return Ok((if ({ let t5038 = py::abs(&v_x.clone())?; Py::Bool({ let t5039 = py::mul(&Py::Float(1e-12), &py::abs(&v_scale.clone())?)?; if py::le(&t5038, &t5039)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ss_influence(mut v_x: Py, mut v_a: Py, mut v_L: Py) -> R {
    let mut v_b: Py = Py::None;
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    if ({ let t3770 = v_x.clone(); Py::Bool({ let t3771 = v_a.clone(); if py::le(&t3770, &t3771)? { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::mul(&py::mul(&v_b.clone(), &v_x.clone())?, &py::sub(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_x.clone(), &v_x.clone())?)?)?, &py::mul(&Py::Int(6i64), &v_L.clone())?)?);
    }
    return Ok(py::truediv(&py::mul(&py::mul(&v_a.clone(), &py::sub(&v_L.clone(), &v_x.clone())?)?, &py::sub(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::pow(&py::sub(&v_L.clone(), &v_x.clone())?, &Py::Int(2i64))?)?)?, &py::mul(&Py::Int(6i64), &v_L.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stress_scale(mut v_v: Py) -> R {
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5012 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5012;
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
    v_pts = { let mut out = vec![]; for t4661 in py::iter(&(v_nodes.clone()))? { let mut v_r: Py; 
v_r = t4661;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) };
    if ({ let t4668 = Py::Bool(({ let mut out = vec![]; for t4662 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t4662;
out.push({ let t4663 = py::len(&v_p.clone())?; Py::Bool({ let t4664 = Py::Int(2i64); if py::ne(&t4663, &t4664) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t4668.truthy() { t4668 } else { py::not(&{ let t4665 = Py::Int(3i64); Py::Bool({ let t4666 = py::len(&v_pts.clone())?; if py::le(&t4665, &t4666)? { { let t4667 = Py::Int(30i64); if py::le(&t4666, &t4667)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 3 to 30 nodes as rows 'x y' (m)"))));
    }
    v_n = py::len(&v_pts.clone())?;
    v_mem = Py::list(vec![]);
    for t4669 in py::iter(&(v_members.clone()))? {
        v_e = t4669;
        if ({ let t4670 = py::len(&v_e.clone())?; Py::Bool({ let t4671 = Py::Int(2i64); if py::ne(&t4670, &t4671) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("members are pairs of node numbers, e.g. 1-2"))));
        }
        let t4673: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                let t4672 = (Py::tuple(vec![py::int(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)?])).items()?;
                if t4672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_i = t4672[0].clone();
                v_j = t4672[1].clone();
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4673 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                return Err(py::value_error(py::to_str(&Py::str("members are pairs of node numbers, e.g. 1-2"))));
            }
            Err(e) => return Err(e),
        }
        if ({ let t4683 = py::not(&{ let t4680 = { let t4674 = Py::Int(1i64); Py::Bool({ let t4675 = v_i.clone(); if py::le(&t4674, &t4675)? { { let t4676 = v_n.clone(); if py::le(&t4675, &t4676)? { true } else { false } } } else { false } }) }; if !t4680.truthy() { t4680 } else { { let t4677 = Py::Int(1i64); Py::Bool({ let t4678 = v_j.clone(); if py::le(&t4677, &t4678)? { { let t4679 = v_n.clone(); if py::le(&t4678, &t4679)? { true } else { false } } } else { false } }) } } }); if t4683.truthy() { t4683 } else { { let t4681 = v_i.clone(); Py::Bool({ let t4682 = v_j.clone(); if py::eq(&t4681, &t4682) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["member ".to_string(), py::format(&py::getitem(&v_e.clone(), &Py::Int(0i64))?, "")?, "-".to_string(), py::format(&py::getitem(&v_e.clone(), &Py::Int(1i64))?, "")?, " does not join two different nodes".to_string()].concat()))));
        }
        if ({ let t4684 = py::getitem(&v_pts.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t4685 = py::getitem(&v_pts.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?; if py::eq(&t4684, &t4685) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a member has zero length"))));
        }
        let _ = py::method(&v_mem.clone(), "append", &[Py::tuple(vec![py::sub(&v_i.clone(), &Py::Int(1i64))?, py::sub(&v_j.clone(), &Py::Int(1i64))?])])?;
    }
    if ({ let t4686 = py::len(&py::set(&{ let mut out = vec![]; for t4688 in py::iter(&(v_mem.clone()))? { let mut v_m: Py; 
v_m = t4688;
out.push(Py::tuple((py::sorted(&v_m.clone(), (Py::Bool(false)).truthy())?).items()?)); } Py::list(out) })?)?; Py::Bool({ let t4687 = py::len(&v_mem.clone())?; if py::ne(&t4686, &t4687) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a member is listed twice"))));
    }
    if ({ let t4698 = py::not(&{ let t4695 = { let t4689 = Py::Int(1i64); Py::Bool({ let t4690 = v_pin.clone(); if py::le(&t4689, &t4690)? { { let t4691 = v_n.clone(); if py::le(&t4690, &t4691)? { true } else { false } } } else { false } }) }; if !t4695.truthy() { t4695 } else { { let t4692 = Py::Int(1i64); Py::Bool({ let t4693 = v_roller.clone(); if py::le(&t4692, &t4693)? { { let t4694 = v_n.clone(); if py::le(&t4693, &t4694)? { true } else { false } } } else { false } }) } } }); if t4698.truthy() { t4698 } else { { let t4696 = v_pin.clone(); Py::Bool({ let t4697 = v_roller.clone(); if py::eq(&t4696, &t4697) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pin and the roller must be two different nodes"))));
    }
    v_ld = Py::list(vec![]);
    for t4699 in py::iter(&(v_loads.clone()))? {
        v_row = t4699;
        if ({ let t4708 = { let t4700 = py::len(&v_row.clone())?; Py::Bool({ let t4701 = Py::Int(3i64); if py::ne(&t4700, &t4701) { true } else { false } }) }; if t4708.truthy() { t4708 } else { { let t4707 = { let t4702 = py::getitem(&v_row.clone(), &Py::Int(0i64))?; Py::Bool({ let t4703 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::ne(&t4702, &t4703) { true } else { false } }) }; if t4707.truthy() { t4707 } else { py::not(&{ let t4704 = Py::Int(1i64); Py::Bool({ let t4705 = py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?; if py::le(&t4704, &t4705)? { { let t4706 = v_n.clone(); if py::le(&t4705, &t4706)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each load is a row 'node Fx Fy' (kN)"))));
        }
        let _ = py::method(&v_ld.clone(), "append", &[Py::tuple(vec![py::sub(&py::int(&py::getitem(&v_row.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?, py::getitem(&v_row.clone(), &Py::Int(1i64))?, py::getitem(&v_row.clone(), &Py::Int(2i64))?])])?;
    }
    if ({ let t4709 = py::add(&py::len(&v_mem.clone())?, &Py::Int(3i64))?; Py::Bool({ let t4710 = py::mul(&Py::Int(2i64), &v_n.clone())?; if py::ne(&t4709, &t4710) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["not statically determinate: ".to_string(), py::format(&py::len(&v_mem.clone())?, "")?, " members + 3 reactions must equal 2 x ".to_string(), py::format(&v_n.clone(), "")?, " joint equations".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![v_pts.clone(), v_mem.clone(), py::sub(&v_pin.clone(), &Py::Int(1i64))?, py::sub(&v_roller.clone(), &Py::Int(1i64))?, v_ld.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec3(mut v_name: Py, mut v_v: Py) -> R {
    if ({ let t2499 = py::len(&v_v.clone())?; Py::Bool({ let t2500 = Py::Int(3i64); if py::ne(&t2499, &t2500) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a vector of three components [x, y, z]".to_string()].concat()))));
    }
    let _ = f__fin(v_name.clone(), Py::tuple(vec![py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?]))?;
    return Ok(Py::list(vec![py::float(&py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, py::float(&py::getitem(&v_v.clone(), &Py::Int(1i64))?)?, py::float(&py::getitem(&v_v.clone(), &Py::Int(2i64))?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__weib(mut v_x0: Py, mut v_theta: Py, mut v_b: Py) -> R {
    v_x0 = (if ({ let t411 = v_x0.clone(); Py::Bool({ let t412 = Py::None; if py::is(&t411, &t412) { true } else { false } }) }).truthy() { py::getitem(&c__WEIBULL()?, &Py::str("x0"))? } else { v_x0.clone() });
    v_theta = (if ({ let t413 = v_theta.clone(); Py::Bool({ let t414 = Py::None; if py::is(&t413, &t414) { true } else { false } }) }).truthy() { py::getitem(&c__WEIBULL()?, &Py::str("theta"))? } else { v_theta.clone() });
    v_b = (if ({ let t415 = v_b.clone(); Py::Bool({ let t416 = Py::None; if py::is(&t415, &t416) { true } else { false } }) }).truthy() { py::getitem(&c__WEIBULL()?, &Py::str("b"))? } else { v_b.clone() });
    if (py::not(&{ let t422 = { let t417 = Py::Int(0i64); Py::Bool({ let t418 = v_x0.clone(); if py::le(&t417, &t418)? { { let t419 = v_theta.clone(); if py::lt(&t418, &t419)? { true } else { false } } } else { false } }) }; if !t422.truthy() { t422 } else { { let t420 = v_b.clone(); Py::Bool({ let t421 = Py::Int(0i64); if py::gt(&t420, &t421)? { true } else { false } }) } } })).truthy() {
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
    if ({ let t517 = { let t512 = v_beta.clone(); Py::Bool({ let t513 = Py::None; if !py::is(&t512, &t513) { true } else { false } }) }; if !t517.truthy() { t517 } else { py::not(&{ let t514 = Py::Int(0i64); Py::Bool({ let t515 = v_beta.clone(); if py::lt(&t514, &t515)? { { let t516 = Py::Float(std::f64::consts::PI); if py::lt(&t515, &t516)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the groove angle must lie between 0 and 180 degrees"))));
    }
    v_mu_e = (if ({ let t518 = v_beta.clone(); Py::Bool({ let t519 = Py::None; if !py::is(&t518, &t519) { true } else { false } }) }).truthy() { py::truediv(&v_mu.clone(), &py::math1("sin", &py::truediv(&v_beta.clone(), &Py::Int(2i64))?)?)? } else { v_mu.clone() });
    if ({ let t520 = py::mul(&v_mu_e.clone(), &v_theta.clone())?; Py::Bool({ let t521 = Py::Int(50i64); if py::gt(&t520, &t521)? { true } else { false } }) }).truthy() {
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
fn f_beam_bending_stress(mut v_M: Py, mut v_I: Py, mut v_y: Py, mut v_c: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__fin(Py::str("M and y"), Py::tuple(vec![v_M.clone(), v_y.clone()]))?;
    let _ = f__pos(Py::str("the second moment of area I"), Py::tuple(vec![v_I.clone()]))?;
    v_s = py::truediv(&py::mul(&py::neg(&v_M.clone())?, &v_y.clone())?, &v_I.clone())?;
    v_out = Py::dict(vec![(Py::str("sigma"), v_s.clone()), (Py::str("sigma_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t211 = v_c.clone(); Py::Bool({ let t212 = Py::None; if !py::is(&t211, &t212) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the extreme-fibre distance c"), Py::tuple(vec![v_c.clone()]))?;
        if ({ let t213 = py::abs(&v_y.clone())?; Py::Bool({ let t214 = py::mul(&v_c.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t213, &t214)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the point must lie inside the section: |y| must not exceed the extreme-fibre distance c"))));
        }
        py::setitem(&v_out.clone(), &Py::str("sigma_max_MPa"), py::truediv(&py::truediv(&py::mul(&py::abs(&v_M.clone())?, &v_c.clone())?, &v_I.clone())?, &Py::Float(1000000.0))?)?;
        py::setitem(&v_out.clone(), &Py::str("S"), py::truediv(&v_I.clone(), &v_c.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beam_frequency(mut v_E: Py, mut v_I: Py, mut v_rho: Py, mut v_A: Py, mut v_L: Py, mut v_ends: Py, mut v_n: Py) -> R {
    let mut v_bl: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("E, I, rho, A and L"), Py::tuple(vec![v_E.clone(), v_I.clone(), v_rho.clone(), v_A.clone(), v_L.clone()]))?;
    if (py::not(&{ let t256 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t256.truthy() { t256 } else { { let t253 = Py::Int(1i64); Py::Bool({ let t254 = v_n.clone(); if py::le(&t253, &t254)? { { let t255 = Py::Int(6i64); if py::le(&t254, &t255)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mode number must be 1 to 6"))));
    }
    v_bl = f__beta_L(v_ends.clone(), v_n.clone())?;
    v_w = py::mul(&py::mul(&v_bl.clone(), &v_bl.clone())?, &py::math1("sqrt", &py::truediv(&py::mul(&v_E.clone(), &v_I.clone())?, &py::mul(&py::mul(&v_rho.clone(), &v_A.clone())?, &py::pow(&v_L.clone(), &Py::Int(4i64))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta_L"), v_bl.clone()), (Py::str("omega"), v_w.clone()), (Py::str("f"), py::truediv(&v_w.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beam_shear_stress(mut v_V: Py, mut v_Q: Py, mut v_I: Py, mut v_b: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__fin(Py::str("the shear force V"), Py::tuple(vec![v_V.clone()]))?;
    let _ = f__pos(Py::str("I and the width b"), Py::tuple(vec![v_I.clone(), v_b.clone()]))?;
    let _ = f__nonneg(Py::str("the first moment of area Q"), Py::tuple(vec![v_Q.clone()]))?;
    v_q = py::truediv(&py::mul(&v_V.clone(), &v_Q.clone())?, &v_I.clone())?;
    v_tau = py::truediv(&v_q.clone(), &v_b.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("tau_MPa"), py::truediv(&v_tau.clone(), &Py::Float(1000000.0))?), (Py::str("q"), v_q.clone()), (Py::str("q_kN_per_m"), py::truediv(&v_q.clone(), &Py::Float(1000.0))?)]))?);
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
    if (py::not(&{ let t408 = Py::Int(0i64); Py::Bool({ let t409 = v_R.clone(); if py::lt(&t408, &t409)? { { let t410 = Py::Int(1i64); if py::lt(&t409, &t410)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reliability must lie strictly between 0 and 1"))));
    }
    let t423 = (f__weib(v_x0.clone(), v_theta.clone(), v_b.clone())?).items()?;
    if t423.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t423[0].clone();
    v_theta = t423[1].clone();
    v_b = t423[2].clone();
    v_a = (if ({ let t424 = v_kind.clone(); Py::Bool({ let t425 = Py::str("ball"); if py::eq(&t424, &t425) { true } else { false } }) }).truthy() { Py::Float(3.0) } else { py::truediv(&Py::Int(10i64), &Py::Int(3i64))? });
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
    if (py::not(&{ let t353 = Py::Int(0i64); Py::Bool({ let t354 = v_R.clone(); if py::lt(&t353, &t354)? { { let t355 = Py::Int(1i64); if py::lt(&t354, &t355)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reliability must lie strictly between 0 and 1"))));
    }
    let t368 = (f__weib(v_x0.clone(), v_theta.clone(), v_b.clone())?).items()?;
    if t368.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t368[0].clone();
    v_theta = t368[1].clone();
    v_b = t368[2].clone();
    v_a = (if ({ let t369 = v_kind.clone(); Py::Bool({ let t370 = Py::str("ball"); if py::eq(&t369, &t370) { true } else { false } }) }).truthy() { Py::Float(3.0) } else { py::truediv(&Py::Int(10i64), &Py::Int(3i64))? });
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
    if ({ let t473 = v_Tc.clone(); Py::Bool({ let t474 = v_T1.clone(); if py::ge(&t473, &t474)? { true } else { false } }) }).truthy() {
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
    v_pts = { let mut out = vec![]; for t1222 in py::iter(&(v_bolts.clone()))? { let mut v_r: Py; 
v_r = t1222;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) };
    if ({ let t1229 = Py::Bool(({ let mut out = vec![]; for t1223 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1223;
out.push({ let t1224 = py::len(&v_p.clone())?; Py::Bool({ let t1225 = Py::Int(2i64); if py::ne(&t1224, &t1225) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1229.truthy() { t1229 } else { py::not(&{ let t1226 = Py::Int(2i64); Py::Bool({ let t1227 = py::len(&v_pts.clone())?; if py::le(&t1226, &t1227)? { { let t1228 = Py::Int(40i64); if py::le(&t1227, &t1228)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 40 bolts as rows 'x y' (mm)"))));
    }
    let _ = f__pos(Py::str("the load P"), Py::tuple(vec![v_P.clone()]))?;
    let _ = f__fin(Py::str("the eccentricity e"), Py::tuple(vec![v_e.clone()]))?;
    v_n = py::len(&v_pts.clone())?;
    let t1239 = (Py::tuple(vec![py::truediv(&py::fsum(&{ let mut out = vec![]; for t1237 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1237;
out.push(py::getitem(&v_p.clone(), &Py::Int(0i64))?); } Py::list(out) })?, &v_n.clone())?, py::truediv(&py::fsum(&{ let mut out = vec![]; for t1238 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1238;
out.push(py::getitem(&v_p.clone(), &Py::Int(1i64))?); } Py::list(out) })?, &v_n.clone())?])).items()?;
    if t1239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xc = t1239[0].clone();
    v_yc = t1239[1].clone();
    v_J = py::fsum(&{ let mut out = vec![]; for t1240 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1241 = (t1240).items()?;
if t1241.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1241[0].clone();
v_y = t1241[1].clone();
out.push(py::add(&py::pow(&py::sub(&v_x.clone(), &v_xc.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_y.clone(), &v_yc.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    if ({ let t1242 = v_J.clone(); Py::Bool({ let t1243 = Py::Int(0i64); if py::le(&t1242, &t1243)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the bolts must not all coincide"))));
    }
    v_em = py::mul(&v_e.clone(), &Py::Float(1000.0))?;
    v_M = py::mul(&v_P.clone(), &v_em.clone())?;
    v_forces = Py::list(vec![]);
    for t1244 in py::iter(&(v_pts.clone()))? {
        let t1245 = (t1244).items()?;
        if t1245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1245[0].clone();
        v_y = t1245[1].clone();
        v_fx = py::truediv(&py::mul(&v_M.clone(), &py::sub(&v_y.clone(), &v_yc.clone())?)?, &v_J.clone())?;
        v_fy = py::sub(&py::truediv(&py::neg(&v_P.clone())?, &v_n.clone())?, &py::truediv(&py::mul(&v_M.clone(), &py::sub(&v_x.clone(), &v_xc.clone())?)?, &v_J.clone())?)?;
        let _ = py::method(&v_forces.clone(), "append", &[py::hypot(&[v_fx.clone(), v_fy.clone()])?])?;
    }
    v_fmax = py::minmax(&[v_forces.clone()], true)?;
    v_idx = py::add(&(match ('g1246: { let mut out = vec![]; for t1247 in py::iter(&(py::enumerate(&v_forces.clone(), 0)?))? { let mut v_f: Py; let mut v_i: Py; 
let t1248 = (t1247).items()?;
if t1248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1248[0].clone();
v_f = t1248[1].clone();
if ({ let t1249 = v_f.clone(); Py::Bool({ let t1250 = py::mul(&v_fmax.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::ge(&t1249, &t1250)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g1246 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }), &Py::Int(1i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("direct_kN"), py::truediv(&py::truediv(&v_P.clone(), &v_n.clone())?, &Py::Float(1000.0))?), (Py::str("max_bolt_force_kN"), py::truediv(&v_fmax.clone(), &Py::Float(1000.0))?), (Py::str("critical_bolt"), v_idx.clone()), (Py::str("polar_sum_mm2"), v_J.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bolt_torque(mut v_K: Py, mut v_d: Py, mut v_F: Py, mut v_T: Py) -> R {
    if (py::not(&{ let t557 = py::math1("isfinite", &v_K.clone())?; if !t557.truthy() { t557 } else { { let t554 = Py::Float(0.05); Py::Bool({ let t555 = v_K.clone(); if py::le(&t554, &t555)? { { let t556 = Py::Float(0.5); if py::le(&t555, &t556)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the nut (torque) factor K must lie between 0.05 and 0.5 (typically 0.2)"))));
    }
    let _ = f__pos(Py::str("d"), Py::tuple(vec![v_d.clone()]))?;
    if ({ let t563 = { let t565 = v_F.clone(); Py::Bool({ let t566 = Py::None; if py::is(&t565, &t566) { true } else { false } }) }; Py::Bool({ let t564 = { let t567 = v_T.clone(); Py::Bool({ let t568 = Py::None; if py::is(&t567, &t568) { true } else { false } }) }; if py::eq(&t563, &t564) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the preload F and the torque T"))));
    }
    if ({ let t569 = v_F.clone(); Py::Bool({ let t570 = Py::None; if !py::is(&t569, &t570) { true } else { false } }) }).truthy() {
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
    if ({ let t2013 = py::mul(&Py::Int(2i64), &v_t.clone())?; Py::Bool({ let t2014 = py::minmax(&[v_B.clone(), v_H.clone()], false)?; if py::ge(&t2013, &t2014)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wall thickness must be less than half the smaller outer dimension"))));
    }
    let t2015 = (Py::tuple(vec![py::sub(&v_B.clone(), &py::mul(&Py::Int(2i64), &v_t.clone())?)?, py::sub(&v_H.clone(), &py::mul(&Py::Int(2i64), &v_t.clone())?)?])).items()?;
    if t2015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2015[0].clone();
    v_h = t2015[1].clone();
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
    let t610 = (Py::tuple(vec![py::minmax(&[v_sA.clone(), v_sB.clone()], true)?, py::minmax(&[v_sA.clone(), v_sB.clone()], false)?])).items()?;
    if t610.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t610[0].clone();
    v_s3 = t610[1].clone();
    if ({ let t615 = { let t611 = v_s1.clone(); Py::Bool({ let t612 = Py::Int(0i64); if py::eq(&t611, &t612) { true } else { false } }) }; if !t615.truthy() { t615 } else { { let t613 = v_s3.clone(); Py::Bool({ let t614 = Py::Int(0i64); if py::eq(&t613, &t614) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stress state is zero"))));
    }
    v_mns = py::minmax(&[(if ({ let t616 = v_s1.clone(); Py::Bool({ let t617 = Py::Int(0i64); if py::gt(&t616, &t617)? { true } else { false } }) }).truthy() { py::truediv(&v_Sut.clone(), &v_s1.clone())? } else { Py::Float(f64::INFINITY) }), (if ({ let t618 = v_s3.clone(); Py::Bool({ let t619 = Py::Int(0i64); if py::lt(&t618, &t619)? { true } else { false } }) }).truthy() { py::truediv(&v_Suc.clone(), &py::neg(&v_s3.clone())?)? } else { Py::Float(f64::INFINITY) })], false)?;
    v_bcm = py::truediv(&Py::Float(1.0), &py::sub(&py::truediv(&py::minmax(&[v_s1.clone(), Py::Float(0.0)], true)?, &v_Sut.clone())?, &py::truediv(&py::minmax(&[v_s3.clone(), Py::Float(0.0)], false)?, &v_Suc.clone())?)?)?;
    if ({ let t620 = v_s3.clone(); Py::Bool({ let t621 = Py::Int(0i64); if py::ge(&t620, &t621)? { true } else { false } }) }).truthy() {
        v_mm = py::truediv(&v_Sut.clone(), &v_s1.clone())?;
    } else {
        if ({ let t622 = v_s1.clone(); Py::Bool({ let t623 = Py::Int(0i64); if py::le(&t622, &t623)? { true } else { false } }) }).truthy() {
            v_mm = py::truediv(&v_Suc.clone(), &py::neg(&v_s3.clone())?)?;
        } else {
            if ({ let t624 = py::neg(&v_s3.clone())?; Py::Bool({ let t625 = v_s1.clone(); if py::le(&t624, &t625)? { true } else { false } }) }).truthy() {
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
    v_a = (if ({ let t663 = v_a.clone(); Py::Bool({ let t664 = Py::None; if py::is(&t663, &t664) { true } else { false } }) }).truthy() { v_L.clone() } else { v_a.clone() });
    if (py::not(&{ let t665 = Py::Int(0i64); Py::Bool({ let t666 = v_a.clone(); if py::lt(&t665, &t666)? { { let t667 = v_L.clone(); if py::le(&t666, &t667)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t783 = { let t779 = py::mul(&Py::Int(2i64), &v_tf.clone())?; Py::Bool({ let t780 = v_h.clone(); if py::ge(&t779, &t780)? { true } else { false } }) }; if t783.truthy() { t783 } else { { let t781 = v_tw.clone(); Py::Bool({ let t782 = v_b.clone(); if py::ge(&t781, &t782)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 2 tf < h and tw < b"))));
    }
    v_hw = py::sub(&v_h.clone(), &py::mul(&Py::Int(2i64), &v_tf.clone())?)?;
    let t798 = (f__rect_set(Py::list(vec![Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_tf.clone(), py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::sub(&v_h.clone(), &py::truediv(&v_tf.clone(), &Py::Int(2i64))?)?]), Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_tf.clone(), py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&v_tf.clone(), &Py::Int(2i64))?]), Py::tuple(vec![Py::Int(1i64), v_tw.clone(), v_hw.clone(), py::truediv(&v_tw.clone(), &Py::Int(2i64))?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])]))?).items()?;
    if t798.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t798[0].clone();
    v_xb = t798[1].clone();
    v__yc = t798[2].clone();
    v_Ix = t798[3].clone();
    v_Iy = t798[4].clone();
    v__ = t798[5].clone();
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
    let t875 = (Py::tuple(vec![py::mul(&v_b.clone(), &v_h.clone())?, py::truediv(&py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, py::truediv(&py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?])).items()?;
    if t875.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t875[0].clone();
    v_Ix = t875[1].clone();
    v_Iy = t875[2].clone();
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
    if ({ let t876 = v_sc.clone(); Py::Bool({ let t877 = Py::Int(0i64); if py::eq(&t876, &t877) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no load is given"))));
    }
    v_corners = Py::list(vec![py::call(&v_sig.clone(), &[py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])?, py::call(&v_sig.clone(), &[py::truediv(&py::neg(&v_b.clone())?, &Py::Int(2i64))?, py::truediv(&v_h.clone(), &Py::Int(2i64))?])?, py::call(&v_sig.clone(), &[py::truediv(&py::neg(&v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::neg(&v_h.clone())?, &Py::Int(2i64))?])?, py::call(&v_sig.clone(), &[py::truediv(&v_b.clone(), &Py::Int(2i64))?, py::truediv(&py::neg(&v_h.clone())?, &Py::Int(2i64))?])?]);
    v_corners = { let mut out = vec![]; for t878 in py::iter(&(v_corners.clone()))? { let mut v_v: Py; 
v_v = t878;
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
    v_rows = { let mut out = vec![]; for t913 in py::iter(&(v_rects.clone()))? { let mut v_r: Py; 
v_r = t913;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t914 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t914;
out.push({ let t915 = py::len(&v_r.clone())?; Py::Bool({ let t916 = Py::Int(4i64); if py::ne(&t915, &t916) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each rectangle is a row 'b h xc yc' (width, height, centre x, centre y in mm)"))));
    }
    if ({ let t919 = py::not(&v_rows.clone()); if t919.truthy() { t919 } else { { let t917 = py::len(&v_rows.clone())?; Py::Bool({ let t918 = Py::Int(20i64); if py::gt(&t917, &t918)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 to 20 rectangles"))));
    }
    for t920 in py::iter(&(v_rows.clone()))? {
        let t926 = (t920).items()?;
        if t926.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_b = t926[0].clone();
        v_h = t926[1].clone();
        v__x = t926[2].clone();
        v__y = t926[3].clone();
        let _ = f__pos(Py::str("each rectangle's b and h"), Py::tuple(vec![v_b.clone(), v_h.clone()]))?;
    }
    for t927 in py::range_iter(&[(py::len(&v_rows.clone())?).i()?]) {
        v_i = t927;
        for t928 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_rows.clone())?).i()?]) {
            v_j = t928;
            let t929 = (py::getitem(&v_rows.clone(), &v_i.clone())?).items()?;
            if t929.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_b1 = t929[0].clone();
            v_h1 = t929[1].clone();
            v_x1 = t929[2].clone();
            v_y1 = t929[3].clone();
            let t930 = (py::getitem(&v_rows.clone(), &v_j.clone())?).items()?;
            if t930.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_b2 = t930[0].clone();
            v_h2 = t930[1].clone();
            v_x2 = t930[2].clone();
            v_y2 = t930[3].clone();
            v_ox = py::sub(&py::minmax(&[py::add(&v_x1.clone(), &py::truediv(&v_b1.clone(), &Py::Int(2i64))?)?, py::add(&v_x2.clone(), &py::truediv(&v_b2.clone(), &Py::Int(2i64))?)?], false)?, &py::minmax(&[py::sub(&v_x1.clone(), &py::truediv(&v_b1.clone(), &Py::Int(2i64))?)?, py::sub(&v_x2.clone(), &py::truediv(&v_b2.clone(), &Py::Int(2i64))?)?], true)?)?;
            v_oy = py::sub(&py::minmax(&[py::add(&v_y1.clone(), &py::truediv(&v_h1.clone(), &Py::Int(2i64))?)?, py::add(&v_y2.clone(), &py::truediv(&v_h2.clone(), &Py::Int(2i64))?)?], false)?, &py::minmax(&[py::sub(&v_y1.clone(), &py::truediv(&v_h1.clone(), &Py::Int(2i64))?)?, py::sub(&v_y2.clone(), &py::truediv(&v_h2.clone(), &Py::Int(2i64))?)?], true)?)?;
            if ({ let t935 = { let t931 = v_ox.clone(); Py::Bool({ let t932 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_b1.clone(), v_b2.clone()], true)?)?; if py::gt(&t931, &t932)? { true } else { false } }) }; if !t935.truthy() { t935 } else { { let t933 = v_oy.clone(); Py::Bool({ let t934 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_h1.clone(), v_h2.clone()], true)?)?; if py::gt(&t933, &t934)? { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["rectangles ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?, " and ".to_string(), py::format(&py::add(&v_j.clone(), &Py::Int(1i64))?, "")?, " overlap: the parts of a composite section must not overlap".to_string()].concat()))));
            }
        }
    }
    let t952 = (f__rect_set({ let mut out = vec![]; for t950 in py::iter(&(v_rows.clone()))? { let mut v_b: Py; let mut v_h: Py; let mut v_x: Py; let mut v_y: Py; 
let t951 = (t950).items()?;
if t951.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_b = t951[0].clone();
v_h = t951[1].clone();
v_x = t951[2].clone();
v_y = t951[3].clone();
out.push(Py::tuple(vec![Py::Int(1i64), v_b.clone(), v_h.clone(), v_x.clone(), v_y.clone()])); } Py::list(out) })?).items()?;
    if t952.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t952[0].clone();
    v_xc = t952[1].clone();
    v_yc = t952[2].clone();
    v_Ix = t952[3].clone();
    v_Iy = t952[4].clone();
    v_Ixy = t952[5].clone();
    v_size = py::minmax(&[{ let mut out = vec![]; for t953 in py::iter(&(v_rows.clone()))? { let mut v_b: Py; let mut v_h: Py; let mut v_x: Py; let mut v_y: Py; 
let t954 = (t953).items()?;
if t954.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_b = t954[0].clone();
v_h = t954[1].clone();
v_x = t954[2].clone();
v_y = t954[3].clone();
out.push(py::add(&py::add(&py::add(&py::abs(&v_x.clone())?, &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?, &py::abs(&v_y.clone())?)?, &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?); } Py::list(out) }], true)?;
    let t957 = (Py::tuple(vec![f__snap(v_xc.clone(), v_size.clone())?, f__snap(v_yc.clone(), v_size.clone())?])).items()?;
    if t957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xc = t957[0].clone();
    v_yc = t957[1].clone();
    v_sc = py::minmax(&[v_Ix.clone(), v_Iy.clone()], true)?;
    v_Ixy = f__snap(v_Ixy.clone(), v_sc.clone())?;
    let t963 = (f__principal(v_Ix.clone(), v_Iy.clone(), v_Ixy.clone())?).items()?;
    if t963.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I1 = t963[0].clone();
    v_I2 = t963[1].clone();
    v_phi = t963[2].clone();
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
    if ({ let t1001 = { let t996 = py::len(&v_w.clone())?; Py::Bool({ let t997 = v_n.clone(); if py::ne(&t996, &t997) { true } else { false } }) }; if t1001.truthy() { t1001 } else { py::not(&{ let t998 = Py::Int(2i64); Py::Bool({ let t999 = v_n.clone(); if py::le(&t998, &t999)? { { let t1000 = Py::Int(6i64); if py::le(&t999, &t1000)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 6 spans and one uniform load per span"))));
    }
    for t1002 in py::iter(&(v_spans.clone()))? {
        v_v = t1002;
        let _ = f__pos(Py::str("every span"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t1008 in py::iter(&(v_w.clone()))? {
        v_v = t1008;
        let _ = f__nonneg(Py::str("every load"), Py::tuple(vec![v_v.clone()]))?;
    }
    if (py::not(&Py::Bool((v_w.clone()).items()?.iter().any(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no load"))));
    }
    v_k = py::sub(&v_n.clone(), &Py::Int(1i64))?;
    v_A = { let mut out = vec![]; for t1014 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v__: Py; 
v__ = t1014;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?); } Py::list(out) };
    v_b = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?;
    for t1015 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_i = t1015;
        let t1016 = (Py::tuple(vec![py::getitem(&v_spans.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_spans.clone(), &v_i.clone())?])).items()?;
        if t1016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_L1 = t1016[0].clone();
        v_L2 = t1016[1].clone();
        v_r = py::sub(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_r.clone(), py::mul(&Py::Int(2i64), &py::add(&v_L1.clone(), &v_L2.clone())?)?)?;
        if ({ let t1017 = v_r.clone(); Py::Bool({ let t1018 = Py::Int(0i64); if py::gt(&t1017, &t1018)? { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &py::sub(&v_r.clone(), &Py::Int(1i64))?, v_L1.clone())?;
        }
        if ({ let t1019 = v_r.clone(); Py::Bool({ let t1020 = py::sub(&v_k.clone(), &Py::Int(1i64))?; if py::lt(&t1019, &t1020)? { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &py::add(&v_r.clone(), &Py::Int(1i64))?, v_L2.clone())?;
        }
        py::setitem(&v_b.clone(), &v_r.clone(), py::truediv(&py::neg(&py::add(&py::mul(&py::getitem(&v_w.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::pow(&v_L1.clone(), &Py::Int(3i64))?)?, &py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::pow(&v_L2.clone(), &Py::Int(3i64))?)?)?)?, &Py::Int(4i64))?)?;
    }
    v_Mi = f__gauss_solve(v_A.clone(), v_b.clone())?;
    v_M = py::add(&py::add(&Py::list(vec![Py::Float(0.0)]), &v_Mi.clone())?, &Py::list(vec![Py::Float(0.0)]))?;
    v_R = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    for t1036 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1036;
        v_L = py::getitem(&v_spans.clone(), &v_i.clone())?;
        v_VL = py::add(&py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &v_L.clone())?, &Py::Int(2i64))?, &py::truediv(&py::sub(&py::getitem(&v_M.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_M.clone(), &v_i.clone())?)?, &v_L.clone())?)?;
        let t1037 = v_R.clone(); let t1038 = v_i.clone();
        py::setitem(&t1037, &t1038, py::iadd(&py::getitem(&t1037, &t1038)?, &v_VL.clone())?)?;
        let t1039 = v_R.clone(); let t1040 = py::add(&v_i.clone(), &Py::Int(1i64))?;
        py::setitem(&t1039, &t1040, py::iadd(&py::getitem(&t1039, &t1040)?, &py::sub(&py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &v_L.clone())?, &v_VL.clone())?)?)?;
    }
    v_sc = py::mul(&py::minmax(&[v_w.clone()], true)?, &py::pow(&py::minmax(&[v_spans.clone()], true)?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("support_moments_kNm"), { let mut out = vec![]; for t1049 in py::iter(&(v_M.clone()))? { let mut v_v: Py; 
v_v = t1049;
out.push(f__snap(v_v.clone(), v_sc.clone())?); } Py::list(out) }), (Py::str("reactions_kN"), { let mut out = vec![]; for t1052 in py::iter(&(v_R.clone()))? { let mut v_v: Py; 
v_v = t1052;
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
    if ({ let t1087 = v_z.clone(); Py::Bool({ let t1088 = Py::Int(1i64); if py::lt(&t1087, &t1088)? { true } else { false } }) }).truthy() {
        v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?;
        v_dec = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_z.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?;
        v_regime = Py::str("underdamped");
    } else {
        let t1091 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), (if ({ let t1089 = v_z.clone(); Py::Bool({ let t1090 = Py::Int(1i64); if py::eq(&t1089, &t1090) { true } else { false } }) }).truthy() { Py::str("critically damped") } else { Py::str("overdamped") })])).items()?;
        if t1091.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_wd = t1091[0].clone();
        v_dec = t1091[1].clone();
        v_regime = t1091[2].clone();
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
    if ({ let t1129 = v_d.clone(); Py::Bool({ let t1130 = v_D.clone(); if py::ge(&t1129, &t1130)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner diameter must be smaller than the outer diameter"))));
    }
    if (py::not(&{ let t1133 = Py::Bool(matches!(v_pairs.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1133.truthy() { t1133 } else { { let t1131 = v_pairs.clone(); Py::Bool({ let t1132 = Py::Int(1i64); if py::ge(&t1131, &t1132)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of friction pairs must be a whole number >= 1"))));
    }
    if ({ let t1134 = v_model.clone(); Py::Bool({ let t1135 = Py::str("uniform wear"); if py::eq(&t1134, &t1135) { true } else { false } }) }).truthy() {
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
    v_s = py::fsum(&{ let mut out = vec![]; for t1186 in py::iter(&(py::zip(&[v_masses.clone(), v_positions.clone()])?))? { let mut v_m: Py; let mut v_p: Py; 
let t1187 = (t1186).items()?;
if t1187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t1187[0].clone();
v_p = t1187[1].clone();
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
    if ({ let t1298 = v_A.clone(); Py::Bool({ let t1299 = Py::None; if !py::is(&t1298, &t1299) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("A"), Py::tuple(vec![v_A.clone()]))?;
        let _ = { let t1300 = v_out.clone(); py::setitem(&t1300, &Py::str("sigma_cr_MPa"), py::truediv(&py::truediv(&v_P.clone(), &v_A.clone())?, &Py::Float(1000000.0))?)?; py::setitem(&t1300, &Py::str("slenderness"), py::truediv(&v_Le.clone(), &py::math1("sqrt", &py::truediv(&v_I.clone(), &v_A.clone())?)?)?)?; Py::None };
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
    for t1343 in py::iter(&(py::zip(&[v_ranges.clone(), v_counts.clone()])?))? {
        let t1348 = (t1343).items()?;
        if t1348.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t1348[0].clone();
        v_n = t1348[1].clone();
        if ({ let t1344 = v_s.clone(); Py::Bool({ let t1345 = v_sD.clone(); if py::ge(&t1344, &t1345)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_terms.clone(), "append", &[py::truediv(&v_n.clone(), &py::mul(&Py::Float(2000000.0), &py::pow(&py::truediv(&v_dsc.clone(), &v_s.clone())?, &Py::Int(3i64))?)?)?])?;
        } else {
            if ({ let t1346 = v_s.clone(); Py::Bool({ let t1347 = v_sL.clone(); if py::ge(&t1346, &t1347)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_terms.clone(), "append", &[py::truediv(&v_n.clone(), &py::mul(&Py::Float(5000000.0), &py::pow(&py::truediv(&v_sD.clone(), &v_s.clone())?, &Py::Int(5i64))?)?)?])?;
            }
        }
    }
    v_D = py::fsum(&v_terms.clone())?;
    if ({ let t1349 = v_D.clone(); Py::Bool({ let t1350 = Py::Int(0i64); if py::eq(&t1349, &t1350) { true } else { false } }) }).truthy() {
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
    v_s = { let mut out = vec![]; for t3772 in py::iter(&(v_masses.clone()))? { let mut v_m: Py; 
v_m = t3772;
out.push(py::math1("sqrt", &v_m.clone())?); } Py::list(out) };
    v_n = py::len(&v_masses.clone())?;
    v_B = { let mut out = vec![]; for t3773 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3773;
out.push({ let mut out = vec![]; for t3774 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3774;
out.push(py::mul(&py::mul(&py::getitem(&v_s.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_s.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    let t3808 = (f__jacobi_sym(v_B.clone())?).items()?;
    if t3808.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t3808[0].clone();
    v__v = t3808[1].clone();
    v_w = py::sorted(&{ let mut out = vec![]; for t3809 in py::iter(&(v_lam.clone()))? { let mut v_l_: Py; 
v_l_ = t3809;
if ({ let t3810 = v_l_.clone(); Py::Bool({ let t3811 = Py::Int(0i64); if py::gt(&t3810, &t3811)? { true } else { false } }) }).truthy() { out.push(py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_l_.clone())?)?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("N_rpm"), { let mut out = vec![]; for t3820 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t3820;
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
    if (py::not(&{ let t1444 = Py::Int(0i64); Py::Bool({ let t1445 = v_a.clone(); if py::lt(&t1444, &t1445)? { { let t1446 = v_L.clone(); if py::lt(&t1445, &t1446)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load position a must lie strictly between the ends (0 < a < L)"))));
    }
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    let t1447 = (Py::tuple(vec![py::mul(&v_L.clone(), &v_L.clone())?, py::pow(&v_L.clone(), &Py::Int(3i64))?])).items()?;
    if t1447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L2 = t1447[0].clone();
    v_L3 = t1447[1].clone();
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
    let t4312 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d1.clone(), &Py::Int(4i64))?)?, &Py::Int(32i64))?, py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_d2.clone(), &Py::Int(4i64))?)?, &Py::Int(32i64))?])).items()?;
    if t4312.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_J1 = t4312[0].clone();
    v_J2 = t4312[1].clone();
    let t4313 = (Py::tuple(vec![py::truediv(&v_a.clone(), &py::mul(&v_G.clone(), &v_J1.clone())?)?, py::truediv(&v_b.clone(), &py::mul(&v_G.clone(), &v_J2.clone())?)?])).items()?;
    if t4313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t4313[0].clone();
    v_f2 = t4313[1].clone();
    v_TA = py::truediv(&py::mul(&v_T0.clone(), &v_f2.clone())?, &py::add(&v_f1.clone(), &v_f2.clone())?)?;
    v_TB = py::sub(&v_T0.clone(), &v_TA.clone())?;
    v_phi = py::mul(&v_TA.clone(), &v_f1.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("TA"), v_TA.clone()), (Py::str("TB"), v_TB.clone()), (Py::str("tau1_MPa"), py::truediv(&py::truediv(&py::truediv(&py::mul(&v_TA.clone(), &v_d1.clone())?, &Py::Int(2i64))?, &v_J1.clone())?, &Py::Float(1000000.0))?), (Py::str("tau2_MPa"), py::truediv(&py::truediv(&py::truediv(&py::mul(&v_TB.clone(), &v_d2.clone())?, &Py::Int(2i64))?, &v_J2.clone())?, &Py::Float(1000000.0))?), (Py::str("twist_at_load_deg"), py::math1("degrees", &v_phi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_force_system(mut v_F: Py, mut v_P: Py, mut v_C: Py, mut v_O: Py) -> R {
    let mut v_Cs: Py = Py::None;
    let mut v_Fs: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Mm: Py = Py::None;
    let mut v_Ps: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Rm: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fsum: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_msum: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_planar: Py = Py::None;
    let mut v_pt: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rm: Py = Py::None;
    v_Fs = f__rows3(Py::str("each force row of F"), v_F.clone())?;
    v_Ps = f__rows3(Py::str("each point row of P"), v_P.clone())?;
    if ({ let t1526 = { let t1522 = py::len(&v_Fs.clone())?; Py::Bool({ let t1523 = py::len(&v_Ps.clone())?; if py::ne(&t1522, &t1523) { true } else { false } }) }; if t1526.truthy() { t1526 } else { { let t1524 = py::len(&v_Fs.clone())?; Py::Bool({ let t1525 = Py::Int(0i64); if py::eq(&t1524, &t1525) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one point of application (a row of P) for every force (a row of F)"))));
    }
    v_Cs = (if ({ let t1527 = v_C.clone(); Py::Bool({ let t1528 = Py::None; if py::is(&t1527, &t1528) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__rows3(Py::str("each couple row of C"), v_C.clone())? });
    v_o = (if ({ let t1529 = v_O.clone(); Py::Bool({ let t1530 = Py::None; if py::is(&t1529, &t1530) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]) } else { f__vec3(Py::str("O"), v_O.clone())? });
    v_R = Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]);
    v_M = Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]);
    let t1531 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t1531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fsum = t1531[0].clone();
    v_msum = t1531[1].clone();
    for t1532 in py::iter(&(py::zip(&[v_Fs.clone(), v_Ps.clone()])?))? {
        let t1538 = (t1532).items()?;
        if t1538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t1538[0].clone();
        v_p = t1538[1].clone();
        v_a = Py::list(vec![py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_p.clone(), &Py::Int(2i64))?, &py::getitem(&v_o.clone(), &Py::Int(2i64))?)?]);
        v_m = f__cross3(v_a.clone(), v_f.clone())?;
        for t1533 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_k = t1533;
            let t1534 = v_R.clone(); let t1535 = v_k.clone();
            py::setitem(&t1534, &t1535, py::iadd(&py::getitem(&t1534, &t1535)?, &py::getitem(&v_f.clone(), &v_k.clone())?)?)?;
            let t1536 = v_M.clone(); let t1537 = v_k.clone();
            py::setitem(&t1536, &t1537, py::iadd(&py::getitem(&t1536, &t1537)?, &py::getitem(&v_m.clone(), &v_k.clone())?)?)?;
        }
        v_fsum = py::iadd(&v_fsum, &py::math1("sqrt", &f__dot3(v_f.clone(), v_f.clone())?)?)?;
        v_msum = py::iadd(&v_msum, &py::math1("sqrt", &f__dot3(v_m.clone(), v_m.clone())?)?)?;
    }
    for t1539 in py::iter(&(v_Cs.clone()))? {
        v_c = t1539;
        for t1540 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_k = t1540;
            let t1541 = v_M.clone(); let t1542 = v_k.clone();
            py::setitem(&t1541, &t1542, py::iadd(&py::getitem(&t1541, &t1542)?, &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        }
        v_msum = py::iadd(&v_msum, &py::math1("sqrt", &f__dot3(v_c.clone(), v_c.clone())?)?)?;
    }
    if ({ let t1547 = { let t1543 = v_fsum.clone(); Py::Bool({ let t1544 = Py::Int(0i64); if py::eq(&t1543, &t1544) { true } else { false } }) }; if !t1547.truthy() { t1547 } else { { let t1545 = v_msum.clone(); Py::Bool({ let t1546 = Py::Int(0i64); if py::eq(&t1545, &t1546) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every force and couple is zero"))));
    }
    v_R = Py::list(vec![f__snap(py::getitem(&v_R.clone(), &Py::Int(0i64))?, v_fsum.clone())?, f__snap(py::getitem(&v_R.clone(), &Py::Int(1i64))?, v_fsum.clone())?, f__snap(py::getitem(&v_R.clone(), &Py::Int(2i64))?, v_fsum.clone())?]);
    v_M = Py::list(vec![f__snap(py::getitem(&v_M.clone(), &Py::Int(0i64))?, v_msum.clone())?, f__snap(py::getitem(&v_M.clone(), &Py::Int(1i64))?, v_msum.clone())?, f__snap(py::getitem(&v_M.clone(), &Py::Int(2i64))?, v_msum.clone())?]);
    v_Rm = py::math1("sqrt", &f__dot3(v_R.clone(), v_R.clone())?)?;
    v_Mm = py::math1("sqrt", &f__dot3(v_M.clone(), v_M.clone())?)?;
    v_out = Py::dict(vec![(Py::str("Rx"), py::getitem(&v_R.clone(), &Py::Int(0i64))?), (Py::str("Ry"), py::getitem(&v_R.clone(), &Py::Int(1i64))?), (Py::str("Rz"), py::getitem(&v_R.clone(), &Py::Int(2i64))?), (Py::str("R"), v_Rm.clone()), (Py::str("MOx"), py::getitem(&v_M.clone(), &Py::Int(0i64))?), (Py::str("MOy"), py::getitem(&v_M.clone(), &Py::Int(1i64))?), (Py::str("MOz"), py::getitem(&v_M.clone(), &Py::Int(2i64))?), (Py::str("MO"), v_Mm.clone())]);
    if ({ let t1550 = v_Rm.clone(); Py::Bool({ let t1551 = Py::Int(0i64); if py::eq(&t1550, &t1551) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("result_type"), (if ({ let t1552 = v_Mm.clone(); Py::Bool({ let t1553 = Py::Int(0i64); if py::eq(&t1552, &t1553) { true } else { false } }) }).truthy() { Py::str("equilibrium") } else { Py::str("couple") }))?;
        return Ok(f__out(v_out.clone())?);
    }
    v_rm = f__dot3(v_R.clone(), v_M.clone())?;
    if ({ let t1563 = py::abs(&v_rm.clone())?; Py::Bool({ let t1564 = py::mul(&py::mul(&Py::Float(1e-09), &v_Rm.clone())?, &v_Mm.clone())?; if py::le(&t1563, &t1564)? { true } else { false } }) }).truthy() {
        v_rm = Py::Float(0.0);
    }
    v_q = f__cross3(v_R.clone(), v_M.clone())?;
    v_pt = { let mut out = vec![]; for t1565 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t1565;
out.push(py::add(&py::getitem(&v_o.clone(), &v_k.clone())?, &py::truediv(&py::getitem(&v_q.clone(), &v_k.clone())?, &py::mul(&v_Rm.clone(), &v_Rm.clone())?)?)?); } Py::list(out) };
    py::setitem(&v_out.clone(), &Py::str("result_type"), (if ({ let t1566 = v_rm.clone(); Py::Bool({ let t1567 = Py::Int(0i64); if py::eq(&t1566, &t1567) { true } else { false } }) }).truthy() { Py::str("single force") } else { Py::str("wrench") }))?;
    py::setitem(&v_out.clone(), &Py::str("axis_x"), py::getitem(&v_pt.clone(), &Py::Int(0i64))?)?;
    py::setitem(&v_out.clone(), &Py::str("axis_y"), py::getitem(&v_pt.clone(), &Py::Int(1i64))?)?;
    py::setitem(&v_out.clone(), &Py::str("axis_z"), py::getitem(&v_pt.clone(), &Py::Int(2i64))?)?;
    py::setitem(&v_out.clone(), &Py::str("pitch"), py::truediv(&v_rm.clone(), &py::mul(&v_Rm.clone(), &v_Rm.clone())?)?)?;
    py::setitem(&v_out.clone(), &Py::str("M_parallel"), py::truediv(&v_rm.clone(), &v_Rm.clone())?)?;
    v_planar = Py::Bool(true);
    for t1568 in py::iter(&(py::zip(&[v_Fs.clone(), v_Ps.clone()])?))? {
        let t1574 = (t1568).items()?;
        if t1574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t1574[0].clone();
        v_p = t1574[1].clone();
        if ({ let t1573 = { let t1569 = py::getitem(&v_f.clone(), &Py::Int(2i64))?; Py::Bool({ let t1570 = Py::Int(0i64); if py::ne(&t1569, &t1570) { true } else { false } }) }; if t1573.truthy() { t1573 } else { { let t1571 = py::getitem(&v_p.clone(), &Py::Int(2i64))?; Py::Bool({ let t1572 = Py::Int(0i64); if py::ne(&t1571, &t1572) { true } else { false } }) } } }).truthy() {
            v_planar = Py::Bool(false);
        }
    }
    for t1575 in py::iter(&(v_Cs.clone()))? {
        v_c = t1575;
        if ({ let t1580 = { let t1576 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t1577 = Py::Int(0i64); if py::ne(&t1576, &t1577) { true } else { false } }) }; if t1580.truthy() { t1580 } else { { let t1578 = py::getitem(&v_c.clone(), &Py::Int(1i64))?; Py::Bool({ let t1579 = Py::Int(0i64); if py::ne(&t1578, &t1579) { true } else { false } }) } } }).truthy() {
            v_planar = Py::Bool(false);
        }
    }
    if ({ let t1585 = { let t1581 = v_O.clone(); Py::Bool({ let t1582 = Py::None; if !py::is(&t1581, &t1582) { true } else { false } }) }; if !t1585.truthy() { t1585 } else { { let t1583 = py::getitem(&v_o.clone(), &Py::Int(2i64))?; Py::Bool({ let t1584 = Py::Int(0i64); if py::ne(&t1583, &t1584) { true } else { false } }) } } }).truthy() {
        v_planar = Py::Bool(false);
    }
    if ({ let t1588 = v_planar.clone(); if !t1588.truthy() { t1588 } else { { let t1586 = v_rm.clone(); Py::Bool({ let t1587 = Py::Int(0i64); if py::eq(&t1586, &t1587) { true } else { false } }) } } }).truthy() {
        if ({ let t1589 = py::getitem(&v_R.clone(), &Py::Int(1i64))?; Py::Bool({ let t1590 = Py::Int(0i64); if py::ne(&t1589, &t1590) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("x_intercept"), py::sub(&py::getitem(&v_pt.clone(), &Py::Int(0i64))?, &py::truediv(&py::mul(&py::getitem(&v_pt.clone(), &Py::Int(1i64))?, &py::getitem(&v_R.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_R.clone(), &Py::Int(1i64))?)?)?)?;
        }
        if ({ let t1591 = py::getitem(&v_R.clone(), &Py::Int(0i64))?; Py::Bool({ let t1592 = Py::Int(0i64); if py::ne(&t1591, &t1592) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("y_intercept"), py::sub(&py::getitem(&v_pt.clone(), &Py::Int(1i64))?, &py::truediv(&py::mul(&py::getitem(&v_pt.clone(), &Py::Int(0i64))?, &py::getitem(&v_R.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_R.clone(), &Py::Int(0i64))?)?)?)?;
        }
    }
    return Ok(f__out(v_out.clone())?);
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
    let t1628 = (Py::tuple(vec![py::sub(&v_k.clone(), &py::mul(&v_m.clone(), &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?)?, py::mul(&v_c.clone(), &v_omega.clone())?])).items()?;
    if t1628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t1628[0].clone();
    v_im = t1628[1].clone();
    if ({ let t1633 = { let t1629 = v_re_.clone(); Py::Bool({ let t1630 = Py::Int(0i64); if py::eq(&t1629, &t1630) { true } else { false } }) }; if !t1633.truthy() { t1633 } else { { let t1631 = v_im.clone(); Py::Bool({ let t1632 = Py::Int(0i64); if py::eq(&t1631, &t1632) { true } else { false } }) } } }).truthy() {
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
    let t1673 = (Py::tuple(vec![py::truediv(&v_d.clone(), &v_a.clone())?, py::truediv(&v_d.clone(), &v_c.clone())?, py::truediv(&py::add(&py::add(&py::sub(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&v_d.clone(), &v_d.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_c.clone())?)?])).items()?;
    if t1673.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K1 = t1673[0].clone();
    v_K2 = t1673[1].clone();
    v_K3 = t1673[2].clone();
    let t1674 = (Py::tuple(vec![py::truediv(&v_d.clone(), &v_b.clone())?, py::truediv(&py::sub(&py::sub(&py::sub(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&v_d.clone(), &v_d.clone())?)?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?)?])).items()?;
    if t1674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_K4 = t1674[0].clone();
    v_K5 = t1674[1].clone();
    let t1675 = (Py::tuple(vec![py::math1("cos", &v_theta2.clone())?, py::math1("sin", &v_theta2.clone())?])).items()?;
    if t1675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ct = t1675[0].clone();
    v_st = t1675[1].clone();
    v_A = py::add(&py::sub(&py::sub(&v_ct.clone(), &v_K1.clone())?, &py::mul(&v_K2.clone(), &v_ct.clone())?)?, &v_K3.clone())?;
    v_B = py::mul(&py::neg(&Py::Int(2i64))?, &v_st.clone())?;
    v_C = py::add(&py::sub(&v_K1.clone(), &py::mul(&py::add(&v_K2.clone(), &Py::Int(1i64))?, &v_ct.clone())?)?, &v_K3.clone())?;
    v_D = py::add(&py::add(&py::sub(&v_ct.clone(), &v_K1.clone())?, &py::mul(&v_K4.clone(), &v_ct.clone())?)?, &v_K5.clone())?;
    v_E = py::mul(&py::neg(&Py::Int(2i64))?, &v_st.clone())?;
    v_F = py::add(&py::add(&v_K1.clone(), &py::mul(&py::sub(&v_K4.clone(), &Py::Int(1i64))?, &v_ct.clone())?)?, &v_K5.clone())?;
    let t1676 = (Py::tuple(vec![py::sub(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_C.clone())?)?, py::sub(&py::mul(&v_E.clone(), &v_E.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_D.clone())?, &v_F.clone())?)?])).items()?;
    if t1676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_disc4 = t1676[0].clone();
    v_disc3 = t1676[1].clone();
    if ({ let t1681 = { let t1677 = v_disc4.clone(); Py::Bool({ let t1678 = Py::Int(0i64); if py::lt(&t1677, &t1678)? { true } else { false } }) }; if t1681.truthy() { t1681 } else { { let t1679 = v_disc3.clone(); Py::Bool({ let t1680 = Py::Int(0i64); if py::lt(&t1679, &t1680)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the linkage cannot be assembled at this input angle"))));
    }
    v_sg = (if ({ let t1682 = v_branch.clone(); Py::Bool({ let t1683 = Py::str("open"); if py::eq(&t1682, &t1683) { true } else { false } }) }).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) });
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
    v_mu = (if ({ let t1684 = v_mu.clone(); Py::Bool({ let t1685 = Py::Int(90i64); if py::gt(&t1684, &t1685)? { true } else { false } }) }).truthy() { py::sub(&Py::Int(180i64), &v_mu.clone())? } else { v_mu.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("theta3_deg"), py::call(&v_deg.clone(), &[v_t3.clone()])?), (Py::str("theta4_deg"), py::call(&v_deg.clone(), &[v_t4.clone()])?), (Py::str("transmission_angle_deg"), v_mu.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gear_train(mut v_drivers: Py, mut v_driven: Py, mut v_n_in: Py, mut v_T_in: Py, mut v_eta: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n_out: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sign: Py = Py::None;
    v_k = py::len(&v_drivers.clone())?;
    if ({ let t1725 = { let t1718 = v_k.clone(); Py::Bool({ let t1719 = py::len(&v_driven.clone())?; if py::ne(&t1718, &t1719) { true } else { false } }) }; if t1725.truthy() { t1725 } else { { let t1724 = { let t1720 = v_k.clone(); Py::Bool({ let t1721 = Py::Int(1i64); if py::lt(&t1720, &t1721)? { true } else { false } }) }; if t1724.truthy() { t1724 } else { { let t1722 = v_k.clone(); Py::Bool({ let t1723 = Py::Int(12i64); if py::gt(&t1722, &t1723)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one driving and one driven tooth count per mesh (1 to 12 meshes, equal-length lists)"))));
    }
    for t1726 in py::iter(&(py::add(&Py::list((v_drivers.clone()).items()?), &Py::list((v_driven.clone()).items()?))?))? {
        v_N = t1726;
        if (py::not(&{ let t1732 = py::math1("isfinite", &v_N.clone())?; if !t1732.truthy() { t1732 } else { { let t1731 = { let t1727 = v_N.clone(); Py::Bool({ let t1728 = Py::Int(1i64); if py::ge(&t1727, &t1728)? { true } else { false } }) }; if !t1731.truthy() { t1731 } else { { let t1729 = v_N.clone(); Py::Bool({ let t1730 = py::math1("floor", &v_N.clone())?; if py::eq(&t1729, &t1730) { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("tooth counts must be whole numbers of at least 1"))));
        }
    }
    let _ = f__fin(Py::str("the input speed"), Py::tuple(vec![v_n_in.clone()]))?;
    if ({ let t1735 = v_n_in.clone(); Py::Bool({ let t1736 = Py::Int(0i64); if py::eq(&t1735, &t1736) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the input speed must not be zero"))));
    }
    if (py::not(&{ let t1742 = py::math1("isfinite", &v_eta.clone())?; if !t1742.truthy() { t1742 } else { { let t1741 = { let t1737 = v_eta.clone(); Py::Bool({ let t1738 = Py::Int(0i64); if py::gt(&t1737, &t1738)? { true } else { false } }) }; if !t1741.truthy() { t1741 } else { { let t1739 = v_eta.clone(); Py::Bool({ let t1740 = Py::Int(1i64); if py::le(&t1739, &t1740)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the overall efficiency must lie in (0, 1]"))));
    }
    v_num = Py::Float(1.0);
    for t1743 in py::iter(&(v_drivers.clone()))? {
        v_N = t1743;
        v_num = py::mul(&v_num.clone(), &v_N.clone())?;
    }
    v_den = Py::Float(1.0);
    for t1744 in py::iter(&(v_driven.clone()))? {
        v_N = t1744;
        v_den = py::mul(&v_den.clone(), &v_N.clone())?;
    }
    v_sign = (if ({ let t1745 = py::modulo(&v_k.clone(), &Py::Int(2i64))?; Py::Bool({ let t1746 = Py::Int(0i64); if py::eq(&t1745, &t1746) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_e = py::truediv(&py::mul(&v_sign.clone(), &v_num.clone())?, &v_den.clone())?;
    v_n_out = py::mul(&v_e.clone(), &v_n_in.clone())?;
    v_out = Py::dict(vec![(Py::str("velocity_ratio"), py::truediv(&v_den.clone(), &v_num.clone())?), (Py::str("train_value"), v_e.clone()), (Py::str("n_out"), v_n_out.clone()), (Py::str("n_out_rpm"), py::mul(&v_n_out.clone(), &c__RPM()?)?), (Py::str("meshes"), v_k.clone())]);
    if ({ let t1747 = v_T_in.clone(); Py::Bool({ let t1748 = Py::None; if !py::is(&t1747, &t1748) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the input torque"), Py::tuple(vec![v_T_in.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("T_out"), py::mul(&py::truediv(&py::mul(&v_T_in.clone(), &v_den.clone())?, &v_num.clone())?, &v_eta.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gerber(mut v_sa: Py, mut v_sm: Py, mut v_Se: Py, mut v_Sut: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__fat_check(v_sa.clone(), v_sm.clone(), v_Se.clone(), v_Sut.clone())?;
    if ({ let t1844 = v_sm.clone(); Py::Bool({ let t1845 = Py::Int(0i64); if py::eq(&t1844, &t1845) { true } else { false } }) }).truthy() {
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
    if ({ let t1890 = v_Sy.clone(); Py::Bool({ let t1891 = Py::None; if !py::is(&t1890, &t1891) { true } else { false } }) }).truthy() {
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
    if ({ let t1930 = v_C.clone(); Py::Bool({ let t1931 = Py::Int(2i64); if py::le(&t1930, &t1931)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t1794 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1795 = v_nu.clone(); if py::lt(&t1794, &t1795)? { { let t1796 = Py::Float(0.5); if py::lt(&t1795, &t1796)? { true } else { false } } } else { false } }) })).truthy() {
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
    let t2116 = (f__rect_set(v_rects.clone())?).items()?;
    if t2116.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2116[0].clone();
    v__xc = t2116[1].clone();
    v__yc = t2116[2].clone();
    v_Ix = t2116[3].clone();
    v_Iy = t2116[4].clone();
    v__ = t2116[5].clone();
    let t2117 = (Py::tuple(vec![py::truediv(&v_Ix.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::truediv(&v_Iy.clone(), &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?])).items()?;
    if t2117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Zx = t2117[0].clone();
    v_Zy = t2117[1].clone();
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
    if (py::not(&{ let t4882 = Py::Int(0i64); Py::Bool({ let t4883 = v_TR.clone(); if py::lt(&t4882, &t4883)? { { let t4884 = Py::Int(1i64); if py::lt(&t4883, &t4884)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t2165 = v_lam.clone(); Py::Bool({ let t2166 = v_Cc.clone(); if py::lt(&t2165, &t2166)? { true } else { false } }) }).truthy() {
        let t2167 = (Py::tuple(vec![py::sub(&v_Sy.clone(), &py::truediv(&py::pow(&py::truediv(&py::mul(&v_Sy.clone(), &v_lam.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &Py::Int(2i64))?, &v_E.clone())?)?, Py::str("Johnson")])).items()?;
        if t2167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sc = t2167[0].clone();
        v_regime = t2167[1].clone();
    } else {
        let t2168 = (Py::tuple(vec![py::truediv(&py::mul(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_E.clone())?, &py::pow(&v_lam.clone(), &Py::Int(2i64))?)?, Py::str("Euler")])).items()?;
        if t2168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sc = t2168[0].clone();
        v_regime = t2168[1].clone();
    }
    v_out = Py::dict(vec![(Py::str("slenderness"), v_lam.clone()), (Py::str("transition_slenderness"), v_Cc.clone()), (Py::str("regime"), v_regime.clone()), (Py::str("sigma_cr_MPa"), py::truediv(&v_sc.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t2169 = v_A.clone(); Py::Bool({ let t2170 = Py::None; if !py::is(&t2169, &t2170) { true } else { false } }) }).truthy() {
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
    if ({ let t4572 = v_a.clone(); Py::Bool({ let t4573 = v_b.clone(); if py::ge(&t4572, &t4573)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner radius a must be smaller than the outer radius b"))));
    }
    if ({ let t4578 = { let t4574 = v_pi.clone(); Py::Bool({ let t4575 = Py::Int(0i64); if py::eq(&t4574, &t4575) { true } else { false } }) }; if !t4578.truthy() { t4578 } else { { let t4576 = v_po.clone(); Py::Bool({ let t4577 = Py::Int(0i64); if py::eq(&t4576, &t4577) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one pressure must be positive"))));
    }
    v_r = (if ({ let t4579 = v_r.clone(); Py::Bool({ let t4580 = Py::None; if py::is(&t4579, &t4580) { true } else { false } }) }).truthy() { v_a.clone() } else { v_r.clone() });
    if (py::not(&{ let t4581 = v_a.clone(); Py::Bool({ let t4582 = v_r.clone(); if py::le(&t4581, &t4582)? { { let t4583 = v_b.clone(); if py::le(&t4582, &t4583)? { true } else { false } } } else { false } }) })).truthy() {
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
    if (py::not(&{ let t2211 = py::math1("isfinite", &v_Kv.clone())?; if !t2211.truthy() { t2211 } else { { let t2209 = v_Kv.clone(); Py::Bool({ let t2210 = Py::Int(1i64); if py::ge(&t2209, &t2210)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the velocity factor Kv must be at least 1"))));
    }
    if ({ let t2212 = v_Y.clone(); Py::Bool({ let t2213 = Py::Int(1i64); if py::gt(&t2212, &t2213)? { true } else { false } }) }).truthy() {
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
    if ({ let t2251 = v_surface.clone(); Py::Bool({ let t2252 = c__MARIN_SURFACE()?; if !py::contains(&t2252, &t2251)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("surface must be ground, machined, cold-drawn, hot-rolled or as-forged"))));
    }
    if ({ let t2253 = v_load.clone(); Py::Bool({ let t2254 = c__MARIN_LOAD()?; if !py::contains(&t2254, &t2253)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("load must be bending, axial or torsion"))));
    }
    if (py::not(&{ let t2255 = Py::Float(0.5); Py::Bool({ let t2256 = v_R.clone(); if py::le(&t2255, &t2256)? { { let t2257 = Py::Int(1i64); if py::lt(&t2256, &t2257)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reliability must lie in [0.5, 1) (e.g. 0.99 or 99%)"))));
    }
    for t2258 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("kd"), v_kd.clone()]), Py::tuple(vec![Py::str("kf"), v_kf.clone()])])))? {
        let t2263 = (t2258).items()?;
        if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t2263[0].clone();
        v_v = t2263[1].clone();
        if (py::not(&{ let t2262 = py::math1("isfinite", &v_v.clone())?; if !t2262.truthy() { t2262 } else { { let t2259 = Py::Int(0i64); Py::Bool({ let t2260 = v_v.clone(); if py::lt(&t2259, &t2260)? { { let t2261 = Py::Float(1.5); if py::le(&t2260, &t2261)? { true } else { false } } } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must lie in (0, 1.5]".to_string()].concat()))));
        }
    }
    v_S = py::truediv(&v_Sut.clone(), &Py::Float(1000000.0))?;
    let t2264 = (py::getitem(&c__MARIN_SURFACE()?, &v_surface.clone())?).items()?;
    if t2264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2264[0].clone();
    v_b = t2264[1].clone();
    v_ka = py::mul(&v_a.clone(), &py::pow(&v_S.clone(), &v_b.clone())?)?;
    if ({ let t2265 = v_load.clone(); Py::Bool({ let t2266 = Py::str("axial"); if py::eq(&t2265, &t2266) { true } else { false } }) }).truthy() {
        v_kb = Py::Float(1.0);
    } else {
        if ({ let t2267 = v_d.clone(); Py::Bool({ let t2268 = Py::None; if py::is(&t2267, &t2268) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the diameter d is needed for bending and torsion (size factor)"))));
        }
        v_dm = py::mul(&v_d.clone(), &Py::Float(1000.0))?;
        if (py::not(&{ let t2269 = Py::Float(2.79); Py::Bool({ let t2270 = v_dm.clone(); if py::le(&t2269, &t2270)? { { let t2271 = Py::Int(254i64); if py::le(&t2270, &t2271)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the size factor covers 2.79 mm <= d <= 254 mm"))));
        }
        v_kb = (if ({ let t2272 = v_dm.clone(); Py::Bool({ let t2273 = Py::Int(51i64); if py::le(&t2272, &t2273)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(1.24), &py::pow(&v_dm.clone(), &py::neg(&Py::Float(0.107))?)?)? } else { py::mul(&Py::Float(1.51), &py::pow(&v_dm.clone(), &py::neg(&Py::Float(0.157))?)?)? });
    }
    v_kc = py::getitem(&c__MARIN_LOAD()?, &v_load.clone())?;
    v_ke = py::sub(&Py::Int(1i64), &py::mul(&Py::Float(0.08), &py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_R.clone()])?)?)?;
    v_Sep = (if ({ let t2274 = v_S.clone(); Py::Bool({ let t2275 = Py::Int(1400i64); if py::le(&t2274, &t2275)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(0.5), &v_S.clone())? } else { Py::Float(700.0) });
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
    if (py::not(&{ let t2308 = Py::Int(1i64); Py::Bool({ let t2309 = v_n.clone(); if py::le(&t2308, &t2309)? { { let t2310 = Py::Int(12i64); if py::le(&t2309, &t2310)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 to 12 masses"))));
    }
    v_need = (if ({ let t2311 = v_ends.clone(); Py::Bool({ let t2312 = Py::str("fixed-free"); if py::eq(&t2311, &t2312) { true } else { false } }) }).truthy() { v_n.clone() } else { py::add(&v_n.clone(), &Py::Int(1i64))? });
    if ({ let t2313 = py::len(&v_springs.clone())?; Py::Bool({ let t2314 = v_need.clone(); if py::ne(&t2313, &t2314) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["a ".to_string(), py::format(&v_ends.clone(), "")?, " chain of ".to_string(), py::format(&v_n.clone(), "")?, " masses needs ".to_string(), py::format(&v_need.clone(), "")?, " springs (wall spring first)".to_string()].concat()))));
    }
    for t2315 in py::iter(&(py::add(&Py::list((v_masses.clone()).items()?), &Py::list((v_springs.clone()).items()?))?))? {
        v_v = t2315;
        let _ = f__pos(Py::str("every mass and spring"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_K = f__chain_K(Py::list((v_springs.clone()).items()?), v_n.clone(), v_ends.clone())?;
    v_h = { let mut out = vec![]; for t2337 in py::iter(&(v_masses.clone()))? { let mut v_mi: Py; 
v_mi = t2337;
out.push(py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_mi.clone())?)?); } Py::list(out) };
    v_A = { let mut out = vec![]; for t2338 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2338;
out.push({ let mut out = vec![]; for t2339 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2339;
out.push(py::mul(&py::mul(&py::getitem(&py::getitem(&v_K.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_h.clone(), &v_i.clone())?)?, &py::getitem(&v_h.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    let t2373 = (f__jacobi_sym(v_A.clone())?).items()?;
    if t2373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2373[0].clone();
    v_vecs = t2373[1].clone();
    v_w = { let mut out = vec![]; for t2374 in py::iter(&(v_lam.clone()))? { let mut v_l_: Py; 
v_l_ = t2374;
out.push(py::math1("sqrt", &py::minmax(&[v_l_.clone(), Py::Float(0.0)], true)?)?); } Py::list(out) };
    v_mode1 = f__norm_mode({ let mut out = vec![]; for t2383 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2383;
out.push(py::mul(&py::getitem(&py::getitem(&v_vecs.clone(), &Py::Int(0i64))?, &v_i.clone())?, &py::getitem(&v_h.clone(), &v_i.clone())?)?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("f_Hz"), { let mut out = vec![]; for t2392 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t2392;
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
    if (py::not(&{ let t1390 = py::neg(&Py::Float(0.2))?; Py::Bool({ let t1391 = v_b.clone(); if py::lt(&t1390, &t1391)? { { let t1392 = Py::Int(0i64); if py::lt(&t1391, &t1392)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the fatigue strength exponent b must be negative (typically -0.05 to -0.15)"))));
    }
    v_smax = py::add(&v_sm.clone(), &v_sa.clone())?;
    if ({ let t1393 = v_method.clone(); Py::Bool({ let t1394 = Py::tuple(vec![Py::str("goodman"), Py::str("morrow")]); if py::contains(&t1394, &t1393)? { true } else { false } }) }).truthy() {
        if ({ let t1395 = v_sm.clone(); Py::Bool({ let t1396 = v_sf.clone(); if py::ge(&t1395, &t1396)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the mean stress must be below the strength used by the correction"))));
        }
        v_sar = py::truediv(&v_sa.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&v_sm.clone(), &v_sf.clone())?)?)?;
    } else {
        if ({ let t1397 = v_smax.clone(); Py::Bool({ let t1398 = Py::Int(0i64); if py::le(&t1397, &t1398)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("SWT and Walker are not defined for a maximum stress <= 0"))));
        }
        v_g = (if ({ let t1399 = v_method.clone(); Py::Bool({ let t1400 = Py::str("swt"); if py::eq(&t1399, &t1400) { true } else { false } }) }).truthy() { Py::Float(0.5) } else { v_gamma.clone() });
        if ({ let t1406 = { let t1401 = v_g.clone(); Py::Bool({ let t1402 = Py::None; if py::is(&t1401, &t1402) { true } else { false } }) }; if t1406.truthy() { t1406 } else { py::not(&{ let t1403 = Py::Int(0i64); Py::Bool({ let t1404 = v_g.clone(); if py::lt(&t1403, &t1404)? { { let t1405 = Py::Int(1i64); if py::le(&t1404, &t1405)? { true } else { false } } } else { false } }) }) } }).truthy() {
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
    v_D = py::fsum(&{ let mut out = vec![]; for t2426 in py::iter(&(py::zip(&[v_ranges.clone(), v_counts.clone()])?))? { let mut v_n: Py; let mut v_s: Py; 
let t2427 = (t2426).items()?;
if t2427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t2427[0].clone();
v_n = t2427[1].clone();
out.push(py::truediv(&v_n.clone(), &py::mul(&v_Nc.clone(), &py::pow(&py::truediv(&v_Sc.clone(), &v_s.clone())?, &v_m.clone())?)?)?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("damage"), v_D.clone()), (Py::str("repeats_to_failure"), py::truediv(&Py::Int(1i64), &v_D.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moment_3d(mut v_r: Py, mut v_F: Py, mut v_O: Py, mut v_axis: Py) -> R {
    let mut v_Fm: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Mm: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_un: Py = Py::None;
    v_r = f__vec3(Py::str("r"), v_r.clone())?;
    v_F = f__vec3(Py::str("F"), v_F.clone())?;
    v_o = (if ({ let t2503 = v_O.clone(); Py::Bool({ let t2504 = Py::None; if py::is(&t2503, &t2504) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]) } else { f__vec3(Py::str("O"), v_O.clone())? });
    v_Fm = py::math1("sqrt", &f__dot3(v_F.clone(), v_F.clone())?)?;
    if ({ let t2505 = v_Fm.clone(); Py::Bool({ let t2506 = Py::Int(0i64); if py::eq(&t2505, &t2506) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the force must not be zero"))));
    }
    v_a = Py::list(vec![py::sub(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_o.clone(), &Py::Int(2i64))?)?]);
    v_sc = py::mul(&v_Fm.clone(), &py::math1("sqrt", &f__dot3(v_a.clone(), v_a.clone())?)?)?;
    v_M = f__cross3(v_a.clone(), v_F.clone())?;
    v_M = Py::list(vec![f__snap(py::getitem(&v_M.clone(), &Py::Int(0i64))?, v_sc.clone())?, f__snap(py::getitem(&v_M.clone(), &Py::Int(1i64))?, v_sc.clone())?, f__snap(py::getitem(&v_M.clone(), &Py::Int(2i64))?, v_sc.clone())?]);
    v_Mm = py::math1("sqrt", &f__dot3(v_M.clone(), v_M.clone())?)?;
    v_out = Py::dict(vec![(Py::str("Mx"), py::getitem(&v_M.clone(), &Py::Int(0i64))?), (Py::str("My"), py::getitem(&v_M.clone(), &Py::Int(1i64))?), (Py::str("Mz"), py::getitem(&v_M.clone(), &Py::Int(2i64))?), (Py::str("M"), v_Mm.clone()), (Py::str("d"), py::truediv(&v_Mm.clone(), &v_Fm.clone())?)]);
    if ({ let t2509 = v_axis.clone(); Py::Bool({ let t2510 = Py::None; if !py::is(&t2509, &t2510) { true } else { false } }) }).truthy() {
        v_u = f__vec3(Py::str("axis"), v_axis.clone())?;
        v_un = py::math1("sqrt", &f__dot3(v_u.clone(), v_u.clone())?)?;
        if ({ let t2511 = v_un.clone(); Py::Bool({ let t2512 = Py::Int(0i64); if py::eq(&t2511, &t2512) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the axis direction must not be zero"))));
        }
        py::setitem(&v_out.clone(), &Py::str("M_axis"), f__snap(py::truediv(&f__dot3(v_M.clone(), v_u.clone())?, &v_un.clone())?, v_sc.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
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
    if ({ let t2462 = v_F.clone(); Py::Bool({ let t2463 = Py::Int(0i64); if py::eq(&t2462, &t2463) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the force must not be zero"))));
    }
    let t2464 = (Py::tuple(vec![py::sub(&v_x.clone(), &v_x0.clone())?, py::sub(&v_y.clone(), &v_y0.clone())?])).items()?;
    if t2464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t2464[0].clone();
    v_ry = t2464[1].clone();
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
    if (py::not(&{ let t2985 = Py::Int(0i64); Py::Bool({ let t2986 = v_a.clone(); if py::lt(&t2985, &t2986)? { { let t2987 = v_L.clone(); if py::lt(&t2986, &t2987)? { true } else { false } } } else { false } }) })).truthy() {
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
fn f_pin_shear_bearing(mut v_P: Py, mut v_d: Py, mut v_t: Py, mut v_n_bolts: Py, mut v_planes: Py, mut v_tau_allow: Py, mut v_sb_allow: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sb: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("the load P and the diameter d"), Py::tuple(vec![v_P.clone(), v_d.clone()]))?;
    if (py::not(&{ let t2558 = { let t2551 = v_n_bolts.clone(); Py::Bool({ let t2552 = Py::Int(1i64); if py::ge(&t2551, &t2552)? { true } else { false } }) }; if !t2558.truthy() { t2558 } else { { let t2557 = { let t2553 = v_n_bolts.clone(); Py::Bool({ let t2554 = Py::Int(200i64); if py::le(&t2553, &t2554)? { true } else { false } }) }; if !t2557.truthy() { t2557 } else { { let t2555 = v_n_bolts.clone(); Py::Bool({ let t2556 = py::math1("floor", &v_n_bolts.clone())?; if py::eq(&t2555, &t2556) { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of bolts must be a whole number from 1 to 200"))));
    }
    if (py::not(&{ let t2566 = { let t2559 = v_planes.clone(); Py::Bool({ let t2560 = Py::Int(1i64); if py::ge(&t2559, &t2560)? { true } else { false } }) }; if !t2566.truthy() { t2566 } else { { let t2565 = { let t2561 = v_planes.clone(); Py::Bool({ let t2562 = Py::Int(4i64); if py::le(&t2561, &t2562)? { true } else { false } }) }; if !t2565.truthy() { t2565 } else { { let t2563 = v_planes.clone(); Py::Bool({ let t2564 = py::math1("floor", &v_planes.clone())?; if py::eq(&t2563, &t2564) { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of shear planes per bolt must be 1, 2, 3 or 4 (1 = single shear, 2 = double shear)"))));
    }
    v_A = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?, &v_d.clone())?, &Py::Int(4i64))?;
    v_tau = py::truediv(&v_P.clone(), &py::mul(&py::mul(&v_n_bolts.clone(), &v_planes.clone())?, &v_A.clone())?)?;
    v_out = Py::dict(vec![(Py::str("A_mm2"), py::mul(&v_A.clone(), &Py::Float(1000000.0))?), (Py::str("tau"), v_tau.clone()), (Py::str("tau_MPa"), py::truediv(&v_tau.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t2567 = v_tau_allow.clone(); Py::Bool({ let t2568 = Py::None; if !py::is(&t2567, &t2568) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the allowable shear stress"), Py::tuple(vec![v_tau_allow.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("n_shear"), py::truediv(&v_tau_allow.clone(), &v_tau.clone())?)?;
    }
    if ({ let t2569 = v_t.clone(); Py::Bool({ let t2570 = Py::None; if !py::is(&t2569, &t2570) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the plate thickness t"), Py::tuple(vec![v_t.clone()]))?;
        v_sb = py::truediv(&v_P.clone(), &py::mul(&py::mul(&v_n_bolts.clone(), &v_d.clone())?, &v_t.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("sigma_b"), v_sb.clone())?;
        py::setitem(&v_out.clone(), &Py::str("sigma_b_MPa"), py::truediv(&v_sb.clone(), &Py::Float(1000000.0))?)?;
        if ({ let t2571 = v_sb_allow.clone(); Py::Bool({ let t2572 = Py::None; if !py::is(&t2571, &t2572) { true } else { false } }) }).truthy() {
            let _ = f__pos(Py::str("the allowable bearing stress"), Py::tuple(vec![v_sb_allow.clone()]))?;
            py::setitem(&v_out.clone(), &Py::str("n_bearing"), py::truediv(&v_sb_allow.clone(), &v_sb.clone())?)?;
        }
    } else {
        if ({ let t2573 = v_sb_allow.clone(); Py::Bool({ let t2574 = Py::None; if !py::is(&t2573, &t2574) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the plate thickness t is needed for the bearing check"))));
        }
    }
    return Ok(f__out(v_out.clone())?);
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
    let t2610 = (Py::tuple(vec![py::math1("cos", &py::mul(&Py::Int(2i64), &v_theta.clone())?)?, py::math1("sin", &py::mul(&Py::Int(2i64), &v_theta.clone())?)?])).items()?;
    if t2610.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c2 = t2610[0].clone();
    v_s2 = t2610[1].clone();
    let t2611 = (Py::tuple(vec![py::truediv(&py::add(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?])).items()?;
    if t2611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2611[0].clone();
    v_d = t2611[1].clone();
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
    if (py::not(&{ let t2654 = { let t2647 = v_Ns.clone(); Py::Bool({ let t2648 = Py::Int(0i64); if py::gt(&t2647, &t2648)? { true } else { false } }) }; if !t2654.truthy() { t2654 } else { { let t2653 = { let t2649 = v_Nr.clone(); Py::Bool({ let t2650 = v_Ns.clone(); if py::gt(&t2649, &t2650)? { true } else { false } }) }; if !t2653.truthy() { t2653 } else { { let t2651 = py::modulo(&py::sub(&v_Nr.clone(), &v_Ns.clone())?, &Py::Int(2i64))?; Py::Bool({ let t2652 = Py::Int(0i64); if py::eq(&t2651, &t2652) { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need Nr > Ns > 0 with Nr - Ns even (the planets have (Nr - Ns)/2 teeth)"))));
    }
    v_given = { let mut out = vec![]; for t2655 in py::iter(&(Py::tuple(vec![v_ws.clone(), v_wr.clone(), v_wc.clone()])))? { let mut v_v: Py; 
v_v = t2655;
out.push({ let t2656 = v_v.clone(); Py::Bool({ let t2657 = Py::None; if !py::is(&t2656, &t2657) { true } else { false } }) }); } Py::list(out) };
    if ({ let t2658 = py::sum(&v_given.clone(), None)?; Py::Bool({ let t2659 = Py::Int(2i64); if py::ne(&t2658, &t2659) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly two of the three speeds ws, wr, wc"))));
    }
    for t2660 in py::iter(&(Py::tuple(vec![v_ws.clone(), v_wr.clone(), v_wc.clone()])))? {
        v_v = t2660;
        if ({ let t2661 = v_v.clone(); Py::Bool({ let t2662 = Py::None; if !py::is(&t2661, &t2662) { true } else { false } }) }).truthy() {
            let _ = f__fin(Py::str("every speed"), Py::tuple(vec![v_v.clone()]))?;
        }
    }
    v_k = py::truediv(&v_Nr.clone(), &v_Ns.clone())?;
    if ({ let t2665 = v_ws.clone(); Py::Bool({ let t2666 = Py::None; if py::is(&t2665, &t2666) { true } else { false } }) }).truthy() {
        v_ws = py::sub(&v_wc.clone(), &py::mul(&v_k.clone(), &py::sub(&v_wr.clone(), &v_wc.clone())?)?)?;
    } else {
        if ({ let t2667 = v_wr.clone(); Py::Bool({ let t2668 = Py::None; if py::is(&t2667, &t2668) { true } else { false } }) }).truthy() {
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
    v_pts = { let mut out = vec![]; for t2703 in py::iter(&(v_vertices.clone()))? { let mut v_r: Py; 
v_r = t2703;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) };
    if ({ let t2710 = Py::Bool(({ let mut out = vec![]; for t2704 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2704;
out.push({ let t2705 = py::len(&v_p.clone())?; Py::Bool({ let t2706 = Py::Int(2i64); if py::ne(&t2705, &t2706) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2710.truthy() { t2710 } else { py::not(&{ let t2707 = Py::Int(3i64); Py::Bool({ let t2708 = py::len(&v_pts.clone())?; if py::le(&t2707, &t2708)? { { let t2709 = Py::Int(60i64); if py::le(&t2708, &t2709)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 3 to 60 vertices as rows 'x y' (mm)"))));
    }
    v_n = py::len(&v_pts.clone())?;
    if ({ let t2711 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t2712 = v_n.clone(); if py::ne(&t2711, &t2712) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("vertices must be distinct"))));
    }
    for t2713 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2713;
        for t2714 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t2714;
            if ({ let t2722 = { let t2715 = v_j.clone(); Py::Bool({ let t2716 = py::add(&v_i.clone(), &Py::Int(1i64))?; if py::eq(&t2715, &t2716) { true } else { false } }) }; if t2722.truthy() { t2722 } else { { let t2721 = { let t2717 = v_i.clone(); Py::Bool({ let t2718 = Py::Int(0i64); if py::eq(&t2717, &t2718) { true } else { false } }) }; if !t2721.truthy() { t2721 } else { { let t2719 = v_j.clone(); Py::Bool({ let t2720 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t2719, &t2720) { true } else { false } }) } } } } }).truthy() {
                continue;
            }
            if (f__segments_cross(py::getitem(&v_pts.clone(), &v_i.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_pts.clone(), &v_j.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the polygon must be simple (its edges must not cross)"))));
            }
        }
    }
    let t2755 = Py::Float(0.0);
    v_a = t2755.clone();
    v_cx = t2755.clone();
    v_cy = t2755.clone();
    v_ixx = t2755.clone();
    v_iyy = t2755.clone();
    v_ixy = t2755.clone();
    v_terms = Py::list(vec![]);
    for t2756 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2756;
        let t2757 = (py::getitem(&v_pts.clone(), &v_i.clone())?).items()?;
        if t2757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x0 = t2757[0].clone();
        v_y0 = t2757[1].clone();
        let t2758 = (py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?).items()?;
        if t2758.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x1 = t2758[0].clone();
        v_y1 = t2758[1].clone();
        v_c = py::sub(&py::mul(&v_x0.clone(), &v_y1.clone())?, &py::mul(&v_x1.clone(), &v_y0.clone())?)?;
        let _ = py::method(&v_terms.clone(), "append", &[Py::tuple(vec![v_c.clone(), v_x0.clone(), v_y0.clone(), v_x1.clone(), v_y1.clone()])])?;
    }
    v_a = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2759 in py::iter(&(v_terms.clone()))? { let mut v__: Py; let mut v_c: Py; 
let t2760 = (t2759).items()?;
if t2760.len() < 1 { return Err(PyErr::Value("unpack".into())); }
v_c = t2760[0].clone();
v__ = Py::list(t2760[1..t2760.len() - 0].to_vec());
out.push(v_c.clone()); } Py::list(out) })?, &Py::Int(2i64))?;
    if ({ let t2761 = py::abs(&v_a.clone())?; Py::Bool({ let t2762 = py::mul(&Py::Float(1e-12), &py::pow(&py::minmax(&[{ let mut out = vec![]; for t2763 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2763;
for t2764 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2764;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?, &Py::Int(2i64))?)?; if py::le(&t2761, &t2762)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polygon has no area"))));
    }
    v_cx = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2765 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2766 = (t2765).items()?;
if t2766.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2766[0].clone();
v_x0 = t2766[1].clone();
v_y0 = t2766[2].clone();
v_x1 = t2766[3].clone();
v_y1 = t2766[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&v_x0.clone(), &v_x1.clone())?)?); } Py::list(out) })?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?;
    v_cy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2767 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2768 = (t2767).items()?;
if t2768.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2768[0].clone();
v_x0 = t2768[1].clone();
v_y0 = t2768[2].clone();
v_x1 = t2768[3].clone();
v_y1 = t2768[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&v_y0.clone(), &v_y1.clone())?)?); } Py::list(out) })?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?;
    v_ixx = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2769 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2770 = (t2769).items()?;
if t2770.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2770[0].clone();
v_x0 = t2770[1].clone();
v_y0 = t2770[2].clone();
v_x1 = t2770[3].clone();
v_y1 = t2770[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::add(&py::mul(&v_y0.clone(), &v_y0.clone())?, &py::mul(&v_y0.clone(), &v_y1.clone())?)?, &py::mul(&v_y1.clone(), &v_y1.clone())?)?)?); } Py::list(out) })?, &Py::Int(12i64))?;
    v_iyy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2771 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2772 = (t2771).items()?;
if t2772.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2772[0].clone();
v_x0 = t2772[1].clone();
v_y0 = t2772[2].clone();
v_x1 = t2772[3].clone();
v_y1 = t2772[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::add(&py::mul(&v_x0.clone(), &v_x0.clone())?, &py::mul(&v_x0.clone(), &v_x1.clone())?)?, &py::mul(&v_x1.clone(), &v_x1.clone())?)?)?); } Py::list(out) })?, &Py::Int(12i64))?;
    v_ixy = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2773 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_x0: Py; let mut v_x1: Py; let mut v_y0: Py; let mut v_y1: Py; 
let t2774 = (t2773).items()?;
if t2774.len() != 5 { return Err(PyErr::Value("unpack".into())); }
v_c = t2774[0].clone();
v_x0 = t2774[1].clone();
v_y0 = t2774[2].clone();
v_x1 = t2774[3].clone();
v_y1 = t2774[4].clone();
out.push(py::mul(&v_c.clone(), &py::add(&py::add(&py::add(&py::mul(&v_x0.clone(), &v_y1.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_x0.clone())?, &v_y0.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_x1.clone())?, &v_y1.clone())?)?, &py::mul(&v_x1.clone(), &v_y0.clone())?)?)?); } Py::list(out) })?, &Py::Int(24i64))?;
    v_sgn = (if ({ let t2775 = v_a.clone(); Py::Bool({ let t2776 = Py::Int(0i64); if py::gt(&t2775, &t2776)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_A = py::abs(&v_a.clone())?;
    v_Ix = py::sub(&py::mul(&v_sgn.clone(), &v_ixx.clone())?, &py::mul(&py::mul(&v_A.clone(), &v_cy.clone())?, &v_cy.clone())?)?;
    v_Iy = py::sub(&py::mul(&v_sgn.clone(), &v_iyy.clone())?, &py::mul(&py::mul(&v_A.clone(), &v_cx.clone())?, &v_cx.clone())?)?;
    v_size = py::minmax(&[{ let mut out = vec![]; for t2777 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2777;
for t2778 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2778;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?;
    let t2781 = (Py::tuple(vec![f__snap(v_cx.clone(), v_size.clone())?, f__snap(v_cy.clone(), v_size.clone())?])).items()?;
    if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2781[0].clone();
    v_cy = t2781[1].clone();
    v_Ixy = f__snap(py::sub(&py::mul(&v_sgn.clone(), &v_ixy.clone())?, &py::mul(&py::mul(&v_A.clone(), &v_cx.clone())?, &v_cy.clone())?)?, py::minmax(&[v_Ix.clone(), v_Iy.clone()], true)?)?;
    let t2787 = (f__principal(v_Ix.clone(), v_Iy.clone(), v_Ixy.clone())?).items()?;
    if t2787.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I1 = t2787[0].clone();
    v_I2 = t2787[1].clone();
    v_phi = t2787[2].clone();
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
    if ({ let t2830 = v_alpha.clone(); Py::Bool({ let t2831 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?; if py::ge(&t2830, &t2831)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the thread half-angle must be below 45 degrees"))));
    }
    if ({ let t2836 = { let t2832 = v_fc.clone(); Py::Bool({ let t2833 = Py::Int(0i64); if py::gt(&t2832, &t2833)? { true } else { false } }) }; if !t2836.truthy() { t2836 } else { { let t2834 = v_dc.clone(); Py::Bool({ let t2835 = Py::None; if py::is(&t2834, &t2835) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a collar friction coefficient needs the collar mean diameter dc"))));
    }
    v_sec = py::truediv(&Py::Int(1i64), &py::math1("cos", &v_alpha.clone())?)?;
    v_collar = py::truediv(&py::mul(&py::mul(&v_F.clone(), &v_fc.clone())?, &{ let t2837 = v_dc.clone(); if t2837.truthy() { t2837 } else { Py::Float(0.0) } })?, &Py::Int(2i64))?;
    v_TR = py::add(&py::truediv(&py::mul(&py::truediv(&py::mul(&v_F.clone(), &v_dm.clone())?, &Py::Int(2i64))?, &py::add(&v_l.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_f.clone())?, &v_dm.clone())?, &v_sec.clone())?)?)?, &py::sub(&py::mul(&Py::Float(std::f64::consts::PI), &v_dm.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_l.clone())?, &v_sec.clone())?)?)?, &v_collar.clone())?;
    if ({ let t2838 = py::sub(&py::mul(&Py::Float(std::f64::consts::PI), &v_dm.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_l.clone())?, &v_sec.clone())?)?; Py::Bool({ let t2839 = Py::Int(0i64); if py::le(&t2838, &t2839)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the lead is too steep for this friction: the screw cannot raise the load"))));
    }
    v_TL_thread = py::truediv(&py::mul(&py::truediv(&py::mul(&v_F.clone(), &v_dm.clone())?, &Py::Int(2i64))?, &py::sub(&py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_f.clone())?, &v_dm.clone())?, &v_sec.clone())?, &v_l.clone())?)?, &py::add(&py::mul(&Py::Float(std::f64::consts::PI), &v_dm.clone())?, &py::mul(&py::mul(&v_f.clone(), &v_l.clone())?, &v_sec.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_raise"), v_TR.clone()), (Py::str("T_lower"), py::add(&v_TL_thread.clone(), &v_collar.clone())?), (Py::str("efficiency"), py::truediv(&py::mul(&v_F.clone(), &v_l.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_TR.clone())?)?), (Py::str("self_locking"), (if ({ let t2848 = v_TL_thread.clone(); Py::Bool({ let t2849 = Py::Int(0i64); if py::gt(&t2848, &t2849)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]))?);
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
    let t2877 = (Py::tuple(vec![py::truediv(&py::add(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, py::hypot(&[py::truediv(&py::sub(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, v_txy.clone()])?])).items()?;
    if t2877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2877[0].clone();
    v_R = t2877[1].clone();
    let t2880 = (Py::tuple(vec![f__snap(py::add(&v_c.clone(), &v_R.clone())?, v_sc.clone())?, f__snap(py::sub(&v_c.clone(), &v_R.clone())?, v_sc.clone())?])).items()?;
    if t2880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t2880[0].clone();
    v_s2 = t2880[1].clone();
    v_th = (if ({ let t2881 = v_R.clone(); Py::Bool({ let t2882 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t2881, &t2882)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&py::mul(&Py::Int(2i64), &v_txy.clone())?, &py::sub(&v_sx.clone(), &v_sy.clone())?)?)?)? });
    if ({ let t2883 = v_th.clone(); Py::Bool({ let t2884 = py::neg(&Py::Int(90i64))?; if py::le(&t2883, &t2884)? { true } else { false } }) }).truthy() {
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
    let t5040 = ({ let mut out = vec![]; for t5037 in py::iter(&(f__jacobi3(v_S.clone())?))? { let mut v_v: Py; 
v_v = t5037;
out.push(f__snap(v_v.clone(), v_sc.clone())?); } Py::list(out) }).items()?;
    if t5040.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s3 = t5040[0].clone();
    v_s2 = t5040[1].clone();
    v_s1 = t5040[2].clone();
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
    for t3080 in py::iter(&(f__rainflow(Py::list((v_history.clone()).items()?))?))? {
        let t3089 = (t3080).items()?;
        if t3089.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_rng_ = t3089[0].clone();
        v__m = t3089[1].clone();
        v_c = t3089[2].clone();
        if ({ let t3081 = v_rng_.clone(); Py::Bool({ let t3082 = Py::Int(0i64); if py::gt(&t3081, &t3082)? { true } else { false } }) }).truthy() {
            v_k = f__r(v_rng_.clone())?;
            py::setitem(&v_agg.clone(), &v_k.clone(), py::add(&py::method(&v_agg.clone(), "get", &[v_k.clone(), Py::Float(0.0)])?, &v_c.clone())?)?;
        }
    }
    v_keys = py::sorted(&v_agg.clone(), (Py::Bool(true)).truthy())?;
    return Ok(f__out(Py::dict(vec![(Py::str("ranges"), v_keys.clone()), (Py::str("counts"), { let mut out = vec![]; for t3092 in py::iter(&(v_keys.clone()))? { let mut v_k: Py; 
v_k = t3092;
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
    v_cyc = { let mut out = vec![]; for t3145 in py::iter(&(f__rainflow(Py::list((v_history.clone()).items()?))?))? { let mut v__m: Py; let mut v_c: Py; let mut v_r_: Py; 
let t3146 = (t3145).items()?;
if t3146.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_r_ = t3146[0].clone();
v__m = t3146[1].clone();
v_c = t3146[2].clone();
if ({ let t3147 = v_r_.clone(); Py::Bool({ let t3148 = Py::Int(0i64); if py::gt(&t3147, &t3148)? { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_r_.clone(), v_c.clone()])); }} Py::list(out) };
    v_D = py::fsum(&{ let mut out = vec![]; for t3149 in py::iter(&(v_cyc.clone()))? { let mut v_c: Py; let mut v_r_: Py; 
let t3150 = (t3149).items()?;
if t3150.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r_ = t3150[0].clone();
v_c = t3150[1].clone();
out.push(py::truediv(&py::mul(&v_c.clone(), &py::pow(&py::truediv(&v_r_.clone(), &v_Sc.clone())?, &v_m.clone())?)?, &v_Nc.clone())?); } Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("damage"), v_D.clone()), (Py::str("repeats_to_failure"), py::truediv(&Py::Int(1i64), &v_D.clone())?), (Py::str("cycles"), py::sum(&{ let mut out = vec![]; for t3159 in py::iter(&(v_cyc.clone()))? { let mut v__r_: Py; let mut v_c: Py; 
let t3160 = (t3159).items()?;
if t3160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__r_ = t3160[0].clone();
v_c = t3160[1].clone();
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
    v_y = { let mut out = vec![]; for t3207 in py::range_iter(&[(py::len(&v_masses.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3207;
out.push(py::mul(&c_G0()?, &py::fsum(&{ let mut out = vec![]; for t3208 in py::range_iter(&[(py::len(&v_masses.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3208;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_masses.clone(), &v_j.clone())?)?); } Py::list(out) })?)?); } Py::list(out) };
    v_w = py::math1("sqrt", &py::truediv(&py::mul(&c_G0()?, &py::fsum(&{ let mut out = vec![]; for t3209 in py::iter(&(py::zip(&[v_masses.clone(), v_y.clone()])?))? { let mut v_m: Py; let mut v_yi: Py; 
let t3210 = (t3209).items()?;
if t3210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t3210[0].clone();
v_yi = t3210[1].clone();
out.push(py::mul(&v_m.clone(), &v_yi.clone())?); } Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t3211 in py::iter(&(py::zip(&[v_masses.clone(), v_y.clone()])?))? { let mut v_m: Py; let mut v_yi: Py; 
let t3212 = (t3211).items()?;
if t3212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_m = t3212[0].clone();
v_yi = t3212[1].clone();
out.push(py::mul(&py::mul(&v_m.clone(), &v_yi.clone())?, &v_yi.clone())?); } Py::list(out) })?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_cr"), v_w.clone()), (Py::str("N_cr_rpm"), py::mul(&v_w.clone(), &c__RPM()?)?), (Py::str("deflections_mm"), { let mut out = vec![]; for t3221 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3221;
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
    let t3251 = (Py::tuple(vec![py::truediv(&py::mul(&v_b.clone(), &py::pow(&v_h.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?, py::truediv(&py::mul(&v_h.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &Py::Int(12i64))?])).items()?;
    if t3251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Ix = t3251[0].clone();
    v_Iy = t3251[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Zx"), py::truediv(&v_Ix.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?), (Py::str("Zy"), py::truediv(&v_Iy.clone(), &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?), (Py::str("J_polar"), py::add(&v_Ix.clone(), &v_Iy.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_restrained_bar_thermal(mut v_E: Py, mut v_alpha: Py, mut v_dT: Py, mut v_L: Py, mut v_A: Py, mut v_gap: Py) -> R {
    let mut v_closing: Py = Py::None;
    let mut v_eps_m: Py = Py::None;
    let mut v_eps_t: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("E and alpha"), Py::tuple(vec![v_E.clone(), v_alpha.clone()]))?;
    let _ = f__fin(Py::str("the temperature change dT"), Py::tuple(vec![v_dT.clone()]))?;
    let _ = f__nonneg(Py::str("the gap"), Py::tuple(vec![v_gap.clone()]))?;
    if ({ let t4512 = v_L.clone(); Py::Bool({ let t4513 = Py::None; if !py::is(&t4512, &t4513) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the length L"), Py::tuple(vec![v_L.clone()]))?;
    }
    if ({ let t4514 = v_A.clone(); Py::Bool({ let t4515 = Py::None; if !py::is(&t4514, &t4515) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the area A"), Py::tuple(vec![v_A.clone()]))?;
    }
    if ({ let t4520 = { let t4516 = v_gap.clone(); Py::Bool({ let t4517 = Py::Int(0i64); if py::gt(&t4516, &t4517)? { true } else { false } }) }; if !t4520.truthy() { t4520 } else { { let t4518 = v_L.clone(); Py::Bool({ let t4519 = Py::None; if py::is(&t4518, &t4519) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the bar length L is needed when a gap is given"))));
    }
    v_eps_t = py::mul(&v_alpha.clone(), &v_dT.clone())?;
    if ({ let t4521 = v_gap.clone(); Py::Bool({ let t4522 = Py::Int(0i64); if py::gt(&t4521, &t4522)? { true } else { false } }) }).truthy() {
        v_closing = py::sub(&py::mul(&v_eps_t.clone(), &v_L.clone())?, &v_gap.clone())?;
        if ({ let t4523 = v_closing.clone(); Py::Bool({ let t4524 = Py::Int(0i64); if py::gt(&t4523, &t4524)? { true } else { false } }) }).truthy() {
            v_eps_m = py::truediv(&py::neg(&v_closing.clone())?, &v_L.clone())?;
        } else {
            v_eps_m = Py::Float(0.0);
        }
    } else {
        v_eps_m = py::neg(&v_eps_t.clone())?;
    }
    v_s = py::mul(&v_E.clone(), &v_eps_m.clone())?;
    v_out = Py::dict(vec![(Py::str("eps_thermal"), v_eps_t.clone()), (Py::str("eps_mech"), v_eps_m.clone()), (Py::str("sigma"), v_s.clone()), (Py::str("sigma_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t4525 = v_L.clone(); Py::Bool({ let t4526 = Py::None; if !py::is(&t4525, &t4526) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("free_expansion_mm"), py::mul(&py::mul(&v_eps_t.clone(), &v_L.clone())?, &Py::Float(1000.0))?)?;
    }
    if ({ let t4527 = v_A.clone(); Py::Bool({ let t4528 = Py::None; if !py::is(&t4527, &t4528) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("force"), py::mul(&v_s.clone(), &v_A.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("force_kN"), py::truediv(&py::mul(&v_s.clone(), &v_A.clone())?, &Py::Float(1000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_resultant_forces(mut v_F: Py, mut v_theta: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_Rx: Py = Py::None;
    let mut v_Ry: Py = Py::None;
    let mut v_scale: Py = Py::None;
    if ({ let t3284 = py::len(&v_F.clone())?; Py::Bool({ let t3285 = py::len(&v_theta.clone())?; if py::ne(&t3284, &t3285) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F and theta must have the same number of entries"))));
    }
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("each force and angle")); v.extend((v_F.clone()).items()?); v.extend((v_theta.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_fin() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_fin() got an unexpected keyword argument".into())) } f__fin(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    if (Py::Bool(({ let mut out = vec![]; for t3288 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t3288;
out.push({ let t3289 = v_f.clone(); Py::Bool({ let t3290 = Py::Int(0i64); if py::lt(&t3289, &t3290)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("force magnitudes must be zero or positive (give the direction by the angle)"))));
    }
    v_scale = py::sum(&v_F.clone(), None)?;
    if ({ let t3291 = v_scale.clone(); Py::Bool({ let t3292 = Py::Int(0i64); if py::le(&t3291, &t3292)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one force must be non-zero"))));
    }
    v_Rx = f__snap(py::fsum(&{ let mut out = vec![]; for t3295 in py::iter(&(py::zip(&[v_F.clone(), v_theta.clone()])?))? { let mut v_f: Py; let mut v_t: Py; 
let t3296 = (t3295).items()?;
if t3296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t3296[0].clone();
v_t = t3296[1].clone();
out.push(py::mul(&v_f.clone(), &f__cosd(v_t.clone())?)?); } Py::list(out) })?, v_scale.clone())?;
    v_Ry = f__snap(py::fsum(&{ let mut out = vec![]; for t3297 in py::iter(&(py::zip(&[v_F.clone(), v_theta.clone()])?))? { let mut v_f: Py; let mut v_t: Py; 
let t3298 = (t3297).items()?;
if t3298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t3298[0].clone();
v_t = t3298[1].clone();
out.push(py::mul(&v_f.clone(), &f__sind(v_t.clone())?)?); } Py::list(out) })?, v_scale.clone())?;
    v_R = py::hypot(&[v_Rx.clone(), v_Ry.clone()])?;
    if ({ let t3299 = v_R.clone(); Py::Bool({ let t3300 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t3299, &t3300)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t4396 = Py::Int(0i64); Py::Bool({ let t4397 = v_nu.clone(); if py::le(&t4396, &t4397)? { { let t4398 = Py::Float(0.5); if py::lt(&t4397, &t4398)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie in [0, 0.5)"))));
    }
    if ({ let t4399 = v_layout.clone(); Py::Bool({ let t4400 = Py::str("rectangular"); if py::eq(&t4399, &t4400) { true } else { false } }) }).truthy() {
        let t4401 = (Py::tuple(vec![v_ea.clone(), v_ec.clone(), py::sub(&py::sub(&py::mul(&Py::Int(2i64), &v_eb.clone())?, &v_ea.clone())?, &v_ec.clone())?])).items()?;
        if t4401.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t4401[0].clone();
        v_ey = t4401[1].clone();
        v_gxy = t4401[2].clone();
    } else {
        let t4402 = (Py::tuple(vec![v_ea.clone(), py::truediv(&py::sub(&py::add(&py::mul(&Py::Int(2i64), &v_eb.clone())?, &py::mul(&Py::Int(2i64), &v_ec.clone())?)?, &v_ea.clone())?, &Py::Int(3i64))?, py::truediv(&py::mul(&Py::Int(2i64), &py::sub(&v_eb.clone(), &v_ec.clone())?)?, &py::math1("sqrt", &Py::Int(3i64))?)?])).items()?;
        if t4402.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t4402[0].clone();
        v_ey = t4402[1].clone();
        v_gxy = t4402[2].clone();
    }
    v_sc = { let t4403 = py::minmax(&[py::abs(&v_ea.clone())?, py::abs(&v_eb.clone())?, py::abs(&v_ec.clone())?], true)?; if t4403.truthy() { t4403 } else { Py::Float(1.0) } };
    let t4404 = (Py::tuple(vec![py::truediv(&py::add(&v_ex.clone(), &v_ey.clone())?, &Py::Int(2i64))?, py::hypot(&[py::truediv(&py::sub(&v_ex.clone(), &v_ey.clone())?, &Py::Int(2i64))?, py::truediv(&v_gxy.clone(), &Py::Int(2i64))?])?])).items()?;
    if t4404.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t4404[0].clone();
    v_R = t4404[1].clone();
    let t4407 = (Py::tuple(vec![f__snap(py::add(&v_c.clone(), &v_R.clone())?, v_sc.clone())?, f__snap(py::sub(&v_c.clone(), &v_R.clone())?, v_sc.clone())?])).items()?;
    if t4407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e1 = t4407[0].clone();
    v_e2 = t4407[1].clone();
    v_th = (if ({ let t4408 = v_R.clone(); Py::Bool({ let t4409 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t4408, &t4409)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&Py::Float(0.5), &py::math1("degrees", &py::atan2(&v_gxy.clone(), &py::sub(&v_ex.clone(), &v_ey.clone())?)?)?)? });
    if ({ let t4410 = v_th.clone(); Py::Bool({ let t4411 = py::neg(&Py::Int(90i64))?; if py::le(&t4410, &t4411)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t3343 = Py::Int(0i64); Py::Bool({ let t3344 = v_nu.clone(); if py::le(&t3343, &t3344)? { { let t3345 = Py::Float(0.5); if py::lt(&t3344, &t3345)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie in [0, 0.5)"))));
    }
    if ({ let t3346 = v_ri.clone(); Py::Bool({ let t3347 = v_ro.clone(); if py::ge(&t3346, &t3347)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the bore radius must be smaller than the outer radius"))));
    }
    v_k = py::mul(&py::mul(&py::truediv(&py::add(&Py::Int(3i64), &v_nu.clone())?, &Py::Int(8i64))?, &v_rho.clone())?, &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?;
    if ({ let t3348 = v_ri.clone(); Py::Bool({ let t3349 = Py::Int(0i64); if py::eq(&t3348, &t3349) { true } else { false } }) }).truthy() {
        let t3350 = py::mul(&py::mul(&v_k.clone(), &v_ro.clone())?, &v_ro.clone())?;
        v_st_max = t3350.clone();
        v_sr_max = t3350.clone();
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
    if ({ let t3441 = py::abs(&py::sub(&v_z.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t3442 = Py::Float(1e-09); if py::lt(&t3441, &t3442)? { true } else { false } }) }).truthy() {
        v_e = py::math1("exp", &py::mul(&py::neg(&v_wn.clone())?, &v_t.clone())?)?;
        v_x = py::mul(&py::add(&v_x0.clone(), &py::mul(&py::add(&v_v0.clone(), &py::mul(&v_wn.clone(), &v_x0.clone())?)?, &v_t.clone())?)?, &v_e.clone())?;
        v_v = py::mul(&py::sub(&v_v0.clone(), &py::mul(&py::mul(&v_wn.clone(), &py::add(&v_v0.clone(), &py::mul(&v_wn.clone(), &v_x0.clone())?)?)?, &v_t.clone())?)?, &v_e.clone())?;
    } else {
        if ({ let t3443 = v_z.clone(); Py::Bool({ let t3444 = Py::Int(1i64); if py::lt(&t3443, &t3444)? { true } else { false } }) }).truthy() {
            v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?;
            v_B = py::truediv(&py::add(&v_v0.clone(), &py::mul(&py::mul(&v_z.clone(), &v_wn.clone())?, &v_x0.clone())?)?, &v_wd.clone())?;
            v_e = py::math1("exp", &py::mul(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &v_t.clone())?)?;
            let t3445 = (Py::tuple(vec![py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?])).items()?;
            if t3445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cs = t3445[0].clone();
            v_sn = t3445[1].clone();
            v_x = py::mul(&v_e.clone(), &py::add(&py::mul(&v_x0.clone(), &v_cs.clone())?, &py::mul(&v_B.clone(), &v_sn.clone())?)?)?;
            v_v = py::mul(&v_e.clone(), &py::add(&py::mul(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &py::add(&py::mul(&v_x0.clone(), &v_cs.clone())?, &py::mul(&v_B.clone(), &v_sn.clone())?)?)?, &py::mul(&v_wd.clone(), &py::add(&py::mul(&py::neg(&v_x0.clone())?, &v_sn.clone())?, &py::mul(&v_B.clone(), &v_cs.clone())?)?)?)?)?;
        } else {
            v_q = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&py::mul(&v_z.clone(), &v_z.clone())?, &Py::Int(1i64))?)?)?;
            let t3446 = (Py::tuple(vec![py::add(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &v_q.clone())?, py::sub(&py::mul(&py::neg(&v_z.clone())?, &v_wn.clone())?, &v_q.clone())?])).items()?;
            if t3446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s1 = t3446[0].clone();
            v_s2 = t3446[1].clone();
            let t3447 = (Py::tuple(vec![py::truediv(&py::sub(&v_v0.clone(), &py::mul(&v_s2.clone(), &v_x0.clone())?)?, &py::sub(&v_s1.clone(), &v_s2.clone())?)?, py::truediv(&py::sub(&py::mul(&v_s1.clone(), &v_x0.clone())?, &v_v0.clone())?, &py::sub(&v_s1.clone(), &v_s2.clone())?)?])).items()?;
            if t3447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_A1 = t3447[0].clone();
            v_A2 = t3447[1].clone();
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
    if ({ let t3527 = v_P.clone(); Py::Bool({ let t3528 = v_Pcr.clone(); if py::ge(&t3527, &t3528)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t3621 = v_n.clone(); Py::Bool({ let t3622 = Py::Int(1i64); if py::ge(&t3621, &t3622)? { true } else { false } }) })).truthy() {
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
    let t3675 = (f__de_stresses(v_d.clone(), v_Ma.clone(), v_Mm.clone(), v_Ta.clone(), v_Tm.clone(), v_Kf.clone(), v_Kfs.clone())?).items()?;
    if t3675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sa = t3675[0].clone();
    v_sm = t3675[1].clone();
    if ({ let t3676 = v_sm.clone(); Py::Bool({ let t3677 = v_Sut.clone(); if py::ge(&t3676, &t3677)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mean von Mises stress reaches Sut"))));
    }
    v_out = Py::dict(vec![(Py::str("n_fatigue"), py::truediv(&Py::Int(1i64), &py::add(&py::truediv(&v_sa.clone(), &v_Se.clone())?, &py::truediv(&v_sm.clone(), &v_Sut.clone())?)?)?), (Py::str("sigma_a_vm_MPa"), py::truediv(&v_sa.clone(), &Py::Float(1000000.0))?), (Py::str("sigma_m_vm_MPa"), py::truediv(&v_sm.clone(), &Py::Float(1000000.0))?)]);
    if ({ let t3678 = v_Sy.clone(); Py::Bool({ let t3679 = Py::None; if !py::is(&t3678, &t3679) { true } else { false } }) }).truthy() {
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
    if ({ let t4219 = v_d_i.clone(); Py::Bool({ let t4220 = v_d.clone(); if py::ge(&t4219, &t4220)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t3855 = v_ri.clone(); Py::Bool({ let t3856 = v_R.clone(); if py::lt(&t3855, &t3856)? { { let t3857 = v_ro.clone(); if py::lt(&t3856, &t3857)? { true } else { false } } } else { false } }) })).truthy() {
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
    if (py::not(&{ let t3566 = Py::Int(0i64); Py::Bool({ let t3567 = v_a.clone(); if py::lt(&t3566, &t3567)? { { let t3568 = v_L.clone(); if py::lt(&t3567, &t3568)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t4035 = v_l.clone(); Py::Bool({ let t4036 = v_r.clone(); if py::le(&t4035, &t4036)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the connecting rod must be longer than the crank"))));
    }
    let t4037 = (Py::tuple(vec![py::math1("sin", &v_theta.clone())?, py::math1("cos", &v_theta.clone())?])).items()?;
    if t4037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4037[0].clone();
    v_c = t4037[1].clone();
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
    if ({ let t4082 = py::not(&{ let t4077 = Py::Int(0i64); Py::Bool({ let t4078 = v_f.clone(); if py::lt(&t4077, &t4078)? { { let t4079 = Py::Int(1i64); if py::lt(&t4078, &t4079)? { true } else { false } } } else { false } }) }); if t4082.truthy() { t4082 } else { { let t4080 = py::mul(&v_f.clone(), &v_Sut.clone())?; Py::Bool({ let t4081 = v_Se.clone(); if py::le(&t4080, &t4081)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 0 < f < 1 and f*Sut > Se"))));
    }
    if (py::not(&{ let t4083 = v_Se.clone(); Py::Bool({ let t4084 = v_sa.clone(); if py::lt(&t4083, &t4084)? { { let t4085 = py::mul(&v_f.clone(), &v_Sut.clone())?; if py::lt(&t4084, &t4085)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t4128 = py::not(&{ let t4123 = Py::Int(0i64); Py::Bool({ let t4124 = v_f.clone(); if py::lt(&t4123, &t4124)? { { let t4125 = Py::Int(1i64); if py::lt(&t4124, &t4125)? { true } else { false } } } else { false } }) }); if t4128.truthy() { t4128 } else { { let t4126 = py::mul(&v_f.clone(), &v_Sut.clone())?; Py::Bool({ let t4127 = v_Se.clone(); if py::le(&t4126, &t4127)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 0 < f < 1 and f*Sut > Se"))));
    }
    if (py::not(&{ let t4129 = Py::Float(1000.0); Py::Bool({ let t4130 = v_N.clone(); if py::le(&t4129, &t4130)? { { let t4131 = Py::Float(1000000.0); if py::le(&t4130, &t4131)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t4257 = { let t4253 = v_N1.clone(); Py::Bool({ let t4254 = Py::Int(1i64); if py::lt(&t4253, &t4254)? { true } else { false } }) }; if t4257.truthy() { t4257 } else { { let t4255 = v_N2.clone(); Py::Bool({ let t4256 = Py::Int(1i64); if py::lt(&t4255, &t4256)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("tooth numbers must be positive"))));
    }
    let _ = f__pos(Py::str("the module"), Py::tuple(vec![v_m.clone()]))?;
    if (py::not(&{ let t4263 = f__deg(Py::Int(10i64))?; Py::Bool({ let t4264 = v_phi.clone(); if py::le(&t4263, &t4264)? { { let t4265 = f__deg(Py::Int(30i64))?; if py::le(&t4264, &t4265)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pressure angle must lie between 10 and 30 degrees"))));
    }
    let t4266 = (Py::tuple(vec![py::truediv(&py::mul(&v_N1.clone(), &v_m.clone())?, &Py::Int(2i64))?, py::truediv(&py::mul(&v_N2.clone(), &v_m.clone())?, &Py::Int(2i64))?])).items()?;
    if t4266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t4266[0].clone();
    v_r2 = t4266[1].clone();
    let t4267 = (Py::tuple(vec![py::add(&v_r1.clone(), &v_m.clone())?, py::add(&v_r2.clone(), &v_m.clone())?])).items()?;
    if t4267.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ra1 = t4267[0].clone();
    v_ra2 = t4267[1].clone();
    let t4268 = (Py::tuple(vec![py::mul(&v_r1.clone(), &py::math1("cos", &v_phi.clone())?)?, py::mul(&v_r2.clone(), &py::math1("cos", &v_phi.clone())?)?])).items()?;
    if t4268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rb1 = t4268[0].clone();
    v_rb2 = t4268[1].clone();
    v_C = py::add(&v_r1.clone(), &v_r2.clone())?;
    let t4269 = (Py::tuple(vec![py::math1("sqrt", &py::sub(&py::pow(&v_ra1.clone(), &Py::Int(2i64))?, &py::pow(&v_rb1.clone(), &Py::Int(2i64))?)?)?, py::math1("sqrt", &py::sub(&py::pow(&v_ra2.clone(), &Py::Int(2i64))?, &py::pow(&v_rb2.clone(), &Py::Int(2i64))?)?)?])).items()?;
    if t4269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u1 = t4269[0].clone();
    v_u2 = t4269[1].clone();
    if ({ let t4274 = { let t4270 = v_u1.clone(); Py::Bool({ let t4271 = py::mul(&v_C.clone(), &py::math1("sin", &v_phi.clone())?)?; if py::gt(&t4270, &t4271)? { true } else { false } }) }; if t4274.truthy() { t4274 } else { { let t4272 = v_u2.clone(); Py::Bool({ let t4273 = py::mul(&v_C.clone(), &py::math1("sin", &v_phi.clone())?)?; if py::gt(&t4272, &t4273)? { true } else { false } }) } } }).truthy() {
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
    if (py::not(&{ let t3895 = Py::Int(0i64); Py::Bool({ let t3896 = v_a.clone(); if py::lt(&t3895, &t3896)? { { let t3897 = v_L.clone(); if py::lt(&t3896, &t3897)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the load position a must lie strictly between the supports (0 < a < L)"))));
    }
    v_b = py::sub(&v_L.clone(), &v_a.clone())?;
    v_EI = py::mul(&v_E.clone(), &v_I.clone())?;
    let t3898 = (Py::tuple(vec![py::truediv(&py::mul(&v_P.clone(), &v_b.clone())?, &v_L.clone())?, py::truediv(&py::mul(&v_P.clone(), &v_a.clone())?, &v_L.clone())?])).items()?;
    if t3898.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_RA = t3898[0].clone();
    v_RB = t3898[1].clone();
    v_Mmax = py::truediv(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_b.clone())?, &v_L.clone())?;
    v_d_load = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_P.clone(), &v_a.clone())?, &v_a.clone())?, &v_b.clone())?, &v_b.clone())?, &py::mul(&py::mul(&Py::Int(3i64), &v_EI.clone())?, &v_L.clone())?)?;
    if ({ let t3899 = v_a.clone(); Py::Bool({ let t3900 = v_b.clone(); if py::ge(&t3899, &t3900)? { true } else { false } }) }).truthy() {
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
    if ({ let t3938 = py::len(&v_x.clone())?; Py::Bool({ let t3939 = py::len(&v_P.clone())?; if py::ne(&t3938, &t3939) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and P must have the same number of entries"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3940 in py::iter(&(v_x.clone()))? { let mut v_xi: Py; 
v_xi = t3940;
out.push(py::not(&{ let t3941 = Py::Int(0i64); Py::Bool({ let t3942 = v_xi.clone(); if py::le(&t3941, &t3942)? { { let t3943 = v_L.clone(); if py::le(&t3942, &t3943)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every load position must lie on the span (0 <= x <= L)"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3944 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3944;
out.push(py::not(&{ let t3947 = py::math1("isfinite", &v_p.clone())?; if !t3947.truthy() { t3947 } else { { let t3945 = v_p.clone(); Py::Bool({ let t3946 = Py::Int(0i64); if py::gt(&t3945, &t3946)? { true } else { false } }) } } })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every load must be positive (downward)"))));
    }
    v_tot = py::fsum(&v_P.clone())?;
    v_RB = py::truediv(&py::fsum(&{ let mut out = vec![]; for t3948 in py::iter(&(py::zip(&[v_P.clone(), v_x.clone()])?))? { let mut v_p: Py; let mut v_xi: Py; 
let t3949 = (t3948).items()?;
if t3949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t3949[0].clone();
v_xi = t3949[1].clone();
out.push(py::mul(&v_p.clone(), &v_xi.clone())?); } Py::list(out) })?, &v_L.clone())?;
    v_RA = py::sub(&v_tot.clone(), &v_RB.clone())?;
    v_pts = py::sorted(&py::set(&v_x.clone())?, (Py::Bool(false)).truthy())?;
    let t3950 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best_M = t3950[0].clone();
    v_best_x = t3950[1].clone();
    for t3951 in py::iter(&(v_pts.clone()))? {
        v_xv = t3951;
        v_M = py::sub(&py::mul(&v_RA.clone(), &v_xv.clone())?, &py::fsum(&{ let mut out = vec![]; for t3952 in py::iter(&(py::zip(&[v_P.clone(), v_x.clone()])?))? { let mut v_p: Py; let mut v_xi: Py; 
let t3953 = (t3952).items()?;
if t3953.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t3953[0].clone();
v_xi = t3953[1].clone();
if ({ let t3954 = v_xi.clone(); Py::Bool({ let t3955 = v_xv.clone(); if py::lt(&t3954, &t3955)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_p.clone(), &py::sub(&v_xv.clone(), &v_xi.clone())?)?); }} Py::list(out) })?)?;
        if ({ let t3956 = v_M.clone(); Py::Bool({ let t3957 = py::mul(&v_best_M.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t3956, &t3957)? { true } else { false } }) }).truthy() {
            let t3958 = (Py::tuple(vec![v_M.clone(), v_xv.clone()])).items()?;
            if t3958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_best_M = t3958[0].clone();
            v_best_x = t3958[1].clone();
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
    if ({ let t4351 = py::len(&v_lengths.clone())?; Py::Bool({ let t4352 = py::len(&v_diameters.clone())?; if py::ne(&t4351, &t4352) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("lengths and diameters must have the same number of entries"))));
    }
    for t4353 in py::iter(&(py::add(&Py::list((v_lengths.clone()).items()?), &Py::list((v_diameters.clone()).items()?))?))? {
        v_v = t4353;
        let _ = f__pos(Py::str("every length and diameter"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t4354 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t4354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tw = t4354[0].clone();
    v_tau = t4354[1].clone();
    for t4355 in py::iter(&(py::zip(&[v_lengths.clone(), v_diameters.clone()])?))? {
        let t4356 = (t4355).items()?;
        if t4356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Li = t4356[0].clone();
        v_di = t4356[1].clone();
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
    if ({ let t4449 = v_tf.clone(); Py::Bool({ let t4450 = v_h.clone(); if py::ge(&t4449, &t4450)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the flange thickness must be less than the overall depth"))));
    }
    if ({ let t4451 = v_tw.clone(); Py::Bool({ let t4452 = v_bf.clone(); if py::gt(&t4451, &t4452)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stem must not be wider than the flange"))));
    }
    v_hs = py::sub(&v_h.clone(), &v_tf.clone())?;
    let t4467 = (f__rect_set(Py::list(vec![Py::tuple(vec![Py::Int(1i64), v_bf.clone(), v_tf.clone(), Py::Float(0.0), py::sub(&v_h.clone(), &py::truediv(&v_tf.clone(), &Py::Int(2i64))?)?]), Py::tuple(vec![Py::Int(1i64), v_tw.clone(), v_hs.clone(), Py::Float(0.0), py::truediv(&v_hs.clone(), &Py::Int(2i64))?])]))?).items()?;
    if t4467.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4467[0].clone();
    v__xc = t4467[1].clone();
    v_yb = t4467[2].clone();
    v_Ix = t4467[3].clone();
    v_Iy = t4467[4].clone();
    v__ = t4467[5].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("y_bar"), v_yb.clone()), (Py::str("Ix"), v_Ix.clone()), (Py::str("Iy"), v_Iy.clone()), (Py::str("Z_top"), py::truediv(&v_Ix.clone(), &py::sub(&v_h.clone(), &v_yb.clone())?)?), (Py::str("Z_bottom"), py::truediv(&v_Ix.clone(), &v_yb.clone())?), (Py::str("rx"), py::math1("sqrt", &py::truediv(&v_Ix.clone(), &v_A.clone())?)?), (Py::str("ry"), py::math1("sqrt", &py::truediv(&v_Iy.clone(), &v_A.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thin_vessel(mut v_p: Py, mut v_r: Py, mut v_t: Py, mut v_shape: Py) -> R {
    let mut v_sh: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let _ = f__pos(Py::str("p, r and t"), Py::tuple(vec![v_p.clone(), v_r.clone(), v_t.clone()]))?;
    if ({ let t4623 = py::truediv(&v_r.clone(), &v_t.clone())?; Py::Bool({ let t4624 = Py::Int(10i64); if py::lt(&t4623, &t4624)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("r/t is below 10: the wall is not thin (use the thick-walled cylinder)"))));
    }
    if ({ let t4625 = v_shape.clone(); Py::Bool({ let t4626 = Py::str("cylinder"); if py::eq(&t4625, &t4626) { true } else { false } }) }).truthy() {
        let t4627 = (Py::tuple(vec![py::truediv(&py::mul(&v_p.clone(), &v_r.clone())?, &v_t.clone())?, py::truediv(&py::mul(&v_p.clone(), &v_r.clone())?, &py::mul(&Py::Int(2i64), &v_t.clone())?)?])).items()?;
        if t4627.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sh = t4627[0].clone();
        v_sl = t4627[1].clone();
    } else {
        let t4628 = py::truediv(&py::mul(&v_p.clone(), &v_r.clone())?, &py::mul(&Py::Int(2i64), &v_t.clone())?)?;
        v_sh = t4628.clone();
        v_sl = t4628.clone();
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
    if ({ let t4927 = v_den.clone(); Py::Bool({ let t4928 = Py::Int(0i64); if py::eq(&t4927, &t4928) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("undamped resonance: the transmissibility is unbounded"))));
    }
    v_TR = py::math1("sqrt", &py::truediv(&v_num.clone(), &v_den.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("TR"), v_TR.clone()), (Py::str("isolation_percent"), (if ({ let t4937 = v_TR.clone(); Py::Bool({ let t4938 = Py::Int(1i64); if py::lt(&t4937, &t4938)? { true } else { false } }) }).truthy() { py::mul(&py::sub(&Py::Int(1i64), &v_TR.clone())?, &Py::Int(100i64))? } else { Py::Float(0.0) }))]))?);
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
    let t4711 = (f__truss_read(v_nodes.clone(), v_members.clone(), v_pin.clone(), v_roller.clone(), v_loads.clone())?).items()?;
    if t4711.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t4711[0].clone();
    v_mem = t4711[1].clone();
    v_pin = t4711[2].clone();
    v_rol = t4711[3].clone();
    v_ld = t4711[4].clone();
    let t4712 = (Py::tuple(vec![py::len(&v_pts.clone())?, py::len(&v_mem.clone())?])).items()?;
    if t4712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4712[0].clone();
    v_m = t4712[1].clone();
    v_A = { let mut out = vec![]; for t4713 in py::range_iter(&[(py::mul(&Py::Int(2i64), &v_n.clone())?).i()?]) { let mut v__: Py; 
v__ = t4713;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_m.clone(), &Py::Int(3i64))?)?); } Py::list(out) };
    v_b = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::mul(&Py::Int(2i64), &v_n.clone())?)?;
    for t4714 in py::iter(&(py::enumerate(&v_mem.clone(), 0)?))? {
        let t4725 = (t4714).items()?;
        if t4725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4725[0].clone();
        let t4726 = (t4725[1].clone()).items()?;
        if t4726.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4726[0].clone();
        v_j = t4726[1].clone();
        let t4715 = (Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_j.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_j.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(1i64))?)?])).items()?;
        if t4715.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dx = t4715[0].clone();
        v_dy = t4715[1].clone();
        v_L = py::hypot(&[v_dx.clone(), v_dy.clone()])?;
        let t4716 = (Py::tuple(vec![py::truediv(&v_dx.clone(), &v_L.clone())?, py::truediv(&v_dy.clone(), &v_L.clone())?])).items()?;
        if t4716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cx = t4716[0].clone();
        v_cy = t4716[1].clone();
        let t4717 = py::getitem(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_i.clone())?)?; let t4718 = v_k.clone();
        py::setitem(&t4717, &t4718, py::iadd(&py::getitem(&t4717, &t4718)?, &v_cx.clone())?)?;
        let t4719 = py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_i.clone())?, &Py::Int(1i64))?)?; let t4720 = v_k.clone();
        py::setitem(&t4719, &t4720, py::iadd(&py::getitem(&t4719, &t4720)?, &v_cy.clone())?)?;
        let t4721 = py::getitem(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_j.clone())?)?; let t4722 = v_k.clone();
        py::setitem(&t4721, &t4722, py::sub(&py::getitem(&t4721, &t4722)?, &v_cx.clone())?)?;
        let t4723 = py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_j.clone())?, &Py::Int(1i64))?)?; let t4724 = v_k.clone();
        py::setitem(&t4723, &t4724, py::sub(&py::getitem(&t4723, &t4724)?, &v_cy.clone())?)?;
    }
    py::setitem(&py::getitem(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_pin.clone())?)?, &v_m.clone(), Py::Float(1.0))?;
    py::setitem(&py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_pin.clone())?, &Py::Int(1i64))?)?, &py::add(&v_m.clone(), &Py::Int(1i64))?, Py::Float(1.0))?;
    py::setitem(&py::getitem(&v_A.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_rol.clone())?, &Py::Int(1i64))?)?, &py::add(&v_m.clone(), &Py::Int(2i64))?, Py::Float(1.0))?;
    for t4727 in py::iter(&(v_ld.clone()))? {
        let t4732 = (t4727).items()?;
        if t4732.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_node = t4732[0].clone();
        v_fx = t4732[1].clone();
        v_fy = t4732[2].clone();
        let t4728 = v_b.clone(); let t4729 = py::mul(&Py::Int(2i64), &v_node.clone())?;
        py::setitem(&t4728, &t4729, py::sub(&py::getitem(&t4728, &t4729)?, &v_fx.clone())?)?;
        let t4730 = v_b.clone(); let t4731 = py::add(&py::mul(&Py::Int(2i64), &v_node.clone())?, &Py::Int(1i64))?;
        py::setitem(&t4730, &t4731, py::sub(&py::getitem(&t4730, &t4731)?, &v_fy.clone())?)?;
    }
    v_x = f__gauss_solve(v_A.clone(), v_b.clone())?;
    v_scale = py::minmax(&[py::add(&py::add(&{ let mut out = vec![]; for t4748 in py::iter(&(v_ld.clone()))? { let mut v__n: Py; let mut v_f: Py; let mut v_g: Py; 
let t4749 = (t4748).items()?;
if t4749.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__n = t4749[0].clone();
v_f = t4749[1].clone();
v_g = t4749[2].clone();
out.push(py::abs(&v_f.clone())?); } Py::list(out) }, &{ let mut out = vec![]; for t4750 in py::iter(&(v_ld.clone()))? { let mut v__n: Py; let mut v_f: Py; let mut v_g: Py; 
let t4751 = (t4750).items()?;
if t4751.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__n = t4751[0].clone();
v_f = t4751[1].clone();
v_g = t4751[2].clone();
out.push(py::abs(&v_g.clone())?); } Py::list(out) })?, &Py::list(vec![Py::Float(1e-300)]))?], true)?;
    v_x = { let mut out = vec![]; for t4752 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4752;
out.push(f__snap(v_v.clone(), v_scale.clone())?); } Py::list(out) };
    v_forces = py::slice(&v_x.clone(), None, Some((v_m.clone()).i()?), None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("forces_kN"), v_forces.clone()), (Py::str("pin_Rx_kN"), py::getitem(&v_x.clone(), &v_m.clone())?), (Py::str("pin_Ry_kN"), py::getitem(&v_x.clone(), &py::add(&v_m.clone(), &Py::Int(1i64))?)?), (Py::str("roller_Ry_kN"), py::getitem(&v_x.clone(), &py::add(&v_m.clone(), &Py::Int(2i64))?)?), (Py::str("max_tension_kN"), py::minmax(&[{ let t4766 = { let mut out = vec![]; for t4763 in py::iter(&(v_forces.clone()))? { let mut v_f: Py; 
v_f = t4763;
if ({ let t4764 = v_f.clone(); Py::Bool({ let t4765 = Py::Int(0i64); if py::gt(&t4764, &t4765)? { true } else { false } }) }).truthy() { out.push(v_f.clone()); }} Py::list(out) }; if t4766.truthy() { t4766 } else { Py::list(vec![Py::Float(0.0)]) } }], true)?), (Py::str("max_compression_kN"), py::neg(&py::minmax(&[{ let t4770 = { let mut out = vec![]; for t4767 in py::iter(&(v_forces.clone()))? { let mut v_f: Py; 
v_f = t4767;
if ({ let t4768 = v_f.clone(); Py::Bool({ let t4769 = Py::Int(0i64); if py::lt(&t4768, &t4769)? { true } else { false } }) }).truthy() { out.push(v_f.clone()); }} Py::list(out) }; if t4770.truthy() { t4770 } else { Py::list(vec![Py::Float(0.0)]) } }], false)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tube_section(mut v_D: Py, mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I: Py = Py::None;
    let _ = f__pos(Py::str("the outer diameter D"), Py::tuple(vec![v_D.clone()]))?;
    let _ = f__nonneg(Py::str("the inner diameter d"), Py::tuple(vec![v_d.clone()]))?;
    if ({ let t1974 = v_d.clone(); Py::Bool({ let t1975 = v_D.clone(); if py::ge(&t1974, &t1975)? { true } else { false } }) }).truthy() {
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
    for t4800 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("alpha"), v_alpha.clone()]), Py::tuple(vec![Py::str("beta"), v_beta.clone()])])))? {
        let t4804 = (t4800).items()?;
        if t4804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t4804[0].clone();
        v_v = t4804[1].clone();
        if (py::not(&{ let t4801 = Py::Int(0i64); Py::Bool({ let t4802 = v_v.clone(); if py::lt(&t4801, &t4802)? { { let t4803 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t4802, &t4803)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t4842 = v_L.clone(); Py::Bool({ let t4843 = Py::None; if !py::is(&t4842, &t4843) { true } else { false } }) }).truthy() {
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
    if ({ let t3393 = v_mu.clone(); Py::Bool({ let t3394 = v_M.clone(); if py::ge(&t3393, &t3394)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the unbalanced mass must be part of the total mass M"))));
    }
    v_den = py::hypot(&[py::sub(&v_k.clone(), &py::mul(&v_M.clone(), &py::pow(&v_omega.clone(), &Py::Int(2i64))?)?)?, py::mul(&v_c.clone(), &v_omega.clone())?])?;
    if ({ let t3395 = v_den.clone(); Py::Bool({ let t3396 = Py::Int(0i64); if py::eq(&t3395, &t3396) { true } else { false } }) }).truthy() {
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
    if ({ let t4968 = v_C.clone(); Py::Bool({ let t4969 = Py::Int(2i64); if py::le(&t4968, &t4969)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the spring index D/d must exceed 2"))));
    }
    v_KW = py::add(&py::truediv(&py::sub(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(1i64))?, &py::sub(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(4i64))?)?, &py::truediv(&Py::Float(0.615), &v_C.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("index"), v_C.clone()), (Py::str("K_wahl"), v_KW.clone()), (Py::str("K_bergstrasser"), py::truediv(&py::add(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(2i64))?, &py::sub(&py::mul(&Py::Int(4i64), &v_C.clone())?, &Py::Int(3i64))?)?), (Py::str("K_s"), py::add(&Py::Int(1i64), &py::truediv(&Py::Float(0.5), &v_C.clone())?)?)])?;
    if ({ let t4970 = v_F.clone(); Py::Bool({ let t4971 = Py::None; if !py::is(&t4970, &t4971) { true } else { false } }) }).truthy() {
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
    let t5049 = (Py::tuple(vec![py::mul(&py::getitem(&v_r.clone(), &Py::str("von_mises_MPa"))?, &Py::Float(1000000.0))?, py::mul(&py::sub(&py::getitem(&v_r.clone(), &Py::str("s1_MPa"))?, &py::getitem(&v_r.clone(), &Py::str("s3_MPa"))?)?, &Py::Float(1000000.0))?])).items()?;
    if t5049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vm = t5049[0].clone();
    v_tr = t5049[1].clone();
    if ({ let t5050 = v_vm.clone(); Py::Bool({ let t5051 = Py::Int(0i64); if py::le(&t5050, &t5051)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stress state is purely hydrostatic: neither criterion predicts yielding"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("von_mises_MPa"), py::truediv(&v_vm.clone(), &Py::Float(1000000.0))?), (Py::str("tresca_MPa"), py::truediv(&v_tr.clone(), &Py::Float(1000000.0))?), (Py::str("n_von_mises"), py::truediv(&v_Sy.clone(), &v_vm.clone())?), (Py::str("n_tresca"), py::truediv(&v_Sy.clone(), &v_tr.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1053(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1055: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1054 = v_inputs.clone(); f_continuous_beam(py::getitem(&t1054, &Py::str("spans"))?, py::getitem(&t1054, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1055 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1056 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1076 = (t1056).items()?;
        if t1076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1076[0].clone();
        v_v = t1076[1].clone();
        if ({ let t1057 = v_k.clone(); Py::Bool({ let t1058 = v_outputs.clone(); if !py::contains(&t1058, &t1057)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1100(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1101 = v_inputs.clone(); f_damped_sdof(py::getitem(&t1101, &Py::str("m"))?, py::getitem(&t1101, &Py::str("c"))?, py::getitem(&t1101, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1123 = (t1103).items()?;
        if t1123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1123[0].clone();
        v_v = t1123[1].clone();
        if ({ let t1104 = v_k.clone(); Py::Bool({ let t1105 = v_outputs.clone(); if !py::contains(&t1105, &t1104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1146: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1145 = v_inputs.clone(); f_disc_clutch(py::getitem(&t1145, &Py::str("F"))?, py::getitem(&t1145, &Py::str("f"))?, py::getitem(&t1145, &Py::str("D"))?, py::getitem(&t1145, &Py::str("d"))?, py::getitem(&t1145, &Py::str("model"))?, py::method(&t1145, "get", &[Py::str("pairs"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1146 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1147 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1167 = (t1147).items()?;
        if t1167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1167[0].clone();
        v_v = t1167[1].clone();
        if ({ let t1148 = v_k.clone(); Py::Bool({ let t1149 = v_outputs.clone(); if !py::contains(&t1149, &t1148)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1198(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1200: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1199 = v_inputs.clone(); f_dunkerley_shaft(py::getitem(&t1199, &Py::str("masses"))?, py::getitem(&t1199, &Py::str("positions"))?, py::getitem(&t1199, &Py::str("L"))?, py::getitem(&t1199, &Py::str("E"))?, py::getitem(&t1199, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1200 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1201 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1221 = (t1201).items()?;
        if t1221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1221[0].clone();
        v_v = t1221[1].clone();
        if ({ let t1202 = v_k.clone(); Py::Bool({ let t1203 = v_outputs.clone(); if !py::contains(&t1203, &t1202)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1259(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1261: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1260 = v_inputs.clone(); f_bolt_group(py::getitem(&t1260, &Py::str("bolts"))?, py::getitem(&t1260, &Py::str("P"))?, py::getitem(&t1260, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1261 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1262 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1282 = (t1262).items()?;
        if t1282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1282[0].clone();
        v_v = t1282[1].clone();
        if ({ let t1263 = v_k.clone(); Py::Bool({ let t1264 = v_outputs.clone(); if !py::contains(&t1264, &t1263)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1310(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1312: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1311 = v_inputs.clone(); f_euler_column(py::getitem(&t1311, &Py::str("E"))?, py::getitem(&t1311, &Py::str("I"))?, py::getitem(&t1311, &Py::str("L"))?, py::getitem(&t1311, &Py::str("ends"))?, py::method(&t1311, "get", &[Py::str("A"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1312 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1313 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1333 = (t1313).items()?;
        if t1333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1333[0].clone();
        v_v = t1333[1].clone();
        if ({ let t1314 = v_k.clone(); Py::Bool({ let t1315 = v_outputs.clone(); if !py::contains(&t1315, &t1314)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1361: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1360 = v_inputs.clone(); f_eurocode_damage(py::getitem(&t1360, &Py::str("ranges"))?, py::getitem(&t1360, &Py::str("counts"))?, py::getitem(&t1360, &Py::str("dsc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1361 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1362 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1382 = (t1362).items()?;
        if t1382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1382[0].clone();
        v_v = t1382[1].clone();
        if ({ let t1363 = v_k.clone(); Py::Bool({ let t1364 = v_outputs.clone(); if !py::contains(&t1364, &t1363)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1415(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1417: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1416 = v_inputs.clone(); f_mean_stress_life(py::getitem(&t1416, &Py::str("sa"))?, py::getitem(&t1416, &Py::str("sm"))?, py::getitem(&t1416, &Py::str("sf"))?, py::getitem(&t1416, &Py::str("b"))?, py::getitem(&t1416, &Py::str("method"))?, py::method(&t1416, "get", &[Py::str("gamma"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1417 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1418 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1438 = (t1418).items()?;
        if t1438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1438[0].clone();
        v_v = t1438[1].clone();
        if ({ let t1419 = v_k.clone(); Py::Bool({ let t1420 = v_outputs.clone(); if !py::contains(&t1420, &t1419)? { true } else { false } }) }).truthy() {
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
fn s_1456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1458: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1457 = v_inputs.clone(); f_ff_point(py::getitem(&t1457, &Py::str("L"))?, py::getitem(&t1457, &Py::str("P"))?, py::getitem(&t1457, &Py::str("a"))?, py::getitem(&t1457, &Py::str("E"))?, py::getitem(&t1457, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1458 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1479 = (t1459).items()?;
        if t1479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1479[0].clone();
        v_v = t1479[1].clone();
        if ({ let t1460 = v_k.clone(); Py::Bool({ let t1461 = v_outputs.clone(); if !py::contains(&t1461, &t1460)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1493(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1494 = v_inputs.clone(); f_ff_udl(py::getitem(&t1494, &Py::str("L"))?, py::getitem(&t1494, &Py::str("w"))?, py::getitem(&t1494, &Py::str("E"))?, py::getitem(&t1494, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1495 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1516 = (t1496).items()?;
        if t1516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1516[0].clone();
        v_v = t1516[1].clone();
        if ({ let t1497 = v_k.clone(); Py::Bool({ let t1498 = v_outputs.clone(); if !py::contains(&t1498, &t1497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1596: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1595 = v_inputs.clone(); f_force_system(py::getitem(&t1595, &Py::str("F"))?, py::getitem(&t1595, &Py::str("P"))?, py::method(&t1595, "get", &[Py::str("C"), Py::None])?, py::method(&t1595, "get", &[Py::str("O"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1596 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1597 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1617 = (t1597).items()?;
        if t1617.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1617[0].clone();
        v_v = t1617[1].clone();
        if ({ let t1598 = v_k.clone(); Py::Bool({ let t1599 = v_outputs.clone(); if !py::contains(&t1599, &t1598)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1642(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1644: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1643 = v_inputs.clone(); f_forced_response(py::getitem(&t1643, &Py::str("m"))?, py::getitem(&t1643, &Py::str("c"))?, py::getitem(&t1643, &Py::str("k"))?, py::getitem(&t1643, &Py::str("F0"))?, py::getitem(&t1643, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1644 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1645 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1665 = (t1645).items()?;
        if t1665.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1665[0].clone();
        v_v = t1665[1].clone();
        if ({ let t1646 = v_k.clone(); Py::Bool({ let t1647 = v_outputs.clone(); if !py::contains(&t1647, &t1646)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1694(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1696: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1695 = v_inputs.clone(); f_four_bar(py::getitem(&t1695, &Py::str("a"))?, py::getitem(&t1695, &Py::str("b"))?, py::getitem(&t1695, &Py::str("c"))?, py::getitem(&t1695, &Py::str("d"))?, py::getitem(&t1695, &Py::str("theta2"))?, py::getitem(&t1695, &Py::str("branch"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1696 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1697 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1717 = (t1697).items()?;
        if t1717.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1717[0].clone();
        v_v = t1717[1].clone();
        if ({ let t1698 = v_k.clone(); Py::Bool({ let t1699 = v_outputs.clone(); if !py::contains(&t1699, &t1698)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1763(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1765: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1764 = v_inputs.clone(); f_gear_train(py::getitem(&t1764, &Py::str("drivers"))?, py::getitem(&t1764, &Py::str("driven"))?, py::getitem(&t1764, &Py::str("n_in"))?, py::method(&t1764, "get", &[Py::str("T_in"), Py::None])?, py::method(&t1764, "get", &[Py::str("eta"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1765 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1766 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1786 = (t1766).items()?;
        if t1786.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1786[0].clone();
        v_v = t1786[1].clone();
        if ({ let t1767 = v_k.clone(); Py::Bool({ let t1768 = v_outputs.clone(); if !py::contains(&t1768, &t1767)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
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
fn s_1808(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1810: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1809 = v_inputs.clone(); f_hooke_3d(py::getitem(&t1809, &Py::str("sx"))?, py::getitem(&t1809, &Py::str("sy"))?, py::getitem(&t1809, &Py::str("sz"))?, py::getitem(&t1809, &Py::str("txy"))?, py::getitem(&t1809, &Py::str("tyz"))?, py::getitem(&t1809, &Py::str("tzx"))?, py::getitem(&t1809, &Py::str("E"))?, py::getitem(&t1809, &Py::str("nu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1810 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1811 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1831 = (t1811).items()?;
        if t1831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1831[0].clone();
        v_v = t1831[1].clone();
        if ({ let t1812 = v_k.clone(); Py::Bool({ let t1813 = v_outputs.clone(); if !py::contains(&t1813, &t1812)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1854(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1856: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1855 = v_inputs.clone(); f_gerber(py::getitem(&t1855, &Py::str("sa"))?, py::getitem(&t1855, &Py::str("sm"))?, py::getitem(&t1855, &Py::str("Se"))?, py::getitem(&t1855, &Py::str("Sut"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1856 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1857 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1877 = (t1857).items()?;
        if t1877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1877[0].clone();
        v_v = t1877[1].clone();
        if ({ let t1858 = v_k.clone(); Py::Bool({ let t1859 = v_outputs.clone(); if !py::contains(&t1859, &t1858)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1901(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1903: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1902 = v_inputs.clone(); f_goodman(py::getitem(&t1902, &Py::str("sa"))?, py::getitem(&t1902, &Py::str("sm"))?, py::getitem(&t1902, &Py::str("Se"))?, py::getitem(&t1902, &Py::str("Sut"))?, py::method(&t1902, "get", &[Py::str("Sy"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1903 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1904 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1924 = (t1904).items()?;
        if t1924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1924[0].clone();
        v_v = t1924[1].clone();
        if ({ let t1905 = v_k.clone(); Py::Bool({ let t1906 = v_outputs.clone(); if !py::contains(&t1906, &t1905)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1940(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1942: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1941 = v_inputs.clone(); f_helical_spring(py::getitem(&t1941, &Py::str("d"))?, py::getitem(&t1941, &Py::str("D"))?, py::getitem(&t1941, &Py::str("Na"))?, py::getitem(&t1941, &Py::str("G"))?, py::getitem(&t1941, &Py::str("F"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1942 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1943 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1963 = (t1943).items()?;
        if t1963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1963[0].clone();
        v_v = t1963[1].clone();
        if ({ let t1944 = v_k.clone(); Py::Bool({ let t1945 = v_outputs.clone(); if !py::contains(&t1945, &t1944)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1984(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1986: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1985 = v_inputs.clone(); f_tube_section(py::getitem(&t1985, &Py::str("D"))?, py::getitem(&t1985, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1986 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1987 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2007 = (t1987).items()?;
        if t2007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2007[0].clone();
        v_v = t2007[1].clone();
        if ({ let t1988 = v_k.clone(); Py::Bool({ let t1989 = v_outputs.clone(); if !py::contains(&t1989, &t1988)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2024(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2026: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2025 = v_inputs.clone(); f_box_section(py::getitem(&t2025, &Py::str("B"))?, py::getitem(&t2025, &Py::str("H"))?, py::getitem(&t2025, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2026 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2027 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2047 = (t2027).items()?;
        if t2047.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2047[0].clone();
        v_v = t2047[1].clone();
        if ({ let t2028 = v_k.clone(); Py::Bool({ let t2029 = v_outputs.clone(); if !py::contains(&t2029, &t2028)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2048(mut v_T: Py, mut v_D: Py, mut v_d: Py, mut v_L: Py, mut v_G: Py) -> R {
    return Ok(f__hollow_shaft(v_T.clone(), v_D.clone(), v_d.clone(), v_L.clone(), v_G.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2069(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2071: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2070 = v_inputs.clone(); s_2048(py::getitem(&t2070, &Py::str("T"))?, py::getitem(&t2070, &Py::str("D"))?, py::getitem(&t2070, &Py::str("d"))?, py::getitem(&t2070, &Py::str("L"))?, py::getitem(&t2070, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2071 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2072 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2092 = (t2072).items()?;
        if t2092.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2092[0].clone();
        v_v = t2092[1].clone();
        if ({ let t2073 = v_k.clone(); Py::Bool({ let t2074 = v_outputs.clone(); if !py::contains(&t2074, &t2073)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2126(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2128: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2127 = v_inputs.clone(); f_i_section(py::getitem(&t2127, &Py::str("b"))?, py::getitem(&t2127, &Py::str("h"))?, py::getitem(&t2127, &Py::str("tf"))?, py::getitem(&t2127, &Py::str("tw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2128 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2129 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2149 = (t2129).items()?;
        if t2149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2149[0].clone();
        v_v = t2149[1].clone();
        if ({ let t2130 = v_k.clone(); Py::Bool({ let t2131 = v_outputs.clone(); if !py::contains(&t2131, &t2130)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2182: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2181 = v_inputs.clone(); f_johnson_column(py::getitem(&t2181, &Py::str("Sy"))?, py::getitem(&t2181, &Py::str("E"))?, py::getitem(&t2181, &Py::str("L"))?, py::getitem(&t2181, &Py::str("r"))?, py::getitem(&t2181, &Py::str("ends"))?, py::method(&t2181, "get", &[Py::str("A"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2182 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2183 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2203 = (t2183).items()?;
        if t2203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2203[0].clone();
        v_v = t2203[1].clone();
        if ({ let t2184 = v_k.clone(); Py::Bool({ let t2185 = v_outputs.clone(); if !py::contains(&t2185, &t2184)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2223 = v_inputs.clone(); f_lewis_stress(py::getitem(&t2223, &Py::str("Wt"))?, py::getitem(&t2223, &Py::str("F"))?, py::getitem(&t2223, &Py::str("m"))?, py::getitem(&t2223, &Py::str("Y"))?, py::method(&t2223, "get", &[Py::str("Kv"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2245 = (t2225).items()?;
        if t2245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2245[0].clone();
        v_v = t2245[1].clone();
        if ({ let t2226 = v_k.clone(); Py::Bool({ let t2227 = v_outputs.clone(); if !py::contains(&t2227, &t2226)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_224(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t226: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t225 = v_inputs.clone(); f_beam_bending_stress(py::getitem(&t225, &Py::str("M"))?, py::getitem(&t225, &Py::str("I"))?, py::getitem(&t225, &Py::str("y"))?, py::method(&t225, "get", &[Py::str("c"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t226 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t227 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t247 = (t227).items()?;
        if t247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t247[0].clone();
        v_v = t247[1].clone();
        if ({ let t228 = v_k.clone(); Py::Bool({ let t229 = v_outputs.clone(); if !py::contains(&t229, &t228)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2284(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2286: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2285 = v_inputs.clone(); f_marin(py::getitem(&t2285, &Py::str("Sut"))?, py::getitem(&t2285, &Py::str("surface"))?, py::getitem(&t2285, &Py::str("load"))?, py::method(&t2285, "get", &[Py::str("d"), Py::None])?, py::method(&t2285, "get", &[Py::str("R"), Py::Float(0.5)])?, py::method(&t2285, "get", &[Py::str("kd"), Py::Float(1.0)])?, py::method(&t2285, "get", &[Py::str("kf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2286 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2287 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2307 = (t2287).items()?;
        if t2307.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2307[0].clone();
        v_v = t2307[1].clone();
        if ({ let t2288 = v_k.clone(); Py::Bool({ let t2289 = v_outputs.clone(); if !py::contains(&t2289, &t2288)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2393(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2395: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2394 = v_inputs.clone(); f_mdof_chain(py::getitem(&t2394, &Py::str("masses"))?, py::getitem(&t2394, &Py::str("springs"))?, py::getitem(&t2394, &Py::str("ends"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2395 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2396 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2416 = (t2396).items()?;
        if t2416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2416[0].clone();
        v_v = t2416[1].clone();
        if ({ let t2397 = v_k.clone(); Py::Bool({ let t2398 = v_outputs.clone(); if !py::contains(&t2398, &t2397)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2438: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2437 = v_inputs.clone(); f_miner_linear(py::getitem(&t2437, &Py::str("ranges"))?, py::getitem(&t2437, &Py::str("counts"))?, py::getitem(&t2437, &Py::str("Sc"))?, py::getitem(&t2437, &Py::str("m"))?, py::method(&t2437, "get", &[Py::str("Nc"), Py::Float(2000000.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2438 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2439 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2459 = (t2439).items()?;
        if t2459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2459[0].clone();
        v_v = t2459[1].clone();
        if ({ let t2440 = v_k.clone(); Py::Bool({ let t2441 = v_outputs.clone(); if !py::contains(&t2441, &t2440)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2475(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2476 = v_inputs.clone(); f_moment_of_force(py::getitem(&t2476, &Py::str("Fx"))?, py::getitem(&t2476, &Py::str("Fy"))?, py::getitem(&t2476, &Py::str("x"))?, py::getitem(&t2476, &Py::str("y"))?, py::method(&t2476, "get", &[Py::str("x0"), Py::Float(0.0)])?, py::method(&t2476, "get", &[Py::str("y0"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2477 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2478 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2498 = (t2478).items()?;
        if t2498.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2498[0].clone();
        v_v = t2498[1].clone();
        if ({ let t2479 = v_k.clone(); Py::Bool({ let t2480 = v_outputs.clone(); if !py::contains(&t2480, &t2479)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2522(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2524: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2523 = v_inputs.clone(); f_moment_3d(py::getitem(&t2523, &Py::str("r"))?, py::getitem(&t2523, &Py::str("F"))?, py::method(&t2523, "get", &[Py::str("O"), Py::None])?, py::method(&t2523, "get", &[Py::str("axis"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2524 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2525 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2545 = (t2525).items()?;
        if t2545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2545[0].clone();
        v_v = t2545[1].clone();
        if ({ let t2526 = v_k.clone(); Py::Bool({ let t2527 = v_outputs.clone(); if !py::contains(&t2527, &t2526)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2584(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2586: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2585 = v_inputs.clone(); f_pin_shear_bearing(py::getitem(&t2585, &Py::str("P"))?, py::getitem(&t2585, &Py::str("d"))?, py::method(&t2585, "get", &[Py::str("t"), Py::None])?, py::method(&t2585, "get", &[Py::str("n_bolts"), Py::Int(1i64)])?, py::method(&t2585, "get", &[Py::str("planes"), Py::Int(1i64)])?, py::method(&t2585, "get", &[Py::str("tau_allow"), Py::None])?, py::method(&t2585, "get", &[Py::str("sb_allow"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2586 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2587 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2607 = (t2587).items()?;
        if t2607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2607[0].clone();
        v_v = t2607[1].clone();
        if ({ let t2588 = v_k.clone(); Py::Bool({ let t2589 = v_outputs.clone(); if !py::contains(&t2589, &t2588)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2623(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2625: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2624 = v_inputs.clone(); f_plane_transform(py::getitem(&t2624, &Py::str("sx"))?, py::getitem(&t2624, &Py::str("sy"))?, py::getitem(&t2624, &Py::str("txy"))?, py::getitem(&t2624, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2625 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2626 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2646 = (t2626).items()?;
        if t2646.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2646[0].clone();
        v_v = t2646[1].clone();
        if ({ let t2627 = v_k.clone(); Py::Bool({ let t2628 = v_outputs.clone(); if !py::contains(&t2628, &t2627)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2679(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2681: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2680 = v_inputs.clone(); f_planetary(py::getitem(&t2680, &Py::str("Ns"))?, py::getitem(&t2680, &Py::str("Nr"))?, py::method(&t2680, "get", &[Py::str("ws"), Py::None])?, py::method(&t2680, "get", &[Py::str("wr"), Py::None])?, py::method(&t2680, "get", &[Py::str("wc"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2681 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2682 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2702 = (t2682).items()?;
        if t2702.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2702[0].clone();
        v_v = t2702[1].clone();
        if ({ let t2683 = v_k.clone(); Py::Bool({ let t2684 = v_outputs.clone(); if !py::contains(&t2684, &t2683)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2796(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2798: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2797 = v_inputs.clone(); f_polygon_section(py::getitem(&t2797, &Py::str("vertices"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2798 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2799 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2819 = (t2799).items()?;
        if t2819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2819[0].clone();
        v_v = t2819[1].clone();
        if ({ let t2800 = v_k.clone(); Py::Bool({ let t2801 = v_outputs.clone(); if !py::contains(&t2801, &t2800)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_280(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t282: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t281 = v_inputs.clone(); f_beam_frequency(py::getitem(&t281, &Py::str("E"))?, py::getitem(&t281, &Py::str("I"))?, py::getitem(&t281, &Py::str("rho"))?, py::getitem(&t281, &Py::str("A"))?, py::getitem(&t281, &Py::str("L"))?, py::getitem(&t281, &Py::str("ends"))?, py::method(&t281, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t282 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t283 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t303 = (t283).items()?;
        if t303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t303[0].clone();
        v_v = t303[1].clone();
        if ({ let t284 = v_k.clone(); Py::Bool({ let t285 = v_outputs.clone(); if !py::contains(&t285, &t284)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2850(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2852: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2851 = v_inputs.clone(); f_power_screw(py::getitem(&t2851, &Py::str("F"))?, py::getitem(&t2851, &Py::str("dm"))?, py::getitem(&t2851, &Py::str("l"))?, py::getitem(&t2851, &Py::str("f"))?, py::method(&t2851, "get", &[Py::str("alpha"), Py::Float(0.0)])?, py::method(&t2851, "get", &[Py::str("fc"), Py::Float(0.0)])?, py::method(&t2851, "get", &[Py::str("dc"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2852 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2853 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2873 = (t2853).items()?;
        if t2873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2873[0].clone();
        v_v = t2873[1].clone();
        if ({ let t2854 = v_k.clone(); Py::Bool({ let t2855 = v_outputs.clone(); if !py::contains(&t2855, &t2854)? { true } else { false } }) }).truthy() {
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
            v_want = { let t2894 = v_inputs.clone(); f_principal_2d(py::getitem(&t2894, &Py::str("sx"))?, py::getitem(&t2894, &Py::str("sy"))?, py::getitem(&t2894, &Py::str("txy"))?)? };
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
fn s_2956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2958: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2957 = v_inputs.clone(); f_principal_3d(py::getitem(&t2957, &Py::str("sx"))?, py::getitem(&t2957, &Py::str("sy"))?, py::getitem(&t2957, &Py::str("sz"))?, py::getitem(&t2957, &Py::str("txy"))?, py::getitem(&t2957, &Py::str("tyz"))?, py::getitem(&t2957, &Py::str("tzx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2958 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2959 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2979 = (t2959).items()?;
        if t2979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2979[0].clone();
        v_v = t2979[1].clone();
        if ({ let t2960 = v_k.clone(); Py::Bool({ let t2961 = v_outputs.clone(); if !py::contains(&t2961, &t2960)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2996(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2998: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2997 = v_inputs.clone(); f_pc_point(py::getitem(&t2997, &Py::str("L"))?, py::getitem(&t2997, &Py::str("P"))?, py::getitem(&t2997, &Py::str("a"))?, py::getitem(&t2997, &Py::str("E"))?, py::getitem(&t2997, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2998 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2999 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3019 = (t2999).items()?;
        if t3019.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3019[0].clone();
        v_v = t3019[1].clone();
        if ({ let t3000 = v_k.clone(); Py::Bool({ let t3001 = v_outputs.clone(); if !py::contains(&t3001, &t3000)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3033(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3035: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3034 = v_inputs.clone(); f_pc_udl(py::getitem(&t3034, &Py::str("L"))?, py::getitem(&t3034, &Py::str("w"))?, py::getitem(&t3034, &Py::str("E"))?, py::getitem(&t3034, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3035 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3036 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3056 = (t3036).items()?;
        if t3056.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3056[0].clone();
        v_v = t3056[1].clone();
        if ({ let t3037 = v_k.clone(); Py::Bool({ let t3038 = v_outputs.clone(); if !py::contains(&t3038, &t3037)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3093(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3095: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3094 = v_inputs.clone(); f_rainflow_count(py::getitem(&t3094, &Py::str("history"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3095 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3096 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3116 = (t3096).items()?;
        if t3116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3116[0].clone();
        v_v = t3116[1].clone();
        if ({ let t3097 = v_k.clone(); Py::Bool({ let t3098 = v_outputs.clone(); if !py::contains(&t3098, &t3097)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3161(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3163: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3162 = v_inputs.clone(); f_rainflow_damage(py::getitem(&t3162, &Py::str("history"))?, py::getitem(&t3162, &Py::str("Sc"))?, py::getitem(&t3162, &Py::str("m"))?, py::method(&t3162, "get", &[Py::str("Nc"), Py::Float(2000000.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3163 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3164 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3184 = (t3164).items()?;
        if t3184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3184[0].clone();
        v_v = t3184[1].clone();
        if ({ let t3165 = v_k.clone(); Py::Bool({ let t3166 = v_outputs.clone(); if !py::contains(&t3166, &t3165)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3223 = v_inputs.clone(); f_rayleigh_shaft(py::getitem(&t3223, &Py::str("masses"))?, py::getitem(&t3223, &Py::str("positions"))?, py::getitem(&t3223, &Py::str("L"))?, py::getitem(&t3223, &Py::str("E"))?, py::getitem(&t3223, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3245 = (t3225).items()?;
        if t3245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3245[0].clone();
        v_v = t3245[1].clone();
        if ({ let t3226 = v_k.clone(); Py::Bool({ let t3227 = v_outputs.clone(); if !py::contains(&t3227, &t3226)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_324(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t326: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t325 = v_inputs.clone(); f_beam_shear_stress(py::getitem(&t325, &Py::str("V"))?, py::getitem(&t325, &Py::str("Q"))?, py::getitem(&t325, &Py::str("I"))?, py::getitem(&t325, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t326 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t327 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t347 = (t327).items()?;
        if t347.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t347[0].clone();
        v_v = t347[1].clone();
        if ({ let t328 = v_k.clone(); Py::Bool({ let t329 = v_outputs.clone(); if !py::contains(&t329, &t328)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3262: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3261 = v_inputs.clone(); f_rect_section(py::getitem(&t3261, &Py::str("b"))?, py::getitem(&t3261, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3262 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3263 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3283 = (t3263).items()?;
        if t3283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3283[0].clone();
        v_v = t3283[1].clone();
        if ({ let t3264 = v_k.clone(); Py::Bool({ let t3265 = v_outputs.clone(); if !py::contains(&t3265, &t3264)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3309(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3311: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3310 = v_inputs.clone(); f_resultant_forces(py::getitem(&t3310, &Py::str("F"))?, py::getitem(&t3310, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3311 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3312 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3332 = (t3312).items()?;
        if t3332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3332[0].clone();
        v_v = t3332[1].clone();
        if ({ let t3313 = v_k.clone(); Py::Bool({ let t3314 = v_outputs.clone(); if !py::contains(&t3314, &t3313)? { true } else { false } }) }).truthy() {
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
            v_want = { let t3360 = v_inputs.clone(); f_rotating_disk(py::getitem(&t3360, &Py::str("rho"))?, py::getitem(&t3360, &Py::str("omega"))?, py::getitem(&t3360, &Py::str("nu"))?, py::getitem(&t3360, &Py::str("ro"))?, py::method(&t3360, "get", &[Py::str("ri"), Py::Float(0.0)])?)? };
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
fn s_3405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3407: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3406 = v_inputs.clone(); f_unbalance(py::getitem(&t3406, &Py::str("M"))?, py::getitem(&t3406, &Py::str("mu"))?, py::getitem(&t3406, &Py::str("e"))?, py::getitem(&t3406, &Py::str("k"))?, py::getitem(&t3406, &Py::str("c"))?, py::getitem(&t3406, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3407 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3408 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3428 = (t3408).items()?;
        if t3428.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3428[0].clone();
        v_v = t3428[1].clone();
        if ({ let t3409 = v_k.clone(); Py::Bool({ let t3410 = v_outputs.clone(); if !py::contains(&t3410, &t3409)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3458: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3457 = v_inputs.clone(); f_sdof_free(py::getitem(&t3457, &Py::str("m"))?, py::getitem(&t3457, &Py::str("c"))?, py::getitem(&t3457, &Py::str("k"))?, py::getitem(&t3457, &Py::str("x0"))?, py::getitem(&t3457, &Py::str("v0"))?, py::getitem(&t3457, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3458 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3479 = (t3459).items()?;
        if t3479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3479[0].clone();
        v_v = t3479[1].clone();
        if ({ let t3460 = v_k.clone(); Py::Bool({ let t3461 = v_outputs.clone(); if !py::contains(&t3461, &t3460)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3493(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3494 = v_inputs.clone(); f_sdof_natural(py::getitem(&t3494, &Py::str("m"))?, py::getitem(&t3494, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3495 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3516 = (t3496).items()?;
        if t3516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3516[0].clone();
        v_v = t3516[1].clone();
        if ({ let t3497 = v_k.clone(); Py::Bool({ let t3498 = v_outputs.clone(); if !py::contains(&t3498, &t3497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3537(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3539: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3538 = v_inputs.clone(); f_secant_column(py::getitem(&t3538, &Py::str("P"))?, py::getitem(&t3538, &Py::str("e"))?, py::getitem(&t3538, &Py::str("L"))?, py::getitem(&t3538, &Py::str("A"))?, py::getitem(&t3538, &Py::str("I"))?, py::getitem(&t3538, &Py::str("c"))?, py::getitem(&t3538, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3539 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3540 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3560 = (t3540).items()?;
        if t3560.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3560[0].clone();
        v_v = t3560[1].clone();
        if ({ let t3541 = v_k.clone(); Py::Bool({ let t3542 = v_outputs.clone(); if !py::contains(&t3542, &t3541)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3577(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3579: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3578 = v_inputs.clone(); f_single_disk(py::getitem(&t3578, &Py::str("m"))?, py::getitem(&t3578, &Py::str("E"))?, py::getitem(&t3578, &Py::str("I"))?, py::getitem(&t3578, &Py::str("L"))?, py::getitem(&t3578, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3579 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3580 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3600 = (t3580).items()?;
        if t3600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3600[0].clone();
        v_v = t3600[1].clone();
        if ({ let t3581 = v_k.clone(); Py::Bool({ let t3582 = v_outputs.clone(); if !py::contains(&t3582, &t3581)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3631(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3633: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3632 = v_inputs.clone(); f_shaft_de_goodman_d(py::getitem(&t3632, &Py::str("n"))?, py::getitem(&t3632, &Py::str("Ma"))?, py::getitem(&t3632, &Py::str("Mm"))?, py::getitem(&t3632, &Py::str("Ta"))?, py::getitem(&t3632, &Py::str("Tm"))?, py::getitem(&t3632, &Py::str("Kf"))?, py::getitem(&t3632, &Py::str("Kfs"))?, py::getitem(&t3632, &Py::str("Se"))?, py::getitem(&t3632, &Py::str("Sut"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3633 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3634 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3654 = (t3634).items()?;
        if t3654.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3654[0].clone();
        v_v = t3654[1].clone();
        if ({ let t3635 = v_k.clone(); Py::Bool({ let t3636 = v_outputs.clone(); if !py::contains(&t3636, &t3635)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3691: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3690 = v_inputs.clone(); f_shaft_de_goodman_n(py::getitem(&t3690, &Py::str("d"))?, py::getitem(&t3690, &Py::str("Ma"))?, py::getitem(&t3690, &Py::str("Mm"))?, py::getitem(&t3690, &Py::str("Ta"))?, py::getitem(&t3690, &Py::str("Tm"))?, py::getitem(&t3690, &Py::str("Kf"))?, py::getitem(&t3690, &Py::str("Kfs"))?, py::getitem(&t3690, &Py::str("Se"))?, py::getitem(&t3690, &Py::str("Sut"))?, py::method(&t3690, "get", &[Py::str("Sy"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3691 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3692 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3712 = (t3692).items()?;
        if t3712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3712[0].clone();
        v_v = t3712[1].clone();
        if ({ let t3693 = v_k.clone(); Py::Bool({ let t3694 = v_outputs.clone(); if !py::contains(&t3694, &t3693)? { true } else { false } }) }).truthy() {
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
fn s_3726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3728: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3727 = v_inputs.clone(); f_shaft_power(py::getitem(&t3727, &Py::str("P"))?, py::getitem(&t3727, &Py::str("n"))?, py::getitem(&t3727, &Py::str("tau_allow"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3728 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3729 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3749 = (t3729).items()?;
        if t3749.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3749[0].clone();
        v_v = t3749[1].clone();
        if ({ let t3730 = v_k.clone(); Py::Bool({ let t3731 = v_outputs.clone(); if !py::contains(&t3731, &t3730)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_379(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t381: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t380 = v_inputs.clone(); f_bearing_rating(py::getitem(&t380, &Py::str("FD"))?, py::getitem(&t380, &Py::str("LD"))?, py::getitem(&t380, &Py::str("n"))?, py::getitem(&t380, &Py::str("R"))?, py::getitem(&t380, &Py::str("kind"))?, py::method(&t380, "get", &[Py::str("af"), Py::Float(1.0)])?, py::method(&t380, "get", &[Py::str("LR"), Py::Float(1000000.0)])?, py::method(&t380, "get", &[Py::str("x0"), Py::None])?, py::method(&t380, "get", &[Py::str("theta"), Py::None])?, py::method(&t380, "get", &[Py::str("b"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t381 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t382 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t402 = (t382).items()?;
        if t402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t402[0].clone();
        v_v = t402[1].clone();
        if ({ let t383 = v_k.clone(); Py::Bool({ let t384 = v_outputs.clone(); if !py::contains(&t384, &t383)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3821(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3823: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3822 = v_inputs.clone(); f_exact_whirl(py::getitem(&t3822, &Py::str("masses"))?, py::getitem(&t3822, &Py::str("positions"))?, py::getitem(&t3822, &Py::str("L"))?, py::getitem(&t3822, &Py::str("E"))?, py::getitem(&t3822, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3823 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3824 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3844 = (t3824).items()?;
        if t3844.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3844[0].clone();
        v_v = t3844[1].clone();
        if ({ let t3825 = v_k.clone(); Py::Bool({ let t3826 = v_outputs.clone(); if !py::contains(&t3826, &t3825)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3866(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3868: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3867 = v_inputs.clone(); f_shrink_fit(py::getitem(&t3867, &Py::str("delta"))?, py::getitem(&t3867, &Py::str("R"))?, py::getitem(&t3867, &Py::str("ro"))?, py::getitem(&t3867, &Py::str("E"))?, py::method(&t3867, "get", &[Py::str("ri"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3868 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3869 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3889 = (t3869).items()?;
        if t3889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3889[0].clone();
        v_v = t3889[1].clone();
        if ({ let t3870 = v_k.clone(); Py::Bool({ let t3871 = v_outputs.clone(); if !py::contains(&t3871, &t3870)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3909(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3911: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3910 = v_inputs.clone(); f_ss_point_load(py::getitem(&t3910, &Py::str("L"))?, py::getitem(&t3910, &Py::str("P"))?, py::getitem(&t3910, &Py::str("a"))?, py::getitem(&t3910, &Py::str("E"))?, py::getitem(&t3910, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3911 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3912 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3932 = (t3912).items()?;
        if t3932.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3932[0].clone();
        v_v = t3932[1].clone();
        if ({ let t3913 = v_k.clone(); Py::Bool({ let t3914 = v_outputs.clone(); if !py::contains(&t3914, &t3913)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3967(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3969: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3968 = v_inputs.clone(); f_ss_point_loads(py::getitem(&t3968, &Py::str("L"))?, py::getitem(&t3968, &Py::str("x"))?, py::getitem(&t3968, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3969 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3970 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3990 = (t3970).items()?;
        if t3990.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3990[0].clone();
        v_v = t3990[1].clone();
        if ({ let t3971 = v_k.clone(); Py::Bool({ let t3972 = v_outputs.clone(); if !py::contains(&t3972, &t3971)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4004(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4006: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4005 = v_inputs.clone(); f_ss_udl(py::getitem(&t4005, &Py::str("L"))?, py::getitem(&t4005, &Py::str("w"))?, py::getitem(&t4005, &Py::str("E"))?, py::getitem(&t4005, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4006 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4007 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4027 = (t4007).items()?;
        if t4027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4027[0].clone();
        v_v = t4027[1].clone();
        if ({ let t4008 = v_k.clone(); Py::Bool({ let t4009 = v_outputs.clone(); if !py::contains(&t4009, &t4008)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4048(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4050: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4049 = v_inputs.clone(); f_slider_crank(py::getitem(&t4049, &Py::str("r"))?, py::getitem(&t4049, &Py::str("l"))?, py::getitem(&t4049, &Py::str("theta"))?, py::getitem(&t4049, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4050 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4051 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4071 = (t4051).items()?;
        if t4071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4071[0].clone();
        v_v = t4071[1].clone();
        if ({ let t4052 = v_k.clone(); Py::Bool({ let t4053 = v_outputs.clone(); if !py::contains(&t4053, &t4052)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4094(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4096: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4095 = v_inputs.clone(); f_sn_life(py::getitem(&t4095, &Py::str("Sut"))?, py::getitem(&t4095, &Py::str("Se"))?, py::getitem(&t4095, &Py::str("f"))?, py::getitem(&t4095, &Py::str("sa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4096 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4097 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4117 = (t4097).items()?;
        if t4117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4117[0].clone();
        v_v = t4117[1].clone();
        if ({ let t4098 = v_k.clone(); Py::Bool({ let t4099 = v_outputs.clone(); if !py::contains(&t4099, &t4098)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4140(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4141 = v_inputs.clone(); f_sn_strength(py::getitem(&t4141, &Py::str("Sut"))?, py::getitem(&t4141, &Py::str("Se"))?, py::getitem(&t4141, &Py::str("f"))?, py::getitem(&t4141, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4142 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4143 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4163 = (t4143).items()?;
        if t4163.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4163[0].clone();
        v_v = t4163[1].clone();
        if ({ let t4144 = v_k.clone(); Py::Bool({ let t4145 = v_outputs.clone(); if !py::contains(&t4145, &t4144)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4186: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4185 = v_inputs.clone(); f_soderberg(py::getitem(&t4185, &Py::str("sa"))?, py::getitem(&t4185, &Py::str("sm"))?, py::getitem(&t4185, &Py::str("Se"))?, py::getitem(&t4185, &Py::str("Sy"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4186 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4187 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4207 = (t4187).items()?;
        if t4207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4207[0].clone();
        v_v = t4207[1].clone();
        if ({ let t4188 = v_k.clone(); Py::Bool({ let t4189 = v_outputs.clone(); if !py::contains(&t4189, &t4188)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4208(mut v_T: Py, mut v_d: Py, mut v_L: Py, mut v_G: Py) -> R {
    return Ok(f_shaft_torsion(v_T.clone(), v_d.clone(), v_L.clone(), v_G.clone(), Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4229(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4231: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4230 = v_inputs.clone(); s_4208(py::getitem(&t4230, &Py::str("T"))?, py::getitem(&t4230, &Py::str("d"))?, py::getitem(&t4230, &Py::str("L"))?, py::getitem(&t4230, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4231 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4232 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4252 = (t4232).items()?;
        if t4252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4252[0].clone();
        v_v = t4252[1].clone();
        if ({ let t4233 = v_k.clone(); Py::Bool({ let t4234 = v_outputs.clone(); if !py::contains(&t4234, &t4233)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4283(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4285: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4284 = v_inputs.clone(); f_spur_contact(py::getitem(&t4284, &Py::str("N1"))?, py::getitem(&t4284, &Py::str("N2"))?, py::getitem(&t4284, &Py::str("m"))?, py::getitem(&t4284, &Py::str("phi"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4285 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4286 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4306 = (t4286).items()?;
        if t4306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4306[0].clone();
        v_v = t4306[1].clone();
        if ({ let t4287 = v_k.clone(); Py::Bool({ let t4288 = v_outputs.clone(); if !py::contains(&t4288, &t4287)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4322(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4324: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4323 = v_inputs.clone(); f_fixed_shaft(py::getitem(&t4323, &Py::str("T0"))?, py::getitem(&t4323, &Py::str("a"))?, py::getitem(&t4323, &Py::str("b"))?, py::getitem(&t4323, &Py::str("d1"))?, py::getitem(&t4323, &Py::str("d2"))?, py::getitem(&t4323, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4324 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4325 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4345 = (t4325).items()?;
        if t4345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4345[0].clone();
        v_v = t4345[1].clone();
        if ({ let t4326 = v_k.clone(); Py::Bool({ let t4327 = v_outputs.clone(); if !py::contains(&t4327, &t4326)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_434(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t436: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t435 = v_inputs.clone(); f_bearing_life(py::getitem(&t435, &Py::str("C"))?, py::getitem(&t435, &Py::str("F"))?, py::getitem(&t435, &Py::str("n"))?, py::getitem(&t435, &Py::str("R"))?, py::getitem(&t435, &Py::str("kind"))?, py::method(&t435, "get", &[Py::str("LR"), Py::Float(1000000.0)])?, py::method(&t435, "get", &[Py::str("x0"), Py::None])?, py::method(&t435, "get", &[Py::str("theta"), Py::None])?, py::method(&t435, "get", &[Py::str("b"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t436 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t437 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t457 = (t437).items()?;
        if t457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t457[0].clone();
        v_v = t457[1].clone();
        if ({ let t438 = v_k.clone(); Py::Bool({ let t439 = v_outputs.clone(); if !py::contains(&t439, &t438)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4365(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4367: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4366 = v_inputs.clone(); f_stepped_shaft(py::getitem(&t4366, &Py::str("T"))?, py::getitem(&t4366, &Py::str("lengths"))?, py::getitem(&t4366, &Py::str("diameters"))?, py::getitem(&t4366, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4367 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4368 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4388 = (t4368).items()?;
        if t4388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4388[0].clone();
        v_v = t4388[1].clone();
        if ({ let t4369 = v_k.clone(); Py::Bool({ let t4370 = v_outputs.clone(); if !py::contains(&t4370, &t4369)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4420(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4422: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4421 = v_inputs.clone(); f_rosette(py::getitem(&t4421, &Py::str("ea"))?, py::getitem(&t4421, &Py::str("eb"))?, py::getitem(&t4421, &Py::str("ec"))?, py::getitem(&t4421, &Py::str("E"))?, py::getitem(&t4421, &Py::str("nu"))?, py::getitem(&t4421, &Py::str("layout"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4422 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4423 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4443 = (t4423).items()?;
        if t4443.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4443[0].clone();
        v_v = t4443[1].clone();
        if ({ let t4424 = v_k.clone(); Py::Bool({ let t4425 = v_outputs.clone(); if !py::contains(&t4425, &t4424)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4476(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4478: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4477 = v_inputs.clone(); f_t_section(py::getitem(&t4477, &Py::str("bf"))?, py::getitem(&t4477, &Py::str("tf"))?, py::getitem(&t4477, &Py::str("h"))?, py::getitem(&t4477, &Py::str("tw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4478 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4479 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4499 = (t4479).items()?;
        if t4499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4499[0].clone();
        v_v = t4499[1].clone();
        if ({ let t4480 = v_k.clone(); Py::Bool({ let t4481 = v_outputs.clone(); if !py::contains(&t4481, &t4480)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4538(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4540: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4539 = v_inputs.clone(); f_restrained_bar_thermal(py::getitem(&t4539, &Py::str("E"))?, py::getitem(&t4539, &Py::str("alpha"))?, py::getitem(&t4539, &Py::str("dT"))?, py::method(&t4539, "get", &[Py::str("L"), Py::None])?, py::method(&t4539, "get", &[Py::str("A"), Py::None])?, py::method(&t4539, "get", &[Py::str("gap"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4540 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4541 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4561 = (t4541).items()?;
        if t4561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4561[0].clone();
        v_v = t4561[1].clone();
        if ({ let t4542 = v_k.clone(); Py::Bool({ let t4543 = v_outputs.clone(); if !py::contains(&t4543, &t4542)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4596: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4595 = v_inputs.clone(); f_lame(py::getitem(&t4595, &Py::str("pi"))?, py::getitem(&t4595, &Py::str("po"))?, py::getitem(&t4595, &Py::str("a"))?, py::getitem(&t4595, &Py::str("b"))?, py::method(&t4595, "get", &[Py::str("r"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4596 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4597 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4617 = (t4597).items()?;
        if t4617.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4617[0].clone();
        v_v = t4617[1].clone();
        if ({ let t4598 = v_k.clone(); Py::Bool({ let t4599 = v_outputs.clone(); if !py::contains(&t4599, &t4598)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4637(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4639: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4638 = v_inputs.clone(); f_thin_vessel(py::getitem(&t4638, &Py::str("p"))?, py::getitem(&t4638, &Py::str("r"))?, py::getitem(&t4638, &Py::str("t"))?, py::getitem(&t4638, &Py::str("shape"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4639 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4640 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4660 = (t4640).items()?;
        if t4660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4660[0].clone();
        v_v = t4660[1].clone();
        if ({ let t4641 = v_k.clone(); Py::Bool({ let t4642 = v_outputs.clone(); if !py::contains(&t4642, &t4641)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4771(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4773: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4772 = v_inputs.clone(); f_truss(py::getitem(&t4772, &Py::str("nodes"))?, py::getitem(&t4772, &Py::str("members"))?, py::getitem(&t4772, &Py::str("pin"))?, py::getitem(&t4772, &Py::str("roller"))?, py::getitem(&t4772, &Py::str("loads"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4773 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4774 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4794 = (t4774).items()?;
        if t4794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4794[0].clone();
        v_v = t4794[1].clone();
        if ({ let t4775 = v_k.clone(); Py::Bool({ let t4776 = v_outputs.clone(); if !py::contains(&t4776, &t4775)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4813(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4815: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4814 = v_inputs.clone(); f_two_cable_tension(py::getitem(&t4814, &Py::str("W"))?, py::getitem(&t4814, &Py::str("alpha"))?, py::getitem(&t4814, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4815 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4816 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4836 = (t4816).items()?;
        if t4836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4836[0].clone();
        v_v = t4836[1].clone();
        if ({ let t4817 = v_k.clone(); Py::Bool({ let t4818 = v_outputs.clone(); if !py::contains(&t4818, &t4817)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t485: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t484 = v_inputs.clone(); f_belt_power(py::getitem(&t484, &Py::str("T1"))?, py::getitem(&t484, &Py::str("m"))?, py::getitem(&t484, &Py::str("v"))?, py::getitem(&t484, &Py::str("mu"))?, py::getitem(&t484, &Py::str("theta"))?, py::method(&t484, "get", &[Py::str("beta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t485 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t486 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t506 = (t486).items()?;
        if t506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t506[0].clone();
        v_v = t506[1].clone();
        if ({ let t487 = v_k.clone(); Py::Bool({ let t488 = v_outputs.clone(); if !py::contains(&t488, &t487)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4853(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4855: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4854 = v_inputs.clone(); f_two_rotor(py::getitem(&t4854, &Py::str("J1"))?, py::getitem(&t4854, &Py::str("J2"))?, py::getitem(&t4854, &Py::str("kt"))?, py::method(&t4854, "get", &[Py::str("L"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4855 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4856 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4876 = (t4856).items()?;
        if t4876.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4876[0].clone();
        v_v = t4876[1].clone();
        if ({ let t4857 = v_k.clone(); Py::Bool({ let t4858 = v_outputs.clone(); if !py::contains(&t4858, &t4857)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4893(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4894 = v_inputs.clone(); f_isolator_design(py::getitem(&t4894, &Py::str("m"))?, py::getitem(&t4894, &Py::str("omega"))?, py::getitem(&t4894, &Py::str("TR"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4896 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4916 = (t4896).items()?;
        if t4916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4916[0].clone();
        v_v = t4916[1].clone();
        if ({ let t4897 = v_k.clone(); Py::Bool({ let t4898 = v_outputs.clone(); if !py::contains(&t4898, &t4897)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4939(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4941: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4940 = v_inputs.clone(); f_transmissibility(py::getitem(&t4940, &Py::str("zeta"))?, py::getitem(&t4940, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4941 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4942 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4962 = (t4942).items()?;
        if t4962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4962[0].clone();
        v_v = t4962[1].clone();
        if ({ let t4943 = v_k.clone(); Py::Bool({ let t4944 = v_outputs.clone(); if !py::contains(&t4944, &t4943)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4981(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4983: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4982 = v_inputs.clone(); f_wahl_factor(py::getitem(&t4982, &Py::str("d"))?, py::getitem(&t4982, &Py::str("D"))?, py::method(&t4982, "get", &[Py::str("F"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4983 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4984 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5004 = (t4984).items()?;
        if t5004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5004[0].clone();
        v_v = t5004[1].clone();
        if ({ let t4985 = v_k.clone(); Py::Bool({ let t4986 = v_outputs.clone(); if !py::contains(&t4986, &t4985)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5052(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5054: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5053 = v_inputs.clone(); f_yield_safety(py::getitem(&t5053, &Py::str("Sy"))?, py::getitem(&t5053, &Py::str("sx"))?, py::method(&t5053, "get", &[Py::str("sy"), Py::Float(0.0)])?, py::method(&t5053, "get", &[Py::str("sz"), Py::Float(0.0)])?, py::method(&t5053, "get", &[Py::str("txy"), Py::Float(0.0)])?, py::method(&t5053, "get", &[Py::str("tyz"), Py::Float(0.0)])?, py::method(&t5053, "get", &[Py::str("tzx"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5054 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5055 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5075 = (t5055).items()?;
        if t5075.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5075[0].clone();
        v_v = t5075[1].clone();
        if ({ let t5056 = v_k.clone(); Py::Bool({ let t5057 = v_outputs.clone(); if !py::contains(&t5057, &t5056)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_530(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t532: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t531 = v_inputs.clone(); f_capstan(py::getitem(&t531, &Py::str("T2"))?, py::getitem(&t531, &Py::str("mu"))?, py::getitem(&t531, &Py::str("theta"))?, py::method(&t531, "get", &[Py::str("beta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t532 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t533 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t553 = (t533).items()?;
        if t553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t553[0].clone();
        v_v = t553[1].clone();
        if ({ let t534 = v_k.clone(); Py::Bool({ let t535 = v_outputs.clone(); if !py::contains(&t535, &t534)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_579(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t581: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t580 = v_inputs.clone(); f_bolt_torque(py::getitem(&t580, &Py::str("K"))?, py::getitem(&t580, &Py::str("d"))?, py::method(&t580, "get", &[Py::str("F"), Py::None])?, py::method(&t580, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t581 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t582 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t602 = (t582).items()?;
        if t602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t602[0].clone();
        v_v = t602[1].clone();
        if ({ let t583 = v_k.clone(); Py::Bool({ let t584 = v_outputs.clone(); if !py::contains(&t584, &t583)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_634(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t636: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t635 = v_inputs.clone(); f_brittle_mohr(py::getitem(&t635, &Py::str("sA"))?, py::getitem(&t635, &Py::str("sB"))?, py::getitem(&t635, &Py::str("Sut"))?, py::getitem(&t635, &Py::str("Suc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t636 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t637 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t657 = (t637).items()?;
        if t657.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t657[0].clone();
        v_v = t657[1].clone();
        if ({ let t638 = v_k.clone(); Py::Bool({ let t639 = v_outputs.clone(); if !py::contains(&t639, &t638)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_676(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t678: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t677 = v_inputs.clone(); f_cantilever_point_load(py::getitem(&t677, &Py::str("L"))?, py::getitem(&t677, &Py::str("P"))?, py::getitem(&t677, &Py::str("E"))?, py::getitem(&t677, &Py::str("I"))?, py::method(&t677, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t678 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t679 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t699 = (t679).items()?;
        if t699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t699[0].clone();
        v_v = t699[1].clone();
        if ({ let t680 = v_k.clone(); Py::Bool({ let t681 = v_outputs.clone(); if !py::contains(&t681, &t680)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_713(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t714 = v_inputs.clone(); f_cantilever_triangular(py::getitem(&t714, &Py::str("L"))?, py::getitem(&t714, &Py::str("w"))?, py::getitem(&t714, &Py::str("E"))?, py::getitem(&t714, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t716 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t736 = (t716).items()?;
        if t736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t736[0].clone();
        v_v = t736[1].clone();
        if ({ let t717 = v_k.clone(); Py::Bool({ let t718 = v_outputs.clone(); if !py::contains(&t718, &t717)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_750(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t752: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t751 = v_inputs.clone(); f_cantilever_udl(py::getitem(&t751, &Py::str("L"))?, py::getitem(&t751, &Py::str("w"))?, py::getitem(&t751, &Py::str("E"))?, py::getitem(&t751, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t752 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t753 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t773 = (t753).items()?;
        if t773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t773[0].clone();
        v_v = t773[1].clone();
        if ({ let t754 = v_k.clone(); Py::Bool({ let t755 = v_outputs.clone(); if !py::contains(&t755, &t754)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_807(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t809: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t808 = v_inputs.clone(); f_channel_section(py::getitem(&t808, &Py::str("b"))?, py::getitem(&t808, &Py::str("h"))?, py::getitem(&t808, &Py::str("tf"))?, py::getitem(&t808, &Py::str("tw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t809 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t810 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t830 = (t810).items()?;
        if t830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t830[0].clone();
        v_v = t830[1].clone();
        if ({ let t811 = v_k.clone(); Py::Bool({ let t812 = v_outputs.clone(); if !py::contains(&t812, &t811)? { true } else { false } }) }).truthy() {
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
fn s_844(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t846: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t845 = v_inputs.clone(); f_circle_section(py::getitem(&t845, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t846 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t847 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t867 = (t847).items()?;
        if t867.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t867[0].clone();
        v_v = t867[1].clone();
        if ({ let t848 = v_k.clone(); Py::Bool({ let t849 = v_outputs.clone(); if !py::contains(&t849, &t848)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_889(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t891: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t890 = v_inputs.clone(); f_combined_rect(py::getitem(&t890, &Py::str("b"))?, py::getitem(&t890, &Py::str("h"))?, py::getitem(&t890, &Py::str("N"))?, py::getitem(&t890, &Py::str("Mx"))?, py::getitem(&t890, &Py::str("My"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t891 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t892 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t912 = (t892).items()?;
        if t912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t912[0].clone();
        v_v = t912[1].clone();
        if ({ let t893 = v_k.clone(); Py::Bool({ let t894 = v_outputs.clone(); if !py::contains(&t894, &t893)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_972(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t974: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t973 = v_inputs.clone(); f_composite_rects(py::getitem(&t973, &Py::str("rects"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t974 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t975 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t995 = (t975).items()?;
        if t995.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t995[0].clone();
        v_v = t995[1].clone();
        if ({ let t976 = v_k.clone(); Py::Bool({ let t977 = v_outputs.clone(); if !py::contains(&t977, &t976)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
