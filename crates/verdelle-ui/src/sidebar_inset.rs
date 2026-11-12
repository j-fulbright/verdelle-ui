use leptos::prelude::*;

/// The page content next to the sidebar (`<main>`), filling the rest of the row. With a
/// `variant=Inset` sidebar before it, it becomes a rounded card with a shadow and a margin.
#[component]
pub fn SidebarInset(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <main class=format!("vdl-sidebar-inset {class}")>{children()}</main> }
}
