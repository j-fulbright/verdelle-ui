use leptos::prelude::*;

/// A list of [`crate::SidebarMenuItem`]s (`<ul>`), 4px apart.
#[component]
pub fn SidebarMenu(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <ul class=format!("vdl-sidebar__menu {class}") data-sidebar="menu">
            {children()}
        </ul>
    }
}
