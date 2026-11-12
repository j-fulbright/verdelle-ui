use crate::primitives::context_menu as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// The item that opens a [`crate::ContextMenuSub`], with a chevron at its end. `disabled`
/// and `inset` work as on [`crate::ContextMenuItem`].
#[component]
pub fn ContextMenuSubTrigger(
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Indents it to line up with sibling items that have an icon or check indicator.
    #[prop(optional)]
    inset: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ContextMenuSubTrigger
            disabled=disabled
            attr:class=format!("vdl-menu__item vdl-menu__sub-trigger {class}")
            attr:data-inset=inset.then_some("")
        >
            {children.with_value(|children| children())}
            <Icon name=IconName::ChevronRight class="vdl-menu__chevron" />
        </primitive::ContextMenuSubTrigger>
    }
}
