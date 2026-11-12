use crate::primitives::navigation_menu as primitive;
use leptos::prelude::*;

/// A small arrow under the bar that slides to the open trigger and points at its panel.
/// Put it right after the [`crate::NavigationMenuList`]; it only makes sense with a
/// viewport (shadcn's Radix version drew it the same way).
#[component]
pub fn NavigationMenuIndicator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <primitive::NavigationMenuIndicator class=format!("vdl-nav__indicator {class}")>
            <div class="vdl-nav__indicator-arrow"></div>
        </primitive::NavigationMenuIndicator>
    }
}
