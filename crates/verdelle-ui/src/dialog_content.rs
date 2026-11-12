use leptos::prelude::*;

/// The body of a dialog. It takes the remaining
/// height and scrolls when the content is taller than the dialog.
#[component]
pub fn DialogContent(children: Children) -> impl IntoView {
    view! { <div class="vdl-dialog__content">{children()}</div> }
}
