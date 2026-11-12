use crate::primitives::select as primitive;
use leptos::prelude::*;

/// A set of related items. Start it with a [`crate::SelectLabel`] to name it.
#[component]
pub fn SelectGroup(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::SelectGroup attr:class=format!("vdl-select__group {class}")>
            {children.with_value(|children| children())}
        </primitive::SelectGroup>
    }
}
