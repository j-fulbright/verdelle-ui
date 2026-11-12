use leptos::prelude::*;

/// The actions of an [`crate::Empty`] (buttons, an input), centred below the header.
#[component]
pub fn EmptyContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-empty__content {class}")>{children()}</div> }
}
