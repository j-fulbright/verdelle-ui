use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::PasswordInput;

#[component]
pub fn PasswordInputsPage() -> impl IntoView {
    let basic = RwSignal::new(String::new());
    let shown = RwSignal::new("hunter2".to_string());
    let invalid = RwSignal::new("short".to_string());
    let disabled = RwSignal::new("locked".to_string());
    let readonly = RwSignal::new("read only".to_string());
    let submitted = RwSignal::new(String::new());

    view! {
        <Page
            title="Password input"
            summary="An Input Group with an eye button that switches the field between password and text. The button is a real toggle: aria-pressed, and its name changes between Show password and Hide password."
        >
            <Section
                title="Basic"
                description="Hidden until the eye is pressed. The value is two-way: what is typed shows below."
            >
                <div class="w-80 max-w-full">
                    <PasswordInput
                        value=basic
                        placeholder="Password"
                        aria_label="Password"
                        autocomplete="current-password"
                    />
                </div>
                <Row label="Value">
                    <span class="text-sm text-muted-foreground" data-testid="pw-basic">
                        {move || basic.get()}
                    </span>
                </Row>
            </Section>

            <Section title="Shown at first" description="default_visible=true.">
                <div class="w-80 max-w-full">
                    <PasswordInput value=shown default_visible=true aria_label="Shown password" />
                </div>
            </Section>

            <Section
                title="States"
                description="Invalid draws the destructive border and ring; disabled dims the field and the button; read-only keeps the normal look."
            >
                <div class="flex w-80 max-w-full flex-col gap-3">
                    <PasswordInput value=invalid invalid=true aria_label="Invalid password" />
                    <PasswordInput value=disabled disabled=true aria_label="Disabled password" />
                    <PasswordInput value=readonly readonly=true aria_label="Read-only password" />
                </div>
            </Section>

            <Section
                title="In a form"
                description="name and autocomplete are passed through, so the browser's password manager and a native form submit work."
            >
                <form
                    class="flex w-80 max-w-full items-center gap-2"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        submitted.set("submitted".to_string());
                    }
                >
                    <PasswordInput
                        value=RwSignal::new(String::new())
                        name="new-password"
                        autocomplete="new-password"
                        placeholder="New password"
                        aria_label="New password"
                    />
                    <button type="submit" class="vdl-btn" data-variant="default" data-size="md">
                        <span class="vdl-btn__content">"Save"</span>
                    </button>
                </form>
                <Row>
                    <span class="text-sm text-muted-foreground" data-testid="pw-submit">
                        {move || submitted.get()}
                    </span>
                </Row>
            </Section>
        </Page>
    }
}
