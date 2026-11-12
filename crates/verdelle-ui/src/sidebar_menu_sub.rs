use leptos::prelude::*;

/// A nested list under a [`crate::SidebarMenuItem`], drawn with a line down its start and hidden
/// while the sidebar is collapsed to icons. Holds [`crate::SidebarMenuSubItem`]s.
#[component]
pub fn SidebarMenuSub(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <ul class=format!("vdl-sidebar__menu-sub {class}") data-sidebar="menu-sub">
            {children()}
        </ul>
    }
}
