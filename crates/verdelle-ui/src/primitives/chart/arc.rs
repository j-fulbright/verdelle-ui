//! Arc maths for pies, donuts, radar and radial bars.
//!
//! Angles are in radians, clockwise from 12 o'clock (so 0 points up and a quarter turn points right),
//! the convention of d3's `arc` and `pie`.

use super::curve::num;
use std::f64::consts::TAU;

/// The point at `angle` and `radius` from the centre `(cx, cy)`.
pub fn polar(cx: f64, cy: f64, radius: f64, angle: f64) -> (f64, f64) {
    (cx + radius * angle.sin(), cy - radius * angle.cos())
}

/// One slice of a pie.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slice {
    pub index: usize,
    pub value: f64,
    pub start: f64,
    pub end: f64,
    /// `value` as a fraction of the total (0 when the total is 0).
    pub fraction: f64,
}

/// Lay out `values` around the arc `start_angle..end_angle`. Each slice takes its share of the sweep;
/// negative and non-finite values count as zero. `pad_angle` is the gap between slices; a slice
/// smaller than the gap shrinks to a sliver at its midpoint instead of inverting. The sweep may run
/// backwards (`end < start`).
pub fn pie_layout(values: &[f64], start_angle: f64, end_angle: f64, pad_angle: f64) -> Vec<Slice> {
    let clean: Vec<f64> = values
        .iter()
        .map(|v| if v.is_finite() && *v > 0.0 { *v } else { 0.0 })
        .collect();
    let total: f64 = clean.iter().sum();
    let sweep = end_angle - start_angle;
    let direction = if sweep < 0.0 { -1.0 } else { 1.0 };
    // Padding between slices cannot eat more than the sweep allows.
    let nonzero = clean.iter().filter(|v| **v > 0.0).count().max(1) as f64;
    let pad = pad_angle.abs().min(sweep.abs() / nonzero) * direction;
    let mut angle = start_angle;
    clean
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            let fraction = if total > 0.0 { value / total } else { 0.0 };
            let a0 = angle;
            let a1 = a0 + sweep * fraction;
            angle = a1;
            let (mut s, mut e) = (a0, a1);
            if value > 0.0 {
                let inset = (pad / 2.0).abs().min((e - s).abs() / 2.0) * direction;
                s += inset;
                e -= inset;
            }
            Slice {
                index,
                value,
                start: s,
                end: e,
                fraction,
            }
        })
        .collect()
}

/// The middle of a slice: halfway between the radii, halfway between the angles.
pub fn arc_centroid(cx: f64, cy: f64, inner: f64, outer: f64, a0: f64, a1: f64) -> (f64, f64) {
    polar(cx, cy, (inner + outer) / 2.0, (a0 + a1) / 2.0)
}

/// A filled ring segment between `inner` and `outer` radius from `a0` to `a1`. With `inner` of zero it
/// is a wedge to the centre. A sweep of a full turn or more draws a whole ring or disc.
pub fn arc_path(cx: f64, cy: f64, inner: f64, outer: f64, a0: f64, a1: f64) -> String {
    let (inner, outer) = (inner.max(0.0), outer.max(0.0));
    let sweep = a1 - a0;
    if outer == 0.0 || sweep == 0.0 {
        return String::new();
    }
    let sweep_flag = if sweep > 0.0 { 1 } else { 0 };
    let ring = |r: f64, flag: u8, to: (f64, f64)| {
        format!("A{0},{0},0,0,{flag},{1},{2}", num(r), num(to.0), num(to.1))
    };
    if sweep.abs() >= TAU - 1e-6 {
        // Two half turns: one arc command cannot describe a full circle.
        let top = (cx, cy - outer);
        let bottom = (cx, cy + outer);
        let mut d = format!("M{},{}", num(top.0), num(top.1));
        d.push_str(&ring(outer, sweep_flag, bottom));
        d.push_str(&ring(outer, sweep_flag, top));
        d.push('Z');
        if inner > 0.0 {
            let (it, ib) = ((cx, cy - inner), (cx, cy + inner));
            let back = 1 - sweep_flag;
            d.push_str(&format!("M{},{}", num(it.0), num(it.1)));
            d.push_str(&ring(inner, back, ib));
            d.push_str(&ring(inner, back, it));
            d.push('Z');
        }
        return d;
    }
    let large = if sweep.abs() > std::f64::consts::PI {
        1
    } else {
        0
    };
    let arc = |r: f64, flag: u8, to: (f64, f64)| {
        format!(
            "A{0},{0},0,{large},{flag},{1},{2}",
            num(r),
            num(to.0),
            num(to.1)
        )
    };
    let o0 = polar(cx, cy, outer, a0);
    let o1 = polar(cx, cy, outer, a1);
    let mut d = format!("M{},{}", num(o0.0), num(o0.1));
    d.push_str(&arc(outer, sweep_flag, o1));
    if inner > 0.0 {
        let i1 = polar(cx, cy, inner, a1);
        let i0 = polar(cx, cy, inner, a0);
        d.push_str(&format!("L{},{}", num(i1.0), num(i1.1)));
        d.push_str(&arc(inner, 1 - sweep_flag, i0));
    } else {
        d.push_str(&format!("L{},{}", num(cx), num(cy)));
    }
    d.push('Z');
    d
}

/// An open arc along one radius, for stroking (a radial bar with round caps).
pub fn arc_stroke_path(cx: f64, cy: f64, radius: f64, a0: f64, a1: f64) -> String {
    let sweep = a1 - a0;
    if radius <= 0.0 || sweep == 0.0 {
        return String::new();
    }
    // A full turn cannot be one arc; stop a hair short.
    let a1 = if sweep.abs() >= TAU - 1e-4 {
        a0 + (TAU - 1e-4) * sweep.signum()
    } else {
        a1
    };
    let sweep = a1 - a0;
    let start = polar(cx, cy, radius, a0);
    let end = polar(cx, cy, radius, a1);
    let large = if sweep.abs() > std::f64::consts::PI {
        1
    } else {
        0
    };
    let flag = if sweep > 0.0 { 1 } else { 0 };
    let r = num(radius);
    format!(
        "M{},{}A{r},{r},0,{large},{flag},{},{}",
        num(start.0),
        num(start.1),
        num(end.0),
        num(end.1)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn angles_run_clockwise_from_twelve() {
        let (x, y) = polar(100.0, 100.0, 50.0, 0.0);
        assert!(close(x, 100.0) && close(y, 50.0));
        let (x, y) = polar(100.0, 100.0, 50.0, FRAC_PI_2);
        assert!(close(x, 150.0) && close(y, 100.0));
        let (x, y) = polar(100.0, 100.0, 50.0, PI);
        assert!(close(x, 100.0) && close(y, 150.0));
    }

    #[test]
    fn pie_shares_the_sweep_by_value() {
        let s = pie_layout(&[1.0, 1.0, 2.0], 0.0, TAU, 0.0);
        assert!(close(s[0].end - s[0].start, FRAC_PI_2));
        assert!(close(s[2].end - s[2].start, PI));
        assert!(close(s[2].end, TAU));
        assert!(close(s[2].fraction, 0.5));
        // Slices touch.
        assert!(close(s[0].end, s[1].start));
    }

    #[test]
    fn pie_ignores_negative_and_bad_values_and_handles_a_zero_total() {
        let s = pie_layout(&[-5.0, f64::NAN, 3.0], 0.0, TAU, 0.0);
        assert_eq!(s[0].value, 0.0);
        assert_eq!(s[1].value, 0.0);
        assert!(close(s[2].end - s[2].start, TAU));
        let z = pie_layout(&[0.0, 0.0], 0.0, TAU, 0.0);
        assert!(z.iter().all(|s| s.start == s.end && s.fraction == 0.0));
        assert!(pie_layout(&[], 0.0, TAU, 0.0).is_empty());
    }

    #[test]
    fn pie_padding_opens_a_gap_and_survives_tiny_slices() {
        let s = pie_layout(&[50.0, 50.0], 0.0, TAU, 0.1);
        assert!(close(s[1].start - s[0].end, 0.1));
        let tiny = pie_layout(&[1000.0, 0.001], 0.0, TAU, 0.5);
        assert!(tiny[1].end >= tiny[1].start);
    }

    #[test]
    fn pie_can_be_a_half_circle_or_run_backwards() {
        let half = pie_layout(&[1.0, 1.0], -FRAC_PI_2, FRAC_PI_2, 0.0);
        assert!(close(half[0].start, -FRAC_PI_2));
        assert!(close(half[1].end, FRAC_PI_2));
        let back = pie_layout(&[1.0, 1.0], 0.0, -PI, 0.0);
        assert!(back[0].end < back[0].start);
    }

    #[test]
    fn ring_segment_path() {
        // A quarter ring from 12 o'clock to 3 o'clock.
        let d = arc_path(100.0, 100.0, 40.0, 60.0, 0.0, FRAC_PI_2);
        assert_eq!(d, "M100,40A60,60,0,0,1,160,100L140,100A40,40,0,0,0,100,60Z");
    }

    #[test]
    fn wedge_goes_to_the_centre() {
        let d = arc_path(100.0, 100.0, 0.0, 60.0, 0.0, PI * 1.5);
        assert_eq!(d, "M100,40A60,60,0,1,1,40,100L100,100Z");
    }

    #[test]
    fn a_full_turn_is_drawn_as_two_arcs_and_a_hole() {
        let d = arc_path(0.0, 0.0, 10.0, 20.0, 0.0, TAU);
        assert_eq!(d.matches('M').count(), 2);
        assert_eq!(d.matches('A').count(), 4);
        assert!(arc_path(0.0, 0.0, 0.0, 20.0, 0.0, TAU).matches('M').count() == 1);
    }

    #[test]
    fn empty_arcs_draw_nothing() {
        assert_eq!(arc_path(0.0, 0.0, 0.0, 0.0, 0.0, 1.0), "");
        assert_eq!(arc_path(0.0, 0.0, 0.0, 10.0, 1.0, 1.0), "");
        assert_eq!(arc_stroke_path(0.0, 0.0, 10.0, 1.0, 1.0), "");
    }

    #[test]
    fn centroid_is_between_the_radii_and_angles() {
        let (x, y) = arc_centroid(0.0, 0.0, 10.0, 30.0, 0.0, PI);
        assert!(close(x, 20.0) && close(y, 0.0));
    }

    #[test]
    fn stroke_arc() {
        let d = arc_stroke_path(100.0, 100.0, 50.0, 0.0, FRAC_PI_2);
        assert_eq!(d, "M100,50A50,50,0,0,1,150,100");
        let big = arc_stroke_path(100.0, 100.0, 50.0, 0.0, PI * 1.5);
        assert!(big.contains(",1,1,"));
    }
}
