use leptos::prelude::*;

/// Page navigation, styled as shadcn/ui's Pagination: a centred `<nav aria-label="pagination">`
/// around a [`crate::PaginationContent`] list of [`crate::PaginationItem`]s holding
/// [`crate::PaginationPrevious`], [`crate::PaginationLink`], [`crate::PaginationEllipsis`]
/// and [`crate::PaginationNext`]. It only lays out links; the app owns the page state.
#[component]
pub fn Pagination(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <nav class=format!("vdl-pagination {class}") role="navigation" aria-label="pagination">
            {children()}
        </nav>
    }
}
