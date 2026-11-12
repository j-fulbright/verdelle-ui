use crate::input_group::InputGroupContext;
use crate::text_field;
use leptos::{ev, prelude::*};

/// The text field of an [`crate::InputGroup`]: a bare native `<input>` that fills the free
/// width. The group draws the border and ring, so unlike [`crate::Input`] this has no
/// frame of its own. `value` is two-way; `on_input`, `on_change`, `on_keydown` and `on_enter` work as on
/// [`crate::Input`]. It is disabled and marked invalid when the group is.
#[component]
pub fn InputGroupInput(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    /// The HTML input type: `text`, `password`, `email`, `search`, and so on.
    #[prop(default = "text")]
    kind: &'static str,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Read-only: focusable and selectable, but the value cannot be changed.
    #[prop(optional, into)]
    readonly: Signal<bool>,
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
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Called on every edit with the new text.
    #[prop(optional, into)]
    on_input: Option<Callback<String>>,
    /// Called when the browser commits the edit (on blur, or Enter), with the text.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Called on every key press in the field.
    #[prop(optional, into)]
    on_keydown: Option<Callback<ev::KeyboardEvent>>,
    /// Called when Enter is pressed (not while an IME is composing).
    #[prop(optional, into)]
    on_enter: Option<Callback<()>>,
) -> impl IntoView {
    let group = use_context::<InputGroupContext>();
    let group_disabled = move || group.is_some_and(|g| g.disabled.get());
    let invalid = move || group.is_some_and(|g| g.invalid.get());
    view! {
        <input
            class=format!("vdl-input-group__control {class}")
            type=kind
            id=id
            name=name
            placeholder=placeholder
            aria-label=aria_label
            aria-invalid=move || invalid().then_some("true")
            disabled=move || disabled.get() || group_disabled()
            readonly=move || readonly.get()
            prop:value=move || value.get()
            on:input=move |ev| {
                let text = event_target_value(&ev);
                value.set(text.clone());
                if let Some(cb) = on_input {
                    cb.run(text);
                }
            }
            on:change=move |ev| {
                if let Some(cb) = on_change {
                    cb.run(event_target_value(&ev));
                }
            }
            on:keydown=move |ev| text_field::handle_keydown(ev, on_keydown, on_enter, false)
        />
    }
}
