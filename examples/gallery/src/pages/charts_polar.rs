use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Card, CardContent, CardDescription, CardHeader, CardTitle, ChartConfig, ChartContainer,
    ChartDatum, ChartLegend, ChartSeries, ChartTooltip, ChartTooltipIndicator, PieChart,
    RadarChart, RadarGrid, RadialBarChart,
};

fn browsers() -> Vec<ChartDatum> {
    [
        ("chrome", 275.0),
        ("safari", 200.0),
        ("firefox", 187.0),
        ("edge", 173.0),
        ("other", 90.0),
    ]
    .into_iter()
    .map(|(k, v)| ChartDatum::new(k).value("visitors", v))
    .collect()
}

fn browser_config() -> ChartConfig {
    ChartConfig::new()
        .series("chrome", "Chrome")
        .series("safari", "Safari")
        .series("firefox", "Firefox")
        .series("edge", "Edge")
        .series("other", "Other")
}

fn skills() -> Vec<ChartDatum> {
    [
        ("Speed", 86.0, 62.0),
        ("Reliability", 70.0, 85.0),
        ("Comfort", 92.0, 55.0),
        ("Safety", 64.0, 90.0),
        ("Range", 78.0, 70.0),
        ("Value", 55.0, 80.0),
    ]
    .into_iter()
    .map(|(k, a, b)| ChartDatum::new(k).value("alpha", a).value("beta", b))
    .collect()
}

fn skill_config() -> ChartConfig {
    ChartConfig::new()
        .series("alpha", "Model A")
        .series("beta", "Model B")
}

fn device_split() -> Vec<ChartDatum> {
    vec![
        ChartDatum::new("usage")
            .value("desktop", 1260.0)
            .value("mobile", 570.0),
    ]
}

fn device_config() -> ChartConfig {
    ChartConfig::new()
        .series("desktop", "Desktop")
        .series("mobile", "Mobile")
}

fn total_text() -> String {
    let total: f64 = browsers().iter().filter_map(|d| d.get("visitors")).sum();
    format!("{total:.0}")
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
pub fn ChartsPolarPage() -> impl IntoView {
    let custom = ChartConfig::new()
        .with(ChartSeries::new("chrome", "Chrome").color("#2563eb"))
        .with(ChartSeries::new("safari", "Safari").color("#16a34a"))
        .with(ChartSeries::new("firefox", "Firefox").theme("#ea580c", "#fb923c"))
        .with(ChartSeries::new("edge", "Edge").color("#9333ea"))
        .with(ChartSeries::new("other", "Other").color("#64748b"));

    view! {
        <Page
            title="Charts: pie, radial and radar"
            summary="Circular charts on the same ChartContainer, ChartConfig, ChartTooltip and ChartLegend as the cartesian ones. Hover a slice or ring, click legend entries to drop one (the rest share the circle), or Tab to the chart and use the arrow keys."
        >
            <Section
                title="Pie chart"
                description="Each datum is a slice named by its config key. The tooltip shows the slice with its colour; hovering dims the others."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Pie" description="hide_label on the tooltip, legend toggles slices">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <PieChart data=browsers() title="Visitors by browser" />
                            <ChartTooltip hide_label=true />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Value labels, pad angle" description="labels=true, pad_angle=3">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <PieChart
                                data=browsers()
                                labels=true
                                pad_angle=3.0
                                title="Visitors by browser, labelled"
                            />
                            <ChartTooltip hide_label=true indicator=ChartTooltipIndicator::Line />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Custom colours" description="Hex colours and a light/dark pair for Firefox">
                        <ChartContainer config=custom class="mx-auto">
                            <PieChart data=browsers() title="Visitors by browser, custom colours" />
                            <ChartTooltip hide_label=true indicator=ChartTooltipIndicator::Dashed />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Half circle" description="start_angle=-90, end_angle=90, a shorter height">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <PieChart
                                data=browsers()
                                inner_radius=0.55
                                start_angle=-90.0
                                end_angle=90.0
                                height=190.0
                                center_value=total_text()
                                center_label="Visitors"
                                title="Visitors by browser, half circle"
                            />
                            <ChartTooltip hide_label=true />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Donut chart"
                description="inner_radius cuts a hole; center_value and center_label put text in it."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Donut with a total" description="inner_radius=0.62, pad_angle=2">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <PieChart
                                data=browsers()
                                inner_radius=0.62
                                pad_angle=2.0
                                center_value=total_text()
                                center_label="Visitors"
                                title="Visitors by browser, donut"
                            />
                            <ChartTooltip hide_label=true />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Thin ring, no centre text" description="inner_radius=0.8">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <PieChart
                                data=browsers()
                                inner_radius=0.8
                                pad_angle=2.0
                                title="Visitors by browser, thin donut"
                            />
                            <ChartTooltip hide_label=true />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Radial bar chart"
                description="Rings (the first outermost), each an arc of its value against max; or one stacked ring."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Rings" description="Track behind each ring, centre label">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <RadialBarChart
                                data=browsers()
                                center_value="5 rings"
                                center_label="by visitors"
                                title="Visitors by browser, rings"
                            />
                            <ChartTooltip hide_label=true />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Partial sweep, no track" description="end_angle=250, track=false, max=300, bar_gap=0.4">
                        <ChartContainer config=browser_config() class="mx-auto">
                            <RadialBarChart
                                data=browsers()
                                end_angle=250.0
                                track=false
                                max=300.0
                                bar_gap=0.4
                                inner_radius=0.2
                                title="Visitors by browser, partial rings"
                            />
                            <ChartTooltip hide_label=true />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Stacked half ring" description="stacked=true, the first datum split between its series">
                        <ChartContainer config=device_config() class="mx-auto">
                            <RadialBarChart
                                data=device_split()
                                stacked=true
                                start_angle=-90.0
                                end_angle=90.0
                                height=190.0
                                inner_radius=0.5
                                center_value="1,830"
                                center_label="Visitors"
                                title="Visitors by device, stacked ring"
                            />
                            <ChartTooltip hide_label=true />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>

            <Section
                title="Radar chart"
                description="The categories are the axes. Hover near an axis for every series on it."
            >
                <div class="grid gap-6 md:grid-cols-2">
                    <Demo title="Polygon grid" description="Two series, filled">
                        <ChartContainer config=skill_config() class="mx-auto">
                            <RadarChart data=skills() title="Model comparison" />
                            <ChartTooltip indicator=ChartTooltipIndicator::Line />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Circle grid with dots" description="grid_shape=Circle, dots=true, fill_opacity=0.3">
                        <ChartContainer config=skill_config() class="mx-auto">
                            <RadarChart
                                data=skills()
                                grid_shape=RadarGrid::Circle
                                dots=true
                                fill_opacity=0.3
                                title="Model comparison, circle grid"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="Lines only, no spokes" description="filled=false, spokes=false">
                        <ChartContainer config=skill_config() class="mx-auto">
                            <RadarChart
                                data=skills()
                                filled=false
                                spokes=false
                                dots=true
                                title="Model comparison, lines"
                            />
                            <ChartTooltip />
                            <ChartLegend />
                        </ChartContainer>
                    </Demo>
                    <Demo title="One series, fixed max" description="series=[alpha], max=100">
                        <ChartContainer config=skill_config() class="mx-auto">
                            <RadarChart
                                data=skills()
                                series=vec!["alpha".to_string()]
                                max=100.0
                                title="Model A"
                            />
                            <ChartTooltip hide_indicator=true />
                        </ChartContainer>
                    </Demo>
                </div>
            </Section>
        </Page>
    }
}
