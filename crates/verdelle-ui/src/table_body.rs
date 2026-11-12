use leptos::prelude::*;

/// The body group (`<tbody>`) of a [`crate::Table`].
#[component]
pub fn TableBody(
    /// Extra classes.
    #[prop(optional, into)]
    class: String,
    /// The rows.
    children: Children,
) -> impl IntoView {
    view! { <tbody class=format!("vdl-table__body {class}")>{children()}</tbody> }
}
