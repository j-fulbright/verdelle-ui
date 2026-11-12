use leptos::prelude::*;

/// The small heading of a [`crate::SidebarGroup`]. It fades out and slides up while the sidebar
/// is collapsed to icons. shadcn lets it render as another element (a collapsible's trigger);
/// here it is a plain `<div>`.
#[component]
pub fn SidebarGroupLabel(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__group-label {class}") data-sidebar="group-label">
            {children()}
        </div>
    }
}
