use leptos::prelude::*;

/// Something placed at the top right of an [`crate::Alert`] (a button, a link), as shadcn's
/// `AlertAction`. The alert makes room for it on the right.
#[component]
pub fn AlertAction(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("vdl-alert__action {class}")>{children()}</div> }
}
