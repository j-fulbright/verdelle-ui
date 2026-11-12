use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// One entry of the bar: a [`crate::NavigationMenuTrigger`] with its
/// [`crate::NavigationMenuContent`], or a [`crate::NavigationMenuLink`].
///
/// `value` identifies the entry (for the menu's `value` and the motion between panels);
/// it is generated when omitted, which is fine for plain links.
#[component]
pub fn NavigationMenuItem(
    #[prop(into, optional)] value: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::NavigationMenuItem value=value attr:class=format!("vdl-nav__item {class}")>
            {children.with_value(|children| children())}
        </primitive::NavigationMenuItem>
    }
}
