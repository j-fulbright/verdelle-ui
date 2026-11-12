use crate::icons;
use crate::primitives::password_toggle as primitive;
use leptos::prelude::*;

/// A password field with a show/hide button, in the Input Group look, built on the
/// headless Password Toggle Field.
///
/// The text is hidden (`type="password"`) until the eye button is pressed, which flips the
/// input to `type="text"`; the button is a real toggle (`aria-pressed`, and its name changes
/// between "Show password" and "Hide password"). `value` is two-way. `invalid` draws the
/// destructive border and ring; `disabled` dims the whole field. `default_visible` starts it
/// revealed. Use `autocomplete` as the browser expects (`current-password` or `new-password`).
#[component]
pub fn PasswordInput(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    #[prop(into, optional)] default_visible: MaybeProp<bool>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Read-only: focusable and selectable, but the value cannot be changed.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Text shown while there is no value.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Form field name; the value is submitted under it.
    #[prop(optional, into)]
    name: Option<String>,
    #[prop(optional, into)] autocomplete: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let id = StoredValue::new(id);
    let name = StoredValue::new(name);
    let placeholder = StoredValue::new(placeholder);
    let aria_label = StoredValue::new(aria_label);
    let autocomplete = StoredValue::new(autocomplete);

    view! {
        <primitive::PasswordToggleField
            default_visible=default_visible
            attr:class=format!("vdl-input-group vdl-password {class}")
            attr:data-disabled=move || disabled.get().then_some("")
            attr:data-invalid=move || invalid.get().then_some("")
        >
            <primitive::PasswordToggleFieldInput
                attr:class="vdl-input-group__control"
                attr:id=move || id.get_value()
                attr:name=move || name.get_value()
                attr:placeholder=move || placeholder.get_value()
                attr:aria-label=move || aria_label.get_value()
                attr:autocomplete=move || autocomplete.get_value()
                attr:aria-invalid=move || invalid.get().then_some("true")
                attr:disabled=move || disabled.get()
                attr:readonly=move || readonly.get()
                prop:value=move || value.get()
                on:input=move |ev| value.set(event_target_value(&ev))
            />
            <div class="vdl-input-group__addon" data-align="inline-end">
                <primitive::PasswordToggleFieldToggle
                    attr:class="vdl-btn vdl-input-group__button vdl-password__toggle"
                    attr:data-variant="ghost"
                    attr:data-size="xs"
                    attr:data-icon-only=""
                    attr:data-ig-size="icon-xs"
                    attr:disabled=move || disabled.get()
                >
                    <span class="vdl-btn__content">
                        <span class="vdl-password__show">{icons::eye()}</span>
                        <span class="vdl-password__hide">{icons::eye_off()}</span>
                    </span>
                </primitive::PasswordToggleFieldToggle>
            </div>
        </primitive::PasswordToggleField>
    }
}
