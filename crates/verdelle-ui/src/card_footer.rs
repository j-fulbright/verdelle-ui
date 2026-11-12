use leptos::prelude::*;

/// The bottom of a [`crate::Card`], usually actions. As in shadcn's nova style it is always a muted band with a rule above it,
/// flush with the card's bottom edge (the card drops its own bottom padding); `bordered` is kept
/// for compatibility and changes nothing.
#[component]
pub fn CardFooter(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] bordered: bool,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-card__footer {class}") data-bordered=bordered.then_some("")>
            {children()}
        </div>
    }
}
