use leptos::prelude::*;

/// Groups a sheet's title and description, with 16px padding.
#[component]
pub fn SheetHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-sheet__header {class}")>{children()}</div> }
}
