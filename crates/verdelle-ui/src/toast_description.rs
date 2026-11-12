use crate::primitives::toast as primitive;
use leptos::prelude::*;

/// The supporting text of a [`crate::Toast`], in the muted colour.
#[component]
pub fn ToastDescription(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ToastDescription attr:class=format!("vdl-toast__description {class}")>
            {children.with_value(|children| children())}
        </primitive::ToastDescription>
    }
}
