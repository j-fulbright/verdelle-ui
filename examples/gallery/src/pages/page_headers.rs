use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, Button, ButtonVariant, Icon, IconName, PageHeader, Size, StatCard,
    StatTone, StatTrend,
};

#[component]
pub fn PageHeadersPage() -> impl IntoView {
    view! {
        <Page
            title="Page header and stat card"
            summary="PageHeader is the top of a page: title, description, actions and a back link or breadcrumb, wrapping on a phone. StatCard is a tile with one number, an optional delta, icon and footer, and a toggle form for filter tiles. Both are Verdelle extras."
        >
            <HeaderExamples />
            <StatExamples />
        </Page>
    }
}

/// A bordered frame so each header can be judged on its own.
#[component]
fn Frame(children: Children) -> impl IntoView {
    view! { <div class="rounded-lg border border-border p-4">{children()}</div> }
}

#[component]
fn HeaderExamples() -> impl IntoView {
    let title = RwSignal::new(String::from("Invoices"));
    let note = RwSignal::new(Some(String::from("Everything billed this quarter.")));
    let back = RwSignal::new(Some(String::from("#back-target")));

    view! {
        <Section title="Page header" description="Title and description take plain values or signals; the actions wrap below the title when the row is too narrow (try the 390px width).">
            <div class="grid gap-4">
                <Frame>
                    <PageHeader title="Title only" level=2 />
                </Frame>
                <Frame>
                    <PageHeader title="With a description" description="A line of muted text under the title that explains the page." level=2 />
                </Frame>
                <Frame>
                    <PageHeader
                        title="With actions"
                        description="Actions sit beside the title and drop below it on a phone."
                        level=2
                        actions=|| {
                            view! {
                                <Button variant=ButtonVariant::Outline>"Export"</Button>
                                <Button>
                                    <Icon name=IconName::Plus />
                                    "New invoice"
                                </Button>
                            }
                        }
                    />
                </Frame>
                <Frame>
                    <PageHeader
                        title="With a back link"
                        description="back_href adds a link above the title; back_label words it."
                        back_href="#back-target"
                        back_label="Back to projects"
                        level=2
                        actions=|| view! { <Button variant=ButtonVariant::Outline>"Edit"</Button> }
                    />
                </Frame>
                <Frame>
                    <PageHeader
                        title="With a breadcrumb"
                        description="The breadcrumb slot takes a Breadcrumb; it wins over back_href."
                        back_href="#ignored"
                        level=2
                        breadcrumb=|| {
                            view! {
                                <Breadcrumb>
                                    <BreadcrumbList>
                                        <BreadcrumbItem><BreadcrumbLink href="#">"Home"</BreadcrumbLink></BreadcrumbItem>
                                        <BreadcrumbSeparator />
                                        <BreadcrumbItem><BreadcrumbLink href="#">"Projects"</BreadcrumbLink></BreadcrumbItem>
                                        <BreadcrumbSeparator />
                                        <BreadcrumbItem><BreadcrumbPage>"Apollo"</BreadcrumbPage></BreadcrumbItem>
                                    </BreadcrumbList>
                                </Breadcrumb>
                            }
                        }
                        actions=|| {
                            view! {
                                <Button variant=ButtonVariant::Outline>"Share"</Button>
                                <Button variant=ButtonVariant::Secondary>"Settings"</Button>
                                <Button>"Deploy"</Button>
                            }
                        }
                    />
                </Frame>
                <Frame>
                    <PageHeader
                        title="A very long page title that has to wrap onto several lines on a narrow screen without breaking the layout"
                        description="Long titles wrap and stay balanced; the actions keep their own row."
                        level=2
                        actions=|| view! { <Button>"Primary action"</Button> }
                    />
                </Frame>
                <Frame>
                    <PageHeader
                        title=title
                        description=note
                        back_href=back
                        level=2
                        actions=move || {
                            view! {
                                <Button
                                    variant=ButtonVariant::Outline
                                    size=Size::Small
                                    on_click=move |_| title.update(|t| *t = if t == "Invoices" { "Receipts".into() } else { "Invoices".into() })
                                >
                                    "Rename"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Outline
                                    size=Size::Small
                                    on_click=move |_| note.update(|n| *n = if n.is_some() { None } else { Some("Everything billed this quarter.".into()) })
                                >
                                    "Toggle description"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Outline
                                    size=Size::Small
                                    on_click=move |_| back.update(|b| *b = if b.is_some() { None } else { Some("#back-target".into()) })
                                >
                                    "Toggle back link"
                                </Button>
                            }
                        }
                    />
                    <p id="back-target" class="mt-3 text-xs text-muted-foreground">"Signals: the three buttons change the title, the description (an Option) and the back link (an Option)."</p>
                </Frame>
            </div>
        </Section>
    }
}

#[component]
fn StatExamples() -> impl IntoView {
    let open = RwSignal::new(true);
    let overdue = RwSignal::new(false);
    let paid = RwSignal::new(false);
    let toggles = RwSignal::new(0_u32);
    let revenue = RwSignal::new(String::from("$12,480"));

    view! {
        <Section title="Stat card" description="A label, a large value and optional extras. The value is a string you format; label, value, delta and icon can be signals.">
            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <StatCard label="Plain" value="1,284" />
                <StatCard label="With an icon" value=revenue icon=IconName::DollarSign />
                <StatCard
                    label="With a footer"
                    value="98.2%"
                    icon=IconName::Activity
                    footer=|| "Uptime over the last 30 days"
                />
            </div>
        </Section>

        <Section title="Delta trend and tone" description="The arrow follows trend; the colour follows tone, which by default follows trend (up green, down red, flat muted). Set tone when the sign and the news disagree: a falling error rate is good.">
            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <StatCard label="Up" value="2,410" delta="+12.5%" trend=StatTrend::Up footer=|| "vs. last month" />
                <StatCard label="Down" value="318" delta="-4.1%" trend=StatTrend::Down footer=|| "vs. last month" />
                <StatCard label="Neutral" value="64" delta="0%" trend=StatTrend::Neutral footer=|| "vs. last month" />
                <StatCard label="Error rate (down is good)" value="0.8%" delta="-0.3 pts" trend=StatTrend::Down tone=StatTone::Positive footer=|| "tone=Positive" />
                <StatCard label="Latency (up is bad)" value="420 ms" delta="+60 ms" trend=StatTrend::Up tone=StatTone::Negative footer=|| "tone=Negative" />
                <StatCard label="Up, plain" value="7" delta="+1" trend=StatTrend::Up tone=StatTone::Neutral footer=|| "tone=Neutral" />
            </div>
        </Section>

        <Section title="Small and changing values" description="small uses 12px padding. The value, delta and icon follow signals.">
            <Row>
                <div class="w-56">
                    <StatCard small=true label="Revenue" value=revenue delta="+3.2%" trend=StatTrend::Up icon=IconName::DollarSign />
                </div>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    on_click=move |_| revenue.update(|r| *r = if r == "$12,480" { "$13,905".into() } else { "$12,480".into() })
                >
                    "Change value"
                </Button>
            </Row>
        </Section>

        <Section
            title="Selectable tiles"
            description="Pass pressed (an RwSignal<bool>) and the whole tile is a toggle button with aria-pressed, for filter tiles. Tab to focus, Space or Enter toggles; on_toggle gets the new state."
        >
            <div class="grid gap-4 sm:grid-cols-3">
                <StatCard label="Open" value="12" icon=IconName::CircleDot pressed=open on_toggle=move |_| toggles.update(|n| *n += 1) />
                <StatCard label="Overdue" value="3" delta="+2" trend=StatTrend::Up tone=StatTone::Negative icon=IconName::Clock pressed=overdue on_toggle=move |_| toggles.update(|n| *n += 1) />
                <StatCard label="Paid" value="41" icon=IconName::CircleCheck pressed=paid footer=|| "Last 30 days" on_toggle=move |_| toggles.update(|n| *n += 1) />
            </div>
            <p class="text-sm text-muted-foreground">
                "Filters on: "
                <span id="stat-filters">
                    {move || {
                        let mut on = Vec::new();
                        if open.get() { on.push("open"); }
                        if overdue.get() { on.push("overdue"); }
                        if paid.get() { on.push("paid"); }
                        if on.is_empty() { "none".to_string() } else { on.join(", ") }
                    }}
                </span>
                ", toggles: " <span id="stat-toggles">{move || toggles.get()}</span>
            </p>
        </Section>
    }
}
