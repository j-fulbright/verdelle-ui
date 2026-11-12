use crate::primitives::accordion as primitive;
use leptos::prelude::*;

pub use primitive::AccordionType;

/// A stack of headings that each show or hide a panel, styled as shadcn/ui's Accordion
/// and built on the headless Accordion (ids, `aria-expanded`, and arrow/Home/End
/// focus between triggers).
///
/// Compose with [`crate::AccordionItem`], [`crate::AccordionTrigger`] and
/// [`crate::AccordionContent`]. `r#type` is [`AccordionType::Single`] (one item open) or
/// [`AccordionType::Multiple`]. With `Single`, `collapsible` lets the open item be closed
/// again. Uncontrolled by default (`default_value` lists the open items); pass `value`
/// and `on_value_change` to control it.
#[component]
pub fn Accordion(
    #[prop(into)] r#type: AccordionType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<Vec<String>>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Vec<String>>>,
    #[prop(optional)] collapsible: bool,
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
        <primitive::Accordion
            r#type=r#type
            value=value
            default_value=default_value
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Vec<String>| {}))
            collapsible=collapsible
            disabled=disabled
            attr:class=format!("vdl-accordion {class}")
        >
            {children.with_value(|children| children())}
        </primitive::Accordion>
    }
}
