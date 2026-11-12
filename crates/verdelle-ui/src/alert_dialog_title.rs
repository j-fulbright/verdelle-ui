use crate::alert_dialog::AlertDialogContext;
use leptos::prelude::*;

/// An alert dialog's title (`<h2>`), which names the dialog.
#[component]
pub fn AlertDialogTitle(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let id = expect_context::<AlertDialogContext>().title_id;
    view! { <h2 id=id class=format!("vdl-alert-dialog__title {class}")>{children()}</h2> }
}
