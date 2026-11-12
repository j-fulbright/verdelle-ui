use crate::primitives::toolbar as primitive;
use leptos::prelude::*;

/// A link in a [`crate::Toolbar`]; one stop of the roving focus.
#[component]
pub fn ToolbarLink(
    /// Link target.
    #[prop(into)]
    href: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ToolbarLink attr:href=href attr:class=format!("vdl-toolbar__link {class}")>
            {children.with_value(|children| children())}
        </primitive::ToolbarLink>
    }
}
