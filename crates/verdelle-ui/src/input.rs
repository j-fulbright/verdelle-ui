use crate::{Size, text_field};
use leptos::{ev, html, prelude::*};

/// A single-line text field, styled as shadcn/ui's Input.
///
/// The browser's own `<input>` does the editing, selection, IME and autofill,
/// inside a styled frame that draws the border, focus ring and disabled state.
///
/// `value` is two-way. Give the field an accessible name with `aria_label`, or
/// point a `<label for>` at its `id`. `prefix` and `suffix` render inside the
/// frame, before and after the text. `class` goes on the frame (use it for width or a
/// background); `field_class` goes on the native `<input>` (use it for a font, such as `font-mono`).
/// `on_input` runs on every edit with the new text, `on_change` when the browser commits it (blur or
/// Enter), `on_keydown` on every key and `on_enter` on a plain Enter (not while an IME is composing).
/// Not provided here: a loading state, and password and number fields, which are
/// [`crate::PasswordInput`] and [`crate::NumberField`]. Only the native attributes listed below
/// are exposed (no `required`, `maxlength`, `pattern`, `inputmode`, `autofocus` or focus and blur
/// callbacks).
#[component]
pub fn Input(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Focusable and copyable, but rejects edits. Keeps the normal appearance.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Show a clear button while there is text to clear.
    #[prop(optional, into)]
    cleanable: Signal<bool>,
    /// The HTML input type: `text`, `password`, `email`, `search`, and so on.
    #[prop(default = "text")]
    kind: &'static str,
    /// Text shown while there is no value.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Mark the field `aria-invalid` and draw the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// The `id` of the element that describes the field (a hint or an error message).
    #[prop(optional, into)]
    aria_describedby: Option<String>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Form field name; the value is submitted under it.
    #[prop(optional, into)]
    name: Option<String>,
    #[prop(optional, into)] autocomplete: Option<String>,
    /// For number inputs (`kind="number"`): the limits and step, as HTML attributes.
    #[prop(optional, into)]
    min: Option<String>,
    #[prop(optional, into)] max: Option<String>,
    #[prop(optional, into)] step: Option<String>,
    #[prop(optional, into)] prefix: Option<ViewFn>,
    #[prop(optional, into)] suffix: Option<ViewFn>,
    /// Extra classes for the frame, for layout (for example Tailwind utilities). They override the
    /// component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Extra classes for the native `<input>` itself, for example `font-mono`.
    #[prop(optional, into)]
    field_class: String,
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
    let field = NodeRef::<html::Input>::new();
    let flag = |s: Signal<bool>| move || s.get().then_some("");
    let show_clear =
        move || cleanable.get() && !disabled.get() && !readonly.get() && !value.get().is_empty();

    view! {
        <div
            class=format!("vdl-input {class}")
            data-size=move || size.get().as_str()
            data-disabled=flag(disabled)
        >
            {prefix.map(|p| view! { <span class="vdl-input__affix">{p.run()}</span> })}
            <input
                node_ref=field
                class=format!("vdl-input__field {field_class}")
                type=kind
                id=id
                name=name
                autocomplete=autocomplete
                min=min
                max=max
                step=step
                placeholder=placeholder
                aria-label=aria_label
                aria-invalid=move || invalid.get().then_some("true")
                aria-describedby=aria_describedby
                disabled=move || disabled.get()
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
            {move || {
                show_clear()
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="vdl-input__clear"
                                aria-label="Clear"
                                tabindex="-1"
                                on:click=move |_| {
                                    value.set(String::new());
                                    if let Some(el) = field.get() {
                                        let _ = el.focus();
                                    }
                                }
                            >
                                "×"
                            </button>
                        }
                    })
            }}
            {suffix.map(|s| view! { <span class="vdl-input__affix">{s.run()}</span> })}
        </div>
    }
}
