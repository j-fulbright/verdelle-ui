use crate::input_group::InputGroupContext;
use crate::text_field;
use leptos::{ev, html, prelude::*};

/// The multi-line field of an [`crate::InputGroup`]: a bare native `<textarea>` at least 64px
/// tall that grows with its content (where the browser supports `field-sizing`); it does not
/// resize by hand. `value` is two-way. Pair it with a block addon for a footer or a header.
///
/// `grow` makes the growth certain (a script measures the text where `field-sizing` is missing)
/// and `max_rows` caps the height in lines, after which the text scrolls. `on_input`, `on_change`,
/// `on_keydown`, `on_enter` (Enter without Shift, not while an IME is composing) and `on_paste`
/// work as on [`crate::Textarea`].
#[component]
pub fn InputGroupTextarea(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    #[prop(default = 3)] rows: u32,
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
    /// Grow with the text. See the component docs.
    #[prop(optional)]
    grow: bool,
    /// The tallest the field gets, in lines; past it the text scrolls.
    #[prop(optional)]
    max_rows: Option<u32>,
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
    let group = use_context::<InputGroupContext>();
    let group_disabled = move || group.is_some_and(|g| g.disabled.get());
    let invalid = move || group.is_some_and(|g| g.invalid.get());
    let field = NodeRef::<html::Textarea>::new();
    text_field::use_grow_fallback(field, value.into(), grow);
    view! {
        <textarea
            node_ref=field
            class=format!("vdl-input-group__control vdl-input-group__textarea {class}")
            rows=rows
            id=id
            name=name
            data-grow=grow.then_some("")
            data-max-rows=max_rows.is_some().then_some("")
            style=text_field::max_rows_style(max_rows)
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
            on:keydown=move |ev| text_field::handle_keydown(ev, on_keydown, on_enter, true)
            on:paste=move |ev| {
                if let Some(cb) = on_paste {
                    cb.run(ev);
                }
            }
        ></textarea>
    }
}
