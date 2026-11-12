use leptos::prelude::*;

/// A surface that groups related content, styled as shadcn/ui's Card. Compose it
/// with [`crate::CardHeader`], [`crate::CardTitle`], [`crate::CardDescription`],
/// [`crate::CardAction`], [`crate::CardContent`] and [`crate::CardFooter`].
///
/// `class` adds layout classes (for example Tailwind utilities); they override the
/// card's own styles. `small` uses 12px instead of 16px padding.
#[component]
pub fn Card(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] small: bool,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-card {class}") data-size=small.then_some("sm")>
            {children()}
        </div>
    }
}
