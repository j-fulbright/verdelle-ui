use leptos::prelude::*;

/// A run of [`crate::InputOtpSlot`]s drawn as one joined box (the code `123-456` is two groups).
#[component]
pub fn InputOtpGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-input-otp__group {class}")>{children()}</div> }
}
