use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Align, Button, ButtonVariant, Icon, IconName, Side, Size, Tooltip, TooltipContent,
    TooltipProvider, TooltipText, TooltipTrigger,
};

fn now_ms() -> f64 {
    leptos::web_sys::js_sys::Date::now()
}

#[component]
pub fn TooltipsPage() -> impl IntoView {
    view! {
        <Page
            title="Tooltips"
            summary="A short label for a trigger. It opens when the pointer rests on the trigger or the trigger is focused with the keyboard, and closes on pointer leave, blur, a press or Escape. Everything here sits inside a TooltipProvider."
        >
            <TooltipProvider>
                <BasicSection />
                <PlacementSection />
                <DelaySection />
                <OpenStateSection />
                <HoverableAndTextSection />
                <CollisionSection />
            </TooltipProvider>
        </Page>
    }
}

#[component]
fn BasicSection() -> impl IntoView {
    view! {
        <Section
            title="Basic"
            description="A text button, an icon button (its tooltip is its label), a tooltip with a shortcut hint, a long one that wraps at 320px, one without an arrow, and a disabled-looking trigger that still shows its tooltip."
        >
            <Row>
                <Tooltip>
                    <TooltipTrigger variant=ButtonVariant::Outline>"Hover me"</TooltipTrigger>
                    <TooltipContent>"Add to library"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger variant=ButtonVariant::Outline icon_only=true aria_label="Add item">
                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            aria-hidden="true"
                        >
                            <path d="M12 5v14M5 12h14"></path>
                        </svg>
                    </TooltipTrigger>
                    <TooltipContent>"Add item"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger variant=ButtonVariant::Outline>"Save"</TooltipTrigger>
                    <TooltipContent>"Save changes " <kbd>"⌘S"</kbd></TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger variant=ButtonVariant::Outline>"Long text"</TooltipTrigger>
                    <TooltipContent>
                        "Tooltips are for short supplementary text; anything longer than this wraps at 320 pixels wide instead of stretching across the screen."
                    </TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger variant=ButtonVariant::Outline>"No arrow"</TooltipTrigger>
                    <TooltipContent arrow=false>"Without an arrow"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger variant=ButtonVariant::Outline disabled=true>"Disabled look"</TooltipTrigger>
                    <TooltipContent>"Not available on your plan"</TooltipContent>
                </Tooltip>
                <Tooltip>
                    <TooltipTrigger class="text-sm underline decoration-dotted underline-offset-4">
                        "A bare trigger"
                    </TooltipTrigger>
                    <TooltipContent>"Styled with class only"</TooltipContent>
                </Tooltip>
            </Row>
        </Section>
    }
}

#[component]
fn PlacementSection() -> impl IntoView {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    const ALIGNS: [Align; 3] = [Align::Start, Align::Center, Align::End];

    view! {
        <Section title="Side and align" description="Default is top, centre. The offsets move the label away from the trigger or along its edge.">
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
                                            <Tooltip>
                                                <TooltipTrigger variant=ButtonVariant::Outline size=Size::Small>
                                                    {align.as_str()}
                                                </TooltipTrigger>
                                                <TooltipContent side=side align=align>
                                                    {format!("{} / {}", side.as_str(), align.as_str())}
                                                </TooltipContent>
                                            </Tooltip>
                                        }
                                    })
                                    .collect_view()}
                            </Row>
                        }
                    })
                    .collect_view()}
                <Row label="offsets">
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Outline size=Size::Small>"side_offset 16"</TooltipTrigger>
                        <TooltipContent side_offset=16.0>"16px further out"</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Outline size=Size::Small>"align_offset 20"</TooltipTrigger>
                        <TooltipContent align=Align::Start align_offset=20.0>"Shifted along the edge"</TooltipContent>
                    </Tooltip>
                </Row>
            </div>
        </Section>
    }
}

#[component]
fn DelaySection() -> impl IntoView {
    let entered = RwSignal::new(0.0_f64);
    let readout = RwSignal::new(String::from("hover a button"));

    let opened = move |name: &'static str| {
        Callback::new(move |open: bool| {
            if open {
                let waited = (now_ms() - entered.get_untracked()).round();
                readout.set(format!(
                    "{name} opened {waited} ms after the pointer arrived"
                ));
            }
        })
    };

    view! {
        <Section
            title="Delay"
            description="The provider below waits 800ms, but skips the wait for 400ms after a tooltip closes: hover the first button and wait, then glide along the row and each one opens at once. The last button overrides the delay with its own 100ms. Keyboard focus always opens instantly."
        >
            <TooltipProvider delay_duration=800 skip_delay_duration=400>
                <div
                    class="flex flex-wrap items-center gap-3"
                    on:pointerover=move |_| {
                        // Pointer-over bubbles from the buttons: note when it arrived.
                        entered.set(now_ms());
                    }
                >
                    <Tooltip on_open_change=opened("One")>
                        <TooltipTrigger variant=ButtonVariant::Outline>"One"</TooltipTrigger>
                        <TooltipContent>"First (800ms)"</TooltipContent>
                    </Tooltip>
                    <Tooltip on_open_change=opened("Two")>
                        <TooltipTrigger variant=ButtonVariant::Outline>"Two"</TooltipTrigger>
                        <TooltipContent>"Second"</TooltipContent>
                    </Tooltip>
                    <Tooltip on_open_change=opened("Three")>
                        <TooltipTrigger variant=ButtonVariant::Outline>"Three"</TooltipTrigger>
                        <TooltipContent>"Third"</TooltipContent>
                    </Tooltip>
                    <Tooltip delay_duration=100 on_open_change=opened("Fast")>
                        <TooltipTrigger variant=ButtonVariant::Secondary>"Fast (100ms)"</TooltipTrigger>
                        <TooltipContent>"Own delay of 100ms"</TooltipContent>
                    </Tooltip>
                </div>
            </TooltipProvider>
            <p class="text-sm text-muted-foreground" id="tip-delay-readout">{move || readout.get()}</p>
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
            description="on_open_change reports every change. The first tooltip is controlled by the button next to it; the second starts open (default_open) and closes when you leave it or press Escape."
        >
            <Row>
                <Tooltip
                    open=open
                    on_open_change=move |o| {
                        open.set(o);
                        push(format!("on_open_change({o})"));
                    }
                >
                    <TooltipTrigger variant=ButtonVariant::Outline>"Controlled"</TooltipTrigger>
                    <TooltipContent side=Side::Bottom>"Controlled tooltip"</TooltipContent>
                </Tooltip>
                <Button variant=ButtonVariant::Ghost on_click=move |_| open.update(|v| *v = !*v)>"Toggle it"</Button>
                <span class="text-sm text-muted-foreground">"open = " <span id="tip-open">{move || open.get().to_string()}</span></span>
                <Tooltip default_open=true>
                    <TooltipTrigger variant=ButtonVariant::Outline>"Starts open"</TooltipTrigger>
                    <TooltipContent side=Side::Bottom>"default_open"</TooltipContent>
                </Tooltip>
            </Row>
            <ul id="tip-log" class="min-h-24 text-sm text-muted-foreground">
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
            description="At the window edge the label flips to the other side and slides to stay in view. The button pins triggers to the window corners."
        >
            <Button variant=ButtonVariant::Secondary selected=show_fixed on_click=move |_| show_fixed.update(|v| *v = !*v)>
                {move || if show_fixed.get() { "Hide the corner triggers" } else { "Show triggers pinned to the window corners" }}
            </Button>
            <Show when=move || show_fixed.get()>
                <div class="fixed top-4 right-4 z-10">
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Default>"Top right, opens top"</TooltipTrigger>
                        <TooltipContent side=Side::Top collision_padding=8.0>"Flips below the trigger"</TooltipContent>
                    </Tooltip>
                </div>
                <div class="fixed right-4 bottom-4 z-10">
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Default>"Corner, opens right"</TooltipTrigger>
                        <TooltipContent side=Side::Right collision_padding=8.0>"Flips to the left"</TooltipContent>
                    </Tooltip>
                </div>
                <div class="fixed bottom-4 left-[32rem] z-10">
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Default>"Bottom, opens below"</TooltipTrigger>
                        <TooltipContent side=Side::Bottom collision_padding=8.0>"Flips above the trigger"</TooltipContent>
                    </Tooltip>
                </div>
                <div class="fixed bottom-4 left-64 z-10">
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Default>"Long label near the left"</TooltipTrigger>
                        <TooltipContent align=Align::Start collision_padding=8.0 side=Side::Left>
                            "This label is wide enough that it must slide as well as flip when it opens to the left."
                        </TooltipContent>
                    </Tooltip>
                </div>
            </Show>
        </Section>
    }
}

/// Hoverable content and the `TooltipText` shorthand.
#[component]
fn HoverableAndTextSection() -> impl IntoView {
    let copied = RwSignal::new(0u32);
    view! {
        <Section
            title="Hoverable content"
            description="By default the tooltip stays open while the pointer is over it (Radix's default), so its text can be selected or a link in it clicked; leaving the trigger waits 120ms so the pointer can cross the gap. hoverable=false closes it the moment the pointer leaves the trigger."
        >
            <Row>
                <Tooltip delay_duration=0>
                    <TooltipTrigger attr:id="hover-yes" variant=ButtonVariant::Outline>
                        "Hoverable (default)"
                    </TooltipTrigger>
                    <TooltipContent side=Side::Bottom side_offset=6.0>
                        <span id="hover-yes-text">"Move onto me, then "</span>
                        <button
                            id="hover-yes-btn"
                            type="button"
                            class="underline"
                            on:click=move |_| copied.update(|n| *n += 1)
                        >
                            "click this"
                        </button>
                    </TooltipContent>
                </Tooltip>
                <Tooltip delay_duration=0 hoverable=false>
                    <TooltipTrigger attr:id="hover-no" variant=ButtonVariant::Outline>
                        "hoverable=false"
                    </TooltipTrigger>
                    <TooltipContent side=Side::Bottom side_offset=6.0>
                        <span id="hover-no-text">"Gone as soon as the pointer leaves the button"</span>
                    </TooltipContent>
                </Tooltip>
                <span id="hover-readout" class="text-sm text-muted-foreground">
                    "clicks in tooltip: " {move || copied.get()}
                </span>
            </Row>
        </Section>

        <Section
            title="TooltipText: a hint on any element"
            description="One line instead of Tooltip + TooltipTrigger + TooltipContent. The single child element becomes the trigger (no wrapper, no extra button): a Button, an icon, a link or a span. It takes the same side, align and delay options."
        >
            <Row>
                <TooltipText text="Save the file" delay_duration=0>
                    <Button attr:id="tt-button" variant=ButtonVariant::Outline>"Save"</Button>
                </TooltipText>
                <TooltipText text="Delete (icon only; the button still needs its own aria-label)" side=Side::Bottom delay_duration=0>
                    <Button
                        attr:id="tt-icon"
                        variant=ButtonVariant::Ghost
                        icon_only=true
                        aria_label="Delete"
                    >
                        <Icon name=IconName::Trash2 />
                    </Button>
                </TooltipText>
                <TooltipText text="On the right, no arrow" side=Side::Right arrow=false delay_duration=0>
                    <Button attr:id="tt-right" variant=ButtonVariant::Secondary>"Right"</Button>
                </TooltipText>
                <TooltipText text="A plain span, aligned to the start" align=Align::Start delay_duration=0>
                    <span
                        id="tt-span"
                        tabindex="0"
                        class="cursor-help rounded border border-dashed px-2 py-1 text-sm"
                    >
                        "hover or focus me"
                    </span>
                </TooltipText>
                <TooltipText text=Signal::derive(move || format!("Copied {} times", copied.get())) delay_duration=0>
                    <Button attr:id="tt-signal" variant=ButtonVariant::Outline>"Reactive text"</Button>
                </TooltipText>
            </Row>
        </Section>
    }
}
