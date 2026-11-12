//! Stacking: where each series' segment sits when series are piled on one another.

/// One stacked value: it spans `start..end` on the value axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
}

impl Segment {
    pub fn low(&self) -> f64 {
        self.start.min(self.end)
    }

    pub fn high(&self) -> f64 {
        self.start.max(self.end)
    }
}

/// How a stack is normalised.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackOffset {
    /// Values as they are.
    #[default]
    None,
    /// Each category scaled so its stack spans 0 to 1 (a 100% chart).
    Expand,
}

/// Stack `series` (each one value per category, all the same length). Positive values pile up
/// from zero and negative ones down from zero, so a mixed category does not cancel itself out.
/// Non-finite values count as zero. Returns the same shape: `result[series][category]`.
pub fn stack(series: &[Vec<f64>], offset: StackOffset) -> Vec<Vec<Segment>> {
    let cats = series.iter().map(Vec::len).max().unwrap_or(0);
    let mut out = vec![Vec::with_capacity(cats); series.len()];
    for c in 0..cats {
        let value = |s: usize| {
            let v = series[s].get(c).copied().unwrap_or(0.0);
            if v.is_finite() { v } else { 0.0 }
        };
        let scale = match offset {
            StackOffset::None => 1.0,
            StackOffset::Expand => {
                let total: f64 = (0..series.len()).map(|s| value(s).abs()).sum();
                if total > 0.0 { 1.0 / total } else { 1.0 }
            }
        };
        let (mut up, mut down) = (0.0, 0.0);
        for (s, col) in out.iter_mut().enumerate() {
            let v = value(s) * scale;
            if v >= 0.0 {
                col.push(Segment {
                    start: up,
                    end: up + v,
                });
                up += v;
            } else {
                col.push(Segment {
                    start: down,
                    end: down + v,
                });
                down += v;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_values_pile_up() {
        let s = stack(
            &[vec![10.0, 20.0], vec![5.0, 0.0], vec![1.0, 2.0]],
            StackOffset::None,
        );
        assert_eq!(
            s[0][0],
            Segment {
                start: 0.0,
                end: 10.0
            }
        );
        assert_eq!(
            s[1][0],
            Segment {
                start: 10.0,
                end: 15.0
            }
        );
        assert_eq!(
            s[2][0],
            Segment {
                start: 15.0,
                end: 16.0
            }
        );
        assert_eq!(
            s[2][1],
            Segment {
                start: 20.0,
                end: 22.0
            }
        );
    }

    #[test]
    fn negatives_stack_downwards_separately() {
        let s = stack(
            &[vec![10.0], vec![-4.0], vec![-6.0], vec![3.0]],
            StackOffset::None,
        );
        assert_eq!(
            s[1][0],
            Segment {
                start: 0.0,
                end: -4.0
            }
        );
        assert_eq!(
            s[2][0],
            Segment {
                start: -4.0,
                end: -10.0
            }
        );
        assert_eq!(
            s[3][0],
            Segment {
                start: 10.0,
                end: 13.0
            }
        );
        assert_eq!(s[2][0].low(), -10.0);
        assert_eq!(s[2][0].high(), -4.0);
    }

    #[test]
    fn expand_makes_every_category_span_one() {
        let s = stack(&[vec![30.0, 0.0], vec![10.0, 0.0]], StackOffset::Expand);
        assert!((s[0][0].end - 0.75).abs() < 1e-12);
        assert!((s[1][0].end - 1.0).abs() < 1e-12);
        // An all-zero category stays flat rather than dividing by zero.
        assert_eq!(
            s[1][1],
            Segment {
                start: 0.0,
                end: 0.0
            }
        );
    }

    #[test]
    fn bad_input_is_treated_as_zero() {
        let s = stack(&[vec![f64::NAN, 4.0], vec![2.0]], StackOffset::None);
        assert_eq!(s[0][0].end, 0.0);
        assert_eq!(
            s[1][0],
            Segment {
                start: 0.0,
                end: 2.0
            }
        );
        // A short series is padded with zeros.
        assert_eq!(
            s[1][1],
            Segment {
                start: 4.0,
                end: 4.0
            }
        );
        assert!(stack(&[], StackOffset::None).is_empty());
    }
}
