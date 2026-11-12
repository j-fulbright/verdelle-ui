use crate::primitives::accordion as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// The heading button of an [`crate::AccordionItem`]. It renders inside an `<h3>` (the
/// accordion pattern's heading) and ends with a chevron that flips while open.
#[component]
pub fn AccordionTrigger(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::AccordionHeader attr:class="vdl-accordion__header">
            <primitive::AccordionTrigger attr:class=format!("vdl-accordion__trigger {class}")>
                {children.with_value(|children| children())}
                <Icon name=IconName::ChevronDown class="vdl-accordion__icon" />
            </primitive::AccordionTrigger>
        </primitive::AccordionHeader>
    }
}
