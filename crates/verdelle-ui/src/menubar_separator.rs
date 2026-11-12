use crate::primitives::menubar as primitive;
use leptos::prelude::*;

/// A thin rule between groups of items in a [`crate::MenubarContent`].
#[component]
pub fn MenubarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitive::MenubarSeparator attr:class=format!("vdl-menu__separator {class}") /> }
}
