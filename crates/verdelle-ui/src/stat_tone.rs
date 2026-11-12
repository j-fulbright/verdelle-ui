use crate::StatTrend;

/// The colour of a [`crate::StatCard`]'s delta: whether the change is good news.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatTone {
    /// Follow the trend: up is positive, down is negative, flat is neutral.
    #[default]
    Auto,
    /// Good news (green), whichever way the number moved: use it for a falling cost or error rate.
    Positive,
    /// Bad news (destructive red), whichever way the number moved.
    Negative,
    /// Plain muted text.
    Neutral,
}

impl StatTone {
    /// The tone that is actually drawn, given the trend (`Auto` follows it).
    pub fn resolve(self, trend: StatTrend) -> StatTone {
        match (self, trend) {
            (StatTone::Auto, StatTrend::Up) => StatTone::Positive,
            (StatTone::Auto, StatTrend::Down) => StatTone::Negative,
            (StatTone::Auto, StatTrend::Neutral) => StatTone::Neutral,
            (other, _) => other,
        }
    }

    /// The value of the `data-tone` attribute (the resolved tone's name).
    pub fn as_str(self) -> &'static str {
        match self {
            StatTone::Auto => "auto",
            StatTone::Positive => "positive",
            StatTone::Negative => "negative",
            StatTone::Neutral => "neutral",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_follows_the_trend_and_explicit_tones_win() {
        assert_eq!(StatTone::Auto.resolve(StatTrend::Up), StatTone::Positive);
        assert_eq!(StatTone::Auto.resolve(StatTrend::Down), StatTone::Negative);
        assert_eq!(
            StatTone::Auto.resolve(StatTrend::Neutral),
            StatTone::Neutral
        );
        // A falling error rate is good news.
        assert_eq!(
            StatTone::Positive.resolve(StatTrend::Down),
            StatTone::Positive
        );
        assert_eq!(
            StatTone::Negative.resolve(StatTrend::Up),
            StatTone::Negative
        );
    }
}
