use leptos::{ev, prelude::*};

/// A small icon button at the top-right of a [`crate::SidebarGroup`] (an "add" button, say),
/// hidden while the sidebar is collapsed to icons. Give it an `aria_label`.
#[component]
pub fn SidebarGroupAction(
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
            class=format!("vdl-sidebar__group-action {class}")
            data-sidebar="group-action"
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
