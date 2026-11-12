use crate::{PaginationLink, icons};
use leptos::{ev, prelude::*};

/// The link to the next page: `text` (default "Next"; hides below 640px wide) and a chevron.
/// Its accessible name is "Go to next page".
#[component]
pub fn PaginationNext(
    /// Link target.
    #[prop(default = "#".into(), into)]
    href: String,
    #[prop(default = "Next".into(), into)] text: String,
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
            class=format!("vdl-pagination__next {class}")
            aria_label="Go to next page"
        >
            <span class="vdl-pagination__text">{text}</span>
            {icons::chevron_right()}
        </PaginationLink>
    }
}
