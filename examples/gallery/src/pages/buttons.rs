use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use std::time::Duration;
use verdelle_ui::{Button, ButtonType, ButtonVariant, Icon, IconName, Size};

const VARIANTS: [ButtonVariant; 6] = [
    ButtonVariant::Default,
    ButtonVariant::Outline,
    ButtonVariant::Secondary,
    ButtonVariant::Ghost,
    ButtonVariant::Destructive,
    ButtonVariant::Link,
];

const SIZES: [(Size, &str); 4] = [
    (Size::XSmall, "xs (24px)"),
    (Size::Small, "sm (28px)"),
    (Size::Medium, "md (32px, default)"),
    (Size::Large, "lg (36px)"),
];

#[component]
pub fn ButtonsPage() -> impl IntoView {
    let loading = RwSignal::new(false);
    let link_clicks = RwSignal::new(0_u32);
    let submits = RwSignal::new(0_u32);
    let saving = RwSignal::new(false);
    let href = RwSignal::new(Some("#link-demo".to_string()));
    let tip = RwSignal::new(None::<String>);

    view! {
        <Page title="Buttons" summary="Six variants, four sizes, and the selected, disabled and loading states.">
            <Section title="Variants and states">
                {VARIANTS
                    .into_iter()
                    .map(|v| {
                        view! {
                            <Row label=v.as_str()>
                                <Button variant=v>"Button"</Button>
                                <Button variant=v selected=true>"Selected"</Button>
                                <Button variant=v disabled=true>"Disabled"</Button>
                                <Button variant=v loading=true>"Loading"</Button>
                                <Button variant=v icon_only=true aria_label="Add">"+"</Button>
                            </Row>
                        }
                    })
                    .collect_view()}
            </Section>

            <Section title="Sizes">
                {SIZES
                    .into_iter()
                    .map(|(s, label)| {
                        view! {
                            <Row label=label>
                                <Button size=s>"Default"</Button>
                                <Button variant=ButtonVariant::Outline size=s>"Outline"</Button>
                                <Button variant=ButtonVariant::Secondary size=s icon_only=true aria_label="Add">"+"</Button>
                            </Row>
                        }
                    })
                    .collect_view()}
            </Section>

            <Section
                title="Layout classes"
                description="The class prop takes utility classes, which override the component's own styles."
            >
                <div class="flex max-w-md gap-2">
                    <Button class="flex-1">"flex-1"</Button>
                    <Button variant=ButtonVariant::Outline>"Auto"</Button>
                </div>
                <Button variant=ButtonVariant::Secondary class="w-full max-w-md">"w-full"</Button>
            </Section>

            <Section title="Loading" description="A loading button keeps its colours, shows a spinner and ignores clicks.">
                <Row>
                    <Button loading=loading>"Saving"</Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| loading.update(|l| *l = !*l)>
                        "Toggle loading"
                    </Button>
                </Row>
            </Section>

            <Section
                title="Link buttons"
                description="With href a Button renders an <a> with the same classes, variants, sizes and states. Tab to focus it and Enter follows the link. A disabled or loading link has no href, aria-disabled and tabindex -1, so it cannot be followed. target=_blank adds rel=noopener noreferrer."
            >
                <Row label="Variants">
                    {VARIANTS
                        .into_iter()
                        .map(|v| {
                            view! {
                                <Button variant=v href="#link-demo" on_click=move |_| link_clicks.update(|n| *n += 1)>
                                    {v.as_str()}
                                </Button>
                            }
                        })
                        .collect_view()}
                </Row>
                <Row label="Sizes">
                    {SIZES
                        .into_iter()
                        .map(|(s, label)| {
                            view! {
                                <Button variant=ButtonVariant::Outline size=s href="#link-demo">
                                    {label}
                                </Button>
                            }
                        })
                        .collect_view()}
                </Row>
                <Row label="External (new tab)">
                    <Button variant=ButtonVariant::Outline href="https://example.com" target="_blank" title="Opens in a new tab">
                        "Docs"
                        <Icon name=IconName::ExternalLink />
                    </Button>
                    <Button variant=ButtonVariant::Link href="https://example.com" target="_blank" rel="nofollow noopener">
                        "Custom rel"
                    </Button>
                </Row>
                <Row label="Icon only">
                    <Button variant=ButtonVariant::Outline icon_only=true href="#link-demo" aria_label="Home" title="Home">
                        <Icon name=IconName::House />
                    </Button>
                </Row>
                <Row label="States">
                    <Button href="#link-demo" selected=true>"Selected"</Button>
                    <Button href="#link-demo" disabled=true>"Disabled link"</Button>
                    <Button href="#link-demo" loading=true>"Loading link"</Button>
                </Row>
                <p id="link-demo" class="text-sm text-muted-foreground">
                    "Link target. Enabled link clicks: "
                    <span id="link-clicks">{move || link_clicks.get()}</span>
                    " (a disabled or loading link adds nothing and does not change the address)."
                </p>
            </Section>

            <Section
                title="Type, title and loading in a form"
                description="kind is the HTML type: Button (default, never submits), Submit or Reset. title is the native tooltip. A loading submit button keeps its label, shows the spinner and does not submit again; here Save loads for a second after each submit."
            >
                <form
                    class="flex flex-wrap items-center gap-3"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        submits.update(|n| *n += 1);
                        saving.set(true);
                        set_timeout(move || saving.set(false), Duration::from_millis(1200));
                    }
                >
                    <Button kind=ButtonType::Submit loading=saving title="Submits the form">"Save"</Button>
                    <Button kind=ButtonType::Submit variant=ButtonVariant::Secondary loading=true>"Submit while loading"</Button>
                    <Button kind=ButtonType::Reset variant=ButtonVariant::Outline>"Reset"</Button>
                    <Button variant=ButtonVariant::Ghost title="type=button: does not submit">"Plain button"</Button>
                    <span class="text-sm text-muted-foreground">
                        "Submits: " <span id="form-submits">{move || submits.get()}</span>
                    </span>
                </form>
            </Section>

            <Section
                title="Passing optional props conditionally"
                description="href, target, rel, title and aria_label take a plain value, an Option, or a signal of an Option, so there is no need for two view branches. Here href and title are signals; None gives an anchor with no href (inert). Whether a Button is a link is decided when it is built, so to switch between a link and a real button, key the view."
            >
                <Row>
                    <Button variant=ButtonVariant::Outline href=href title=tip>"Conditional link"</Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=move |_| {
                            href.update(|h| *h = if h.is_some() { None } else { Some("#link-demo".to_string()) });
                            tip.update(|t| *t = if t.is_some() { None } else { Some("Now it has a tooltip".to_string()) });
                        }
                    >
                        "Toggle href and title"
                    </Button>
                    <span class="text-sm text-muted-foreground">
                        "href: " <span id="cond-href">{move || format!("{:?}", href.get())}</span>
                    </span>
                </Row>
            </Section>

            <Section
                title="Leptos note: Fn children and optional props"
                description="Two things that trip up newcomers when passing data into components."
            >
                <div class="grid max-w-3xl gap-4 text-sm">
                    <div class="min-w-0 space-y-2">
                        <p class="font-medium">"1. Children closures must be Fn"</p>
                        <p class="text-muted-foreground">
                            "Components that take children as ChildrenFn (Show, Collapsible content, dialog bodies, and any component that may rebuild its children) call them more than once, so the closure may not move a value out. Wrap data you need inside in a StoredValue (or an Arc, or a signal) and read it with with_value / get_value."
                        </p>
                        <pre class="overflow-x-auto rounded-md bg-muted p-3 text-xs">{FN_NOTE}</pre>
                    </div>
                    <div class="min-w-0 space-y-2">
                        <p class="font-medium">"2. Optional props: Option, MaybeProp and into"</p>
                        <p class="text-muted-foreground">
                            "A prop declared #[prop(optional, into)] x: MaybeProp<T> accepts T, Option<T>, a signal of Option<T> and a closure, so a conditional value is one expression. For an Option<String> that is not a signal, pass it as is. Verdelle's href, title, target, rel, aria_label (Button), icon (Alert, StatCard, ...), description and back_href (PageHeader), delta (StatCard) and name (AvatarFallback) work this way."
                        </p>
                        <pre class="overflow-x-auto rounded-md bg-muted p-3 text-xs">{OPTIONAL_NOTE}</pre>
                    </div>
                </div>
            </Section>
        </Page>
    }
}

const FN_NOTE: &str = r#"// Does not compile: `name` is moved into the first call of the closure.
// view! { <Collapsible>{move || view! { <p>{name}</p> }}</Collapsible> }

let name = StoredValue::new(name); // String is not Copy; StoredValue is.
view! {
    <CollapsibleContent>
        {move || view! { <p>{name.get_value()}</p> }}
    </CollapsibleContent>
}"#;

const OPTIONAL_NOTE: &str = r#"let href: Option<String> = maybe_url();          // plain Option
let tip = RwSignal::new(None::<String>);           // a signal of an Option

// One view, no `if let Some(..) { <Button href=..> } else { <Button> }`:
view! { <Button href=href title=tip>"Open"</Button> }

// Or compute it: any closure of Option<String> works through Signal::derive.
view! { <Button title=Signal::derive(move || row.get().note)>"Open"</Button> }"#;
