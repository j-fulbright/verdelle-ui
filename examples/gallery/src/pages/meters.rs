use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{Meter, MeterLabel, MeterValue};

#[component]
fn Dial(value: RwSignal<f64>, label: &'static str) -> impl IntoView {
    view! {
        <input
            type="range"
            min="0"
            max="100"
            class="w-64"
            aria-label=label
            prop:value=move || value.get().to_string()
            on:input=move |ev| {
                if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                    value.set(v);
                }
            }
        />
    }
}

#[component]
pub fn MetersPage() -> impl IntoView {
    let basic = RwSignal::new(60.0);
    let disk = RwSignal::new(40.0);
    let battery = RwSignal::new(80.0);
    let temp = RwSignal::new(50.0);
    let score = RwSignal::new(7.0);

    view! {
        <Page
            title="Meter"
            summary="A gauge for a measurement in a known range: role=meter with aria-valuenow, aria-valuemin and aria-valuemax. It is not Progress (a task advancing). low, high and optimum colour the fill the way the HTML meter element does."
        >
            <Section
                title="Basic"
                description="value from 0 to 100 with a label and the reading. Drag the range input to change it."
            >
                <div class="flex w-80 max-w-full flex-col gap-3">
                    <Meter value=basic>
                        <MeterLabel>"Storage used"</MeterLabel>
                        <MeterValue text=|v| format!("{v}%") />
                    </Meter>
                    <Dial value=basic label="Storage used value" />
                </div>
            </Section>

            <Section
                title="Low is good"
                description="low=60, high=85, optimum=10 (disk usage): the fill is primary up to 60, muted up to 85 and destructive above."
            >
                <div class="flex w-80 max-w-full flex-col gap-3">
                    <Meter value=disk low=60.0 high=85.0 optimum=10.0>
                        <MeterLabel>"Disk"</MeterLabel>
                        <MeterValue text=|v| format!("{v}%") />
                    </Meter>
                    <Dial value=disk label="Disk value" />
                </div>
            </Section>

            <Section
                title="High is good"
                description="low=20, high=50, optimum=90 (battery): destructive below 20, muted up to 50, primary above."
            >
                <div class="flex w-80 max-w-full flex-col gap-3">
                    <Meter value=battery low=20.0 high=50.0 optimum=90.0>
                        <MeterLabel>"Battery"</MeterLabel>
                        <MeterValue text=|v| format!("{v}%") />
                    </Meter>
                    <Dial value=battery label="Battery value" />
                </div>
            </Section>

            <Section
                title="Middle is good"
                description="low=30, high=70, optimum=50 (temperature): primary in the middle, muted at both ends."
            >
                <div class="flex w-80 max-w-full flex-col gap-3">
                    <Meter value=temp low=30.0 high=70.0 optimum=50.0>
                        <MeterLabel>"Temperature"</MeterLabel>
                        <MeterValue text=|v| format!("{v}°") />
                    </Meter>
                    <Dial value=temp label="Temperature value" />
                </div>
            </Section>

            <Section
                title="Custom range and text"
                description="min=0 and max=10, and value_text gives the screen-reader text (aria-valuetext) as well as the reading."
            >
                <div class="flex w-80 max-w-full flex-col gap-3">
                    <Meter value=score max=10.0 value_text=|v| format!("{v} out of 10")>
                        <MeterLabel>"Score"</MeterLabel>
                        <MeterValue text=|v| format!("{v} / 10") />
                    </Meter>
                    <Row>
                        <input
                            type="range"
                            min="0"
                            max="10"
                            step="1"
                            class="w-64"
                            aria-label="Score value"
                            prop:value=move || score.get().to_string()
                            on:input=move |ev| {
                                if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                    score.set(v);
                                }
                            }
                        />
                    </Row>
                </div>
            </Section>

            <Section
                title="Without a label"
                description="No label or value, so give it aria_label."
            >
                <div class="w-80 max-w-full">
                    <Meter value=35.0 aria_label="Signal strength" />
                </div>
            </Section>
        </Page>
    }
}
