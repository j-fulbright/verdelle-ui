use leptos::prelude::*;

/// Supporting text under a [`crate::PopoverTitle`].
#[component]
pub fn PopoverDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <p class=format!("vdl-popover__description {class}")>{children()}</p> }
}
