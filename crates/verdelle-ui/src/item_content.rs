use leptos::prelude::*;

/// The text column of an [`crate::Item`]: it takes the free width. A second `ItemContent`
/// directly after the first does not grow (for a trailing value).
#[component]
pub fn ItemContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-item__content {class}")>{children()}</div> }
}
