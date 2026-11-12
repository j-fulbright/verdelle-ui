use crate::primitives::popover as primitive;
use crate::{ButtonVariant, Size};
use leptos::prelude::*;

/// A button inside a [`crate::PopoverContent`] that closes the popover. Looks like a
/// [`crate::Button`] when given a `variant`, otherwise it is a bare button.
#[component]
pub fn PopoverClose(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let styled = variant.is_some();

    view! {
        <primitive::PopoverClose
            attr:class=if styled { format!("vdl-btn {class}") } else { class }
            attr:data-variant=variant.map(|v| v.as_str())
            attr:data-size=styled.then(|| size.as_str())
        >
            {move || {
                let content = children.with_value(|children| children());
                if styled {
                    view! { <span class="vdl-btn__content">{content}</span> }.into_any()
                } else {
                    content.into_any()
                }
            }}
        </primitive::PopoverClose>
    }
}
