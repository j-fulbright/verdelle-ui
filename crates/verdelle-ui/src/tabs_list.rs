use crate::primitives::tabs as primitive;
use leptos::prelude::*;

/// The row of [`crate::TabsTrigger`]s. It is only as wide as its tabs; give it a width
/// through `class` (for example `w-full`) to stretch it.
#[component]
pub fn TabsList(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::TabsList attr:class=format!("vdl-tabs__list {class}")>
            {children.with_value(|children| children())}
        </primitive::TabsList>
    }
}
