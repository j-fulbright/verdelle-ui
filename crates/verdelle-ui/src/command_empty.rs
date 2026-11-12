use crate::primitives::command as primitive;
use leptos::prelude::*;

/// What a [`crate::Command`] says when no item matches the search.
#[component]
pub fn CommandEmpty(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::CommandEmpty class=format!("vdl-command__empty {class}")>
            {children.with_value(|children| children())}
        </primitive::CommandEmpty>
    }
}
