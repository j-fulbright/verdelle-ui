use crate::primitives::toolbar as primitive;
use leptos::prelude::*;

/// A divider between groups of a [`crate::Toolbar`]; it runs across the toolbar's axis.
#[component]
pub fn ToolbarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitive::ToolbarSeparator attr:class=format!("vdl-separator {class}") /> }
}
