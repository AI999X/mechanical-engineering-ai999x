//! Cross-sections drawn to scale from their stated dimensions, with the centroid and axes the model returned and its
//! section properties listed beside them.

use crate::figures_plot::{self as pl};
use ax_engine::figkit::{Canvas, Solved, P};
use ax_engine::figures::Figure;

/// One outline of a part: a polygon or a circle (section coordinates, y up).
pub enum Sub {
    Poly(Vec<(f64, f64)>),
    Circle((f64, f64), f64),
}

fn bbox(parts: &[Vec<Sub>]) -> Option<(f64, f64, f64, f64)> {
    let mut xs = vec![];
    let mut ys = vec![];
    for p in parts {
        for s in p {
            match s {
                Sub::Poly(v) => {
                    for q in v {
                        xs.push(q.0);
                        ys.push(q.1);
                    }
                }
                Sub::Circle(c, r) => {
                    xs.extend([c.0 - r, c.0 + r]);
                    ys.extend([c.1 - r, c.1 + r]);
                }
            }
        }
    }
    let (x0, x1) = pl::range(&xs)?;
    let (y0, y1) = pl::range(&ys)?;
    (x1 > x0 && y1 > y0).then_some((x0, y0, x1, y1))
}

/// The mapping of section coordinates into the drawing box (keeping the shape's proportions).
pub struct Map {
    sc: f64,
    ox: f64,
    oy: f64,
}

impl Map {
    pub fn p(&self, q: (f64, f64)) -> P {
        (self.ox + q.0 * self.sc, self.oy - q.1 * self.sc)
    }
}

/// Draw the parts into the box (x0, y0, x1, y1) of the canvas; returns the mapping.
pub fn draw_parts(c: &mut Canvas, parts: &[Vec<Sub>], bx: (f64, f64, f64, f64)) -> Option<Map> {
    let (a0, b0, a1, b1) = bbox(parts)?;
    let sc = ((bx.2 - bx.0) / (a1 - a0)).min((bx.3 - bx.1) / (b1 - b0));
    let ox = (bx.0 + bx.2) / 2.0 - (a0 + a1) / 2.0 * sc;
    let oy = (bx.1 + bx.3) / 2.0 + (b0 + b1) / 2.0 * sc;
    let m = Map { sc, ox, oy };
    for part in parts {
        let mut d = String::new();
        for s in part {
            match s {
                Sub::Poly(v) => {
                    for (i, q) in v.iter().enumerate() {
                        let p = m.p(*q);
                        d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
                    }
                    d.push_str("Z ");
                }
                Sub::Circle(ctr, r) => {
                    let p = m.p(*ctr);
                    let rr = r * sc;
                    d.push_str(&format!("M{:.1},{:.1} A{rr:.1},{rr:.1} 0 1 0 {:.1},{:.1} A{rr:.1},{rr:.1} 0 1 0 {:.1},{:.1} Z ",
                                        p.0 - rr, p.1, p.0 + rr, p.1, p.0 - rr, p.1));
                }
            }
        }
        let d = d.trim_end();
        c.raw(&format!("<path class=\"axp-ka\" fill-rule=\"evenodd\" d=\"{d}\"/><path class=\"axp-k\" d=\"{d}\"/>"));
    }
    Some(m)
}

/// The centroidal axes (dashed, named x and y) and the centroid (a dot) at q.
fn axes(c: &mut Canvas, m: &Map, parts: &[Vec<Sub>], q: (f64, f64)) {
    let Some((a0, b0, a1, b1)) = bbox(parts) else { return };
    let p = m.p(q);
    let (l, r) = (m.p((a0, 0.0)).0 - 24.0, m.p((a1, 0.0)).0 + 24.0);
    let (t, b) = (m.p((0.0, b1)).1 - 24.0, m.p((0.0, b0)).1 + 24.0);
    c.dashed((l, p.1), (r, p.1));
    c.dashed((p.0, t), (p.0, b));
    c.name(r + 6.0, p.1 + 4.0, "start", "x");
    c.name(p.0 + 6.0, t + 10.0, "start", "y");
    c.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", p.0, p.1));
}

/// A section figure: the parts, the centroid and axes, the listed values (right).
fn section_figure(s: &Solved, title: &str, caption: &str, parts: Vec<Vec<Sub>>, centroid: (f64, f64),
                  principal_deg: Option<f64>, values: &[String], numbered: bool) -> Option<Figure> {
    let mut c = Canvas::new(360.0);
    let m = draw_parts(&mut c, &parts, (50.0, 50.0, 290.0, 320.0))?;
    if numbered && parts.len() > 1 {
        for (i, part) in parts.iter().enumerate() {
            if let Some(Sub::Poly(v)) = part.first() {
                let n = v.len() as f64;
                let cx = v.iter().map(|q| q.0).sum::<f64>() / n;
                let cy = v.iter().map(|q| q.1).sum::<f64>() / n;
                let p = m.p((cx, cy));
                c.name(p.0, p.1 + 4.0, "middle", &format!("{}", i + 1));
            }
        }
    }
    axes(&mut c, &m, &parts, centroid);
    if let Some(th) = principal_deg {
        let p = m.p(centroid);
        for (k, a) in [(1, th), (2, th + 90.0)] {
            let r = a.to_radians();
            let u = (r.cos(), -r.sin());
            let len = 125.0;
            c.raw(&format!("<path class=\"axp-c3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p.0 - u.0 * len, p.1 - u.1 * len,
                           p.0 + u.0 * len, p.1 + u.1 * len));
            c.name(p.0 + u.0 * (len + 10.0), p.1 + u.1 * (len + 10.0) + 4.0, "middle", &format!("{k}"));
        }
        c.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", p.0, p.1));
    }
    for (i, v) in values.iter().enumerate() {
        c.value(330.0, 62.0 + 19.0 * i as f64, "start", v);
    }
    Some(s.figure("section", title, caption, c.body(), c.h))
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> Sub {
    Sub::Poly(vec![(x, y), (x + w, y), (x + w, y + h), (x, y + h)])
}
fn a2(x: f64) -> String {
    format!("{} mm²", pl::nn(x * 1e6))
}
fn i4(x: f64) -> String {
    format!("{} mm⁴", pl::nn(x * 1e12))
}
fn z3(x: f64) -> String {
    format!("{} mm³", pl::nn(x * 1e9))
}
fn l1(x: f64) -> String {
    format!("{} mm", pl::nn(x * 1e3))
}

pub fn rectangular(s: &Solved) -> Option<Figure> {
    let (b, h) = (s.inp("b")?, s.inp("h")?);
    let o = |k: &str| s.out(k);
    let v = vec![format!("A = {}", a2(o("A")?)), format!("Ix = {}", i4(o("Ix")?)), format!("Iy = {}", i4(o("Iy")?)),
                 format!("Zx = {}", z3(o("Zx")?)), format!("Zy = {}", z3(o("Zy")?)), format!("rx = {}", l1(o("rx")?)),
                 format!("ry = {}", l1(o("ry")?)), format!("J = {}", i4(o("J_polar")?))];
    section_figure(s, "Rectangular section: area and second moments",
                   &format!("Rectangular section b = {} by h = {} drawn to scale with its centroid and centroidal axes (at \
                             mid-width and mid-depth). Marked: the returned {}.", l1(b), l1(h), v.join(", ")),
                   vec![vec![rect(-b / 2.0, -h / 2.0, b, h)]], (0.0, 0.0), None, &v, false)
}

pub fn circular(s: &Solved) -> Option<Figure> {
    let d = s.inp("d")?;
    let o = |k: &str| s.out(k);
    let v = vec![format!("A = {}", a2(o("A")?)), format!("I = {}", i4(o("I")?)), format!("J = {}", i4(o("J")?)),
                 format!("Z = {}", z3(o("Z")?)), format!("Zp = {}", z3(o("Zp")?)), format!("r = {}", l1(o("r")?))];
    section_figure(s, "Circular section properties",
                   &format!("Solid circular section of diameter d = {} drawn to scale with its centroid. Marked: the \
                             returned {}.", l1(d), v.join(", ")),
                   vec![vec![Sub::Circle((0.0, 0.0), d / 2.0)]], (0.0, 0.0), None, &v, false)
}

pub fn hollow_circular(s: &Solved) -> Option<Figure> {
    let (dd, d) = (s.inp("D")?, s.inp("d")?);
    if !(d >= 0.0 && d < dd) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let v = vec![format!("A = {}", a2(o("A")?)), format!("I = {}", i4(o("I")?)), format!("J = {}", i4(o("J")?)),
                 format!("Z = {}", z3(o("Z")?)), format!("Zp = {}", z3(o("Zp")?)), format!("r = {}", l1(o("r")?))];
    let mut part = vec![Sub::Circle((0.0, 0.0), dd / 2.0)];
    if d > 0.0 {
        part.push(Sub::Circle((0.0, 0.0), d / 2.0));
    }
    section_figure(s, "Hollow circular section properties",
                   &format!("Tube section, outer diameter D = {}, inner diameter d = {}, to scale. Marked: the returned {}.",
                            l1(dd), l1(d), v.join(", ")),
                   vec![part], (0.0, 0.0), None, &v, false)
}

pub fn hollow_rectangular(s: &Solved) -> Option<Figure> {
    let (bb, hh, t) = (s.inp("B")?, s.inp("H")?, s.inp("t")?);
    if !(t > 0.0 && 2.0 * t < bb && 2.0 * t < hh) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let v = vec![format!("A = {}", a2(o("A")?)), format!("Ix = {}", i4(o("Ix")?)), format!("Iy = {}", i4(o("Iy")?)),
                 format!("Zx = {}", z3(o("Zx")?)), format!("Zy = {}", z3(o("Zy")?)), format!("Zpx = {}", z3(o("Zpx")?)),
                 format!("rx = {}", l1(o("rx")?)), format!("ry = {}", l1(o("ry")?))];
    section_figure(s, "Hollow rectangular section properties",
                   &format!("Hollow rectangular section B = {} by H = {}, wall t = {}, to scale with its centroid. Marked: \
                             the returned {}.", l1(bb), l1(hh), l1(t), v.join(", ")),
                   vec![vec![rect(-bb / 2.0, -hh / 2.0, bb, hh), rect(-bb / 2.0 + t, -hh / 2.0 + t, bb - 2.0 * t, hh - 2.0 * t)]],
                   (0.0, 0.0), None, &v, false)
}

pub fn i_section(s: &Solved) -> Option<Figure> {
    let (b, h, tf, tw) = (s.inp("b")?, s.inp("h")?, s.inp("tf")?, s.inp("tw")?);
    if !(2.0 * tf < h && tw < b && tf > 0.0 && tw > 0.0) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let v = vec![format!("A = {}", a2(o("A")?)), format!("Ix = {}", i4(o("Ix")?)), format!("Iy = {}", i4(o("Iy")?)),
                 format!("Zx = {}", z3(o("Zx")?)), format!("Zpx = {}", z3(o("Zpx")?)), format!("rx = {}", l1(o("rx")?)),
                 format!("ry = {}", l1(o("ry")?)), format!("shape factor {}", pl::nn(o("shape_factor_x")?))];
    let (bh, wh) = (b / 2.0, tw / 2.0);
    let poly = vec![(-bh, 0.0), (bh, 0.0), (bh, tf), (wh, tf), (wh, h - tf), (bh, h - tf), (bh, h), (-bh, h), (-bh, h - tf),
                    (-wh, h - tf), (-wh, tf), (-bh, tf)];
    section_figure(s, "I-section properties",
                   &format!("I-section (flanges b = {} by tf = {}, overall depth h = {}, web tw = {}) drawn to scale with \
                             its centroid at mid-depth. Marked: the returned {}.", l1(b), l1(tf), l1(h), l1(tw), v.join(", ")),
                   vec![vec![Sub::Poly(poly)]], (0.0, h / 2.0), None, &v, false)
}

pub fn channel(s: &Solved) -> Option<Figure> {
    let (b, h, tf, tw) = (s.inp("b")?, s.inp("h")?, s.inp("tf")?, s.inp("tw")?);
    if !(2.0 * tf < h && tw < b && tf > 0.0 && tw > 0.0) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let xb = o("x_bar")?;
    let v = vec![format!("A = {}", a2(o("A")?)), format!("x̄ = {}", l1(xb)), format!("Ix = {}", i4(o("Ix")?)),
                 format!("Iy = {}", i4(o("Iy")?)), format!("Zx = {}", z3(o("Zx")?)), format!("Zy,min = {}", z3(o("Zy_min")?)),
                 format!("rx = {}", l1(o("rx")?)), format!("ry = {}", l1(o("ry")?))];
    let poly = vec![(0.0, 0.0), (b, 0.0), (b, tf), (tw, tf), (tw, h - tf), (b, h - tf), (b, h), (0.0, h)];
    section_figure(s, "Channel section: centroid and second moments",
                   &format!("Channel section (flanges b = {} by tf = {}, depth h = {}, web tw = {}) to scale; the centroid \
                             is placed at the returned x̄ = {} from the back of the web, at mid-depth. Marked: {}.", l1(b),
                            l1(tf), l1(h), l1(tw), l1(xb), v.join(", ")),
                   vec![vec![Sub::Poly(poly)]], (xb, h / 2.0), None, &v, false)
}

pub fn t_section(s: &Solved) -> Option<Figure> {
    let (bf, tf, h, tw) = (s.inp("bf")?, s.inp("tf")?, s.inp("h")?, s.inp("tw")?);
    if !(tf < h && tw < bf && tf > 0.0 && tw > 0.0) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let yb = o("y_bar")?;
    let v = vec![format!("A = {}", a2(o("A")?)), format!("ȳ = {}", l1(yb)), format!("Ix = {}", i4(o("Ix")?)),
                 format!("Iy = {}", i4(o("Iy")?)), format!("Z_top = {}", z3(o("Z_top")?)),
                 format!("Z_bottom = {}", z3(o("Z_bottom")?)), format!("rx = {}", l1(o("rx")?)),
                 format!("ry = {}", l1(o("ry")?))];
    let (fb, wb) = (bf / 2.0, tw / 2.0);
    let poly = vec![(-wb, 0.0), (wb, 0.0), (wb, h - tf), (fb, h - tf), (fb, h), (-fb, h), (-fb, h - tf), (-wb, h - tf)];
    section_figure(s, "T-section: centroid and second moments",
                   &format!("T-section (flange {} by {}, overall depth h = {}, stem {}) drawn to scale; the centroid is \
                             placed at the returned ȳ = {} above the bottom of the stem. Marked: {}.", l1(bf), l1(tf), l1(h),
                            l1(tw), l1(yb), v.join(", ")),
                   vec![vec![Sub::Poly(poly)]], (0.0, yb), None, &v, false)
}

pub fn angle(s: &Solved) -> Option<Figure> {
    let (b, h, t) = (s.inp("b")?, s.inp("h")?, s.inp("t")?);
    if !(t > 0.0 && t < b && t < h) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let (xb, yb, th) = (o("x_bar")?, o("y_bar")?, o("principal_angle_deg")?);
    let v = vec![format!("A = {}", a2(o("A")?)), format!("x̄ = {}", l1(xb)), format!("ȳ = {}", l1(yb)), format!("Ix = {}", i4(o("Ix")?)),
                 format!("Iy = {}", i4(o("Iy")?)), format!("Ixy = {}", i4(o("Ixy")?)), format!("I1 = {}", i4(o("I1")?)),
                 format!("I2 = {}", i4(o("I2")?)), format!("θp = {}°", pl::nn(th))];
    let poly = vec![(0.0, 0.0), (b, 0.0), (b, t), (t, t), (t, h), (0.0, h)];
    section_figure(s, "Angle section: centroid and principal axes",
                   &format!("Angle section, legs b = {} and h = {}, thickness t = {}, drawn to scale with the returned \
                             centroid (x̄ = {}, ȳ = {} from the heel), the centroidal x and y axes and the principal axes 1 \
                             and 2 at the returned angle θp = {}°. Marked: {}.", l1(b), l1(h), l1(t), l1(xb), l1(yb), pl::nn(th),
                            v.join(", ")),
                   vec![vec![Sub::Poly(poly)]], (xb, yb), Some(th), &v, false)
}

/// composite_section_properties: rectangles (width, height, centre x, centre y) in any length unit; the results in
/// that unit.
pub fn composite(s: &Solved) -> Option<Figure> {
    let rects = s.mat("rects")?;
    if rects.is_empty() || rects.len() > 12 || rects.iter().any(|r| r.len() < 4 || !(r[0] > 0.0 && r[1] > 0.0)) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let (xb, yb) = (o("x_bar")?, o("y_bar")?);
    let v = vec![format!("A = {}", pl::nn(o("A")?)), format!("x̄ = {}", pl::nn(xb)), format!("ȳ = {}", pl::nn(yb)), format!("Ix = {}", pl::nn(o("Ix")?)),
                 format!("Iy = {}", pl::nn(o("Iy")?)), format!("Ixy = {}", pl::nn(o("Ixy")?)), format!("I1 = {}", pl::nn(o("I1")?)),
                 format!("I2 = {}", pl::nn(o("I2")?))];
    let parts: Vec<Vec<Sub>> = rects.iter().map(|r| vec![rect(r[2] - r[0] / 2.0, r[3] - r[1] / 2.0, r[0], r[1])]).collect();
    let listed = rects.iter().map(|r| r.iter().take(4).map(|x| pl::nn(*x)).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>().join("; ");
    section_figure(s, "Composite section: centroid and second moments",
                   &format!("Composite section of {} rectangles (width, height, centre x, centre y: {}) drawn to scale and \
                             numbered, with the returned centroid (x̄ = {}, ȳ = {}) and centroidal axes. Marked: {} (units of \
                             the dimensions).", rects.len(), listed, pl::nn(xb), pl::nn(yb), v.join(", ")),
                   parts, (xb, yb), None, &v, true)
}

pub fn polygon(s: &Solved) -> Option<Figure> {
    let vs = s.mat("vertices")?;
    if vs.len() < 3 || vs.len() > 40 || vs.iter().any(|r| r.len() < 2) {
        return None;
    }
    let o = |k: &str| s.out(k);
    let (xb, yb) = (o("x_bar")?, o("y_bar")?);
    let v = vec![format!("A = {}", pl::nn(o("A")?)), format!("x̄ = {}", pl::nn(xb)), format!("ȳ = {}", pl::nn(yb)), format!("Ix = {}", pl::nn(o("Ix")?)),
                 format!("Iy = {}", pl::nn(o("Iy")?)), format!("Ixy = {}", pl::nn(o("Ixy")?)), format!("I1 = {}", pl::nn(o("I1")?)),
                 format!("I2 = {}", pl::nn(o("I2")?))];
    let pts: Vec<(f64, f64)> = vs.iter().map(|r| (r[0], r[1])).collect();
    let mut c = Canvas::new(360.0);
    let parts = vec![vec![Sub::Poly(pts.clone())]];
    let m = draw_parts(&mut c, &parts, (50.0, 50.0, 290.0, 320.0))?;
    axes(&mut c, &m, &parts, (xb, yb));
    if pts.len() <= 16 {
        let (cx, cy) = m.p((xb, yb));
        for (i, q) in pts.iter().enumerate() {
            let p = m.p(*q);
            let (dx, dy) = (p.0 - cx, p.1 - cy);
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            c.raw(&format!("<circle class=\"axp-kd\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\"/>", p.0, p.1));
            c.name(p.0 + dx / d * 14.0, p.1 + dy / d * 14.0 + 4.0, "middle", &format!("{}", i + 1));
        }
    }
    for (i, t) in v.iter().enumerate() {
        c.value(330.0, 62.0 + 19.0 * i as f64, "start", t);
    }
    let listed = pts.iter().map(|q| format!("{} {}", pl::nn(q.0), pl::nn(q.1))).collect::<Vec<_>>().join("; ");
    Some(s.figure("section", "Polygon section: centroid and second moments",
                  &format!("Polygon section with vertices {} (numbered in order), drawn to scale with the returned centroid \
                            (x̄ = {}, ȳ = {}) and centroidal axes. Marked: {} (units of the vertex coordinates).", listed,
                           pl::nn(xb), pl::nn(yb), v.join(", ")), c.body(), c.h))
}

/// combined_axial_and_bending_stress: the corner stresses (+x,+y), (-x,+y), (-x,-y), (+x,-y) the model returned, and
/// the neutral axis where the plane through them is zero.
pub fn combined(s: &Solved) -> Option<Figure> {
    let (b, h) = (s.inp("b")?, s.inp("h")?);
    let cs = s.out_list("sigma_corners_MPa")?;
    if cs.len() != 4 || !(b > 0.0 && h > 0.0) {
        return None;
    }
    let (smax, smin, sax) = (s.out("sigma_max_MPa")?, s.out("sigma_min_MPa")?, s.out("sigma_axial_MPa")?);
    let scale = cs.iter().map(|v| v.abs()).fold(1e-9, f64::max);
    // a linear stress field: opposite corners sum alike
    if ((cs[0] + cs[2]) - (cs[1] + cs[3])).abs() > 1e-6 * scale {
        return None;
    }
    let c0 = cs.iter().sum::<f64>() / 4.0;
    let cx = ((cs[0] - cs[1]) + (cs[3] - cs[2])) / (2.0 * b);
    let cy = ((cs[0] - cs[3]) + (cs[1] - cs[2])) / (2.0 * h);
    let mut c = Canvas::new(360.0);
    pl::legend(&mut c, 20.0, 22.0, &[("neutral axis (zero stress)".to_string(), "axp-c3")]);
    let parts = vec![vec![rect(-b / 2.0, -h / 2.0, b, h)]];
    let m = draw_parts(&mut c, &parts, (150.0, 70.0, 370.0, 310.0))?;
    axes(&mut c, &m, &parts, (0.0, 0.0));
    // the neutral axis inside the rectangle: points on its edges where c0 + cx x + cy y = 0
    let mut na = vec![];
    let (hb, hh) = (b / 2.0, h / 2.0);
    if cy.abs() > 1e-300 {
        for x in [-hb, hb] {
            let y = -(c0 + cx * x) / cy;
            if y.abs() <= hh {
                na.push((x, y));
            }
        }
    }
    if cx.abs() > 1e-300 {
        for y in [-hh, hh] {
            let x = -(c0 + cy * y) / cx;
            if x.abs() <= hb {
                na.push((x, y));
            }
        }
    }
    if na.len() >= 2 {
        let (p, q) = (m.p(na[0]), m.p(na[1]));
        c.raw(&format!("<path class=\"axp-c3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p.0, p.1, q.0, q.1));
    }
    let corners = [(hb, hh), (-hb, hh), (-hb, -hh), (hb, -hh)];
    for (i, (q, v)) in corners.iter().zip(&cs).enumerate() {
        let p = m.p(*q);
        let class = if *v >= 0.0 { "axp-d" } else { "axp-d3" };
        c.raw(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", p.0, p.1));
        let (anchor, dx) = if q.0 > 0.0 { ("start", 8.0) } else { ("end", -8.0) };
        let dy = if q.1 > 0.0 { -10.0 } else { 20.0 };
        c.value(p.0 + dx, p.1 + dy, anchor, &pl::mpa(*v));
        let _ = i;
    }
    c.value(16.0, 346.0, "start", &format!("σmax = {}, σmin = {}, axial {}", pl::mpa(smax), pl::mpa(smin), pl::mpa(sax)));
    Some(s.figure("section", "Combined axial and bending stress at the corners",
                  &format!("Rectangular section b = {} by h = {} under the stated axial force and bending moments. Marked: \
                            the corner stresses the model returned ({} MPa at (+x,+y), (-x,+y), (-x,-y), (+x,-y); tension in \
                            the first colour), σmax = {}, σmin = {}, axial part {}; the neutral axis joins the points of zero \
                            stress on the plane through the corner values.", l1(b), l1(h),
                           cs.iter().map(|v| pl::nn(*v)).collect::<Vec<_>>().join(", "), pl::mpa(smax), pl::mpa(smin),
                           pl::mpa(sax)), c.body(), c.h))
}
