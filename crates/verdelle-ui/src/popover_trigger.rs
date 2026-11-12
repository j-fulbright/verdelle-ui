use crate::primitives::popover as primitive;
use crate::{ButtonVariant, Size};
use leptos::prelude::*;

/// The element that opens a [`crate::Popover`]. It is a `<button>`.
///
/// With a `variant` it looks like a [`crate::Button`] of that variant and `size`; without
/// one it is a bare button that you style with `class`.
#[component]
pub fn PopoverTrigger(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    #[prop(optional)] icon_only: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let styled = variant.is_some();

    view! {
        <primitive::PopoverTrigger
            attr:class=if styled { format!("vdl-btn {class}") } else { class }
            attr:data-variant=variant.map(|v| v.as_str())
            attr:data-size=styled.then(|| size.as_str())
            attr:data-icon-only=(styled && icon_only).then_some("")
        >
            {move || {
                let content = children.with_value(|children| children());
                if styled {
                    view! { <span class="vdl-btn__content">{content}</span> }.into_any()
                } else {
                    content.into_any()
                }
            }}
        </primitive::PopoverTrigger>
    }
}
