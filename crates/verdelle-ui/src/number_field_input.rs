use crate::number_field::NumberFieldStyle;
use crate::primitives::number_field as primitive;
use leptos::prelude::*;

/// The text field of a [`crate::NumberField`] (`role="spinbutton"`). Typing accepts digits, one
/// `.` and a leading sign; blur and Enter settle the value (clamped and rounded). Up and Down
/// step; Shift or PageUp/PageDown take the large step, Alt the small one; Home and End go to
/// `min` and `max` when set. Name it with `aria_label` or a `<label for>` pointing at `id`.
#[component]
pub fn NumberFieldInput(
    /// Text shown while there is no value.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let style = use_context::<NumberFieldStyle>();
    let invalid = move || style.is_some_and(|s| s.invalid.get());
    view! {
        <primitive::NumberFieldInput
            attr:class=format!("vdl-number-field__input {class}")
            attr:id=id
            attr:placeholder=placeholder
            attr:aria-label=aria_label
            attr:aria-invalid=move || invalid().then_some("true")
        />
    }
}
