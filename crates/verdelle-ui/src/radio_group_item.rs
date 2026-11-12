use crate::primitives::radio_group as primitive;
use leptos::prelude::*;

/// One radio button of a [`crate::RadioGroup`]. `value` is what the group's value
/// becomes when it is chosen. Name it with a [`crate::Label`] pointing at its `id`,
/// or with `aria_label`.
#[component]
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::RadioGroupItem
            value=value
            disabled=disabled
            attr:id=id
            attr:aria-label=aria_label
            attr:class=format!("vdl-radio-group__item {class}")
        >
            <primitive::RadioGroupIndicator attr:class="vdl-radio-group__indicator" />
        </primitive::RadioGroupItem>
    }
}
