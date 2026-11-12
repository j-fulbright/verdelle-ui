use crate::primitives::select as primitive;
use leptos::prelude::*;

/// The heading of a [`crate::SelectGroup`]. It names the group for assistive tech, so it must
/// be inside one.
#[component]
pub fn SelectLabel(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::SelectLabel attr:class=format!("vdl-select__label {class}")>
            {children.with_value(|children| children())}
        </primitive::SelectLabel>
    }
}
