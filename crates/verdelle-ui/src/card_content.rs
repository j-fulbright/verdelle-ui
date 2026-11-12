use leptos::prelude::*;

/// The main body of a [`crate::Card`].
#[component]
pub fn CardContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-card__content {class}")>{children()}</div> }
}
