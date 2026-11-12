use crate::input_otp::InputOtpStyle;
use crate::primitives::otp_field as primitive;
use leptos::prelude::*;

/// One character of an [`crate::InputOtp`]; `index` is its position, from 0. Its accessible
/// name is "Digit N of M".
#[component]
pub fn InputOtpSlot(index: usize, #[prop(optional, into)] class: String) -> impl IntoView {
    let invalid = use_context::<InputOtpStyle>().map(|s| s.invalid);
    view! {
        <primitive::OneTimePasswordFieldInput
            index=index
            attr:aria-invalid=move || invalid.and_then(|i| i.get().then_some("true"))
            attr:class=format!("vdl-input-otp__slot {class}")
        />
    }
}
