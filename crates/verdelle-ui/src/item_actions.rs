use leptos::prelude::*;

/// The trailing buttons of an [`crate::Item`].
#[component]
pub fn ItemActions(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-item__actions {class}")>{children()}</div> }
}
