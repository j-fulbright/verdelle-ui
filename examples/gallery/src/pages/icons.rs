use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Badge, BadgeVariant, Button, ButtonVariant, Icon, IconName, Input, InputGroup, InputGroupAddon,
    InputGroupAlign, InputGroupInput, Size,
};

/// The variant name as written in code, e.g. `ChevronRight`.
fn variant(icon: IconName) -> String {
    format!("{icon:?}")
}

#[component]
pub fn IconsPage() -> impl IntoView {
    view! {
        <Page
            title="Icons"
            summary="Icon draws one of 230+ Lucide icons (ISC) as inline SVG: typed names, 16px by default, currentColor, hidden from screen readers unless it has a label."
        >
            <Catalogue />
            <Sizes />
            <StrokeWidths />
            <Colours />
            <Accessibility />
            <Dynamic />
            <InComponents />
        </Page>
    }
}

#[component]
fn Catalogue() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let picked = RwSignal::new(IconName::Check);
    let shown = move || {
        let q = query.get().trim().to_lowercase();
        IconName::ALL
            .iter()
            .copied()
            .filter(|i| q.is_empty() || i.as_str().contains(&q))
            .collect::<Vec<_>>()
    };

    view! {
        <Section
            title="All icons"
            description="Search by name; click an icon to see how to write it. IconName::ALL lists them all, so a picker like this one is one filter away."
        >
            <div class="max-w-sm">
                <Input value=query kind="search" placeholder="Search icons..." aria_label="Search icons" cleanable=true />
            </div>
            <p class="text-sm text-muted-foreground" data-testid="icon-count">
                {move || {
                    let n = shown().len();
                    format!("{n} of {}", IconName::ALL.len())
                }}
            </p>
            <div class="flex items-center gap-3 rounded-md border border-border px-4 py-3">
                <Icon name=picked size=24 />
                <code class="text-sm" data-testid="icon-snippet">
                    {move || format!("<Icon name=IconName::{} />", variant(picked.get()))}
                </code>
                <span class="text-xs text-muted-foreground">{move || picked.get().as_str()}</span>
            </div>
            <ul
                class="grid grid-cols-[repeat(auto-fill,minmax(110px,1fr))] gap-2"
                data-testid="icon-grid"
            >
                {move || {
                    shown()
                        .into_iter()
                        .map(|icon| {
                            view! {
                                <li>
                                    <button
                                        type="button"
                                        class="flex w-full flex-col items-center gap-2 rounded-md border border-border px-2 py-3 text-xs hover:bg-accent focus-visible:outline-2 focus-visible:outline-ring"
                                        title=variant(icon)
                                        on:click=move |_| picked.set(icon)
                                    >
                                        <Icon name=icon size=24 />
                                        <span class="w-full truncate text-center text-muted-foreground">
                                            {icon.as_str()}
                                        </span>
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
            {move || {
                shown().is_empty().then(|| view! { <p class="text-sm text-muted-foreground">"No icon matches."</p> })
            }}
        </Section>
    }
}

#[component]
fn Sizes() -> impl IntoView {
    view! {
        <Section title="Size" description="size is in pixels. The default is 16, shadcn's size-4. A CSS rule on the svg wins, which is how components size their own icons.">
            <Row label="12, 16, 20, 24, 32, 48">
                {[12u32, 16, 20, 24, 32, 48]
                    .into_iter()
                    .map(|s| view! { <Icon name=IconName::Server size=s /> })
                    .collect_view()}
            </Row>
            <Row label="Default">
                <Icon name=IconName::Database />
            </Row>
            <Row label="Sized by CSS">
                <Icon name=IconName::Database class="size-8" />
                <span class="text-xs text-muted-foreground">"class=\"size-8\" beats size"</span>
            </Row>
        </Section>
    }
}

#[component]
fn StrokeWidths() -> impl IntoView {
    view! {
        <Section title="Stroke width" description="stroke_width is the line width on the 24-unit grid; the default is 2. Thin lines suit large icons.">
            <Row label="1, 1.5, 2, 2.5, 3">
                {[1.0f32, 1.5, 2.0, 2.5, 3.0]
                    .into_iter()
                    .map(|w| view! { <Icon name=IconName::Bell size=32 stroke_width=w /> })
                    .collect_view()}
            </Row>
            <Row label="Large and thin">
                <Icon name=IconName::Rocket size=64 stroke_width=1.0 />
                <Icon name=IconName::Sparkles size=64 stroke_width=1.0 />
            </Row>
        </Section>
    }
}

#[component]
fn Colours() -> impl IntoView {
    view! {
        <Section title="Colour" description="Icons draw in currentColor, so they take the text colour of whatever contains them: a utility class on the icon, or on a parent.">
            <Row label="Utility classes">
                <Icon name=IconName::CircleCheck size=20 class="text-primary" />
                <Icon name=IconName::TriangleAlert size=20 class="text-destructive" />
                <Icon name=IconName::Info size=20 class="text-muted-foreground" />
            </Row>
            <Row label="From the parent">
                <span class="inline-flex items-center gap-2 rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground">
                    <Icon name=IconName::Zap /> "On primary"
                </span>
                <span class="inline-flex items-center gap-2 rounded-md bg-destructive px-3 py-1.5 text-sm text-white">
                    <Icon name=IconName::Flame /> "On destructive"
                </span>
                <span class="inline-flex items-center gap-2 rounded-md bg-muted px-3 py-1.5 text-sm text-muted-foreground">
                    <Icon name=IconName::Cloud /> "On muted"
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn Accessibility() -> impl IntoView {
    view! {
        <Section title="Accessibility" description="Without a label the icon is aria-hidden: next to text that says the same thing, that is right. A lone icon that means something takes a label, which makes it an image with that name.">
            <Row label="Decorative">
                <span class="inline-flex items-center gap-2 text-sm">
                    <Icon name=IconName::Trash2 /> "Delete"
                </span>
                <code class="text-xs text-muted-foreground">"aria-hidden=\"true\""</code>
            </Row>
            <Row label="With a label">
                <Icon name=IconName::TriangleAlert size=20 label="Warning" class="text-destructive" />
                <code class="text-xs text-muted-foreground">"role=\"img\" aria-label=\"Warning\""</code>
            </Row>
        </Section>
    }
}

#[component]
fn Dynamic() -> impl IntoView {
    let shown = RwSignal::new(false);
    let name = Signal::derive(move || {
        if shown.get() {
            IconName::EyeOff
        } else {
            IconName::Eye
        }
    });
    view! {
        <Section title="Name as a signal" description="name accepts a signal, so one Icon switches between two icons without remounting.">
            <Row>
                <Button
                    variant=ButtonVariant::Outline
                    icon_only=true
                    aria_label="Toggle visibility"
                    on_click=move |_| shown.update(|s| *s = !*s)
                >
                    <Icon name=name />
                </Button>
                <span class="text-sm text-muted-foreground" data-testid="icon-dynamic">
                    {move || if shown.get() { "eye-off" } else { "eye" }}
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn InComponents() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    view! {
        <Section title="Inside components" description="Button, Badge and Input Group size and space an icon child themselves; the icon only needs to be there.">
            <Row label="Button">
                <Button><Icon name=IconName::Mail /> "Login with email"</Button>
                <Button variant=ButtonVariant::Outline size=Size::Small>
                    <Icon name=IconName::Download /> "Download"
                </Button>
                <Button variant=ButtonVariant::Secondary>"Next" <Icon name=IconName::ArrowRight /></Button>
                <Button variant=ButtonVariant::Destructive><Icon name=IconName::Trash2 /> "Delete"</Button>
            </Row>
            <Row label="Icon only">
                <Button variant=ButtonVariant::Outline icon_only=true aria_label="Settings">
                    <Icon name=IconName::Settings />
                </Button>
                <Button variant=ButtonVariant::Ghost icon_only=true aria_label="Notifications">
                    <Icon name=IconName::Bell />
                </Button>
                <Button icon_only=true aria_label="Add"><Icon name=IconName::Plus /></Button>
            </Row>
            <Row label="Badge">
                <Badge><Icon name=IconName::BadgeCheck /> "Verified"</Badge>
                <Badge variant=BadgeVariant::Secondary><Icon name=IconName::GitBranch /> "main"</Badge>
                <Badge variant=BadgeVariant::Destructive><Icon name=IconName::CircleAlert /> "Failed"</Badge>
                <Badge variant=BadgeVariant::Outline><Icon name=IconName::Clock /> "Pending"</Badge>
            </Row>
            <div class="grid max-w-md gap-3">
                <InputGroup>
                    <InputGroupInput value=search placeholder="Search..." aria_label="Search" />
                    <InputGroupAddon><Icon name=IconName::Search /></InputGroupAddon>
                </InputGroup>
                <InputGroup>
                    <InputGroupInput value=email kind="email" placeholder="you@example.com" aria_label="Email" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd><Icon name=IconName::Mail /></InputGroupAddon>
                </InputGroup>
            </div>
        </Section>
    }
}
