use crate::Separator;
use leptos::prelude::*;

/// A horizontal rule between the items of an [`crate::ItemGroup`] (a [`crate::Separator`]
/// with 8px above and below).
#[component]
pub fn ItemSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <Separator class=format!("vdl-item__separator {class}") /> }
}
