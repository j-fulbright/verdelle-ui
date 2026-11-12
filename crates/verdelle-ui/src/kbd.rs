use leptos::prelude::*;

/// A keyboard key or shortcut, styled as shadcn/ui's Kbd. Renders a `<kbd>`.
/// Group several keys with [`crate::KbdGroup`].
#[component]
pub fn Kbd(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <kbd class=format!("vdl-kbd {class}")>{children()}</kbd> }
}
