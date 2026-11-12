use leptos::prelude::*;

/// The `<ul>` of a [`crate::Pagination`], 4px between items.
#[component]
pub fn PaginationContent(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <ul class=format!("vdl-pagination__content {class}")>{children()}</ul> }
}
