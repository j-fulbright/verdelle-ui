use crate::icons;
use leptos::prelude::*;

/// A "more pages" mark between page links, 32px square like an icon button. Carries a
/// screen-reader-only "More pages" label.
#[component]
pub fn PaginationEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <span class=format!("vdl-pagination__ellipsis {class}") aria-hidden="true">
            {icons::ellipsis()}
            <span class="vdl-sr-only">"More pages"</span>
        </span>
    }
}
