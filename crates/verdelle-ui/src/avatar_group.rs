use leptos::prelude::*;

/// Overlaps avatars in a row, each ringed with the page background. Put an
/// [`crate::AvatarGroupCount`] last for "+3". Avatars should share a size.
#[component]
pub fn AvatarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-avatar-group {class}")>{children()}</div> }
}
