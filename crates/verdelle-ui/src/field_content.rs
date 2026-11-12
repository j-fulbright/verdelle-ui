use leptos::prelude::*;

/// A column that holds a [`crate::FieldLabel`] and [`crate::FieldDescription`] beside a
/// control in a horizontal [`crate::Field`] (a checkbox with a label and a hint, say).
#[component]
pub fn FieldContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-field-content {class}")>{children()}</div> }
}
