use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, NumberField, NumberFieldDecrement, NumberFieldGroup,
    NumberFieldIncrement, NumberFieldInput, NumberFieldScrubArea, Size,
};

fn show(v: Option<f64>) -> String {
    v.map_or("empty".to_string(), |v| v.to_string())
}

#[component]
fn Plain(#[prop(optional, into)] label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <span class="text-xs text-muted-foreground">{label}</span>
            {children()}
        </div>
    }
}

#[component]
pub fn NumberFieldsPage() -> impl IntoView {
    let basic = RwSignal::new(Some(5.0));
    let controlled = RwSignal::new(Some(20.0));
    let commits = RwSignal::new(0u32);
    let ranged = RwSignal::new(Some(3.0));
    let money = RwSignal::new(Some(19.99));
    let scrub = RwSignal::new(Some(50.0));
    let submitted = RwSignal::new(String::new());

    view! {
        <Page
            title="Number field"
            summary="A text input for numbers with minus and plus buttons. The input is a spinbutton: Up and Down step, Shift and PageUp/PageDown take the large step, Alt the small one. Typing is filtered to digits, one point and a sign; blur and Enter settle the value (clamped and rounded). Base UI has a Number Field, shadcn does not."
        >
            <Section
                title="Basic"
                description="Uncontrolled, starting at 5. Hold a button to repeat. The value below follows every change."
            >
                <NumberField
                    default_value=5.0
                    on_value_change=move |v| basic.set(v)
                >
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Quantity" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
                <Row label="Value">
                    <span class="text-sm text-muted-foreground" data-testid="nf-basic">
                        {move || show(basic.get())}
                    </span>
                </Row>
            </Section>

            <Section
                title="Min and max"
                description="min=0 and max=10: the buttons disable at the limits, stepping and typing clamp, Home and End jump to the limits."
            >
                <NumberField default_value=3.0 min=0.0 max=10.0 on_value_change=move |v| ranged.set(v)>
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Rating" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
                <Row label="Value">
                    <span class="text-sm text-muted-foreground" data-testid="nf-range">
                        {move || show(ranged.get())}
                    </span>
                </Row>
            </Section>

            <Section
                title="Steps"
                description="step=5, small_step=1, large_step=25. Arrow keys step 5, Alt+Arrow 1, Shift+Arrow and PageUp/PageDown 25."
            >
                <NumberField default_value=50.0 step=5.0 small_step=1.0 large_step=25.0>
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Volume" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
            </Section>

            <Section
                title="Snap on step"
                description="snap_on_step with step=5 from min=0: 3 steps up to 5, not to 8."
            >
                <NumberField default_value=3.0 min=0.0 step=5.0 snap_on_step=true>
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Snapped" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
            </Section>

            <Section
                title="Decimals, prefix and suffix"
                description="decimals=2 rounds and shows two places; prefix and suffix are shown around the number and may be left out when typing. 0.1 steps stay free of float noise."
            >
                <Row>
                    <Plain label="Price">
                        <NumberField
                            default_value=19.99
                            decimals=2
                            prefix="$"
                            step=0.01
                            min=0.0
                            on_value_change=move |v| money.set(v)
                        >
                            <NumberFieldGroup>
                                <NumberFieldDecrement />
                                <NumberFieldInput aria_label="Price" />
                                <NumberFieldIncrement />
                            </NumberFieldGroup>
                        </NumberField>
                    </Plain>
                    <Plain label="Weight">
                        <NumberField default_value=72.5 suffix=" kg" step=0.1>
                            <NumberFieldGroup>
                                <NumberFieldDecrement />
                                <NumberFieldInput aria_label="Weight" />
                                <NumberFieldIncrement />
                            </NumberFieldGroup>
                        </NumberField>
                    </Plain>
                </Row>
                <Row label="Price value">
                    <span class="text-sm text-muted-foreground" data-testid="nf-money">
                        {move || show(money.get())}
                    </span>
                </Row>
            </Section>

            <Section
                title="Empty"
                description="No value: the placeholder shows, and the first step goes to 1 (or into the range)."
            >
                <NumberField>
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Empty" placeholder="Any" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
            </Section>

            <Section
                title="Controlled"
                description="value is a signal that is read and written; on_value_commit counts settled values (blur, Enter, steps)."
            >
                <NumberField
                    value=controlled
                    on_value_commit=move |_| commits.update(|n| *n += 1)
                >
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Controlled" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
                <Row>
                    <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| controlled.set(Some(100.0))>
                        "Set 100"
                    </Button>
                    <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| controlled.set(None)>
                        "Clear"
                    </Button>
                    <span class="text-sm text-muted-foreground" data-testid="nf-controlled">
                        {move || format!("value: {} / commits: {}", show(controlled.get()), commits.get())}
                    </span>
                </Row>
            </Section>

            <Section title="Sizes" description="Small 28px, default 32px, large 36px.">
                <Row>
                    <NumberField default_value=1.0 size=Size::Small>
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Small" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                    <NumberField default_value=2.0>
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Default" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                    <NumberField default_value=3.0 size=Size::Large>
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Large" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                </Row>
            </Section>

            <Section
                title="States"
                description="Invalid draws the destructive border and ring; disabled dims the field and blocks every input; read-only can be focused and copied but not changed."
            >
                <Row>
                    <NumberField default_value=7.0 invalid=true>
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Invalid" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                    <NumberField default_value=7.0 disabled=true>
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Disabled" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                    <NumberField default_value=7.0 read_only=true>
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Read only" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                </Row>
            </Section>

            <Section
                title="Scrub area"
                description="Drag the label sideways: one step per 4px, right for up, left for down (Shift large, Alt small). The pointer is captured, so the drag survives leaving the label."
            >
                <NumberField value=scrub min=0.0 max=100.0>
                    <NumberFieldScrubArea class="w-24">"Opacity"</NumberFieldScrubArea>
                    <NumberFieldGroup>
                        <NumberFieldDecrement />
                        <NumberFieldInput aria_label="Opacity" />
                        <NumberFieldIncrement />
                    </NumberFieldGroup>
                </NumberField>
            </Section>

            <Section
                title="In a form"
                description="name submits the number itself (without prefix or suffix) through a hidden input."
            >
                <form
                    class="flex items-end gap-2"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        let form = event_target::<leptos::web_sys::HtmlFormElement>(&ev);
                        if let Ok(data) = leptos::web_sys::FormData::new_with_form(&form) {
                            let amount = data.get("amount").as_string().unwrap_or_default();
                            submitted.set(format!("amount={amount}"));
                        }
                    }
                >
                    <NumberField default_value=12.5 decimals=2 prefix="$" name="amount">
                        <NumberFieldGroup>
                            <NumberFieldDecrement />
                            <NumberFieldInput aria_label="Amount" />
                            <NumberFieldIncrement />
                        </NumberFieldGroup>
                    </NumberField>
                    <button type="submit" class="vdl-btn" data-variant="default" data-size="md">
                        <span class="vdl-btn__content">"Submit"</span>
                    </button>
                </form>
                <Row>
                    <span class="text-sm text-muted-foreground" data-testid="nf-form">
                        {move || submitted.get()}
                    </span>
                </Row>
            </Section>
        </Page>
    }
}
