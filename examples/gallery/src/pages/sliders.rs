use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{Button, ButtonVariant, Orientation, Size, Slider};

fn label(text: &str) -> Vec<String> {
    vec![text.to_string()]
}

fn labels(texts: &[&str]) -> Vec<String> {
    texts.iter().map(|t| t.to_string()).collect()
}

#[component]
pub fn SlidersPage() -> impl IntoView {
    let volume = RwSignal::new(vec![40.0]);
    let range = RwSignal::new(vec![20.0, 70.0]);
    let committed = RwSignal::new(String::from("(none yet)"));
    let changes = RwSignal::new(0_u32);
    let on_volume = Callback::new(move |v: Vec<f64>| {
        changes.update(|c| *c += 1);
        volume.set(v);
    });
    let on_range = Callback::new(move |v: Vec<f64>| range.set(v));
    let on_commit = Callback::new(move |v: Vec<f64>| committed.set(format!("{v:?}")));
    let fmt = |v: Vec<f64>| {
        v.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" / ")
    };

    view! {
        <Page
            title="Slider"
            summary="Slider, built on the headless primitives. Click or drag the track; with a thumb focused: arrows step, PageUp/PageDown move ten steps, Home/End go to the ends."
        >
            <Section title="Single value" description="Uncontrolled; at the ends the thumb stays inside the track.">
                <div class="grid max-w-md gap-6">
                    <Slider default_value=vec![50.0] thumb_labels=label("Volume") id="sl-basic" />
                    <Slider default_value=vec![0.0] thumb_labels=label("Start") id="sl-zero" />
                    <Slider default_value=vec![100.0] thumb_labels=label("End") id="sl-full" />
                </div>
            </Section>

            <Section title="Step and limits" description="step 10 snaps to tens; the second runs 0 to 1 in steps of 0.1; the third runs from -50 to 50 in steps of 5.">
                <div class="grid max-w-md gap-6">
                    <Slider default_value=vec![30.0] step=10.0 thumb_labels=label("Stepped") id="sl-step" />
                    <Slider
                        default_value=vec![0.5]
                        min=0.0
                        max=1.0
                        step=0.1
                        thumb_labels=label("Opacity")
                        id="sl-frac"
                    />
                    <Slider
                        default_value=vec![-20.0]
                        min=-50.0
                        max=50.0
                        step=5.0
                        thumb_labels=label("Offset")
                        id="sl-neg"
                    />
                </div>
            </Section>

            <Section title="Multiple thumbs" description="Two thumbs make a range; three make a split. Thumbs cannot pass each other.">
                <div class="grid max-w-md gap-6">
                    <Slider
                        default_value=vec![25.0, 75.0]
                        thumb_labels=labels(&["Minimum", "Maximum"])
                        id="sl-range"
                    />
                    <Slider
                        default_value=vec![20.0, 50.0, 80.0]
                        thumb_labels=labels(&["Low", "Middle", "High"])
                        id="sl-three"
                    />
                </div>
            </Section>

            <Section title="Vertical" description="orientation=vertical: the slider fills its parent's height (here 192px). Up raises the value.">
                <div class="flex h-48 items-stretch gap-10">
                    <Slider orientation=Orientation::Vertical default_value=vec![60.0] thumb_labels=label("Vertical") id="sl-vert" />
                    <Slider
                        orientation=Orientation::Vertical
                        default_value=vec![20.0, 80.0]
                        thumb_labels=labels(&["Vertical minimum", "Vertical maximum"])
                        id="sl-vert-range"
                    />
                </div>
            </Section>

            <Section title="Inverted and disabled" description="inverted fills from the right and flips the arrow keys. Disabled ignores the pointer and keys and cannot be focused.">
                <div class="grid max-w-md gap-6">
                    <Slider inverted=true default_value=vec![30.0] thumb_labels=label("Inverted") id="sl-inv" />
                    <Slider disabled=true default_value=vec![40.0] thumb_labels=label("Disabled") id="sl-dis" />
                    <Slider
                        disabled=true
                        default_value=vec![20.0, 60.0]
                        thumb_labels=labels(&["Disabled minimum", "Disabled maximum"])
                        id="sl-dis-range"
                    />
                </div>
            </Section>

            <Section title="Controlled" description="Signals own the values. on_value_change fires on every move, on_value_commit when a drag or key press ends.">
                <div class="grid max-w-md gap-4">
                    <Slider
                        value=volume
                        on_value_change=on_volume
                        on_value_commit=on_commit
                        thumb_labels=label("Volume (controlled)")
                        id="sl-ctl"
                    />
                    <Slider
                        value=range
                        on_value_change=on_range
                        thumb_labels=labels(&["Range minimum", "Range maximum"])
                        id="sl-ctl-range"
                    />
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| {
                                volume.set(vec![90.0]);
                                range.set(vec![10.0, 30.0]);
                            }
                        >
                            "Set 90 and 10-30"
                        </Button>
                    </Row>
                    <span id="sl-readout" class="text-sm text-muted-foreground">
                        "volume: " {move || fmt(volume.get())} ", range: " {move || fmt(range.get())}
                        ", change events (volume): " {move || changes.get()} ", last commit: "
                        {move || committed.get()}
                    </span>
                </div>
            </Section>
        </Page>
    }
}
