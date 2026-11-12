use crate::primitives::dropdown_menu as primitive;
use leptos::prelude::*;

/// A thin rule between groups of items in a [`crate::DropdownMenuContent`].
#[component]
pub fn DropdownMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitive::DropdownMenuSeparator attr:class=format!("vdl-menu__separator {class}") /> }
}
