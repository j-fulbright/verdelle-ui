use crate::primitives::command as primitive;
use leptos::prelude::*;

/// The scrolling list of a [`crate::Command`] (`role="listbox"`), up to 288px tall. `label` is
/// its accessible name.
#[component]
pub fn CommandList(
    #[prop(into, optional)] label: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::CommandList label=label attr:class=format!("vdl-command__list {class}")>
            {children.with_value(|children| children())}
        </primitive::CommandList>
    }
}
