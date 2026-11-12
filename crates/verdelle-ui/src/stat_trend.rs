/// The direction of a [`crate::StatCard`]'s change: which arrow its delta shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatTrend {
    /// Went up: an up arrow.
    Up,
    /// Went down: a down arrow.
    Down,
    /// Did not change: a dash.
    #[default]
    Neutral,
}

impl StatTrend {
    /// The value of the `data-trend` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            StatTrend::Up => "up",
            StatTrend::Down => "down",
            StatTrend::Neutral => "neutral",
        }
    }
}
