use leptos::prelude::*;

/// A small heading inside a [`crate::MenubarContent`] or [`crate::MenubarGroup`].
/// `inset` indents it to line up with items that have a check or icon space.
#[component]
pub fn MenubarLabel(
    /// Indents it to line up with sibling items that have an icon or check indicator.
    #[prop(optional)]
    inset: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-menu__label {class}") data-inset=inset.then_some("")>
            {children()}
        </div>
    }
}
