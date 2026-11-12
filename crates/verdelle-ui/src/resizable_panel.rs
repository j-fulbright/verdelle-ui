use crate::primitives::resizable as primitive;
use leptos::prelude::*;

/// One panel of a [`crate::ResizablePanelGroup`], styled as shadcn/ui's `ResizablePanel`.
///
/// `default_size` is the starting size in percent (panels without one share what is left),
/// `min_size` and `max_size` limit it. A `collapsible` panel dragged (or stepped with the
/// keyboard) below `min_size` collapses to `collapsed_size` (default 0, hidden) and
/// `data-state` becomes `collapsed`; `collapsed` is a two-way signal for that state, so setting
/// it collapses or opens the panel. These are read once, when the panel is built.
#[component]
pub fn ResizablePanel(
    #[prop(optional)] default_size: Option<f64>,
    #[prop(default = 0.0)] min_size: f64,
    #[prop(default = 100.0)] max_size: f64,
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] collapsed_size: f64,
    #[prop(optional)] collapsed: Option<RwSignal<bool>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <primitive::ResizablePanel
            default_size=default_size
            min_size=min_size
            max_size=max_size
            collapsible=collapsible
            collapsed_size=collapsed_size
            collapsed=collapsed
            class=format!("vdl-resizable-panel {class}")
        >
            {children.map(|c| c())}
        </primitive::ResizablePanel>
    }
}
