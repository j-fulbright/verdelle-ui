use leptos::prelude::*;

/// The supporting text of an [`crate::Item`]: muted, clamped to two lines. Links inside
/// are underlined.
#[component]
pub fn ItemDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <p class=format!("vdl-item__description {class}")>{children()}</p> }
}
