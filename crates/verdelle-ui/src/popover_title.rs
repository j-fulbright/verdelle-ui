use leptos::prelude::*;

/// The heading of a [`crate::PopoverContent`].
#[component]
pub fn PopoverTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-popover__title {class}")>{children()}</div> }
}
