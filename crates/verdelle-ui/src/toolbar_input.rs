use crate::primitives::toolbar as primitive;
use leptos::prelude::*;

/// A text field in a [`crate::Toolbar`]. Left and Right move the caret and only hand focus
/// to the next item at the edge of the text. `value` is two-way.
#[component]
pub fn ToolbarInput(
    /// The value, read and written by the component.
    value: RwSignal<String>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Text shown while there is no value.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Form field name; the value is submitted under it.
    #[prop(optional, into)]
    name: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::ToolbarInput
            disabled=disabled
            attr:class=format!("vdl-toolbar__input {class}")
            attr:r#type="text"
            attr:placeholder=placeholder
            attr:aria-label=aria_label
            attr:name=name
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
