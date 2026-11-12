//! Number formatting for tick labels and tooltips.

/// Format with thousands separators and at most two decimals, trimming trailing zeros
/// (`1234567.5` -> `1,234,567.5`). Non-finite values give `"-"`.
pub fn format_number(v: f64) -> String {
    format_with(v, 2)
}

/// Format an axis tick with as many decimals as its step needs (a step of 0.25 gives two).
pub fn format_tick(v: f64, step: f64) -> String {
    let mut decimals = 0usize;
    if step > 0.0 && step.is_finite() {
        while decimals < 6 && (step * 10f64.powi(decimals as i32)).fract().abs() > 1e-9 {
            // fract() of a value just under an integer (0.30000000000000004 * 10) is near 1.
            if (1.0 - (step * 10f64.powi(decimals as i32)).fract().abs()) < 1e-9 {
                break;
            }
            decimals += 1;
        }
    }
    format_with(v, decimals)
}

fn format_with(v: f64, decimals: usize) -> String {
    if !v.is_finite() {
        return "-".into();
    }
    let s = format!("{:.*}", decimals, v.abs());
    let (int, frac) = match s.split_once('.') {
        Some((i, f)) => (i, f.trim_end_matches('0')),
        None => (s.as_str(), ""),
    };
    let mut out = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    if !frac.is_empty() {
        out.push('.');
        out.push_str(frac);
    }
    // A negative that rounds to zero is just zero.
    if v < 0.0 && out.chars().any(|c| c.is_ascii_digit() && c != '0') {
        out.insert(0, '-');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_thousands_and_trims() {
        assert_eq!(format_number(1234567.5), "1,234,567.5");
        assert_eq!(format_number(12.0), "12");
        assert_eq!(format_number(999.0), "999");
        assert_eq!(format_number(1000.0), "1,000");
        assert_eq!(format_number(0.456), "0.46");
        assert_eq!(format_number(-1500.0), "-1,500");
        assert_eq!(format_number(-0.001), "0");
        assert_eq!(format_number(f64::NAN), "-");
    }

    #[test]
    fn tick_decimals_follow_the_step() {
        assert_eq!(format_tick(50.0, 50.0), "50");
        assert_eq!(format_tick(0.5, 0.5), "0.5");
        assert_eq!(format_tick(0.25, 0.25), "0.25");
        assert_eq!(format_tick(0.3, 0.1), "0.3");
        assert_eq!(format_tick(2500.0, 500.0), "2,500");
    }
}
