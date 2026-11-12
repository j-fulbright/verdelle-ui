use leptos::prelude::*;

/// A round tile at the end of an [`crate::AvatarGroup`] for the people not shown ("+3"). It
/// matches the size of the group's avatars.
#[component]
pub fn AvatarGroupCount(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("vdl-avatar-group__count {class}")>{children()}</div> }
}
