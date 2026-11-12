use leptos::prelude::*;

/// Helper text under a control. Give it an `id` and point the control's `aria_describedby`
/// at it so assistive technology reads it with the control. Links inside are underlined.
#[component]
pub fn FieldDescription(
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <p id=id class=format!("vdl-field-description {class}")>
            {children()}
        </p>
    }
}
