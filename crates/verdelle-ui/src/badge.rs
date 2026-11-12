use crate::{BadgeVariant, Icon, IconName};
use leptos::{ev, prelude::*};

/// A small pill-shaped label, styled as shadcn/ui's Badge.
///
/// With `on_click` it renders as a `<button>` (shadcn lets a badge be a link or a button) and gets
/// hover and focus styles; otherwise it is a plain `<span>`. An inline `<svg>` child is sized as
/// an icon.
///
/// With `on_remove` the badge becomes a removable chip, for filters and tags: an X button at the
/// end, a tab stop of its own with the accessible name `remove_label` (default "Remove"; make it
/// specific, like "Remove filter: open"). The button calls `on_remove` and does not remove
/// anything itself, so the parent drops the chip from its list (Enter or Space on the focused X
/// works like a click). Combined with `on_click` the label part is a button and the X is a second
/// one beside it. Verdelle extra; shadcn's Badge has no remove button.
///
/// For a count or dot overlaid on another element, use [`crate::Indicator`].
/// See `NAMING.md`.
#[component]
pub fn Badge(
    #[prop(optional, into)] variant: Signal<BadgeVariant>,
    /// Extra classes, for layout. They override the badge's own styles.
    #[prop(optional, into)]
    class: String,
    /// Tooltip text. For a clickable badge, say what clicking does.
    #[prop(optional, into)]
    title: Option<String>,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Adds an X button that calls this. See the component docs.
    #[prop(optional, into)]
    on_remove: Option<Callback<()>>,
    /// The X button's accessible name.
    #[prop(default = "Remove".to_string(), into)]
    remove_label: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let class = format!("vdl-badge {class}");

    if let Some(on_remove) = on_remove {
        let remove = view! {
            <button
                type="button"
                class="vdl-badge__remove"
                aria-label=remove_label
                on:click=move |ev| {
                    ev.stop_propagation();
                    on_remove.run(());
                }
            >
                <Icon name=IconName::X />
            </button>
        };
        let body = match on_click {
            Some(on_click) => view! {
                <button
                    type="button"
                    class="vdl-badge__main"
                    title=title
                    on:click=move |ev| on_click.run(ev)
                >
                    {children()}
                </button>
            }
            .into_any(),
            None => view! { <span class="vdl-badge__main">{children()}</span> }.into_any(),
        };
        return view! {
            <span
                class=class
                data-variant=move || variant.get().as_str()
                data-removable=""
                data-interactive=on_click.is_some().then_some("")
            >
                {body}
                {remove}
            </span>
        }
        .into_any();
    }

    match on_click {
        Some(on_click) => view! {
            <button
                type="button"
                class=class
                data-variant=move || variant.get().as_str()
                data-interactive=""
                title=title
                on:click=move |ev| on_click.run(ev)
            >
                {children()}
            </button>
        }
        .into_any(),
        None => view! {
            <span class=class data-variant=move || variant.get().as_str() title=title>
                {children()}
            </span>
        }
        .into_any(),
    }
}
