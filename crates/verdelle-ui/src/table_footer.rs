use leptos::prelude::*;

/// The footer group (`<tfoot>`) of a [`crate::Table`]: a muted band with medium-weight text,
/// for totals. Its rows do not tint on hover.
#[component]
pub fn TableFooter(
    /// Extra classes.
    #[prop(optional, into)]
    class: String,
    /// The footer rows.
    children: Children,
) -> impl IntoView {
    view! { <tfoot class=format!("vdl-table__footer {class}")>{children()}</tfoot> }
}
