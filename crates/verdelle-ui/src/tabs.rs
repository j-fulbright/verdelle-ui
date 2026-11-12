use crate::primitives::tabs as primitive;
use leptos::prelude::*;

/// Tabs, styled as shadcn/ui's Tabs and built on the headless Tabs, which
/// provide the roles, roving keyboard focus (arrows, Home, End) and ids.
///
/// Compose with [`crate::TabsList`], [`crate::TabsTrigger`] and [`crate::TabsContent`].
/// Uncontrolled by default (`default_value` picks the first tab); pass `value` and
/// `on_value_change` to control it. Not provided: shadcn's `orientation` (vertical tabs) and the
/// `line` list variant.
#[component]
pub fn Tabs(
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Tabs
            value=value
            default_value=default_value
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: String| {}))
            attr:class=format!("vdl-tabs {class}")
        >
            {children.with_value(|children| children())}
        </primitive::Tabs>
    }
}
