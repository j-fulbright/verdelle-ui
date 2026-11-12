use crate::primitives::context_menu as primitive;
use leptos::prelude::*;

/// A thin rule between groups of items in a [`crate::ContextMenuContent`].
#[component]
pub fn ContextMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitive::ContextMenuSeparator attr:class=format!("vdl-menu__separator {class}") /> }
}
