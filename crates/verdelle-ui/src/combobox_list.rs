use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// The scrolling list of options inside a [`crate::ComboboxContent`] (`role="listbox"`). It
/// scrolls when taller than the popup's room, up to 288px.
#[component]
pub fn ComboboxList(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxList attr:class=format!("vdl-combobox__list {class}")>
            {children.with_value(|children| children())}
        </primitive::ComboboxList>
    }
}
