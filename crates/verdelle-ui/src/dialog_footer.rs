use leptos::prelude::*;

/// The action row of a dialog, right-aligned.
#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! { <div class="vdl-dialog__footer">{children()}</div> }
}
