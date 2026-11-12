use crate::primitives::resizable as primitive;
use leptos::prelude::*;

/// The draggable divider between two panels, styled as shadcn/ui's `ResizableHandle`: a hairline
/// with a wider hit area. `with_handle` adds the small grip in the middle. It is a focusable
/// `role="separator"`: the arrow keys along the group's axis move it (Shift: ten steps), Home
/// and End send it to either end, and Enter collapses or opens a collapsible neighbour.
/// `disabled` fixes the two panels' boundary; `aria_label` names it (for example "Resize
/// sidebar").
#[component]
pub fn ResizableHandle(
    #[prop(optional)] with_handle: bool,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: Signal<bool>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::ResizableHandle
            disabled=disabled
            aria_label=aria_label
            class=format!("vdl-resizable-handle {class}")
        >
            {with_handle.then(|| view! { <div class="vdl-resizable-handle__grip"></div> })}
        </primitive::ResizableHandle>
    }
}
