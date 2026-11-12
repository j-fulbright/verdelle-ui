use leptos::prelude::*;

/// A trail of links showing where the page sits, styled as shadcn/ui's Breadcrumb.
/// A `<nav aria-label="breadcrumb">` around a [`crate::BreadcrumbList`] of
/// [`crate::BreadcrumbItem`]s holding a [`crate::BreadcrumbLink`], [`crate::BreadcrumbPage`]
/// (the current page), [`crate::BreadcrumbSeparator`] or [`crate::BreadcrumbEllipsis`].
#[component]
pub fn Breadcrumb(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <nav class=format!("vdl-breadcrumb {class}") aria-label="breadcrumb">
            {children()}
        </nav>
    }
}
