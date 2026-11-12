use crate::icons;
use crate::primitives::number_field as primitive;
use leptos::prelude::*;

/// The minus button of a [`crate::NumberField`]: steps down, and keeps stepping while held. It
/// is not a tab stop (the input handles the keyboard) and disables itself at `min`.
#[component]
pub fn NumberFieldDecrement(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <primitive::NumberFieldDecrement attr:class=format!(
            "vdl-number-field__button {class}",
        )>{icons::minus()}</primitive::NumberFieldDecrement>
    }
}
