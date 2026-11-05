//! The question reader between capabilities that take inputs of the same kinds (ask.rs, siblings): only the wording
//! tells them apart, so the words' favourite is never overruled by a sibling giving a result of the same name (a
//! hogging moment) that happens to read completely.
//! Found by tools/ask_module_compare.py (stress dataset, 2026-11-03): a propped cantilever's question was answered as
//! a fixed-fixed beam (17,491 N m instead of 26,236 N m).

use mechanical_engineering_ai999x::pack;
use serde_json::Value;

fn ask(q: &str) -> Value {
    serde_json::from_str(&ax_engine::ask::ask_json(pack(), q)).unwrap()
}

#[test]
fn a_propped_cantilever_is_never_answered_as_a_fixed_fixed_beam() {
    let a = ask("In a structural laboratory test, a propped cantilever beam fixed at end A and simply supported at end \
                 B is subjected to a uniform load per unit length (downward) of 8155.34 N/m across its span of 5.07309 \
                 m. Given the material's Young's modulus of 1.562e11 Pa and the cross-section's second moment of area \
                 of 2.630e-4 m^4, calculate the hogging moment at A.");
    assert!(a["outcome"] != "SOLVED" || a["capability"] == "propped_cantilever_udl", "{a}");
}

#[test]
fn a_fixed_fixed_beam_question_is_still_answered() {
    let a = ask("A fixed-fixed beam of span 6 m carries a uniformly distributed load of 10 kN/m, with E = 200 GPa and \
                 I = 8e-5 m^4. Find the hogging moment at each fixed end.");
    assert_eq!(a["outcome"], "SOLVED", "{a}");
    assert_eq!(a["capability"], "fixed_fixed_beam_udl", "{a}");
}
