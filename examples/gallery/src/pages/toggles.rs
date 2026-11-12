use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Orientation, Size, Toggle, ToggleGroup, ToggleGroupItem, ToggleVariant,
};

fn icon(paths: &'static str) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            inner_html=paths
        ></svg>
    }
}

fn bold() -> impl IntoView {
    icon(r#"<path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/>"#)
}
fn italic() -> impl IntoView {
    icon(
        r#"<line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/>"#,
    )
}
fn underline() -> impl IntoView {
    icon(r#"<path d="M6 4v6a6 6 0 0 0 12 0V4"/><line x1="4" x2="20" y1="20" y2="20"/>"#)
}
fn align_left() -> impl IntoView {
    icon(r#"<path d="M21 5H3"/><path d="M15 12H3"/><path d="M17 19H3"/>"#)
}
fn align_center() -> impl IntoView {
    icon(r#"<path d="M21 5H3"/><path d="M17 12H7"/><path d="M19 19H5"/>"#)
}
fn align_right() -> impl IntoView {
    icon(r#"<path d="M21 5H3"/><path d="M21 12H9"/><path d="M21 19H7"/>"#)
}

#[component]
pub fn TogglesPage() -> impl IntoView {
    let pressed = RwSignal::new(false);
    let align = RwSignal::new(vec![String::from("center")]);
    let marks = RwSignal::new(vec![String::from("bold"), String::from("underline")]);
    let last_change = RwSignal::new(String::from("(none yet)"));

    let on_align = Callback::new(move |v: Vec<String>| {
        last_change.set(format!("align={v:?}"));
        align.set(v);
    });
    let on_marks = Callback::new(move |v: Vec<String>| {
        last_change.set(format!("marks={v:?}"));
        marks.set(v);
    });

    view! {
        <Page
            title="Toggle and toggle group"
            summary="Toggle and ToggleGroup, built on the headless primitives. Space or Enter presses a toggle; in a group, arrow keys, Home and End move focus (one tab stop for the whole group)."
        >
            <Section title="Toggle: variant and size" description="Default and outline, each in sm (28px), default (32px) and lg (36px). Click or press Space.">
                <div class="grid gap-3">
                    <Row label="Default">
                        <Toggle size=Size::Small aria_label="Bold small">{bold()}</Toggle>
                        <Toggle aria_label="Bold">{bold()}</Toggle>
                        <Toggle size=Size::Large aria_label="Bold large">{bold()}</Toggle>
                    </Row>
                    <Row label="Outline">
                        <Toggle variant=ToggleVariant::Outline size=Size::Small aria_label="Italic small">{italic()}</Toggle>
                        <Toggle variant=ToggleVariant::Outline aria_label="Italic">{italic()}</Toggle>
                        <Toggle variant=ToggleVariant::Outline size=Size::Large aria_label="Italic large">{italic()}</Toggle>
                    </Row>
                    <Row label="With text">
                        <Toggle id="tg-text" aria_label="Underline text">{underline()} "Underline"</Toggle>
                        <Toggle variant=ToggleVariant::Outline>{italic()} "Italic"</Toggle>
                        <Toggle>"Text only"</Toggle>
                    </Row>
                    <Row label="Pressed at start">
                        <Toggle default_pressed=true aria_label="Bold pressed">{bold()}</Toggle>
                        <Toggle default_pressed=true variant=ToggleVariant::Outline aria_label="Italic pressed">{italic()}</Toggle>
                    </Row>
                    <Row label="Disabled">
                        <Toggle disabled=true aria_label="Disabled off">{bold()}</Toggle>
                        <Toggle disabled=true default_pressed=true variant=ToggleVariant::Outline aria_label="Disabled on">{italic()}</Toggle>
                    </Row>
                </div>
            </Section>

            <Section title="Controlled toggle" description="A signal owns the state; the button flips it from outside.">
                <Row>
                    <Toggle
                        id="tg-ctl"
                        variant=ToggleVariant::Outline
                        pressed=pressed
                        on_pressed_change=move |v| pressed.set(v)
                    >
                        {bold()} "Bold"
                    </Toggle>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        on_click=move |_| pressed.update(|v| *v = !*v)
                    >
                        "Flip from outside"
                    </Button>
                    <span id="tg-readout" class="text-sm text-muted-foreground">
                        "pressed: " {move || pressed.get().to_string()}
                    </span>
                </Row>
            </Section>

            <Section title="Toggle group: single" description="One item pressed at a time; pressing it again clears it. Starts on Center.">
                <div class="grid gap-3">
                    <ToggleGroup aria_label="Text alignment" default_value=vec![String::from("center")]>
                        <ToggleGroupItem value="left" aria_label="Align left">{align_left()}</ToggleGroupItem>
                        <ToggleGroupItem value="center" aria_label="Align center">{align_center()}</ToggleGroupItem>
                        <ToggleGroupItem value="right" aria_label="Align right">{align_right()}</ToggleGroupItem>
                    </ToggleGroup>
                    <ToggleGroup aria_label="Nothing pressed at first" variant=ToggleVariant::Outline>
                        <ToggleGroupItem value="a">"Day"</ToggleGroupItem>
                        <ToggleGroupItem value="b">"Week"</ToggleGroupItem>
                        <ToggleGroupItem value="c">"Month"</ToggleGroupItem>
                    </ToggleGroup>
                </div>
            </Section>

            <Section title="Toggle group: multiple" description="multiple=true: any number pressed. Starts with Bold and Underline.">
                <ToggleGroup aria_label="Text style" multiple=true default_value=vec![String::from("bold"), String::from("underline")]>
                    <ToggleGroupItem value="bold" aria_label="Bold">{bold()}</ToggleGroupItem>
                    <ToggleGroupItem value="italic" aria_label="Italic">{italic()}</ToggleGroupItem>
                    <ToggleGroupItem value="underline" aria_label="Underline">{underline()}</ToggleGroupItem>
                </ToggleGroup>
            </Section>

            <Section title="Toggle group: variant, size and spacing" description="Spacing is in 4px steps (default 2). At 0 the outline items join into one bar and share borders.">
                <div class="grid gap-3">
                    <Row label="Outline, spacing 2">
                        <ToggleGroup aria_label="Outline spaced" variant=ToggleVariant::Outline multiple=true>
                            <ToggleGroupItem value="bold" aria_label="Bold">{bold()}</ToggleGroupItem>
                            <ToggleGroupItem value="italic" aria_label="Italic">{italic()}</ToggleGroupItem>
                            <ToggleGroupItem value="underline" aria_label="Underline">{underline()}</ToggleGroupItem>
                        </ToggleGroup>
                    </Row>
                    <Row label="Outline, spacing 0">
                        <ToggleGroup aria_label="Outline joined" variant=ToggleVariant::Outline spacing=0 multiple=true default_value=vec![String::from("italic")]>
                            <ToggleGroupItem value="bold" aria_label="Bold">{bold()}</ToggleGroupItem>
                            <ToggleGroupItem value="italic" aria_label="Italic">{italic()}</ToggleGroupItem>
                            <ToggleGroupItem value="underline" aria_label="Underline">{underline()}</ToggleGroupItem>
                        </ToggleGroup>
                    </Row>
                    <Row label="Default, spacing 0">
                        <ToggleGroup aria_label="Default joined" spacing=0 default_value=vec![String::from("left")]>
                            <ToggleGroupItem value="left" aria_label="Align left">{align_left()}</ToggleGroupItem>
                            <ToggleGroupItem value="center" aria_label="Align center">{align_center()}</ToggleGroupItem>
                            <ToggleGroupItem value="right" aria_label="Align right">{align_right()}</ToggleGroupItem>
                        </ToggleGroup>
                    </Row>
                    <Row label="Small">
                        <ToggleGroup aria_label="Small group" size=Size::Small variant=ToggleVariant::Outline spacing=0>
                            <ToggleGroupItem value="a">"One"</ToggleGroupItem>
                            <ToggleGroupItem value="b">"Two"</ToggleGroupItem>
                            <ToggleGroupItem value="c">"Three"</ToggleGroupItem>
                        </ToggleGroup>
                    </Row>
                    <Row label="Large">
                        <ToggleGroup aria_label="Large group" size=Size::Large variant=ToggleVariant::Outline spacing=0>
                            <ToggleGroupItem value="a">"One"</ToggleGroupItem>
                            <ToggleGroupItem value="b">"Two"</ToggleGroupItem>
                            <ToggleGroupItem value="c">"Three"</ToggleGroupItem>
                        </ToggleGroup>
                    </Row>
                </div>
            </Section>

            <Section title="Toggle group: vertical" description="orientation=vertical: Up/Down move focus. Spacing 0 stacks the items into one bar.">
                <div class="flex flex-wrap items-start gap-8">
                    <ToggleGroup aria_label="Vertical spaced" orientation=Orientation::Vertical variant=ToggleVariant::Outline>
                        <ToggleGroupItem value="bold">{bold()} "Bold"</ToggleGroupItem>
                        <ToggleGroupItem value="italic">{italic()} "Italic"</ToggleGroupItem>
                        <ToggleGroupItem value="underline">{underline()} "Underline"</ToggleGroupItem>
                    </ToggleGroup>
                    <ToggleGroup aria_label="Vertical joined" orientation=Orientation::Vertical variant=ToggleVariant::Outline spacing=0 multiple=true>
                        <ToggleGroupItem value="bold">{bold()} "Bold"</ToggleGroupItem>
                        <ToggleGroupItem value="italic">{italic()} "Italic"</ToggleGroupItem>
                        <ToggleGroupItem value="underline">{underline()} "Underline"</ToggleGroupItem>
                    </ToggleGroup>
                </div>
            </Section>

            <Section title="Disabled" description="A disabled group disables every item; a disabled item is skipped by the arrow keys.">
                <div class="grid gap-3">
                    <ToggleGroup aria_label="Disabled group" disabled=true variant=ToggleVariant::Outline default_value=vec![String::from("a")]>
                        <ToggleGroupItem value="a">"A"</ToggleGroupItem>
                        <ToggleGroupItem value="b">"B"</ToggleGroupItem>
                    </ToggleGroup>
                    <ToggleGroup aria_label="One disabled item" variant=ToggleVariant::Outline>
                        <ToggleGroupItem value="a">"A"</ToggleGroupItem>
                        <ToggleGroupItem value="b" disabled=true>"B (disabled)"</ToggleGroupItem>
                        <ToggleGroupItem value="c">"C"</ToggleGroupItem>
                    </ToggleGroup>
                </div>
            </Section>

            <Section title="Controlled groups" description="Signals own the values; the readouts show them.">
                <div class="grid gap-3">
                    <ToggleGroup
                        aria_label="Alignment (controlled)"
                        variant=ToggleVariant::Outline
                        value=align
                        on_value_change=on_align
                    >
                        <ToggleGroupItem value="left" aria_label="Left">{align_left()}</ToggleGroupItem>
                        <ToggleGroupItem value="center" aria_label="Center">{align_center()}</ToggleGroupItem>
                        <ToggleGroupItem value="right" aria_label="Right">{align_right()}</ToggleGroupItem>
                    </ToggleGroup>
                    <ToggleGroup
                        aria_label="Styles (controlled)"
                        variant=ToggleVariant::Outline
                        multiple=true
                        value=marks
                        on_value_change=on_marks
                    >
                        <ToggleGroupItem value="bold" aria_label="Bold">{bold()}</ToggleGroupItem>
                        <ToggleGroupItem value="italic" aria_label="Italic">{italic()}</ToggleGroupItem>
                        <ToggleGroupItem value="underline" aria_label="Underline">{underline()}</ToggleGroupItem>
                    </ToggleGroup>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| {
                                align.set(vec![String::from("right")]);
                                marks.set(vec![]);
                            }
                        >
                            "Set right, clear styles"
                        </Button>
                    </Row>
                    <span id="tgroup-readout" class="text-sm text-muted-foreground">
                        "align: " {move || format!("{:?}", align.get())} ", marks: "
                        {move || format!("{:?}", marks.get())} ", last change: "
                        {move || last_change.get()}
                    </span>
                </div>
            </Section>
            <DeselectableExamples />
        </Page>
    }
}

/// Single mode with and without `deselectable`.
#[component]
fn DeselectableExamples() -> impl IntoView {
    let free = RwSignal::new(vec![String::from("left")]);
    let locked = RwSignal::new(vec![String::from("left")]);
    view! {
        <Section
            title="deselectable (single mode)"
            description="Pressing the pressed item clears a single-choice group by default, as Radix, Base UI and shadcn do. deselectable=false keeps one item always chosen, for a segmented control."
        >
            <div class="grid gap-3">
                <Row label="Default">
                    <ToggleGroup
                        attr:id="tg-free"
                        variant=ToggleVariant::Outline
                        aria_label="Alignment, can clear"
                        value=free
                        on_value_change={move |v: Vec<String>| free.set(v)}
                    >
                        <ToggleGroupItem value="left">"Left"</ToggleGroupItem>
                        <ToggleGroupItem value="center">"Center"</ToggleGroupItem>
                        <ToggleGroupItem value="right">"Right"</ToggleGroupItem>
                    </ToggleGroup>
                    <span id="tg-free-readout" class="text-sm text-muted-foreground">
                        {move || format!("{:?}", free.get())}
                    </span>
                </Row>
                <Row label="deselectable=false">
                    <ToggleGroup
                        attr:id="tg-locked"
                        variant=ToggleVariant::Outline
                        deselectable=false
                        aria_label="Alignment, always one"
                        value=locked
                        on_value_change={move |v: Vec<String>| locked.set(v)}
                    >
                        <ToggleGroupItem value="left">"Left"</ToggleGroupItem>
                        <ToggleGroupItem value="center">"Center"</ToggleGroupItem>
                        <ToggleGroupItem value="right">"Right"</ToggleGroupItem>
                    </ToggleGroup>
                    <span id="tg-locked-readout" class="text-sm text-muted-foreground">
                        {move || format!("{:?}", locked.get())}
                    </span>
                </Row>
            </div>
        </Section>
    }
}
