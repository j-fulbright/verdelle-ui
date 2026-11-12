use leptos::prelude::*;

/// One `<li>` of a [`crate::BreadcrumbList`].
#[component]
pub fn BreadcrumbItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <li class=format!("vdl-breadcrumb__item {class}")>{children()}</li> }
}
