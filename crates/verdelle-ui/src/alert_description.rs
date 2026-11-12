use leptos::prelude::*;

/// The body text of an [`crate::Alert`].
#[component]
pub fn AlertDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("vdl-alert__description {class}")>{children()}</div> }
}
