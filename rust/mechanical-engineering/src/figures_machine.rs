//! Machines, structures and vibration: shafts with disks, oscillators, bolt groups, free-body diagrams, linkages,
//! gears, bars and shafts in tension and torsion, columns, trusses; responses, resonance curves, regime maps and
//! parameter fields from further solves of the model.

use crate::figures_plot::{self as pl, force, moment, mpa, Plot};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::{Input, Val};

/// A number input given as an integer or a number.
fn n_in(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| match s.val(sym)? {
        Val::Int(i) => Some(i as f64),
        Val::Num(x) if x.is_finite() => Some(x),
        _ => None,
    })
}

// ------------------------------------------------------------------------------------------------ shafts with disks
fn shaft_disks(s: &Solved, title: &str, masses: &[f64], pos: &[f64], l: f64, top: &[String], under: &[String],
               caption: &str) -> Option<Figure> {
    if masses.len() != pos.len() || masses.is_empty() || masses.len() > 10 || !(l > 0.0) {
        return None;
    }
    let mut c = Canvas::new(250.0);
    let (x0, x1, y) = (70.0, 450.0, 150.0);
    let px = |x: f64| x0 + (x1 - x0) * x / l;
    c.line((x0 - 14.0, y), (x1 + 14.0, y));
    pl::pin(&mut c, (x0, y + 2.0));
    pl::pin(&mut c, (x1, y + 2.0));
    let mmax = masses.iter().copied().fold(0.0f64, f64::max).max(1e-12);
    for (i, (m, x)) in masses.iter().zip(pos).enumerate() {
        let h = 30.0 + 70.0 * (m / mmax).sqrt();
        let xx = px(*x);
        pl::rect(&mut c, xx - 5.0, y - h / 2.0, 10.0, h);
        c.name(xx + 9.0, y - h / 2.0 + 10.0, "start", &format!("{} kg", num(*m)));
        if let Some(u) = under.get(i) {
            c.value(xx, y + h / 2.0 + 16.0, "middle", u);
        }
    }
    for (i, t) in top.iter().enumerate() {
        c.value(90.0, 28.0 + 19.0 * i as f64, "start", t);
    }
    c.name(260.0, 236.0, "middle", &format!("bearing span L = {}", q(l, "m")));
    Some(s.figure("schematic", title, caption, c.body(), c.h))
}

fn speed_text(w: f64, rpm: f64) -> String {
    format!("ωcr = {} rad/s ({} rpm)", num(w), num(rpm))
}

pub fn dunkerley(s: &Solved) -> Option<Figure> {
    let (ms, ps, l) = (s.vec("masses")?, s.vec("positions")?, s.inp("L")?);
    let (w, n) = (s.out("omega_cr")?, s.out("N_cr_rpm")?);
    shaft_disks(s, "Shaft with disks: Dunkerley critical speed", &ms, &ps, l, &[speed_text(w, n)], &[],
                &format!("Simply supported shaft, bearing span L = {}, carrying disks of {} kg at {} m from the left \
                          bearing, drawn to scale along the span (disk heights grow with mass). Marked: the returned \
                          critical speed {}.", q(l, "m"), join(&ms), join(&ps), speed_text(w, n)))
}

pub fn rayleigh(s: &Solved) -> Option<Figure> {
    let (ms, ps, l) = (s.vec("masses")?, s.vec("positions")?, s.inp("L")?);
    let (w, n) = (s.out("omega_cr")?, s.out("N_cr_rpm")?);
    let d = s.out_list("deflections_mm").unwrap_or_default();
    let under: Vec<String> = d.iter().map(|v| format!("{} mm", num(*v))).collect();
    shaft_disks(s, "Shaft with disks: Rayleigh critical speed", &ms, &ps, l,
                &[speed_text(w, n), "static deflections under the disks:".to_string()], &under,
                &format!("Simply supported shaft, bearing span L = {}, carrying disks of {} kg at {} m from the left \
                          bearing, drawn to scale (disk heights grow with mass). Marked: the returned critical speed {} and \
                          the static deflections under the disks, {} mm.", q(l, "m"), join(&ms), join(&ps), speed_text(w, n),
                         join(&d)))
}

pub fn whirling(s: &Solved) -> Option<Figure> {
    let (ms, ps, l) = (s.vec("masses")?, s.vec("positions")?, s.inp("L")?);
    let (w, n) = (s.out_list("omega")?, s.out_list("N_rpm")?);
    let top: Vec<String> = w.iter().zip(&n).take(4).enumerate()
        .map(|(i, (a, b))| format!("ω{} = {} rad/s ({} rpm)", i + 1, num(*a), num(*b))).collect();
    shaft_disks(s, "Shaft with disks: its whirling speeds", &ms, &ps, l, &top, &[],
                &format!("Simply supported shaft, bearing span L = {}, carrying disks of {} kg at {} m from the left \
                          bearing, drawn to scale (disk heights grow with mass). Marked: the returned whirling speeds {} \
                          rad/s ({} rpm).", q(l, "m"), join(&ms), join(&ps), join(&w), join(&n)))
}

pub fn single_disk(s: &Solved) -> Option<Figure> {
    let (m, l, a) = (s.inp("m")?, s.inp("L")?, s.inp("a")?);
    let (kk, w, n, d) = (s.out("k")?, s.out("omega_cr")?, s.out("N_cr_rpm")?, s.out("static_deflection_mm")?);
    shaft_disks(s, "Shaft with one disk: critical speed", &[m], &[a], l,
                &[speed_text(w, n), format!("k = {} N/m at the disk", pl::nn(kk))], &[format!("δst = {} mm", num(d))],
                &format!("Simply supported shaft, bearing span L = {}, carrying a disk of {} at {} from the left bearing, \
                          drawn to scale. Marked: the returned stiffness k = {} N/m at the disk, {}, static deflection {} \
                          mm.", q(l, "m"), q(m, "kg"), q(a, "m"), pl::nn(kk), speed_text(w, n), num(d)))
}

fn join(v: &[f64]) -> String {
    v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")
}

// ------------------------------------------------------------------------------------------------ oscillators
pub fn sdof_schematic(s: &Solved) -> Option<Figure> {
    let (m, kk) = (s.inp("m")?, s.inp("k")?);
    let (wn, fnn, t, d) = (s.out("omega_n")?, s.out("f_n")?, s.out("period")?, s.out("static_deflection_mm")?);
    let mut c = Canvas::new(300.0);
    pl::ceiling(&mut c, 180.0, 340.0, 40.0);
    pl::spring(&mut c, (260.0, 40.0), (260.0, 170.0));
    pl::rect(&mut c, 216.0, 170.0, 88.0, 74.0);
    c.bold(260.0, 212.0, "middle", &format!("m = {}", q(m, "kg")));
    c.name(276.0, 108.0, "start", &format!("k = {} N/m", pl::nn(kk)));
    for (i, t_) in [format!("ωn = {} rad/s", num(wn)), format!("fn = {} Hz", num(fnn)), format!("T = {} s", num(t)),
                    format!("δst = {} mm", num(d))].iter().enumerate() {
        c.value(320.0, 186.0 + 19.0 * i as f64, "start", t_);
    }
    Some(s.figure("schematic", "Spring-mass oscillator and its natural frequency",
                  &format!("A mass m = {} on a spring k = {} N/m. Marked: the returned natural frequency ωn = {} rad/s \
                            (fn = {} Hz), period {} s and static deflection {} mm under its own weight.", q(m, "kg"), pl::nn(kk),
                           num(wn), num(fnn), num(t), num(d)), c.body(), c.h))
}

pub fn mdof_chain(s: &Solved) -> Option<Figure> {
    let (ms, ks) = (s.vec("masses")?, s.vec("springs")?);
    let ends = s.txt("ends").unwrap_or_else(|| "fixed-free".into());
    let (fs, mode) = (s.out_list("f_Hz")?, s.out_list("mode1")?);
    let n = ms.len();
    let both = ends == "fixed-fixed";
    if n == 0 || n > 8 || ks.len() != if both { n + 1 } else { n } || mode.len() != n {
        return None;
    }
    let mut c = Canvas::new(280.0);
    let y = 170.0;
    let (x0, x1) = (40.0, if both { 480.0 } else { 470.0 });
    pl::ground(&mut c, x0, x1, y + 36.0);
    pl::wall(&mut c, x0, y - 50.0, y + 36.0, true);
    if both {
        pl::wall(&mut c, x1, y - 50.0, y + 36.0, false);
    }
    let slots = if both { 2 * n + 1 } else { 2 * n };
    let seg = (x1 - x0) / slots as f64;
    let bw = seg.min(56.0);
    let amax = mode.iter().map(|v| v.abs()).fold(1e-12, f64::max);
    let mut x = x0;
    for i in 0..n {
        let xs = x + seg;
        pl::spring(&mut c, (x, y), (xs + (seg - bw) / 2.0, y));
        c.name((x + xs) / 2.0 + 6.0, y - 24.0, "middle", &num(ks[i]));
        let bx = xs + (seg - bw) / 2.0;
        pl::rect(&mut c, bx, y - 32.0, bw, 64.0);
        c.name(bx + bw / 2.0, y + 5.0, "middle", &format!("{} kg", num(ms[i])));
        let len = 50.0 * mode[i] / amax;
        let mx = bx + bw / 2.0;
        if len.abs() > 2.0 {
            pl::arrow(&mut c, (mx - len / 2.0, y - 52.0), (mx + len / 2.0, y - 52.0), 0);
        }
        c.value(mx, y - 62.0, "middle", &num(mode[i]));
        x = bx + bw;
    }
    if both {
        pl::spring(&mut c, (x, y), (x1, y));
        c.name((x + x1) / 2.0, y - 24.0, "middle", &num(ks[n]));
    }
    c.name(260.0, 40.0, "middle", "first mode shape (arrows)");
    c.value(40.0, 262.0, "start", &format!("natural frequencies: {} Hz",
                                           fs.iter().take(5).map(|v| num(*v)).collect::<Vec<_>>().join(", ")));
    Some(s.figure("schematic", "Spring-mass chain: frequencies and first mode",
                  &format!("Spring-mass chain ({ends}): masses {} kg, springs {} N/m (the wall spring first). Arrows: the \
                            first mode shape the model returned ({}, largest entry 1). Marked: the returned natural \
                            frequencies {} Hz.", join(&ms), join(&ks), join(&mode), join(&fs)), c.body(), c.h))
}

fn oscillator_drawing(c: &mut Canvas, m: f64, kk: f64, cc: f64, y: f64) {
    pl::wall(c, 120.0, y - 46.0, y + 46.0, true);
    pl::ground(c, 120.0, 460.0, y + 56.0);
    pl::spring(c, (120.0, y - 22.0), (330.0, y - 22.0));
    c.line((120.0, y + 22.0), (210.0, y + 22.0));
    c.raw(&format!("<path class=\"axp-k\" d=\"M210,{:.1} L250,{:.1} M210,{:.1} L250,{:.1} M228,{:.1} L228,{:.1}\"/>",
                   y + 12.0, y + 12.0, y + 32.0, y + 32.0, y + 14.0, y + 30.0));
    c.line((210.0, y + 12.0), (210.0, y + 32.0));
    c.line((228.0, y + 22.0), (330.0, y + 22.0));
    pl::rect(c, 330.0, y - 40.0, 90.0, 80.0);
    c.bold(375.0, y + 5.0, "middle", &format!("m = {}", q(m, "kg")));
    c.name(225.0, y - 36.0, "middle", &format!("k = {} N/m", pl::nn(kk)));
    c.name(170.0, y + 50.0, "middle", &format!("c = {} N·s/m", num(cc)));
    pl::arrow(c, (430.0, y - 50.0), (470.0, y - 50.0), 3);
    c.name(476.0, y - 46.0, "start", "x");
}

/// The time to show a free response over: a few periods, or a few decay times when overdamped.
fn show_time(s: &Solved, m: f64, kk: f64, cc: f64) -> Option<(f64, f64, f64)> {
    let d = s.solve_other("damped_sdof_parameters", &[("m", Input::Float(m)), ("c", Input::Float(cc)), ("k", Input::Float(kk))])?;
    let (wn, z) = (k::num_of(&d, "omega_n")?, k::num_of(&d, "zeta")?);
    let t = if z < 1.0 {
        let per = std::f64::consts::TAU / (wn * (1.0 - z * z).sqrt().max(1e-9));
        if z > 0.0 { (5.0 * per).min(4.6 / (z * wn)).max(1.5 * per) } else { 5.0 * per }
    } else {
        5.0 / (wn * (z - (z * z - 1.0).sqrt())).max(1e-300)
    };
    Some((t, wn, z))
}

fn response_figure(s: &Solved, title: &str, caption: &str, m: f64, kk: f64, cc: f64, pts: Vec<(f64, f64)>,
                   mark: Option<((f64, f64), String)>, legend: &str, note: &str) -> Option<Figure> {
    if pts.len() < 10 {
        return None;
    }
    let mut c = Canvas::new(420.0);
    oscillator_drawing(&mut c, m, kk, cc, 70.0);
    pl::legend(&mut c, 96.0, 168.0, &[(legend.to_string(), "axp-c")]);
    let (yl, yh) = pl::range(&pts.iter().map(|p| p.1).chain(mark.as_ref().map(|m| m.0 .1)).collect::<Vec<_>>())?;
    let tmax = pts.last()?.0;
    let p = Plot::new((96.0, 184.0, 504.0, 360.0), (0.0, tmax), (yl.min(0.0), yh.max(0.0)), (false, false), (false, true), "x");
    p.frame(&mut c, "t, time (s)", "x (mm)");
    p.zeros(&mut c, false, true);
    p.open(&mut c);
    p.path(&mut c, &pts, "axp-c");
    p.close(&mut c);
    if let Some((pt, lab)) = mark {
        p.mark(&mut c, pt, &lab);
    }
    if !note.is_empty() {
        c.value(500.0, 200.0, "end", note);
    }
    Some(s.figure("response", title, caption, c.body(), c.h))
}

pub fn damped_response(s: &Solved) -> Option<Figure> {
    let (m, cc, kk) = (s.inp("m")?, s.inp("c")?, s.inp("k")?);
    let (wn, z) = (s.out("omega_n")?, s.out("zeta")?);
    let (tend, _, _) = show_time(s, m, kk, cc)?;
    let x0 = 0.01;
    let mut pts = vec![];
    for t in k::span(0.0, tend, 121, false) {
        let a = s.solve_other("sdof_free_response", &[("m", Input::Float(m)), ("c", Input::Float(cc)),
                                                       ("k", Input::Float(kk)), ("x0", Input::Float(x0)),
                                                       ("v0", Input::Float(0.0)), ("t", Input::Float(t))]);
        if let Some(v) = a.and_then(|a| k::num_of(&a, "x_mm")) {
            pts.push((t, v));
        }
    }
    let wd = s.out("omega_d");
    let dl = s.out("log_decrement");
    let note = format!("ζ = {}{}{}", num(z), wd.map(|w| format!(", ωd = {} rad/s", num(w))).unwrap_or_default(),
                       dl.map(|d| format!(", δ = {}", num(d))).unwrap_or_default());
    response_figure(s, "Damped oscillator",
                    &format!("A damped single-degree-of-freedom system m = {}, c = {} N·s/m, k = {} N/m and its free decay \
                              from x0 = 10 mm (an initial displacement chosen for the drawing; 121 instants computed by the \
                              model's free-response capability). Marked: the returned ωn = {} rad/s, {} ({}).", q(m, "kg"),
                             num(cc), pl::nn(kk), num(wn), note, s.out_text("regime").unwrap_or_default()),
                    m, kk, cc, pts, None, "free decay from x0 = 10 mm", &note)
}

pub fn free_response(s: &Solved) -> Option<Figure> {
    let (m, cc, kk, t0) = (s.inp("m")?, s.inp("c")?, s.inp("k")?, s.inp("t")?);
    let xm = s.out("x_mm")?;
    let (tend, _, _) = show_time(s, m, kk, cc)?;
    let tend = tend.max(t0 * 1.1);
    let mut pts = vec![];
    for t in k::span(0.0, tend, 121, false) {
        if let Some(v) = s.with(&[("t", t)]).and_then(|a| k::num_of(&a, "x_mm")) {
            pts.push((t, v));
        }
    }
    let v = s.out("v");
    response_figure(s, "Free vibration of a damped oscillator",
                    &format!("Free vibration of a spring-mass-damper (m = {}, c = {} N·s/m, k = {} N/m) released from the \
                              stated x0 and v0: the displacement at 121 instants up to {}, each computed by the model. \
                              Marked: t = {}, x = {} mm{}.", q(m, "kg"), num(cc), pl::nn(kk), q(tend, "s"), q(t0, "s"), num(xm),
                             v.map(|v| format!(", v = {} m/s", num(v))).unwrap_or_default()),
                    m, kk, cc, pts, Some(((t0, xm), format!("x({}) = {} mm", q(t0, "s"), num(xm)))), "displacement", "")
}

pub fn forced_curves(s: &Solved) -> Option<Figure> {
    let (cc, w, r, mag) = (s.inp("c")?, s.inp("omega")?, s.out("r")?, s.out("magnification")?);
    if !(r > 0.0 && w > 0.0) {
        return None;
    }
    let wn = w / r;
    let ws = k::span(wn * 0.02, (wn * 3.0).max(w * 1.2), 60, false);
    let base = if cc > 0.0 { cc } else { 1.0 };
    let mut series = vec![];
    for (i, f) in [1.0, 2.5, 5.0, 10.0].iter().enumerate() {
        let cv = base * f;
        let pts: Vec<(f64, f64)> = ws.iter().filter_map(|x| {
            let a = s.with(&[("c", cv), ("omega", *x)])?;
            Some((*x, k::num_of(&a, "magnification")?))
        }).collect();
        if pts.len() >= 2 {
            series.push(pl::curve(&format!("c = {} N·s/m", num(cv)), pts, i));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = pl::chart(&series, "ω, forcing frequency (rad/s)", "X/Xst, magnification", Some(((w, mag), num(mag))),
                         (false, false), true)?;
    Some(s.chart("response", "Forced vibration: resonance curves for four dampings",
                 &format!("Steady-state magnification of a forced spring-mass-damper (natural frequency {} rad/s from the \
                           returned frequency ratio) for four damping coefficients, 60 forcing frequencies each computed by \
                           the model; marked: ω = {} rad/s, c = {} N·s/m, magnification = {}.", num(wn), num(w), num(cc),
                          num(mag)), &body))
}

/// The regime (underdamped, critically damped, overdamped) over damping and mass, k fixed: one solve per cell.
pub fn damped_decision(s: &Solved) -> Option<Figure> {
    let (m, cc) = (s.inp("m")?, s.inp("c")?);
    if !(m > 0.0 && cc > 0.0) {
        return None;
    }
    let xs = k::span(cc / 10.0, cc * 10.0, 17, true);
    let ys = k::span(m / 10.0, m * 10.0, 13, true);
    let mut names: Vec<String> = vec![];
    let mut cells = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            if let Some(a) = s.with(&[("c", *x), ("m", *y)]) {
                if let Some(Some(t)) = a.values.iter().find(|v| v.0 == "regime").map(|v| v.1.as_str().map(String::from)) {
                    if !names.contains(&t) {
                        names.push(t.clone());
                    }
                    cells.push((i, j, t));
                }
            }
        }
    }
    if cells.len() < 10 {
        return None;
    }
    let mut c = Canvas::new(360.0);
    let p = Plot::new((96.0, 50.0, 504.0, 290.0), (xs[0], xs[xs.len() - 1]), (ys[0], ys[ys.len() - 1]), (true, true),
                      (false, false), "d");
    let edge = |v: &[f64], i: usize| -> (f64, f64) {
        let l = if i == 0 { v[0] } else { (v[i - 1] * v[i]).sqrt() };
        let r = if i + 1 >= v.len() { v[i] } else { (v[i] * v[i + 1]).sqrt() };
        (l, r)
    };
    let classes = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];
    for (i, j, t) in &cells {
        let kx = names.iter().position(|n| n == t).unwrap_or(0);
        let (l, r) = edge(&xs, *i);
        let (b, tp) = edge(&ys, *j);
        let (x1, x2) = (p.x(l), p.x(r));
        let (y1, y2) = (p.y(tp), p.y(b));
        c.raw(&format!("<rect class=\"{}\" x=\"{x1:.1}\" y=\"{y1:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", classes[kx % 4],
                       x2 - x1 + 0.4, y2 - y1 + 0.4));
    }
    p.frame(&mut c, "c, damping coefficient (N·s/m)", "m, mass (kg)");
    p.dot(&mut c, (cc, m), "axp-m", 5.0);
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"9\"/>", p.x(cc), p.y(m)));
    let items: Vec<(String, &str)> = names.iter().enumerate().map(|(i, n)| (n.clone(), classes[i % 4])).collect();
    pl::legend(&mut c, 96.0, 24.0, &items);
    let here = s.out_text("regime").unwrap_or_default();
    Some(s.figure("decision", "Damping regime over damping and mass",
                  &format!("Underdamped, critically damped or overdamped over c from {} to {} N·s/m and m from {} to {} \
                            (logarithmic axes; k as given): each of the {} cells is one run of the model, coloured by the \
                            regime it returned. Marked: the inputs given ({here}).", num(xs[0]), num(xs[xs.len() - 1]),
                           q(ys[0], "kg"), q(ys[ys.len() - 1], "kg"), cells.len()), c.body(), c.h))
}

pub fn two_rotor_mode(s: &Solved) -> Option<Figure> {
    let (j1, j2, l) = (s.inp("J1")?, s.inp("J2")?, s.inp("L")?);
    let (w, f, ratio, node) = (s.out("omega")?, s.out("f")?, s.out("amplitude_ratio")?, s.out("node_from_rotor1")?);
    // the straight mode line crosses zero at the returned node
    let zero = l / (1.0 - ratio);
    if !(l > 0.0) || (zero - node).abs() > 1e-6 * l {
        return None;
    }
    let mut c = Canvas::new(320.0);
    pl::legend(&mut c, 96.0, 22.0, &[("mode shape (straight between the rotors)".to_string(), "axp-c")]);
    let ymax = 1.0f64.max(ratio.abs());
    let p = Plot::new((96.0, 44.0, 504.0, 258.0), (0.0, l), (-ymax, ymax), (false, false), (true, true), "m");
    p.frame(&mut c, "x, position along the shaft (m)", "relative twist amplitude");
    p.zeros(&mut c, false, true);
    for (x, jj) in [(0.0, j1), (l, j2)] {
        let px = p.x(x);
        c.raw(&format!("<rect class=\"axp-s4\" x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"{:.1}\"/>", px - 7.0, p.y0 + 10.0,
                       p.y1 - p.y0 - 20.0));
        c.raw(&format!("<rect class=\"axp-f\" x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"{:.1}\"/>", px - 7.0, p.y0 + 10.0,
                       p.y1 - p.y0 - 20.0));
        c.name(if x == 0.0 { px + 10.0 } else { px - 10.0 }, p.y1 - 16.0, if x == 0.0 { "start" } else { "end" },
               &format!("J = {} kg·m²", num(jj)));
    }
    p.open(&mut c);
    p.area(&mut c, &[(0.0, 1.0), (l, ratio)], "axp-s");
    p.path(&mut c, &[(0.0, 1.0), (l, ratio)], "axp-c");
    p.close(&mut c);
    p.dot(&mut c, (l, ratio), "axp-d", 4.0);
    let (x, y) = p.pt((l, ratio));
    c.value(x - 12.0, y + (if ratio < 0.0 { 18.0 } else { -10.0 }), "end", &num(ratio));
    if node >= 0.0 && node <= l {
        p.mark(&mut c, (node, 0.0), &format!("node {} from rotor 1", q(node, "m")));
    }
    Some(s.figure("mode", "Two-rotor torsional mode and its node",
                  &format!("Two rotors (J1 = {} kg·m², J2 = {} kg·m²) joined by a shaft of length {}. The mode line joins \
                            rotor 1 (amplitude 1) to the returned amplitude ratio {} at rotor 2. Marked: the returned node {} \
                            from rotor 1; natural frequency ω = {} rad/s (f = {} Hz).", num(j1), num(j2), q(l, "m"),
                           num(ratio), q(node, "m"), num(w), num(f)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ bolt group
pub fn bolt_group(s: &Solved) -> Option<Figure> {
    let bolts = s.mat("bolts")?;
    let (p, e) = (s.inp("P")?, s.inp("e")?);
    let (direct, fmax, crit, polar) = (s.out("direct_kN")?, s.out("max_bolt_force_kN")?, s.out("critical_bolt")?,
                                       s.out("polar_sum_mm2")?);
    if bolts.is_empty() || bolts.len() > 24 || bolts.iter().any(|r| r.len() < 2) {
        return None;
    }
    let n = bolts.len() as f64;
    let (cx, cy) = (bolts.iter().map(|r| r[0]).sum::<f64>() / n, bolts.iter().map(|r| r[1]).sum::<f64>() / n);
    let lx = cx + e * 1000.0;
    let xs: Vec<f64> = bolts.iter().map(|r| r[0]).chain([lx, cx]).collect();
    let ys: Vec<f64> = bolts.iter().map(|r| r[1]).collect();
    let (x0, x1) = pl::range(&xs)?;
    let (y0, y1) = pl::range(&ys)?;
    let span = (x1 - x0).max(y1 - y0).max(1e-9);
    let sc = (380.0 / (x1 - x0).max(1e-9)).min(190.0 / (y1 - y0).max(1e-9)).min(380.0 / span * 2.0);
    let ox = 70.0 + (380.0 - (x1 - x0) * sc) / 2.0 - x0 * sc;
    let oy = 100.0 + (190.0 + (y1 - y0) * sc) / 2.0 + y0 * sc;
    let m = |x: f64, y: f64| (ox + x * sc, oy - y * sc);
    let mut c = Canvas::new(360.0);
    let ci = crit.round() as usize;
    for (i, r) in bolts.iter().enumerate() {
        let pt = m(r[0], r[1]);
        let hot = i + 1 == ci;
        c.raw(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"13\"/>", if hot { "axp-c" } else { "axp-k" }, pt.0, pt.1));
        c.name(pt.0, pt.1 + 5.0, "middle", &format!("{}", i + 1));
        if hot {
            c.value(pt.0 + 18.0, pt.1 - 14.0, "start", &format!("{} kN", num(fmax)));
        }
    }
    let cp = m(cx, cy);
    c.raw(&format!("<circle class=\"axp-d3\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/>", cp.0, cp.1));
    let lxp = m(lx, cy).0;
    c.dashed((lxp, 60.0), (lxp, 340.0));
    let (ay, by) = if p >= 0.0 { (30.0, 92.0) } else { (92.0, 30.0) };
    pl::arrow(&mut c, (lxp, ay), (lxp, by), 0);
    c.name(lxp + 8.0, 40.0, "start", &format!("P = {}", force(p.abs())));
    c.line((cp.0, 318.0), (lxp, 318.0));
    c.line((cp.0, 312.0), (cp.0, 324.0));
    c.name((cp.0 + lxp) / 2.0, 310.0, "middle", &format!("e = {}", pl::mm(e)));
    c.value(60.0, 346.0, "start", &format!("direct shear {} kN/bolt, Σr² = {} mm²", num(direct), pl::nn(polar)));
    Some(s.figure("schematic", "Eccentric bolt group: the most loaded bolt",
                  &format!("Bolt group ({} bolts at {} mm, numbered) under a load P = {} whose line of action is e = {} from \
                            the group centroid (the mean bolt position, marked). Marked: the returned most-loaded bolt {} with \
                            {} kN, direct shear {} kN per bolt, polar sum {} mm².", bolts.len(),
                           bolts.iter().map(|r| format!("{} {}", num(r[0]), num(r[1]))).collect::<Vec<_>>().join("; "),
                           force(p), pl::mm(e), ci, num(fmax), num(direct), pl::nn(polar)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ free bodies
/// A belt wrapped θ round a pulley: the contact arc over the top and the two straight runs, tensions as arrows.
fn belt(c: &mut Canvas, theta: f64, t1: &str, t2: &str) {
    let ctr = (200.0, 190.0);
    let r = 70.0;
    pl::circle(c, ctr, r, false);
    c.dot(ctr);
    let al = std::f64::consts::FRAC_PI_2 + theta / 2.0;
    let ar = std::f64::consts::FRAC_PI_2 - theta / 2.0;
    let rb = r + 4.0;
    let pt = |a: f64| (ctr.0 + rb * a.cos(), ctr.1 - rb * a.sin());
    let (pa, pb) = (pt(al), pt(ar));
    let large = if theta > std::f64::consts::PI { 1 } else { 0 };
    c.raw(&format!("<path class=\"axp-c\" d=\"M{:.1},{:.1} A{rb:.1},{rb:.1} 0 {large} 1 {:.1},{:.1}\"/>", pa.0, pa.1, pb.0, pb.1));
    // the runs leave along the tangents
    let ul = (-al.sin(), -al.cos());
    let ur = (ar.sin(), ar.cos());
    let len = 105.0;
    let ea = (pa.0 + ul.0 * len, pa.1 + ul.1 * len);
    let eb = (pb.0 + ur.0 * len, pb.1 + ur.1 * len);
    pl::arrow(c, pa, ea, 0);
    pl::arrow(c, pb, eb, 0);
    c.value(ea.0 - 6.0, ea.1 + 4.0, "end", t1);
    c.value(eb.0 + 6.0, eb.1 + 4.0, "start", t2);
    c.name(ctr.0, ctr.1 - 34.0, "middle", &format!("θ = {}°", num(theta.to_degrees())));
}

pub fn belt_drive(s: &Solved) -> Option<Figure> {
    let (t1, theta, mu) = (s.inp("T1")?, s.inp("theta")?, s.inp("mu")?);
    let (tc, t2, pw) = (s.out("Tc")?, s.out("T2")?, s.out("P")?);
    if !(theta > 0.0 && theta < std::f64::consts::TAU) {
        return None;
    }
    let v = s.inp("v");
    let mut c = Canvas::new(340.0);
    belt(&mut c, theta, &format!("T1 = {}", force(t1)), &format!("T2 = {}", force(t2)));
    for (i, t) in [format!("P = {}", q(pw, "W")), format!("Tc = {}", force(tc)), format!("μ = {}", num(mu))].iter().enumerate() {
        c.value(340.0, 50.0 + 19.0 * i as f64, "start", t);
    }
    Some(s.figure("fbd", "Belt drive: tensions and transmitted power",
                  &format!("A belt on its pulley (wrap angle θ = {}°, μ = {}{}) at the allowable tight-side tension T1 = {}. \
                            Marked: the returned slack-side tension T2 = {}, centrifugal tension Tc = {} and transmitted power \
                            P = {}.", num(theta.to_degrees()), num(mu),
                           v.map(|v| format!(", belt speed {} m/s", num(v))).unwrap_or_default(), force(t1), force(t2),
                           force(tc), q(pw, "W")), c.body(), c.h))
}

pub fn capstan(s: &Solved) -> Option<Figure> {
    let (t2, mu, theta) = (s.inp("T2")?, s.inp("mu")?, s.inp("theta")?);
    let (t1, ratio, ff) = (s.out("T1")?, s.out("ratio")?, s.out("friction_force")?);
    if !(theta > 0.0 && theta < std::f64::consts::TAU) {
        return None;
    }
    let mut c = Canvas::new(340.0);
    belt(&mut c, theta, &format!("T1 = {}", force(t1)), &format!("T2 = {}", force(t2)));
    for (i, t) in [format!("T1/T2 = {}", num(ratio)), format!("friction {}", force(ff)), format!("μ = {}", num(mu))]
        .iter().enumerate() {
        c.value(340.0, 50.0 + 19.0 * i as f64, "start", t);
    }
    Some(s.figure("fbd", "Capstan: belt friction on a drum",
                  &format!("A rope or belt wrapped θ = {}° round a drum with friction coefficient μ = {} and slack-side \
                            tension T2 = {}. Marked: the returned largest tight-side tension T1 = {}, the ratio T1/T2 = {} and \
                            the friction force {}.", num(theta.to_degrees()), num(mu), force(t2), force(t1), num(ratio),
                           force(ff)), c.body(), c.h))
}

pub fn moment_force(s: &Solved) -> Option<Figure> {
    let (fx, fy, x, y) = (s.inp("Fx")?, s.inp("Fy")?, s.inp("x")?, s.inp("y")?);
    let (x0, y0) = (s.inp("x0").unwrap_or(0.0), s.inp("y0").unwrap_or(0.0));
    let (mm, d) = (s.out("M")?, s.out("d")?);
    let fm = (fx * fx + fy * fy).sqrt();
    if !(fm > 0.0) {
        return None;
    }
    // the foot of the perpendicular from O to the line of action, and the distance it gives
    let u = (fx / fm, fy / fm);
    let t = (x0 - x) * u.0 + (y0 - y) * u.1;
    let foot = (x + u.0 * t, y + u.1 * t);
    let dist = ((foot.0 - x0).powi(2) + (foot.1 - y0).powi(2)).sqrt();
    if (dist - d).abs() > 1e-6 * d.abs().max(1e-9) + 1e-12 {
        return None;
    }
    let reach = ((x - x0).powi(2) + (y - y0).powi(2)).sqrt().max(d).max(1e-9);
    let fl = reach * 0.7;                                                 // the force arrow's drawn length
    let tip = (x + u.0 * fl, y + u.1 * fl);
    let l1 = (x - u.0 * reach * 1.3, y - u.1 * reach * 1.3);
    let l2 = (x + u.0 * reach * 1.1, y + u.1 * reach * 1.1);
    let xs = [x0, x, tip.0, foot.0, l1.0, l2.0];
    let ys = [y0, y, tip.1, foot.1, l1.1, l2.1];
    let mut c = Canvas::new(360.0);
    let p = Plot::equal((60.0, 40.0, 460.0, 330.0), &xs, &ys, "f")?;
    let (po, pa, pt_, pf) = (p.pt((x0, y0)), p.pt((x, y)), p.pt(tip), p.pt(foot));
    c.dashed(p.pt(l1), p.pt(l2));
    c.line(po, pa);
    c.raw(&format!("<path class=\"axp-c2\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", po.0, po.1, pf.0, pf.1));
    pl::arrow(&mut c, pa, pt_, 0);
    c.dot(po);
    c.dot(pa);
    c.name(po.0 - 26.0, po.1 + 16.0, "end", "O");
    c.name(pa.0 + 8.0, pa.1 + 14.0, "start", &format!("({}, {})", num(x), num(y)));
    c.name(pt_.0 + 8.0, pt_.1, "start", &format!("F = ({}, {})", num(fx), num(fy)));
    c.value((po.0 + pf.0) / 2.0 + 8.0, (po.1 + pf.1) / 2.0 + 16.0, "start", &format!("d = {}", num(d)));
    let ccw = mm >= 0.0;
    pl::arc_arrow(&mut c, po, 18.0, if ccw { 120.0 } else { 240.0 }, if ccw { 240.0 } else { 120.0 }, 2);
    c.value(po.0 - 10.0, po.1 - 28.0, "end", &format!("M = {}", num(mm)));
    Some(s.figure("fbd", "Moment of a force about a point",
                  &format!("A force (Fx, Fy) = ({}, {}) applied at ({}, {}) and its moment about O = ({}, {}); the dashed line \
                            is the line of action and the green segment the perpendicular from O to it. Marked: the returned \
                            moment M = {} (counter-clockwise positive) and distance d = {}.", num(fx), num(fy), num(x), num(y),
                           num(x0), num(y0), num(mm), num(d)), c.body(), c.h))
}

pub fn resultant(s: &Solved) -> Option<Figure> {
    let (fs, th) = (s.vec("F")?, s.vec("theta")?);
    let (r, rx, ry, dir) = (s.out("R")?, s.out("Rx")?, s.out("Ry")?, s.out("direction_deg")?);
    if fs.len() != th.len() || fs.is_empty() || fs.len() > 10 {
        return None;
    }
    // head to tail
    let mut pts = vec![(0.0, 0.0)];
    for (f, t) in fs.iter().zip(&th) {
        let (lx, ly) = *pts.last()?;
        pts.push((lx + f * t.to_radians().cos(), ly + f * t.to_radians().sin()));
    }
    let end = *pts.last()?;
    let sc = r.abs().max(fs.iter().map(|v| v.abs()).fold(0.0, f64::max)).max(1e-12);
    if (end.0 - rx).abs() > 1e-6 * sc || (end.1 - ry).abs() > 1e-6 * sc {
        return None;
    }
    let mut c = Canvas::new(360.0);
    pl::legend(&mut c, 30.0, 22.0, &[("force polygon (head to tail)".to_string(), "axp-c3"),
                                    ("resultant".to_string(), "axp-c")]);
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let p = Plot::equal((80.0, 60.0, 440.0, 330.0), &xs, &ys, "r")?;
    for (i, w) in pts.windows(2).enumerate() {
        pl::arrow(&mut c, p.pt(w[0]), p.pt(w[1]), 2);
        let mid = p.pt(((w[0].0 + w[1].0) / 2.0, (w[0].1 + w[1].1) / 2.0));
        if fs.len() <= 6 {
            c.name(mid.0 + 8.0, mid.1 - 6.0, "start", &format!("F{} = {} at {}°", i + 1, num(fs[i]), num(th[i])));
        }
    }
    pl::arrow(&mut c, p.pt((0.0, 0.0)), p.pt(end), 0);
    let pe = p.pt(end);
    c.value(pe.0 + 6.0, pe.1 - 8.0, "start", &format!("R = {} at {}°", num(r), num(dir)));
    c.dot(p.pt((0.0, 0.0)));
    Some(s.figure("fbd", "Resultant of concurrent forces",
                  &format!("Concurrent forces F = {} at angles {}° (from +x, counter-clockwise), drawn to one scale, their \
                            head-to-tail polygon and the resultant the model returned: R = {} at {}° (Rx = {}, Ry = {}). The \
                            polygon closes on the resultant.", join(&fs), join(&th), num(r), num(dir), num(rx), num(ry)),
                  c.body(), c.h))
}

pub fn two_cable(s: &Solved) -> Option<Figure> {
    let (w, a, b) = (s.inp("W")?, s.inp("alpha")?, s.inp("beta")?);
    let (t1, t2) = (s.out("T1")?, s.out("T2")?);
    if !(a > 0.0 && b > 0.0 && a < std::f64::consts::FRAC_PI_2 && b < std::f64::consts::FRAC_PI_2) {
        return None;
    }
    let mut c = Canvas::new(360.0);
    let node = (260.0, 190.0);
    let top = 70.0;
    let hl = (node.1 - top) / a.tan();
    let hr = (node.1 - top) / b.tan();
    let (la, rb) = ((node.0 - hl).max(20.0), (node.0 + hr).min(500.0));
    pl::ceiling(&mut c, la - 20.0, rb + 20.0, top);
    let pa = (node.0 - (node.1 - top) / a.tan(), top);
    let pb = (node.0 + (node.1 - top) / b.tan(), top);
    c.line(node, pa);
    c.line(node, pb);
    let tm = t1.abs().max(t2.abs()).max(w.abs()).max(1e-12);
    let sc = 95.0 / tm;
    let ua = (-(a.cos()), -(a.sin()));
    let ub = (b.cos(), -(b.sin()));
    let ea = (node.0 + ua.0 * t1 * sc, node.1 + ua.1 * t1 * sc);
    let eb = (node.0 + ub.0 * t2 * sc, node.1 + ub.1 * t2 * sc);
    pl::arrow(&mut c, node, ea, 0);
    pl::arrow(&mut c, node, eb, 0);
    pl::arrow(&mut c, node, (node.0, node.1 + w * sc), 2);
    c.value(ea.0 - 4.0, ea.1 - 6.0, "end", &format!("T1 = {}", force(t1)));
    c.value(eb.0 + 4.0, eb.1 - 6.0, "start", &format!("T2 = {}", force(t2)));
    c.name(node.0, node.1 + w * sc + 18.0, "middle", &format!("W = {}", force(w)));
    c.dashed((node.0 - 60.0, node.1), (node.0 + 60.0, node.1));
    c.name(node.0 - 64.0, node.1 - 8.0, "end", &format!("α = {}°", num(a.to_degrees())));
    c.name(node.0 + 64.0, node.1 - 8.0, "start", &format!("β = {}°", num(b.to_degrees())));
    c.dot(node);
    Some(s.figure("fbd", "Free-body diagram: a weight on two cables",
                  &format!("Free-body diagram of a weight W = {} held by two cables rising at α = {}° (left) and β = {}° \
                            (right) above the horizontal. The force arrows are drawn to one scale. Marked: the returned \
                            tensions T1 = {} and T2 = {}.", force(w), num(a.to_degrees()), num(b.to_degrees()), force(t1),
                           force(t2)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ mechanisms
fn draw_links(c: &mut Canvas, p: &Plot, segs: &[((f64, f64), (f64, f64), usize)]) {
    for (a, b, k_) in segs {
        let (pa, pb) = (p.pt(*a), p.pt(*b));
        c.raw(&format!("<path class=\"{}\" style=\"stroke-width:4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", pl::line_class(*k_),
                       pa.0, pa.1, pb.0, pb.1));
    }
}

pub fn four_bar(s: &Solved) -> Option<Figure> {
    let (a, b, cc, d, t2) = (s.inp("a")?, s.inp("b")?, s.inp("c")?, s.inp("d")?, s.inp("theta2")?);
    let (t3, t4, mu) = (s.out("theta3_deg")?, s.out("theta4_deg")?, s.out("transmission_angle_deg")?);
    let pa = (a * t2.cos(), a * t2.sin());
    let pb = (pa.0 + b * t3.to_radians().cos(), pa.1 + b * t3.to_radians().sin());
    let o4 = (d, 0.0);
    let pb2 = (o4.0 + cc * t4.to_radians().cos(), o4.1 + cc * t4.to_radians().sin());
    // the loop closes with the returned angles
    if ((pb.0 - pb2.0).powi(2) + (pb.1 - pb2.1).powi(2)).sqrt() > 1e-6 * (a + b + cc + d) {
        return None;
    }
    let xs = [0.0, pa.0, pb.0, d];
    let ys = [0.0, pa.1, pb.1, 0.0];
    let mut c = Canvas::new(380.0);
    let p = Plot::equal((70.0, 40.0, 450.0, 330.0), &xs, &ys, "k")?;
    c.dashed(p.pt((0.0, 0.0)), p.pt(o4));
    draw_links(&mut c, &p, &[((0.0, 0.0), pa, 0), (pa, pb, 2), (o4, pb, 1)]);
    for q_ in [(0.0, 0.0), o4] {
        pl::pin(&mut c, p.pt(q_));
    }
    for q_ in [(0.0, 0.0), pa, pb, o4] {
        c.dot(p.pt(q_));
    }
    let mid = |u: (f64, f64), v: (f64, f64)| p.pt(((u.0 + v.0) / 2.0, (u.1 + v.1) / 2.0));
    let m1 = mid((0.0, 0.0), pa);
    c.name(m1.0 - 8.0, m1.1 - 8.0, "end", &format!("a = {}", num(a)));
    let m2 = mid(pa, pb);
    c.name(m2.0 - 8.0, m2.1, "end", &format!("b = {}", num(b)));
    let m3 = mid(o4, pb);
    c.name(m3.0 + 8.0, m3.1, "start", &format!("c = {}", num(cc)));
    let m4 = mid((0.0, 0.0), o4);
    c.name(m4.0, m4.1 + 18.0, "middle", &format!("d = {}", num(d)));
    let (pp, po4) = (p.pt(pb), p.pt(o4));
    c.value(pp.0 + 10.0, pp.1 - 8.0, "start", &format!("μ = {}°", num(mu)));
    c.value(16.0, 360.0, "start", &format!("θ2 = {}°, θ3 = {}°, θ4 = {}°", num(t2.to_degrees()), num(t3), num(t4)));
    let _ = po4;
    Some(s.figure("mechanism", "Four-bar linkage at the given crank angle",
                  &format!("Four-bar linkage with crank a = {}, coupler b = {}, rocker c = {} and ground link d = {} at crank \
                            angle θ2 = {}°, drawn to scale from the returned angles (the loop closes). Marked: θ3 = {}°, θ4 = \
                            {}°, transmission angle μ = {}°.", num(a), num(b), num(cc), num(d), num(t2.to_degrees()), num(t3),
                           num(t4), num(mu)), c.body(), c.h))
}

pub fn slider_crank(s: &Solved) -> Option<Figure> {
    let (r, l, th0) = (s.inp("r")?, s.inp("l")?, s.inp("theta")?);
    // at a dead centre the linkage is a straight line: drawn at 60° instead (said so)
    let dead = th0.sin().abs() < 0.05;
    let (th, a) = if dead { (60f64.to_radians(), s.with(&[("theta", 60f64.to_radians())])?) } else { (th0, s.a.clone()) };
    let (x, v, acc, phi) = (k::num_of(&a, "x")?, k::num_of(&a, "v")?, k::num_of(&a, "a")?, k::num_of(&a, "rod_angle_deg")?);
    let pa = (r * th.cos(), r * th.sin());
    let pb = (x, 0.0);
    if (((pb.0 - pa.0).powi(2) + pa.1.powi(2)).sqrt() - l).abs() > 1e-6 * l {
        return None;
    }
    let xs = [-r, r, x + 0.15 * l, 0.0];
    let ys = [-r, r, 0.0];
    let mut c = Canvas::new(300.0);
    let p = Plot::equal((40.0, 50.0, 480.0, 240.0), &xs, &ys, "s")?;
    let po = p.pt((0.0, 0.0));
    let rr = p.x(r) - p.x(0.0);
    pl::dashed_circle(&mut c, po, rr);
    let pbp = p.pt(pb);
    pl::ground(&mut c, pbp.0 - 70.0, pbp.0 + 50.0, pbp.1 + 14.0);
    pl::rect(&mut c, pbp.0 - 26.0, pbp.1 - 14.0, 52.0, 28.0);
    draw_links(&mut c, &p, &[((0.0, 0.0), pa, 0), (pa, pb, 2)]);
    pl::pin(&mut c, po);
    for q_ in [(0.0, 0.0), pa, pb] {
        c.dot(p.pt(q_));
    }
    c.name(po.0 - 10.0, po.1 - 14.0, "end", &format!("θ = {}°", num(th.to_degrees())));
    let mid = p.pt(((pa.0 + pb.0) / 2.0, pa.1 / 2.0));
    c.value(mid.0, mid.1 - 10.0, "middle", &format!("φ = {}°", num(phi)));
    c.value(pbp.0, pbp.1 - 24.0, "middle", &format!("x = {}", pl::mm(x)));
    c.value(260.0, 284.0, "middle", &format!("v = {} m/s, a = {} m/s²", num(v), num(acc)));
    Some(s.figure("mechanism", &format!("Slider-crank at a crank angle of {}°", num(th.to_degrees())),
                  &format!("In-line slider-crank, crank r = {}, connecting rod l = {}, drawn to scale at θ = {}°{}. Marked: \
                            the model's slider position x = {}, velocity {} m/s, acceleration {} m/s² and rod angle {}°.",
                           pl::mm(r), pl::mm(l), num(th.to_degrees()),
                           if dead { " (an angle chosen for the drawing: the given angle is a dead centre, where the links \
                                      lie on one line)" } else { "" }, pl::mm(x), num(v), num(acc), num(phi)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ gears
/// A gear outline with n teeth round centre c: pitch radius rp, teeth of height h (schematic trapezoids).
fn gear(c: &mut Canvas, ctr: P, rp: f64, n: usize, h: f64, internal: bool, rot: f64) {
    if n < 3 || n > 400 {
        pl::circle(c, ctr, rp, !internal);
        return;
    }
    let (r_in, r_out) = if internal { (rp - h / 2.0, rp + h / 2.0) } else { (rp - h / 2.0, rp + h / 2.0) };
    let mut d = String::new();
    for i in 0..n {
        let a0 = rot + std::f64::consts::TAU * i as f64 / n as f64;
        let st = std::f64::consts::TAU / n as f64;
        let (tip, root) = if internal { (r_in, r_out) } else { (r_out, r_in) };
        let pts = [(a0, root), (a0 + st * 0.15, tip), (a0 + st * 0.45, tip), (a0 + st * 0.6, root)];
        for (j, (a, rr)) in pts.iter().enumerate() {
            let p = (ctr.0 + rr * a.cos(), ctr.1 - rr * a.sin());
            d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 && j == 0 { "M" } else { "L" }, p.0, p.1));
        }
    }
    let class = if internal { "axp-k" } else { "axp-ka" };
    if !internal {
        c.raw(&format!("<path class=\"{class}\" d=\"{}Z\"/>", d));
    }
    c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:1.2\" d=\"{}Z\"/>", d));
}

pub fn planetary(s: &Solved) -> Option<Figure> {
    let (ns, nr) = (n_in(s, "Ns")?, n_in(s, "Nr")?);
    let (ws, wr, wc, wp, np) = (s.out("ws_rpm")?, s.out("wr_rpm")?, s.out("wc_rpm")?, s.out("wp_rpm")?, s.out("Np")?);
    if !(ns >= 3.0 && nr > ns && (nr - ns - 2.0 * np).abs() < 1e-9) {
        return None;
    }
    let mut c = Canvas::new(360.0);
    let ctr = (140.0, 180.0);
    let sc = 135.0 / (nr / 2.0 + 1.5);
    let (rs, rr, rpl) = (ns / 2.0 * sc, nr / 2.0 * sc, np / 2.0 * sc);
    let h = (sc * 1.0).clamp(1.5, 8.0);
    pl::circle(&mut c, ctr, rr + h + 8.0, false);
    gear(&mut c, ctr, rr, nr as usize, h, true, 0.0);
    gear(&mut c, ctr, rs, ns as usize, h, false, 0.0);
    let orbit = rs + rpl;
    for i in 0..3 {
        let a = std::f64::consts::FRAC_PI_2 + std::f64::consts::TAU * i as f64 / 3.0;
        let pc = (ctr.0 + orbit * a.cos(), ctr.1 - orbit * a.sin());
        gear(&mut c, pc, rpl, np as usize, h, false, 0.0);
        c.raw(&format!("<path class=\"axp-c2\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", ctr.0, ctr.1, pc.0, pc.1));
        c.dot(pc);
    }
    c.dot(ctr);
    for (i, t) in [format!("sun ({} teeth): {} rpm", num(ns), num(ws)), format!("ring ({} teeth): {} rpm", num(nr), num(wr)),
                   format!("carrier: {} rpm", num(wc)), format!("planet ({} teeth): {} rpm", num(np), num(wp))]
        .iter().enumerate() {
        c.value(300.0, 140.0 + 20.0 * i as f64, "start", t);
    }
    Some(s.figure("gears", "Planetary gear train: member speeds",
                  &format!("A simple planetary train with a {}-tooth sun and a {}-tooth ring (pitch circles to scale; the \
                            carrier arms in green; three planets drawn, their number does not change the speeds). Marked: \
                            the returned speeds sun {} rpm, ring {} rpm, carrier {} rpm, planet {} rpm ({} teeth).", num(ns),
                           num(nr), num(ws), num(wr), num(wc), num(wp), num(np)), c.body(), c.h))
}

pub fn spur_mesh(s: &Solved) -> Option<Figure> {
    let (n1, n2, m, phi) = (n_in(s, "N1")?, n_in(s, "N2")?, s.inp("m")?, s.inp("phi")?);
    let (d1, d2, cd, z, pb, cr) = (s.out("d1_mm")?, s.out("d2_mm")?, s.out("C_mm")?, s.out("Z_mm")?, s.out("pb_mm")?,
                                   s.out("contact_ratio")?);
    let mm_ = m * 1000.0;
    let (r1, r2) = (d1 / 2.0, d2 / 2.0);
    if !(r1 > 0.0 && r2 > 0.0) {
        return None;
    }
    // the path of contact: the line of action through the pitch point, between the addendum circles
    let (ra1, ra2) = (r1 + mm_, r2 + mm_);
    let o1 = (0.0, 0.0);
    let o2 = (cd, 0.0);
    let pp = (r1, 0.0);
    let u = (phi.sin(), phi.cos());                 // the line of action, at φ to the common tangent (vertical)
    let hit = |o: (f64, f64), ra: f64, sign: f64| -> Option<(f64, f64)> {
        let (dx, dy) = (pp.0 - o.0, pp.1 - o.1);
        let bq = dx * u.0 + dy * u.1;
        let cq = dx * dx + dy * dy - ra * ra;
        let disc = bq * bq - cq;
        if disc < 0.0 {
            return None;
        }
        let t = if sign > 0.0 { -bq + disc.sqrt() } else { -bq - disc.sqrt() };
        Some((pp.0 + u.0 * t, pp.1 + u.1 * t))
    };
    let e1 = hit(o2, ra2, -1.0)?;
    let e2 = hit(o1, ra1, 1.0)?;
    let zz = ((e1.0 - e2.0).powi(2) + (e1.1 - e2.1).powi(2)).sqrt();
    if (zz - z).abs() > 1e-3 * z.max(1e-9) {
        return None;
    }
    let mut c = Canvas::new(380.0);
    let xs = [-ra1, cd + ra2];
    let ys = [-ra2.max(ra1), ra2.max(ra1)];
    let p = Plot::equal((20.0, 50.0, 500.0, 350.0), &xs, &ys, "g")?;
    let sc = p.x(1.0) - p.x(0.0);
    let (c1, c2) = (p.pt(o1), p.pt(o2));
    gear(&mut c, c1, r1 * sc, n1 as usize, 2.25 * mm_ * sc, false, 0.0);
    gear(&mut c, c2, r2 * sc, n2 as usize, 2.25 * mm_ * sc, false, std::f64::consts::PI / n2);
    pl::dashed_circle(&mut c, c1, r1 * sc);
    pl::dashed_circle(&mut c, c2, r2 * sc);
    c.dot(c1);
    c.dot(c2);
    let (a, b) = (p.pt(e1), p.pt(e2));
    c.raw(&format!("<path class=\"axp-c\" style=\"stroke-width:4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0, a.1, b.0, b.1));
    c.raw(&format!("<circle class=\"axp-d3\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\"/>", p.pt(pp).0, p.pt(pp).1));
    c.value(b.0 + 10.0, b.1 + 6.0, "start", &format!("Z = {} mm", num(z)));
    c.name(c1.0, (c1.1 + ra1 * sc + 18.0).min(372.0), "middle", &format!("N1 = {}, d1 = {} mm", num(n1), num(d1)));
    c.name(c2.0, (c2.1 + ra2 * sc + 18.0).min(372.0), "middle", &format!("N2 = {}, d2 = {} mm", num(n2), num(d2)));
    c.value(20.0, 26.0, "start", &format!("contact ratio {} · pb = {} mm · C = {} mm", num(cr), num(pb), num(cd)));
    Some(s.figure("gears", "Spur-gear mesh and the path of contact",
                  &format!("A spur-gear pair, N1 = {} and N2 = {} teeth, module {}, pressure angle {}°, drawn to scale (pitch \
                            circles dashed, tooth outlines schematic, addendum of one module). The solid segment on the line \
                            of action is the path of contact between the two addendum circles; its length agrees with the \
                            returned Z = {} mm. Marked: contact ratio {}, base pitch {} mm, centre distance {} mm.", num(n1),
                           num(n2), pl::mm(m), num(phi.to_degrees()), num(z), num(cr), num(pb), num(cd)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ bars and shafts
pub fn axial_bar(s: &Solved) -> Option<Figure> {
    let (ls, ars, loads) = (s.vec("lengths")?, s.vec("areas")?, s.vec("loads")?);
    let (nk, st) = (s.out_list("N_kN")?, s.out_list("stress_MPa")?);
    let n = ls.len();
    if n == 0 || n > 8 || ars.len() != n || loads.len() != n || nk.len() != n || st.len() != n || ls.iter().any(|v| !(*v > 0.0))
        || ars.iter().any(|v| !(*v > 0.0)) {
        return None;
    }
    let total: f64 = ls.iter().sum();
    let amax = ars.iter().copied().fold(0.0f64, f64::max);
    let mut c = Canvas::new(390.0);
    let (x0, x1, yc) = (96.0, 480.0, 110.0);
    let px = |x: f64| x0 + (x1 - x0) * x / total;
    pl::wall(&mut c, x0, yc - 50.0, yc + 50.0, true);
    let mut x = 0.0;
    for i in 0..n {
        let h = 18.0 + 52.0 * (ars[i] / amax).sqrt();
        pl::rect(&mut c, px(x), yc - h / 2.0, px(x + ls[i]) - px(x), h);
        let lab = format!("A = {} mm², σ = {}", num(ars[i]), mpa(st[i]));
        let half = 3.95 * lab.chars().count() as f64;
        let mid = ((px(x) + px(x + ls[i])) / 2.0).max(x0 + 10.0 + half);
        c.value(mid, yc + h / 2.0 + 16.0 + 16.0 * (i % 2) as f64, "middle", &lab);
        x += ls[i];
        let xe = px(x);
        if loads[i] != 0.0 {
            let (a, b) = if loads[i] > 0.0 { ((xe + 2.0, yc - 52.0), (xe + 30.0, yc - 52.0)) } else {
                ((xe - 2.0, yc - 52.0), (xe - 30.0, yc - 52.0))
            };
            c.line((xe, yc - 44.0), (xe, yc - 58.0));
            pl::arrow(&mut c, a, b, 0);
            c.name(xe, yc - 64.0, "middle", &format!("{} kN", num(loads[i])));
        }
    }
    // the axial force, piecewise constant
    let mut pts = vec![];
    let mut xx = 0.0;
    for i in 0..n {
        pts.push((xx, nk[i]));
        xx += ls[i];
        pts.push((xx, nk[i]));
    }
    let (yl, yh) = pl::range(&nk.iter().copied().chain([0.0]).collect::<Vec<_>>())?;
    let p = Plot::new((x0, 220.0, x1, 336.0), (0.0, total), (yl, yh + 0.25 * (yh - yl)), (false, false), (false, true), "n");
    p.frame(&mut c, "x, distance from the fixed end (mm)", "N (kN)");
    p.zeros(&mut c, false, true);
    p.open(&mut c);
    let mut fillp = vec![(0.0, 0.0)];
    fillp.extend(pts.iter().copied());
    fillp.push((total, 0.0));
    p.fill(&mut c, &fillp, "axp-s");
    p.path(&mut c, &fillp, "axp-c");
    p.close(&mut c);
    let mut xx = 0.0;
    for i in 0..n {
        let (cx, cy) = p.pt((xx + ls[i] / 2.0, nk[i]));
        c.value(cx, if nk[i] >= 0.0 { cy - 6.0 } else { cy + 16.0 }, "middle", &format!("N = {} kN", num(nk[i])));
        xx += ls[i];
    }
    let el = s.out("elongation_mm");
    Some(s.figure("bar", "Stepped bar: axial forces and stresses",
                  &format!("A stepped bar fixed at its left end: segment lengths {} mm, areas {} mm², axial loads {} kN at the \
                            far end of each segment (positive pulling away from the support); lengths to scale, heights \
                            growing with area. Below: the internal axial force of each segment as returned ({} kN), stresses \
                            {} MPa{}.", join(&ls), join(&ars), join(&loads), join(&nk), join(&st),
                           el.map(|e| format!("; total elongation {} mm", num(e))).unwrap_or_default()), c.body(), c.h))
}

pub fn bar_two_walls(s: &Solved) -> Option<Figure> {
    let (p, l1, a1, l2, a2) = (s.inp("P")?, s.inp("L1")?, s.inp("A1")?, s.inp("L2")?, s.inp("A2")?);
    let (ra, rb, s1, s2, d) = (s.out("RA")?, s.out("RB")?, s.out("stress1_MPa")?, s.out("stress2_MPa")?, s.out("displacement_mm")?);
    if !(l1 > 0.0 && l2 > 0.0 && a1 > 0.0 && a2 > 0.0) {
        return None;
    }
    let mut c = Canvas::new(260.0);
    let (x0, x1, y) = (90.0, 430.0, 130.0);
    let xj = x0 + (x1 - x0) * l1 / (l1 + l2);
    let amax = a1.max(a2);
    let (h1, h2) = (20.0 + 30.0 * (a1 / amax).sqrt(), 20.0 + 30.0 * (a2 / amax).sqrt());
    pl::wall(&mut c, x0, y - 45.0, y + 45.0, true);
    pl::wall(&mut c, x1, y - 45.0, y + 45.0, false);
    pl::rect(&mut c, x0, y - h1, xj - x0, 2.0 * h1);
    pl::rect(&mut c, xj, y - h2, x1 - xj, 2.0 * h2);
    c.value((x0 + xj) / 2.0, y + 5.0, "middle", &format!("σ = {}", mpa(s1)));
    c.value((xj + x1) / 2.0, y + 5.0, "middle", &format!("σ = {}", mpa(s2)));
    pl::arrow(&mut c, (xj - 40.0, y - h1.max(h2) - 10.0), (xj, y - h1.max(h2) - 10.0), 0);
    c.name(xj - 44.0, y - h1.max(h2) - 6.0, "end", &format!("P = {}", force(p)));
    c.value(xj, y - h1.max(h2) - 26.0, "middle", &format!("δ = {} mm", num(d)));
    c.raw(&format!("<circle class=\"axp-d3\" cx=\"{xj:.1}\" cy=\"{y:.1}\" r=\"3.5\"/>"));
    pl::arrow(&mut c, (x0 + 40.0, y + 70.0), (x0 + 4.0, y + 70.0), 1);
    pl::arrow(&mut c, (x1 + 2.0, y + 70.0), (x1 - 34.0, y + 70.0), 1);
    c.value(x0 + 48.0, y + 74.0, "start", &format!("RA = {}", force(ra)));
    c.value(x1 - 42.0, y + 74.0, "end", &format!("RB = {}", force(rb)));
    Some(s.figure("bar", "Bar between two walls: reactions and stresses",
                  &format!("A two-part bar held between rigid walls (part 1: L = {}, A = {} m²; part 2: L = {}, A = {} m²) with \
                            an axial load P = {} at the joint, towards B. Marked: the returned reactions RA = {} and RB = {}, \
                            stresses {} and {}, joint displacement {} mm.", q(l1, "m"), num(a1), q(l2, "m"), num(a2), force(p),
                           force(ra), force(rb), mpa(s1), mpa(s2), num(d)), c.body(), c.h))
}

pub fn fixed_shaft(s: &Solved) -> Option<Figure> {
    let (t0, a, b, d1, d2) = (s.inp("T0")?, s.inp("a")?, s.inp("b")?, s.inp("d1")?, s.inp("d2")?);
    let (ta, tb, s1, s2, tw) = (s.out("TA")?, s.out("TB")?, s.out("tau1_MPa")?, s.out("tau2_MPa")?, s.out("twist_at_load_deg")?);
    if !(a > 0.0 && b > 0.0 && d1 > 0.0 && d2 > 0.0) {
        return None;
    }
    let mut c = Canvas::new(260.0);
    let (x0, x1, y) = (90.0, 440.0, 130.0);
    let xj = x0 + (x1 - x0) * a / (a + b);
    let dm = d1.max(d2);
    let (h1, h2) = (10.0 + 26.0 * d1 / dm, 10.0 + 26.0 * d2 / dm);
    pl::wall(&mut c, x0, y - 45.0, y + 45.0, true);
    pl::wall(&mut c, x1, y - 45.0, y + 45.0, false);
    pl::rect(&mut c, x0, y - h1, xj - x0, 2.0 * h1);
    pl::rect(&mut c, xj, y - h2, x1 - xj, 2.0 * h2);
    c.dashed((x0 - 10.0, y), (x1 + 10.0, y));
    pl::arc_arrow(&mut c, (xj, y), h1.max(h2) + 8.0, 100.0, 260.0, 0);
    c.name(xj, y - h1.max(h2) - 22.0, "middle", &format!("T0 = {}", moment(t0)));
    c.value((x0 + xj) / 2.0, y + h1 + 18.0, "middle", &format!("τ1 = {}", mpa(s1)));
    c.value((xj + x1) / 2.0, y + h2 + 18.0, "middle", &format!("τ2 = {}", mpa(s2)));
    c.value(x0, y + 72.0, "start", &format!("TA = {}", moment(ta)));
    c.value(x1, y + 72.0, "end", &format!("TB = {}", moment(tb)));
    c.value(260.0, 244.0, "middle", &format!("rotation at the load {}°", num(tw)));
    Some(s.figure("shaft", "Shaft fixed at both ends: reaction torques",
                  &format!("A shaft built in at both ends with a torque T0 = {} applied a = {} from end A (b = {} from end B; \
                            diameters {} and {}), lengths to scale, diameters enlarged. Marked: the returned reaction torques \
                            TA = {} and TB = {}, shear stresses τ1 = {} and τ2 = {}, rotation at the load {}°.", moment(t0),
                           q(a, "m"), q(b, "m"), pl::mm(d1), pl::mm(d2), moment(ta), moment(tb), mpa(s1), mpa(s2), num(tw)),
                  c.body(), c.h))
}

pub fn stepped_shaft(s: &Solved) -> Option<Figure> {
    let (t, ls, ds) = (s.inp("T")?, s.vec("lengths")?, s.vec("diameters")?);
    let (tw, tmax) = (s.out("twist_deg")?, s.out("tau_max_MPa")?);
    let n = ls.len();
    if n == 0 || n > 8 || ds.len() != n || ls.iter().chain(&ds).any(|v| !(*v > 0.0)) {
        return None;
    }
    let total: f64 = ls.iter().sum();
    let dm = ds.iter().copied().fold(0.0f64, f64::max);
    let mut c = Canvas::new(260.0);
    let (x0, x1, y) = (80.0, 440.0, 150.0);
    let px = |x: f64| x0 + (x1 - x0) * x / total;
    pl::wall(&mut c, x0, y - 40.0, y + 40.0, true);
    let mut x = 0.0;
    for i in 0..n {
        let h = 6.0 + 26.0 * ds[i] / dm;
        pl::rect(&mut c, px(x), y - h, px(x + ls[i]) - px(x), 2.0 * h);
        c.name((px(x) + px(x + ls[i])) / 2.0, y + h + 18.0 + 16.0 * (i % 2) as f64, "middle",
               &format!("d = {} mm, {} mm", num(ds[i]), num(ls[i])));
        x += ls[i];
    }
    c.dashed((x0 - 10.0, y), (x1 + 20.0, y));
    pl::arc_arrow(&mut c, (x1, y), 36.0, 100.0, 260.0, 0);
    c.name(x1 + 4.0, y - 44.0, "middle", &format!("T = {}", moment(t)));
    c.value(90.0, 40.0, "start", &format!("twist {}°", num(tw)));
    c.value(90.0, 60.0, "start", &format!("τmax = {}", mpa(tmax)));
    Some(s.figure("shaft", "Stepped shaft in torsion",
                  &format!("A stepped solid shaft (lengths {} mm, diameters {} mm, diameters drawn enlarged) fixed at the left \
                            and carrying a torque T = {}. Marked: the returned total angle of twist {}° and largest shear \
                            stress {}.", join(&ls), join(&ds), moment(t), num(tw), mpa(tmax)), c.body(), c.h))
}

pub fn euler_column(s: &Solved) -> Option<Figure> {
    let l = s.inp("L")?;
    let ends = s.txt("ends")?;
    let (pcr, kk, le) = (s.out("P_cr")?, s.out("K")?, s.out("Le")?);
    let mut c = Canvas::new(380.0);
    let (x, top, bot) = (270.0, 80.0, 330.0);
    pl::rect(&mut c, x - 6.0, top, 12.0, bot - top);
    let (lower, upper) = match ends.as_str() {
        "pinned-pinned" => ("pinned", "pinned"),
        "fixed-free" => ("fixed", "free"),
        "fixed-fixed" => ("fixed", "fixed"),
        "fixed-pinned" => ("fixed", "pinned"),
        _ => return None,
    };
    match lower {
        "fixed" => pl::ground(&mut c, x - 30.0, x + 30.0, bot),
        _ => pl::pin(&mut c, (x, bot)),
    }
    match upper {
        "fixed" => pl::ceiling(&mut c, x - 30.0, x + 30.0, top),
        "pinned" => {
            c.raw(&format!("<path class=\"axp-k\" d=\"M{x:.1},{top:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", x - 10.0, top - 14.0,
                           x + 10.0, top - 14.0));
        }
        _ => {}
    }
    pl::arrow(&mut c, (x, 26.0), (x, top - if upper == "pinned" { 16.0 } else { 2.0 }), 0);
    c.value(x + 10.0, 36.0, "start", &format!("Pcr = {}", force(pcr)));
    c.name(x - 40.0, (top + bot) / 2.0, "end", &ends);
    c.line((x + 70.0, top), (x + 70.0, bot));
    c.line((x + 64.0, top), (x + 76.0, top));
    c.line((x + 64.0, bot), (x + 76.0, bot));
    c.name(x + 82.0, (top + bot) / 2.0 - 16.0, "start", &format!("L = {}", q(l, "m")));
    c.value(x + 82.0, (top + bot) / 2.0 + 4.0, "start", &format!("Le = KL = {}", q(le, "m")));
    c.value(x + 82.0, (top + bot) / 2.0 + 24.0, "start", &format!("K = {}", pl::nn(kk)));
    Some(s.figure("column", "Column: end conditions and Euler load",
                  &format!("A column of length L = {} with {} ends. Marked: the returned effective length factor K = {}, \
                            effective length {} and critical buckling load Pcr = {}.", q(l, "m"), ends, pl::nn(kk), q(le, "m"),
                           force(pcr)), c.body(), c.h))
}

pub fn secant_family(s: &Solved) -> Option<Figure> {
    let (p, i, sm) = (s.inp("P")?, s.inp("I")?, s.out("sigma_max_MPa")?);
    let xs = k::span(i / 10.0, i * 10.0, 41, true);
    let fam = s.family("I", &xs, "sigma_max_MPa", "P", &[p, p / 2.0, p * 2.0], "N");
    if fam.is_empty() {
        return None;
    }
    let body = k::lines(&fam, "I, second moment of area (m⁴)", "σmax (MPa)", "", Some((i, sm)), &mpa(sm), true);
    Some(s.chart("curve-2", "Eccentrically loaded column (secant formula): maximum stress against I",
                 &format!("Maximum compressive stress against I from {} to {} m⁴ on a logarithmic axis, one curve for each \
                           load P ({}, half and twice it), 41 points each computed by the model (loads past buckling are \
                           left out). Marked: I = {} m⁴, σmax = {}.", num(i / 10.0), num(i * 10.0), force(p), num(i),
                          mpa(sm)), &body))
}

pub fn wahl_field(s: &Solved) -> Option<Figure> {
    let (d, dd, kw) = (s.inp("d")?, s.inp("D")?, s.out("K_wahl")?);
    let xs = k::span(d / 10.0, d * 10.0, 21, true);
    let ys = k::span(dd / 10.0, dd * 10.0, 15, true);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with(&[("d", *x), ("D", *y)]).and_then(|a| k::num_of(&a, "K_wahl"));
            if v.is_some() {
                n += 1;
            }
            col.push(v);
        }
        vals.push(col);
    }
    if n < 10 {
        return None;
    }
    let body = k::field(&xs, &ys, &vals, (d, dd), "d, wire diameter (m)", "D, mean coil diameter (m)",
                        &format!("K = {}", num(kw)), (true, true));
    Some(s.chart("field", "Wahl factor over wire and coil diameter",
                 &format!("Wahl factor over d from {} to {} and D from {} to {} (logarithmic axes): each of the {} cells is \
                           one run of the model, shaded by its value (blank where the spring index is refused). Marked: \
                           d = {}, D = {}, K = {}.", pl::mm(xs[0]), pl::mm(xs[20]), pl::mm(ys[0]), pl::mm(ys[14]), n,
                          pl::mm(d), pl::mm(dd), num(kw)), &body))
}

// ------------------------------------------------------------------------------------------------ truss
pub fn truss(s: &Solved) -> Option<Figure> {
    let nodes = s.mat("nodes")?;
    let members = match s.val("members")? {
        Val::Edges(e) => e,
        _ => return None,
    };
    let loads = s.mat("loads").unwrap_or_default();
    let (pin, rol) = (n_in(s, "pin")? as usize, n_in(s, "roller")? as usize);
    let f = s.out_list("forces_kN")?;
    let (prx, pry, rry) = (s.out("pin_Rx_kN")?, s.out("pin_Ry_kN")?, s.out("roller_Ry_kN")?);
    if nodes.len() < 2 || nodes.len() > 30 || f.len() != members.len() || members.len() > 60 {
        return None;
    }
    let idx = |t: &str| -> Option<usize> { t.trim().parse::<usize>().ok().filter(|i| *i >= 1 && *i <= nodes.len()) };
    let xs: Vec<f64> = nodes.iter().map(|r| r[0]).collect();
    let ys: Vec<f64> = nodes.iter().map(|r| r[1]).collect();
    let mut c = Canvas::new(380.0);
    pl::legend(&mut c, 40.0, 22.0, &[("tension".to_string(), "axp-c"), ("compression".to_string(), "axp-c3")]);
    let p = Plot::equal((70.0, 80.0, 450.0, 300.0), &xs, &ys, "t")?;
    let fm = f.iter().map(|v| v.abs()).fold(0.0, f64::max).max(1e-12);
    for ((u, v, _), fv) in members.iter().zip(&f) {
        let (i, j) = (idx(u)?, idx(v)?);
        let (a, b) = (p.pt((xs[i - 1], ys[i - 1])), p.pt((xs[j - 1], ys[j - 1])));
        let class = if fv.abs() <= 1e-9 * fm { "axp-c4" } else if *fv > 0.0 { "axp-c" } else { "axp-c3" };
        c.raw(&format!("<path class=\"{class}\" style=\"stroke-width:3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0, a.1, b.0, b.1));
        if members.len() <= 16 {
            // beside the member, on the side away from the truss's middle
            let (mx, my) = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            let mut n = (-dy / d, dx / d);
            let ctr = p.pt(((xs.iter().sum::<f64>()) / xs.len() as f64, (ys.iter().sum::<f64>()) / ys.len() as f64));
            if (mx - ctr.0) * n.0 + (my - ctr.1) * n.1 < 0.0 {
                n = (-n.0, -n.1);
            }
            let anchor = if n.0 > 0.4 { "start" } else if n.0 < -0.4 { "end" } else { "middle" };
            c.value(mx + n.0 * 10.0, my + n.1 * 12.0 + 4.0, anchor, &num(*fv));
        }
    }
    for (i, (x, y)) in xs.iter().zip(&ys).enumerate() {
        let pt = p.pt((*x, *y));
        if i + 1 == pin {
            pl::pin(&mut c, (pt.0, pt.1 + 3.0));
        } else if i + 1 == rol {
            pl::roller(&mut c, (pt.0, pt.1 + 3.0));
        }
        c.dot(pt);
        if nodes.len() <= 16 {
            c.name(pt.0 + 8.0, pt.1 - 6.0, "start", &format!("{}", i + 1));
        }
    }
    let lm = loads.iter().map(|r| (r.get(1).copied().unwrap_or(0.0).powi(2) + r.get(2).copied().unwrap_or(0.0).powi(2)).sqrt())
        .fold(0.0, f64::max).max(1e-12);
    for r in loads.iter().take(10) {
        if r.len() < 3 {
            continue;
        }
        let ni = r[0].round() as usize;
        if ni < 1 || ni > nodes.len() {
            continue;
        }
        let pt = p.pt((xs[ni - 1], ys[ni - 1]));
        let (fx, fy) = (r[1], r[2]);
        let len = 50.0 * (fx * fx + fy * fy).sqrt() / lm;
        let mag = (fx * fx + fy * fy).sqrt().max(1e-12);
        let tail = (pt.0 - fx / mag * len, pt.1 + fy / mag * len);
        pl::arrow(&mut c, tail, (pt.0 - fx / mag * 4.0, pt.1 + fy / mag * 4.0), 1);
        c.name(tail.0, tail.1 - 6.0, "middle", &format!("{} kN", num(mag)));
    }
    if (1..=nodes.len()).contains(&pin) && (1..=nodes.len()).contains(&rol) {
        let pp = p.pt((xs[pin - 1], ys[pin - 1]));
        let rp = p.pt((xs[rol - 1], ys[rol - 1]));
        c.value(pp.0, (pp.1 + 44.0).min(372.0), "middle", &format!("{} kN", num(pry)));
        c.value(rp.0, (rp.1 + 44.0).min(372.0), "middle", &format!("{} kN", num(rry)));
    }
    Some(s.figure("truss", "Truss member forces: tension and compression",
                  &format!("Plane truss with joints at {} m, members {}, a pin at joint {} and a roller at joint {}, drawn to \
                            scale. Each member is labelled with the force the model returned (kN, positive = tension) and \
                            coloured by tension or compression. Marked: reactions at the pin {} kN across and {} kN up, at \
                            the roller {} kN up.",
                           nodes.iter().map(|r| format!("{} {}", num(r[0]), num(r[1]))).collect::<Vec<_>>().join("; "),
                           members.iter().map(|(u, v, _)| format!("{u}-{v}")).collect::<Vec<_>>().join(", "), pin, rol,
                           num(prx), num(pry), num(rry)), c.body(), c.h))
}
