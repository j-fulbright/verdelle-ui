use leptos::prelude::*;

/// A dialog's visible title (`<h2>`).
#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    view! { <h2 class="vdl-dialog__title">{children()}</h2> }
}
