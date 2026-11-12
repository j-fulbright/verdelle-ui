use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// The panel of links an item's [`crate::NavigationMenuTrigger`] opens.
///
/// In a menu with a viewport it is shown inside the shared viewport and slides in from the
/// side of the previous panel when you move between items (`data-motion` is
/// `from-start`, `from-end`, `to-start` or `to-end`). Without a viewport it hangs under
/// its item as a popover-styled box.
#[component]
pub fn NavigationMenuContent(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::NavigationMenuContent class=format!("vdl-nav__content {class}")>
            {children.with_value(|children| children())}
        </primitive::NavigationMenuContent>
    }
}
