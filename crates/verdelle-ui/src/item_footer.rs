use leptos::prelude::*;

/// A full-width row below the content of an [`crate::Item`].
#[component]
pub fn ItemFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-item__footer {class}")>{children()}</div> }
}
