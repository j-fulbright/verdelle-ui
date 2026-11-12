use crate::{Icon, IconName};
use leptos::prelude::*;

/// A dash between [`crate::InputOtpGroup`]s.
#[component]
pub fn InputOtpSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("vdl-input-otp__separator {class}") role="separator">
            <Icon name=IconName::Minus />
        </div>
    }
}
