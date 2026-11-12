use leptos::prelude::*;

/// Muted text (a unit, a scheme such as `https://`, a character count) inside an
/// [`crate::InputGroupAddon`].
#[component]
pub fn InputGroupText(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <span class=format!("vdl-input-group__text {class}")>{children()}</span> }
}
