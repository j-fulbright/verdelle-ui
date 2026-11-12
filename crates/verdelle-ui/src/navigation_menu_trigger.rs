use crate::icons::chevron_down;
use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// The button that opens its item's [`crate::NavigationMenuContent`]. A chevron after the
/// label turns upside down while the panel is open. `disabled` stops it opening.
#[component]
pub fn NavigationMenuTrigger(
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional)]
    disabled: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::NavigationMenuTrigger
            disabled=disabled
            attr:class=format!("vdl-nav__trigger {class}")
        >
            {children.with_value(|children| children())}
            <span class="vdl-nav__chevron" aria-hidden="true">{chevron_down()}</span>
        </primitive::NavigationMenuTrigger>
    }
}
