use leptos::prelude::*;

/// A small dot at the bottom-right corner of an [`crate::Avatar`] (a status marker). It is
/// 8, 10 or 12px by the avatar's size; an inline icon inside shows from the default size
/// up. Give it an accessible name with `aria_label` if the status matters.
#[component]
pub fn AvatarBadge(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <span
            class=format!("vdl-avatar__badge {class}")
            role=aria_label.is_some().then_some("img")
            aria-label=aria_label
        >
            {children.map(|c| c())}
        </span>
    }
}
