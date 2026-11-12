use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// The row (or column, when the menu is vertical) of [`crate::NavigationMenuItem`]s.
#[component]
pub fn NavigationMenuList(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::NavigationMenuList attr:class=format!("vdl-nav__list {class}")>
            {children.with_value(|children| children())}
        </primitive::NavigationMenuList>
    }
}
