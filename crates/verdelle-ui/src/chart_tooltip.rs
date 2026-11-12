use crate::ChartTooltipIndicator;
use crate::chart_context::use_chart;
use crate::chart_tooltip_content::tooltip_body;
use leptos::prelude::*;

/// The tooltip that follows the pointer over a chart (shadcn's `ChartTooltip` with
/// `ChartTooltipContent` as its content). Put it in the [`crate::ChartContainer`] next to the chart; the
/// chart tells it what to show. It is positioned with plain absolute coordinates from the pointer
/// (or from the focused data point, when the chart is used by keyboard) and opens to the other
/// side near the container's far edges. Screen readers get the same text from a live region in the chart.
///
/// The options are those of [`ChartTooltipContent`].
#[component]
pub fn ChartTooltip(
    #[prop(optional)] indicator: ChartTooltipIndicator,
    #[prop(optional)] hide_label: bool,
    #[prop(optional)] hide_indicator: bool,
    #[prop(optional, into)] label_formatter: Option<Callback<String, String>>,
    #[prop(optional, into)] value_formatter: Option<Callback<f64, String>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let ctx = use_chart();
    let class = StoredValue::new(class);
    view! {
        {move || {
            ctx.tooltip
                .get()
                .map(|state| {
                    let style = format!("left:{}px;top:{}px", state.x, state.y);
                    let flip_x = state.flip_x;
                    let flip_y = state.flip_y;
                    view! {
                        <div
                            class="vdl-chart-tooltip-anchor"
                            style=style
                            aria-hidden="true"
                            data-flip-x=flip_x.then_some("")
                            data-flip-y=flip_y.then_some("")
                        >
                            {tooltip_body(
                                &state,
                                indicator,
                                hide_label,
                                hide_indicator,
                                label_formatter,
                                value_formatter,
                                &class.get_value(),
                            )}
                        </div>
                    }
                })
        }}
    }
}
