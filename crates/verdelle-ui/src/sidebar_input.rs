use crate::Input;
use leptos::prelude::*;

/// A search or filter field for the top of the sidebar: an [`crate::Input`] 32px high, on the
/// page background, with no shadow.
#[component]
pub fn SidebarInput(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    /// Text shown while there is no value.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(default = "Search".to_string(), into)]
    aria_label: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__input {class}") data-sidebar="input">
            <Input value=value placeholder=placeholder.unwrap_or_default() aria_label=aria_label />
        </div>
    }
}
