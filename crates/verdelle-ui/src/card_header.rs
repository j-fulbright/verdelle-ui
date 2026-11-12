use leptos::prelude::*;

/// The top of a [`crate::Card`]: a title, a description, and optionally a
/// [`crate::CardAction`] at the end. `bordered` draws a rule beneath it.
#[component]
pub fn CardHeader(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] bordered: bool,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-card__header {class}") data-bordered=bordered.then_some("")>
            {children()}
        </div>
    }
}
