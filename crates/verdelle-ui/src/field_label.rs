use leptos::prelude::*;

/// The label of a [`crate::Field`]: a [`crate::Label`] that can also wrap a whole Field,
/// which makes a selectable card (the card highlights while its control is checked).
#[component]
pub fn FieldLabel(
    #[prop(optional, into)] html_for: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <label for=html_for class=format!("vdl-label vdl-field-label {class}")>
            {children()}
        </label>
    }
}
