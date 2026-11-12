use crate::Skeleton;
use leptos::prelude::*;

/// A placeholder menu row for loading states: an optional icon square and a text bar, `width`
/// percent of the row (70 by default). shadcn picks a random width; a fixed one keeps the
/// server and browser renders identical, so vary it yourself across rows.
#[component]
pub fn SidebarMenuSkeleton(
    #[prop(optional)] show_icon: bool,
    #[prop(default = 70)] width: u8,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <div
            class=format!("vdl-sidebar__menu-skeleton {class}")
            data-sidebar="menu-skeleton"
            style=format!("--skeleton-width:{width}%")
        >
            {show_icon.then(|| view! { <Skeleton class="vdl-sidebar__menu-skeleton-icon" /> })}
            <Skeleton class="vdl-sidebar__menu-skeleton-text" />
        </div>
    }
}
