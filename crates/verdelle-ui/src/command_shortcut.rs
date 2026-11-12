use leptos::prelude::*;

/// A key hint pushed to the end of a [`crate::CommandItem`] (`"⌘K"`). A label only: it does not
/// bind the keys.
#[component]
pub fn CommandShortcut(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <span class=format!("vdl-command__shortcut {class}")>{children()}</span> }
}
