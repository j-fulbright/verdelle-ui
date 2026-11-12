use crate::{ItemVariant, Size};
use leptos::prelude::*;

/// A row of media, text and actions, styled as shadcn/ui's Item. Compose it with
/// [`crate::ItemMedia`], [`crate::ItemContent`] (holding [`crate::ItemTitle`] and
/// [`crate::ItemDescription`]), [`crate::ItemActions`], [`crate::ItemHeader`] and
/// [`crate::ItemFooter`]; list several in an [`crate::ItemGroup`].
///
/// Variants are `Default` (no border), `Outline` and `Muted`. Sizes are `Medium`
/// (the default, 14px/16px padding), `Small` and `XSmall`; `Large` falls back to the default.
/// Pass `href` and the item renders as an `<a>` with a hover fill and a focus ring
/// (shadcn does this with its `render` prop); without it the item is a `<div>`.
#[component]
pub fn Item(
    #[prop(optional, into)] variant: Signal<ItemVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Link target.
    #[prop(optional, into)]
    href: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let class = format!("vdl-item {class}");
    match href {
        Some(href) => view! {
            <a
                class=class
                href=href
                data-variant=move || variant.get().as_str()
                data-size=move || size.get().as_str()
            >
                {children()}
            </a>
        }
        .into_any(),
        None => view! {
            <div
                class=class
                data-variant=move || variant.get().as_str()
                data-size=move || size.get().as_str()
            >
                {children()}
            </div>
        }
        .into_any(),
    }
}
