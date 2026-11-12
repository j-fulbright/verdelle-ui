use crate::primitives::toast as primitive;
use crate::{Icon, IconName};
use crate::{ToastClose, ToastVariant};
use leptos::prelude::*;

/// One notification, styled to match shadcn/ui's Sonner toast (card colours, border, radius,
/// shadow) on the headless Radix-shaped toast primitive. Put it inside a
/// [`crate::ToastProvider`] and [`crate::ToastViewport`], or let [`crate::Toaster`] and
/// [`crate::use_toaster`] manage the list for you.
///
/// Compose the body from [`crate::ToastTitle`], [`crate::ToastDescription`] and
/// [`crate::ToastAction`]. `variant` picks the icon and colour; a `Destructive` toast is
/// announced to screen readers at once, the others politely. It closes after `duration`
/// milliseconds (the provider's default when unset, 0 for never), when the close button
/// is used, on Escape while focused, or on a swipe towards the screen edge. Hovering or
/// focusing the viewport pauses the timer, and it resumes with the time that was left.
///
/// Uncontrolled by default; `on_open_change(false)` runs once when it closes. It animates in
/// and out (as a plain fade under `prefers-reduced-motion`), so a list that removes closed toasts should
/// wait a moment (about 300ms) before dropping them, as [`crate::Toaster`] does.
#[component]
pub fn Toast(
    #[prop(optional, into)] variant: Signal<ToastVariant>,
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Milliseconds before it closes itself. 0 keeps it until closed.
    #[prop(into, optional)]
    duration: MaybeProp<i32>,
    /// Show the X button. On by default.
    #[prop(default = true)]
    show_close_button: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <primitive::Toast
            open=open
            default_open=default_open
            on_open_change=on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
            duration=duration
            assertive=Signal::derive(move || variant.get() == ToastVariant::Destructive)
            attr:class=format!("vdl-toast {class}")
            attr:data-variant=move || variant.get().as_str()
        >
            {move || {
                (variant.get() != ToastVariant::Default)
                    .then(|| view! { <span class="vdl-toast__icon">{icon(variant.get())}</span> })
            }}
            <div class="vdl-toast__body">{children.with_value(|children| children())}</div>
            {show_close_button.then(|| view! { <ToastClose /> })}
        </primitive::Toast>
    }
}

/// The 16px icon for a variant (Lucide's circle-check, info, triangle-alert and octagon-x,
/// the icons shadcn's Sonner uses).
fn icon(variant: ToastVariant) -> AnyView {
    let name = match variant {
        ToastVariant::Success => IconName::CircleCheck,
        ToastVariant::Info => IconName::Info,
        ToastVariant::Warning => IconName::TriangleAlert,
        ToastVariant::Destructive | ToastVariant::Default => IconName::OctagonX,
    };
    view! { <Icon name=name /> }.into_any()
}
