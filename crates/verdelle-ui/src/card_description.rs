use leptos::prelude::*;

/// Muted supporting text under a [`crate::CardTitle`].
#[component]
pub fn CardDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <p class=format!("vdl-card__description {class}")>{children()}</p> }
}
