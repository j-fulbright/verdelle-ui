use crate::ChartLegendContent;
use leptos::prelude::*;

/// A chart's legend (shadcn's `ChartLegend` with `ChartLegendContent`): one entry per series, which
/// also toggles the series. Put it in the [`crate::ChartContainer`] after the chart for a legend below
/// it, or before for one above (`top` adds the matching space). Options are those of
/// [`ChartLegendContent`].
#[component]
pub fn ChartLegend(
    /// The legend is above the chart (shadcn's `verticalAlign="top"`): space goes below it, not above.
    #[prop(optional)]
    top: bool,
    #[prop(optional)] hide_icon: bool,
    #[prop(default = true)] toggle: bool,
    #[prop(optional, into)] keys: Vec<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <div class="vdl-chart-legend-wrap" data-align=if top { "top" } else { "bottom" }>
            <ChartLegendContent hide_icon=hide_icon toggle=toggle keys=keys class=class />
        </div>
    }
}
