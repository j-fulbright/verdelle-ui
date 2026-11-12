use crate::{ChartConfig, IconName};
use leptos::{html, prelude::*};

/// One line of a chart tooltip: a series (or slice), its colour and its value.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartTooltipItem {
    pub key: String,
    /// The label from the [`ChartConfig`].
    pub name: String,
    pub value: f64,
    /// A CSS colour (`var(--color-desktop)`).
    pub color: String,
    pub icon: Option<IconName>,
}

/// What a chart's tooltip shows right now, and where. A chart sets it on hover or when a data point
/// is focused with the keyboard; [`crate::ChartTooltip`] draws it.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartTooltipState {
    /// The category (or the slice's label).
    pub label: String,
    pub items: Vec<ChartTooltipItem>,
    /// Where to draw it, in pixels from the container's top left corner.
    pub x: f64,
    pub y: f64,
    /// The tooltip opens to the left of / above the point when the point is in the far half.
    pub flip_x: bool,
    pub flip_y: bool,
}

/// What a chart's parts share: the config, which series the legend has hidden, the tooltip.
#[derive(Clone, Copy)]
pub(crate) struct ChartContext {
    pub config: StoredValue<ChartConfig>,
    pub hidden: RwSignal<Vec<String>>,
    pub tooltip: RwSignal<Option<ChartTooltipState>>,
    pub root: NodeRef<html::Div>,
}

impl ChartContext {
    pub fn cfg(&self) -> ChartConfig {
        self.config.get_value()
    }

    pub fn is_hidden(&self, key: &str) -> bool {
        self.hidden.get().iter().any(|k| k == key)
    }

    /// The keys of `all` that are not hidden.
    pub fn visible(&self, all: &[String]) -> Vec<String> {
        let hidden = self.hidden.get();
        all.iter()
            .filter(|k| !hidden.contains(k))
            .cloned()
            .collect()
    }

    /// Hide or show `key`. The last visible series cannot be hidden: an empty chart says nothing.
    pub fn toggle(&self, key: &str, all: &[String]) {
        self.hidden.update(|h| {
            if let Some(i) = h.iter().position(|k| k == key) {
                h.remove(i);
            } else if all.iter().filter(|k| !h.contains(k)).count() > 1 {
                h.push(key.to_string());
            }
        });
    }
}

pub(crate) fn use_chart() -> ChartContext {
    use_context::<ChartContext>().expect(
        "chart parts (ChartTooltip, ChartLegend, the charts) must be inside a ChartContainer",
    )
}
