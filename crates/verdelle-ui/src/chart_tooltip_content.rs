use crate::primitives::chart::format_number;
use crate::{ChartTooltipIndicator, ChartTooltipState, Icon};
use leptos::prelude::*;

/// The inside of a chart tooltip (shadcn's `ChartTooltipContent`): the label, then a row per series
/// with its colour indicator, name and value. [`crate::ChartTooltip`] uses it for the hover tooltip;
/// use it directly to show the same layout elsewhere.
///
/// `indicator` is the shape of the colour mark; `hide_label` and `hide_indicator` drop those parts.
/// `label_formatter` and `value_formatter` change how the category and the numbers read (the default
/// value format groups thousands). With one row and a line or dashed indicator the label moves
/// inside the row, as shadcn does.
#[component]
pub fn ChartTooltipContent(
    state: ChartTooltipState,
    #[prop(optional)] indicator: ChartTooltipIndicator,
    #[prop(optional)] hide_label: bool,
    #[prop(optional)] hide_indicator: bool,
    #[prop(optional, into)] label_formatter: Option<Callback<String, String>>,
    #[prop(optional, into)] value_formatter: Option<Callback<f64, String>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    tooltip_body(
        &state,
        indicator,
        hide_label,
        hide_indicator,
        label_formatter,
        value_formatter,
        &class,
    )
}

/// The tooltip markup (shared by the component and [`crate::ChartTooltip`], which holds its options
/// as `Option`s that a component prop cannot take).
pub(crate) fn tooltip_body(
    state: &ChartTooltipState,
    indicator: ChartTooltipIndicator,
    hide_label: bool,
    hide_indicator: bool,
    label_formatter: Option<Callback<String, String>>,
    value_formatter: Option<Callback<f64, String>>,
    class: &str,
) -> AnyView {
    let label = if hide_label || state.label.is_empty() {
        None
    } else {
        Some(match label_formatter {
            Some(f) => f.run(state.label.clone()),
            None => state.label.clone(),
        })
    };
    let nest = state.items.len() == 1 && indicator != ChartTooltipIndicator::Dot;
    let label_top = if nest { None } else { label.clone() };
    let label_nested = if nest { label } else { None };
    let rows = state
        .items
        .iter()
        .map(|item| {
            let value = match value_formatter {
                Some(f) => f.run(item.value),
                None => format_number(item.value),
            };
            let mark = if let Some(icon) = item.icon {
                view! { <span class="vdl-chart-tooltip__icon"><Icon name=icon size=10 /></span> }
                    .into_any()
            } else if hide_indicator {
                ().into_any()
            } else {
                view! {
                    <span
                        class="vdl-chart-tooltip__indicator"
                        style=format!("--vdl-indicator:{}", item.color)
                    ></span>
                }
                .into_any()
            };
            view! {
                <div
                    class="vdl-chart-tooltip__item"
                    data-indicator=indicator.as_str()
                    data-nest=nest.then_some("")
                >
                    {mark}
                    <div class="vdl-chart-tooltip__row">
                        <div class="vdl-chart-tooltip__text">
                            {label_nested
                                .clone()
                                .map(|l| view! { <div class="vdl-chart-tooltip__label">{l}</div> })}
                            <span class="vdl-chart-tooltip__name">{item.name.clone()}</span>
                        </div>
                        <span class="vdl-chart-tooltip__value">{value}</span>
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class=format!("vdl-chart-tooltip {class}")>
            {label_top.map(|l| view! { <div class="vdl-chart-tooltip__label">{l}</div> })}
            <div class="vdl-chart-tooltip__items">{rows}</div>
        </div>
    }
    .into_any()
}
