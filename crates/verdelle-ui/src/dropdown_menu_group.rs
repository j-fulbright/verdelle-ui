use crate::primitives::dropdown_menu as primitive;
use leptos::prelude::*;

/// Groups related items of a [`crate::DropdownMenuContent`] (`role="group"`), usually under
/// a [`crate::DropdownMenuLabel`].
#[component]
pub fn DropdownMenuGroup(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::DropdownMenuGroup attr:class=format!("vdl-menu__group {class}")>
            {children.with_value(|children| children())}
        </primitive::DropdownMenuGroup>
    }
}
