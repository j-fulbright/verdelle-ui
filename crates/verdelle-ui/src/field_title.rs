use leptos::prelude::*;

/// Label-styled text that is not a `<label>`: the title inside a [`crate::FieldContent`]
/// when the whole Field is wrapped in a [`crate::FieldLabel`] card.
#[component]
pub fn FieldTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-field-title {class}")>{children()}</div> }
}
