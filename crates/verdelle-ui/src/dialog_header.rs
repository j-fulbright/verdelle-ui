use leptos::prelude::*;

/// Groups a dialog's title and description.
#[component]
pub fn DialogHeader(children: Children) -> impl IntoView {
    view! { <div class="vdl-dialog__header">{children()}</div> }
}
