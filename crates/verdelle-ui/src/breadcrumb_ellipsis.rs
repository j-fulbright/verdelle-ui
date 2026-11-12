use crate::icons;
use leptos::prelude::*;

/// A "more" mark standing for collapsed levels of a [`crate::Breadcrumb`]. Wrap it in a menu
/// trigger to reveal them. Carries a screen-reader-only "More" label.
#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <span class=format!("vdl-breadcrumb__ellipsis {class}") role="presentation" aria-hidden="true">
            {icons::ellipsis()}
            <span class="vdl-sr-only">"More"</span>
        </span>
    }
}
