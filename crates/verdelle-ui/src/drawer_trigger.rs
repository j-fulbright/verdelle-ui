use crate::{Button, ButtonVariant, Size};
use leptos::prelude::*;

/// A button that opens a [`crate::Drawer`]. The drawer is one component (as [`crate::Sheet`]), so
/// the trigger cannot sit inside it: give it the same `open` signal. It is a [`crate::Button`];
/// pass `variant` and `size` as you would there. Any button can open a drawer with
/// `open.set(true)`; this is the shorthand.
#[component]
pub fn DrawerTrigger(
    /// The open state, read and written by the component.
    open: RwSignal<bool>,
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional, into)]
    variant: Signal<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <Button variant=variant size=size class=class on_click=move |_| open.set(true)>
            {children()}
        </Button>
    }
}
