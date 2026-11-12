use leptos::prelude::*;

/// A sheet's visible title (`<h2>`).
#[component]
pub fn SheetTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h2 class=format!("vdl-sheet__title {class}")>{children()}</h2> }
}
