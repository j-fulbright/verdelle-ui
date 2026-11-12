//! SVG path data for lines and areas through a list of points, with the curve types of d3-shape.

use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// How neighbouring points are joined.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Curve {
    /// Straight segments.
    Linear,
    /// A smooth curve that never overshoots the data (no dip below a flat run): Fritsch-Carlson,
    /// d3's `curveMonotoneX`. Needs x to increase (or decrease) steadily. The default.
    #[default]
    Monotone,
    /// A natural cubic spline: smooth, but may overshoot between points.
    Natural,
    /// Level to the midpoint between two points, then straight up or down (d3 `curveStep`).
    Step,
    /// Up or down at the first point of each pair, then level (d3 `curveStepBefore`).
    StepBefore,
    /// Level to the next point, then up or down there (d3 `curveStepAfter`).
    StepAfter,
}

impl Curve {
    pub const ALL: [Curve; 6] = [
        Curve::Linear,
        Curve::Monotone,
        Curve::Natural,
        Curve::Step,
        Curve::StepBefore,
        Curve::StepAfter,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Curve::Linear => "linear",
            Curve::Monotone => "monotone",
            Curve::Natural => "natural",
            Curve::Step => "step",
            Curve::StepBefore => "step-before",
            Curve::StepAfter => "step-after",
        }
    }
}

/// A number with at most two decimals and no trailing zeros, for compact path data.
pub fn num(v: f64) -> String {
    let r = (v * 100.0).round() / 100.0;
    let r = if r == 0.0 { 0.0 } else { r };
    let mut s = format!("{r:.2}");
    while s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

fn pt(p: Point) -> String {
    format!("{},{}", num(p.x), num(p.y))
}

fn finite(points: &[Point]) -> Vec<Point> {
    points
        .iter()
        .copied()
        .filter(|p| p.x.is_finite() && p.y.is_finite())
        .collect()
}

fn sign(x: f64) -> f64 {
    if x < 0.0 { -1.0 } else { 1.0 }
}

/// The tangent at the middle of three points that keeps the curve monotone (d3's `slope3`).
fn slope3(a: Point, b: Point, c: Point) -> f64 {
    let h0 = b.x - a.x;
    let h1 = c.x - b.x;
    if h0 == 0.0 || h1 == 0.0 {
        return 0.0;
    }
    let s0 = (b.y - a.y) / h0;
    let s1 = (c.y - b.y) / h1;
    let p = (s0 * h1 + s1 * h0) / (h0 + h1);
    let t = (sign(s0) + sign(s1)) * s0.abs().min(s1.abs()).min(0.5 * p.abs());
    if t.is_finite() { t } else { 0.0 }
}

/// The tangent at an end point, given the tangent `t` at its neighbour (d3's `slope2`).
fn slope2(a: Point, b: Point, t: f64) -> f64 {
    let h = b.x - a.x;
    if h != 0.0 {
        (3.0 * (b.y - a.y) / h - t) / 2.0
    } else {
        t
    }
}

/// Control points of a natural spline through one coordinate (d3's `controlPoints`).
fn control_points(x: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = x.len() - 1;
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut r = vec![0.0; n];
    a[0] = 0.0;
    b[0] = 2.0;
    r[0] = x[0] + 2.0 * x[1];
    for i in 1..n.saturating_sub(1) {
        a[i] = 1.0;
        b[i] = 4.0;
        r[i] = 4.0 * x[i] + 2.0 * x[i + 1];
    }
    a[n - 1] = 2.0;
    b[n - 1] = 7.0;
    r[n - 1] = 8.0 * x[n - 1] + x[n];
    for i in 1..n {
        let m = a[i] / b[i - 1];
        b[i] -= m;
        r[i] -= m * r[i - 1];
    }
    a[n - 1] = r[n - 1] / b[n - 1];
    for i in (0..n.saturating_sub(1)).rev() {
        a[i] = (r[i] - a[i + 1]) / b[i];
    }
    b[n - 1] = (x[n] + a[n - 1]) / 2.0;
    for i in 0..n.saturating_sub(1) {
        b[i] = 2.0 * x[i + 1] - a[i + 1];
    }
    (a, b)
}

/// Path commands through `points`, starting with `first` (`M` for a new path, `L` to continue one).
fn commands(points: &[Point], curve: Curve, first: char) -> String {
    let mut d = String::new();
    let Some(&p0) = points.first() else {
        return d;
    };
    let _ = write!(d, "{first}{}", pt(p0));
    let n = points.len();
    if n == 1 {
        return d;
    }
    match curve {
        Curve::Linear => {
            for p in &points[1..] {
                let _ = write!(d, "L{}", pt(*p));
            }
        }
        _ if n == 2 && matches!(curve, Curve::Monotone | Curve::Natural) => {
            let _ = write!(d, "L{}", pt(points[1]));
        }
        Curve::Monotone => {
            // Tangents: interior by slope3, the ends by slope2 against their neighbour.
            let mut t = vec![0.0; n];
            for i in 1..n - 1 {
                t[i] = slope3(points[i - 1], points[i], points[i + 1]);
            }
            t[0] = slope2(points[0], points[1], t[1]);
            t[n - 1] = slope2(points[n - 2], points[n - 1], t[n - 2]);
            for i in 0..n - 1 {
                let (a, b) = (points[i], points[i + 1]);
                let dx = (b.x - a.x) / 3.0;
                let _ = write!(
                    d,
                    "C{},{} {},{} {}",
                    num(a.x + dx),
                    num(a.y + dx * t[i]),
                    num(b.x - dx),
                    num(b.y - dx * t[i + 1]),
                    pt(b)
                );
            }
        }
        Curve::Natural => {
            let xs: Vec<f64> = points.iter().map(|p| p.x).collect();
            let ys: Vec<f64> = points.iter().map(|p| p.y).collect();
            let (ax, bx) = control_points(&xs);
            let (ay, by) = control_points(&ys);
            for i in 0..n - 1 {
                let _ = write!(
                    d,
                    "C{},{} {},{} {}",
                    num(ax[i]),
                    num(ay[i]),
                    num(bx[i]),
                    num(by[i]),
                    pt(points[i + 1])
                );
            }
        }
        Curve::Step | Curve::StepBefore | Curve::StepAfter => {
            let t = match curve {
                Curve::StepBefore => 0.0,
                Curve::StepAfter => 1.0,
                _ => 0.5,
            };
            for w in points.windows(2) {
                let (a, b) = (w[0], w[1]);
                let mx = a.x + (b.x - a.x) * t;
                let _ = write!(d, "L{},{}L{},{}", num(mx), num(a.y), num(mx), num(b.y));
                if mx != b.x {
                    let _ = write!(d, "L{}", pt(b));
                }
            }
        }
    }
    d
}

/// The path of a line through `points`. Points with a non-finite coordinate are skipped (the line
/// joins across the gap). An empty list gives an empty string.
pub fn line_path(points: &[Point], curve: Curve) -> String {
    commands(&finite(points), curve, 'M')
}

/// A closed shape between an `upper` and a `lower` edge (both left to right, same length): the
/// upper edge through its curve, a straight join, the lower edge back through the same curve.
/// For an area chart the lower edge is the baseline or the stack below.
pub fn area_path(upper: &[Point], lower: &[Point], curve: Curve) -> String {
    let top = finite(upper);
    let mut bottom = finite(lower);
    if top.is_empty() || bottom.is_empty() {
        return String::new();
    }
    bottom.reverse();
    let mut d = commands(&top, curve, 'M');
    d.push_str(&commands(&bottom, curve, 'L'));
    d.push('Z');
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f64, y: f64) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn numbers_are_compact() {
        assert_eq!(num(1.0), "1");
        assert_eq!(num(1.5), "1.5");
        assert_eq!(num(1.234_56), "1.23");
        assert_eq!(num(-0.001), "0");
        assert_eq!(num(100.0), "100");
    }

    #[test]
    fn linear_path() {
        let d = line_path(&[p(0.0, 10.0), p(10.0, 20.0), p(20.0, 5.0)], Curve::Linear);
        assert_eq!(d, "M0,10L10,20L20,5");
    }

    #[test]
    fn empty_and_single_inputs() {
        assert_eq!(line_path(&[], Curve::Monotone), "");
        assert_eq!(line_path(&[p(3.0, 4.0)], Curve::Monotone), "M3,4");
        assert_eq!(area_path(&[], &[], Curve::Linear), "");
    }

    #[test]
    fn non_finite_points_are_skipped() {
        let d = line_path(&[p(0.0, 0.0), p(1.0, f64::NAN), p(2.0, 2.0)], Curve::Linear);
        assert_eq!(d, "M0,0L2,2");
    }

    #[test]
    fn two_points_are_a_straight_line_for_smooth_curves() {
        let pts = [p(0.0, 0.0), p(10.0, 10.0)];
        assert_eq!(line_path(&pts, Curve::Monotone), "M0,0L10,10");
        assert_eq!(line_path(&pts, Curve::Natural), "M0,0L10,10");
    }

    #[test]
    fn step_variants() {
        let pts = [p(0.0, 0.0), p(10.0, 10.0)];
        assert_eq!(line_path(&pts, Curve::Step), "M0,0L5,0L5,10L10,10");
        assert_eq!(line_path(&pts, Curve::StepBefore), "M0,0L0,0L0,10L10,10");
        assert_eq!(line_path(&pts, Curve::StepAfter), "M0,0L10,0L10,10");
    }

    /// Parse the cubic segments out of path data: `(c1, c2, end)` per `C`.
    fn cubics(d: &str) -> Vec<[Point; 3]> {
        d.split('C')
            .skip(1)
            .map(|seg| {
                let nums: Vec<f64> = seg
                    .split([',', ' '])
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect();
                [
                    p(nums[0], nums[1]),
                    p(nums[2], nums[3]),
                    p(nums[4], nums[5]),
                ]
            })
            .collect()
    }

    #[test]
    fn monotone_does_not_overshoot_a_flat_run() {
        // A plateau followed by a rise: a natural spline dips below 5, monotone never does.
        let pts = [
            p(0.0, 5.0),
            p(10.0, 5.0),
            p(20.0, 5.0),
            p(30.0, 50.0),
            p(40.0, 50.0),
        ];
        let d = line_path(&pts, Curve::Monotone);
        for seg in cubics(&d) {
            for c in &seg[..2] {
                assert!(c.y >= 5.0 - 1e-9 && c.y <= 50.0 + 1e-9, "{c:?}");
            }
        }
        // The flat run's tangent is zero, so its control points stay on y = 5.
        let segs = cubics(&d);
        assert_eq!(segs[0][0].y, 5.0);
        assert_eq!(segs[0][1].y, 5.0);
    }

    #[test]
    fn monotone_passes_through_every_point() {
        let pts = [p(0.0, 0.0), p(10.0, 30.0), p(20.0, 10.0), p(30.0, 40.0)];
        let segs = cubics(&line_path(&pts, Curve::Monotone));
        assert_eq!(segs.len(), 3);
        for (i, s) in segs.iter().enumerate() {
            assert_eq!(s[2], pts[i + 1]);
        }
        // At a local extreme (10,30) the tangent is flat: both adjoining control points have y = 30.
        assert_eq!(segs[0][1].y, 30.0);
        assert_eq!(segs[1][0].y, 30.0);
    }

    #[test]
    fn natural_spline_is_smooth_through_the_points() {
        let pts = [p(0.0, 0.0), p(10.0, 10.0), p(20.0, 0.0), p(30.0, 10.0)];
        let segs = cubics(&line_path(&pts, Curve::Natural));
        assert_eq!(segs.len(), 3);
        for (i, s) in segs.iter().enumerate() {
            assert_eq!(s[2], pts[i + 1]);
        }
        // C1 continuity: the control points either side of a knot are mirror images through it.
        for i in 0..2 {
            let knot = pts[i + 1];
            let (before, after) = (segs[i][1], segs[i + 1][0]);
            assert!((before.x + after.x - 2.0 * knot.x).abs() < 0.02);
            assert!((before.y + after.y - 2.0 * knot.y).abs() < 0.02);
        }
    }

    #[test]
    fn area_goes_out_along_the_top_and_back_along_the_bottom() {
        let top = [p(0.0, 10.0), p(10.0, 5.0)];
        let bottom = [p(0.0, 20.0), p(10.0, 20.0)];
        assert_eq!(
            area_path(&top, &bottom, Curve::Linear),
            "M0,10L10,5L10,20L0,20Z"
        );
        let smooth = area_path(
            &[p(0.0, 10.0), p(10.0, 0.0), p(20.0, 10.0)],
            &[p(0.0, 20.0), p(10.0, 20.0), p(20.0, 20.0)],
            Curve::Monotone,
        );
        assert!(smooth.starts_with("M0,10C"));
        assert!(smooth.contains("L20,20C"));
        assert!(smooth.ends_with("0,20Z"));
    }
}
