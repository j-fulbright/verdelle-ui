//! "Nice" axis ticks: round steps (1, 2, 5 times a power of ten) and a domain that lands on them.

/// A domain widened to whole steps, and the ticks across it.
#[derive(Clone, Debug, PartialEq)]
pub struct NiceScale {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub ticks: Vec<f64>,
}

/// A step of 1, 2 or 5 times a power of ten close to `raw`.
fn nice_step(raw: f64) -> f64 {
    let exp = raw.log10().floor();
    let base = 10f64.powf(exp);
    let frac = raw / base;
    let nice = if frac <= 1.0 {
        1.0
    } else if frac <= 2.0 {
        2.0
    } else if frac <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * base
}

/// Round away float noise left by multiplying a step (`0.30000000000000004` -> `0.3`).
fn tidy(v: f64, step: f64) -> f64 {
    let digits = (-step.log10().floor()).max(0.0) as i32 + 2;
    let p = 10f64.powi(digits.min(12));
    (v * p).round() / p
}

/// Ticks for the data range `min..=max`, about `count` of them. With `nice` the domain is widened
/// to whole steps (so a chart's axis ends on a tick); without it the ticks stay inside the data.
/// A flat range (`min == max`) is opened up around the value; an empty or non-finite one gives 0 to 1.
pub fn nice_scale(min: f64, max: f64, count: usize, nice: bool) -> NiceScale {
    let (mut lo, mut hi) = (min.min(max), min.max(max));
    if !min.is_finite() || !max.is_finite() {
        lo = 0.0;
        hi = 1.0;
    }
    if lo == hi {
        if lo == 0.0 {
            hi = 1.0;
        } else if lo > 0.0 {
            lo = 0.0;
        } else {
            hi = 0.0;
        }
    }
    let count = count.max(1) as f64;
    let step = nice_step((hi - lo) / count);
    let (start, end) = if nice {
        (
            tidy((lo / step).floor() * step, step),
            tidy((hi / step).ceil() * step, step),
        )
    } else {
        (
            tidy((lo / step).ceil() * step, step),
            tidy((hi / step).floor() * step, step),
        )
    };
    let n = ((end - start) / step).round().max(0.0) as usize;
    let ticks = (0..=n)
        .map(|i| tidy(start + step * i as f64, step))
        .collect();
    NiceScale {
        min: if nice { start } else { lo },
        max: if nice { end } else { hi },
        step,
        ticks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widens_to_round_numbers() {
        let s = nice_scale(0.0, 186.0, 4, true);
        assert_eq!(s.step, 50.0);
        assert_eq!(s.min, 0.0);
        assert_eq!(s.max, 200.0);
        assert_eq!(s.ticks, vec![0.0, 50.0, 100.0, 150.0, 200.0]);
    }

    #[test]
    fn small_ranges_get_decimal_steps_without_float_noise() {
        let s = nice_scale(0.0, 0.9, 3, true);
        assert_eq!(s.step, 0.5);
        assert_eq!(s.ticks, vec![0.0, 0.5, 1.0]);
        let s = nice_scale(0.0, 0.3, 3, true);
        assert_eq!(s.ticks, vec![0.0, 0.1, 0.2, 0.3]);
    }

    #[test]
    fn negative_ranges_include_zero_crossing_ticks() {
        let s = nice_scale(-30.0, 90.0, 4, true);
        assert_eq!(s.step, 50.0);
        assert_eq!(s.ticks, vec![-50.0, 0.0, 50.0, 100.0]);
    }

    #[test]
    fn without_nice_the_ticks_stay_inside() {
        let s = nice_scale(3.0, 97.0, 4, false);
        assert_eq!((s.min, s.max), (3.0, 97.0));
        assert!(s.ticks.iter().all(|t| *t >= 3.0 && *t <= 97.0));
        assert_eq!(s.ticks, vec![50.0]);
    }

    #[test]
    fn degenerate_input_still_gives_a_usable_axis() {
        let s = nice_scale(5.0, 5.0, 4, true);
        assert!(s.max > s.min && s.ticks.len() >= 2);
        let s = nice_scale(0.0, 0.0, 4, true);
        assert_eq!((s.min, s.max), (0.0, 1.0));
        let s = nice_scale(f64::NAN, 3.0, 4, true);
        assert_eq!((s.min, s.max), (0.0, 1.0));
        // Swapped bounds are put in order.
        let s = nice_scale(100.0, 0.0, 2, true);
        assert_eq!((s.min, s.max), (0.0, 100.0));
    }

    #[test]
    fn large_numbers() {
        let s = nice_scale(0.0, 12_345.0, 5, true);
        assert_eq!(s.step, 5000.0);
        assert_eq!(s.max, 15_000.0);
    }
}
