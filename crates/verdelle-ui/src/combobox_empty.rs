use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// What the popup says when no option matches what has been typed. Put it in the
/// [`crate::ComboboxContent`], next to the [`crate::ComboboxList`].
#[component]
pub fn ComboboxEmpty(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxEmpty class=format!("vdl-combobox__empty {class}")>
            {children.with_value(|children| children())}
        </primitive::ComboboxEmpty>
    }
}
