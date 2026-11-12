use crate::primitives::accordion as primitive;
use leptos::prelude::*;

/// One section of an [`crate::Accordion`]: a trigger and its content. `value` names it
/// in the accordion's value. A disabled item cannot be opened or closed.
#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::AccordionItem
            value=value
            disabled=disabled
            attr:class=format!("vdl-accordion__item {class}")
        >
            {children.with_value(|children| children())}
        </primitive::AccordionItem>
    }
}
