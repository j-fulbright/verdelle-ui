use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// Options under one [`crate::ComboboxLabel`]. It hides itself while none of its options match.
#[component]
pub fn ComboboxGroup(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxGroup attr:class=format!("vdl-combobox__group {class}")>
            {children.with_value(|children| children())}
        </primitive::ComboboxGroup>
    }
}
