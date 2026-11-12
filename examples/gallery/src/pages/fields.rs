use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::primitives::form::Form;
use verdelle_ui::{
    Checkbox, Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel,
    FieldLegend, FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle, Input,
    RadioGroup, RadioGroupItem, Switch, Textarea,
};

fn focus_id(id: &str) {
    use leptos::wasm_bindgen::JsCast;
    if let Some(el) = leptos::web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
        .and_then(|e| e.dyn_into::<leptos::web_sys::HtmlElement>().ok())
    {
        let _ = el.focus();
    }
}

#[component]
pub fn FieldsPage() -> impl IntoView {
    let no_errors: Vec<String> = Vec::new();
    // Basic
    let username = RwSignal::new(String::new());
    // Invalid examples
    let bad_email = RwSignal::new(String::from("not-an-email"));
    let bad_pw = RwSignal::new(String::from("abc"));
    let bio = RwSignal::new(String::new());
    let terms_bad = RwSignal::new(false);
    // Disabled
    let dis_text = RwSignal::new(String::from("Locked value"));
    let dis_check = RwSignal::new(true);
    // Horizontal
    let h_check = RwSignal::new(true);
    let h_check2 = RwSignal::new(false);
    // Cards
    let plan = RwSignal::new(String::from("pro"));
    let card_a = RwSignal::new(true);
    let card_b = RwSignal::new(false);
    // Sign-up form
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let about = RwSignal::new(String::new());
    let agree = RwSignal::new(false);
    let attempted = RwSignal::new(false);
    let result = RwSignal::new(String::from("(not submitted)"));
    let name_errors = Signal::derive(move || {
        let mut e = Vec::new();
        if attempted.get() && name.get().trim().is_empty() {
            e.push(String::from("Enter your name."));
        }
        e
    });
    let email_errors = Signal::derive(move || {
        let mut e = Vec::new();
        if attempted.get() {
            let v = email.get();
            if v.trim().is_empty() {
                e.push(String::from("Enter your email address."));
            } else if !v.contains('@') || !v.contains('.') {
                e.push(String::from("That does not look like an email address."));
            }
        }
        e
    });
    let password_errors = Signal::derive(move || {
        let mut e = Vec::new();
        if attempted.get() {
            let v = password.get();
            if v.chars().count() < 8 {
                e.push(String::from("Use at least 8 characters."));
            }
            if !v.chars().any(|c| c.is_ascii_digit()) {
                e.push(String::from("Include a number."));
            }
        }
        e
    });
    let agree_errors = Signal::derive(move || {
        let mut e = Vec::new();
        if attempted.get() && !agree.get() {
            e.push(String::from("You must accept the terms."));
        }
        e
    });
    let on_submit = Callback::new(move |ev: leptos::web_sys::Event| {
        ev.prevent_default();
        attempted.set(true);
        let first_bad = if !name_errors.get_untracked().is_empty() {
            Some("su-name")
        } else if !email_errors.get_untracked().is_empty() {
            Some("su-email")
        } else if !password_errors.get_untracked().is_empty() {
            Some("su-password")
        } else if !agree_errors.get_untracked().is_empty() {
            Some("su-terms")
        } else {
            None
        };
        match first_bad {
            Some(id) => {
                result.set(String::from("(blocked: fix the errors)"));
                focus_id(id);
            }
            None => result.set(format!(
                "ok: {} <{}>",
                name.get_untracked(),
                email.get_untracked()
            )),
        }
    });

    view! {
        <Page
            title="Field"
            summary="The Field family: Field, FieldLabel, FieldDescription, FieldError, FieldGroup, FieldSet, FieldLegend, FieldContent, FieldTitle and FieldSeparator lay out and label any control. Errors use the destructive colour and are announced."
        >
            <Section title="Field, label and description" description="Label above the control (vertical, the default); the description is linked to the input with aria-describedby.">
                <div class="max-w-sm">
                    <Field>
                        <FieldLabel html_for="f-username">"Username"</FieldLabel>
                        <Input id="f-username" value=username placeholder="jdoe" aria_describedby="f-username-hint" />
                        <FieldDescription id="f-username-hint">
                            "Choose a unique username for your account."
                        </FieldDescription>
                    </Field>
                </div>
            </Section>

            <Section title="Orientation" description="Vertical stacks the label over the control. Horizontal puts them side by side. Responsive is vertical until its FieldGroup is 28rem wide.">
                <div class="grid max-w-md gap-6">
                    <FieldGroup>
                        <Field orientation=FieldOrientation::Horizontal>
                            <Checkbox id="f-h1" checked=h_check />
                            <FieldLabel html_for="f-h1">"Accept the terms and conditions"</FieldLabel>
                        </Field>
                        <Field orientation=FieldOrientation::Horizontal>
                            <Checkbox id="f-h2" checked=h_check2 aria_describedby="f-h2-hint" />
                            <FieldContent>
                                <FieldLabel html_for="f-h2">"Share usage data"</FieldLabel>
                                <FieldDescription id="f-h2-hint">
                                    "Helps us improve. You can turn this off later in settings."
                                </FieldDescription>
                            </FieldContent>
                        </Field>
                        <Field orientation=FieldOrientation::Horizontal>
                            <FieldContent>
                                <FieldLabel html_for="f-h3">"Airplane mode"</FieldLabel>
                                <FieldDescription>"Turns off all wireless connections."</FieldDescription>
                            </FieldContent>
                            <Switch id="f-h3" />
                        </Field>
                    </FieldGroup>
                </div>
                <div class="grid max-w-xl gap-4">
                    <div class="w-80 max-w-full rounded-lg border p-4">
                        <p class="mb-3 text-xs text-muted-foreground">"Responsive, in a 320px group: stays vertical"</p>
                        <FieldGroup>
                            <Field orientation=FieldOrientation::Responsive>
                                <FieldContent>
                                    <FieldLabel html_for="f-r1">"Display name"</FieldLabel>
                                    <FieldDescription>"Shown on your profile."</FieldDescription>
                                </FieldContent>
                                <Input id="f-r1" value=RwSignal::new(String::new()) placeholder="Ada" />
                            </Field>
                        </FieldGroup>
                    </div>
                    <div class="rounded-lg border p-4">
                        <p class="mb-3 text-xs text-muted-foreground">"Responsive, in a wide group: label left, control right"</p>
                        <FieldGroup>
                            <Field orientation=FieldOrientation::Responsive>
                                <FieldContent>
                                    <FieldLabel html_for="f-r2">"Display name"</FieldLabel>
                                    <FieldDescription>"Shown on your profile."</FieldDescription>
                                </FieldContent>
                                <Input id="f-r2" value=RwSignal::new(String::new()) placeholder="Ada" />
                            </Field>
                        </FieldGroup>
                    </div>
                </div>
            </Section>

            <Section title="Invalid and error messages" description="invalid on the Field turns its text destructive; invalid on the control draws the ring. FieldError renders nothing until there are errors; several distinct errors become a list.">
                <div class="grid max-w-sm gap-6">
                    <FieldGroup>
                        <Field invalid=true>
                            <FieldLabel html_for="f-bad-email">"Email"</FieldLabel>
                            <Input id="f-bad-email" value=bad_email invalid=true aria_describedby="f-bad-email-err" />
                            <FieldError id="f-bad-email-err" errors=vec![String::from("Enter a valid email address.")] />
                        </Field>
                        <Field invalid=true>
                            <FieldLabel html_for="f-bad-pw">"Password"</FieldLabel>
                            <Input id="f-bad-pw" kind="password" value=bad_pw invalid=true aria_describedby="f-bad-pw-err" />
                            <FieldError
                                id="f-bad-pw-err"
                                errors=vec![
                                    String::from("Use at least 8 characters."),
                                    String::from("Include a number."),
                                    String::from("Use at least 8 characters."),
                                ]
                            />
                        </Field>
                        <Field invalid=true>
                            <FieldLabel html_for="f-bad-bio">"Bio"</FieldLabel>
                            <Textarea id="f-bad-bio" value=bio invalid=true aria_describedby="f-bad-bio-err" />
                            <FieldError id="f-bad-bio-err">"Tell us a little about yourself."</FieldError>
                        </Field>
                        <Field invalid=true orientation=FieldOrientation::Horizontal>
                            <Checkbox id="f-bad-terms" checked=terms_bad invalid=true aria_describedby="f-bad-terms-err" />
                            <FieldContent>
                                <FieldLabel html_for="f-bad-terms">"I agree to the terms"</FieldLabel>
                                <FieldError id="f-bad-terms-err" errors=vec![String::from("You must accept the terms.")] />
                            </FieldContent>
                        </Field>
                        <Field>
                            <FieldLabel html_for="f-no-err">"No errors: nothing rendered below"</FieldLabel>
                            <Input id="f-no-err" value=RwSignal::new(String::new()) />
                            <FieldError errors=no_errors />
                        </Field>
                    </FieldGroup>
                </div>
            </Section>

            <Section title="Disabled" description="disabled on the Field dims the label and title; disable the control as well. A disabled FieldSet disables everything inside.">
                <div class="grid max-w-sm gap-6">
                    <Field disabled=true>
                        <FieldLabel html_for="f-dis">"Disabled field"</FieldLabel>
                        <Input id="f-dis" value=dis_text disabled=true />
                        <FieldDescription>"You cannot change this."</FieldDescription>
                    </Field>
                    <Field disabled=true orientation=FieldOrientation::Horizontal>
                        <Checkbox id="f-dis-c" checked=dis_check disabled=true />
                        <FieldLabel html_for="f-dis-c">"Disabled checkbox"</FieldLabel>
                    </Field>
                    <FieldSet disabled=true>
                        <FieldLegend variant=FieldLegendVariant::Label>"Disabled fieldset"</FieldLegend>
                        <FieldGroup>
                            <Field>
                                <FieldLabel html_for="f-dis-fs">"Inside the fieldset"</FieldLabel>
                                <Input id="f-dis-fs" value=RwSignal::new(String::from("Also locked")) />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </div>
            </Section>

            <Section title="FieldSet and FieldLegend" description="A fieldset names a group of related fields. The legend is 16px (legend) or label-sized (label); a description placed right after it tucks closer.">
                <div class="grid max-w-md gap-8">
                    <FieldSet>
                        <FieldLegend>"Address"</FieldLegend>
                        <FieldDescription>"Where should we send your order?"</FieldDescription>
                        <FieldGroup>
                            <Field>
                                <FieldLabel html_for="f-street">"Street"</FieldLabel>
                                <Input id="f-street" value=RwSignal::new(String::new()) />
                            </Field>
                            <div class="grid grid-cols-2 gap-4">
                                <Field>
                                    <FieldLabel html_for="f-city">"City"</FieldLabel>
                                    <Input id="f-city" value=RwSignal::new(String::new()) />
                                </Field>
                                <Field>
                                    <FieldLabel html_for="f-zip">"Postal code"</FieldLabel>
                                    <Input id="f-zip" value=RwSignal::new(String::new()) />
                                </Field>
                            </div>
                        </FieldGroup>
                    </FieldSet>
                    <FieldSet>
                        <FieldLegend variant=FieldLegendVariant::Label>"Notify me about"</FieldLegend>
                        <FieldDescription>"Choose one."</FieldDescription>
                        <RadioGroup aria_label="Notify me about" default_value="all">
                            <Field orientation=FieldOrientation::Horizontal>
                                <RadioGroupItem id="f-n-all" value="all" />
                                <FieldLabel html_for="f-n-all">"All new messages"</FieldLabel>
                            </Field>
                            <Field orientation=FieldOrientation::Horizontal>
                                <RadioGroupItem id="f-n-mention" value="mention" />
                                <FieldLabel html_for="f-n-mention">"Direct messages and mentions"</FieldLabel>
                            </Field>
                            <Field orientation=FieldOrientation::Horizontal>
                                <RadioGroupItem id="f-n-none" value="none" />
                                <FieldLabel html_for="f-n-none">"Nothing"</FieldLabel>
                            </Field>
                        </RadioGroup>
                    </FieldSet>
                </div>
            </Section>

            <Section title="FieldGroup and FieldSeparator" description="A group spaces its fields; a separator draws a line, with optional text in the middle.">
                <div class="max-w-sm">
                    <FieldGroup>
                        <Field>
                            <FieldLabel html_for="f-g-email">"Email"</FieldLabel>
                            <Input id="f-g-email" value=RwSignal::new(String::new()) placeholder="you@example.com" />
                        </Field>
                        <FieldSeparator />
                        <Field>
                            <FieldLabel html_for="f-g-pw">"Password"</FieldLabel>
                            <Input id="f-g-pw" kind="password" value=RwSignal::new(String::new()) />
                        </Field>
                        <FieldSeparator>"Or continue with"</FieldSeparator>
                        <Field>
                            <FieldLabel html_for="f-g-code">"Recovery code"</FieldLabel>
                            <Input id="f-g-code" value=RwSignal::new(String::new()) />
                        </Field>
                    </FieldGroup>
                </div>
            </Section>

            <Section title="Selectable cards" description="Wrap a Field in a FieldLabel to make the whole card the label. It highlights when its control is checked, on hover and on keyboard focus.">
                <div class="grid max-w-md gap-6">
                    <FieldGroup>
                        <RadioGroup
                            aria_label="Plan"
                            value=plan
                            on_value_change=move |v| plan.set(v)
                        >
                            <FieldLabel html_for="f-plan-pro">
                                <Field orientation=FieldOrientation::Horizontal>
                                    <FieldContent>
                                        <FieldTitle>"Pro"</FieldTitle>
                                        <FieldDescription>"For individuals and small teams."</FieldDescription>
                                    </FieldContent>
                                    <RadioGroupItem id="f-plan-pro" value="pro" />
                                </Field>
                            </FieldLabel>
                            <FieldLabel html_for="f-plan-team">
                                <Field orientation=FieldOrientation::Horizontal>
                                    <FieldContent>
                                        <FieldTitle>"Team"</FieldTitle>
                                        <FieldDescription>"Shared workspaces and roles."</FieldDescription>
                                    </FieldContent>
                                    <RadioGroupItem id="f-plan-team" value="team" />
                                </Field>
                            </FieldLabel>
                        </RadioGroup>
                        <span id="f-plan-readout" class="text-sm text-muted-foreground">
                            "plan: " {move || plan.get()}
                        </span>
                        <FieldLabel html_for="f-card-a">
                            <Field orientation=FieldOrientation::Horizontal>
                                <Checkbox id="f-card-a" checked=card_a />
                                <FieldContent>
                                    <FieldTitle>"Email updates"</FieldTitle>
                                    <FieldDescription>"Product news, once a month."</FieldDescription>
                                </FieldContent>
                            </Field>
                        </FieldLabel>
                        <FieldLabel html_for="f-card-b">
                            <Field orientation=FieldOrientation::Horizontal disabled=true>
                                <Checkbox id="f-card-b" checked=card_b disabled=true />
                                <FieldContent>
                                    <FieldTitle>"Beta features (disabled)"</FieldTitle>
                                    <FieldDescription>"Not available on your plan."</FieldDescription>
                                </FieldContent>
                            </Field>
                        </FieldLabel>
                    </FieldGroup>
                </div>
            </Section>

            <Section title="Validation with the form primitive" description="verdelle_ui::primitives::form::Form handles submit; the page validates (name, email, password rules, terms), shows FieldError per field, and moves focus to the first problem. Errors clear as you fix them.">
                <div class="max-w-sm">
                    <Form on_submit=on_submit attr:novalidate="" attr:id="su-form">
                        <FieldGroup>
                            <Field invalid=Signal::derive(move || !name_errors.get().is_empty())>
                                <FieldLabel html_for="su-name">"Name"</FieldLabel>
                                <Input
                                    id="su-name"
                                    value=name
                                    invalid=Signal::derive(move || !name_errors.get().is_empty())
                                    aria_describedby="su-name-err"
                                />
                                <FieldError id="su-name-err" errors=name_errors />
                            </Field>
                            <Field invalid=Signal::derive(move || !email_errors.get().is_empty())>
                                <FieldLabel html_for="su-email">"Email"</FieldLabel>
                                <Input
                                    id="su-email"
                                    kind="email"
                                    value=email
                                    placeholder="you@example.com"
                                    invalid=Signal::derive(move || !email_errors.get().is_empty())
                                    aria_describedby="su-email-err"
                                />
                                <FieldError id="su-email-err" errors=email_errors />
                            </Field>
                            <Field invalid=Signal::derive(move || !password_errors.get().is_empty())>
                                <FieldLabel html_for="su-password">"Password"</FieldLabel>
                                <Input
                                    id="su-password"
                                    kind="password"
                                    value=password
                                    invalid=Signal::derive(move || !password_errors.get().is_empty())
                                    aria_describedby="su-password-hint su-password-err"
                                />
                                <FieldDescription id="su-password-hint">
                                    "At least 8 characters, with a number."
                                </FieldDescription>
                                <FieldError id="su-password-err" errors=password_errors />
                            </Field>
                            <Field>
                                <FieldLabel html_for="su-about">"About you (optional)"</FieldLabel>
                                <Textarea id="su-about" value=about />
                            </Field>
                            <Field
                                orientation=FieldOrientation::Horizontal
                                invalid=Signal::derive(move || !agree_errors.get().is_empty())
                            >
                                <Checkbox
                                    id="su-terms"
                                    checked=agree
                                    invalid=Signal::derive(move || !agree_errors.get().is_empty())
                                    aria_describedby="su-terms-err"
                                />
                                <FieldContent>
                                    <FieldLabel html_for="su-terms">"I accept the terms"</FieldLabel>
                                    <FieldError id="su-terms-err" errors=agree_errors />
                                </FieldContent>
                            </Field>
                            <Field orientation=FieldOrientation::Horizontal>
                                <button type="submit" id="su-submit" class="vdl-btn" data-variant="default" data-size="md">
                                    "Create account"
                                </button>
                            </Field>
                            <span id="su-result" class="text-sm text-muted-foreground">
                                "result: " {move || result.get()}
                            </span>
                        </FieldGroup>
                    </Form>
                </div>
            </Section>
        </Page>
    }
}
