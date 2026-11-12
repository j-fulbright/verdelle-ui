use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{Badge, BadgeVariant, Button, ButtonVariant, Icon, IconName, Indicator};

const VARIANTS: [BadgeVariant; 4] = [
    BadgeVariant::Default,
    BadgeVariant::Secondary,
    BadgeVariant::Destructive,
    BadgeVariant::Outline,
];

#[component]
pub fn BadgesPage() -> impl IntoView {
    let count = RwSignal::new(3_usize);
    let clicks = RwSignal::new(0_u32);
    let tags = RwSignal::new(vec!["design", "backend", "urgent", "bug fix"]);
    let removed = RwSignal::new(0_u32);

    view! {
        <Page title="Badges and indicators" summary="A badge is a small pill label. An indicator is a count or dot overlaid on another element.">
            <Section title="Badge">
                <Row>
                    {VARIANTS
                        .into_iter()
                        .map(|v| view! { <Badge variant=v>{v.as_str()}</Badge> })
                        .collect_view()}
                </Row>
            </Section>

            <Section title="Clickable badge" description="With on_click a badge renders as a button, with hover and focus styles.">
                <Row>
                    <Badge variant=BadgeVariant::Secondary title="Click to count" on_click=move |_| clicks.update(|c| *c += 1)>
                        "clicked " {move || clicks.get()}
                    </Badge>
                </Row>
            </Section>

            <Section title="Indicator" description="The marker hides at zero. A dot always shows when dot is set.">
                <div class="flex flex-wrap items-center gap-6">
                    <Indicator count=count>
                        <Button variant=ButtonVariant::Outline icon_only=true aria_label="Notifications">"🔔"</Button>
                    </Indicator>
                    <Indicator dot=Signal::derive(move || count.get() > 0)>
                        <Button variant=ButtonVariant::Outline icon_only=true aria_label="Messages">"✉"</Button>
                    </Indicator>
                    <Indicator count=count max=9 color="var(--primary)" foreground="var(--primary-foreground)">
                        <Button variant=ButtonVariant::Secondary>"Inbox"</Button>
                    </Indicator>
                    <Button variant=ButtonVariant::Outline on_click=move |_| count.update(|n| *n += 4)>"+4"</Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| count.set(0)>"Clear"</Button>
                </div>
            </Section>
            <Section
                title="Removable badge"
                description="With on_remove a badge gets an X button with an accessible name (remove_label; make it specific). It calls on_remove and the parent drops the chip. Tab to the X, Enter or Space removes. Works with every variant and with on_click."
            >
                <Row label="Filter chips">
                    {move || {
                        tags.get()
                            .into_iter()
                            .map(|t| {
                                view! {
                                    <Badge
                                        variant=BadgeVariant::Secondary
                                        remove_label=format!("Remove filter: {t}")
                                        on_remove=move |_| tags.update(|v| v.retain(|x| *x != t))
                                    >
                                        {t}
                                    </Badge>
                                }
                            })
                            .collect_view()
                    }}
                    {move || tags.get().is_empty().then(|| view! { <span class="text-sm text-muted-foreground">"No filters"</span> })}
                    <Button
                        variant=ButtonVariant::Outline
                        size=verdelle_ui::Size::Small
                        on_click=move |_| tags.set(vec!["design", "backend", "urgent", "bug fix"])
                    >
                        "Restore"
                    </Button>
                </Row>
                <Row label="Variants">
                    {VARIANTS
                        .into_iter()
                        .map(|v| {
                            view! {
                                <Badge variant=v remove_label=format!("Remove {}", v.as_str()) on_remove=move |_| removed.update(|n| *n += 1)>
                                    {v.as_str()}
                                </Badge>
                            }
                        })
                        .collect_view()}
                    <span class="text-sm text-muted-foreground">
                        "Removed clicks: " <span id="badge-removed">{move || removed.get()}</span>
                    </span>
                </Row>
                <Row label="With icon and click">
                    <Badge variant=BadgeVariant::Outline on_remove=move |_| removed.update(|n| *n += 1)>
                        <Icon name=IconName::Tag />
                        "release"
                    </Badge>
                    <Badge
                        variant=BadgeVariant::Secondary
                        title="Click the label to edit"
                        on_click=move |_| clicks.update(|n| *n += 1)
                        on_remove=move |_| removed.update(|n| *n += 1)
                    >
                        "Edit me"
                    </Badge>
                    <span class="text-sm text-muted-foreground">"Label clicks: " {move || clicks.get()}</span>
                </Row>
            </Section>
        </Page>
    }
}
