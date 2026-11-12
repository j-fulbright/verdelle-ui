use leptos::prelude::*;

/// The `<ol>` of a [`crate::Breadcrumb`]: muted 14px text that wraps onto more lines when
/// it runs out of room.
#[component]
pub fn BreadcrumbList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <ol class=format!("vdl-breadcrumb__list {class}")>{children()}</ol> }
}
