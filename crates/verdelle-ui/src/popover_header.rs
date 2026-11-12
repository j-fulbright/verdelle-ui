use leptos::prelude::*;

/// A group for the title and description at the top of a [`crate::PopoverContent`].
#[component]
pub fn PopoverHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-popover__header {class}")>{children()}</div> }
}
