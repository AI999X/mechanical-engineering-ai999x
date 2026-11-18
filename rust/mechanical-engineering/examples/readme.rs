//! Mechanical Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p mechanical-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&mechanical_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "cantilever_point_load", "inputs": {"L": "2 m", "P": "5 kN", "E": "200 GPa", "I": "8e-06 m^4"}}));
    println!();
    show(json!({"key": "johnson_column", "inputs": {"Sy": "250 MPa", "E": "200 GPa", "L": "1.5 m", "r": "20 mm", "ends": "pinned-pinned"}}));
    println!();
    show(json!({"key": "brittle_failure_mohr", "inputs": {"sA": "90 MPa", "sB": "-50 MPa", "Sut": "210 MPa", "Suc": "750 MPa"}}));
    println!();
    show(json!({"key": "marin_endurance_limit", "inputs": {"Sut": "690 MPa", "surface": "machined", "load": "bending", "d": "32 mm", "R": "0.99"}}));
    println!();
    show(json!({"key": "belt_friction_capstan", "inputs": {"T2": "200 N", "mu": "0.3", "theta": "180 deg"}}));
    println!();
    show(json!({"key": "shrink_fit_pressure", "inputs": {"delta": "0.02 mm", "R": "25 mm", "ro": "50 mm", "E": "207 GPa"}}));
    println!();
    show(json!({"key": "composite_section_properties", "inputs": {"rects": [[200, 20, 0, 10], [20, 300, 0, 170]]}}));
    println!();
    show(json!({"key": "force_system_resultant", "inputs": {"F": [[0, -100, 0], [0, -200, 0], [50, 0, 0]], "P": [[1, 0, 0], [3, 0, 0], [0, 2, 0]]}}));
    println!();
    show(json!({"key": "combined_axial_and_bending_stress", "inputs": {"b": "100 mm", "h": "200 mm", "N": "-50 kN", "Mx": "8 kN*m", "My": "2 kN*m"}}));
    println!();
    show(json!({"key": "bar_fixed_at_both_ends", "inputs": {"P": "60 kN", "L1": "0.4 m", "A1": "500 mm^2", "E1": "200 GPa", "L2": "0.4 m", "A2": "500 mm^2", "E2": "200 GPa"}}));
    println!();
    show(json!({"key": "beam_bending_stress", "inputs": {"M": "10 kN*m", "I": "8e-06 m^4", "y": "100 mm"}}));
    println!();
    show(json!({"key": "continuous_beam_three_moment", "inputs": {"spans": [6, 6], "w": [10, 10]}}));
    println!();
    show(json!({"key": "stepped_shaft_twist", "inputs": {"T": "500 N*m", "lengths": [400, 600], "diameters": [40, 30], "G": "80 GPa"}}));
    println!();
    show(json!({"key": "shaft_whirling_speeds", "inputs": {"masses": [50], "positions": [0.5], "L": "1 m", "E": "200 GPa", "I": "2e-07 m^4"}}));
    println!();
    show(json!({"key": "cantilever_triangular_load", "inputs": {"L": "3 m", "w": "6 kN/m", "E": "200 GPa", "I": "2e-05 m^4"}}));
}
