use crate::ToastPosition;
use crate::primitives::toast as primitive;
use leptos::prelude::*;

/// Which corner or edge the toasts under a provider appear at; read by [`crate::ToastViewport`].
#[derive(Clone, Copy)]
pub(crate) struct ToastPositionContext(pub Signal<ToastPosition>);

/// Holds the toasts below it. Put one around the part of the app that shows toasts, with a
/// [`crate::ToastViewport`] inside it; [`crate::Toaster`] does both for you.
///
/// `duration` is how long a toast stays, in milliseconds, unless the toast sets its own
/// (default 5000; 0 keeps it until closed). `position` decides where the viewport sits and
/// which way a swipe dismisses.
#[component]
pub fn ToastProvider(
    #[prop(into, optional)] duration: MaybeProp<i32>,
    #[prop(into, optional)] position: Signal<ToastPosition>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    provide_context(ToastPositionContext(position));

    view! {
        <primitive::ToastProvider
            duration=duration
            swipe_direction=Signal::derive(move || position.get().swipe_direction().to_string())
        >
            {children.with_value(|children| children())}
        </primitive::ToastProvider>
    }
}
