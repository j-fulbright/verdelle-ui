use leptos::prelude::*;

/// A visible title or note for a [`crate::Table`] (`<caption>`).
/// It also names the table for assistive technology. Place it first inside the table.
#[component]
pub fn TableCaption(
    /// Extra classes.
    #[prop(optional, into)]
    class: String,
    /// The caption text.
    children: Children,
) -> impl IntoView {
    view! { <caption class=format!("vdl-table__caption {class}")>{children()}</caption> }
}
