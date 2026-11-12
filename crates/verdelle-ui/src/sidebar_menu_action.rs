use leptos::{ev, prelude::*};

/// A small icon button at the end of a [`crate::SidebarMenuItem`], next to its button (a "more"
/// menu, say); hidden while the sidebar is collapsed to icons. With `show_on_hover` it is
/// invisible until the row is hovered or focused (always visible on small screens).
/// Give it an `aria_label`.
#[component]
pub fn SidebarMenuAction(
    #[prop(optional)] show_on_hover: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=format!("vdl-sidebar__menu-action {class}")
            data-sidebar="menu-action"
            data-show-on-hover=show_on_hover.then_some("")
            aria-label=aria_label
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
