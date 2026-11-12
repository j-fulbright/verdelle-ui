use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// A link in the bar or inside a panel. Selecting it closes the menu.
///
/// `active` marks the current page (`aria-current="page"` and a muted fill). With
/// `trigger_style` it looks like a [`crate::NavigationMenuTrigger`] (shadcn's
/// `navigationMenuTriggerStyle()`), which is how a plain link sits in the bar next to
/// triggers. Inside a panel, put a `vdl-nav__link-title` and `vdl-nav__link-description`
/// in it for the usual title-and-summary layout.
#[component]
pub fn NavigationMenuLink(
    /// Link target.
    #[prop(optional, into)]
    href: Option<String>,
    #[prop(into, optional)] active: MaybeProp<bool>,
    /// Style it as a bar item, like a trigger.
    #[prop(optional)]
    trigger_style: bool,
    /// Called when the item is chosen, by click or keyboard.
    #[prop(into, optional)]
    on_select: Option<Callback<()>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let base = if trigger_style {
        "vdl-nav__trigger vdl-nav__link"
    } else {
        "vdl-nav__link"
    };

    view! {
        <primitive::NavigationMenuLink
            active=active
            on_select=Callback::new(move |()| {
                if let Some(cb) = on_select {
                    cb.run(());
                }
            })
            attr:class=format!("{base} {class}")
            attr:href=href
        >
            {children.with_value(|children| children())}
        </primitive::NavigationMenuLink>
    }
}
