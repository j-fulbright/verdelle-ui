use crate::primitives::number_field as primitive;
use leptos::prelude::*;

/// A region of a [`crate::NumberField`] (usually its label) that changes the value when
/// dragged sideways: one `step` per few pixels, right for up and left for down, with Shift for
/// large and Alt for small steps. It shows a resize cursor. Not keyboard-reachable on its own;
/// the input is.
#[component]
pub fn NumberFieldScrubArea(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::NumberFieldScrubArea attr:class=format!(
            "vdl-number-field__scrub {class}",
        )>{children.with_value(|children| children())}</primitive::NumberFieldScrubArea>
    }
}
