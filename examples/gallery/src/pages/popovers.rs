use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Align, Button, ButtonVariant, Input, Label, Popover, PopoverClose, PopoverContent,
    PopoverDescription, PopoverHeader, PopoverTitle, PopoverTrigger, Side,
};

#[component]
pub fn PopoversPage() -> impl IntoView {
    view! {
        <Page
            title="Popovers"
            summary="A non-modal panel next to its trigger. It positions itself with floating-ui (flips and slides at the viewport edge), closes on Escape or a press outside, moves focus in on open and back to the trigger on close."
        >
            <FormPopover />
            <PlacementSection />
            <ControlledSection />
            <DismissalSection />
            <CollisionSection />
        </Page>
    }
}

#[component]
fn FormPopover() -> impl IntoView {
    let width = RwSignal::new(String::from("100%"));
    let max_width = RwSignal::new(String::from("300px"));
    let height = RwSignal::new(String::from("25px"));
    let saved = RwSignal::new(String::from("nothing saved"));
    let open = RwSignal::new(false);

    view! {
        <Section
            title="With a form inside"
            description="shadcn's dimensions example. Focus moves to the first input when it opens; Tab moves between the fields; Save closes it (controlled open) and Escape or a click outside also closes it."
        >
            <Row>
                <Popover open=open on_open_change=move |o| open.set(o)>
                    <PopoverTrigger variant=ButtonVariant::Outline>"Open popover"</PopoverTrigger>
                    <PopoverContent label="Dimensions" width=320u32>
                        <PopoverHeader>
                            <PopoverTitle>"Dimensions"</PopoverTitle>
                            <PopoverDescription>"Set the dimensions for the layer."</PopoverDescription>
                        </PopoverHeader>
                        <div class="grid gap-2">
                            <div class="grid grid-cols-3 items-center gap-4">
                                <Label html_for="pop-width">"Width"</Label>
                                <div class="col-span-2">
                                    <Input id="pop-width" value=width />
                                </div>
                            </div>
                            <div class="grid grid-cols-3 items-center gap-4">
                                <Label html_for="pop-max-width">"Max. width"</Label>
                                <div class="col-span-2">
                                    <Input id="pop-max-width" value=max_width />
                                </div>
                            </div>
                            <div class="grid grid-cols-3 items-center gap-4">
                                <Label html_for="pop-height">"Height"</Label>
                                <div class="col-span-2">
                                    <Input id="pop-height" value=height />
                                </div>
                            </div>
                        </div>
                        <Button on_click=move |_| {
                            saved.set(format!("w {} / max {} / h {}", width.get(), max_width.get(), height.get()));
                            open.set(false);
                        }>"Save"</Button>
                    </PopoverContent>
                </Popover>
                <span class="text-sm text-muted-foreground">"Saved: " {move || saved.get()}</span>
            </Row>
        </Section>
    }
}

#[component]
fn PlacementSection() -> impl IntoView {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    const ALIGNS: [Align; 3] = [Align::Start, Align::Center, Align::End];

    view! {
        <Section
            title="Side and align"
            description="Each trigger opens on its side with its alignment. The popover is 160px wide so the alignment is visible. The label inside shows the side it ended up on."
        >
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
                                            <Popover>
                                                <PopoverTrigger variant=ButtonVariant::Outline size=verdelle_ui::Size::Small>
                                                    {align.as_str()}
                                                </PopoverTrigger>
                                                <PopoverContent side=side align=align width=160u32 class="text-center">
                                                    {format!("{} / {}", side.as_str(), align.as_str())}
                                                </PopoverContent>
                                            </Popover>
                                        }
                                    })
                                    .collect_view()}
                            </Row>
                        }
                    })
                    .collect_view()}
                <Row label="offsets">
                    <Popover>
                        <PopoverTrigger variant=ButtonVariant::Outline size=verdelle_ui::Size::Small>
                            "side_offset 24"
                        </PopoverTrigger>
                        <PopoverContent side_offset=24.0 width=160u32>"24px from the trigger"</PopoverContent>
                    </Popover>
                    <Popover>
                        <PopoverTrigger variant=ButtonVariant::Outline size=verdelle_ui::Size::Small>
                            "align_offset 24"
                        </PopoverTrigger>
                        <PopoverContent align=Align::Start align_offset=24.0 width=160u32>
                            "Shifted 24px along the edge"
                        </PopoverContent>
                    </Popover>
                    <Popover>
                        <PopoverTrigger variant=ButtonVariant::Outline size=verdelle_ui::Size::Small>
                            "side_offset 0"
                        </PopoverTrigger>
                        <PopoverContent side_offset=0.0 width=160u32>"Touching the trigger"</PopoverContent>
                    </Popover>
                </Row>
            </div>
        </Section>
    }
}

#[component]
fn ControlledSection() -> impl IntoView {
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
            description="on_open_change reports every change. The buttons outside drive the same popover through the controlled open signal; default_open shows a popover that starts open."
        >
            <Row>
                <Popover open=open on_open_change=move |o| {
                    open.set(o);
                    push(format!("on_open_change({o})"));
                }>
                    <PopoverTrigger variant=ButtonVariant::Outline>"Controlled"</PopoverTrigger>
                    <PopoverContent width=220u32 label="Controlled popover">
                        <PopoverHeader>
                            <PopoverTitle>"Controlled"</PopoverTitle>
                            <PopoverDescription>"Open: " {move || open.get().to_string()}</PopoverDescription>
                        </PopoverHeader>
                        <Button variant=ButtonVariant::Secondary size=verdelle_ui::Size::Small on_click=move |_| open.set(false)>
                            "Close from inside"
                        </Button>
                    </PopoverContent>
                </Popover>
                <Button variant=ButtonVariant::Ghost on_click=move |_| open.set(true)>"Open it from outside"</Button>
                <Button variant=ButtonVariant::Ghost on_click=move |_| open.set(false)>"Close it from outside"</Button>
            </Row>
            <p class="text-sm text-muted-foreground">
                "open = " <span id="pop-open-readout">{move || open.get().to_string()}</span>
            </p>
            <ul id="pop-log" class="text-sm text-muted-foreground">
                {move || log.get().into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}
            </ul>
        </Section>
    }
}

#[component]
fn DismissalSection() -> impl IntoView {
    let outside = RwSignal::new(0u32);
    let escape = RwSignal::new(0u32);

    view! {
        <Section
            title="Dismissal hooks"
            description="Escape and outside presses are counted. The second popover calls prevent_default on both, so it only closes from its own button."
        >
            <Row>
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Outline>"Count dismissals"</PopoverTrigger>
                    <PopoverContent
                        width=220u32
                        on_escape_key_down=move |_| escape.update(|n| *n += 1)
                        on_pointer_down_outside=move |_| outside.update(|n| *n += 1)
                    >
                        "Press Escape or click outside."
                    </PopoverContent>
                </Popover>
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Outline>"Sticky popover"</PopoverTrigger>
                    <PopoverContent
                        width=240u32
                        on_escape_key_down=move |ev: leptos::web_sys::KeyboardEvent| ev.prevent_default()
                        on_pointer_down_outside=move |ev: leptos::web_sys::PointerEvent| ev.prevent_default()
                    >
                        <p>"Escape and outside clicks are ignored here."</p>
                        <PopoverClose variant=ButtonVariant::Secondary size=verdelle_ui::Size::Small>"Close"</PopoverClose>
                    </PopoverContent>
                </Popover>
                <span class="text-sm text-muted-foreground">
                    "Escape: " <span id="pop-escape-count">{move || escape.get()}</span> ", outside: "
                    <span id="pop-outside-count">{move || outside.get()}</span>
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn CollisionSection() -> impl IntoView {
    let show_fixed = RwSignal::new(false);

    view! {
        <Section
            title="Collisions"
            description="Near a viewport edge the popover flips to the other side and slides to stay in view (collision_padding keeps 8px clear here). The button below pins triggers to the bottom corners of the window; the last one has avoid_collisions off, so it runs off screen. On a phone a side=left or right popover wider than half the window cannot fit on either side (these demos spill over); use top or bottom there."
        >
            <div class="flex justify-between">
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Outline>"Left edge, opens left"</PopoverTrigger>
                    <PopoverContent side=Side::Left collision_padding=8.0 width=360u32>
                        "Flips to the right because there is no room on the left."
                    </PopoverContent>
                </Popover>
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Outline>"Right edge, opens right"</PopoverTrigger>
                    <PopoverContent side=Side::Right collision_padding=8.0 width=360u32>
                        "Flips to the left because there is no room on the right."
                    </PopoverContent>
                </Popover>
            </div>
            <Button variant=ButtonVariant::Secondary selected=show_fixed on_click=move |_| show_fixed.update(|v| *v = !*v)>
                {move || if show_fixed.get() { "Hide the corner triggers" } else { "Show triggers pinned to the window corners" }}
            </Button>
            <Show when=move || show_fixed.get()>
            <div class="fixed bottom-4 left-[32rem] z-10">
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Default>"Bottom, opens below"</PopoverTrigger>
                    <PopoverContent collision_padding=8.0 width=240u32>
                        "Flips above the trigger when the viewport bottom is too close."
                    </PopoverContent>
                </Popover>
            </div>
            <div class="fixed right-4 bottom-4 z-10 flex gap-2">
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Default>"Corner, align start"</PopoverTrigger>
                    <PopoverContent align=Align::Start collision_padding=8.0 width=360u32>
                        "Aligned to the trigger's start edge, but slid left so it stays inside the window."
                    </PopoverContent>
                </Popover>
                <Popover>
                    <PopoverTrigger variant=ButtonVariant::Secondary>"No collision handling"</PopoverTrigger>
                    <PopoverContent avoid_collisions=false width=300u32>
                        "avoid_collisions=false: this stays below the trigger and can run off screen."
                    </PopoverContent>
                </Popover>
            </div>
            </Show>
        </Section>
    }
}
