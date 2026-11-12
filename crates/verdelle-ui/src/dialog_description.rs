use leptos::prelude::*;

/// Muted supporting text under a dialog's title.
#[component]
pub fn DialogDescription(children: Children) -> impl IntoView {
    view! { <p class="vdl-dialog__description">{children()}</p> }
}
