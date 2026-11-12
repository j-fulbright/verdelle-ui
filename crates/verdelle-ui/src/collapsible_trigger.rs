use crate::primitives::collapsible as primitive;
use crate::{ButtonVariant, Size};
use leptos::prelude::*;

/// The button that toggles a [`crate::Collapsible`].
///
/// Plain by default, so you can put anything inside it. Pass `variant` (and `size`,
/// `icon_only`) to draw it as a [`crate::Button`]: shadcn does this with
/// `render={<Button/>}`, and we cannot swap the element, so the trigger takes the Button's
/// look instead.
///
/// The trigger carries `data-state="open" | "closed"`, and so does the content, so an icon can
/// turn with it: give an icon the class `vdl-collapsible__icon` and it rotates when the trigger
/// it is inside (or any element with `data-state="open"` it sits directly in) is open. With
/// `state` it binds to a [`crate::CollapsibleState`] instead of the enclosing
/// [`crate::Collapsible`]; with `as_child` it is the one element you give it (a `<tr>`, a link)
/// rather than a `<button>`, and gets the behaviour and ARIA attributes but not the component's
/// classes (a non-button needs its own `tabindex` and key handling).
#[component]
pub fn CollapsibleTrigger(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    #[prop(optional)] icon_only: bool,
    /// Bind to a shared state (see [`crate::use_collapsible`]) instead of the enclosing
    /// [`crate::Collapsible`].
    #[prop(optional, into)]
    state: MaybeProp<crate::CollapsibleState>,
    /// Use the single child element instead of a `<button>`.
    #[prop(optional)]
    as_child: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let styled = variant.is_some();
    let variant = variant.unwrap_or_default();

    view! {
        <primitive::CollapsibleTrigger
            state=state
            as_child=as_child
            attr:class=(!as_child).then(|| format!("vdl-collapsible__trigger {} {class}", if styled { "vdl-btn" } else { "" }))
            attr:data-variant=(styled && !as_child).then(|| variant.as_str())
            attr:data-size=(styled && !as_child).then(|| size.as_str())
            attr:data-icon-only=(styled && icon_only && !as_child).then_some("")
        >
            {move || {
                if styled && !as_child {
                    view! { <span class="vdl-btn__content">{children.with_value(|c| c())}</span> }
                        .into_any()
                } else {
                    children.with_value(|c| c()).into_any()
                }
            }}
        </primitive::CollapsibleTrigger>
    }
}
