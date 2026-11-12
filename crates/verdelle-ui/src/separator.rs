use crate::primitives::separator as primitive;
use leptos::prelude::*;

pub use primitive::Orientation;

/// A thin dividing line, styled as shadcn/ui's Separator and built on the
/// headless Separator. A `decorative` separator is hidden from assistive
/// technology; otherwise it is announced as a separator.
#[component]
pub fn Separator(
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] decorative: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::Separator
            orientation=orientation
            decorative=decorative
            attr:class=format!("vdl-separator {class}")
        />
    }
}
