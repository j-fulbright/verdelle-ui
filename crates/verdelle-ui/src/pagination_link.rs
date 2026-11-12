use crate::Size;
use leptos::{ev, prelude::*};

/// A page link, an `<a>` drawn as a [`crate::Button`]: `Outline` when `active`
/// (with `aria-current="page"`), `Ghost` otherwise. By default it is a square icon-size
/// button holding a page number; pass `icon_only=false` for a text-width one.
#[component]
pub fn PaginationLink(
    /// Link target.
    #[prop(into)]
    href: String,
    #[prop(optional, into)] active: Signal<bool>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    #[prop(default = true)] icon_only: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Runs on click. Call `prevent_default()` on the event to handle the page change yourself.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <a
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
            class=format!("vdl-btn vdl-pagination__link {class}")
            href=href
            data-variant=move || if active.get() { "outline" } else { "ghost" }
            data-size=move || size.get().as_str()
            data-icon-only=icon_only.then_some("")
            data-active=move || active.get().then_some("")
            aria-current=move || active.get().then_some("page")
            aria-label=aria_label
        >
            <span class="vdl-btn__content">{children()}</span>
        </a>
    }
}
