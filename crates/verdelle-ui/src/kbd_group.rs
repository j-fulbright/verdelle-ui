use leptos::prelude::*;

/// Lays out several [`crate::Kbd`] keys in a row (for example `Ctrl` `+` `K`).
/// Like shadcn's, it renders a `<kbd>` so the whole shortcut is one element.
#[component]
pub fn KbdGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <kbd class=format!("vdl-kbd-group {class}")>{children()}</kbd> }
}
