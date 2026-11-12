use leptos::prelude::*;

/// A card's title. Renders an `<h3>`, so it takes part in the page's heading outline.
#[component]
pub fn CardTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h3 class=format!("vdl-card__title {class}")>{children()}</h3> }
}
