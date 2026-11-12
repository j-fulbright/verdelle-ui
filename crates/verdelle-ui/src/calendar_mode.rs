/// What choosing days in a [`crate::Calendar`] does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    /// One day. Use `value` and `on_value_change`.
    #[default]
    Single,
    /// Any number of days. Use `values` and `on_values_change`.
    Multiple,
    /// A start and an end day, with the days between them shaded. Use `range` and `on_range_change`.
    Range,
}

impl CalendarMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CalendarMode::Single => "single",
            CalendarMode::Multiple => "multiple",
            CalendarMode::Range => "range",
        }
    }
}
