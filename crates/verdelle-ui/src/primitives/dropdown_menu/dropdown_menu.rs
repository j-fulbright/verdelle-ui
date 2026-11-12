use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::dismissable_layer::{
    use_dismissable_layer, use_dismissable_layer_with,
};
use crate::primitives::core::focus_scope::use_focus_scope;
use crate::primitives::core::id::use_id;
use crate::primitives::core::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, parse_align, parse_side,
};
use crate::primitives::core::portal::Portal;
use crate::primitives::core::presence::{PortalKeep, keep_portal_mounted, use_presence};
use crate::primitives::core::primitive::Primitive;
use floating_ui_leptos::Padding;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct MenuContextValue {
    pub(crate) open: Signal<bool>,
    pub(crate) on_open_change: Callback<bool>,
    pub(crate) trigger_ref: AnyNodeRef,
    pub(crate) content_id: String,
    pub(crate) dir: Signal<crate::primitives::core::direction::Direction>,
    /// Set when the menu was opened from the keyboard, so the first item gets focus.
    pub(crate) open_with_keyboard: RwSignal<bool>,
}

#[derive(Clone, Debug)]
pub(crate) struct MenuRadioGroupContextValue {
    pub(crate) value: Signal<Option<String>>,
    pub(crate) on_value_change: Callback<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct MenuItemCheckedContextValue {
    pub(crate) checked: Signal<bool>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct SubMenuContextValue {
    open: RwSignal<bool>,
    content_id: String,
    trigger_ref: AnyNodeRef,
    /// Pending close after the pointer left the sub trigger without entering the sub menu.
    leave_timer: RwSignal<Option<i32>>,
}

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<crate::primitives::core::direction::Direction>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let dir = Signal::derive(move || {
        dir.get()
            .unwrap_or(crate::primitives::core::direction::Direction::Ltr)
    });

    let (open_state, set_open) =
        crate::primitives::core::use_controllable_state::use_controllable_state(
            crate::primitives::core::use_controllable_state::UseControllableStateParams {
                prop: open,
                on_change: on_open_change.map(|cb| {
                    Callback::new(move |v: Option<bool>| {
                        if let Some(v) = v {
                            cb.run(v);
                        }
                    })
                }),
                default_prop: default_open,
            },
        );
    let open = Signal::derive(move || open_state.get().unwrap_or(false));
    let base_id = use_id(None).get_untracked();

    let context = MenuContextValue {
        open,
        on_open_change: Callback::new(move |v: bool| set_open.run(Some(v))),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-content", base_id),
        dir,
        open_with_keyboard: RwSignal::new(false),
    };

    let context = StoredValue::new(context);
    view! {
        <Popper>
            <Provider value=context.get_value()>
                {children.with_value(|c| c())}
            </Provider>
        </Popper>
    }
}

// ---------------------------------------------------------------------------
// Trigger
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    /// Also open with ArrowRight, for a menu that flies out to the right of its trigger.
    #[prop(into, optional)]
    arrow_right_opens: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let refs = use_composed_refs(vec![node_ref, ctx.trigger_ref]);
    let content_id = StoredValue::new(ctx.content_id.clone());

    view! {
        <PopperAnchor as_child=true>
            <Primitive element=html::button as_child=as_child node_ref=refs
                attr:r#type="button"
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || ctx.open.get().to_string()
                attr:aria-controls=move || ctx.open.get().then(|| content_id.get_value())
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                on:click=move |event: web_sys::MouseEvent| {
                    // Enter and Space also produce a click; keyboard opening is handled in keydown.
                    if event.detail() > 0 {
                        ctx.open_with_keyboard.set(false);
                        ctx.on_open_change.run(!ctx.open.get_untracked());
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    let opens = matches!(key.as_str(), "ArrowDown" | "ArrowUp" | "Enter" | " ")
                        || (key == "ArrowRight" && arrow_right_opens.get_untracked().unwrap_or(false));
                    if opens {
                        event.prevent_default();
                        ctx.open_with_keyboard.set(true);
                        ctx.on_open_change.run(true);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </PopperAnchor>
    }
}

// ---------------------------------------------------------------------------
// Portal
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(into, optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] _force_mount: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let ctx_for_portal = StoredValue::new(ctx.clone());
    let keep = PortalKeep::new();
    view! {
        <Provider value=keep>
        <Show when=move || ctx.open.get() || keep.get()>
            <Portal container=container container_ref=container_ref>
                <Provider value=ctx_for_portal.get_value()>
                    {children.with_value(|c| c())}
                </Provider>
            </Portal>
        </Show>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// Content
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    /// Called for every key press inside the menu, after the menu's own handling. A key that
    /// a nested sub menu handled does not reach it (sub menus stop those from bubbling).
    #[prop(into, optional)]
    on_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    /// Called as the menu closes, when focus would return to whatever had it before the menu
    /// opened. Call `prevent_default` on the event to place focus yourself.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    /// Which side to position on: "top" | "right" | "bottom" | "left"
    #[prop(into, optional)]
    side: MaybeProp<String>,
    /// Offset from the trigger (pixels).
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// Alignment along the side: "start" | "center" | "end"
    #[prop(into, optional)]
    align: MaybeProp<String>,
    /// Offset along the alignment axis (pixels).
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// Whether to flip/shift to avoid viewport collisions.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// Padding from viewport edge when avoiding collisions (pixels).
    #[prop(into, optional)]
    collision_padding: MaybeProp<f64>,
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

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("bottom".into())));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align = Signal::derive(move || parse_align(&align.get().unwrap_or("center".into())));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding =
        Signal::derive(move || Padding::All(collision_padding.get().unwrap_or(0.0)));

    let ctx = expect_context::<MenuContextValue>();
    // loop defaults to true; focus_menu_item always wraps (matching Radix default)
    let _do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));
    let present = Signal::derive(move || ctx.open.get());
    let presence = use_presence(present);
    keep_portal_mounted(presence.is_present);

    let focus_ref = use_focus_scope(
        Signal::derive(|| true),
        Signal::derive(|| true),
        None,
        on_close_auto_focus,
    );
    // A press on the trigger is not an "outside" interaction: the trigger's own click toggles
    // the menu, and dismissing here as well would reopen it.
    let trigger_ref = ctx.trigger_ref;
    let on_pointer_down_outside = Callback::new(move |event: web_sys::PointerEvent| {
        let on_trigger = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .zip(trigger_ref.get_untracked())
            .is_some_and(|(target, trigger)| trigger.contains(Some(&target)));
        if on_trigger {
            event.prevent_default();
        } else if let Some(cb) = on_pointer_down_outside {
            cb.run(event);
        }
    });
    let dismiss_ref = use_dismissable_layer(
        on_escape_key_down,
        Some(on_pointer_down_outside),
        None,
        None,
        Some(Callback::new(move |()| ctx.on_open_change.run(false))),
        Signal::derive(move || !ctx.open.get()),
    );
    let content_ref = AnyNodeRef::new();
    // `presence.node_ref` goes on the popper's own content element (below), the one the CSS
    // animates, not on the menu element inside it.
    let refs = use_composed_refs(vec![node_ref, focus_ref, dismiss_ref, content_ref]);

    // Move focus into the menu once it is mounted: to the first item when it was opened from
    // the keyboard, otherwise to the menu itself so arrow keys work. Deferred one tick so it
    // runs after the focus scope has recorded the trigger as the element to return to.
    Effect::new(move |_| {
        if let Some(menu) = content_ref.get() {
            let first_item = ctx.open_with_keyboard.get_untracked();
            set_timeout_once(0, move || {
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                let target = if first_item {
                    menu_items(&menu).into_iter().next()
                } else {
                    None
                };
                match target {
                    Some(item) => {
                        let _ = item.focus_with_options(&options);
                    }
                    None => {
                        if let Ok(el) = menu.clone().dyn_into::<web_sys::HtmlElement>() {
                            let _ = el.focus_with_options(&options);
                        }
                    }
                }
            });
        }
    });

    let content_id = StoredValue::new(ctx.content_id.clone());
    let search_buffer: RwSignal<String> = RwSignal::new(String::new());
    let search_timer: RwSignal<Option<i32>> = RwSignal::new(None);

    view! {
        <Show when=move || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                class=class
                style=style
                node_ref=presence.node_ref
            >
                <Primitive element=html::div as_child=as_child node_ref=refs
                    attr:id=content_id.get_value()
                    attr:role="menu"
                    attr:aria-orientation="vertical"
                    attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                    attr:tabindex="-1"
                    on:keydown=move |event: KeyboardEvent| {
                        match event.key().as_str() {
                            "Tab" => { event.prevent_default(); }
                            "ArrowDown" | "PageDown" => { event.prevent_default(); focus_menu_item(&event, true); }
                            "ArrowUp" | "PageUp" => { event.prevent_default(); focus_menu_item(&event, false); }
                            "Home" => { event.prevent_default(); focus_menu_item_edge(&event, true); }
                            "End" => { event.prevent_default(); focus_menu_item_edge(&event, false); }
                            key if key.len() == 1
                                && key != " "
                                && !event.ctrl_key()
                                && !event.meta_key() =>
                            {
                                handle_typeahead(&event, key, search_buffer, search_timer);
                            }
                            _ => {}
                        }
                        if let Some(cb) = on_key_down {
                            cb.run(event);
                        }
                    }
                    // Leaving the items hands focus back to the menu, clearing the highlight.
                    on:pointerleave=move |event: web_sys::PointerEvent| {
                        if let Some(menu) = event
                            .current_target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            let _ = menu.focus();
                        }
                    }
                >
                    {children.with_value(|c| c())}
                </Primitive>
            </PopperContent>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Group
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Label
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Item
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let handle_select = move || {
        if !disabled.get_untracked() {
            if let Some(on_select) = on_select {
                on_select.run(());
            }
            ctx.on_open_change.run(false);
        }
    };

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="menuitem"
            attr:data-disabled=move || disabled.get().then_some("")
            attr:aria-disabled=move || disabled.get().then_some("true")
            attr:tabindex="-1"
            on:click=move |_| handle_select()
            on:pointermove=move |event: web_sys::PointerEvent| {
                focus_on_hover(&event, disabled.get_untracked())
            }
            on:keydown=move |event: KeyboardEvent| {
                if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get_untracked() {
                    event.prevent_default();
                    handle_select();
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// CheckboxItem
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    /// Close the menu after the item is toggled. Off by default, as in Base UI.
    #[prop(into, optional)]
    close_on_select: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let item_checked_ctx = MenuItemCheckedContextValue { checked };

    view! {
        <Provider value=item_checked_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menuitemcheckbox"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || if checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:aria-disabled=move || disabled.get().then_some("true")
                attr:tabindex="-1"
                on:pointermove=move |event: web_sys::PointerEvent| {
                    focus_on_hover(&event, disabled.get_untracked())
                }
                on:click=move |_| {
                    if !disabled.get_untracked() {
                        if let Some(cb) = on_checked_change { cb.run(!checked.get_untracked()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        if close_on_select.get_untracked().unwrap_or(false) {
                            ctx.on_open_change.run(false);
                        }
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get_untracked() {
                        event.prevent_default();
                        if let Some(cb) = on_checked_change { cb.run(!checked.get_untracked()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        if close_on_select.get_untracked().unwrap_or(false) {
                            ctx.on_open_change.run(false);
                        }
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// RadioGroup + RadioItem
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = Signal::derive(move || value.get());
    let radio_ctx = MenuRadioGroupContextValue {
        value,
        on_value_change: Callback::new(move |v: String| {
            if let Some(cb) = on_value_change {
                cb.run(v);
            }
        }),
    };

    view! {
        <Provider value=radio_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn DropdownMenuRadioItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    /// Close the menu after the item is chosen. Off by default, as in Base UI.
    #[prop(into, optional)]
    close_on_select: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let radio_ctx = expect_context::<MenuRadioGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked =
        Signal::derive(move || radio_ctx.value.get().as_deref() == Some(item_value.as_str()));
    let item_checked_ctx = MenuItemCheckedContextValue { checked };

    view! {
        <Provider value=item_checked_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menuitemradio"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || if checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:aria-disabled=move || disabled.get().then_some("true")
                attr:tabindex="-1"
                on:pointermove=move |event: web_sys::PointerEvent| {
                    focus_on_hover(&event, disabled.get_untracked())
                }
                on:click=move |_| {
                    if !disabled.get_untracked() {
                        radio_ctx.on_value_change.run(item_value_click.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        if close_on_select.get_untracked().unwrap_or(false) {
                            ctx.on_open_change.run(false);
                        }
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get_untracked() {
                        event.prevent_default();
                        radio_ctx.on_value_change.run(value.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        if close_on_select.get_untracked().unwrap_or(false) {
                            ctx.on_open_change.run(false);
                        }
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// ItemIndicator
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let checked_ctx = expect_context::<MenuItemCheckedContextValue>();

    view! {
        <Show when=move || force_mount.get() || checked_ctx.checked.get()>
            <Primitive element=html::span as_child=as_child node_ref=node_ref
                attr:data-state=move || if checked_ctx.checked.get() { "checked" } else { "unchecked" }
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </Primitive>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Separator
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="separator"
            attr:aria-orientation="horizontal"
        >
            {""}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Arrow
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <PopperArrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </PopperArrow>
    }
}

// ---------------------------------------------------------------------------
// Sub / SubTrigger / SubContent
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(false));

    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_state.set(o);
        }
    });
    Effect::new(move |_| {
        if let Some(cb) = on_open_change {
            cb.run(open_state.get());
        }
    });

    let base_id = use_id(None).get_untracked();
    let sub_ctx = SubMenuContextValue {
        open: open_state,
        content_id: format!("{}-sub", base_id),
        trigger_ref: AnyNodeRef::new(),
        leave_timer: RwSignal::new(None),
    };

    view! {
        <Provider value=sub_ctx>
            <Popper>
                {children.with_value(|c| c())}
            </Popper>
        </Provider>
    }
}

#[component]
pub fn DropdownMenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let menu_ctx = expect_context::<MenuContextValue>();
    let sub_ctx = expect_context::<SubMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let refs = use_composed_refs(vec![node_ref, sub_ctx.trigger_ref]);
    let sub_content_id = StoredValue::new(sub_ctx.content_id.clone());

    view! {
        <PopperAnchor as_child=MaybeProp::derive(|| Some(true))>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:role="menuitem"
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || sub_ctx.open.get().to_string()
                attr:aria-controls=move || sub_ctx.open.get().then(|| sub_content_id.get_value())
                attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:tabindex="-1"
                attr:aria-disabled=move || disabled.get().then_some("true")
                on:click=move |_| {
                    if !disabled.get_untracked() { sub_ctx.open.set(true); }
                }
                on:pointermove=move |event: web_sys::PointerEvent| {
                    focus_on_hover(&event, disabled.get_untracked())
                }
                on:pointerenter=move |_| {
                    clear_leave_timer(sub_ctx.leave_timer);
                    if !disabled.get_untracked() { sub_ctx.open.set(true); }
                }
                // Leaving the trigger closes the sub menu soon after, unless the pointer
                // reaches the sub menu (its pointerenter cancels this) or comes back.
                on:pointerleave=move |_| {
                    clear_leave_timer(sub_ctx.leave_timer);
                    let id = set_timeout_once(150, move || {
                        sub_ctx.leave_timer.set(None);
                        sub_ctx.open.set(false);
                    });
                    sub_ctx.leave_timer.set(id);
                }
                on:keydown=move |event: KeyboardEvent| {
                    let open_key = if menu_ctx.dir.get_untracked() == crate::primitives::core::direction::Direction::Rtl { "ArrowLeft" } else { "ArrowRight" };
                    if (event.key() == open_key || matches!(event.key().as_str(), "Enter" | " "))
                        && !disabled.get_untracked()
                    {
                        event.prevent_default();
                        event.stop_propagation();
                        sub_ctx.open.set(true);
                        // Focus the first item once the sub menu is mounted.
                        let sub_id = sub_content_id.get_value();
                        set_timeout_once(0, move || {
                            if let Some(sub) = web_sys::window()
                                .and_then(|w| w.document())
                                .and_then(|d| d.get_element_by_id(&sub_id))
                                && let Some(item) = menu_items(&sub).into_iter().next()
                            {
                                let options = web_sys::FocusOptions::new();
                                options.set_prevent_scroll(true);
                                let _ = item.focus_with_options(&options);
                            }
                        });
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </PopperAnchor>
    }
}

#[component]
pub fn DropdownMenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    /// Offset along the alignment axis (pixels).
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
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
    let menu_ctx = expect_context::<MenuContextValue>();
    let sub_ctx = expect_context::<SubMenuContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let present = Signal::derive(move || force_mount.get() || sub_ctx.open.get());
    let presence = use_presence(present);

    // Default sub-menu side: right for LTR, left for RTL
    let popper_side = Signal::derive(move || {
        if menu_ctx.dir.get() == crate::primitives::core::direction::Direction::Rtl {
            parse_side("left")
        } else {
            parse_side("right")
        }
    });
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align = Signal::derive(|| parse_align("start"));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(|| true);
    let popper_collision_padding = Signal::derive(|| Padding::All(0.0));

    // Focus staying on the trigger (or moving to a sibling item) must not close the sub menu:
    // it opens on hover while the trigger still has focus. Pointer presses outside still do.
    // Escape closes the whole menu, not just the sub menu (as Radix does).
    let dismiss_ref = use_dismissable_layer_with(
        Some(Callback::new(move |_| menu_ctx.on_open_change.run(false))),
        None,
        None,
        None,
        Some(Callback::new(move |()| sub_ctx.open.set(false))),
        Signal::derive(move || !sub_ctx.open.get()),
        false,
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, dismiss_ref]);

    // Safe triangle: when the pointer leaves the sub-content, compute a triangle
    // from the exit point to the two far corners of the sub-trigger. Keep the
    // sub-menu open while the pointer remains inside that triangle (checked via
    // document-level pointermove). Close when the pointer exits the triangle or
    // after a 400ms safety timeout.
    let grace_timer: RwSignal<Option<i32>> = RwSignal::new(None);
    // Store the safe triangle polygon: 3 points [(x,y); 3]
    let safe_triangle: RwSignal<Option<[(f64, f64); 3]>> = RwSignal::new(None);

    let clear_grace = move || {
        if let Some(id) = grace_timer.get_untracked()
            && let Some(w) = web_sys::window()
        {
            w.clear_timeout_with_handle(id);
        }
        grace_timer.set(None);
        safe_triangle.set(None);
    };

    on_cleanup(clear_grace);

    view! {
        <Show when=move || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                class=class
                style=style
                as_child=as_child
                node_ref=refs
                attr:id=sub_ctx.content_id.clone()
                attr:role="menu"
                attr:aria-orientation="vertical"
                attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                on:keydown=move |event: KeyboardEvent| {
                    let close_key = if menu_ctx.dir.get_untracked() == crate::primitives::core::direction::Direction::Rtl { "ArrowRight" } else { "ArrowLeft" };
                    // Handled keys must not bubble: this element sits inside the parent menu's
                    // DOM, whose own arrow-key handling would move focus a second time.
                    match event.key().as_str() {
                        "ArrowDown" => { event.prevent_default(); event.stop_propagation(); focus_menu_item(&event, true); }
                        "ArrowUp" => { event.prevent_default(); event.stop_propagation(); focus_menu_item(&event, false); }
                        "Home" => { event.prevent_default(); event.stop_propagation(); focus_menu_item_edge(&event, true); }
                        "End" => { event.prevent_default(); event.stop_propagation(); focus_menu_item_edge(&event, false); }
                        k if k == close_key => {
                            event.prevent_default();
                            event.stop_propagation();
                            sub_ctx.open.set(false);
                            if let Some(trigger) = sub_ctx.trigger_ref.get_untracked()
                                && let Ok(el) = trigger.dyn_into::<web_sys::HtmlElement>()
                            {
                                let _ = el.focus();
                            }
                        }
                        "Escape" => { sub_ctx.open.set(false); }
                        _ => {}
                    }
                }
                on:pointerenter=move |_| {
                    clear_leave_timer(sub_ctx.leave_timer);
                    clear_grace();
                }
                on:pointerleave=move |event: web_sys::PointerEvent| {
                    clear_grace();
                    let px = event.client_x() as f64;
                    let py = event.client_y() as f64;

                    // Compute safe triangle: from pointer exit point to the two
                    // far corners of the sub-trigger (the side facing the content).
                    let triangle = sub_ctx.trigger_ref.get().map(|trigger| {
                        let r = trigger.get_bounding_client_rect();
                        let is_rtl = menu_ctx.dir.get() == crate::primitives::core::direction::Direction::Rtl;
                        // Content opens to the right (LTR) or left (RTL) of trigger.
                        // The two "far" corners are the trigger edge facing the content.
                        if is_rtl {
                            // Content is to the left, trigger's left edge faces content
                            [(px, py), (r.left(), r.top()), (r.left(), r.bottom())]
                        } else {
                            // Content is to the right, trigger's right edge faces content
                            [(px, py), (r.right(), r.top()), (r.right(), r.bottom())]
                        }
                    });

                    safe_triangle.set(triangle);

                    // Install document-level pointermove to check if pointer is in triangle.
                    // If pointer exits triangle, close immediately.
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let handler = web_sys::wasm_bindgen::closure::Closure::<dyn Fn(web_sys::PointerEvent)>::new(
                            move |ev: web_sys::PointerEvent| {
                                let mx = ev.client_x() as f64;
                                let my = ev.client_y() as f64;
                                // The listener outlives the sub menu (it is never removed), so
                                // the signal may be disposed by now: then there is nothing to do.
                                if let Some(tri) = safe_triangle.try_get_untracked().flatten()
                                    && !point_in_triangle(mx, my, tri) {
                                        // Pointer left the safe triangle — close immediately
                                        clear_grace();
                                        sub_ctx.open.set(false);
                                    }
                            },
                        );
                        let _ = document.add_event_listener_with_callback(
                            "pointermove",
                            handler.as_ref().unchecked_ref(),
                        );
                        // Remove listener when grace period ends or sub-menu re-entered
                        // (handled via clear_grace + cleanup). Leak is bounded by the
                        // 400ms timeout below which always fires.
                        handler.forget();
                    }

                    // Safety timeout: close after 400ms regardless
                    let id = web_sys::window().and_then(|w| {
                        w.set_timeout_with_callback_and_timeout_and_arguments_0(
                            web_sys::wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                                if safe_triangle.try_get_untracked().flatten().is_some() {
                                    safe_triangle.set(None);
                                    sub_ctx.open.set(false);
                                }
                            })
                            .into_js_value()
                            .unchecked_ref(),
                            400,
                        )
                        .ok()
                    });
                    grace_timer.set(id);
                }
            >
                {children.with_value(|c| c())}
            </PopperContent>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The enabled items that belong to this menu: not the ones in a sub menu nested inside it.
pub(crate) fn menu_items(container: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let Ok(nodes) = container.query_selector_all(
        "[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])",
    ) else {
        return vec![];
    };
    let mut items = vec![];
    for i in 0..nodes.length() {
        let Some(node) = nodes.item(i) else { continue };
        let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() else {
            continue;
        };
        let owner = el.closest("[role='menu']").ok().flatten();
        if owner.as_ref() == Some(container) {
            items.push(el);
        }
    }
    items
}

fn current_container(event: &KeyboardEvent) -> Option<web_sys::Element> {
    event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
}

pub(crate) fn focus_item(item: &web_sys::HtmlElement) {
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = item.focus_with_options(&options);
}

/// Move focus to the next (or previous) enabled item, wrapping at the ends.
fn focus_menu_item(event: &KeyboardEvent, forward: bool) {
    let Some(container) = current_container(event) else {
        return;
    };
    let items = menu_items(&container);
    if items.is_empty() {
        return;
    }
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let idx = active.as_ref().and_then(|a| {
        items
            .iter()
            .position(|item| AsRef::<web_sys::Element>::as_ref(item) == a)
    });
    let next = match (forward, idx) {
        (true, Some(i)) => (i + 1) % items.len(),
        (true, None) => 0,
        (false, Some(i)) => (i + items.len() - 1) % items.len(),
        (false, None) => items.len() - 1,
    };
    focus_item(&items[next]);
}

/// Focus the first or last enabled item.
fn focus_menu_item_edge(event: &KeyboardEvent, first: bool) {
    let Some(container) = current_container(event) else {
        return;
    };
    let items = menu_items(&container);
    let item = if first { items.first() } else { items.last() };
    if let Some(item) = item {
        focus_item(item);
    }
}

/// Highlight (focus) the item under the pointer.
fn focus_on_hover(event: &web_sys::PointerEvent, disabled: bool) {
    if disabled {
        return;
    }
    if let Some(item) = event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let already = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element())
            .is_some_and(|a| &a == AsRef::<web_sys::Element>::as_ref(&item));
        if !already {
            focus_item(&item);
        }
    }
}

fn clear_leave_timer(timer: RwSignal<Option<i32>>) {
    if let Some(id) = timer.get_untracked()
        && let Some(w) = web_sys::window()
    {
        w.clear_timeout_with_handle(id);
    }
    timer.set(None);
}

/// Run `f` once after `ms` milliseconds. Returns the timer id.
pub(crate) fn set_timeout_once(ms: i32, f: impl FnOnce() + 'static) -> Option<i32> {
    let closure = web_sys::wasm_bindgen::closure::Closure::once_into_js(f);
    web_sys::window().and_then(|w| {
        w.set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), ms)
            .ok()
    })
}

/// Typeahead search: accumulate typed characters and focus the first item whose text starts with them.
fn handle_typeahead(
    event: &KeyboardEvent,
    key: &str,
    search_buffer: RwSignal<String>,
    search_timer: RwSignal<Option<i32>>,
) {
    let Some(container) = current_container(event) else {
        return;
    };

    // Restart the 1-second reset.
    if let Some(id) = search_timer.get_untracked()
        && let Some(w) = web_sys::window()
    {
        w.clear_timeout_with_handle(id);
    }
    search_buffer.update(|buf| buf.push_str(key));
    search_timer.set(set_timeout_once(1000, move || {
        search_buffer.set(String::new());
    }));

    let search = search_buffer.get_untracked().to_lowercase();
    if let Some(item) = menu_items(&container).into_iter().find(|item| {
        item.text_content()
            .is_some_and(|text| text.trim().to_lowercase().starts_with(&search))
    }) {
        focus_item(&item);
    }
}

/// Test if point (px, py) is inside triangle [(x1,y1), (x2,y2), (x3,y3)]
/// using the sign-of-cross-product (barycentric) method.
fn point_in_triangle(px: f64, py: f64, tri: [(f64, f64); 3]) -> bool {
    let [(x1, y1), (x2, y2), (x3, y3)] = tri;

    let d1 = (px - x2) * (y1 - y2) - (x1 - x2) * (py - y2);
    let d2 = (px - x3) * (y2 - y3) - (x2 - x3) * (py - y3);
    let d3 = (px - x1) * (y3 - y1) - (x3 - x1) * (py - y1);

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}
