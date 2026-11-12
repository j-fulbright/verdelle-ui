use leptos::prelude::*;

/// Groups an alert dialog's media, title and description.
#[component]
pub fn AlertDialogHeader(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("vdl-alert-dialog__header {class}")>{children()}</div> }
}
