//! Menubar: a horizontal bar of menus with keyboard navigation between them.
//!
//! Originally ported from Leptix, which duplicated the whole menu. It now builds on the
//! dropdown menu's implementation: each [`MenubarMenu`] provides the dropdown menu's context
//! (open when the bar's value is that menu), so the items, groups, check and radio items,
//! sub menus and the content's focus and key handling *are* the dropdown menu's, re-exported
//! under the menubar's names. This module adds the bar itself (value, roving tab stop,
//! Left/Right between menus, hover switching once one is open).

use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::direction::{Direction, use_direction};
use crate::primitives::core::id::use_id;
use crate::primitives::core::popper::{Popper, PopperAnchor};
use crate::primitives::core::primitive::Primitive;
use crate::primitives::dropdown_menu::{DropdownMenuContent, MenuContextValue, focus_item};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

pub use crate::primitives::dropdown_menu::DropdownMenuArrow as MenubarArrow;
pub use crate::primitives::dropdown_menu::DropdownMenuCheckboxItem as MenubarCheckboxItem;
pub use crate::primitives::dropdown_menu::DropdownMenuGroup as MenubarGroup;
pub use crate::primitives::dropdown_menu::DropdownMenuItem as MenubarItem;
pub use crate::primitives::dropdown_menu::DropdownMenuItemIndicator as MenubarItemIndicator;
pub use crate::primitives::dropdown_menu::DropdownMenuLabel as MenubarLabel;
pub use crate::primitives::dropdown_menu::DropdownMenuPortal as MenubarPortal;
pub use crate::primitives::dropdown_menu::DropdownMenuRadioGroup as MenubarRadioGroup;
pub use crate::primitives::dropdown_menu::DropdownMenuRadioItem as MenubarRadioItem;
pub use crate::primitives::dropdown_menu::DropdownMenuSeparator as MenubarSeparator;
pub use crate::primitives::dropdown_menu::DropdownMenuSub as MenubarSub;
pub use crate::primitives::dropdown_menu::DropdownMenuSubContent as MenubarSubContent;
pub use crate::primitives::dropdown_menu::DropdownMenuSubTrigger as MenubarSubTrigger;

const TRIGGER_SELECTOR: &str = "[data-vdl-menubar-trigger]:not([data-disabled])";

#[derive(Clone, Copy)]
struct MenubarContextValue {
    /// The value of the open menu, if any.
    value: Signal<Option<String>>,
    set_value: Callback<Option<String>>,
    dir: Signal<Direction>,
    do_loop: Signal<bool>,
    root_ref: AnyNodeRef,
    /// Set when the open menu was reached by keyboard, so its first item gets focus.
    open_with_keyboard: RwSignal<bool>,
    /// The trigger that holds the roving tab stop.
    current_trigger: RwSignal<Option<String>>,
}

#[derive(Clone)]
struct MenubarMenuContextValue {
    value: String,
}

#[component]
pub fn Menubar(
    /// The open menu's value (`None`: all closed). Give it to control the bar.
    #[prop(into, optional)]
    value: Option<Signal<Option<String>>>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    // Controlled when `value` is given (it may be `None`: nothing open); otherwise the bar
    // keeps the open menu itself, starting from `default_value`.
    let own_value = RwSignal::new(default_value.get_untracked());
    let current = Signal::derive(move || match value {
        Some(controlled) => controlled.get(),
        None => own_value.get(),
    });
    let set_value = Callback::new(move |next: Option<String>| {
        if current.get_untracked() == next {
            return;
        }
        if value.is_none() {
            own_value.set(next.clone());
        }
        if let Some(cb) = on_value_change {
            cb.run(next);
        }
    });
    let root_ref = AnyNodeRef::new();
    let refs = use_composed_refs(vec![node_ref, root_ref]);
    let ctx = MenubarContextValue {
        value: current,
        set_value,
        dir: direction,
        do_loop,
        root_ref,
        open_with_keyboard: RwSignal::new(false),
        current_trigger: RwSignal::new(None),
    };

    // Roving tab stop: until a trigger is focused, the first one holds it. Set after mount
    // so the server and the first client render agree (every trigger is tabbable until then).
    {
        Effect::new(move |_| {
            if ctx.current_trigger.get_untracked().is_none()
                && let Some(first) = triggers(&ctx).into_iter().next()
            {
                ctx.current_trigger
                    .set(first.get_attribute("data-menu-value"));
            }
        });
    }

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:role="menubar"
                attr:aria-orientation="horizontal"
                attr:data-orientation="horizontal"
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn MenubarMenu(
    /// Identifies the menu in the bar's `value`. Generated when omitted; give one when the
    /// bar's value is controlled or must match between server and browser.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let bar = expect_context::<MenubarContextValue>();
    let base_id = use_id(None).get_untracked();
    let menu_value = value.get_untracked().unwrap_or_else(|| base_id.clone());

    let my_value = menu_value.clone();
    let open = Signal::derive(move || bar.value.get().as_deref() == Some(my_value.as_str()));
    let my_value = menu_value.clone();
    let on_open_change = Callback::new(move |v: bool| {
        if v {
            bar.set_value.run(Some(my_value.clone()));
        } else if bar.value.get_untracked().as_deref() == Some(my_value.as_str()) {
            bar.set_value.run(None);
        }
    });
    let menu_ctx = MenuContextValue {
        open,
        on_open_change,
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{base_id}-content"),
        dir: bar.dir,
        open_with_keyboard: bar.open_with_keyboard,
    };
    let menubar_menu = StoredValue::new(MenubarMenuContextValue { value: menu_value });

    view! {
        <Popper>
            <Provider value=menu_ctx.clone()>
                <Provider value=menubar_menu.get_value()>
                    {children.with_value(|c| c())}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let bar = expect_context::<MenubarContextValue>();
    let menu = expect_context::<MenuContextValue>();
    let menubar_menu = expect_context::<MenubarMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let refs = use_composed_refs(vec![node_ref, menu.trigger_ref]);
    let content_id = StoredValue::new(menu.content_id.clone());
    let value = StoredValue::new(menubar_menu.value);

    view! {
        <PopperAnchor as_child=true>
            <Primitive element=html::button as_child=as_child node_ref=refs
                attr:r#type="button"
                attr:role="menuitem"
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || menu.open.get().to_string()
                attr:aria-controls=move || menu.open.get().then(|| content_id.get_value())
                attr:data-state=move || if menu.open.get() { "open" } else { "closed" }
                attr:data-vdl-menubar-trigger=""
                attr:data-menu-value=move || value.get_value()
                attr:data-disabled=move || disabled.get().then_some("")
                attr:aria-disabled=move || disabled.get().then_some("true")
                attr:tabindex=move || match bar.current_trigger.get() {
                    Some(current) if current != value.get_value() => "-1",
                    _ => "0",
                }
                on:focus=move |_| bar.current_trigger.set(Some(value.get_value()))
                on:click=move |event: web_sys::MouseEvent| {
                    // Enter and Space also produce a click; keyboard opening is handled in keydown.
                    if event.detail() > 0 && !disabled.get_untracked() {
                        menu.open_with_keyboard.set(false);
                        menu.on_open_change.run(!menu.open.get_untracked());
                    }
                }
                // Once a menu is open, moving over another trigger switches to its menu.
                on:pointerenter=move |event: web_sys::PointerEvent| {
                    if disabled.get_untracked() || event.pointer_type() == "touch" {
                        return;
                    }
                    if bar.value.get_untracked().is_some() && !menu.open.get_untracked() {
                        if let Some(trigger) = event
                            .current_target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            focus_item(&trigger);
                        }
                        menu.open_with_keyboard.set(false);
                        menu.on_open_change.run(true);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if disabled.get_untracked() {
                        return;
                    }
                    let rtl = bar.dir.get_untracked() == Direction::Rtl;
                    match event.key().as_str() {
                        "ArrowDown" | "ArrowUp" | "Enter" | " " => {
                            event.prevent_default();
                            menu.open_with_keyboard.set(true);
                            menu.on_open_change.run(true);
                        }
                        "ArrowRight" => {
                            event.prevent_default();
                            move_between_menus(&bar, Some(value.get_value()), !rtl);
                        }
                        "ArrowLeft" => {
                            event.prevent_default();
                            move_between_menus(&bar, Some(value.get_value()), rtl);
                        }
                        "Home" => {
                            event.prevent_default();
                            move_to_edge(&bar, true);
                        }
                        "End" => {
                            event.prevent_default();
                            move_to_edge(&bar, false);
                        }
                        _ => {}
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </PopperAnchor>
    }
}

/// The menu panel. As [`DropdownMenuContent`], plus: Left/Right switch to the neighbouring
/// menu (swapped in right-to-left) and closing the bar returns focus to this menu's trigger.
#[component]
pub fn MenubarContent(
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    /// Which side to position on: "top" | "right" | "bottom" | "left"
    #[prop(into, optional)]
    side: MaybeProp<String>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    /// Alignment along the side: "start" | "center" | "end"
    #[prop(into, optional)]
    align: MaybeProp<String>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_padding: MaybeProp<f64>,
    /// Class for the floating content element.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Inline style for the floating content element.
    #[prop(into, optional)]
    style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let bar = expect_context::<MenubarContextValue>();
    let menu = expect_context::<MenuContextValue>();
    let menubar_menu = expect_context::<MenubarMenuContextValue>();
    let trigger_ref = menu.trigger_ref;

    let on_key_down = Callback::new(move |event: web_sys::KeyboardEvent| {
        let rtl = bar.dir.get_untracked() == Direction::Rtl;
        match event.key().as_str() {
            "ArrowRight" => {
                event.prevent_default();
                move_between_menus(&bar, Some(menubar_menu.value.clone()), !rtl);
            }
            "ArrowLeft" => {
                event.prevent_default();
                move_between_menus(&bar, Some(menubar_menu.value.clone()), rtl);
            }
            _ => {}
        }
    });
    // The menu that replaces this one on a switch records the trigger as the place to return
    // to, so a plain "return to whatever had focus before" would land on a stale element.
    // Instead: when the whole bar closes, focus goes to this menu's trigger; on a switch the
    // focus is already where the next menu wants it.
    let on_close_auto_focus = Callback::new(move |event: web_sys::Event| {
        event.prevent_default();
        if bar.value.get_untracked().is_none()
            && let Some(trigger) = trigger_ref
                .get_untracked()
                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
        {
            focus_item(&trigger);
        }
    });

    view! {
        <DropdownMenuContent
            on_escape_key_down=Callback::new(move |e| {
                if let Some(cb) = on_escape_key_down {
                    cb.run(e);
                }
            })
            on_close_auto_focus=on_close_auto_focus
            on_pointer_down_outside=Callback::new(move |e| {
                if let Some(cb) = on_pointer_down_outside {
                    cb.run(e);
                }
            })
            on_key_down=on_key_down
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_padding=collision_padding
            class=class
            style=style
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|c| c())}
        </DropdownMenuContent>
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn triggers(bar: &MenubarContextValue) -> Vec<web_sys::HtmlElement> {
    let Some(root) = bar.root_ref.get_untracked() else {
        return vec![];
    };
    let Ok(nodes) = root.query_selector_all(TRIGGER_SELECTOR) else {
        return vec![];
    };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        .collect()
}

/// Focus the trigger `forward` (or backward) from the menu `from`. When a menu is open, its
/// neighbour opens too, with the first item focused.
fn move_between_menus(bar: &MenubarContextValue, from: Option<String>, forward: bool) {
    let list = triggers(bar);
    if list.is_empty() {
        return;
    }
    let idx = from.and_then(|v| {
        list.iter()
            .position(|t| t.get_attribute("data-menu-value").as_deref() == Some(v.as_str()))
    });
    let last = list.len() - 1;
    let target = match (idx, forward) {
        (Some(i), true) if i < last => i + 1,
        (Some(_), true) => {
            if bar.do_loop.get_untracked() {
                0
            } else {
                last
            }
        }
        (Some(i), false) if i > 0 => i - 1,
        (Some(_), false) => {
            if bar.do_loop.get_untracked() {
                last
            } else {
                0
            }
        }
        (None, true) => 0,
        (None, false) => last,
    };
    open_or_focus(bar, &list[target]);
}

fn move_to_edge(bar: &MenubarContextValue, first: bool) {
    let list = triggers(bar);
    let target = if first { list.first() } else { list.last() };
    if let Some(target) = target {
        open_or_focus(bar, target);
    }
}

fn open_or_focus(bar: &MenubarContextValue, trigger: &web_sys::HtmlElement) {
    // Read before focusing: moving focus makes the open menu dismiss itself (focus outside).
    let was_open = bar.value.get_untracked().is_some();
    // Focus the trigger first so the next menu's focus scope returns here when it closes.
    focus_item(trigger);
    if was_open && let Some(value) = trigger.get_attribute("data-menu-value") {
        bar.open_with_keyboard.set(true);
        bar.set_value.run(Some(value));
    }
}
