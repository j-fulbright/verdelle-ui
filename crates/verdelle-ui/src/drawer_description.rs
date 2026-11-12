use leptos::prelude::*;

/// Supporting text under a drawer's title, in the muted colour.
#[component]
pub fn DrawerDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <p class=format!("vdl-drawer__description {class}")>{children()}</p> }
}
