use crate::primitives::resizable as primitive;
use leptos::prelude::*;

use crate::Orientation;

/// A row or column of resizable panels, styled as shadcn/ui's `ResizablePanelGroup`. Give it
/// a size (it is `h-full w-full`, so a parent with a height works, or add a `class`). Put
/// [`crate::ResizablePanel`]s in it with a [`crate::ResizableHandle`] between each pair;
/// panels and handles are fixed when the group is built.
///
/// `orientation` is the layout: `Horizontal` (the default) puts panels side by side,
/// `Vertical` stacks them. `sizes` is an optional two-way signal with the layout in percent
/// (one entry per panel, adding up to 100): start it with a saved layout to restore it, or
/// empty to use the panels' `default_size`. `on_layout_change` runs on every change and
/// `on_layout_end` when a drag or key press finishes (save the layout there; there is no
/// built-in storage). `keyboard_step` is the percent an arrow key moves a handle (Shift: ten
/// times), `aria_label` names the group.
#[component]
pub fn ResizablePanelGroup(
    #[prop(into, optional)] orientation: Signal<Orientation>,
    #[prop(optional)] sizes: Option<RwSignal<Vec<f64>>>,
    #[prop(optional, into)] on_layout_change: Option<Callback<Vec<f64>>>,
    #[prop(optional, into)] on_layout_end: Option<Callback<Vec<f64>>>,
    #[prop(default = 1.0)] keyboard_step: f64,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <primitive::ResizablePanelGroup
            orientation=orientation
            sizes=sizes
            on_layout_change=on_layout_change
            on_layout_end=on_layout_end
            keyboard_step=keyboard_step
            aria_label=aria_label
            class=format!("vdl-resizable-group {class}")
        >
            {children()}
        </primitive::ResizablePanelGroup>
    }
}
