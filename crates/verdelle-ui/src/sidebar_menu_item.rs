use leptos::prelude::*;

/// One row of a [`crate::SidebarMenu`] (`<li>`): a [`crate::SidebarMenuButton`], and optionally a
/// [`crate::SidebarMenuAction`], a [`crate::SidebarMenuBadge`] and a [`crate::SidebarMenuSub`].
#[component]
pub fn SidebarMenuItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <li class=format!("vdl-sidebar__menu-item {class}") data-sidebar="menu-item">
            {children()}
        </li>
    }
}
