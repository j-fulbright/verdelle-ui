//! Time field model: the pure logic behind [`crate::TimeInput`] (hour and minute segments,
//! 12 and 24 hour clocks, typing digits, stepping, `HH:MM` text). Verdelle-original; no DOM.

/// Which clock the field shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HourCycle {
    /// 24-hour clock: hours 00 to 23, no AM/PM segment. The default.
    #[default]
    H24,
    /// 12-hour clock: hours 1 to 12 with an AM/PM segment.
    H12,
}

/// The segments of the field, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment {
    /// The hour.
    Hour,
    /// The minute.
    Minute,
    /// AM or PM (12-hour clock only).
    Period,
}

/// What is typed into the segments so far. Any part may be empty while the user is typing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fields {
    /// The hour as displayed: 0 to 23 on a 24-hour clock, 1 to 12 on a 12-hour one.
    pub hour: Option<u8>,
    /// The minute, 0 to 59.
    pub minute: Option<u8>,
    /// `Some(true)` for PM, `Some(false)` for AM (12-hour clock only).
    pub pm: Option<bool>,
}

impl Fields {
    /// The segments of a `HH:MM` (24-hour) value, shown on `cycle`. Unparseable text gives
    /// empty fields.
    pub fn from_value(value: &str, cycle: HourCycle) -> Fields {
        let Some((h, m)) = parse_time(value) else {
            return Fields::default();
        };
        match cycle {
            HourCycle::H24 => Fields {
                hour: Some(h),
                minute: Some(m),
                pm: None,
            },
            HourCycle::H12 => Fields {
                hour: Some(match h % 12 {
                    0 => 12,
                    n => n,
                }),
                minute: Some(m),
                pm: Some(h >= 12),
            },
        }
    }

    /// The `HH:MM` (24-hour) value once every segment is filled, else `None`.
    pub fn to_value(&self, cycle: HourCycle) -> Option<String> {
        let (hour, minute) = (self.hour?, self.minute?);
        let hour24 = match cycle {
            HourCycle::H24 => hour,
            HourCycle::H12 => {
                let pm = self.pm?;
                (hour % 12) + if pm { 12 } else { 0 }
            }
        };
        Some(format_time(hour24, minute))
    }
}

/// Parse `HH:MM` (also `H:MM`, and `HH:MM:SS` ignoring seconds) into 24-hour (hour, minute).
pub fn parse_time(value: &str) -> Option<(u8, u8)> {
    let mut parts = value.trim().split(':');
    let hour: u8 = parts.next()?.parse().ok()?;
    let minute: u8 = parts.next()?.parse().ok()?;
    if let Some(seconds) = parts.next() {
        let seconds: u8 = seconds.parse().ok()?;
        if seconds > 59 {
            return None;
        }
    }
    if parts.next().is_some() || hour > 23 || minute > 59 {
        return None;
    }
    Some((hour, minute))
}

/// Format a 24-hour hour and minute as `HH:MM`.
pub fn format_time(hour: u8, minute: u8) -> String {
    format!("{hour:02}:{minute:02}")
}

/// Whether `value` (`HH:MM`) lies within `min` and `max` (each `HH:MM`, either optional). A
/// range whose `min` is later than `max` wraps past midnight (22:00 to 06:00). Text that does not
/// parse never fails the check.
pub fn in_range(value: &str, min: Option<&str>, max: Option<&str>) -> bool {
    let minutes = |s: &str| parse_time(s).map(|(h, m)| u16::from(h) * 60 + u16::from(m));
    let Some(v) = minutes(value) else {
        return true;
    };
    let min = min.and_then(minutes);
    let max = max.and_then(minutes);
    match (min, max) {
        (Some(lo), Some(hi)) if lo > hi => v >= lo || v <= hi,
        (lo, hi) => lo.is_none_or(|lo| v >= lo) && hi.is_none_or(|hi| v <= hi),
    }
}

/// The lowest and highest value of a segment.
pub fn segment_range(segment: Segment, cycle: HourCycle) -> (u8, u8) {
    match (segment, cycle) {
        (Segment::Hour, HourCycle::H24) => (0, 23),
        (Segment::Hour, HourCycle::H12) => (1, 12),
        (Segment::Minute, _) => (0, 59),
        (Segment::Period, _) => (0, 1),
    }
}

/// The next value of a segment after pressing Up (`delta` = 1) or Down (`delta` = -1), wrapping
/// at the ends. An empty segment starts from the low end on Up and the high end on Down. Minutes
/// move to the next multiple of `minute_step` in that direction.
pub fn step(
    segment: Segment,
    cycle: HourCycle,
    current: Option<u8>,
    delta: i32,
    minute_step: u8,
) -> u8 {
    let (lo, hi) = segment_range(segment, cycle);
    let Some(cur) = current else {
        return if delta >= 0 { lo } else { hi };
    };
    if segment == Segment::Minute && minute_step > 1 {
        let step = i32::from(minute_step);
        let cur = i32::from(cur);
        let next = if delta >= 0 {
            (cur / step + 1) * step
        } else if cur % step == 0 {
            cur - step
        } else {
            cur / step * step
        };
        return next.rem_euclid(60) as u8;
    }
    let span = i32::from(hi) - i32::from(lo) + 1;
    (i32::from(cur) - i32::from(lo) + delta).rem_euclid(span) as u8 + lo
}

/// The result of typing one digit into a segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Typed {
    /// The segment's new value, or `None` to leave it as it was (a leading 0 on a 12-hour hour).
    pub value: Option<u8>,
    /// The digit typed so far that a second digit could still extend (`None` when settled).
    pub pending: Option<u8>,
    /// Whether focus should move to the next segment.
    pub advance: bool,
}

/// Type a digit into the hour or minute segment. `pending` is the first digit typed just before
/// (what [`Typed::pending`] returned). A second digit that would give an impossible value
/// starts over as a first digit.
pub fn type_digit(segment: Segment, cycle: HourCycle, pending: Option<u8>, digit: u8) -> Typed {
    debug_assert!(digit <= 9);
    let (lo, hi) = segment_range(segment, cycle);
    if let Some(first) = pending {
        let combined = first * 10 + digit;
        if (lo..=hi).contains(&combined) {
            return Typed {
                value: Some(combined),
                pending: None,
                advance: true,
            };
        }
        if first == 0 && digit == 0 {
            // "00" on a 12-hour hour: nothing valid yet, keep waiting.
            return Typed {
                value: None,
                pending: Some(0),
                advance: false,
            };
        }
        // Does not fit: start over with this digit.
    }
    // A first digit. It is final when no second digit could keep it in range.
    if digit == 0 && lo > 0 {
        return Typed {
            value: None,
            pending: Some(0),
            advance: false,
        };
    }
    let tens_limit = hi / 10; // the largest first digit that may be extended
    let final_now = digit > tens_limit;
    Typed {
        value: Some(digit),
        pending: if final_now { None } else { Some(digit) },
        advance: final_now,
    }
}

/// Set the period from a typed letter: `a` or `p` in either case.
pub fn type_period(ch: char) -> Option<bool> {
    match ch.to_ascii_lowercase() {
        'a' => Some(false),
        'p' => Some(true),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(value: Option<u8>, pending: Option<u8>, advance: bool) -> Typed {
        Typed {
            value,
            pending,
            advance,
        }
    }

    #[test]
    fn parses_and_formats_times() {
        assert_eq!(parse_time("09:05"), Some((9, 5)));
        assert_eq!(parse_time("9:05"), Some((9, 5)));
        assert_eq!(parse_time("23:59:30"), Some((23, 59)));
        assert_eq!(parse_time("24:00"), None);
        assert_eq!(parse_time("12:60"), None);
        assert_eq!(parse_time("noon"), None);
        assert_eq!(parse_time(""), None);
        assert_eq!(format_time(9, 5), "09:05");
    }

    #[test]
    fn twelve_hour_fields_round_trip() {
        for h in 0..24u8 {
            for m in [0u8, 7, 59] {
                let text = format_time(h, m);
                let fields = Fields::from_value(&text, HourCycle::H12);
                assert_eq!(
                    fields.to_value(HourCycle::H12).as_deref(),
                    Some(text.as_str())
                );
            }
        }
        let midnight = Fields::from_value("00:30", HourCycle::H12);
        assert_eq!(midnight.hour, Some(12));
        assert_eq!(midnight.pm, Some(false));
        let noon = Fields::from_value("12:00", HourCycle::H12);
        assert_eq!((noon.hour, noon.pm), (Some(12), Some(true)));
        let evening = Fields::from_value("18:15", HourCycle::H12);
        assert_eq!((evening.hour, evening.pm), (Some(6), Some(true)));
    }

    #[test]
    fn value_needs_every_segment() {
        let mut f = Fields::from_value("14:20", HourCycle::H12);
        f.pm = None;
        assert_eq!(f.to_value(HourCycle::H12), None);
        // The 24-hour clock has no period.
        assert_eq!(f.to_value(HourCycle::H24).as_deref(), Some("02:20"));
        f.minute = None;
        assert_eq!(f.to_value(HourCycle::H24), None);
    }

    #[test]
    fn range_check_handles_wrap_past_midnight() {
        assert!(in_range("10:00", Some("09:00"), Some("17:00")));
        assert!(!in_range("08:59", Some("09:00"), Some("17:00")));
        assert!(!in_range("17:01", Some("09:00"), Some("17:00")));
        assert!(in_range("09:00", Some("09:00"), None));
        assert!(in_range("23:00", None, None));
        // 22:00 to 06:00 runs over midnight.
        assert!(in_range("23:30", Some("22:00"), Some("06:00")));
        assert!(in_range("05:00", Some("22:00"), Some("06:00")));
        assert!(!in_range("12:00", Some("22:00"), Some("06:00")));
    }

    #[test]
    fn stepping_wraps_and_starts_from_the_ends() {
        use Segment::*;
        assert_eq!(step(Hour, HourCycle::H24, Some(23), 1, 1), 0);
        assert_eq!(step(Hour, HourCycle::H24, Some(0), -1, 1), 23);
        assert_eq!(step(Hour, HourCycle::H12, Some(12), 1, 1), 1);
        assert_eq!(step(Hour, HourCycle::H12, Some(1), -1, 1), 12);
        assert_eq!(step(Hour, HourCycle::H12, None, 1, 1), 1);
        assert_eq!(step(Hour, HourCycle::H12, None, -1, 1), 12);
        assert_eq!(step(Minute, HourCycle::H24, Some(59), 1, 1), 0);
        assert_eq!(step(Minute, HourCycle::H24, None, -1, 1), 59);
        assert_eq!(step(Period, HourCycle::H12, Some(1), 1, 1), 0);
    }

    #[test]
    fn minute_step_snaps_to_multiples() {
        use Segment::*;
        assert_eq!(step(Minute, HourCycle::H24, Some(0), 1, 15), 15);
        assert_eq!(step(Minute, HourCycle::H24, Some(7), 1, 15), 15);
        assert_eq!(step(Minute, HourCycle::H24, Some(45), 1, 15), 0);
        assert_eq!(step(Minute, HourCycle::H24, Some(7), -1, 15), 0);
        assert_eq!(step(Minute, HourCycle::H24, Some(15), -1, 15), 0);
        assert_eq!(step(Minute, HourCycle::H24, Some(0), -1, 15), 45);
    }

    #[test]
    fn typing_a_24_hour_hour() {
        use Segment::Hour;
        let c = HourCycle::H24;
        // 3 can only be 03: final at once.
        assert_eq!(type_digit(Hour, c, None, 3), typed(Some(3), None, true));
        // 1 may become 10 to 19.
        let first = type_digit(Hour, c, None, 1);
        assert_eq!(first, typed(Some(1), Some(1), false));
        assert_eq!(
            type_digit(Hour, c, first.pending, 4),
            typed(Some(14), None, true)
        );
        // 2 then 5 is not an hour: starts over as 5.
        let two = type_digit(Hour, c, None, 2);
        assert_eq!(two.pending, Some(2));
        assert_eq!(
            type_digit(Hour, c, two.pending, 5),
            typed(Some(5), None, true)
        );
        // 0 then 7 is 07.
        let zero = type_digit(Hour, c, None, 0);
        assert_eq!(zero, typed(Some(0), Some(0), false));
        assert_eq!(type_digit(Hour, c, zero.pending, 7).value, Some(7));
    }

    #[test]
    fn typing_a_12_hour_hour() {
        use Segment::Hour;
        let c = HourCycle::H12;
        // A leading 0 sets nothing yet.
        let zero = type_digit(Hour, c, None, 0);
        assert_eq!(zero, typed(None, Some(0), false));
        assert_eq!(
            type_digit(Hour, c, zero.pending, 9),
            typed(Some(9), None, true)
        );
        // 1 waits for 10, 11 or 12.
        let one = type_digit(Hour, c, None, 1);
        assert_eq!(one, typed(Some(1), Some(1), false));
        assert_eq!(type_digit(Hour, c, one.pending, 2).value, Some(12));
        // 1 then 3 is not an hour: becomes 3.
        assert_eq!(type_digit(Hour, c, one.pending, 3).value, Some(3));
        // 2 to 9 are final.
        assert!(type_digit(Hour, c, None, 2).advance);
    }

    #[test]
    fn typing_a_minute() {
        use Segment::Minute;
        let c = HourCycle::H24;
        assert!(type_digit(Minute, c, None, 7).advance);
        let five = type_digit(Minute, c, None, 5);
        assert_eq!(five, typed(Some(5), Some(5), false));
        assert_eq!(
            type_digit(Minute, c, five.pending, 9),
            typed(Some(59), None, true)
        );
        let zero = type_digit(Minute, c, None, 0);
        assert_eq!(zero.value, Some(0));
        assert_eq!(type_digit(Minute, c, zero.pending, 5).value, Some(5));
    }

    #[test]
    fn typed_period_letters() {
        assert_eq!(type_period('a'), Some(false));
        assert_eq!(type_period('P'), Some(true));
        assert_eq!(type_period('x'), None);
    }
}
