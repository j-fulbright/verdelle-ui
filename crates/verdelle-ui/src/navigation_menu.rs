use crate::Orientation;
use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// A navigation bar, styled as shadcn/ui's Navigation Menu: a row of links and triggers,
/// where a trigger opens a panel of links under it. Built on the headless
/// Navigation Menu.
///
/// Compose with [`crate::NavigationMenuList`], [`crate::NavigationMenuItem`],
/// [`crate::NavigationMenuTrigger`] + [`crate::NavigationMenuContent`] (a panel) or
/// [`crate::NavigationMenuLink`] (a plain link), and optionally a
/// [`crate::NavigationMenuIndicator`] after the list.
///
/// Items are identified by `value`. Pointer: a panel opens when the pointer rests on its
/// trigger for `delay_duration` ms (default 200), and for `skip_delay_duration` ms (default
/// 300) after one closes the next opens at once. The pointer may cross onto the panel;
/// leaving both closes it. A click toggles; Escape closes and returns focus to the trigger.
/// Keyboard: Left/Right (Up/Down when vertical), Home and End move along the bar; Down
/// (Right when vertical) opens a panel and moves into it, where Up/Down move between links.
///
/// With `viewport` (the default) all panels share one box under the bar that resizes and
/// slides between them and keeps clear of the window edge; without it each panel hangs
/// under its own trigger. Uncontrolled by default; pass `value` (and `on_value_change`,
/// which gets an empty string on close) to control it.
#[component]
pub fn NavigationMenu(
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the open item's value, or an empty string when the menu closes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] delay_duration: MaybeProp<i32>,
    #[prop(into, optional)] skip_delay_duration: MaybeProp<i32>,
    #[prop(optional)] orientation: Orientation,
    /// Share one resizing panel for all items. On by default.
    #[prop(default = true)]
    viewport: bool,
    /// Accessible name of the navigation landmark. Default "Main".
    #[prop(into, optional)]
    label: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let on_value_change = Callback::new(move |v: String| {
        if let Some(cb) = on_value_change {
            cb.run(v);
        }
    });

    view! {
        <primitive::NavigationMenu
            value=value
            default_value=default_value
            on_value_change=on_value_change
            delay_duration=delay_duration
            skip_delay_duration=skip_delay_duration
            orientation=orientation.to_string()
            label=label
            attr:class=format!("vdl-nav {class}")
            attr:data-viewport=viewport.to_string()
        >
            {children.with_value(|children| children())}
            {viewport
                .then(|| {
                    view! {
                        <primitive::NavigationMenuViewport
                            class="vdl-nav__positioner"
                            viewport_class="vdl-nav__viewport"
                        />
                    }
                })}
        </primitive::NavigationMenu>
    }
}
