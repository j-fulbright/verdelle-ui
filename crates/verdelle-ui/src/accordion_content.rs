use crate::primitives::accordion as primitive;
use leptos::prelude::*;

/// The panel of an [`crate::AccordionItem`]. It slides open and closed (height
/// animation, skipped under `prefers-reduced-motion`); closed panels are not rendered.
/// `class` goes on the padded inner box.
#[component]
pub fn AccordionContent(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::AccordionContent attr:class="vdl-accordion__content">
            <div class=format!("vdl-accordion__content-inner {class}")>
                {children.with_value(|children| children())}
            </div>
        </primitive::AccordionContent>
    }
}
