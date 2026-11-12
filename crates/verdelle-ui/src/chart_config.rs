use crate::IconName;

/// One series of a chart (or one slice, for a pie): its key, what people read, and its colour.
///
/// The colour is any CSS colour; with none given the series takes `var(--chart-N)` by its position
/// in the [`ChartConfig`] (the five shadcn chart tokens, repeating after five), so a theme recolours the chart.
/// [`ChartSeries::theme`] gives a separate colour for dark mode.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
    pub key: String,
    pub label: String,
    pub color: Option<String>,
    pub dark_color: Option<String>,
    pub icon: Option<IconName>,
}

impl ChartSeries {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            color: None,
            dark_color: None,
            icon: None,
        }
    }

    /// A CSS colour, such as `var(--chart-3)`, `#2563eb` or `oklch(0.6 0.2 250)`.
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Separate colours for light and dark mode (shadcn's `theme: { light, dark }`).
    pub fn theme(mut self, light: impl Into<String>, dark: impl Into<String>) -> Self {
        self.color = Some(light.into());
        self.dark_color = Some(dark.into());
        self
    }

    /// An icon shown in the tooltip and legend instead of the colour swatch.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// What a chart's series are called and coloured (shadcn's `ChartConfig`). The order is the order of
/// the legend and of the default `--chart-N` colours, and the colour of a series does not change when
/// another is hidden.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartConfig {
    pub series: Vec<ChartSeries>,
}

impl ChartConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a series.
    pub fn with(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Add a series with the default colour for its position.
    pub fn series(self, key: impl Into<String>, label: impl Into<String>) -> Self {
        self.with(ChartSeries::new(key, label))
    }

    pub fn get(&self, key: &str) -> Option<&ChartSeries> {
        self.series.iter().find(|s| s.key == key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.series.iter().map(|s| s.key.clone()).collect()
    }

    /// The label for `key`, or the key itself when it is not in the config.
    pub fn label(&self, key: &str) -> String {
        self.get(key)
            .map(|s| s.label.clone())
            .unwrap_or_else(|| key.to_string())
    }

    /// The CSS colour to draw `key` with: `var(--color-<key>)`, which [`crate::ChartContainer`] defines.
    /// Keys not in the config fall back to the chart tokens by their hash-free position `fallback`.
    pub fn color_of(&self, key: &str, fallback: usize) -> String {
        if self.get(key).is_some() {
            format!("var(--color-{})", css_key(key))
        } else {
            format!("var(--chart-{})", fallback % 5 + 1)
        }
    }

    /// A stable fingerprint of the colours and keys, used to scope the container's dark-mode style.
    pub(crate) fn fingerprint(&self) -> String {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for s in &self.series {
            for b in s
                .key
                .bytes()
                .chain(s.color.iter().flat_map(|c| c.bytes()))
                .chain(s.dark_color.iter().flat_map(|c| c.bytes()))
                .chain(std::iter::once(0xff))
            {
                h ^= b as u64;
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
        }
        format!("{h:012x}")
    }
}

/// A key as it may appear in a CSS custom property name: letters, digits and dashes.
pub(crate) fn css_key(key: &str) -> String {
    key.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// One row of chart data: a name (the category on an axis, or the key of a pie slice) and its values.
///
/// ```ignore
/// ChartDatum::new("January").value("desktop", 186.0).value("mobile", 80.0)
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartDatum {
    pub name: String,
    pub values: Vec<(String, f64)>,
}

impl ChartDatum {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: Vec::new(),
        }
    }

    pub fn value(mut self, key: impl Into<String>, value: f64) -> Self {
        self.values.push((key.into(), value));
        self
    }

    pub fn get(&self, key: &str) -> Option<f64> {
        self.values.iter().find(|(k, _)| k == key).map(|(_, v)| *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_and_colours_come_from_the_config() {
        let c = ChartConfig::new()
            .series("desktop", "Desktop")
            .with(ChartSeries::new("mobile", "Mobile").color("red"));
        assert_eq!(c.label("desktop"), "Desktop");
        assert_eq!(c.label("tablet"), "tablet");
        assert_eq!(c.color_of("mobile", 0), "var(--color-mobile)");
        assert_eq!(c.color_of("tablet", 7), "var(--chart-3)");
        assert_eq!(c.keys(), vec!["desktop", "mobile"]);
    }

    #[test]
    fn keys_are_made_safe_for_css() {
        assert_eq!(css_key("page views/2"), "page-views-2");
    }

    #[test]
    fn the_fingerprint_changes_with_colours() {
        let a = ChartConfig::new().series("a", "A");
        let b = ChartConfig::new().with(ChartSeries::new("a", "A").color("red"));
        assert_ne!(a.fingerprint(), b.fingerprint());
        assert_eq!(a.fingerprint(), a.clone().fingerprint());
    }

    #[test]
    fn datum_values_are_found_by_key() {
        let d = ChartDatum::new("Jan").value("a", 1.0).value("b", 2.0);
        assert_eq!(d.get("b"), Some(2.0));
        assert_eq!(d.get("c"), None);
    }
}
