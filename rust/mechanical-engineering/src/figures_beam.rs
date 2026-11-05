//! Beams: the loaded beam drawn to scale over its shear-force and bending-moment diagrams, which follow by statics
//! from the reactions and fixing moments the model returned (drawn only when those reactions balance the loads, and
//! when the diagram's largest moment agrees with the model's own), and a cantilever's deflected shape from further
//! solves of the model.

use crate::figures_plot::{self as pl, force, moment, Plot};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;

#[derive(Clone, Copy, PartialEq)]
pub enum Sup {
    Pin,
    Roller,
    Fixed,
}

/// A loaded beam with the reactions the model returned (forces in N, lengths in m, loads downward positive).
pub struct Beam {
    pub l: f64,
    pub sups: Vec<(f64, Sup, String)>,
    /// point loads (x, P down)
    pub pts: Vec<(f64, f64)>,
    /// linear distributed loads (x1, x2, w1, w2) down
    pub dist: Vec<(f64, f64, f64, f64)>,
    /// reactions (x, R up)
    pub reac: Vec<(f64, f64)>,
    /// hogging fixing moments at the left and right walls (magnitudes as returned, sagging positive convention)
    pub m_left: f64,
    pub m_right: f64,
}

impl Beam {
    /// Shear force and bending moment just right of x (forces at positions < x counted).
    pub fn vm(&self, x: f64) -> (f64, f64) {
        let mut v = 0.0;
        let mut m = 0.0;
        for (xr, r) in &self.reac {
            if *xr < x {
                v += r;
                m += r * (x - xr);
            }
        }
        for (xp, p) in &self.pts {
            if *xp < x {
                v -= p;
                m -= p * (x - xp);
            }
        }
        for (x1, x2, w1, w2) in &self.dist {
            if x <= *x1 || x2 <= x1 {
                continue;
            }
            let u = x.min(*x2) - x1;
            let s = (w2 - w1) / (x2 - x1);
            let f = w1 * u + s * u * u / 2.0;
            let mo = (x - x1) * f - (w1 * u * u / 2.0 + s * u * u * u / 3.0);
            v -= f;
            m -= mo;
        }
        if x > 0.0 {
            m -= self.m_left;
        }
        if x > self.l {
            m += self.m_right;
        }
        (v, m)
    }
    fn total_load(&self) -> f64 {
        let mut t: f64 = self.pts.iter().map(|p| p.1.abs()).sum();
        for (x1, x2, w1, w2) in &self.dist {
            t += (w1.abs() + w2.abs()) / 2.0 * (x2 - x1).abs();
        }
        t
    }
    /// The reactions balance the loads: past the right end, no shear and no moment remain.
    pub fn balanced(&self) -> bool {
        let t = self.total_load().max(1e-300);
        let (v, m) = self.vm(self.l * (1.0 + 1e-9) + 1e-12);
        v.abs() <= 1e-6 * t && m.abs() <= 1e-6 * t * self.l
    }
    /// Sample positions: a grid plus each side of every jump.
    pub fn xs(&self) -> Vec<f64> {
        let mut xs: Vec<f64> = (0..=200).map(|i| self.l * i as f64 / 200.0).collect();
        let e = self.l * 1e-7;
        for x in self.pts.iter().map(|p| p.0).chain(self.reac.iter().map(|r| r.0)) {
            if x > 0.0 && x < self.l {
                xs.push(x - e);
                xs.push(x + e);
            }
        }
        xs.push(e);
        xs.push(self.l - e);
        xs.retain(|x| *x > 0.0 && *x < self.l);
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs.dedup();
        xs
    }
}

/// The force unit of a beam's diagrams: kN when the loads are large.
fn fscale(b: &Beam) -> (f64, &'static str) {
    let big = b.reac.iter().map(|r| r.1.abs()).chain(b.pts.iter().map(|p| p.1.abs())).fold(0.0f64, f64::max);
    if big >= 1000.0 { (1000.0, "k") } else { (1.0, "") }
}

/// The diagrams' points: (x, V) and (x, M) in the chosen unit.
fn diagrams(b: &Beam) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let (sc, _) = fscale(b);
    let xs = b.xs();
    let mut vv = vec![];
    let mut mm = vec![];
    for x in &xs {
        let (v, m) = b.vm(*x);
        vv.push((*x, k::snap(v, b.total_load()) / sc));
        mm.push((*x, k::snap(m, b.total_load() * b.l) / sc));
    }
    (vv, mm)
}

/// The largest moment magnitude of the diagram (N·m).
pub fn diagram_mmax(b: &Beam) -> f64 {
    b.xs().iter().map(|x| b.vm(*x).1.abs()).fold(0.0, f64::max)
}

/// The beam drawn at the top of a figure: supports, loads, reactions; `info`: a line of results above it.
fn draw_beam(c: &mut Canvas, b: &Beam, x0: f64, x1: f64, info: &str) {
    let px = |x: f64| x0 + (x1 - x0) * x / b.l;
    let y = 100.0;
    pl::rect(c, x0, y - 5.0, x1 - x0, 10.0);
    // distributed loads: arrows under an outline, heights to one scale
    let wmax = b.dist.iter().map(|d| d.2.abs().max(d.3.abs())).fold(0.0f64, f64::max);
    for (x1d, x2d, w1, w2) in &b.dist {
        if wmax <= 0.0 {
            break;
        }
        let hh = |w: f64| 8.0 + 30.0 * w.abs() / wmax;
        let n = (((px(*x2d) - px(*x1d)) / 26.0).round() as usize).clamp(2, 24);
        let mut top = vec![];
        for i in 0..=n {
            let x = x1d + (x2d - x1d) * i as f64 / n as f64;
            let w = w1 + (w2 - w1) * i as f64 / n as f64;
            let h = hh(w);
            top.push((px(x), y - 6.0 - h));
            if w.abs() > 1e-12 * wmax {
                pl::arrow(c, (px(x), y - 6.0 - h), (px(x), y - 6.0), 2);
            }
        }
        c.wire(&top);
        let lab = if (w1 - w2).abs() <= 1e-9 * wmax { format!("w = {}/m", force(*w1)) } else {
            format!("w = {}/m to {}/m", force(*w1), force(*w2))
        };
        c.value((px(*x1d) + px(*x2d)) / 2.0, y - 46.0 - 8.0, "middle", &lab);
    }
    // point loads
    for (i, (xp, p)) in b.pts.iter().enumerate() {
        let xx = px(*xp);
        let top = if wmax > 0.0 { 34.0 } else { 44.0 };
        if *p >= 0.0 {
            pl::arrow(c, (xx, top), (xx, y - 6.0), 0);
        } else {
            pl::arrow(c, (xx, y - 6.0), (xx, top), 0);
        }
        let ly = top - 6.0 - if i % 2 == 1 { 14.0 } else { 0.0 };
        c.name(xx, ly, "middle", &format!("P = {}", force(p.abs())));
    }
    // supports and reactions
    for (xs_, sup, name) in &b.sups {
        let xx = px(*xs_);
        match sup {
            Sup::Pin => pl::pin(c, (xx, y + 5.0)),
            Sup::Roller => pl::roller(c, (xx, y + 5.0)),
            Sup::Fixed => {
                let left = *xs_ <= b.l / 2.0;
                pl::wall(c, xx, y - 34.0, y + 34.0, left);
            }
        }
        if let Some((_, r)) = b.reac.iter().find(|r| (r.0 - xs_).abs() <= 1e-9 * b.l) {
            let (a, bb) = if *r >= 0.0 { ((xx, 172.0), (xx, 128.0)) } else { ((xx, 128.0), (xx, 172.0)) };
            if r.abs() > 0.0 {
                pl::arrow(c, a, bb, 1);
            }
            let anchor = if xx < x0 + 40.0 { "start" } else if xx > x1 - 40.0 { "end" } else { "middle" };
            let ax = if anchor == "start" { xx + 6.0 } else if anchor == "end" { xx - 6.0 } else { xx };
            c.value(ax, 188.0, anchor, &format!("{name} = {}", force(*r)));
        }
    }
    if b.m_left.abs() > 0.0 {
        c.value(px(0.0) + 10.0, y + 26.0, "start", &format!("MA = {}", moment(b.m_left)));
    }
    if b.m_right.abs() > 0.0 {
        c.value(px(b.l) - 10.0, y + 26.0, "end", &format!("MB = {}", moment(b.m_right)));
    }
    if !info.is_empty() {
        c.bold(16.0, 18.0, "start", info);
    }
}

/// A beam over its shear-force and bending-moment diagrams (560 tall).
pub fn beam_figure(s: &Solved, kind: &str, title: &str, caption: &str, b: &Beam, info: &str) -> Option<Figure> {
    if !(b.l > 0.0) || !b.balanced() {
        return None;
    }
    let (vv, mm) = diagrams(b);
    let (_, pre) = fscale(b);
    let (x0, x1) = (84.0, 494.0);
    let mut c = Canvas::new(566.0);
    draw_beam(&mut c, b, x0, x1, info);
    let vr = pl::range(&vv.iter().map(|p| p.1).chain([0.0]).collect::<Vec<_>>())?;
    let mr = pl::range(&mm.iter().map(|p| p.1).chain([0.0]).collect::<Vec<_>>())?;
    let pv = Plot::new((x0, 214.0, x1, 334.0), (0.0, b.l), vr, (false, false), (false, true), "v");
    let pm = Plot::new((x0, 382.0, x1, 502.0), (0.0, b.l), mr, (false, false), (false, true), "m");
    pv.frame(&mut c, "", &format!("V ({pre}N)"));
    pm.frame(&mut c, "x, position along the beam (m)", &format!("M ({pre}N·m)"));
    for (p, pts, k_) in [(&pv, &vv, 2usize), (&pm, &mm, 0usize)] {
        p.zeros(&mut c, false, true);
        p.open(&mut c);
        p.area(&mut c, pts, if k_ == 2 { "axp-s3" } else { "axp-s" });
        p.path(&mut c, pts, pl::line_class(k_));
        p.close(&mut c);
    }
    c.name(x1, 208.0, "end", "shear force");
    c.name(x1, 376.0, "end", "bending moment");
    // the extremes, labelled
    let unit_v = format!("{pre}N");
    let unit_m = format!("{pre}N·m");
    label_extremes(&mut c, &pv, &vv, &unit_v);
    label_extremes(&mut c, &pm, &mm, &unit_m);
    Some(s.figure(kind, title, caption, c.body(), c.h))
}

fn label_extremes(c: &mut Canvas, p: &Plot, pts: &[(f64, f64)], unit: &str) {
    let Some(hi) = pts.iter().copied().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) else { return };
    let Some(lo) = pts.iter().copied().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) else { return };
    let span = (hi.1 - lo.1).abs().max(1e-300);
    for (pt, up) in [(hi, true), (lo, false)] {
        if pt.1.abs() <= 5e-3 * span {
            continue;
        }
        p.dot(c, pt, "axp-d", 3.5);
        let (x, y) = p.pt(pt);
        // beside the point: to its right, or to its left near the right end
        let (tx, anchor) = if x > p.x1 - 110.0 { (x - 7.0, "end") } else { (x + 7.0, "start") };
        let ty = if up { y - 7.0 } else { y + 16.0 };
        c.value(tx, ty.clamp(p.y0 + 13.0, p.y1 - 4.0), anchor, &format!("{} {unit}", num(pt.1)));
        if (hi.1 - lo.1).abs() <= 1e-12 {
            break;
        }
    }
}

/// x positions and loads as a list of "a at b".
fn loads_text(b: &Beam) -> String {
    b.pts.iter().map(|(x, p)| format!("{} at x = {}", force(*p), q(*x, "m"))).collect::<Vec<_>>().join(", ")
}

/// Check the diagram's largest moment against the model's own (when returned).
fn agrees(b: &Beam, m_model: Option<f64>) -> bool {
    match m_model {
        Some(m) => (diagram_mmax(b) - m.abs()).abs() <= 2e-3 * m.abs().max(1e-300),
        None => true,
    }
}

// ------------------------------------------------------------------------------------------------ the capabilities
pub fn ss_point(s: &Solved) -> Option<Figure> {
    let (l, p, a) = (s.inp("L")?, s.inp("P")?, s.inp("a")?);
    let (ra, rb, mmax) = (s.out("RA")?, s.out("RB")?, s.out("M_max")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Pin, "RA".into()), (l, Sup::Roller, "RB".into())], pts: vec![(a, p)], dist: vec![],
                   reac: vec![(0.0, ra), (l, rb)], m_left: 0.0, m_right: 0.0 };
    if !agrees(&b, Some(mmax)) {
        return None;
    }
    let dmax = s.out("max_deflection_mm");
    let info = dmax.map(|d| format!("max deflection {} mm", num(d))).unwrap_or_default();
    beam_figure(s, "beam", "Simply supported beam with a point load",
                &format!("Simply supported beam, span L = {}, point load P = {} at a = {}, drawn to scale with its \
                          shear-force and bending-moment diagrams, derived by statics from the reactions the model \
                          returned. Marked: RA = {}, RB = {}, Mmax = {}{}.", q(l, "m"), force(p), q(a, "m"), force(ra),
                         force(rb), moment(mmax), dmax.map(|d| format!(", maximum deflection {} mm", num(d))).unwrap_or_default()),
                &b, &info)
}

pub fn ss_udl(s: &Solved) -> Option<Figure> {
    let (l, w) = (s.inp("L")?, s.inp("w")?);
    let (ra, rb, mmax) = (s.out("RA")?, s.out("RB")?, s.out("M_max")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Pin, "RA".into()), (l, Sup::Roller, "RB".into())], pts: vec![],
                   dist: vec![(0.0, l, w, w)], reac: vec![(0.0, ra), (l, rb)], m_left: 0.0, m_right: 0.0 };
    if !agrees(&b, Some(mmax)) {
        return None;
    }
    let dmax = s.out("max_deflection_mm");
    beam_figure(s, "beam", "Simply supported beam under a uniform load",
                &format!("Simply supported beam, span L = {}, uniform load w = {}/m, with its shear-force and \
                          bending-moment diagrams derived by statics from the returned reactions. Marked: RA = {}, RB = {}, \
                          Mmax = {}{}.", q(l, "m"), force(w), force(ra), force(rb), moment(mmax),
                         dmax.map(|d| format!(", maximum deflection {} mm at midspan", num(d))).unwrap_or_default()),
                &b, &dmax.map(|d| format!("max deflection {} mm", num(d))).unwrap_or_default())
}

/// simply_supported_beam_point_loads: loads in kN at positions in m (the model's own units); its reactions in kN.
fn ss_loads_beam(s: &Solved) -> Option<(Beam, f64, f64)> {
    let l = s.inp("L")?;
    let (xs, ps) = (s.vec("x")?, s.vec("P")?);
    if xs.len() != ps.len() || xs.is_empty() || xs.len() > 12 {
        return None;
    }
    let (ra, rb, mmax, xm) = (s.out("RA")?, s.out("RB")?, s.out("M_max")?, s.out("x_Mmax")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Pin, "RA".into()), (l, Sup::Roller, "RB".into())],
                   pts: xs.iter().zip(&ps).map(|(x, p)| (*x, p * 1000.0)).collect(), dist: vec![],
                   reac: vec![(0.0, ra * 1000.0), (l, rb * 1000.0)], m_left: 0.0, m_right: 0.0 };
    if !agrees(&b, Some(mmax * 1000.0)) {
        return None;
    }
    Some((b, mmax, xm))
}

pub fn ss_loads_full(s: &Solved) -> Option<Figure> {
    let (b, mmax, xm) = ss_loads_beam(s)?;
    beam_figure(s, "beam-2", "Beam with several point loads",
                &format!("Simply supported beam, span L = {}, point loads {}, to scale, over its shear-force and \
                          bending-moment diagrams derived by statics from the returned reactions. Marked: RA = {}, RB = {}, \
                          Mmax = {} at x = {}.", q(b.l, "m"), loads_text(&b), force(b.reac[0].1), force(b.reac[1].1),
                         moment(mmax * 1000.0), q(xm, "m")),
                &b, &format!("Mmax = {} at x = {}", moment(mmax * 1000.0), q(xm, "m")))
}

pub fn ss_loads_chart(s: &Solved) -> Option<Figure> {
    let (b, mmax, xm) = ss_loads_beam(s)?;
    let (vv, mm) = diagrams(&b);
    let (sc, pre) = fscale(&b);
    let series = vec![pl::curve(&format!("bending moment M ({pre}N·m)"), mm, 0),
                      pl::curve(&format!("shear force V ({pre}N)"), vv, 2)];
    let body = pl::chart(&series, "x, position along the span (m)", &format!("V ({pre}N), M ({pre}N·m)"),
                         Some(((xm, mmax * 1000.0 / sc), format!("Mmax = {}", moment(mmax * 1000.0)))), (false, false),
                         true)?;
    Some(s.chart("beam", "Shear and bending-moment diagrams of a beam",
                 &format!("Shear-force and bending-moment diagrams of a simply supported beam of span {} with point \
                           loads {}, drawn from the reactions the model returned (RA = {}, RB = {}); marked: the maximum \
                           moment it returned, x = {}, M = {}.", q(b.l, "m"), loads_text(&b), force(b.reac[0].1),
                          force(b.reac[1].1), q(xm, "m"), moment(mmax * 1000.0)), &body))
}

pub fn cantilever_point(s: &Solved) -> Option<Figure> {
    let (l, p) = (s.inp("L")?, s.inp("P")?);
    let a = s.inp("a").unwrap_or(l);
    let (r, mf) = (s.out("R")?, s.out("M_fixed")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "R".into())], pts: vec![(a, p)], dist: vec![], reac: vec![(0.0, r)],
                   m_left: mf, m_right: 0.0 };
    let d = s.out("deflection_at_load_mm");
    beam_figure(s, "beam-2", "Cantilever with a point load",
                &format!("Cantilever of length L = {} fixed at x = 0 carrying a point load P = {} at x = {}, with its \
                          shear-force and bending-moment diagrams derived by statics from the returned support reaction and \
                          fixing moment. Marked: R = {}, M = {} at the wall{}.", q(l, "m"), force(p), q(a, "m"), force(r),
                         moment(mf), d.map(|d| format!(", deflection under the load {} mm", num(d))).unwrap_or_default()),
                &b, &s.out("tip_deflection_mm").map(|d| format!("tip deflection {} mm", num(d))).unwrap_or_default())
}

/// A cantilever's deflected shape under its load at the tip: by the reciprocal theorem the deflection at x is the tip
/// deflection the model returns for the load placed at x.
pub fn cantilever_shape(s: &Solved) -> Option<Figure> {
    let (l, p) = (s.inp("L")?, s.inp("P")?);
    let a = s.inp("a").unwrap_or(l);
    if (a - l).abs() > 1e-9 * l {
        // the reciprocal reading gives the shape under a tip load only: a load elsewhere is not drawn this way
        return None;
    }
    let xs = k::span(0.0, l, 31, false);
    let mut pts = vec![(0.0, 0.0)];
    for x in xs.iter().skip(1) {
        let b = s.with(&[("a", *x)])?;
        pts.push((*x, -k::num_of(&b, "tip_deflection_mm")?));
    }
    let tip = -pts.last()?.1;
    let mut un = pl::curve("unloaded beam", vec![(0.0, 0.0), (l, 0.0)], 3);
    un.dashed = true;
    let series = vec![un, pl::curve(&format!("deflected beam, P = {} at the tip", force(p)), pts, 0)];
    let body = pl::chart(&series, "x, distance from the fixed end (m)", "deflection (mm)",
                         Some(((l, -tip), format!("tip: {} mm", num(tip)))), (false, false), true)?;
    Some(s.chart("beam", "A cantilever's deflected shape under a tip load",
                 &format!("Deflected shape of a cantilever of length L = {} under a tip load P = {}: the deflection at each \
                           of 31 positions x is the tip deflection the model returns for the load placed at x (the \
                           reciprocal theorem). Marked: x = {}, tip deflection = {} mm.", q(l, "m"), force(p), q(l, "m"),
                          num(tip)), &body))
}

pub fn cantilever_tri(s: &Solved) -> Option<Figure> {
    let (l, w) = (s.inp("L")?, s.inp("w")?);
    let (r, mf) = (s.out("R")?, s.out("M_fixed")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "R".into())], pts: vec![], dist: vec![(0.0, l, w, 0.0)],
                   reac: vec![(0.0, r)], m_left: mf, m_right: 0.0 };
    let d = s.out("tip_deflection_mm");
    beam_figure(s, "beam", "Cantilever under a triangular load",
                &format!("Cantilever of length L = {} fixed at x = 0 carrying a triangular load falling from w = {}/m at \
                          the support to zero at the free end, with its shear-force and bending-moment diagrams derived by \
                          statics from the returned support reaction and fixing moment. Marked: R = {}, M = {} at the \
                          wall{}.", q(l, "m"), force(w), force(r), moment(mf),
                         d.map(|d| format!(", tip deflection {} mm", num(d))).unwrap_or_default()),
                &b, &d.map(|d| format!("tip deflection {} mm", num(d))).unwrap_or_default())
}

pub fn cantilever_udl(s: &Solved) -> Option<Figure> {
    let (l, w) = (s.inp("L")?, s.inp("w")?);
    let (r, mf) = (s.out("R")?, s.out("M_fixed")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "R".into())], pts: vec![], dist: vec![(0.0, l, w, w)],
                   reac: vec![(0.0, r)], m_left: mf, m_right: 0.0 };
    let d = s.out("tip_deflection_mm");
    beam_figure(s, "beam", "Cantilever under a uniform load",
                &format!("Cantilever of length L = {} fixed at x = 0 carrying a uniform load w = {}/m, with its \
                          shear-force and bending-moment diagrams derived by statics from the returned support reaction and \
                          fixing moment. Marked: R = {}, M = {} at the wall{}.", q(l, "m"), force(w), force(r), moment(mf),
                         d.map(|d| format!(", tip deflection {} mm", num(d))).unwrap_or_default()),
                &b, &d.map(|d| format!("tip deflection {} mm", num(d))).unwrap_or_default())
}

pub fn ff_point(s: &Solved) -> Option<Figure> {
    let (l, p, a) = (s.inp("L")?, s.inp("P")?, s.inp("a")?);
    let (ra, rb, ma, mb, ml) = (s.out("RA")?, s.out("RB")?, s.out("MA")?, s.out("MB")?, s.out("M_load")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "RA".into()), (l, Sup::Fixed, "RB".into())], pts: vec![(a, p)],
                   dist: vec![], reac: vec![(0.0, ra), (l, rb)], m_left: ma, m_right: mb };
    if !agrees(&b, Some(ma.abs().max(mb.abs()).max(ml.abs()))) {
        return None;
    }
    beam_figure(s, "beam", "Fixed-fixed beam with a point load",
                &format!("Beam built in at both ends, span L = {}, point load P = {} at a = {}, with shear-force and \
                          bending-moment diagrams derived by statics from the returned reactions and fixing moments. \
                          Marked: RA = {}, RB = {}, MA = {}, MB = {} (hogging), M under the load = {}.", q(l, "m"), force(p),
                         q(a, "m"), force(ra), force(rb), moment(ma), moment(mb), moment(ml)),
                &b, &s.out("deflection_at_load_mm").map(|d| format!("deflection under the load {} mm", num(d)))
                    .unwrap_or_default())
}

pub fn ff_udl(s: &Solved) -> Option<Figure> {
    let (l, w) = (s.inp("L")?, s.inp("w")?);
    let (ra, rb, me, mmid) = (s.out("RA")?, s.out("RB")?, s.out("M_end")?, s.out("M_mid")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "RA".into()), (l, Sup::Fixed, "RB".into())], pts: vec![],
                   dist: vec![(0.0, l, w, w)], reac: vec![(0.0, ra), (l, rb)], m_left: me, m_right: me };
    if !agrees(&b, Some(me.abs().max(mmid.abs()))) {
        return None;
    }
    let d = s.out("max_deflection_mm");
    beam_figure(s, "beam", "Fixed-fixed beam under a uniform load",
                &format!("Beam built in at both ends, span L = {}, uniform load w = {}/m, with shear-force and \
                          bending-moment diagrams derived by statics from the returned reactions and end moments. Marked: \
                          RA = {}, RB = {}, end moments {} (hogging), midspan moment {}{}.", q(l, "m"), force(w), force(ra),
                         force(rb), moment(me), moment(mmid),
                         d.map(|d| format!(", maximum deflection {} mm", num(d))).unwrap_or_default()),
                &b, &d.map(|d| format!("max deflection {} mm", num(d))).unwrap_or_default())
}

pub fn pc_point(s: &Solved) -> Option<Figure> {
    let (l, p, a) = (s.inp("L")?, s.inp("P")?, s.inp("a")?);
    let (ra, rb, ma, ml) = (s.out("RA")?, s.out("RB")?, s.out("MA")?, s.out("M_load")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "RA".into()), (l, Sup::Roller, "RB".into())], pts: vec![(a, p)],
                   dist: vec![], reac: vec![(0.0, ra), (l, rb)], m_left: ma, m_right: 0.0 };
    if !agrees(&b, Some(ma.abs().max(ml.abs()))) {
        return None;
    }
    let d = s.out("deflection_at_load_mm");
    beam_figure(s, "beam", "Propped cantilever with a point load",
                &format!("Propped cantilever (fixed at A, x = 0; roller at B, x = {}) with a point load P = {} at a = {}, \
                          with shear-force and bending-moment diagrams derived by statics from the returned reactions. \
                          Marked: RA = {}, RB = {}, MA = {} (hogging), M under the load = {}{}.", q(l, "m"), force(p),
                         q(a, "m"), force(ra), force(rb), moment(ma), moment(ml),
                         d.map(|d| format!(", deflection there {} mm", num(d))).unwrap_or_default()),
                &b, &d.map(|d| format!("deflection under the load {} mm", num(d))).unwrap_or_default())
}

pub fn pc_udl(s: &Solved) -> Option<Figure> {
    let (l, w) = (s.inp("L")?, s.inp("w")?);
    let (ra, rb, ma, ms, xm) = (s.out("RA")?, s.out("RB")?, s.out("MA")?, s.out("M_span")?, s.out("x_M_span")?);
    let b = Beam { l, sups: vec![(0.0, Sup::Fixed, "RA".into()), (l, Sup::Roller, "RB".into())], pts: vec![],
                   dist: vec![(0.0, l, w, w)], reac: vec![(0.0, ra), (l, rb)], m_left: ma, m_right: 0.0 };
    if !agrees(&b, Some(ma.abs().max(ms.abs()))) {
        return None;
    }
    let d = s.out("max_deflection_mm");
    beam_figure(s, "beam", "Propped cantilever under a uniform load",
                &format!("Propped cantilever (fixed at x = 0, roller at x = {}) under a uniform load w = {}/m, with \
                          shear-force and bending-moment diagrams derived by statics from the returned reactions. Marked: \
                          RA = {}, RB = {}, MA = {} (hogging), largest span moment {} at x = {}{}.", q(l, "m"), force(w),
                         force(ra), force(rb), moment(ma), moment(ms), q(xm, "m"),
                         d.map(|d| format!(", maximum deflection {} mm", num(d))).unwrap_or_default()),
                &b, &d.map(|d| format!("max deflection {} mm", num(d))).unwrap_or_default())
}

/// continuous_beam_three_moment: spans in m and loads in kN/m (the model's own units), reactions in kN.
pub fn continuous(s: &Solved) -> Option<Figure> {
    let (spans, w) = (s.vec("spans")?, s.vec("w")?);
    let (mo, re) = (s.out_list("support_moments_kNm")?, s.out_list("reactions_kN")?);
    if spans.len() != w.len() || spans.is_empty() || spans.len() > 8 || re.len() != spans.len() + 1
        || spans.iter().any(|x| !(*x > 0.0)) {
        return None;
    }
    let mut xs = vec![0.0];
    for sp in &spans {
        xs.push(xs.last()? + sp);
    }
    let l = *xs.last()?;
    let mut dist = vec![];
    for i in 0..spans.len() {
        dist.push((xs[i], xs[i + 1], w[i] * 1000.0, w[i] * 1000.0));
    }
    let names = ["R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9"];
    let sups: Vec<(f64, Sup, String)> = xs.iter().enumerate()
        .map(|(i, x)| (*x, if i == 0 { Sup::Pin } else { Sup::Roller }, names[i].to_string())).collect();
    let b = Beam { l, sups, pts: vec![], dist, reac: xs.iter().zip(&re).map(|(x, r)| (*x, r * 1000.0)).collect(),
                   m_left: 0.0, m_right: 0.0 };
    // the diagram's moment over each inner support is the model's support moment
    for (i, x) in xs.iter().enumerate().skip(1).take(spans.len() - 1) {
        let m = b.vm(*x).1 / 1000.0;
        if (m - mo[i]).abs() > 1e-3 * mo.iter().map(|v| v.abs()).fold(1e-9, f64::max) {
            return None;
        }
    }
    beam_figure(s, "beam", "Continuous beam: reactions, shear and moment",
                &format!("Continuous beam over {} supports, spans {} m, uniform loads {} kN/m per span, with shear-force \
                          and bending-moment diagrams derived by statics from the returned reactions. Marked: reactions {} \
                          kN and support moments {} kN·m (three-moment equation).", xs.len(),
                         spans.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "),
                         w.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "),
                         re.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "),
                         mo.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", ")),
                &b, &format!("support moments {} kN·m", mo.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", ")))
}
