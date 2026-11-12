use crate::primitives::toast as primitive;
use leptos::prelude::*;

/// The heading line of a [`crate::Toast`].
#[component]
pub fn ToastTitle(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ToastTitle attr:class=format!("vdl-toast__title {class}")>
            {children.with_value(|children| children())}
        </primitive::ToastTitle>
    }
}
