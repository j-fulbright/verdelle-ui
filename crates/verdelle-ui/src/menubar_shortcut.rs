use leptos::prelude::*;

/// A key-combination hint at the end of a [`crate::MenubarItem`], such as `⌘K`.
/// It is only a label: it does not bind the keys.
#[component]
pub fn MenubarShortcut(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <span class=format!("vdl-menu__shortcut {class}")>{children()}</span> }
}
