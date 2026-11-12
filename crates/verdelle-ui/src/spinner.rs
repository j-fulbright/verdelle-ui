use leptos::prelude::*;

/// A spinning ring for loading states. It is as large as the surrounding text (`1em`),
/// takes the text colour, and announces itself as "Loading" to assistive technology.
#[component]
pub fn Spinner(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <span class=format!("vdl-spinner {class}") role="status" aria-label="Loading"></span> }
}
