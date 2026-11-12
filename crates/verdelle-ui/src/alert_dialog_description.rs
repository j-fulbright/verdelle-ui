use crate::alert_dialog::AlertDialogContext;
use leptos::prelude::*;

/// Muted text that describes the decision; the dialog's accessible description.
#[component]
pub fn AlertDialogDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let id = expect_context::<AlertDialogContext>().description_id;
    view! { <p id=id class=format!("vdl-alert-dialog__description {class}")>{children()}</p> }
}
