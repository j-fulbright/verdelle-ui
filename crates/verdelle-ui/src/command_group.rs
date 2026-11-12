use crate::primitives::command as primitive;
use leptos::prelude::*;

/// Items under a heading. `heading` is the title text; the group hides itself while none of its
/// items match the search.
#[component]
pub fn CommandGroup(
    #[prop(into, optional)] heading: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let heading = StoredValue::new(heading);
    view! {
        <primitive::CommandGroup attr:class=format!("vdl-command__group {class}")>
            {heading
                .get_value()
                .map(|heading| {
                    view! {
                        <primitive::CommandGroupHeading attr:class="vdl-command__heading">
                            {heading.clone()}
                        </primitive::CommandGroupHeading>
                    }
                })}
            {children.with_value(|children| children())}
        </primitive::CommandGroup>
    }
}
