use leptos::prelude::*;

/// Groups a drawer's title and description, with 16px padding. Centred for a top or bottom
/// drawer on small screens, as shadcn's.
#[component]
pub fn DrawerHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-drawer__header {class}")>{children()}</div> }
}
