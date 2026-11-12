//! Context menu: a menu that opens at the pointer on right-click (or a touch long-press).
//!
//! Originally ported from Leptix, which duplicated the whole menu. It now shares the
//! dropdown menu's implementation: the root provides the same context the dropdown menu
//! does, so every part but the root and the trigger *is* the dropdown menu part (re-exported
//! under the context menu's names). Fixes to the menu logic land in one place.

use crate::primitives::core::direction::Direction;
use crate::primitives::core::id::use_id;
use crate::primitives::core::popper::{Popper, use_popper_anchor_ref};
use crate::primitives::core::primitive::Primitive;
use crate::primitives::dropdown_menu::MenuContextValue;
use crate::primitives::dropdown_menu::set_timeout_once;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

pub use crate::primitives::dropdown_menu::DropdownMenuArrow as ContextMenuArrow;
pub use crate::primitives::dropdown_menu::DropdownMenuCheckboxItem as ContextMenuCheckboxItem;
pub use crate::primitives::dropdown_menu::DropdownMenuContent as ContextMenuContent;
pub use crate::primitives::dropdown_menu::DropdownMenuGroup as ContextMenuGroup;
pub use crate::primitives::dropdown_menu::DropdownMenuItem as ContextMenuItem;
pub use crate::primitives::dropdown_menu::DropdownMenuItemIndicator as ContextMenuItemIndicator;
pub use crate::primitives::dropdown_menu::DropdownMenuLabel as ContextMenuLabel;
pub use crate::primitives::dropdown_menu::DropdownMenuPortal as ContextMenuPortal;
pub use crate::primitives::dropdown_menu::DropdownMenuRadioGroup as ContextMenuRadioGroup;
pub use crate::primitives::dropdown_menu::DropdownMenuRadioItem as ContextMenuRadioItem;
pub use crate::primitives::dropdown_menu::DropdownMenuSeparator as ContextMenuSeparator;
pub use crate::primitives::dropdown_menu::DropdownMenuSub as ContextMenuSub;
pub use crate::primitives::dropdown_menu::DropdownMenuSubContent as ContextMenuSubContent;
pub use crate::primitives::dropdown_menu::DropdownMenuSubTrigger as ContextMenuSubTrigger;

/// How long a touch must be held to count as a long press, as in Radix.
const LONG_PRESS_MS: i32 = 700;
/// How far (px) a touch may move before the long press is cancelled.
const LONG_PRESS_SLOP: f64 = 10.0;

#[derive(Clone, Copy)]
struct ContextMenuState {
    /// Viewport position of the last request to open the menu.
    position: RwSignal<(f64, f64)>,
    /// A pending long-press timer.
    press_timer: RwSignal<Option<i32>>,
    /// Where the long press started.
    press_start: RwSignal<(f64, f64)>,
}

#[component]
pub fn ContextMenu(
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open = RwSignal::new(false);
    let dir = Signal::derive(move || dir.get().unwrap_or(Direction::Ltr));
    let base_id = use_id(None).get_untracked();

    let ctx = MenuContextValue {
        open: open.into(),
        on_open_change: Callback::new(move |v: bool| {
            if open.get_untracked() != v {
                open.set(v);
                if let Some(cb) = on_open_change {
                    cb.run(v);
                }
            }
        }),
        // Not attached to anything: a press on the trigger area dismisses the menu like any
        // other press outside it.
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{base_id}-content"),
        dir,
        open_with_keyboard: RwSignal::new(false),
    };
    let state = ContextMenuState {
        position: RwSignal::new((0.0, 0.0)),
        press_timer: RwSignal::new(None),
        press_start: RwSignal::new((0.0, 0.0)),
    };

    // A right-click anywhere else while the menu is open closes it (the dismissable layer
    // ignores right-clicks). One on the menu itself stays inside it. A right-click on the
    // trigger closes it here and the trigger reopens it at the new spot.
    {
        let ctx = ctx.clone();
        Effect::new(move |_| {
            let ctx = ctx.clone();
            let handle = window_event_listener(leptos::ev::contextmenu, move |event| {
                // The trigger prevents the default of the events it handles.
                if event.default_prevented() || !ctx.open.get_untracked() {
                    return;
                }
                let inside_menu = event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .zip(
                        web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id(&ctx.content_id)),
                    )
                    .is_some_and(|(target, menu)| menu.contains(Some(&target)));
                if inside_menu {
                    event.prevent_default();
                } else {
                    ctx.on_open_change.run(false);
                }
            });
            on_cleanup(move || handle.remove());
        });
    }

    view! {
        <Popper>
            <Provider value=ctx.clone()>
                <Provider value=state>
                    {children.with_value(|c| c())}
                    <ContextMenuAnchor />
                </Provider>
            </Provider>
        </Popper>
    }
}

/// A zero-size fixed element at the pointer, which the menu positions itself against.
#[component]
fn ContextMenuAnchor() -> impl IntoView {
    let state = expect_context::<ContextMenuState>();
    let anchor_ref = use_popper_anchor_ref();
    // Rendered afresh for every position: floating-ui follows the anchor element, and only a
    // new element makes an already open menu move to the new spot.
    move || {
        let (x, y) = state.position.get();
        view! {
            <span
                node_ref=anchor_ref
                data-vdl-context-menu-anchor=""
                aria-hidden="true"
                style=format!(
                    "position:fixed;left:{x}px;top:{y}px;width:0;height:0;pointer-events:none;visibility:hidden;"
                )
            ></span>
        }
    }
}

/// The area that opens the menu: right-click, the context-menu key, Shift+F10, or a touch
/// held for 700ms.
#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let state = expect_context::<ContextMenuState>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let open_at = move |x: f64, y: f64| {
        state.position.set((x, y));
        ctx.open_with_keyboard.set(false);
        ctx.on_open_change.run(true);
    };
    let clear_press = move || {
        if let Some(id) = state.press_timer.get_untracked()
            && let Some(w) = web_sys::window()
        {
            w.clear_timeout_with_handle(id);
        }
        state.press_timer.set(None);
    };
    on_cleanup(clear_press);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || disabled.get().then_some("")
            on:contextmenu=move |event: web_sys::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }
                // A touch long press may fire this as well; the menu opens either way.
                clear_press();
                event.prevent_default();
                let (x, y) = (event.client_x() as f64, event.client_y() as f64);
                if ctx.open.get_untracked() {
                    // Close first so the menu is placed afresh at the new spot.
                    ctx.on_open_change.run(false);
                    set_timeout_once(0, move || open_at(x, y));
                } else {
                    open_at(x, y);
                }
            }
            on:pointerdown=move |event: web_sys::PointerEvent| {
                if disabled.get_untracked() || event.pointer_type() == "mouse" {
                    return;
                }
                clear_press();
                let (x, y) = (event.client_x() as f64, event.client_y() as f64);
                state.press_start.set((x, y));
                state.press_timer.set(set_timeout_once(LONG_PRESS_MS, move || {
                    state.press_timer.set(None);
                    open_at(x, y);
                }));
            }
            on:pointermove=move |event: web_sys::PointerEvent| {
                if state.press_timer.get_untracked().is_some() {
                    let (sx, sy) = state.press_start.get_untracked();
                    let (dx, dy) = (event.client_x() as f64 - sx, event.client_y() as f64 - sy);
                    if dx.hypot(dy) > LONG_PRESS_SLOP {
                        clear_press();
                    }
                }
            }
            on:pointerup=move |_| clear_press()
            on:pointercancel=move |_| clear_press()
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}
