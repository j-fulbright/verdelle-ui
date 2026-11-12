use crate::{PaginationLink, icons};
use leptos::{ev, prelude::*};

/// The link to the previous page: a chevron and `text` (default "Previous"; the text hides
/// below 640px wide). Its accessible name is "Go to previous page".
#[component]
pub fn PaginationPrevious(
    /// Link target.
    #[prop(default = "#".into(), into)]
    href: String,
    #[prop(default = "Previous".into(), into)] text: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    view! {
        <PaginationLink
            on_click=Callback::new(move |e| {
                if let Some(cb) = on_click {
                    cb.run(e);
                }
            })
            href=href
            icon_only=false
            class=format!("vdl-pagination__previous {class}")
            aria_label="Go to previous page"
        >
            {icons::chevron_left()}
            <span class="vdl-pagination__text">{text}</span>
        </PaginationLink>
    }
}
