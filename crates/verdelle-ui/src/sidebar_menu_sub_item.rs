use leptos::prelude::*;

/// One row of a [`crate::SidebarMenuSub`] (`<li>`) holding a [`crate::SidebarMenuSubButton`].
#[component]
pub fn SidebarMenuSubItem(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <li class=format!("vdl-sidebar__menu-sub-item {class}") data-sidebar="menu-sub-item">
            {children()}
        </li>
    }
}
