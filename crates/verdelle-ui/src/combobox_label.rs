use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// The title of a [`crate::ComboboxGroup`]; it also names the group for assistive technology.
#[component]
pub fn ComboboxLabel(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxLabel attr:class=format!("vdl-combobox__label {class}")>
            {children.with_value(|children| children())}
        </primitive::ComboboxLabel>
    }
}
