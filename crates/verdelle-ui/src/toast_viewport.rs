use crate::ToastPosition;
use crate::primitives::toast as primitive;
use crate::toast_provider::ToastPositionContext;
use leptos::prelude::*;

/// The fixed region of the screen where toasts appear, as the list that screen readers
/// find with the label (default "Notifications"). Pressing `F8` moves focus into it, and
/// hovering or focusing it pauses every toast's timer.
///
/// It sits at the position given to the surrounding [`crate::ToastProvider`]. Toasts stack
/// with the newest nearest the screen edge. It is placed with `top`/`left`/`bottom`/`right`
/// and a transform, never margins.
#[component]
pub fn ToastViewport(
    #[prop(into, optional)] label: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let position = use_context::<ToastPositionContext>()
        .map(|ctx| ctx.0)
        .unwrap_or_else(|| Signal::stored(ToastPosition::default()));

    view! {
        <primitive::ToastViewport
            label=label
            attr:class=format!("vdl-toast-viewport {class}")
            attr:data-position=move || position.get().as_str()
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </primitive::ToastViewport>
    }
}
