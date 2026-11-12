use crate::Orientation;
use leptos::prelude::*;

/// Joins buttons (and inputs, selects and [`crate::ButtonGroupText`]) into one segmented
/// control, styled as shadcn/ui's Button Group: the inner corners square off and the inner
/// borders merge. Direct children are what is joined. Nest a group inside a group for
/// separate clusters, 8px apart.
///
/// `Horizontal` is the default; `Vertical` stacks them. An input child takes the free width.
/// Give the group an accessible name with `aria_label` when its purpose is not obvious.
#[component]
pub fn ButtonGroup(
    #[prop(optional, into)] orientation: Signal<Orientation>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("vdl-button-group {class}")
            role="group"
            data-orientation=move || orientation.get().to_string()
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
