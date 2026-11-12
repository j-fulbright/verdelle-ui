use leptos::prelude::*;

/// A key-combination hint at the end of a [`crate::ContextMenuItem`], such as `⌘K`.
/// It is only a label: it does not bind the keys.
#[component]
pub fn ContextMenuShortcut(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <span class=format!("vdl-menu__shortcut {class}")>{children()}</span> }
}
