use crate::primitives::tooltip as primitive;
use crate::{ButtonVariant, Size};
use leptos::prelude::*;

/// The element a [`crate::Tooltip`] describes. It is a `<button>`.
///
/// With a `variant` it looks like a [`crate::Button`] of that variant and `size`; without
/// one it is a bare button that you style with `class`. `disabled` only looks disabled
/// (and sets `aria-disabled`): a truly disabled button receives no pointer events, so it
/// could not show its tooltip, and keeps focus and hover instead.
#[component]
pub fn TooltipTrigger(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    #[prop(optional)] icon_only: bool,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional)]
    disabled: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name for triggers whose content is not text, such as an icon.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let styled = variant.is_some();

    view! {
        <primitive::TooltipTrigger
            attr:class=if styled { format!("vdl-btn {class}") } else { class }
            attr:data-variant=variant.map(|v| v.as_str())
            attr:data-size=styled.then(|| size.as_str())
            attr:data-icon-only=(styled && icon_only).then_some("")
            attr:data-soft-disabled=disabled.then_some("")
            attr:aria-disabled=disabled.then_some("true")
            attr:aria-label=aria_label
        >
            {move || {
                let content = children.with_value(|children| children());
                if styled {
                    view! { <span class="vdl-btn__content">{content}</span> }.into_any()
                } else {
                    content.into_any()
                }
            }}
        </primitive::TooltipTrigger>
    }
}
