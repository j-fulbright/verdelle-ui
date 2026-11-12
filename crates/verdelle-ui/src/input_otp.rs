use crate::primitives::otp_field as primitive;
use leptos::{context::Provider, prelude::*};

pub use primitive::InputValidationType;

/// What an [`InputOtp`] hands its slots.
#[derive(Clone, Copy, Debug)]
pub(crate) struct InputOtpStyle {
    pub invalid: Signal<bool>,
}

/// A one-time code field, styled as shadcn/ui's InputOTP and built on the headless
/// OneTimePasswordField (one `<input>` per character).
///
/// Compose with [`crate::InputOtpGroup`], [`crate::InputOtpSlot`] (one per character, by
/// `index`) and [`crate::InputOtpSeparator`]. `max_length` is the number of slots you
/// render. Typing fills a slot and moves on, Backspace clears and steps back, the arrow
/// keys, Home and End move between slots, and pasting spreads over the slots. `pattern`
/// limits what can be typed: digits (default), letters and digits, or anything.
/// Uncontrolled by default (`default_value`); pass `value` and `on_value_change` to control
/// it. `on_complete` fires when every slot is filled. Give `name` to submit the code with a
/// form. `invalid` marks every slot `aria-invalid`.
#[component]
pub fn InputOtp(
    #[prop(into, optional, default = 6.into())] max_length: MaybeProp<usize>,
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] on_complete: Option<Callback<String>>,
    #[prop(optional)] pattern: InputValidationType,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(into, optional)]
    invalid: Signal<bool>,
    /// Form field name; the value is submitted under it.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Provider value=InputOtpStyle { invalid }>
            <primitive::OneTimePasswordField
                max_length=max_length
                value=value
                default_value=default_value
                on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: String| {}))
                on_complete=on_complete.unwrap_or_else(|| Callback::new(|_: String| {}))
                validation_type=pattern
                disabled=disabled
                attr:aria-label=aria_label
                attr:class=format!("vdl-input-otp {class}")
            >
                {children.with_value(|children| children())}
                {move || {
                    name.get()
                        .map(|n| view! { <primitive::OneTimePasswordFieldHiddenInput name=n /> })
                }}
            </primitive::OneTimePasswordField>
        </Provider>
    }
}
