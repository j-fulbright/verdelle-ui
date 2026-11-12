use crate::primitives::tabs as primitive;
use leptos::prelude::*;

/// The panel shown while its tab (the [`crate::TabsTrigger`] with the same `value`) is
/// selected. Inactive panels are not rendered.
#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::TabsContent value=value attr:class=format!("vdl-tabs__content {class}")>
            {children.with_value(|children| children())}
        </primitive::TabsContent>
    }
}
