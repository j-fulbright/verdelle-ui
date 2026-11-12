use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, HourCycle, Label, Size, TimeInput, ToggleGroup, ToggleGroupItem,
    ToggleVariant,
};

#[component]
pub fn TimeInputsPage() -> impl IntoView {
    let controlled = RwSignal::new(Some(String::from("09:30")));
    let reported = RwSignal::new(String::from("(nothing yet)"));
    let cycle_choice = RwSignal::new(vec![String::from("24")]);
    let cycle = Signal::derive(move || {
        if cycle_choice.get().first().map(String::as_str) == Some("12") {
            HourCycle::H12
        } else {
            HourCycle::H24
        }
    });
    let submitted = RwSignal::new(String::from("(nothing yet)"));
    let range_value = RwSignal::new(Some(String::from("08:15")));
    let overnight_value = RwSignal::new(Some(String::from("12:00")));

    view! {
        <Page
            title="Time input"
            summary="A time of day in segments (hour, minute and, on a 12-hour clock, AM/PM) in the Input look. The value is HH:MM on a 24-hour clock whichever clock is shown. Type digits, Up and Down step, Left and Right move between segments, Backspace clears, a and p set AM and PM."
        >
            <Section
                title="24-hour (default) and 12-hour"
                description="Click a segment and type: a first digit that cannot start a longer number moves on by itself (a 3 in the hour is 03; a 1 waits for a second digit)."
            >
                <div class="grid max-w-md gap-3">
                    <Row label="24-hour">
                        <TimeInput attr:id="time-24" aria_label="Start time (24-hour)" />
                    </Row>
                    <Row label="12-hour">
                        <TimeInput
                            attr:id="time-12"
                            aria_label="Start time (12-hour)"
                            hour_cycle=HourCycle::H12
                        />
                    </Row>
                    <Row label="Default value">
                        <TimeInput aria_label="Default 14:45" default_value="14:45" />
                        <TimeInput
                            aria_label="Default 14:45, 12-hour"
                            default_value="14:45"
                            hour_cycle=HourCycle::H12
                        />
                    </Row>
                </div>
            </Section>

            <Section
                title="Controlled, on_value_change and switching the clock"
                description="Pass a signal and it follows when the signal changes, including to empty. on_value_change gets Some(HH:MM) once every segment is filled and None when one is cleared. The group below switches hour_cycle on the same value."
            >
                <div class="grid gap-3">
                    <Row>
                        <TimeInput
                            attr:id="time-controlled"
                            aria_label="Controlled time"
                            hour_cycle=cycle
                            value=controlled
                            on_value_change={move |v: Option<String>| {
                                reported.set(format!("{v:?}"));
                                controlled.set(v);
                            }}
                        />
                        <ToggleGroup
                            variant=ToggleVariant::Outline
                            size=Size::Small
                            deselectable=false
                            aria_label="Clock"
                            value=cycle_choice
                            on_value_change={move |v: Vec<String>| cycle_choice.set(v)}
                        >
                            <ToggleGroupItem attr:id="cycle-24" value="24">"24h"</ToggleGroupItem>
                            <ToggleGroupItem attr:id="cycle-12" value="12">"12h"</ToggleGroupItem>
                        </ToggleGroup>
                    </Row>
                    <Row>
                        <Button
                            attr:id="time-set"
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(Some("17:05".into()))
                        >
                            "Set 17:05"
                        </Button>
                        <Button
                            attr:id="time-clear"
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(None)
                        >
                            "Clear"
                        </Button>
                        <span id="time-readout" class="text-sm text-muted-foreground">
                            "value " {move || format!("{:?}", controlled.get())} " | last change "
                            {move || reported.get()}
                        </span>
                    </Row>
                </div>
            </Section>

            <Section
                title="min and max"
                description="Typing is not blocked (the native input does not block it either); a time outside the range marks the field invalid. A min later than max is a range across midnight."
            >
                <div class="grid max-w-md gap-3">
                    <Row label="09:00 to 17:00">
                        <TimeInput
                            attr:id="time-range"
                            aria_label="Within business hours"
                            min="09:00"
                            max="17:00"
                            value=range_value
                            on_value_change={move |v: Option<String>| range_value.set(v)}
                        />
                        <span class="text-xs text-muted-foreground">"08:15 starts invalid"</span>
                    </Row>
                    <Row label="22:00 to 06:00">
                        <TimeInput
                            attr:id="time-overnight"
                            aria_label="Overnight window"
                            min="22:00"
                            max="06:00"
                            value=overnight_value
                            on_value_change={move |v: Option<String>| overnight_value.set(v)}
                        />
                        <span class="text-xs text-muted-foreground">"12:00 starts invalid"</span>
                    </Row>
                </div>
            </Section>

            <Section
                title="minute_step"
                description="Up and Down move the minute to the next multiple of the step (here 15); typing still takes any minute."
            >
                <Row>
                    <TimeInput attr:id="time-step" aria_label="Quarter hours" minute_step=15 default_value="10:00" />
                </Row>
            </Section>

            <Section title="Sizes" description="Heights follow Button and Input: 24, 28, 32 and 36px.">
                <div class="grid max-w-md gap-3">
                    <Row label="xs">
                        <TimeInput aria_label="xs" size=Size::XSmall default_value="08:00" />
                    </Row>
                    <Row label="sm">
                        <TimeInput aria_label="sm" size=Size::Small default_value="08:00" />
                    </Row>
                    <Row label="md">
                        <TimeInput aria_label="md" default_value="08:00" />
                    </Row>
                    <Row label="lg">
                        <TimeInput aria_label="lg" size=Size::Large default_value="08:00" hour_cycle=HourCycle::H12 />
                    </Row>
                </div>
            </Section>

            <Section title="States" description="Invalid, disabled and read-only.">
                <div class="grid max-w-md gap-3">
                    <Row label="Invalid">
                        <TimeInput attr:id="time-invalid" aria_label="Invalid" invalid=true default_value="03:00" />
                    </Row>
                    <Row label="Disabled">
                        <TimeInput attr:id="time-disabled" aria_label="Disabled" disabled=true default_value="03:00" />
                    </Row>
                    <Row label="Read-only">
                        <TimeInput attr:id="time-readonly" aria_label="Read-only" readonly=true default_value="03:00" />
                    </Row>
                </div>
            </Section>

            <Section
                title="In a form"
                description="name submits HH:MM (24-hour, whatever clock is shown); required makes the browser refuse a form with no time."
            >
                <form
                    id="time-form"
                    class="flex flex-wrap items-center gap-3"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        let form = leptos::wasm_bindgen::JsCast::unchecked_into::<leptos::web_sys::HtmlFormElement>(
                            leptos::prelude::event_target::<leptos::web_sys::Element>(&ev),
                        );
                        let data = leptos::web_sys::FormData::new_with_form(&form).ok();
                        let value = data.and_then(|d| d.get("meeting").as_string()).unwrap_or_default();
                        submitted.set(value);
                    }
                >
                    <Label html_for="time-form-field">"Meeting at"</Label>
                    <TimeInput
                        id="time-form-field"
                        name="meeting"
                        required=true
                        hour_cycle=HourCycle::H12
                    />
                    <button type="submit" id="time-submit" class="vdl-btn" data-variant="default" data-size="md">
                        "Submit"
                    </button>
                    <span id="time-submitted" class="text-sm text-muted-foreground">
                        "submitted: " {move || submitted.get()}
                    </span>
                </form>
            </Section>
        </Page>
    }
}
