use crate::Size;
use leptos::{ev, prelude::*};

/// A link in a [`crate::SidebarMenuSubItem`]: an `<a>` when `href` is set, else a `<button>`.
/// `is_active` marks the current page (`data-active`, and `aria-current="page"` on a link).
/// `size` is `Small` (12px text) or anything else (14px). 28px high.
#[component]
pub fn SidebarMenuSubButton(
    /// Link target.
    #[prop(optional, into)]
    href: Option<String>,
    #[prop(optional, into)] is_active: Signal<bool>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// The content.
    children: Children,
) -> impl IntoView {
    let size = if size == Size::Small { "sm" } else { "md" };
    let class = format!("vdl-sidebar__menu-sub-button {class}");
    let active = move || is_active.get().then_some("");
    let click = move |ev| {
        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };
    match href {
        Some(href) => view! {
            <a
                class=class
                href=href
                data-sidebar="menu-sub-button"
                data-size=size
                data-active=active
                aria-current=move || is_active.get().then_some("page")
                on:click=click
            >
                {children()}
            </a>
        }
        .into_any(),
        None => view! {
            <button
                type="button"
                class=class
                data-sidebar="menu-sub-button"
                data-size=size
                data-active=active
                on:click=click
            >
                {children()}
            </button>
        }
        .into_any(),
    }
}
