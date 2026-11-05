//! Drawing helpers of Mechanical Engineering's figures (model-local, beside ax_engine::figkit): plot panels with their
//! own axes (several panels in one figure, equal scales for Mohr's circles and failure envelopes), coloured arrows,
//! supports, walls, moment arcs and legends. Every drawing uses the website's figure classes (axp-*).

use ax_engine::figkit::{num, q, Canvas, P};

/// One axis: values lo..hi drawn from pixel a to pixel b (logarithmic when `log`).
#[derive(Clone, Copy)]
pub struct Ax {
    pub lo: f64,
    pub hi: f64,
    pub log: bool,
    pub a: f64,
    pub b: f64,
}

impl Ax {
    pub fn new(lo: f64, hi: f64, log: bool, a: f64, b: f64, pad: bool) -> Ax {
        let (mut lo, mut hi) = (lo, hi);
        if log {
            if !(lo > 0.0) {
                lo = hi / 10.0;
            }
            if !(hi > lo) {
                hi = lo * 10.0;
            }
            if pad {
                lo /= 1.25;
                hi *= 1.25;
            }
        } else {
            if !(hi > lo) || !(hi - lo).is_finite() {
                let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
                lo -= d;
                hi += d;
            }
            if pad {
                let d = (hi - lo) * 0.07;
                lo -= d;
                hi += d;
            }
        }
        Ax { lo, hi, log, a, b }
    }
    pub fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    pub fn ticks(&self, n: usize) -> Vec<f64> {
        if self.log && self.hi / self.lo < 20.0 {
            return nice_ticks(self.lo, self.hi, n);
        }
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let cnt = (e1 - e0 + 1).max(1);
            let step = ((cnt as f64) / 6.0).ceil().max(1.0) as usize;
            let mut v: Vec<f64> = (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
            if v.is_empty() {
                v.push(self.lo * (self.hi / self.lo).sqrt());
            }
            return v;
        }
        nice_ticks(self.lo, self.hi, n)
    }
}

pub fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 12 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

/// The smallest and largest of some values (None when empty or not finite).
pub fn range(v: &[f64]) -> Option<(f64, f64)> {
    let f: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if f.is_empty() {
        return None;
    }
    Some(f.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x))))
}

/// A plot panel: its pixel box (x0, y0, x1, y1) and its two axes.
pub struct Plot {
    pub xa: Ax,
    pub ya: Ax,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub id: String,
}

impl Plot {
    /// A panel over the value ranges xr and yr (padded when asked); `id`: a distinct letter for its clip path.
    pub fn new(bx: (f64, f64, f64, f64), xr: (f64, f64), yr: (f64, f64), log: (bool, bool), pad: (bool, bool), id: &str)
               -> Plot {
        let (x0, y0, x1, y1) = bx;
        Plot { xa: Ax::new(xr.0, xr.1, log.0, x0, x1, pad.0), ya: Ax::new(yr.0, yr.1, log.1, y1, y0, pad.1), x0, y0, x1, y1,
               id: id.to_string() }
    }
    /// A panel with one scale on both axes (a circle stays a circle), holding every point given, padded a little.
    pub fn equal(bx: (f64, f64, f64, f64), xs: &[f64], ys: &[f64], id: &str) -> Option<Plot> {
        let (x0, y0, x1, y1) = bx;
        let (xl, xh) = range(xs)?;
        let (yl, yh) = range(ys)?;
        let (w, h) = (x1 - x0, y1 - y0);
        let mut sx = (xh - xl).max(1e-300);
        let mut sy = (yh - yl).max(1e-300);
        if xh - xl <= 0.0 && yh - yl <= 0.0 {
            sx = 1.0;
            sy = 1.0;
        }
        // units per pixel: the larger need, padded 10 %
        let u = (sx / w).max(sy / h) * 1.1;
        let (cx, cy) = ((xl + xh) / 2.0, (yl + yh) / 2.0);
        let (hx, hy) = (u * w / 2.0, u * h / 2.0);
        Some(Plot { xa: Ax { lo: cx - hx, hi: cx + hx, log: false, a: x0, b: x1 },
                    ya: Ax { lo: cy - hy, hi: cy + hy, log: false, a: y1, b: y0 }, x0, y0, x1, y1, id: id.to_string() })
    }
    pub fn x(&self, v: f64) -> f64 {
        self.xa.px(v)
    }
    pub fn y(&self, v: f64) -> f64 {
        self.ya.px(v)
    }
    pub fn pt(&self, p: (f64, f64)) -> P {
        (self.x(p.0), self.y(p.1))
    }
    /// Grid, tick values, the frame and the axis names (the vertical one turned, left of the ticks).
    pub fn frame(&self, c: &mut Canvas, xlab: &str, ylab: &str) {
        for t in self.xa.ticks(5) {
            let x = self.x(t);
            if x < self.x0 - 0.5 || x > self.x1 + 0.5 {
                continue;
            }
            c.raw(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>", self.y0, self.y1));
            c.text(x, self.y1 + 17.0, "middle", "axp-t", &num(t));
        }
        for t in self.ya.ticks(4) {
            let y = self.y(t);
            if y < self.y0 - 0.5 || y > self.y1 + 0.5 {
                continue;
            }
            c.raw(&format!("<line class=\"axp-g\" x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\"/>", self.x0, self.x1));
            c.text(self.x0 - 6.0, y + 5.0, "end", "axp-t", &num(t));
        }
        c.raw(&format!("<rect class=\"axp-f\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", self.x0, self.y0,
                       self.x1 - self.x0, self.y1 - self.y0));
        if !xlab.is_empty() {
            c.text((self.x0 + self.x1) / 2.0, self.y1 + 37.0, "middle", "axp-l", xlab);
        }
        if !ylab.is_empty() {
            c.raw(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                           (self.y0 + self.y1) / 2.0, esc(ylab)));
        }
    }
    /// The zero lines (grey) when zero lies inside the panel.
    pub fn zeros(&self, c: &mut Canvas, vertical: bool, horizontal: bool) {
        if horizontal && !self.ya.log && self.ya.lo < 0.0 && self.ya.hi > 0.0 {
            let y = self.y(0.0);
            c.raw(&format!("<path class=\"axp-c4\" d=\"M{:.1},{y:.1} L{:.1},{y:.1}\"/>", self.x0, self.x1));
        }
        if vertical && !self.xa.log && self.xa.lo < 0.0 && self.xa.hi > 0.0 {
            let x = self.x(0.0);
            c.raw(&format!("<path class=\"axp-c4\" d=\"M{x:.1},{:.1} L{x:.1},{:.1}\"/>", self.y0, self.y1));
        }
    }
    /// Start drawing clipped to the panel (close with `close`).
    pub fn open(&self, c: &mut Canvas) {
        c.raw(&format!("<clipPath id=\"CLIPID{}\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>\
                        <g clip-path=\"url(#CLIPID{})\">", self.id, self.x0, self.y0, self.x1 - self.x0, self.y1 - self.y0,
                       self.id));
    }
    pub fn close(&self, c: &mut Canvas) {
        c.raw("</g>");
    }
    /// A polyline through the points (a non-finite point breaks it).
    pub fn path(&self, c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            c.raw(&format!("<path class=\"{class}\" d=\"{d}\"/>"));
        }
    }
    /// A dashed polyline.
    pub fn dashed(&self, c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            c.raw(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"{d}\"/>"));
        }
    }
    fn d(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for p in pts {
            let ok = p.0.is_finite() && p.1.is_finite() && (!self.xa.log || p.0 > 0.0) && (!self.ya.log || p.1 > 0.0);
            if !ok {
                pen = false;
                continue;
            }
            let (x, y) = self.pt(*p);
            d.push_str(&format!("{}{x:.1},{y:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        d.trim_end().to_string()
    }
    /// A filled polygon (closed).
    pub fn fill(&self, c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            c.raw(&format!("<path class=\"{class}\" d=\"{d} Z\"/>"));
        }
    }
    /// The area between a curve and zero, filled.
    pub fn area(&self, c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
        if pts.len() < 2 {
            return;
        }
        let mut v = vec![(pts[0].0, 0.0)];
        v.extend_from_slice(pts);
        v.push((pts[pts.len() - 1].0, 0.0));
        self.fill(c, &v, class);
    }
    pub fn dot(&self, c: &mut Canvas, p: (f64, f64), class: &str, r: f64) {
        let (x, y) = self.pt(p);
        if x.is_finite() && y.is_finite() {
            c.raw(&format!("<circle class=\"{class}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r:.1}\"/>"));
        }
    }
    /// A marked point (the visitor's) with its label beside it, kept inside the panel.
    pub fn mark(&self, c: &mut Canvas, p: (f64, f64), label: &str) {
        let (x, y) = self.pt(p);
        if !(x.is_finite() && y.is_finite()) {
            return;
        }
        c.raw(&format!("<circle class=\"axp-m\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"6\"/>"));
        let w = 9.0 * label.chars().count() as f64;
        let (tx, anchor) = if x + 10.0 + w <= self.x1 { (x + 10.0, "start") } else if x - 10.0 - w >= self.x0 {
            (x - 10.0, "end")
        } else {
            (((self.x0 + self.x1) / 2.0).clamp(self.x0 + w / 2.0, self.x1 - w / 2.0), "middle")
        };
        let ty = if y < self.y0 + 26.0 { y + 22.0 } else { y - 10.0 };
        c.text(tx, ty, anchor, "axp-mt", label);
    }
}

/// Text escaped for SVG.
pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A legend row from (x, y): a short line in each class and its name.
/// Returns the baseline of its last row.
pub fn legend(c: &mut Canvas, x: f64, y: f64, items: &[(String, &str)]) -> f64 {
    let mut px = x;
    let mut py = y;
    for (name, class) in items {
        if name.is_empty() {
            continue;
        }
        let w = 30.0 + 7.2 * name.chars().count() as f64 + 16.0;
        if px + w > 512.0 && px > x {
            px = x;
            py += 18.0;
        }
        if class.starts_with("axp-c") {
            c.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", px, py - 4.0, px + 22.0, py - 4.0));
        } else {
            c.raw(&format!("<rect class=\"{class}\" x=\"{px:.1}\" y=\"{:.1}\" width=\"14\" height=\"10\"/>", py - 9.0));
        }
        c.text(px + 28.0, py, "start", "axp-lt", name);
        px += w;
    }
    py
}

/// One curve of a chart: its name (none: left out of the legend), points, line class, dashed or not.
pub struct Curve {
    pub name: String,
    pub pts: Vec<(f64, f64)>,
    pub class: &'static str,
    pub dashed: bool,
}

pub fn curve(name: &str, pts: Vec<(f64, f64)>, k: usize) -> Curve {
    Curve { name: name.to_string(), pts, class: LINE[k % 4], dashed: false }
}

/// A chart in the generic 520 x 340 frame: a legend that wraps, the curves, an optional marked point with its label.
/// `y_from_zero`: the vertical range includes zero.
pub fn chart(curves: &[Curve], xlab: &str, ylab: &str, mark: Option<((f64, f64), String)>, log: (bool, bool),
             y_from_zero: bool) -> Option<String> {
    let mut c = Canvas::new(340.0);
    let items: Vec<(String, &str)> = curves.iter().map(|k| (k.name.clone(), k.class)).collect();
    let last = legend(&mut c, 96.0, 22.0, &items);
    let ok = |p: &(f64, f64)| p.0.is_finite() && p.1.is_finite() && (!log.0 || p.0 > 0.0) && (!log.1 || p.1 > 0.0);
    let mut xs: Vec<f64> = curves.iter().flat_map(|k| k.pts.iter().filter(|p| ok(p)).map(|p| p.0)).collect();
    let mut ys: Vec<f64> = curves.iter().flat_map(|k| k.pts.iter().filter(|p| ok(p)).map(|p| p.1)).collect();
    if let Some((m, _)) = &mark {
        xs.push(m.0);
        ys.push(m.1);
    }
    if y_from_zero && !log.1 {
        ys.push(0.0);
    }
    let xr = range(&xs)?;
    let yr = range(&ys)?;
    let p = Plot::new((96.0, last + 16.0, 504.0, 278.0), xr, yr, log, (false, true), "c");
    p.frame(&mut c, xlab, ylab);
    p.zeros(&mut c, false, true);
    p.open(&mut c);
    for k in curves {
        if k.dashed {
            p.dashed(&mut c, &k.pts, k.class);
        } else {
            p.path(&mut c, &k.pts, k.class);
        }
    }
    p.close(&mut c);
    if let Some((m, lab)) = mark {
        p.mark(&mut c, m, &lab);
    }
    Some(c.s)
}

/// A number for a label: 4 significant digits, whole numbers up to a million written out (20000, not 2e+04).
pub fn nn(x: f64) -> String {
    if x.is_finite() && x.abs() >= 1e4 && x.abs() < 1e6 {
        format!("{:.0}", x)
    } else {
        num(x)
    }
}

const LINE: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
const DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];

/// The line class of colour k (0 accent, 1 green, 2 violet, 3 grey).
pub fn line_class(k: usize) -> &'static str {
    LINE[k % 4]
}
/// An arrow from a to b in colour k (0 accent, 1 green, 2 violet, 3 grey).
pub fn arrow(c: &mut Canvas, a: P, b: P, k: usize) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt();
    if !(d > 1.0) {
        return;
    }
    let u = (dx / d, dy / d);
    let n = (-u.1, u.0);
    let hl = 9.0f64.min(d * 0.6);
    let base = (b.0 - u.0 * hl, b.1 - u.1 * hl);
    c.raw(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", LINE[k % 4], a.0, a.1, base.0 + u.0, base.1 + u.1));
    c.raw(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", DOT[k % 4], b.0, b.1,
                   base.0 + n.0 * 4.5, base.1 + n.1 * 4.5, base.0 - n.0 * 4.5, base.1 - n.1 * 4.5));
}

/// A circular arc round c of radius r from angle a0 to a1 (degrees, counter-clockwise on the page, 0 = right) with an
/// arrowhead at the end, in colour k.
pub fn arc_arrow(c: &mut Canvas, ctr: P, r: f64, a0: f64, a1: f64, k: usize) {
    let p = |a: f64| (ctr.0 + r * a.to_radians().cos(), ctr.1 - r * a.to_radians().sin());
    let (s, e) = (p(a0), p(a1));
    let large = if (a1 - a0).abs() > 180.0 { 1 } else { 0 };
    let sweep = if a1 > a0 { 0 } else { 1 };
    c.raw(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} A{r:.1},{r:.1} 0 {large} {sweep} {:.1},{:.1}\"/>", LINE[k % 4], s.0, s.1,
                   e.0, e.1));
    // the head: tangent at the end, in the sense of travel
    let sg = if a1 > a0 { 1.0 } else { -1.0 };
    let t = a1.to_radians();
    let u = (-t.sin() * sg, -t.cos() * sg);
    let n = (-u.1, u.0);
    let base = (e.0 - u.0 * 8.0, e.1 - u.1 * 8.0);
    c.raw(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", DOT[k % 4], e.0 + u.0 * 2.0,
                   e.1 + u.1 * 2.0, base.0 + n.0 * 4.5, base.1 + n.1 * 4.5, base.0 - n.0 * 4.5, base.1 - n.1 * 4.5));
}

/// Hatching under a horizontal line from x0 to x1 at y (the ground).
pub fn ground(c: &mut Canvas, x0: f64, x1: f64, y: f64) {
    c.line((x0, y), (x1, y));
    let mut x = x0 + 4.0;
    let mut d = String::new();
    while x <= x1 {
        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", x, y, x - 6.0, y + 6.0));
        x += 7.0;
    }
    c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:1.2\" d=\"{}\"/>", d.trim_end()));
}

/// A wall: a vertical line at x from y0 to y1, hatched on its left (`left`) or right side.
pub fn wall(c: &mut Canvas, x: f64, y0: f64, y1: f64, left: bool) {
    c.line((x, y0), (x, y1));
    let s = if left { -1.0 } else { 1.0 };
    let mut y = y0 + 3.0;
    let mut d = String::new();
    while y <= y1 {
        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", x, y, x + 7.0 * s, y - 6.0));
        y += 7.0;
    }
    c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:1.2\" d=\"{}\"/>", d.trim_end()));
}

/// A ceiling: a horizontal line hatched above it.
pub fn ceiling(c: &mut Canvas, x0: f64, x1: f64, y: f64) {
    c.line((x0, y), (x1, y));
    let mut x = x0 + 2.0;
    let mut d = String::new();
    while x <= x1 {
        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", x, y, x + 6.0, y - 6.0));
        x += 7.0;
    }
    c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:1.2\" d=\"{}\"/>", d.trim_end()));
}

/// A pinned support whose apex is at p (a triangle on hatched ground).
pub fn pin(c: &mut Canvas, p: P) {
    c.raw(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", p.0, p.1, p.0 - 10.0, p.1 + 16.0,
                   p.0 + 10.0, p.1 + 16.0));
    ground(c, p.0 - 15.0, p.0 + 15.0, p.1 + 16.0);
}

/// A roller support whose apex is at p (a triangle on two rollers).
pub fn roller(c: &mut Canvas, p: P) {
    c.raw(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", p.0, p.1, p.0 - 10.0, p.1 + 12.0,
                   p.0 + 10.0, p.1 + 12.0));
    for dx in [-5.0, 5.0] {
        c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.5\"/>", p.0 + dx, p.1 + 14.5));
    }
    ground(c, p.0 - 15.0, p.0 + 15.0, p.1 + 17.0);
}

/// A filled rectangle (a part, a mass), outlined.
pub fn rect(c: &mut Canvas, x: f64, y: f64, w: f64, h: f64) {
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>\
                    <rect class=\"axp-k\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>"));
}

/// A circle (a pulley, a pitch circle), outlined, optionally filled.
pub fn circle(c: &mut Canvas, p: P, r: f64, filled: bool) {
    if filled {
        c.raw(&format!("<circle class=\"axp-ka\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", p.0, p.1));
    }
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", p.0, p.1));
}

/// A dashed circle (a pitch circle, a crank path).
pub fn dashed_circle(c: &mut Canvas, p: P, r: f64) {
    c.raw(&format!("<circle class=\"axp-c4\" stroke-dasharray=\"6 4\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", p.0, p.1));
}

/// A zig-zag spring from a to b.
pub fn spring(c: &mut Canvas, a: P, b: P) {
    c.resistor(a, b);
}

/// A force in newtons as text (kN when large).
pub fn force(x: f64) -> String {
    q(x, "N")
}
/// A moment in N·m as text (kN·m when large).
pub fn moment(x: f64) -> String {
    format!("{}·m", q(x, "N"))
}
/// A stress in MPa.
pub fn mpa(x: f64) -> String {
    format!("{} MPa", num(x))
}
/// A length in metres shown in millimetres.
pub fn mm(x: f64) -> String {
    format!("{} mm", num(x * 1000.0))
}
