use leptos::prelude::*;

/// A full-width row above the content of an [`crate::Item`].
#[component]
pub fn ItemHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-item__header {class}")>{children()}</div> }
}
