use leptos::prelude::*;

/// The scrolling middle of a sidebar, holding its [`crate::SidebarGroup`]s. It scrolls without a
/// visible scrollbar, and does not scroll at all while the sidebar is collapsed to icons.
#[component]
pub fn SidebarContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__content {class}") data-sidebar="content">
            {children()}
        </div>
    }
}
