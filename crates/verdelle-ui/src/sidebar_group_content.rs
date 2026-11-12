use leptos::prelude::*;

/// The body of a [`crate::SidebarGroup`], 14px text, usually a [`crate::SidebarMenu`].
#[component]
pub fn SidebarGroupContent(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__group-content {class}") data-sidebar="group-content">
            {children()}
        </div>
    }
}
