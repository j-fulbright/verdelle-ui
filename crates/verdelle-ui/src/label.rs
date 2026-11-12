use leptos::prelude::*;

/// A form label, styled as shadcn/ui's Label. Point it at a control with `html_for`
/// (the control's `id`), or wrap the control in it.
#[component]
pub fn Label(
    #[prop(optional, into)] html_for: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <label for=html_for class=format!("vdl-label {class}")>
            {children()}
        </label>
    }
}
