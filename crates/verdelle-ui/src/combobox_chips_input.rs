use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// The search field inside [`crate::ComboboxChips`]: bare, filling the row after the chips.
#[component]
pub fn ComboboxChipsInput(
    /// Text shown while there is no value.
    #[prop(into, optional)]
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
    view! {
        <primitive::ComboboxInput
            attr:class=format!("vdl-combobox__chips-input {class}")
            attr:placeholder=placeholder
            attr:aria-label=aria_label
            attr:id=id
        />
    }
}
