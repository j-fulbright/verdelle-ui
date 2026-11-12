use leptos::prelude::*;

/// The header group (`<thead>`) of a [`crate::Table`].
#[component]
pub fn TableHeader(
    /// Extra classes.
    #[prop(optional, into)]
    class: String,
    /// The header rows.
    children: Children,
) -> impl IntoView {
    view! { <thead class=format!("vdl-table__header {class}")>{children()}</thead> }
}
