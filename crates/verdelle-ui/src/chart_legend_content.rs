use crate::Icon;
use crate::chart_context::use_chart;
use leptos::prelude::*;

/// The list of series in a legend (shadcn's `ChartLegendContent`): a colour swatch (or the series'
/// icon) and the label from the [`crate::ChartConfig`]. With `toggle` (the default) every entry is a
/// button with `aria-pressed` that hides or shows its series in the charts; the last visible series
/// cannot be hidden. `keys` limits and orders the entries (default, when empty: every series in the config);
/// `hide_icon` shows swatches even for series with an icon. Use inside a [`crate::ChartContainer`].
#[component]
pub fn ChartLegendContent(
    #[prop(optional)] hide_icon: bool,
    #[prop(default = true)] toggle: bool,
    #[prop(optional, into)] keys: Vec<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let ctx = use_chart();
    let all = if keys.is_empty() {
        ctx.cfg().keys()
    } else {
        keys
    };
    let entries = all
        .iter()
        .enumerate()
        .map(|(i, key)| {
            let label = ctx.cfg().label(key);
            let color = ctx.cfg().color_of(key, i);
            let icon = ctx.cfg().get(key).and_then(|s| s.icon);
            let mark = match icon {
                Some(icon) if !hide_icon => {
                    view! { <span class="vdl-chart-legend__icon"><Icon name=icon size=12 /></span> }
                        .into_any()
                }
                _ => view! {
                    <span
                        class="vdl-chart-legend__swatch"
                        style=format!("--vdl-indicator:{color}")
                    ></span>
                }
                .into_any(),
            };
            let key = StoredValue::new(key.clone());
            let all = StoredValue::new(all.clone());
            let hidden = move || ctx.is_hidden(&key.get_value());
            if toggle {
                view! {
                    <li>
                        <button
                            type="button"
                            class="vdl-chart-legend__item"
                            aria-pressed=move || (!hidden()).to_string()
                            data-hidden=move || hidden().then_some("")
                            on:click=move |_| ctx.toggle(&key.get_value(), &all.get_value())
                        >
                            {mark}
                            <span>{label}</span>
                        </button>
                    </li>
                }
                .into_any()
            } else {
                view! {
                    <li class="vdl-chart-legend__item">
                        {mark}
                        <span>{label}</span>
                    </li>
                }
                .into_any()
            }
        })
        .collect_view();
    view! { <ul class=format!("vdl-chart-legend {class}")>{entries}</ul> }
}
