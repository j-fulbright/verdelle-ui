use crate::primitives::context_menu as primitive;
use leptos::prelude::*;

/// Groups related items of a [`crate::ContextMenuContent`] (`role="group"`), usually under
/// a [`crate::ContextMenuLabel`].
#[component]
pub fn ContextMenuGroup(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ContextMenuGroup attr:class=format!("vdl-menu__group {class}")>
            {children.with_value(|children| children())}
        </primitive::ContextMenuGroup>
    }
}
