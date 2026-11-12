use leptos::prelude::*;

/// One `<li>` of a [`crate::PaginationContent`].
#[component]
pub fn PaginationItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <li class=format!("vdl-pagination__item {class}")>{children()}</li> }
}
