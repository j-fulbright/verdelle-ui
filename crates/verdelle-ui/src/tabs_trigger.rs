use crate::primitives::tabs as primitive;
use leptos::prelude::*;

/// One tab button. `value` ties it to the [`crate::TabsContent`] with the same value.
#[component]
pub fn TabsTrigger(
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
        <primitive::TabsTrigger
            value=value
            disabled=disabled
            attr:class=format!("vdl-tabs__trigger {class}")
        >
            {children.with_value(|children| children())}
        </primitive::TabsTrigger>
    }
}
