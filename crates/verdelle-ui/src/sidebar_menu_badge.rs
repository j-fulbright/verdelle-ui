use leptos::prelude::*;

/// A count at the end of a [`crate::SidebarMenuItem`] (unread, for example); it ignores the
/// pointer and is hidden while the sidebar is collapsed to icons.
#[component]
pub fn SidebarMenuBadge(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__menu-badge {class}") data-sidebar="menu-badge">
            {children()}
        </div>
    }
}
