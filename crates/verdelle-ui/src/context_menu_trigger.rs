use crate::primitives::context_menu as primitive;
use leptos::prelude::*;

/// The area that opens a [`crate::ContextMenu`]: right-click, the context-menu key,
/// Shift+F10, or a touch held for 700ms. It is a block-level `<div>` with text selection
/// switched off (as shadcn does); give it its size and look with `class`. A `disabled`
/// trigger leaves the browser's own context menu alone.
#[component]
pub fn ContextMenuTrigger(
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ContextMenuTrigger
            disabled=disabled
            attr:class=format!("vdl-context-menu__trigger {class}")
        >
            {children.with_value(|children| children())}
        </primitive::ContextMenuTrigger>
    }
}
