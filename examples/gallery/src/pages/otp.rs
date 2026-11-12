use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, InputOtp, InputOtpGroup, InputOtpSeparator, InputOtpSlot,
    InputValidationType, Size,
};

#[component]
pub fn OtpPage() -> impl IntoView {
    let code = RwSignal::new(String::new());
    let completed = RwSignal::new(String::from("(not yet)"));
    let on_complete = Callback::new(move |v: String| completed.set(v));
    let on_change = Callback::new(move |v: String| code.set(v));
    let controlled = RwSignal::new(String::from("12"));
    let on_controlled = Callback::new(move |v: String| controlled.set(v));
    let submitted = RwSignal::new(String::from("(nothing yet)"));
    let wrong = RwSignal::new(false);
    let check = Callback::new(move |v: String| wrong.set(v != "123456"));

    view! {
        <Page
            title="Input OTP"
            summary="InputOtp, built on the headless one-time-password primitive. Type to fill and advance, Backspace to clear and step back, arrows/Home/End to move, paste a whole code."
        >
            <Section title="Six digits in two groups" description="The usual shape: two groups and a separator. Paste 123456 into any slot to fill them all.">
                <div class="grid gap-3">
                    <InputOtp
                        aria_label="Verification code"
                        on_value_change=on_change
                        on_complete=on_complete
                    >
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                        </InputOtpGroup>
                        <InputOtpSeparator />
                        <InputOtpGroup>
                            <InputOtpSlot index=3 />
                            <InputOtpSlot index=4 />
                            <InputOtpSlot index=5 />
                        </InputOtpGroup>
                    </InputOtp>
                    <span id="otp-readout" class="text-sm text-muted-foreground">
                        "code: " {move || format!("\"{}\"", code.get())} ", completed: "
                        {move || completed.get()}
                    </span>
                </div>
            </Section>

            <Section title="Length and joined group" description="max_length 4 with one group; max_length 8 shows that the length is whatever you render.">
                <div class="grid gap-4">
                    <InputOtp max_length=4 aria_label="PIN">
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                            <InputOtpSlot index=3 />
                        </InputOtpGroup>
                    </InputOtp>
                    <InputOtp max_length=8 aria_label="Long code">
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                            <InputOtpSlot index=3 />
                        </InputOtpGroup>
                        <InputOtpSeparator />
                        <InputOtpGroup>
                            <InputOtpSlot index=4 />
                            <InputOtpSlot index=5 />
                            <InputOtpSlot index=6 />
                            <InputOtpSlot index=7 />
                        </InputOtpGroup>
                    </InputOtp>
                </div>
            </Section>

            <Section title="Pattern" description="Digits only (default), letters and digits, or anything. Characters the pattern rejects are ignored, also when pasted.">
                <div class="grid gap-4">
                    <Row label="Digits">
                        <InputOtp max_length=4 aria_label="Digits only">
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                    </Row>
                    <Row label="Letters and digits">
                        <InputOtp max_length=4 pattern=InputValidationType::Alphanumeric aria_label="Alphanumeric">
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                    </Row>
                    <Row label="Any text">
                        <InputOtp max_length=4 pattern=InputValidationType::Text aria_label="Any text">
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                    </Row>
                </div>
            </Section>

            <Section title="Default value, disabled and invalid" description="A default of 12 fills two slots; disabled dims and blocks input; invalid draws the destructive ring.">
                <div class="grid gap-4">
                    <Row label="Default 12">
                        <InputOtp max_length=4 default_value="12" aria_label="With default">
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                    </Row>
                    <Row label="Disabled">
                        <InputOtp max_length=4 default_value="1234" disabled=true aria_label="Disabled">
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                    </Row>
                    <Row label="Invalid">
                        <InputOtp max_length=4 default_value="9999" invalid=true aria_label="Invalid">
                            <InputOtpGroup>
                                <InputOtpSlot index=0 />
                                <InputOtpSlot index=1 />
                                <InputOtpSlot index=2 />
                                <InputOtpSlot index=3 />
                            </InputOtpGroup>
                        </InputOtp>
                    </Row>
                </div>
            </Section>

            <Section title="Validated on complete" description="The right code is 123456; anything else completes in the invalid state.">
                <div class="grid gap-2">
                    <InputOtp aria_label="Check me" invalid=wrong on_complete=check>
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                            <InputOtpSlot index=3 />
                            <InputOtpSlot index=4 />
                            <InputOtpSlot index=5 />
                        </InputOtpGroup>
                    </InputOtp>
                    <span id="otp-check" class="text-sm text-muted-foreground">
                        {move || if wrong.get() { "Wrong code." } else { "Waiting for a full code, or correct." }}
                    </span>
                </div>
            </Section>

            <Section title="Controlled" description="A signal owns the code; the buttons set and clear it from outside.">
                <div class="grid gap-3">
                    <InputOtp
                        max_length=4
                        aria_label="Controlled"
                        value=controlled
                        on_value_change=on_controlled
                    >
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                            <InputOtpSlot index=3 />
                        </InputOtpGroup>
                    </InputOtp>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(String::from("4321"))
                        >
                            "Set 4321"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(String::new())
                        >
                            "Clear"
                        </Button>
                    </Row>
                    <span id="otp-ctl-readout" class="text-sm text-muted-foreground">
                        "value: " {move || format!("\"{}\"", controlled.get())}
                    </span>
                </div>
            </Section>

            <Section title="In a form" description="With name set, the code submits with the form.">
                <form
                    class="grid max-w-md gap-3"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        use leptos::wasm_bindgen::JsCast;
                        if let Some(form) = ev
                            .target()
                            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                            && let Ok(data) = leptos::web_sys::FormData::new_with_form(&form)
                        {
                            submitted.set(
                                data.get("otp").as_string().unwrap_or_else(|| "(absent)".into()),
                            );
                        }
                    }
                >
                    <InputOtp max_length=4 name="otp" aria_label="Form code">
                        <InputOtpGroup>
                            <InputOtpSlot index=0 />
                            <InputOtpSlot index=1 />
                            <InputOtpSlot index=2 />
                            <InputOtpSlot index=3 />
                        </InputOtpGroup>
                    </InputOtp>
                    <div>
                        <button type="submit" id="otp-submit" class="vdl-btn" data-variant="default" data-size="md">
                            "Verify"
                        </button>
                    </div>
                    <span id="otp-form-readout" class="text-sm text-muted-foreground">
                        "submitted: " {move || submitted.get()}
                    </span>
                </form>
            </Section>
        </Page>
    }
}
