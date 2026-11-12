use leptos::prelude::*;

/// A box that keeps a width-to-height ratio, styled as shadcn/ui's Aspect Ratio.
///
/// `ratio` is width divided by height (`16.0 / 9.0`). The box is `position:
/// relative` and takes its width from the parent (or `class`); children are
/// laid out inside it, for example an `<img>` with `class="size-full object-cover"`.
#[component]
pub fn AspectRatio(
    ratio: f64,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-aspect-ratio {class}") style=format!("--vdl-ratio: {ratio}")>
            {children()}
        </div>
    }
}
