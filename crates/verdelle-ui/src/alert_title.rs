use leptos::prelude::*;

/// The bold first line of an [`crate::Alert`].
#[component]
pub fn AlertTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-alert__title {class}")>{children()}</div> }
}
