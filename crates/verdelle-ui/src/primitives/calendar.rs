//! The headless model behind a calendar: dates without a time zone, month grids, keyboard movement,
//! bounds, and what a click does in single, multiple and range selection. It does no rendering and
//! touches no DOM, so it is testable natively; `verdelle_ui::Calendar` is the styled component built on
//! it.
//!
//! [`CalendarDate`] is a small dependency-free civil date (year, month, day, proleptic Gregorian).
//! `chrono` and `time` were left out on purpose: a calendar needs a dozen date operations, both crates
//! pull in time zone and formatting machinery a wasm bundle should not carry, and neither renders
//! month names from a plain string table.
//!
//! This is original Verdelle code (not a Leptix port): Base UI and React Aria (`@internationalized/date`)
//! are the references for behaviour, react-day-picker for what shadcn's Calendar does.

use std::fmt;

/// A day of the week. The week of a calendar starts on one of these.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Weekday {
    #[default]
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    pub const ALL: [Weekday; 7] = [
        Weekday::Sunday,
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
    ];

    /// 0 for Sunday up to 6 for Saturday.
    pub fn index(self) -> u8 {
        self as u8
    }

    /// The weekday for an index; wraps, so 7 is Sunday again.
    pub fn from_index(index: i64) -> Weekday {
        Weekday::ALL[index.rem_euclid(7) as usize]
    }
}

/// A calendar date with no time and no time zone. Ordered by year, then month, then day.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalendarDate {
    year: i32,
    month: u8,
    day: u8,
}

impl fmt::Debug for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso())
    }
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso())
    }
}

/// Whether `year` is a leap year in the Gregorian calendar.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// How many days `month` (1 to 12) has in `year`.
pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

// Howard Hinnant's civil-date algorithms: days since 1970-01-01 and back.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

impl CalendarDate {
    /// The earliest and latest years the date picker deals in (four digits, as ISO 8601 writes them).
    pub const MIN_YEAR: i32 = 1;
    pub const MAX_YEAR: i32 = 9999;

    /// A date, or `None` when the month or day does not exist (February 30th) or the year is out of range.
    pub fn new(year: i32, month: u8, day: u8) -> Option<CalendarDate> {
        if !(Self::MIN_YEAR..=Self::MAX_YEAR).contains(&year)
            || !(1..=12).contains(&month)
            || day == 0
            || day > days_in_month(year, month)
        {
            return None;
        }
        Some(CalendarDate { year, month, day })
    }

    pub fn year(self) -> i32 {
        self.year
    }

    /// 1 to 12.
    pub fn month(self) -> u8 {
        self.month
    }

    /// 1 to 31.
    pub fn day(self) -> u8 {
        self.day
    }

    /// Days since 1970-01-01 (negative before it).
    pub fn to_days(self) -> i64 {
        days_from_civil(self.year as i64, self.month as i64, self.day as i64)
    }

    /// The date `days` days after 1970-01-01. Dates outside years 1 to 9999 clamp to the ends.
    pub fn from_days(days: i64) -> CalendarDate {
        let (y, m, d) = civil_from_days(days);
        if y < Self::MIN_YEAR as i64 {
            return CalendarDate {
                year: Self::MIN_YEAR,
                month: 1,
                day: 1,
            };
        }
        if y > Self::MAX_YEAR as i64 {
            return CalendarDate {
                year: Self::MAX_YEAR,
                month: 12,
                day: 31,
            };
        }
        CalendarDate {
            year: y as i32,
            month: m as u8,
            day: d as u8,
        }
    }

    pub fn weekday(self) -> Weekday {
        // 1970-01-01 was a Thursday.
        Weekday::from_index(self.to_days() + 4)
    }

    pub fn add_days(self, days: i64) -> CalendarDate {
        CalendarDate::from_days(self.to_days() + days)
    }

    /// Moves by whole months, keeping the day of the month or, when the target month is shorter, its
    /// last day (January 31st plus a month is February 28th or 29th).
    pub fn add_months(self, months: i64) -> CalendarDate {
        let total = self.year as i64 * 12 + (self.month as i64 - 1) + months;
        let year = total.div_euclid(12);
        if year < Self::MIN_YEAR as i64 {
            return CalendarDate::new(Self::MIN_YEAR, 1, 1).unwrap_or(self);
        }
        if year > Self::MAX_YEAR as i64 {
            return CalendarDate::new(Self::MAX_YEAR, 12, 31).unwrap_or(self);
        }
        let year = year as i32;
        let month = total.rem_euclid(12) as u8 + 1;
        CalendarDate {
            year,
            month,
            day: self.day.min(days_in_month(year, month)),
        }
    }

    /// Moves by whole years; February 29th becomes the 28th in a common year.
    pub fn add_years(self, years: i64) -> CalendarDate {
        self.add_months(years * 12)
    }

    pub fn start_of_month(self) -> CalendarDate {
        CalendarDate { day: 1, ..self }
    }

    pub fn end_of_month(self) -> CalendarDate {
        CalendarDate {
            day: days_in_month(self.year, self.month),
            ..self
        }
    }

    /// The first day of this date's week, for a week that starts on `week_starts_on`.
    pub fn start_of_week(self, week_starts_on: Weekday) -> CalendarDate {
        let back = (self.weekday().index() as i64 - week_starts_on.index() as i64).rem_euclid(7);
        self.add_days(-back)
    }

    pub fn end_of_week(self, week_starts_on: Weekday) -> CalendarDate {
        self.start_of_week(week_starts_on).add_days(6)
    }

    /// Whether both dates fall in the same month of the same year.
    pub fn same_month(self, other: CalendarDate) -> bool {
        self.year == other.year && self.month == other.month
    }

    /// The ISO 8601 week this date is in: its week-numbering year and the week number (1 to 53).
    pub fn iso_week(self) -> (i32, u32) {
        let iso_weekday = match self.weekday() {
            Weekday::Sunday => 7,
            other => other.index() as i64,
        };
        let ordinal =
            self.to_days() - CalendarDate::new(self.year, 1, 1).map_or(0, |d| d.to_days()) + 1;
        let week = (ordinal - iso_weekday + 10) / 7;
        if week < 1 {
            let year = self.year - 1;
            return (year, iso_weeks_in_year(year));
        }
        if week > iso_weeks_in_year(self.year) as i64 {
            return (self.year + 1, 1);
        }
        (self.year, week as u32)
    }

    /// `YYYY-MM-DD`, the format an `<input type="date">` and a form submission use.
    pub fn to_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Parses exactly `YYYY-MM-DD`. Returns `None` for anything else, including days that do not exist.
    pub fn parse_iso(text: &str) -> Option<CalendarDate> {
        let bytes = text.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let digits = |range: std::ops::Range<usize>| -> Option<u32> {
            let part = &text[range];
            if part.bytes().all(|b| b.is_ascii_digit()) {
                part.parse().ok()
            } else {
                None
            }
        };
        let year = digits(0..4)?;
        let month = digits(5..7)?;
        let day = digits(8..10)?;
        CalendarDate::new(year as i32, month as u8, day as u8)
    }

    /// Today in UTC. This is what the server renders and what the browser hydrates with, so both agree;
    /// use [`CalendarDate::today`] in the browser for the visitor's own date.
    pub fn today_utc() -> CalendarDate {
        #[cfg(target_arch = "wasm32")]
        let millis = js_sys::Date::now();
        #[cfg(not(target_arch = "wasm32"))]
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0);
        CalendarDate::from_days((millis / 86_400_000.0).floor() as i64)
    }

    /// Today by the visitor's clock and time zone in the browser; UTC everywhere else.
    pub fn today() -> CalendarDate {
        #[cfg(target_arch = "wasm32")]
        {
            let now = js_sys::Date::new_0();
            CalendarDate::new(
                now.get_full_year() as i32,
                now.get_month() as u8 + 1,
                now.get_date() as u8,
            )
            .unwrap_or_else(CalendarDate::today_utc)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            CalendarDate::today_utc()
        }
    }
}

fn iso_weeks_in_year(year: i32) -> u32 {
    let p = |y: i32| (y + y.div_euclid(4) - y.div_euclid(100) + y.div_euclid(400)).rem_euclid(7);
    if p(year) == 4 || p(year - 1) == 3 {
        53
    } else {
        52
    }
}

/// The rows of a month grid: whole weeks, with the days of the neighbouring months that fill the first
/// and last row. `fixed_weeks` always gives six rows, so the grid does not change height between
/// months.
pub fn month_weeks(
    year: i32,
    month: u8,
    week_starts_on: Weekday,
    fixed_weeks: bool,
) -> Vec<[CalendarDate; 7]> {
    let Some(first) = CalendarDate::new(year, month, 1) else {
        return Vec::new();
    };
    let start = first.start_of_week(week_starts_on);
    let lead = (first.to_days() - start.to_days()) as usize;
    let rows = if fixed_weeks {
        6
    } else {
        (lead + days_in_month(year, month) as usize).div_ceil(7)
    };
    (0..rows)
        .map(|row| std::array::from_fn(|col| start.add_days((row * 7 + col) as i64)))
        .collect()
}

/// A key that moves the focused day in a month grid (the ARIA date-picker grid pattern).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridKey {
    Left,
    Right,
    Up,
    Down,
    /// Start of the week.
    Home,
    /// End of the week.
    End,
    PageUp,
    PageDown,
    ShiftPageUp,
    ShiftPageDown,
}

impl GridKey {
    /// The grid key for a `KeyboardEvent.key` value and its Shift state.
    pub fn from_key(key: &str, shift: bool) -> Option<GridKey> {
        Some(match (key, shift) {
            ("ArrowLeft", _) => GridKey::Left,
            ("ArrowRight", _) => GridKey::Right,
            ("ArrowUp", _) => GridKey::Up,
            ("ArrowDown", _) => GridKey::Down,
            ("Home", _) => GridKey::Home,
            ("End", _) => GridKey::End,
            ("PageUp", false) => GridKey::PageUp,
            ("PageDown", false) => GridKey::PageDown,
            ("PageUp", true) => GridKey::ShiftPageUp,
            ("PageDown", true) => GridKey::ShiftPageDown,
            _ => return None,
        })
    }

    /// Which way the key moves: -1 for earlier days, 1 for later ones.
    pub fn direction(self) -> i64 {
        match self {
            GridKey::Left
            | GridKey::Up
            | GridKey::PageUp
            | GridKey::ShiftPageUp
            | GridKey::Home => -1,
            _ => 1,
        }
    }
}

/// Where `key` moves the focus from `date`: a day, a week, to the ends of the week, a month, a year.
pub fn navigate(date: CalendarDate, key: GridKey, week_starts_on: Weekday) -> CalendarDate {
    match key {
        GridKey::Left => date.add_days(-1),
        GridKey::Right => date.add_days(1),
        GridKey::Up => date.add_days(-7),
        GridKey::Down => date.add_days(7),
        GridKey::Home => date.start_of_week(week_starts_on),
        GridKey::End => date.end_of_week(week_starts_on),
        GridKey::PageUp => date.add_months(-1),
        GridKey::PageDown => date.add_months(1),
        GridKey::ShiftPageUp => date.add_years(-1),
        GridKey::ShiftPageDown => date.add_years(1),
    }
}

/// The earliest and latest selectable dates, either of which may be open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bounds {
    pub min: Option<CalendarDate>,
    pub max: Option<CalendarDate>,
}

impl Bounds {
    pub fn contains(&self, date: CalendarDate) -> bool {
        self.min.is_none_or(|m| date >= m) && self.max.is_none_or(|m| date <= m)
    }

    pub fn clamp(&self, date: CalendarDate) -> CalendarDate {
        if let Some(min) = self.min
            && date < min
        {
            return min;
        }
        if let Some(max) = self.max
            && date > max
        {
            return max;
        }
        date
    }
}

/// How far [`find_enabled`] looks for an enabled day before it gives up.
const SCAN_LIMIT: i64 = 800;

/// The day to focus when moving to `candidate`: the candidate if it can be selected, otherwise the next
/// selectable day in `direction` (-1 or 1), staying inside `bounds`. A candidate outside the bounds is
/// pulled back to the edge first. `None` when there is no selectable day that way.
pub fn find_enabled(
    candidate: CalendarDate,
    direction: i64,
    bounds: &Bounds,
    is_disabled: &dyn Fn(CalendarDate) -> bool,
) -> Option<CalendarDate> {
    let step = if direction < 0 { -1 } else { 1 };
    let mut date = bounds.clamp(candidate);
    for _ in 0..SCAN_LIMIT {
        if !is_disabled(date) {
            return Some(date);
        }
        let next = date.add_days(step);
        if next == date || !bounds.contains(next) {
            return None;
        }
        date = next;
    }
    None
}

/// A range of dates being chosen: nothing, just a start, or a start and an end (never end before start).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DateRange {
    pub start: Option<CalendarDate>,
    pub end: Option<CalendarDate>,
}

/// Where a day sits in a [`DateRange`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangePart {
    /// A range of a single day, or one with only a start so far.
    Only,
    Start,
    Middle,
    End,
}

impl DateRange {
    pub fn new(start: CalendarDate, end: CalendarDate) -> DateRange {
        if end < start {
            DateRange {
                start: Some(end),
                end: Some(start),
            }
        } else {
            DateRange {
                start: Some(start),
                end: Some(end),
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Both ends chosen.
    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Where `date` falls in the range, if it is in it.
    pub fn part(&self, date: CalendarDate) -> Option<RangePart> {
        let start = self.start?;
        let Some(end) = self.end else {
            return (date == start).then_some(RangePart::Only);
        };
        if date < start || date > end {
            None
        } else if start == end {
            Some(RangePart::Only)
        } else if date == start {
            Some(RangePart::Start)
        } else if date == end {
            Some(RangePart::End)
        } else {
            Some(RangePart::Middle)
        }
    }

    /// How many days the range covers, both ends included.
    pub fn days(&self) -> Option<i64> {
        Some(self.end?.to_days() - self.start?.to_days() + 1)
    }
}

/// The range after the day `clicked` is chosen: the first click starts a range, the second ends it (a
/// click before the start makes that day the start and the old start the end), and a click after a
/// complete range starts a new one.
pub fn next_range(current: DateRange, clicked: CalendarDate) -> DateRange {
    match (current.start, current.end) {
        (Some(start), None) => DateRange::new(start, clicked),
        _ => DateRange {
            start: Some(clicked),
            end: None,
        },
    }
}

/// The dates after `date` is toggled in a multiple selection, sorted. Choosing a date that is in
/// the list removes it. With `max` dates already chosen a new one is ignored.
pub fn toggle_date(
    current: &[CalendarDate],
    date: CalendarDate,
    max: Option<usize>,
) -> Vec<CalendarDate> {
    let mut next: Vec<CalendarDate> = current.to_vec();
    if let Some(at) = next.iter().position(|d| *d == date) {
        next.remove(at);
    } else if max.is_none_or(|m| next.len() < m) {
        next.push(date);
    }
    next.sort();
    next
}

/// How a date is written out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateFormat {
    /// `November 9, 2026`
    #[default]
    Long,
    /// `Nov 9, 2026`
    Medium,
    /// `2026-11-09`
    Iso,
}

/// The words a calendar shows, English by default. Replace any of them to translate or reword.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarNames {
    /// January first.
    pub months: Vec<String>,
    pub months_short: Vec<String>,
    /// Sunday first, whatever the week start.
    pub weekdays: Vec<String>,
    /// What the column header shows, Sunday first (`Su`).
    pub weekdays_short: Vec<String>,
    pub previous_month: String,
    pub next_month: String,
    pub month: String,
    pub year: String,
    pub week_number: String,
    pub today: String,
}

impl Default for CalendarNames {
    fn default() -> Self {
        CalendarNames::english()
    }
}

impl CalendarNames {
    pub fn english() -> CalendarNames {
        let owned = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        CalendarNames {
            months: owned(&[
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]),
            months_short: owned(&[
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]),
            weekdays: owned(&[
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ]),
            weekdays_short: owned(&["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]),
            previous_month: "Go to the previous month".into(),
            next_month: "Go to the next month".into(),
            month: "Month".into(),
            year: "Year".into(),
            week_number: "Week".into(),
            today: "today".into(),
        }
    }

    fn pick(list: &[String], index: usize) -> &str {
        list.get(index).map(String::as_str).unwrap_or("")
    }

    pub fn month_name(&self, month: u8) -> &str {
        Self::pick(&self.months, month.saturating_sub(1) as usize)
    }

    pub fn month_short(&self, month: u8) -> &str {
        Self::pick(&self.months_short, month.saturating_sub(1) as usize)
    }

    pub fn weekday_name(&self, weekday: Weekday) -> &str {
        Self::pick(&self.weekdays, weekday.index() as usize)
    }

    pub fn weekday_short(&self, weekday: Weekday) -> &str {
        Self::pick(&self.weekdays_short, weekday.index() as usize)
    }

    /// `November 2026`, the caption of a month.
    pub fn month_caption(&self, date: CalendarDate) -> String {
        format!("{} {}", self.month_name(date.month()), date.year())
    }

    /// `Monday, November 9, 2026`, what a screen reader reads for a day.
    pub fn day_label(&self, date: CalendarDate) -> String {
        format!(
            "{}, {} {}, {}",
            self.weekday_name(date.weekday()),
            self.month_name(date.month()),
            date.day(),
            date.year()
        )
    }

    pub fn format(&self, date: CalendarDate, format: DateFormat) -> String {
        match format {
            DateFormat::Long => {
                format!(
                    "{} {}, {}",
                    self.month_name(date.month()),
                    date.day(),
                    date.year()
                )
            }
            DateFormat::Medium => {
                format!(
                    "{} {}, {}",
                    self.month_short(date.month()),
                    date.day(),
                    date.year()
                )
            }
            DateFormat::Iso => date.to_iso(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u8, day: u8) -> CalendarDate {
        CalendarDate::new(y, m, day).expect("valid date")
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2026));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2026, 2), 28);
        assert_eq!(days_in_month(2026, 4), 30);
        assert_eq!(days_in_month(2026, 12), 31);
    }

    #[test]
    fn new_rejects_dates_that_do_not_exist() {
        assert!(CalendarDate::new(2026, 2, 29).is_none());
        assert!(CalendarDate::new(2024, 2, 29).is_some());
        assert!(CalendarDate::new(2026, 13, 1).is_none());
        assert!(CalendarDate::new(2026, 0, 1).is_none());
        assert!(CalendarDate::new(2026, 4, 31).is_none());
        assert!(CalendarDate::new(0, 1, 1).is_none());
        assert!(CalendarDate::new(10_000, 1, 1).is_none());
    }

    #[test]
    fn weekday_matches_known_dates() {
        assert_eq!(d(1970, 1, 1).weekday(), Weekday::Thursday);
        assert_eq!(d(2000, 1, 1).weekday(), Weekday::Saturday);
        assert_eq!(d(2026, 11, 9).weekday(), Weekday::Monday);
        assert_eq!(d(2024, 2, 29).weekday(), Weekday::Thursday);
        assert_eq!(d(1969, 7, 20).weekday(), Weekday::Sunday);
    }

    #[test]
    fn days_round_trip_across_many_years() {
        for days in (-719_000..2_900_000).step_by(997) {
            let date = CalendarDate::from_days(days);
            assert_eq!(date.to_days(), days, "{date}");
        }
        assert_eq!(d(1970, 1, 1).to_days(), 0);
        assert_eq!(d(1970, 1, 2).to_days(), 1);
        assert_eq!(d(1969, 12, 31).to_days(), -1);
    }

    #[test]
    fn add_days_crosses_month_and_year_ends() {
        assert_eq!(d(2026, 12, 31).add_days(1), d(2027, 1, 1));
        assert_eq!(d(2024, 2, 28).add_days(1), d(2024, 2, 29));
        assert_eq!(d(2026, 2, 28).add_days(1), d(2026, 3, 1));
        assert_eq!(d(2026, 1, 1).add_days(-1), d(2025, 12, 31));
        assert_eq!(d(2026, 3, 1).add_days(-365), d(2025, 3, 1));
    }

    #[test]
    fn add_months_keeps_the_day_or_the_last_day_of_a_shorter_month() {
        assert_eq!(d(2026, 1, 31).add_months(1), d(2026, 2, 28));
        assert_eq!(d(2024, 1, 31).add_months(1), d(2024, 2, 29));
        assert_eq!(d(2026, 3, 31).add_months(-1), d(2026, 2, 28));
        assert_eq!(d(2026, 11, 15).add_months(2), d(2027, 1, 15));
        assert_eq!(d(2026, 1, 15).add_months(-1), d(2025, 12, 15));
        assert_eq!(d(2026, 6, 15).add_months(-18), d(2024, 12, 15));
    }

    #[test]
    fn add_years_turns_a_leap_day_into_the_28th() {
        assert_eq!(d(2024, 2, 29).add_years(1), d(2025, 2, 28));
        assert_eq!(d(2024, 2, 29).add_years(4), d(2028, 2, 29));
        assert_eq!(d(2026, 5, 5).add_years(-1), d(2025, 5, 5));
    }

    #[test]
    fn arithmetic_stops_at_the_supported_years() {
        assert_eq!(d(9999, 12, 31).add_days(1), d(9999, 12, 31));
        assert_eq!(d(1, 1, 1).add_days(-1), d(1, 1, 1));
        assert_eq!(d(9999, 6, 1).add_years(5), d(9999, 12, 31));
    }

    #[test]
    fn week_boundaries_depend_on_the_week_start() {
        // Monday the 9th of November 2026.
        let date = d(2026, 11, 9);
        assert_eq!(date.start_of_week(Weekday::Sunday), d(2026, 11, 8));
        assert_eq!(date.start_of_week(Weekday::Monday), date);
        assert_eq!(date.end_of_week(Weekday::Monday), d(2026, 11, 15));
        assert_eq!(date.end_of_week(Weekday::Sunday), d(2026, 11, 14));
        assert_eq!(date.start_of_week(Weekday::Saturday), d(2026, 11, 7));
    }

    #[test]
    fn month_grid_has_the_right_rows_and_leading_days() {
        // February 2026 starts on a Sunday and has 28 days: exactly four weeks from a Sunday start.
        let weeks = month_weeks(2026, 2, Weekday::Sunday, false);
        assert_eq!(weeks.len(), 4);
        assert_eq!(weeks[0][0], d(2026, 2, 1));
        assert_eq!(weeks[3][6], d(2026, 2, 28));
        // From a Monday start, the first row begins on January 26th and there are five rows.
        let weeks = month_weeks(2026, 2, Weekday::Monday, false);
        assert_eq!(weeks.len(), 5);
        assert_eq!(weeks[0][0], d(2026, 1, 26));
        assert_eq!(weeks[0][6], d(2026, 2, 1));
        assert_eq!(weeks[4][6], d(2026, 3, 1));
        // A month that needs six rows: August 2026 starts on a Saturday.
        let weeks = month_weeks(2026, 8, Weekday::Sunday, false);
        assert_eq!(weeks.len(), 6);
        assert_eq!(weeks[0][6], d(2026, 8, 1));
        assert_eq!(weeks[5][0], d(2026, 8, 30));
    }

    #[test]
    fn fixed_weeks_always_give_six_rows_of_consecutive_days() {
        let weeks = month_weeks(2026, 2, Weekday::Sunday, true);
        assert_eq!(weeks.len(), 6);
        let flat: Vec<CalendarDate> = weeks.iter().flatten().copied().collect();
        for pair in flat.windows(2) {
            assert_eq!(pair[1], pair[0].add_days(1));
        }
    }

    #[test]
    fn iso_weeks_follow_iso_8601() {
        assert_eq!(d(2026, 1, 1).iso_week(), (2026, 1));
        // January 3rd 2021 is a Sunday and belongs to the last week of 2020, which has 53 weeks.
        assert_eq!(d(2021, 1, 3).iso_week(), (2020, 53));
        assert_eq!(d(2020, 12, 31).iso_week(), (2020, 53));
        assert_eq!(d(2024, 12, 30).iso_week(), (2025, 1));
        assert_eq!(d(2026, 11, 9).iso_week(), (2026, 46));
        assert_eq!(d(2023, 1, 1).iso_week(), (2022, 52));
    }

    #[test]
    fn grid_keys_move_the_focus() {
        let date = d(2026, 11, 9);
        let ws = Weekday::Sunday;
        assert_eq!(navigate(date, GridKey::Left, ws), d(2026, 11, 8));
        assert_eq!(navigate(date, GridKey::Right, ws), d(2026, 11, 10));
        assert_eq!(navigate(date, GridKey::Up, ws), d(2026, 11, 2));
        assert_eq!(navigate(date, GridKey::Down, ws), d(2026, 11, 16));
        assert_eq!(navigate(date, GridKey::Home, ws), d(2026, 11, 8));
        assert_eq!(navigate(date, GridKey::End, ws), d(2026, 11, 14));
        assert_eq!(navigate(date, GridKey::PageUp, ws), d(2026, 10, 9));
        assert_eq!(navigate(date, GridKey::PageDown, ws), d(2026, 12, 9));
        assert_eq!(navigate(date, GridKey::ShiftPageUp, ws), d(2025, 11, 9));
        assert_eq!(navigate(date, GridKey::ShiftPageDown, ws), d(2027, 11, 9));
        assert_eq!(
            navigate(d(2026, 3, 31), GridKey::PageUp, ws),
            d(2026, 2, 28)
        );
        assert_eq!(navigate(date, GridKey::Home, Weekday::Monday), date);
    }

    #[test]
    fn grid_key_parses_key_names_and_shift() {
        assert_eq!(GridKey::from_key("ArrowLeft", false), Some(GridKey::Left));
        assert_eq!(GridKey::from_key("PageUp", false), Some(GridKey::PageUp));
        assert_eq!(
            GridKey::from_key("PageUp", true),
            Some(GridKey::ShiftPageUp)
        );
        assert_eq!(
            GridKey::from_key("PageDown", true),
            Some(GridKey::ShiftPageDown)
        );
        assert_eq!(GridKey::from_key("a", false), None);
        assert_eq!(GridKey::from_key("Enter", false), None);
        assert_eq!(GridKey::Left.direction(), -1);
        assert_eq!(GridKey::Down.direction(), 1);
        assert_eq!(GridKey::Home.direction(), -1);
        assert_eq!(GridKey::End.direction(), 1);
    }

    #[test]
    fn bounds_contain_and_clamp() {
        let bounds = Bounds {
            min: Some(d(2026, 11, 5)),
            max: Some(d(2026, 11, 20)),
        };
        assert!(bounds.contains(d(2026, 11, 5)));
        assert!(bounds.contains(d(2026, 11, 20)));
        assert!(!bounds.contains(d(2026, 11, 4)));
        assert!(!bounds.contains(d(2026, 11, 21)));
        assert_eq!(bounds.clamp(d(2026, 1, 1)), d(2026, 11, 5));
        assert_eq!(bounds.clamp(d(2027, 1, 1)), d(2026, 11, 20));
        assert_eq!(bounds.clamp(d(2026, 11, 10)), d(2026, 11, 10));
        assert!(Bounds::default().contains(d(1, 1, 1)));
    }

    #[test]
    fn find_enabled_skips_disabled_days_in_the_direction_of_travel() {
        let bounds = Bounds::default();
        // Weekends are disabled.
        let weekend =
            |date: CalendarDate| matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday);
        // Saturday the 14th: forwards lands on Monday the 16th, backwards on Friday the 13th.
        assert_eq!(
            find_enabled(d(2026, 11, 14), 1, &bounds, &weekend),
            Some(d(2026, 11, 16))
        );
        assert_eq!(
            find_enabled(d(2026, 11, 14), -1, &bounds, &weekend),
            Some(d(2026, 11, 13))
        );
        assert_eq!(
            find_enabled(d(2026, 11, 12), 1, &bounds, &weekend),
            Some(d(2026, 11, 12))
        );
    }

    #[test]
    fn find_enabled_stays_inside_the_bounds() {
        let bounds = Bounds {
            min: Some(d(2026, 11, 1)),
            max: Some(d(2026, 11, 30)),
        };
        let never = |_: CalendarDate| false;
        assert_eq!(
            find_enabled(d(2026, 12, 15), 1, &bounds, &never),
            Some(d(2026, 11, 30))
        );
        assert_eq!(
            find_enabled(d(2026, 10, 15), -1, &bounds, &never),
            Some(d(2026, 11, 1))
        );
        // Everything from the 25th to the end is disabled: nothing to land on going forwards.
        let late = |date: CalendarDate| date >= d(2026, 11, 25);
        assert_eq!(find_enabled(d(2026, 11, 26), 1, &bounds, &late), None);
        assert_eq!(
            find_enabled(d(2026, 11, 26), -1, &bounds, &late),
            Some(d(2026, 11, 24))
        );
        // Nothing is enabled at all.
        let always = |_: CalendarDate| true;
        assert_eq!(find_enabled(d(2026, 11, 10), 1, &bounds, &always), None);
    }

    #[test]
    fn range_clicks_build_ranges() {
        let first = next_range(DateRange::default(), d(2026, 11, 10));
        assert_eq!(first.start, Some(d(2026, 11, 10)));
        assert_eq!(first.end, None);
        assert!(!first.is_complete());
        let second = next_range(first, d(2026, 11, 14));
        assert_eq!(second, DateRange::new(d(2026, 11, 10), d(2026, 11, 14)));
        assert!(second.is_complete());
        // A click after a complete range starts again.
        let third = next_range(second, d(2026, 11, 20));
        assert_eq!(third.start, Some(d(2026, 11, 20)));
        assert_eq!(third.end, None);
        // A second click before the start swaps them.
        let backwards = next_range(first, d(2026, 11, 5));
        assert_eq!(backwards, DateRange::new(d(2026, 11, 5), d(2026, 11, 10)));
        // The same day twice is a one-day range.
        let one_day = next_range(first, d(2026, 11, 10));
        assert_eq!(one_day.days(), Some(1));
    }

    #[test]
    fn range_parts_mark_start_middle_and_end() {
        let range = DateRange::new(d(2026, 11, 10), d(2026, 11, 13));
        assert_eq!(range.part(d(2026, 11, 9)), None);
        assert_eq!(range.part(d(2026, 11, 10)), Some(RangePart::Start));
        assert_eq!(range.part(d(2026, 11, 11)), Some(RangePart::Middle));
        assert_eq!(range.part(d(2026, 11, 13)), Some(RangePart::End));
        assert_eq!(range.part(d(2026, 11, 14)), None);
        assert_eq!(range.days(), Some(4));
        let only_start = DateRange {
            start: Some(d(2026, 11, 10)),
            end: None,
        };
        assert_eq!(only_start.part(d(2026, 11, 10)), Some(RangePart::Only));
        assert_eq!(only_start.part(d(2026, 11, 11)), None);
        assert_eq!(only_start.days(), None);
        assert!(DateRange::default().is_empty());
        let single = DateRange::new(d(2026, 11, 10), d(2026, 11, 10));
        assert_eq!(single.part(d(2026, 11, 10)), Some(RangePart::Only));
    }

    #[test]
    fn toggling_dates_adds_removes_and_respects_the_limit() {
        let list = toggle_date(&[], d(2026, 11, 12), None);
        let list = toggle_date(&list, d(2026, 11, 5), None);
        assert_eq!(list, vec![d(2026, 11, 5), d(2026, 11, 12)]);
        let list = toggle_date(&list, d(2026, 11, 5), None);
        assert_eq!(list, vec![d(2026, 11, 12)]);
        let capped = toggle_date(&list, d(2026, 11, 20), Some(1));
        assert_eq!(capped, vec![d(2026, 11, 12)]);
        let removed = toggle_date(&capped, d(2026, 11, 12), Some(1));
        assert!(removed.is_empty());
    }

    #[test]
    fn iso_text_round_trips_and_rejects_bad_input() {
        let date = d(2026, 11, 9);
        assert_eq!(date.to_iso(), "2026-11-09");
        assert_eq!(CalendarDate::parse_iso("2026-11-09"), Some(date));
        assert_eq!(CalendarDate::parse_iso("0001-01-01"), Some(d(1, 1, 1)));
        assert_eq!(CalendarDate::parse_iso("2026-02-30"), None);
        assert_eq!(CalendarDate::parse_iso("2026-1-1"), None);
        assert_eq!(CalendarDate::parse_iso("2026/11/09"), None);
        assert_eq!(CalendarDate::parse_iso("26-11-09"), None);
        assert_eq!(CalendarDate::parse_iso("2026-11-9x"), None);
        assert_eq!(CalendarDate::parse_iso("+026-11-09"), None);
        assert_eq!(CalendarDate::parse_iso(""), None);
        assert_eq!(CalendarDate::parse_iso("0000-01-01"), None);
        assert_eq!(CalendarDate::parse_iso("2026-11-09 "), None);
    }

    #[test]
    fn names_format_dates() {
        let names = CalendarNames::english();
        let date = d(2026, 11, 9);
        assert_eq!(names.format(date, DateFormat::Long), "November 9, 2026");
        assert_eq!(names.format(date, DateFormat::Medium), "Nov 9, 2026");
        assert_eq!(names.format(date, DateFormat::Iso), "2026-11-09");
        assert_eq!(names.day_label(date), "Monday, November 9, 2026");
        assert_eq!(names.month_caption(date), "November 2026");
        assert_eq!(names.weekday_short(Weekday::Thursday), "Th");
        assert_eq!(names.month_name(12), "December");
    }

    #[test]
    fn dates_order_by_year_month_day() {
        assert!(d(2026, 11, 9) < d(2026, 11, 10));
        assert!(d(2026, 11, 30) < d(2026, 12, 1));
        assert!(d(2025, 12, 31) < d(2026, 1, 1));
    }
}
