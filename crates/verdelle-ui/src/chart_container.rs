use crate::ChartConfig;
use crate::chart_config::css_key;
use crate::chart_context::ChartContext;
use leptos::{context::Provider, html, prelude::*};

/// The frame of a chart (shadcn's `ChartContainer`). It takes the [`ChartConfig`] (what the series are
/// called and coloured), defines a `--color-<key>` custom property for every series, and holds
/// the chart, [`crate::ChartTooltip`] and [`crate::ChartLegend`] as children:
///
/// ```ignore
/// <ChartContainer config=config>
///     <BarChart data=data title="Visitors by month" />
///     <ChartTooltip />
///     <ChartLegend />
/// </ChartContainer>
/// ```
///
/// `class` adds layout classes. The chart is an SVG sized by its `viewBox`, so it scales with the
/// container's width.
#[component]
pub fn ChartContainer(
    config: ChartConfig,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let id = format!("chart-{}", config.fingerprint());
    let mut light = String::new();
    let mut dark = String::new();
    for (i, s) in config.series.iter().enumerate() {
        let key = css_key(&s.key);
        let color = s
            .color
            .clone()
            .unwrap_or_else(|| format!("var(--chart-{})", i % 5 + 1));
        light.push_str(&format!("--color-{key}:{color};"));
        if let Some(d) = &s.dark_color {
            dark.push_str(&format!("--color-{key}:{d};"));
        }
    }
    // A style element rather than inline custom properties: inline ones would beat the `.dark` rule.
    let mut css = format!("[data-chart=\"{id}\"]{{{light}}}");
    if !dark.is_empty() {
        css.push_str(&format!(".dark [data-chart=\"{id}\"]{{{dark}}}"));
    }
    let root = NodeRef::<html::Div>::new();
    let ctx = ChartContext {
        config: StoredValue::new(config),
        hidden: RwSignal::new(Vec::new()),
        tooltip: RwSignal::new(None),
        root,
    };

    view! {
        <Provider value=ctx>
            <div class=format!("vdl-chart {class}") data-chart=id node_ref=root>
                <style inner_html=css></style>
                {children()}
            </div>
        </Provider>
    }
}
