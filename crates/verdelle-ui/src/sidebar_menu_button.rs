use crate::primitives::tooltip as primitive;
use crate::sidebar_context::use_sidebar;
use crate::{Side, SidebarMenuButtonVariant, Size, Tooltip, TooltipContent};
use leptos::{ev, prelude::*};

/// The main control of a [`crate::SidebarMenuItem`]: an icon and a label. An `<a>` when `href` is
/// set (in a router app, a plain anchor is intercepted by the router as usual), else a
/// `<button>`; shadcn swaps the element with `render`.
///
/// - `is_active`: the current page; `data-active`, a filled background and `aria-current="page"`.
/// - `variant`: [`SidebarMenuButtonVariant`], `Default` or `Outline`.
/// - `size`: `Small` 28px, default 32px, `Large` 48px.
/// - `tooltip`: text shown beside the button on hover or focus, only while the sidebar is
///   collapsed to icons on a large screen (the label is clipped then, so this is the only
///   label). It needs the surrounding [`crate::SidebarProvider`].
///   Only one tooltip shows at a time, and it closes on pointer leave, blur, a press or Escape.
/// - `tooltip_hidden`: keeps the tooltip closed while true.
/// - `disabled`: greys it out and ignores clicks.
///
/// While the sidebar is collapsed to icons the button is a 32px square with only its icon
/// visible; put the label in a `<span>` so it can be clipped and truncated.
#[component]
pub fn SidebarMenuButton(
    /// Link target.
    #[prop(optional, into)]
    href: Option<String>,
    #[prop(optional, into)] is_active: Signal<bool>,
    #[prop(optional)] variant: SidebarMenuButtonVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    #[prop(optional, into)] tooltip: Option<String>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// For a button that opens and closes a group: sets `aria-expanded` from this signal.
    #[prop(optional, into)]
    expanded: Option<Signal<bool>>,
    /// Keeps the tooltip closed while true, for example while a menu opened from this button
    /// is showing (the icon rail's group flyout).
    #[prop(optional, into)]
    tooltip_hidden: Signal<bool>,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_sidebar();
    let size = match size {
        Size::Small | Size::XSmall => "sm",
        Size::Medium => "md",
        Size::Large => "lg",
    };
    let class = format!("vdl-sidebar__menu-button {class}");
    let active = move || is_active.get().then_some("");
    let off = move || disabled.get().then_some("true");
    let click = move |ev: ev::MouseEvent| {
        if disabled.get_untracked() {
            ev.prevent_default();
            return;
        }
        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };

    let make = move || match href.clone() {
        Some(href) => view! {
            <a
                class=class.clone()
                href=href
                data-sidebar="menu-button"
                data-size=size
                data-variant=variant.as_str()
                data-active=active
                aria-current=move || is_active.get().then_some("page")
                aria-disabled=off
                aria-label=aria_label.clone()
                aria-expanded=move || expanded.map(|e| e.get().to_string())
                on:click=click
            >
                {children()}
            </a>
        }
        .into_any(),
        None => view! {
            <button
                type="button"
                class=class.clone()
                data-sidebar="menu-button"
                data-size=size
                data-variant=variant.as_str()
                data-active=active
                aria-disabled=off
                disabled=move || disabled.get()
                aria-label=aria_label.clone()
                aria-expanded=move || expanded.map(|e| e.get().to_string())
                on:click=click
            >
                {children()}
            </button>
        }
        .into_any(),
    };

    let make = StoredValue::new(make);
    match tooltip {
        None => make.with_value(|make| make()),
        Some(text) => {
            // Hover and focus ask to open it; it only shows while the labels are hidden. The
            // open state is ours (controlled), and a request that arrives while the tooltip is
            // not allowed is dropped: remembering it would show the tooltip later, when the
            // sidebar collapses, for every item that was hovered while it was expanded.
            let text = StoredValue::new(text);
            let open = RwSignal::new(false);
            let allowed = move || !ctx.expanded() && !ctx.is_mobile.get() && !tooltip_hidden.get();
            Effect::new(move |_| {
                if !allowed() {
                    open.set(false);
                }
            });
            view! {
                <Tooltip
                    open=open
                    on_open_change=Callback::new(move |o: bool| open.set(o && allowed()))
                >
                    <primitive::TooltipTrigger as_child=true>{make.with_value(|make| make())}</primitive::TooltipTrigger>
                    <TooltipContent side=Side::Right arrow=false>
                        {text.get_value()}
                    </TooltipContent>
                </Tooltip>
            }
            .into_any()
        }
    }
}
