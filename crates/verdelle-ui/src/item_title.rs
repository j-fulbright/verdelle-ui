use leptos::prelude::*;

/// The title of an [`crate::Item`]: 14px, medium weight, clamped to one line.
///
/// `wrap` lets a title made of several children (a name, a badge, a status) lay out as a flex row
/// that wraps onto more lines instead of being cut at one line.
#[component]
pub fn ItemTitle(
    /// Lay the children out as a wrapping flex row and lift the one-line clamp.
    #[prop(optional)]
    wrap: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-item__title {class}") data-wrap=wrap.then_some("")>
            {children()}
        </div>
    }
}
