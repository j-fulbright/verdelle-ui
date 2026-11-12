use leptos::prelude::*;

/// An ancestor page in a [`crate::Breadcrumb`]: an `<a>` that turns to the foreground colour
/// on hover. shadcn can render another element in its place with `render`; here, wrap your
/// router's link by passing the same `class` or use the plain anchor.
#[component]
pub fn BreadcrumbLink(
    /// Link target.
    #[prop(into)]
    href: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <a class=format!("vdl-breadcrumb__link {class}") href=href>
            {children()}
        </a>
    }
}
