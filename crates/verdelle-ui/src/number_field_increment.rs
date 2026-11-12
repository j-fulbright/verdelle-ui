use crate::icons;
use crate::primitives::number_field as primitive;
use leptos::prelude::*;

/// The plus button of a [`crate::NumberField`]: steps up, and keeps stepping while held. It is
/// not a tab stop (the input handles the keyboard) and disables itself at `max`.
#[component]
pub fn NumberFieldIncrement(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <primitive::NumberFieldIncrement attr:class=format!(
            "vdl-number-field__button {class}",
        )>{icons::plus()}</primitive::NumberFieldIncrement>
    }
}
