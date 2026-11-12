use leptos::prelude::*;

/// The heading of an [`crate::Empty`] (18px, medium weight). A `<div>`, as in shadcn.
#[component]
pub fn EmptyTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-empty__title {class}")>{children()}</div> }
}
