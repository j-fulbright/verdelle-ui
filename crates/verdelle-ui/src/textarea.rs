use crate::{Size, text_field};
use leptos::{ev, html, prelude::*};

/// A multi-line text field, styled as shadcn/ui's Textarea.
///
/// The same frame as [`crate::Input`], around a native `<textarea>`: at least 64px
/// tall, growing with its content (where the browser supports `field-sizing`) and
/// resizable vertically.
///
/// `grow` makes the growth certain and hides the resize handle: the field is as tall as its text
/// in every browser (a script measures it where `field-sizing` is missing), up to `max_rows`
/// lines, after which it scrolls. `max_rows` also caps a field that is not `grow`.
///
/// `on_input` runs on every edit with the new text, `on_change` when the browser commits it (on
/// blur), `on_keydown` on every key, `on_enter` on Enter without Shift (not while an IME is
/// composing; it does not stop the new line, so call `prevent_default` in `on_keydown` for a
/// chat-style "Enter sends"), and `on_paste` on a paste. `class` goes on the frame, `field_class` on
/// the native `<textarea>`.
#[component]
pub fn Textarea(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Read-only: focusable and selectable, but the value cannot be changed.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    #[prop(default = 3)] rows: u32,
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
    /// Grow with the text and hide the resize handle. See the component docs.
    #[prop(optional)]
    grow: bool,
    /// The tallest the field gets, in lines; past it the text scrolls.
    #[prop(optional)]
    max_rows: Option<u32>,
    /// Extra classes for the frame, for layout (for example Tailwind utilities). They override the
    /// component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Extra classes for the native `<textarea>` itself, for example `font-mono`.
    #[prop(optional, into)]
    field_class: String,
    /// Called on every edit with the new text.
    #[prop(optional, into)]
    on_input: Option<Callback<String>>,
    /// Called when the browser commits the edit (on blur), with the text.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Called on every key press in the field.
    #[prop(optional, into)]
    on_keydown: Option<Callback<ev::KeyboardEvent>>,
    /// Called when Enter is pressed without Shift (not while an IME is composing).
    #[prop(optional, into)]
    on_enter: Option<Callback<()>>,
    /// Called on a paste, before the browser inserts the text.
    #[prop(optional, into)]
    on_paste: Option<Callback<ev::ClipboardEvent>>,
) -> impl IntoView {
    let flag = |s: Signal<bool>| move || s.get().then_some("");
    let field = NodeRef::<html::Textarea>::new();
    text_field::use_grow_fallback(field, value.into(), grow);

    view! {
        <div
            class=format!("vdl-input {class}")
            data-multiline=""
            data-size=move || size.get().as_str()
            data-disabled=flag(disabled)
        >
            <textarea
                node_ref=field
                class=format!("vdl-input__field {field_class}")
                id=id
                name=name
                rows=rows
                data-grow=grow.then_some("")
                data-max-rows=max_rows.is_some().then_some("")
                style=text_field::max_rows_style(max_rows)
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
                on:keydown=move |ev| text_field::handle_keydown(ev, on_keydown, on_enter, true)
                on:paste=move |ev| {
                    if let Some(cb) = on_paste {
                        cb.run(ev);
                    }
                }
            ></textarea>
        </div>
    }
}
