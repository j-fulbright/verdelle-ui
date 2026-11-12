use leptos::prelude::*;

/// The frame of a [`crate::NumberField`]: one border and focus ring around the buttons and
/// the input.
#[component]
pub fn NumberFieldGroup(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-number-field__group {class}") role="group">
            {children()}
        </div>
    }
}
