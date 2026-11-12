use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// A divider between groups. It hides while a search narrows the list (set `always_render` to
/// keep it).
#[component]
pub fn ComboboxSeparator(
    #[prop(into, optional)] always_render: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::ComboboxSeparator
            always_render=always_render
            attr:class=format!("vdl-combobox__separator {class}")
        />
    }
}
