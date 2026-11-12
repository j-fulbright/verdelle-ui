use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Align, Avatar, AvatarFallback, Button, ButtonVariant, HoverCard, HoverCardContent,
    HoverCardTrigger, Side, Size,
};

fn now_ms() -> f64 {
    leptos::web_sys::js_sys::Date::now()
}

#[component]
pub fn HoverCardsPage() -> impl IntoView {
    view! {
        <Page
            title="Hover cards"
            summary="A preview panel for a link. It opens when the pointer rests on the link or the link is focused with the keyboard, and closes when the pointer has left both the link and the panel, on Escape, or on a press outside. Not for touch, and never for anything essential."
        >
            <BasicSection />
            <PlacementSection />
            <DelaySection />
            <OpenStateSection />
            <CollisionSection />
        </Page>
    }
}

#[component]
fn Profile(
    initials: &'static str,
    name: &'static str,
    handle: &'static str,
    bio: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex justify-between gap-4">
            <div class="shrink-0">
                <Avatar>
                    <AvatarFallback>{initials}</AvatarFallback>
                </Avatar>
            </div>
            <div class="space-y-1">
                <h4 class="text-sm font-semibold">{handle}</h4>
                <p class="text-sm">{bio}</p>
                <div class="text-xs text-muted-foreground">{name}</div>
            </div>
        </div>
    }
}

#[component]
fn BasicSection() -> impl IntoView {
    view! {
        <Section
            title="Basic"
            description="A link with an avatar-and-text card (shadcn's example), a card with a plain text body, and a card with a custom width. Tab onto a link to open its card from the keyboard; Escape closes it."
        >
            <Row>
                <HoverCard>
                    <HoverCardTrigger href="#verdelle" class="text-sm font-medium underline underline-offset-4">
                        "@verdelle"
                    </HoverCardTrigger>
                    <HoverCardContent>
                        <Profile
                            initials="BS"
                            name="Joined January 2026"
                            handle="@verdelle"
                            bio="shadcn/ui components for Leptos, on headless primitives."
                        />
                    </HoverCardContent>
                </HoverCard>
                <HoverCard>
                    <HoverCardTrigger href="#docs" class="text-sm underline underline-offset-4">
                        "Documentation"
                    </HoverCardTrigger>
                    <HoverCardContent>
                        <p class="text-sm">"Guides, API reference and examples for every component."</p>
                    </HoverCardContent>
                </HoverCard>
                <HoverCard>
                    <HoverCardTrigger class="text-sm underline decoration-dotted underline-offset-4">
                        "No href (still focusable)"
                    </HoverCardTrigger>
                    <HoverCardContent width=360>
                        <p class="text-sm">"A 360px wide card on a trigger without an href."</p>
                    </HoverCardContent>
                </HoverCard>
            </Row>
        </Section>
    }
}

#[component]
fn PlacementSection() -> impl IntoView {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    const ALIGNS: [Align; 3] = [Align::Start, Align::Center, Align::End];

    view! {
        <Section title="Side and align" description="Default is bottom, centre, 4px away.">
            <div class="space-y-3">
                {SIDES
                    .into_iter()
                    .map(|side| {
                        view! {
                            <Row label=side.as_str()>
                                {ALIGNS
                                    .into_iter()
                                    .map(|align| {
                                        view! {
                                            <HoverCard open_delay=200>
                                                <HoverCardTrigger class="inline-flex">
                                                    <span class="rounded-md border px-2 py-1 text-xs">{align.as_str()}</span>
                                                </HoverCardTrigger>
                                                <HoverCardContent side=side align=align width=200>
                                                    <p class="text-sm">{format!("{} / {}", side.as_str(), align.as_str())}</p>
                                                </HoverCardContent>
                                            </HoverCard>
                                        }
                                    })
                                    .collect_view()}
                            </Row>
                        }
                    })
                    .collect_view()}
                <Row label="offsets">
                    <HoverCard open_delay=200>
                        <HoverCardTrigger class="inline-flex">
                            <span class="rounded-md border px-2 py-1 text-xs">"side_offset 16"</span>
                        </HoverCardTrigger>
                        <HoverCardContent side_offset=16.0 width=200>
                            <p class="text-sm">"16px further out"</p>
                        </HoverCardContent>
                    </HoverCard>
                    <HoverCard open_delay=200>
                        <HoverCardTrigger class="inline-flex">
                            <span class="rounded-md border px-2 py-1 text-xs">"align_offset 24"</span>
                        </HoverCardTrigger>
                        <HoverCardContent align=Align::Start align_offset=24.0 width=200>
                            <p class="text-sm">"Shifted along the edge"</p>
                        </HoverCardContent>
                    </HoverCard>
                </Row>
            </div>
        </Section>
    }
}

#[component]
fn DelaySection() -> impl IntoView {
    let entered = RwSignal::new(0.0_f64);
    let readout = RwSignal::new(String::from("hover a link"));

    let opened = move |name: &'static str| {
        Callback::new(move |open: bool| {
            let elapsed = (now_ms() - entered.get_untracked()).round();
            if open {
                readout.set(format!(
                    "{name} opened {elapsed} ms after the pointer arrived"
                ));
            } else {
                readout.set(format!(
                    "{name} closed {elapsed} ms after the pointer arrived"
                ));
            }
        })
    };

    view! {
        <Section
            title="Delays"
            description="open_delay is how long the pointer must rest before the card opens; close_delay is how long it may be away before the card closes (crossing onto the card cancels it). Defaults are 600ms and 300ms. Each link below uses its own."
        >
            <div
                class="flex flex-wrap items-center gap-6"
                on:pointerover=move |_| entered.set(now_ms())
            >
                <HoverCard open_delay=0 close_delay=0 on_open_change=opened("Instant")>
                    <HoverCardTrigger class="text-sm underline underline-offset-4">"0 / 0 (instant)"</HoverCardTrigger>
                    <HoverCardContent width=200>
                        <p class="text-sm">"No delays at all."</p>
                    </HoverCardContent>
                </HoverCard>
                <HoverCard on_open_change=opened("Default")>
                    <HoverCardTrigger class="text-sm underline underline-offset-4">"600 / 300 (default)"</HoverCardTrigger>
                    <HoverCardContent width=200>
                        <p class="text-sm">"Shadcn's defaults."</p>
                    </HoverCardContent>
                </HoverCard>
                <HoverCard open_delay=1500 close_delay=1000 on_open_change=opened("Slow")>
                    <HoverCardTrigger class="text-sm underline underline-offset-4">"1500 / 1000 (slow)"</HoverCardTrigger>
                    <HoverCardContent width=200>
                        <p class="text-sm">"Waits a long time both ways."</p>
                    </HoverCardContent>
                </HoverCard>
            </div>
            <p class="text-sm text-muted-foreground" id="hc-delay-readout">{move || readout.get()}</p>
        </Section>
    }
}

#[component]
fn OpenStateSection() -> impl IntoView {
    let open = RwSignal::new(false);
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |line: String| {
        log.update(|l| {
            l.insert(0, line);
            l.truncate(5);
        })
    };

    view! {
        <Section
            title="Open state and callbacks"
            description="on_open_change reports every change. The first card is controlled by the button next to it; the second starts open (default_open) and closes when the pointer touches and leaves it, or on Escape."
        >
            <Row>
                <HoverCard
                    open=open
                    on_open_change=move |o| {
                        open.set(o);
                        push(format!("on_open_change({o})"));
                    }
                >
                    <HoverCardTrigger href="#controlled" class="text-sm underline underline-offset-4">"Controlled"</HoverCardTrigger>
                    <HoverCardContent side=Side::Right width=200>
                        <p class="text-sm">"Controlled card"</p>
                    </HoverCardContent>
                </HoverCard>
                <Button variant=ButtonVariant::Ghost size=Size::Small on_click=move |_| open.update(|v| *v = !*v)>
                    "Toggle it"
                </Button>
                <span class="text-sm text-muted-foreground">"open = " <span id="hc-open">{move || open.get().to_string()}</span></span>
                <HoverCard default_open=true>
                    <HoverCardTrigger href="#starts-open" class="text-sm underline underline-offset-4">"Starts open"</HoverCardTrigger>
                    <HoverCardContent side=Side::Bottom width=200>
                        <p class="text-sm">"default_open"</p>
                    </HoverCardContent>
                </HoverCard>
            </Row>
            <ul id="hc-log" class="min-h-24 text-sm text-muted-foreground">
                {move || log.get().into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}
            </ul>
        </Section>
    }
}

#[component]
fn CollisionSection() -> impl IntoView {
    let show_fixed = RwSignal::new(false);

    view! {
        <Section
            title="Collisions"
            description="At the window edge the card flips to the other side and slides to stay in view. The button pins links to the window corners."
        >
            <Button variant=ButtonVariant::Secondary selected=show_fixed on_click=move |_| show_fixed.update(|v| *v = !*v)>
                {move || if show_fixed.get() { "Hide the corner links" } else { "Show links pinned to the window corners" }}
            </Button>
            <Show when=move || show_fixed.get()>
                <div class="fixed top-4 right-4 z-10">
                    <HoverCard open_delay=200>
                        <HoverCardTrigger class="text-sm underline">"Top right, opens above"</HoverCardTrigger>
                        <HoverCardContent side=Side::Top collision_padding=8.0>
                            <p class="text-sm">"Flips below the link."</p>
                        </HoverCardContent>
                    </HoverCard>
                </div>
                <div class="fixed right-4 bottom-4 z-10">
                    <HoverCard open_delay=200>
                        <HoverCardTrigger class="text-sm underline">"Corner, opens right"</HoverCardTrigger>
                        <HoverCardContent side=Side::Right collision_padding=8.0>
                            <p class="text-sm">"Flips to the left."</p>
                        </HoverCardContent>
                    </HoverCard>
                </div>
                <div class="fixed bottom-4 left-64 z-10">
                    <HoverCard open_delay=200>
                        <HoverCardTrigger class="text-sm underline">"Bottom, opens below"</HoverCardTrigger>
                        <HoverCardContent side=Side::Bottom collision_padding=8.0>
                            <p class="text-sm">"Flips above the link."</p>
                        </HoverCardContent>
                    </HoverCard>
                </div>
            </Show>
        </Section>
    }
}
