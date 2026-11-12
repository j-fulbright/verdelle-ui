use crate::{Orientation, Separator};
use leptos::prelude::*;

/// A divider between buttons of a [`crate::ButtonGroup`]. Unlike [`crate::Separator`] it
/// defaults to `Vertical` (a horizontal group is the common case), uses the input border
/// colour and stretches to the group's height. Useful between filled buttons, which have
/// no border of their own.
#[component]
pub fn ButtonGroupSeparator(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! { <Separator orientation=orientation class=format!("vdl-button-group__separator {class}") /> }
}
