use leptos::prelude::*;

/// The centred stack of media, title and description inside an [`crate::Empty`].
#[component]
pub fn EmptyHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-empty__header {class}")>{children()}</div> }
}
