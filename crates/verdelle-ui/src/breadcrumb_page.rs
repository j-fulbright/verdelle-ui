use leptos::prelude::*;

/// The current page, last in a [`crate::Breadcrumb`]: foreground-coloured text that is not a
/// link (`aria-current="page"`).
#[component]
pub fn BreadcrumbPage(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <span
            class=format!("vdl-breadcrumb__page {class}")
            role="link"
            aria-disabled="true"
            aria-current="page"
        >
            {children()}
        </span>
    }
}
