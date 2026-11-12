use leptos::prelude::*;

/// A drawer's visible title (`<h2>`).
#[component]
pub fn DrawerTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h2 class=format!("vdl-drawer__title {class}")>{children()}</h2> }
}
