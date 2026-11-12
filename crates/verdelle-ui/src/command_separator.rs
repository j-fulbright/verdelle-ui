use crate::primitives::command as primitive;
use leptos::prelude::*;

/// A divider between groups. It hides while a search narrows the list (`always_render` keeps it).
#[component]
pub fn CommandSeparator(
    #[prop(into, optional)] always_render: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::CommandSeparator
            always_render=always_render
            attr:class=format!("vdl-command__separator {class}")
        />
    }
}
