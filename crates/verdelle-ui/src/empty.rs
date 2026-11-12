use leptos::prelude::*;

/// The placeholder for a view with nothing to show yet, styled as shadcn/ui's Empty.
/// Compose it with [`crate::EmptyHeader`] (holding [`crate::EmptyMedia`],
/// [`crate::EmptyTitle`] and [`crate::EmptyDescription`]) and [`crate::EmptyContent`]
/// for the actions.
///
/// It centres its content and has 24px padding. `bordered` draws the dashed outline
/// that shadcn's examples add with `border border-dashed` (the component itself only
/// sets the dash style, so this is the same look with one prop).
#[component]
pub fn Empty(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] bordered: bool,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-empty {class}") data-bordered=bordered.then_some("")>
            {children()}
        </div>
    }
}
