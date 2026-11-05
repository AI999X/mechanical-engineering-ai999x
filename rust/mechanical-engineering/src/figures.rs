//! Mechanical Engineering's own live figures (ax_engine::figkit): every capability's published figure kinds, drawn for
//! the visitor's inputs from what the model computed (beams with their shear and moment diagrams, cross-sections, fatigue
//! diagrams, Mohr's circles, mechanisms, gears, vibration responses, ...). Written by hand for the Rust build (not
//! generated). A figure the inputs make impossible is left out, never drawn wrong (docs/FIGURE_PARITY.md).

use crate::figures_beam as bm;
use crate::figures_machine as mc;
use crate::figures_section as sc;
use crate::figures_stress as st;
use ax_engine::figkit::Solved;
use ax_engine::figures::Figure;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // beams
        "simply_supported_beam_point_load" => vec![bm::ss_point(s)],
        "simply_supported_beam_udl" => vec![bm::ss_udl(s)],
        "simply_supported_beam_point_loads" => vec![bm::ss_loads_chart(s), bm::ss_loads_full(s)],
        "cantilever_point_load" => vec![bm::cantilever_shape(s), bm::cantilever_point(s)],
        "cantilever_triangular_load" => vec![bm::cantilever_tri(s)],
        "cantilever_udl" => vec![bm::cantilever_udl(s)],
        "fixed_fixed_beam_point_load" => vec![bm::ff_point(s)],
        "fixed_fixed_beam_udl" => vec![bm::ff_udl(s)],
        "propped_cantilever_point_load" => vec![bm::pc_point(s)],
        "propped_cantilever_udl" => vec![bm::pc_udl(s)],
        "continuous_beam_three_moment" => vec![bm::continuous(s)],
        // sections
        "rectangular_section_properties" => vec![sc::rectangular(s)],
        "circular_section_properties" => vec![sc::circular(s)],
        "hollow_circular_section_properties" => vec![sc::hollow_circular(s)],
        "hollow_rectangular_section_properties" => vec![sc::hollow_rectangular(s)],
        "i_section_properties" => vec![sc::i_section(s)],
        "channel_section_properties" => vec![sc::channel(s)],
        "t_section_properties" => vec![sc::t_section(s)],
        "angle_section_properties" => vec![sc::angle(s)],
        "composite_section_properties" => vec![sc::composite(s)],
        "polygon_section_properties" => vec![sc::polygon(s)],
        "combined_axial_and_bending_stress" => vec![sc::combined(s)],
        // fatigue and failure
        "goodman_fatigue_safety_factor" => vec![st::goodman_all(s), st::goodman_one(s)],
        "gerber_fatigue_safety_factor" => vec![st::single_fatigue(s, "Gerber", "Sut")],
        "soderberg_fatigue_safety_factor" => vec![st::single_fatigue(s, "Soderberg", "Sy")],
        "asme_elliptic_fatigue_safety_factor" => vec![st::single_fatigue(s, "ASME-elliptic", "Sy")],
        "yield_criteria_safety_factor" => vec![st::yield_envelope(s)],
        "brittle_failure_mohr" => vec![st::brittle_envelope(s)],
        "sn_fatigue_life" => vec![st::sn_line(s)],
        "rainflow_counting" => vec![st::rainflow(s)],
        // stress states
        "plane_stress_transformation" => vec![st::plane_transform_mohr(s)],
        "principal_stresses_2d" => vec![st::principal2d_element(s), st::principal2d_mohr(s)],
        "principal_stresses_3d" => vec![st::principal3d_mohr(s)],
        "strain_rosette" => vec![st::rosette_mohr(s)],
        "thin_walled_pressure_vessel" => vec![st::vessel_element(s)],
        "solid_shaft_torsion" => vec![st::torsion_profile(s, false)],
        "hollow_shaft_torsion" => vec![st::torsion_profile(s, true)],
        "thick_walled_cylinder" => vec![st::lame_profile(s)],
        // machines and structures
        "dunkerley_critical_speed" => vec![mc::dunkerley(s)],
        "rayleigh_critical_speed" => vec![mc::rayleigh(s)],
        "shaft_whirling_speeds" => vec![mc::whirling(s)],
        "shaft_critical_speed_single_disk" => vec![mc::single_disk(s)],
        "sdof_natural_frequency" => vec![mc::sdof_schematic(s)],
        "mdof_spring_mass_chain" => vec![mc::mdof_chain(s)],
        "eccentric_bolt_group" => vec![mc::bolt_group(s)],
        "belt_drive_power" => vec![mc::belt_drive(s)],
        "belt_friction_capstan" => vec![mc::capstan(s)],
        "moment_of_a_force" => vec![mc::moment_force(s)],
        "resultant_of_concurrent_forces" => vec![mc::resultant(s)],
        "two_cable_equilibrium" => vec![mc::two_cable(s)],
        "four_bar_linkage_position" => vec![mc::four_bar(s)],
        "slider_crank_kinematics" => vec![mc::slider_crank(s)],
        "planetary_gear_train" => vec![mc::planetary(s)],
        "spur_gear_contact_ratio" => vec![mc::spur_mesh(s)],
        "axial_bar_segments" => vec![mc::axial_bar(s)],
        "bar_fixed_at_both_ends" => vec![mc::bar_two_walls(s)],
        "statically_indeterminate_shaft" => vec![mc::fixed_shaft(s)],
        "stepped_shaft_twist" => vec![mc::stepped_shaft(s)],
        "euler_buckling" => vec![mc::euler_column(s)],
        "secant_column" => vec![mc::secant_family(s)],
        "wahl_factor" => vec![mc::wahl_field(s)],
        "truss_member_forces" => vec![mc::truss(s)],
        // vibration
        "damped_sdof_parameters" => vec![mc::damped_decision(s), mc::damped_response(s)],
        "forced_vibration_amplitude" => vec![mc::forced_curves(s)],
        "sdof_free_response" => vec![mc::free_response(s)],
        "two_rotor_torsional_vibration" => vec![mc::two_rotor_mode(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}
