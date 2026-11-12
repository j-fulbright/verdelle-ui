use crate::primitives::select as primitive;
use leptos::prelude::*;

/// A hairline between groups of items.
#[component]
pub fn SelectSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <primitive::SelectSeparator attr:class=format!("vdl-select__separator {class}") /> }
}
