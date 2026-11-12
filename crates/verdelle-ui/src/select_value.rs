use crate::primitives::select as primitive;
use leptos::prelude::*;

/// The selected item's text, or `placeholder` while nothing is selected.
#[component]
pub fn SelectValue(
    /// Text shown while there is no value.
    #[prop(into, optional)]
    placeholder: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::SelectValue
            placeholder=placeholder
            attr:class=format!("vdl-select__value {class}")
        />
    }
}
