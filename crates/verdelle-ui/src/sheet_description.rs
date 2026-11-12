use leptos::prelude::*;

/// Supporting text under a sheet's title, in the muted colour.
#[component]
pub fn SheetDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <p class=format!("vdl-sheet__description {class}")>{children()}</p> }
}
