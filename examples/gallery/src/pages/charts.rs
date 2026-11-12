use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    AreaChart, BarChart, Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader,
    CardTitle, ChartConfig, ChartContainer, ChartCurve, ChartDatum, ChartGrid, ChartLegend,
    ChartOrientation, ChartSeries, ChartTooltip, ChartTooltipIndicator, IconName, LineChart,
};

const MONTHS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];

fn visitors() -> Vec<ChartDatum> {
    let desktop = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
    let mobile = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
    MONTHS
        .iter()
        .enumerate()
        .map(|(i, m)| {
            ChartDatum::new(*m)
                .value("desktop", desktop[i])
                .value("mobile", mobile[i])
        })
        .collect()
}

fn config() -> ChartConfig {
    ChartConfig::new()
        .series("desktop", "Desktop")
        .series("mobile", "Mobile")
}

/// Revenue minus spend: positive and negative values around a zero line.
fn balance() -> Vec<ChartDatum> {
    let v = [120.0, -45.0, 80.0, -90.0, 150.0, 60.0];
    MONTHS
        .iter()
        .enumerate()
        .map(|(i, m)| ChartDatum::new(*m).value("net", v[i]))
        .collect()
}

/// Thirty days of one series, enough to need label thinning.
fn daily() -> Vec<ChartDatum> {
    (0..30)
        .map(|d| {
            let x = d as f64;
            let v = 120.0 + 60.0 * (x / 4.0).sin() + 25.0 * (x / 1.7).cos() + x * 2.0;
            ChartDatum::new(format!("Day {}", d + 1)).value("views", v.round())
        })
        .collect()
}

fn short(s: String) -> String {
    s.chars().take(3).collect()
}

fn short_cb() -> Callback<String, String> {
    Callback::new(short)
}

fn themed() -> ChartConfig {
    ChartConfig::new()
        .with(
            ChartSeries::new("desktop", "Desktop")
                .theme("oklch(0.55 0.2 260)", "oklch(0.75 0.15 260)")
                .icon(IconName::Monitor),
        )
        .with(
            ChartSeries::new("mobile", "Mobile")
                .theme("#d97706", "#fbbf24")
                .icon(IconName::Smartphone),
        )
}

#[component]
fn Demo(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>{title}</CardTitle>
                <CardDescription>{description}</CardDescription>
            </CardHeader>
            <CardContent>{children()}</CardContent>
        </Card>
    }
}

#[component]
pub fn ChartsPage() -> impl IntoView {
    let live = RwSignal::new(visitors());
    let seed = RwSignal::new(1u32);
    let randomise = move |_| {
        seed.update(|s| *s = s.wrapping_mul(1_103_515_245).wrapping_add(12_345));
        let mut s = seed.get_untracked();
        let mut next = move || {
            s = s.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            50.0 + ((s >> 16) % 250) as f64
        };
        live.set(
            MONTHS
                .iter()
                .map(|m| {
                    ChartDatum::new(*m)
                        .value("desktop", next())
                        .value("mobile", next())
                })
                .collect(),
        );
    };

    view! {
        <Page
            title="Charts"
            summary="Bar, line and area charts drawn as SVG from Rust, with no JavaScript charting library: a ChartContainer with a ChartConfig (series labels and colours, defaulting to the --chart-1..5 theme tokens), a chart, and optionally ChartTooltip and ChartLegend. Hover for the tooltip, click legend entries to hide series, or Tab to a chart and use the arrow keys, Home, End and Escape. Each chart also carries a hidden data table and reads the tooltip aloud."
        >
            <Section
                title="Bar chart"
                description="Grouped bars with the tooltip and legend (shadcn's default bar chart). The category formatter shortens the month names."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Grouped" description="January - June 2026">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                category_formatter=short_cb()
                                title="Visitors by month and device"
                                description="Desktop and mobile visitors for January to June."
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo
                        title="One series, labels, no value axis"
                        description="value_axis=false, bar_labels, a single --chart token"
                    >
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                series=vec!["desktop".to_string()]
                                value_axis=false
                                bar_labels=true
                                grid=ChartGrid::None
                                category_formatter=short_cb()
                                title="Desktop visitors"
                            />
                            <ChartTooltip hide_label=true />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Stacked" description="stacked: segments with a thin gap, only the outer end is rounded">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                stacked=true
                                category_formatter=short_cb()
                                title="Visitors by month, stacked"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="100% stacked" description="percent: every month spans the full height, the axis reads in percent">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                percent=true
                                category_formatter=short_cb()
                                title="Share of visitors by device"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Horizontal" description="orientation=Horizontal, vertical grid lines">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                orientation=ChartOrientation::Horizontal
                                category_formatter=short_cb()
                                title="Visitors by month, horizontal"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Horizontal, one series, labelled" description="Single series horizontal with value labels at the bar ends">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                series=vec!["mobile".to_string()]
                                orientation=ChartOrientation::Horizontal
                                bar_labels=true
                                value_axis=false
                                grid=ChartGrid::None
                                category_formatter=short_cb()
                                title="Mobile visitors by month, horizontal"
                            />
                            <ChartTooltip indicator=ChartTooltipIndicator::Line hide_label=true />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Negative values" description="Bars below a zero line round at their lower end">
                        <ChartContainer
                            config=ChartConfig::new().with(ChartSeries::new("net", "Net").color("var(--chart-2)"))
                            class="mx-auto"
                        >
                            <BarChart
                                data=balance()
                                category_formatter=short_cb()
                                title="Net result by month"
                            />
                            <ChartTooltip />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Axes and grid" description="category_axis=false, value_axis=false, ChartGrid::Both">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart
                                data=visitors()
                                category_axis=false
                                value_axis=false
                                grid=ChartGrid::Both
                                title="Visitors, no axes"
                            />
                            <ChartTooltip />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Line chart"
                description="Monotone curves by default (they never overshoot the data). The crosshair and a ring on each line follow the pointer."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Two series" description="Default curve, no dots">
                        <ChartContainer config=config() class="mx-auto">
                            <LineChart
                                data=visitors()
                                category_formatter=short_cb()
                                title="Visitors by month"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="With dots, linear" description="dots=true, curve=Linear">
                        <ChartContainer config=config() class="mx-auto">
                            <LineChart
                                data=visitors()
                                curve=ChartCurve::Linear
                                dots=true
                                category_formatter=short_cb()
                                title="Visitors by month, linear"
                            />
                            <ChartTooltip indicator=ChartTooltipIndicator::Line />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo
                        title="Thirty points"
                        description="Labels thin themselves out; no dots, legend omitted for one series"
                    >
                        <ChartContainer
                            config=ChartConfig::new().with(ChartSeries::new("views", "Page views").color("var(--chart-1)"))
                            class="mx-auto"
                        >
                            <LineChart
                                data=daily()
                                grid=ChartGrid::None
                                point_padding=0.0
                                title="Page views, 30 days"
                            />
                            <ChartTooltip hide_indicator=true />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Fixed domain" description="domain=(0, 400) with a custom value format">
                        <ChartContainer config=config() class="mx-auto">
                            <LineChart
                                data=visitors()
                                domain=(0.0, 400.0)
                                value_formatter=Callback::new(|v: f64| format!("{v:.0}k"))
                                category_formatter=short_cb()
                                title="Visitors in thousands"
                            />
                            <ChartTooltip />
                        </ChartContainer>
                    </Demo>
                </div>
                <h3 class="pt-2 text-sm font-medium">"Curve types"</h3>
                <div class="grid gap-6 md:grid-cols-3">
                    {[
                        ("Linear", ChartCurve::Linear),
                        ("Monotone", ChartCurve::Monotone),
                        ("Natural", ChartCurve::Natural),
                        ("Step", ChartCurve::Step),
                        ("Step before", ChartCurve::StepBefore),
                        ("Step after", ChartCurve::StepAfter),
                    ]
                        .into_iter()
                        .map(|(name, curve)| {
                            view! {
                                <Demo title=name description=format!("curve={:?}", curve)>
                                    <ChartContainer config=config() class="mx-auto">
                                        <LineChart
                                            data=visitors()
                                            series=vec!["desktop".to_string()]
                                            curve=curve
                                            dots=true
                                            width=320.0
                                            height=200.0
                                            category_formatter=short_cb()
                                            title=format!("Desktop visitors, {name} curve")
                                        />
                                        <ChartTooltip hide_label=true />
                                    </ChartContainer>
                                </Demo>
                            }
                        })
                        .collect_view()}
                </div>
            </Section>

            <Section
                title="Area chart"
                description="Areas fill down to the baseline, or stack on each other."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Overlapping" description="fill_opacity 0.4, edge to edge">
                        <ChartContainer config=config() class="mx-auto">
                            <AreaChart
                                data=visitors()
                                category_formatter=short_cb()
                                title="Visitors by month"
                            />
                            <ChartTooltip indicator=ChartTooltipIndicator::Dot />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Stacked" description="stacked=true">
                        <ChartContainer config=config() class="mx-auto">
                            <AreaChart
                                data=visitors()
                                stacked=true
                                fill_opacity=0.7
                                category_formatter=short_cb()
                                title="Visitors by month, stacked"
                            />
                            <ChartTooltip indicator=ChartTooltipIndicator::Line />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="100% stacked" description="percent=true, curve=Natural">
                        <ChartContainer config=config() class="mx-auto">
                            <AreaChart
                                data=visitors()
                                percent=true
                                curve=ChartCurve::Natural
                                fill_opacity=0.7
                                category_formatter=short_cb()
                                title="Share of visitors by device"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Step, no line, padded" description="curve=Step, line=false, point_padding=0.5, no axes">
                        <ChartContainer config=config() class="mx-auto">
                            <AreaChart
                                data=visitors()
                                curve=ChartCurve::Step
                                line=false
                                point_padding=0.5
                                category_axis=false
                                value_axis=false
                                grid=ChartGrid::None
                                title="Visitors by month, step"
                            />
                            <ChartTooltip />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Tooltip options"
                description="ChartTooltip takes indicator (dot, line, dashed), hide_label, hide_indicator, label_formatter and value_formatter."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Dot (default)" description="indicator=Dot">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Dot indicator" />
                            <ChartTooltip indicator=ChartTooltipIndicator::Dot />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Line" description="indicator=Line">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Line indicator" />
                            <ChartTooltip indicator=ChartTooltipIndicator::Line />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Dashed" description="indicator=Dashed">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Dashed indicator" />
                            <ChartTooltip indicator=ChartTooltipIndicator::Dashed />
                        </ChartContainer>
                    </Demo>
                    <Demo title="No label, no indicator" description="hide_label=true, hide_indicator=true">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Plain tooltip" />
                            <ChartTooltip hide_label=true hide_indicator=true />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Formatters" description="label_formatter and value_formatter">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Formatted tooltip" />
                            <ChartTooltip
                                label_formatter=Callback::new(|l: String| format!("Month of {l}"))
                                value_formatter=Callback::new(|v: f64| format!("{v:.0} visits"))
                            />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Icons from the config" description="Series with an icon show it instead of the swatch">
                        <ChartContainer config=themed() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Visitors with icons" />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Legend and custom config"
                description="The legend is a row of toggle buttons (aria-pressed). The last visible series cannot be hidden. Colours come from the config: any CSS colour, or a light and dark pair."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Legend above" description="ChartLegend top=true placed before the chart">
                        <ChartContainer config=config() class="mx-auto">
                            <ChartLegend top=true />
                            <LineChart data=visitors() category_formatter=short_cb() title="Visitors, legend above" />
                            <ChartTooltip />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Static legend" description="toggle=false: swatches only">
                        <ChartContainer config=config() class="mx-auto">
                            <BarChart data=visitors() category_formatter=short_cb() title="Visitors, static legend" />
                            <ChartTooltip />
                            <ChartLegend toggle=false />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Custom colours and icons" description="theme(light, dark) per series; legend with icons, switch the mode to see it">
                        <ChartContainer config=themed() class="mx-auto">
                            <AreaChart data=visitors() category_formatter=short_cb() title="Visitors, custom colours" />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Icons hidden" description="ChartLegend hide_icon=true shows swatches although the config has icons">
                        <ChartContainer config=themed() class="mx-auto">
                            <LineChart data=visitors() category_formatter=short_cb() title="Visitors, swatches" />
                            <ChartTooltip />
                            <ChartLegend hide_icon=true />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Reactive data"
                description="data is a signal: the chart redraws when it changes, and hidden series stay hidden."
            >
                <Demo title="Randomised" description="Press the button; hover and keyboard keep working">
                    <div class="mb-3">
                        <Button variant=ButtonVariant::Outline on:click=randomise>
                            "Randomise"
                        </Button>
                    </div>
                    <ChartContainer config=config() class="mx-auto">
                        <BarChart data=live category_formatter=short_cb() title="Random visitors" />
                        <ChartTooltip />
                        <ChartLegend />
                    </ChartContainer>
                </Demo>
            </Section>
        </Page>
    }
}
