use leptos::prelude::*;

/// A section of a sidebar with 8px padding: an optional [`crate::SidebarGroupLabel`] and a
/// [`crate::SidebarGroupContent`].
#[component]
pub fn SidebarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__group {class}") data-sidebar="group">
            {children()}
        </div>
    }
}
