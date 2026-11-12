use crate::primitives::toast as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// The X button that closes a [`crate::Toast`]. [`crate::Toast`] shows one by default
/// (`show_close_button`); use this directly only to place it yourself.
#[component]
pub fn ToastClose(
    /// Accessible name. Defaults to "Close".
    #[prop(optional, into)]
    label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let label = label.unwrap_or_else(|| "Close".to_string());

    view! {
        <primitive::ToastClose
            attr:class=format!("vdl-toast__close {class}")
            attr:aria-label=label
        >
            <Icon name=IconName::X />
        </primitive::ToastClose>
    }
}
