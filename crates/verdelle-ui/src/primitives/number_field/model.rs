//! The number logic behind the Number Field: parsing what is typed, formatting a value, and
//! working out the result of a step. Plain functions, no DOM, so they are unit tested.

/// Numbers are rounded to at most this many decimals, so `0.1 + 0.2` shows as `0.3`.
const MAX_DECIMALS: usize = 12;

/// What typed text means.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parsed {
    /// Nothing but (at most) the prefix and suffix: the field is empty.
    Empty,
    /// Unfinished input such as `-` or `.`: not a number yet, not wrong either.
    Partial,
    Number(f64),
    /// Characters that can never be part of a number.
    Invalid,
}

/// Read `text` as a number written with a `.` decimal point, an optional leading sign, and the
/// given `prefix` and `suffix` around it (either may be empty). No thousands separators, no
/// exponents.
pub fn parse_number(text: &str, prefix: &str, suffix: &str) -> Parsed {
    let mut t = text.trim();
    if !prefix.is_empty()
        && let Some(rest) = t.strip_prefix(prefix.trim())
    {
        t = rest.trim_start();
    }
    if !suffix.is_empty()
        && let Some(rest) = t.strip_suffix(suffix.trim())
    {
        t = rest.trim_end();
    }
    if t.is_empty() {
        return Parsed::Empty;
    }
    let body = t.strip_prefix(['-', '+']).unwrap_or(t);
    let mut dots = 0;
    for c in body.chars() {
        match c {
            '0'..='9' => {}
            '.' => dots += 1,
            _ => return Parsed::Invalid,
        }
    }
    if dots > 1 {
        return Parsed::Invalid;
    }
    if body.is_empty() || body == "." {
        return Parsed::Partial;
    }
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() => Parsed::Number(v),
        _ => Parsed::Invalid,
    }
}

/// How many digits follow the decimal point when `v` is written the short way.
pub fn decimals_of(v: f64) -> usize {
    let s = format!("{}", round_to(v, MAX_DECIMALS));
    s.split_once('.').map_or(0, |(_, frac)| frac.len())
}

/// Round to `decimals` places (at most 12).
pub fn round_to(v: f64, decimals: usize) -> f64 {
    let factor = 10f64.powi(decimals.min(MAX_DECIMALS) as i32);
    let r = (v * factor).round() / factor;
    if r == 0.0 { 0.0 } else { r }
}

/// Write `v` with exactly `fixed` decimals, or as short as it can be when `fixed` is `None`,
/// between `prefix` and `suffix`.
pub fn format_number(v: f64, fixed: Option<usize>, prefix: &str, suffix: &str) -> String {
    let body = match fixed {
        Some(d) => {
            let r = round_to(v, d);
            format!("{r:.d$}")
        }
        None => format!("{}", round_to(v, MAX_DECIMALS)),
    };
    format!("{prefix}{body}{suffix}")
}

/// Keep `v` between `min` and `max`, when they are set.
pub fn clamp_to(v: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let v = min.map_or(v, |m| v.max(m));
    max.map_or(v, |m| v.min(m))
}

/// The value after one step of `amount` up (`direction` 1) or down (-1) from `current` (an empty
/// field counts as 0), kept within `min` and `max`. With `snap`, the result is moved onto the
/// grid of multiples of `amount` counted from `min` (or from 0), so 3 stepping up by 5 gives 5.
pub fn step_value(
    current: Option<f64>,
    direction: i8,
    amount: f64,
    min: Option<f64>,
    max: Option<f64>,
    snap: bool,
) -> f64 {
    let base = current.unwrap_or(0.0);
    let dir = f64::from(direction.signum());
    let origin = min.unwrap_or(0.0);
    let raw = if snap && amount > 0.0 {
        let position = (base - origin) / amount;
        // Move to the next grid line in the direction of travel; a value already on a line
        // (within float noise) moves a whole step.
        let line = if dir > 0.0 {
            (position + 1e-9).floor() + 1.0
        } else {
            (position - 1e-9).ceil() - 1.0
        };
        origin + line * amount
    } else {
        base + dir * amount
    };
    let decimals = decimals_of(base)
        .max(decimals_of(amount))
        .max(decimals_of(origin));
    clamp_to(round_to(raw, decimals), min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_numbers() {
        assert_eq!(parse_number("42", "", ""), Parsed::Number(42.0));
        assert_eq!(parse_number(" -3.5 ", "", ""), Parsed::Number(-3.5));
        assert_eq!(parse_number("+7", "", ""), Parsed::Number(7.0));
        assert_eq!(parse_number(".5", "", ""), Parsed::Number(0.5));
        assert_eq!(parse_number("1.", "", ""), Parsed::Number(1.0));
    }

    #[test]
    fn empty_and_unfinished_text_are_not_errors() {
        assert_eq!(parse_number("", "", ""), Parsed::Empty);
        assert_eq!(parse_number("  ", "", ""), Parsed::Empty);
        assert_eq!(parse_number("-", "", ""), Parsed::Partial);
        assert_eq!(parse_number(".", "", ""), Parsed::Partial);
        assert_eq!(parse_number("-.", "", ""), Parsed::Partial);
    }

    #[test]
    fn rejects_what_cannot_become_a_number() {
        assert_eq!(parse_number("1e5", "", ""), Parsed::Invalid);
        assert_eq!(parse_number("1,000", "", ""), Parsed::Invalid);
        assert_eq!(parse_number("1.2.3", "", ""), Parsed::Invalid);
        assert_eq!(parse_number("abc", "", ""), Parsed::Invalid);
        assert_eq!(parse_number("--1", "", ""), Parsed::Invalid);
    }

    #[test]
    fn prefix_and_suffix_are_stripped_and_optional() {
        assert_eq!(parse_number("$12.50", "$", ""), Parsed::Number(12.5));
        assert_eq!(parse_number("12.50", "$", ""), Parsed::Number(12.5));
        assert_eq!(parse_number("$ 12", "$ ", ""), Parsed::Number(12.0));
        assert_eq!(parse_number("30 kg", "", " kg"), Parsed::Number(30.0));
        assert_eq!(parse_number("$", "$", ""), Parsed::Empty);
        assert_eq!(parse_number("$-", "$", ""), Parsed::Partial);
    }

    #[test]
    fn formats_short_or_fixed() {
        assert_eq!(format_number(3.0, None, "", ""), "3");
        assert_eq!(format_number(0.1 + 0.2, None, "", ""), "0.3");
        assert_eq!(format_number(3.0, Some(2), "", ""), "3.00");
        assert_eq!(format_number(2.005, Some(2), "", ""), "2.01");
        assert_eq!(format_number(-0.0, Some(1), "", ""), "0.0");
        assert_eq!(format_number(12.5, Some(2), "$", ""), "$12.50");
        assert_eq!(format_number(30.0, None, "", " kg"), "30 kg");
    }

    #[test]
    fn counts_decimals() {
        assert_eq!(decimals_of(3.0), 0);
        assert_eq!(decimals_of(0.25), 2);
        assert_eq!(decimals_of(0.1 + 0.2), 1);
    }

    #[test]
    fn clamps_to_either_bound() {
        assert_eq!(clamp_to(5.0, Some(0.0), Some(3.0)), 3.0);
        assert_eq!(clamp_to(-5.0, Some(0.0), Some(3.0)), 0.0);
        assert_eq!(clamp_to(5.0, None, None), 5.0);
        assert_eq!(clamp_to(5.0, Some(7.0), None), 7.0);
    }

    #[test]
    fn steps_without_float_noise() {
        assert_eq!(step_value(Some(0.1), 1, 0.2, None, None, false), 0.3);
        assert_eq!(step_value(Some(0.3), -1, 0.1, None, None, false), 0.2);
        assert_eq!(step_value(Some(1.0), 1, 1.0, None, None, false), 2.0);
        assert_eq!(step_value(Some(1.0), -1, 10.0, None, None, false), -9.0);
    }

    #[test]
    fn stepping_stops_at_the_bounds() {
        assert_eq!(
            step_value(Some(9.0), 1, 5.0, Some(0.0), Some(10.0), false),
            10.0
        );
        assert_eq!(
            step_value(Some(2.0), -1, 5.0, Some(0.0), Some(10.0), false),
            0.0
        );
    }

    #[test]
    fn an_empty_field_steps_from_zero_into_the_range() {
        assert_eq!(step_value(None, 1, 1.0, None, None, false), 1.0);
        assert_eq!(step_value(None, -1, 1.0, None, None, false), -1.0);
        assert_eq!(step_value(None, 1, 1.0, Some(5.0), None, false), 5.0);
    }

    #[test]
    fn snapping_moves_to_the_next_grid_line() {
        assert_eq!(step_value(Some(3.0), 1, 5.0, Some(0.0), None, true), 5.0);
        assert_eq!(step_value(Some(3.0), -1, 5.0, Some(0.0), None, true), 0.0);
        assert_eq!(step_value(Some(5.0), 1, 5.0, Some(0.0), None, true), 10.0);
        assert_eq!(step_value(Some(10.0), -1, 5.0, Some(0.0), None, true), 5.0);
        // the grid starts at min
        assert_eq!(step_value(Some(3.0), 1, 5.0, Some(2.0), None, true), 7.0);
    }
}
