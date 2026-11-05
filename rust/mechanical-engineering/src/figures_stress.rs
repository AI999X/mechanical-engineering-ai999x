//! Stress and fatigue: fatigue diagrams and failure envelopes traced by the model's own safety factor along
//! proportional load lines, Mohr's circles traced by its stress transformation, stress elements, stress profiles
//! through a section, the S-N line and rainflow cycle histograms.

use crate::figures_plot::{self as pl, mpa, Plot};
use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

// ------------------------------------------------------------------------------------------------ fatigue diagrams
/// The failure line of one criterion in the (mean, alternating) plane, MPa: along each of `n` proportional load lines
/// the model's safety factor scales the load to the line. `key` None: this capability, else another one with `fixed`.
fn fatigue_trace(s: &Solved, key: Option<&str>, fixed: &[(&str, f64)], n: usize) -> Vec<(f64, f64)> {
    let (sa, sm) = (s.inp("sa").unwrap_or(1e8), s.inp("sm").unwrap_or(0.0));
    let s0 = (sa * sa + sm * sm).sqrt().max(1.0);
    let mut out = vec![];
    for i in 0..n {
        // the first load line just above the mean-stress axis (a purely static load has no alternating part)
        let phi = (std::f64::consts::FRAC_PI_2 * i as f64 / (n - 1) as f64).max(2e-4);
        let (m, a) = (s0 * phi.cos(), s0 * phi.sin());
        let ans = match key {
            None => s.with(&[("sm", m), ("sa", a)]),
            Some(kk) => {
                let mut set: Vec<(&str, Input)> = fixed.iter().map(|(n, v)| (*n, Input::Float(*v))).collect();
                set.push(("sm", Input::Float(m)));
                set.push(("sa", Input::Float(a)));
                s.solve_other(kk, &set)
            }
        };
        if let Some(nf) = ans.and_then(|b| k::num_of(&b, "n_fatigue")) {
            if nf > 0.0 {
                out.push((nf * m / 1e6, nf * a / 1e6));
            }
        }
    }
    out
}

fn fatigue_chart(s: &Solved, kind: &str, title: &str, series: Vec<(String, Vec<(f64, f64)>)>, crit: &str,
                 limits: &str) -> Option<Figure> {
    let (sa, sm, n) = (s.inp("sa")?, s.inp("sm")?, s.out("n_fatigue")?);
    if series.iter().all(|x| x.1.len() < 2) || !(n > 0.0) {
        return None;
    }
    let (pm, pa) = (sm / 1e6, sa / 1e6);
    let mut all: Vec<pl::Curve> = series.into_iter().enumerate().filter(|(_, x)| x.1.len() >= 2)
        .map(|(i, (name, pts))| pl::curve(&name, pts, i)).collect();
    let mut ll = pl::curve("load line", vec![(0.0, 0.0), (pm * n, pa * n)], 3);
    ll.dashed = true;
    all.push(ll);
    let body = pl::chart(&all, "σm, mean stress (MPa)", "σa, alternating stress (MPa)",
                         Some(((pm, pa), format!("n = {}", num(n)))), (false, false), true)?;
    Some(s.chart(kind, title,
                 &format!("{crit} fatigue diagram for {limits}: each line is traced by the model's safety factor along 31 \
                           proportional load lines (each point where n = 1). Marked: the operating point σm = {}, σa = {}, \
                           its load line to the {crit} line, and the returned safety factor n = {}.", mpa(pm), mpa(pa),
                          num(n)), &body))
}

pub fn goodman_all(s: &Solved) -> Option<Figure> {
    let (se, sut) = (s.inp("Se")?, s.inp("Sut")?);
    let mut series = vec![("Goodman".to_string(), fatigue_trace(s, None, &[], 31)),
                          ("Gerber".to_string(), fatigue_trace(s, Some("gerber_fatigue_safety_factor"), &[("Se", se), ("Sut", sut)], 31))];
    if let Some(sy) = s.inp("Sy") {
        series.push(("Soderberg".to_string(), fatigue_trace(s, Some("soderberg_fatigue_safety_factor"), &[("Se", se), ("Sy", sy)], 31)));
    }
    let lim = format!("Se = {}, Sut = {}{}", mpa(se / 1e6), mpa(sut / 1e6),
                      s.inp("Sy").map(|v| format!(", Sy = {}", mpa(v / 1e6))).unwrap_or_default());
    fatigue_chart(s, "fatigue", "Fatigue diagram: Goodman, Gerber and Soderberg envelopes", series, "Goodman", &lim)
}

pub fn goodman_one(s: &Solved) -> Option<Figure> {
    let (se, sut) = (s.inp("Se")?, s.inp("Sut")?);
    let series = vec![("Goodman line".to_string(), fatigue_trace(s, None, &[], 31))];
    fatigue_chart(s, "fatigue-2", "Goodman diagram with the load line", series, "Goodman",
                  &format!("Se = {}, Sut = {}", mpa(se / 1e6), mpa(sut / 1e6)))
}

pub fn single_fatigue(s: &Solved, name: &str, other: &str) -> Option<Figure> {
    let se = s.inp("Se")?;
    let o = s.inp(other)?;
    let series = vec![(format!("{name} line"), fatigue_trace(s, None, &[], 31))];
    fatigue_chart(s, "fatigue", &format!("{name} diagram with the load line"), series, name,
                  &format!("Se = {}, {other} = {}", mpa(se / 1e6), mpa(o / 1e6)))
}

// ------------------------------------------------------------------------------------------------ failure envelopes
fn envelope_plot(s: &Solved, title: &str, caption: &str, curves: Vec<(String, Vec<(f64, f64)>)>, state: (f64, f64),
                 n: f64, label: &str, xlab: &str, ylab: &str) -> Option<Figure> {
    let mut xs: Vec<f64> = curves.iter().flat_map(|c| c.1.iter().map(|p| p.0)).collect();
    let mut ys: Vec<f64> = curves.iter().flat_map(|c| c.1.iter().map(|p| p.1)).collect();
    xs.extend([state.0, 0.0]);
    ys.extend([state.1, 0.0]);
    let mut c = Canvas::new(420.0);
    let items: Vec<(String, &str)> = curves.iter().enumerate().map(|(i, x)| (x.0.clone(), pl::line_class(i))).collect();
    pl::legend(&mut c, 96.0, 22.0, &items);
    let p = Plot::equal((96.0, 56.0, 504.0, 368.0), &xs, &ys, "e")?;
    p.frame(&mut c, xlab, ylab);
    p.zeros(&mut c, true, true);
    p.open(&mut c);
    for (i, cv) in curves.iter().enumerate() {
        let mut v = cv.1.clone();
        if let Some(f) = v.first().copied() {
            v.push(f);
        }
        p.path(&mut c, &v, pl::line_class(i));
    }
    p.dashed(&mut c, &[(0.0, 0.0), (state.0 * n, state.1 * n)], "axp-c4");
    p.close(&mut c);
    p.mark(&mut c, state, label);
    Some(s.figure("envelope", title, caption, c.body(), c.h))
}

pub fn yield_envelope(s: &Solved) -> Option<Figure> {
    let sy = s.inp("Sy")?;
    for k_ in ["sz", "tyz", "tzx"] {
        if s.inp(k_).map(|v| v != 0.0).unwrap_or(false) {
            return None;                                   // a 3-D state has no place in the plane of two principal stresses
        }
    }
    let (sx, syy, txy) = (s.inp("sx")?, s.inp("sy").unwrap_or(0.0), s.inp("txy").unwrap_or(0.0));
    let (nvm, ntr) = (s.out("n_von_mises")?, s.out("n_tresca")?);
    let pr = s.solve_other("principal_stresses_2d", &[("sx", Input::Float(sx)), ("sy", Input::Float(syy)),
                                                       ("txy", Input::Float(txy))])?;
    let (s1, s2) = (k::num_of(&pr, "s1_MPa")?, k::num_of(&pr, "s2_MPa")?);
    let s0 = sy / 2.0;
    let mut vm = vec![];
    let mut tr = vec![];
    for i in 0..72 {
        let th = (i as f64 * 5.0).to_radians();
        let (a, b) = (s0 * th.cos(), s0 * th.sin());
        if let Some(ans) = s.with(&[("sx", a), ("sy", b), ("txy", 0.0)]) {
            if let Some(n) = k::num_of(&ans, "n_von_mises").filter(|n| *n > 0.0) {
                vm.push((n * a / 1e6, n * b / 1e6));
            }
            if let Some(n) = k::num_of(&ans, "n_tresca").filter(|n| *n > 0.0) {
                tr.push((n * a / 1e6, n * b / 1e6));
            }
        }
    }
    if vm.len() < 8 || tr.len() < 8 {
        return None;
    }
    envelope_plot(s, "Yield envelopes: von Mises and Tresca",
                  &format!("Yield envelopes for Sy = {} in the principal-stress plane (plane stress): each point is where \
                            the model's safety factor along a proportional load line equals 1 (72 directions). Marked: the \
                            stress state at its principal stresses σ1 = {}, σ2 = {} (from the model's principal-stress \
                            capability), with safety factors n = {} (von Mises) and {} (Tresca).", mpa(sy / 1e6), num(s1),
                           mpa(s2), num(nvm), num(ntr)),
                  vec![("von Mises".into(), vm), ("Tresca".into(), tr)], (s1, s2), nvm,
                  &format!("n = {} / {}", num(nvm), num(ntr)), "σ1 (MPa)", "σ2 (MPa)")
}

pub fn brittle_envelope(s: &Solved) -> Option<Figure> {
    let (sa, sb, sut, suc) = (s.inp("sA")?, s.inp("sB")?, s.inp("Sut")?, s.inp("Suc")?);
    let ncm = s.out("n_coulomb_mohr")?;
    let s0 = sut / 2.0;
    let outs = ["n_coulomb_mohr", "n_modified_mohr", "n_max_normal"];
    let mut cs: Vec<Vec<(f64, f64)>> = vec![vec![], vec![], vec![]];
    for i in 0..72 {
        let th = (i as f64 * 5.0).to_radians();
        let (a, b) = (s0 * th.cos(), s0 * th.sin());
        if let Some(ans) = s.with(&[("sA", a), ("sB", b)]) {
            for (j, o) in outs.iter().enumerate() {
                if let Some(n) = k::num_of(&ans, o).filter(|n| *n > 0.0 && n.is_finite()) {
                    cs[j].push((n * a / 1e6, n * b / 1e6));
                }
            }
        }
    }
    if cs.iter().any(|c| c.len() < 8) {
        return None;
    }
    let names = ["Coulomb-Mohr", "modified Mohr", "maximum normal stress"];
    let curves: Vec<(String, Vec<(f64, f64)>)> = names.iter().map(|n| n.to_string()).zip(cs).collect();
    envelope_plot(s, "Brittle failure envelopes and the load line",
                  &format!("Failure envelopes of a brittle material (Sut = {}, Suc = {}) in the σA, σB plane: each envelope \
                            point is the stress state at which the model's safety factor along a proportional load line \
                            equals 1 (72 directions). Marked: the operating point σA = {}, σB = {}, its load line, and the \
                            returned Coulomb-Mohr safety factor n = {}.", mpa(sut / 1e6), mpa(suc / 1e6), mpa(sa / 1e6),
                           mpa(sb / 1e6), num(ncm)),
                  curves, (sa / 1e6, sb / 1e6), ncm, &format!("n = {} (Coulomb-Mohr)", num(ncm)), "σA (MPa)", "σB (MPa)")
}

// ------------------------------------------------------------------------------------------------ Mohr's circles
/// The circle traced by the model's stress transformation: (σx', τx'y') at planes 0 to 180°.
fn transform_circle(s: &Solved, own: bool, sx: f64, sy: f64, txy: f64) -> Vec<(f64, f64)> {
    let mut v = vec![];
    for i in 0..=36 {
        let th = (i as f64 * 5.0).to_radians();
        let a = if own { s.with(&[("theta", th)]) } else {
            s.solve_other("plane_stress_transformation", &[("sx", Input::Float(sx)), ("sy", Input::Float(sy)),
                                                            ("txy", Input::Float(txy)), ("theta", Input::Float(th))])
        };
        if let Some(a) = a {
            if let (Some(x), Some(t)) = (k::num_of(&a, "sx_t_MPa"), k::num_of(&a, "txy_t_MPa")) {
                v.push((x, t));
            }
        }
    }
    v
}

struct MohrMark {
    p: (f64, f64),
    class: &'static str,
    label: String,
}

fn mohr_plot(s: &Solved, kind: &str, title: &str, caption: &str, circles: &[(Vec<(f64, f64)>, usize)],
             diam: &[((f64, f64), (f64, f64), bool)], marks: &[MohrMark], legend: &[(String, &str)], xlab: &str,
             ylab: &str) -> Option<Figure> {
    let mut xs = vec![0.0];
    let mut ys = vec![0.0];
    for (c, _) in circles {
        xs.extend(c.iter().map(|p| p.0));
        ys.extend(c.iter().map(|p| p.1));
    }
    // room above the circle for the labels of its top point
    if let Some((lo, hi)) = pl::range(&ys) {
        ys.push(hi + 0.12 * (hi - lo));
        ys.push(lo - 0.12 * (hi - lo));
    }
    let mut c = Canvas::new(420.0);
    let top = if legend.is_empty() { 30.0 } else { pl::legend(&mut c, 96.0, 22.0, legend) + 16.0 };
    let p = Plot::equal((96.0, top.max(40.0), 504.0, 368.0), &xs, &ys, "o")?;
    p.frame(&mut c, xlab, ylab);
    p.zeros(&mut c, true, true);
    p.open(&mut c);
    for (cv, k_) in circles {
        p.path(&mut c, cv, pl::line_class(*k_));
    }
    for (a, b, dashed) in diam {
        if *dashed {
            p.dashed(&mut c, &[*a, *b], "axp-c4");
        } else {
            p.path(&mut c, &[*a, *b], "axp-c2");
        }
    }
    p.close(&mut c);
    for m in marks {
        p.dot(&mut c, m.p, m.class, 4.5);
        let (x, y) = p.pt(m.p);
        let right = x < (p.x0 + p.x1) / 2.0 + 60.0;
        let (tx, anchor) = if right { (x + 8.0, "start") } else { (x - 8.0, "end") };
        c.value(tx, y - 8.0, anchor, &m.label);
    }
    Some(s.figure(kind, title, caption, c.body(), c.h))
}

/// A circle through its centre and radius (drawn from the principal values the model returned).
fn circle_pts(cx: f64, r: f64) -> Vec<(f64, f64)> {
    (0..=96).map(|i| {
        let t = std::f64::consts::TAU * i as f64 / 96.0;
        (cx + r * t.cos(), r * t.sin())
    }).collect()
}

pub fn plane_transform_mohr(s: &Solved) -> Option<Figure> {
    let (sx, sy, txy, th) = (s.inp("sx")?, s.inp("sy")?, s.inp("txy")?, s.inp("theta")?);
    let (sxt, syt, txyt) = (s.out("sx_t_MPa")?, s.out("sy_t_MPa")?, s.out("txy_t_MPa")?);
    let circ = transform_circle(s, true, sx, sy, txy);
    if circ.len() < 20 {
        return None;
    }
    let pr = s.solve_other("principal_stresses_2d", &[("sx", Input::Float(sx)), ("sy", Input::Float(sy)),
                                                       ("txy", Input::Float(txy))])?;
    let (s1, s2, tm) = (k::num_of(&pr, "s1_MPa")?, k::num_of(&pr, "s2_MPa")?, k::num_of(&pr, "tau_max_inplane_MPa")?);
    let (mx, my, mt) = (sx / 1e6, sy / 1e6, txy / 1e6);
    let marks = vec![MohrMark { p: (s1, 0.0), class: "axp-d", label: format!("σ1 = {}", num(s1)) },
                     MohrMark { p: (s2, 0.0), class: "axp-d", label: format!("σ2 = {}", num(s2)) },
                     MohrMark { p: ((s1 + s2) / 2.0, tm), class: "axp-d4", label: format!("τmax = {}", num(tm)) },
                     MohrMark { p: (mx, mt), class: "axp-d3", label: "x".into() },
                     MohrMark { p: (my, -mt), class: "axp-d3", label: "y".into() },
                     MohrMark { p: (sxt, txyt), class: "axp-d2", label: format!("x′ ({}, {})", num(sxt), num(txyt)) },
                     MohrMark { p: (syt, -txyt), class: "axp-d2", label: String::new() }];
    mohr_plot(s, "mohr", "Stress transformation on Mohr's circle",
              &format!("Mohr's circle of the plane stress state σx = {}, σy = {}, τxy = {} (traced by the model's stress \
                        transformation at 37 planes; principal stresses from the model: σ1 = {}, σ2 = {}). The dashed \
                        diameter joins the x and y faces; the solid diameter is the element rotated by θ = {}°, whose \
                        stresses the model returned: σx′ = {}, σy′ = {}, τx′y′ = {}.", mpa(mx), mpa(my), mpa(mt), mpa(s1),
                       mpa(s2), num(th.to_degrees()), mpa(sxt), mpa(syt), mpa(txyt)),
              &[(circ, 0)], &[((mx, mt), (my, -mt), true), ((sxt, txyt), (syt, -txyt), false)], &marks,
              &[("Mohr's circle".into(), "axp-c"), ("x and y faces".into(), "axp-c4"), ("rotated element".into(), "axp-c2")],
              "σ, normal stress (MPa)", "τ, shear stress (MPa)")
}

pub fn principal2d_mohr(s: &Solved) -> Option<Figure> {
    let (sx, sy, txy) = (s.inp("sx")?, s.inp("sy")?, s.inp("txy")?);
    let (s1, s2, tm, thp) = (s.out("s1_MPa")?, s.out("s2_MPa")?, s.out("tau_max_inplane_MPa")?, s.out("theta_p_deg")?);
    let circ = transform_circle(s, false, sx, sy, txy);
    if circ.len() < 20 {
        return None;
    }
    let (mx, my, mt) = (sx / 1e6, sy / 1e6, txy / 1e6);
    let marks = vec![MohrMark { p: (s1, 0.0), class: "axp-d", label: format!("σ1 = {}", num(s1)) },
                     MohrMark { p: (s2, 0.0), class: "axp-d", label: format!("σ2 = {}", num(s2)) },
                     MohrMark { p: ((s1 + s2) / 2.0, tm), class: "axp-d4", label: format!("τmax = {}", num(tm)) },
                     MohrMark { p: (mx, mt), class: "axp-d3", label: "x".into() },
                     MohrMark { p: (my, -mt), class: "axp-d3", label: "y".into() }];
    mohr_plot(s, "mohr", "Mohr's circle of a plane stress state",
              &format!("Mohr's circle for σx = {}, σy = {}, τxy = {}: the normal and shear stress on planes rotated by 0 to \
                        180° (37 states computed by the model's stress transformation) trace the circle; the principal \
                        stresses it returned are σ1 = {} and σ2 = {} at θp = {}°; marked: the centre's τmax = {}.", mpa(mx),
                       mpa(my), mpa(mt), mpa(s1), mpa(s2), num(thp), mpa(tm)),
              &[(circ, 0)], &[((mx, mt), (my, -mt), true)], &marks,
              &[("Mohr's circle".into(), "axp-c"), ("x and y faces".into(), "axp-c4")],
              "σ, normal stress (MPa)", "τ, shear stress (MPa)")
}

pub fn principal3d_mohr(s: &Solved) -> Option<Figure> {
    let (s1, s2, s3, tm) = (s.out("s1_MPa")?, s.out("s2_MPa")?, s.out("s3_MPa")?, s.out("tau_max_MPa")?);
    let circles = vec![(circle_pts((s1 + s3) / 2.0, (s1 - s3).abs() / 2.0), 1usize),
                       (circle_pts((s1 + s2) / 2.0, (s1 - s2).abs() / 2.0), 0usize),
                       (circle_pts((s2 + s3) / 2.0, (s2 - s3).abs() / 2.0), 2usize)];
    // equal principal stresses share one point and one label
    let span = (s1 - s3).abs().max(1e-9);
    let mut groups: Vec<(f64, Vec<&str>)> = vec![];
    for (v, n) in [(s1, "σ1"), (s2, "σ2"), (s3, "σ3")] {
        match groups.iter_mut().find(|g| (g.0 - v).abs() <= 1e-9 * span) {
            Some(g) => g.1.push(n),
            None => groups.push((v, vec![n])),
        }
    }
    let mut marks: Vec<MohrMark> = groups.iter()
        .map(|(v, ns)| MohrMark { p: (*v, 0.0), class: "axp-d", label: format!("{} = {}", ns.join(" = "), num(*v)) }).collect();
    marks.push(MohrMark { p: ((s1 + s3) / 2.0, tm), class: "axp-d4", label: format!("τmax = {}", num(tm)) });
    let vm = s.out("von_mises_MPa");
    mohr_plot(s, "mohr", "Mohr's circles of a 3-D stress state",
              &format!("The three Mohr's circles of the 3-D stress state, drawn through the returned principal stresses \
                        σ1 = {}, σ2 = {}, σ3 = {} (circles of zero radius are points). Marked: τmax = {}{}.", num(s1), num(s2),
                       mpa(s3), mpa(tm), vm.map(|v| format!("; also returned: von Mises {}", mpa(v))).unwrap_or_default()),
              &circles, &[], &marks, &[], "σ, normal stress (MPa)", "τ, shear stress (MPa)")
}

pub fn rosette_mohr(s: &Solved) -> Option<Figure> {
    let (ex, ey, g, e1, e2, thp) = (s.out("ex")?, s.out("ey")?, s.out("gxy")?, s.out("e1")?, s.out("e2")?, s.out("theta_p_deg")?);
    let (cx, r) = ((e1 + e2) / 2.0, (e1 - e2).abs() / 2.0);
    // the x and y points lie on the circle through the principal strains
    let rx = ((ex - cx).powi(2) + (g / 2.0).powi(2)).sqrt();
    if (rx - r).abs() > 1e-6 * r.max(1e-9) + 1e-9 {
        return None;
    }
    let marks = vec![MohrMark { p: (e1, 0.0), class: "axp-d", label: format!("ε1 = {}", num(e1)) },
                     MohrMark { p: (e2, 0.0), class: "axp-d", label: format!("ε2 = {}", num(e2)) },
                     MohrMark { p: (ex, g / 2.0), class: "axp-d3", label: "x".into() },
                     MohrMark { p: (ey, -g / 2.0), class: "axp-d3", label: "y".into() }];
    mohr_plot(s, "mohr", "Strain rosette on Mohr's circle of strain",
              &format!("Mohr's circle of strain from the rosette readings: the returned εx = {}, εy = {}, γxy = {} µε give \
                        the x and y points (ε, ±γ/2); the circle passes through the returned principal strains ε1 = {} and \
                        ε2 = {} µε at θp = {}°.", num(ex), num(ey), num(g), num(e1), num(e2), num(thp)),
              &[(circle_pts(cx, r), 0)], &[((ex, g / 2.0), (ey, -g / 2.0), true)], &marks,
              &[("Mohr's circle of strain".into(), "axp-c"), ("x and y directions".into(), "axp-c4")],
              "ε, normal strain (µε)", "γ/2, half shear strain (µε)")
}

// ------------------------------------------------------------------------------------------------ stress elements
/// A square element centred at c, turned by `deg` (counter-clockwise), with normal stresses on its faces (arrows outward
/// for tension, lengths to one scale) and their values.
fn element(c: &mut Canvas, ctr: (f64, f64), deg: f64, half: f64, sa: f64, sb: f64, smax: f64) {
    let t = deg.to_radians();
    let u = (t.cos(), -t.sin());
    let v = (-t.sin(), -t.cos());
    let corner = |a: f64, b: f64| (ctr.0 + (u.0 * a + v.0 * b) * half, ctr.1 + (u.1 * a + v.1 * b) * half);
    let pts = [corner(-1.0, -1.0), corner(1.0, -1.0), corner(1.0, 1.0), corner(-1.0, 1.0)];
    c.raw(&format!("<path class=\"axp-ka\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>\
                    <path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", pts[0].0, pts[0].1,
                   pts[1].0, pts[1].1, pts[2].0, pts[2].1, pts[3].0, pts[3].1, pts[0].0, pts[0].1, pts[1].0, pts[1].1,
                   pts[2].0, pts[2].1, pts[3].0, pts[3].1));
    for (dir, val) in [(u, sa), (v, sb)] {
        if val.abs() <= 1e-9 * smax {
            continue;
        }
        let len = 14.0 + 30.0 * val.abs() / smax.max(1e-300);
        for sg in [1.0, -1.0] {
            let face = (ctr.0 + dir.0 * half * sg, ctr.1 + dir.1 * half * sg);
            let far = (face.0 + dir.0 * (len + 4.0) * sg, face.1 + dir.1 * (len + 4.0) * sg);
            let near = (face.0 + dir.0 * 4.0 * sg, face.1 + dir.1 * 4.0 * sg);
            if val > 0.0 {
                pl::arrow(c, near, far, 0);
            } else {
                pl::arrow(c, far, near, 0);
            }
        }
        let tip = (ctr.0 + dir.0 * (half + len + 16.0), ctr.1 + dir.1 * (half + len + 16.0));
        c.value(tip.0, tip.1 + 4.0, "middle", &num(val));
    }
}

pub fn principal2d_element(s: &Solved) -> Option<Figure> {
    let (sx, sy, txy) = (s.inp("sx")? / 1e6, s.inp("sy")? / 1e6, s.inp("txy")? / 1e6);
    let (s1, s2, thp) = (s.out("s1_MPa")?, s.out("s2_MPa")?, s.out("theta_p_deg")?);
    let smax = [sx, sy, s1, s2].iter().map(|v| v.abs()).fold(1e-9, f64::max);
    let mut c = Canvas::new(300.0);
    element(&mut c, (130.0, 150.0), 0.0, 40.0, sx, sy, smax);
    c.name(130.0, 286.0, "middle", "given");
    // the shear stress along the faces (positive: up on the right face, right on the top face)
    if txy.abs() > 1e-9 * smax {
        let sg = txy.signum();
        let (cx, cy, h) = (130.0, 150.0, 46.0);
        pl::arrow(&mut c, (cx + h, cy + 18.0 * sg), (cx + h, cy - 18.0 * sg), 2);
        pl::arrow(&mut c, (cx - h, cy - 18.0 * sg), (cx - h, cy + 18.0 * sg), 2);
        pl::arrow(&mut c, (cx - 18.0 * sg, cy - h), (cx + 18.0 * sg, cy - h), 2);
        pl::arrow(&mut c, (cx + 18.0 * sg, cy + h), (cx - 18.0 * sg, cy + h), 2);
        c.value(cx + 52.0, cy - 52.0, "start", &format!("τ = {}", num(txy)));
    }
    // the principal element: σ1 along the returned angle
    element(&mut c, (390.0, 150.0), thp, 40.0, s1, s2, smax);
    c.name(390.0, 286.0, "middle", "principal");
    pl::arrow(&mut c, (232.0, 262.0), (290.0, 262.0), 3);
    c.value(261.0, 252.0, "middle", &format!("θp = {}°", num(thp)));
    Some(s.figure("element", "Principal stresses on a rotated element",
                  &format!("The stress element (σx = {}, σy = {}, τxy = {}) and the principal element rotated by the \
                            returned θp = {}°, with the returned principal stresses σ1 = {} and σ2 = {} on its faces (arrows \
                            outward = tension, lengths to one scale).", mpa(sx), mpa(sy), mpa(txy), num(thp), mpa(s1),
                           mpa(s2)), c.body(), c.h))
}

pub fn vessel_element(s: &Solved) -> Option<Figure> {
    let (hoop, lon) = (s.out("hoop_MPa")?, s.out("longitudinal_MPa")?);
    let shape = s.txt("shape").unwrap_or_else(|| "cylinder".into());
    let smax = hoop.abs().max(lon.abs()).max(1e-9);
    let mut c = Canvas::new(300.0);
    if shape.starts_with("sphere") {
        pl::circle(&mut c, (200.0, 150.0), 110.0, true);
    } else {
        c.raw("<path class=\"axp-ka\" d=\"M40,60 L330,60 L330,240 L40,240 Z\"/>");
        c.raw("<path class=\"axp-k\" d=\"M40,60 L330,60 L330,240 L40,240\"/>");
        c.raw("<path class=\"axp-k\" d=\"M40,60 Q16,150 40,240\"/>");
    }
    let ctr = (185.0, 150.0);
    pl::rect(&mut c, ctr.0 - 22.0, ctr.1 - 22.0, 44.0, 44.0);
    let lh = 20.0 + 60.0 * hoop.abs() / smax;
    let ll = 20.0 + 60.0 * lon.abs() / smax;
    for sg in [1.0, -1.0] {
        let (a, b) = ((ctr.0, ctr.1 - 22.0 * sg), (ctr.0, ctr.1 - (22.0 + lh) * sg));
        if hoop >= 0.0 { pl::arrow(&mut c, a, b, 0) } else { pl::arrow(&mut c, b, a, 0) }
        let (a, b) = ((ctr.0 + 22.0 * sg, ctr.1), (ctr.0 + (22.0 + ll) * sg, ctr.1));
        if lon >= 0.0 { pl::arrow(&mut c, a, b, 2) } else { pl::arrow(&mut c, b, a, 2) }
    }
    c.value(ctr.0, ctr.1 - 22.0 - lh - 8.0, "middle", &format!("hoop {}", mpa(hoop)));
    c.value(ctr.0 + 22.0 + ll + 8.0, ctr.1 - 10.0, "start", &format!("longitudinal {}", mpa(lon)));
    let (ti, ta) = (s.out("tau_inplane_MPa"), s.out("tau_abs_MPa"));
    if let Some(t) = ti {
        c.value(504.0, 256.0, "end", &format!("τ in-plane = {}", mpa(t)));
    }
    if let Some(t) = ta {
        c.value(504.0, 280.0, "end", &format!("τ absolute = {}", mpa(t)));
    }
    let (p, r, t) = (s.inp("p")?, s.inp("r")?, s.inp("t")?);
    Some(s.figure("element", "Thin-walled vessel: hoop and longitudinal stress",
                  &format!("A thin-walled {} (radius {}, wall {}, pressure {}) and a wall element with the returned stresses \
                            drawn to one scale: hoop {}, longitudinal {}{}.", shape, pl::mm(r), pl::mm(t), mpa(p / 1e6),
                           mpa(hoop), mpa(lon),
                           match (ti, ta) { (Some(a), Some(b)) => format!("; also returned: in-plane shear {}, absolute \
                                             shear {}", mpa(a), mpa(b)), _ => String::new() }), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ profiles
/// Torsional shear stress across a shaft's diameter, linear in the radius between the returned surface values.
pub fn torsion_profile(s: &Solved, hollow: bool) -> Option<Figure> {
    let dd = if hollow { s.inp("D")? } else { s.inp("d")? };
    let di = if hollow { s.inp("d")? } else { 0.0 };
    let (tmax, tin) = (s.out("tau_max_MPa")?, s.out("tau_inner_MPa").unwrap_or(0.0));
    let (ro, ri) = (dd / 2.0 * 1000.0, di / 2.0 * 1000.0);
    if !(ro > ri && ri >= 0.0 && tmax > 0.0) {
        return None;
    }
    let mut c = Canvas::new(300.0);
    pl::legend(&mut c, 96.0, 22.0, &[("shear stress (linear in r)".to_string(), "axp-c")]);
    let p = Plot::new((96.0, 44.0, 504.0, 236.0), (-ro, ro), (-tmax * 1.3, tmax * 1.3), (false, false), (true, false), "t");
    // the material, shaded
    for (a, b) in [(-ro, -ri), (ri, ro)] {
        let (x1, x2) = (p.x(a), p.x(b));
        c.raw(&format!("<rect class=\"axp-s4\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", x1.min(x2), p.y0 + 4.0,
                       (x2 - x1).abs(), p.y1 - p.y0 - 8.0));
    }
    p.frame(&mut c, "r, position across the diameter (mm)", "τ, shear stress (MPa)");
    p.zeros(&mut c, true, true);
    p.open(&mut c);
    let right = vec![(ri, tin), (ro, tmax)];
    let left = vec![(-ri, -tin), (-ro, -tmax)];
    p.area(&mut c, &right, "axp-s");
    p.area(&mut c, &left, "axp-s");
    p.path(&mut c, &right, "axp-c");
    p.path(&mut c, &left, "axp-c");
    p.close(&mut c);
    p.dot(&mut c, (ro, tmax), "axp-d", 4.0);
    p.dot(&mut c, (-ro, -tmax), "axp-d", 4.0);
    let (x, y) = p.pt((ro, tmax));
    c.value(x - 8.0, y - 8.0, "end", &format!("τmax = {}", mpa(tmax)));
    if hollow && ri > 0.0 {
        p.dot(&mut c, (ri, tin), "axp-d", 4.0);
        let (x, y) = p.pt((ri, tin));
        c.value(x - 8.0, y + 4.0, "end", &format!("τ inner = {}", mpa(tin)));
    }
    let tw = s.out("twist_deg");
    let title = if hollow { "Torsional shear stress across a tube" } else { "Torsional shear stress across a solid shaft" };
    Some(s.figure("profile", title,
                  &format!("Shear stress across the {} section (shaded: material; outer diameter {}{}) under the stated \
                            torque, linear in the radius between the returned surface values. Marked: τmax = {}{}{}.",
                           if hollow { "tube" } else { "shaft" }, pl::mm(dd),
                           if hollow { format!(", inner {}", pl::mm(di)) } else { String::new() }, mpa(tmax),
                           if hollow { format!(", τ at the bore = {}", mpa(tin)) } else { String::new() },
                           tw.map(|t| format!("; also returned: twist {}°", num(t))).unwrap_or_default()), c.body(), c.h))
}

pub fn lame_profile(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("a")?, s.inp("b")?);
    if !(b > a && a > 0.0) {
        return None;
    }
    let (sti, sto) = (s.out("st_inner_MPa")?, s.out("st_outer_MPa")?);
    let sz = s.out("sz_closed_MPa");
    let mut hoop = vec![];
    let mut rad = vec![];
    for r in k::span(a, b, 41, false) {
        let ans = s.with(&[("r", r)])?;
        hoop.push((r * 1000.0, k::num_of(&ans, "st_MPa")?));
        rad.push((r * 1000.0, k::num_of(&ans, "sr_MPa")?));
    }
    let mut series = vec![pl::curve("hoop σθ", hoop, 0), pl::curve("radial σr", rad, 2)];
    if let Some(z) = sz {
        let mut ax = pl::curve("axial (closed ends)", vec![(a * 1000.0, z), (b * 1000.0, z)], 1);
        ax.dashed = true;
        series.push(ax);
    }
    let body = pl::chart(&series, "r, radius (mm)", "stress (MPa)", Some(((a * 1000.0, sti), format!("hoop {}", mpa(sti)))),
                         (false, false), true)?;
    Some(s.chart("profile", "Thick cylinder: hoop and radial stress through the wall",
                 &format!("Lamé stresses through the wall of a thick cylinder (inner radius {}, outer {}): hoop and radial \
                           stress at 41 radii, each computed by the model. Marked: hoop stress {} at the bore and {} at the \
                           outside{}.", pl::mm(a), pl::mm(b), mpa(sti), mpa(sto),
                          sz.map(|z| format!(", axial stress with closed ends {}", mpa(z))).unwrap_or_default()), &body))
}

// ------------------------------------------------------------------------------------------------ S-N and rainflow
pub fn sn_line(s: &Solved) -> Option<Figure> {
    let (sut, se, f, sa) = (s.inp("Sut")?, s.inp("Se")?, s.inp("f")?, s.inp("sa")?);
    let n = s.out("N")?;
    let (lo, hi) = (se, f * sut);
    if !(hi > lo && lo > 0.0) {
        return None;
    }
    let mut pts = vec![];
    for v in k::span(lo * 1.0005, hi * 0.9995, 31, true) {
        if let Some(b) = s.with(&[("sa", v)]) {
            if let Some(nn) = k::num_of(&b, "N").filter(|x| *x > 0.0) {
                pts.push((nn, v / 1e6));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut c = Canvas::new(320.0);
    pl::legend(&mut c, 96.0, 22.0, &[("S-N line".to_string(), "axp-c"), ("endurance limit".to_string(), "axp-c4")]);
    let xs: Vec<f64> = pts.iter().map(|p| p.0).chain([n]).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([sa / 1e6, se / 1e6]).collect();
    let (xl, xh) = pl::range(&xs)?;
    let (yl, yh) = pl::range(&ys)?;
    let p = Plot::new((96.0, 40.0, 504.0, 258.0), (xl, xh), (yl, yh), (true, true), (true, true), "s");
    p.frame(&mut c, "N, cycles to failure", "Sf, fatigue strength (MPa)");
    p.open(&mut c);
    p.dashed(&mut c, &[(p.xa.lo, se / 1e6), (p.xa.hi, se / 1e6)], "axp-c4");
    p.path(&mut c, &pts, "axp-c");
    p.close(&mut c);
    p.mark(&mut c, (n, sa / 1e6), &format!("N = {}", num(n)));
    Some(s.figure("sn", "S-N diagram: life at an alternating stress",
                  &format!("S-N line for Sut = {}, Se = {}, f = {}: the life the model returns at {} alternating stresses \
                            between Se and f·Sut (log-log axes). Marked: the alternating stress σa = {} and the returned life \
                            N = {} cycles.", mpa(sut / 1e6), mpa(se / 1e6), num(f), pts.len(), mpa(sa / 1e6), num(n)),
                  c.body(), c.h))
}

pub fn rainflow(s: &Solved) -> Option<Figure> {
    let h = s.vec("history")?;
    let (ranges, counts) = (s.out_list("ranges")?, s.out_list("counts")?);
    if ranges.len() != counts.len() || ranges.is_empty() || h.len() < 2 || h.len() > 400 || ranges.len() > 60 {
        return None;
    }
    let total = s.out("total_cycles");
    let mut c = Canvas::new(470.0);
    pl::legend(&mut c, 96.0, 22.0, &[("load history (input)".to_string(), "axp-c")]);
    let hp: Vec<(f64, f64)> = h.iter().enumerate().map(|(i, v)| ((i + 1) as f64, *v)).collect();
    let (yl, yh) = pl::range(&h)?;
    let p1 = Plot::new((96.0, 40.0, 504.0, 176.0), (1.0, h.len() as f64), (yl, yh), (false, false), (true, true), "h");
    p1.frame(&mut c, "reversal number", "stress");
    p1.zeros(&mut c, false, true);
    p1.open(&mut c);
    p1.path(&mut c, &hp, "axp-c");
    if hp.len() <= 60 {
        for q in &hp {
            p1.dot(&mut c, *q, "axp-d", 3.0);
        }
    }
    p1.close(&mut c);
    let rmax = ranges.iter().copied().fold(0.0f64, f64::max);
    let cmax = counts.iter().copied().fold(0.0f64, f64::max);
    if !(rmax > 0.0 && cmax > 0.0) {
        return None;
    }
    let p2 = Plot::new((96.0, 250.0, 504.0, 410.0), (0.0, rmax * 1.08), (0.0, cmax * 1.15), (false, false), (false, false), "r");
    p2.frame(&mut c, "stress range", "cycles counted");
    let bw = ((p2.x1 - p2.x0) / (ranges.len() as f64 * 2.5)).clamp(4.0, 26.0);
    for (r, n) in ranges.iter().zip(&counts) {
        let (x, y) = p2.pt((*r, *n));
        let y0 = p2.y(0.0);
        c.raw(&format!("<rect class=\"axp-b3\" x=\"{:.1}\" y=\"{:.1}\" width=\"{bw:.1}\" height=\"{:.1}\"/>", x - bw / 2.0, y,
                       (y0 - y).max(0.0)));
        if ranges.len() <= 16 {
            c.value(x, y - 4.0, "middle", &num(*n));
        }
    }
    Some(s.figure("histogram", "Rainflow counting: history and cycle histogram",
                  &format!("Rainflow counting of the load history {} (top). Bottom: the ranges and cycle counts the model \
                            returned (half cycles count 0.5): ranges {} with counts {}{}.",
                           h.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "),
                           ranges.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "),
                           counts.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "),
                           total.map(|t| format!("; total {} cycles", num(t))).unwrap_or_default()), c.body(), c.h))
}
