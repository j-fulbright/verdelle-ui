use leptos::prelude::*;

/// The supporting text of an [`crate::Empty`]. Links inside it are underlined.
#[component]
pub fn EmptyDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("vdl-empty__description {class}")>{children()}</div> }
}
