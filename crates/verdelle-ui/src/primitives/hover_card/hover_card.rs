use crate::primitives::core::popper::{
    Popper, PopperArrow, PopperContent, Sticky, UpdatePositionStrategy, parse_align, parse_side,
};
use crate::primitives::core::portal::Portal;
use crate::primitives::core::presence::{PortalKeep, keep_portal_mounted, use_presence};
use crate::primitives::core::primitive::Primitive;
use floating_ui_leptos::Padding;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

fn window() -> web_sys::Window {
    web_sys::window().expect("should have a Window")
}

/// Run `f` once after `ms` milliseconds; returns the timer id. The closure is built when
/// the timer is set, in the browser: building one while rendering would panic on the server.
fn set_timeout(f: impl FnOnce() + 'static, ms: i32) -> i32 {
    let callback = Closure::once_into_js(f);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), ms)
        .expect("set_timeout should succeed")
}

fn clear_timeout(id: i32) {
    window().clear_timeout_with_handle(id);
}

#[derive(Clone, Debug)]
struct HoverCardContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    content_id: String,
    open_delay: Signal<i32>,
    close_delay: Signal<i32>,
    open_timer_id: RwSignal<Option<i32>>,
    close_timer_id: RwSignal<Option<i32>>,
}

impl HoverCardContextValue {
    fn clear_open_timer(&self) {
        if let Some(id) = self.open_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
            self.open_timer_id.set(None);
        }
    }

    fn clear_close_timer(&self) {
        if let Some(id) = self.close_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
            self.close_timer_id.set(None);
        }
    }

    /// Open after the open delay (at once when it is zero), cancelling a pending close.
    fn schedule_open(&self) {
        self.clear_close_timer();
        self.clear_open_timer();
        if self.open.get_untracked() {
            return;
        }
        let delay = self.open_delay.get_untracked();
        if delay <= 0 {
            self.on_open.run(());
        } else {
            let on_open = self.on_open;
            let timer = self.open_timer_id;
            let id = set_timeout(
                move || {
                    timer.try_set(None);
                    on_open.run(());
                },
                delay,
            );
            self.open_timer_id.set(Some(id));
        }
    }

    /// Close after the close delay (at once when it is zero), cancelling a pending open.
    fn schedule_close(&self) {
        self.clear_open_timer();
        self.clear_close_timer();
        let delay = self.close_delay.get_untracked();
        if delay <= 0 {
            self.on_close.run(());
        } else {
            let on_close = self.on_close;
            let timer = self.close_timer_id;
            let id = set_timeout(
                move || {
                    timer.try_set(None);
                    on_close.run(());
                },
                delay,
            );
            self.close_timer_id.set(Some(id));
        }
    }

    fn close_now(&self) {
        self.clear_open_timer();
        self.clear_close_timer();
        self.on_close.run(());
    }
}

#[component]
pub fn HoverCard(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional, default = 700.into())] open_delay: MaybeProp<i32>,
    #[prop(into, optional, default = 300.into())] close_delay: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open_delay = Signal::derive(move || open_delay.get().unwrap_or(700));
    let close_delay = Signal::derive(move || close_delay.get().unwrap_or(300));

    let (open_state, set_open) =
        crate::primitives::core::use_controllable_state::use_controllable_state(
            crate::primitives::core::use_controllable_state::UseControllableStateParams {
                prop: open,
                on_change: on_open_change.map(|cb| {
                    Callback::new(move |value: Option<bool>| {
                        if let Some(value) = value {
                            cb.run(value);
                        }
                    })
                }),
                default_prop: default_open,
            },
        );
    let open = Signal::derive(move || open_state.get().unwrap_or(false));
    let base_id = crate::primitives::core::id::use_id(None).get_untracked();
    let open_timer_id: RwSignal<Option<i32>> = RwSignal::new(None);
    let close_timer_id: RwSignal<Option<i32>> = RwSignal::new(None);

    let context_value = HoverCardContextValue {
        open,
        on_open: Callback::new(move |()| set_open.run(Some(true))),
        on_close: Callback::new(move |()| set_open.run(Some(false))),
        content_id: format!("{}-hovercard", base_id),
        open_delay,
        close_delay,
        open_timer_id,
        close_timer_id,
    };

    // Clean up timers on unmount
    on_cleanup(move || {
        if let Some(id) = open_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
        }
        if let Some(id) = close_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
        }
    });

    let ctx = StoredValue::new(context_value.clone());

    view! {
        <Provider value=context_value>
            <Popper>
                {move || ctx.with_value(|_| children.with_value(|children| children()))}
            </Popper>
        </Provider>
    }
}

#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<HoverCardContextValue>();
    let composed_refs = crate::primitives::core::compose_refs::use_composed_refs(vec![
        node_ref,
        crate::primitives::core::popper::use_popper_anchor_ref(),
    ]);

    let ctx_enter = context.clone();
    let ctx_leave = context.clone();
    let ctx_focus = context.clone();
    let ctx_blur = context.clone();
    let content_id = StoredValue::new(context.content_id.clone());

    view! {
        <Primitive
            element=html::a
            as_child=as_child
            node_ref=composed_refs
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            attr:aria-expanded=move || context.open.get().to_string()
            attr:aria-controls=move || context.open.get().then(|| content_id.get_value())
            // Hover cards are not for touch: a tap should just follow the link.
            on:pointerenter=move |e: leptos::ev::PointerEvent| {
                if e.pointer_type() != "touch" {
                    ctx_enter.schedule_open();
                }
            }
            on:pointerleave=move |e: leptos::ev::PointerEvent| {
                if e.pointer_type() != "touch" {
                    ctx_leave.schedule_close();
                }
            }
            // Only keyboard focus opens it: a press focuses the trigger too, and that must
            // not reopen a card the pointer just closed.
            on:focus=move |e: leptos::ev::FocusEvent| {
                let visible = e
                    .current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .and_then(|el| el.matches(":focus-visible").ok())
                    .unwrap_or(true);
                if visible {
                    ctx_focus.clear_close_timer();
                    ctx_focus.clear_open_timer();
                    ctx_focus.on_open.run(());
                }
            }
            on:blur=move |_| ctx_blur.close_now()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn HoverCardPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<HoverCardContextValue>();

    let context_for_portal = StoredValue::new(context.clone());
    let keep = PortalKeep::new();
    view! {
        <Provider value=keep>
        <Show when=move || context.open.get() || keep.get()>
            <Portal container=container>
                <Provider value=context_for_portal.get_value()>
                    {children.with_value(|children| children())}
                </Provider>
            </Portal>
        </Show>
        </Provider>
    }
}

#[component]
pub fn HoverCardContent(
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
    /// Per-side padding from viewport edge. Overrides `collision_padding` when set.
    #[prop(into, optional)]
    collision_padding_object: MaybeProp<Padding>,
    #[prop(into, optional)] collision_boundary: MaybeProp<SendWrapper<Vec<web_sys::Element>>>,
    #[prop(into, optional)] sticky: MaybeProp<String>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] update_position_strategy: MaybeProp<String>,
    /// Class for the content element (not the positioning wrapper, which `attr:` reaches).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Inline style for the content element.
    #[prop(into, optional)]
    style: MaybeProp<String>,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("bottom".into())));
    let popper_align = Signal::derive(move || parse_align(&align.get().unwrap_or("center".into())));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding = Signal::derive(move || {
        collision_padding_object
            .get()
            .unwrap_or_else(|| Padding::All(collision_padding.get().unwrap_or(0.0)))
    });
    let popper_collision_boundary = Signal::derive(move || {
        collision_boundary
            .get()
            .unwrap_or_else(|| SendWrapper::new(vec![]))
    });
    let popper_sticky = Signal::derive(move || match sticky.get().as_deref() {
        Some("always") => Sticky::Always,
        _ => Sticky::Partial,
    });
    let popper_hide_when_detached =
        Signal::derive(move || hide_when_detached.get().unwrap_or(false));
    let popper_update_position_strategy =
        Signal::derive(move || match update_position_strategy.get().as_deref() {
            Some("always") => UpdatePositionStrategy::Always,
            _ => UpdatePositionStrategy::Optimized,
        });

    let context = expect_context::<HoverCardContextValue>();

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);
    keep_portal_mounted(presence.is_present);

    // Fire auto-focus callbacks when content opens/closes.
    let prev_open = RwSignal::new(false);
    Effect::new(move |_| {
        let is_open = context.open.get();
        if is_open && !prev_open.get_untracked() {
            if let Some(cb) = on_open_auto_focus {
                cb.run(web_sys::Event::new("focusin").unwrap());
            }
        } else if !is_open
            && prev_open.get_untracked()
            && let Some(cb) = on_close_auto_focus
        {
            cb.run(web_sys::Event::new("focusout").unwrap());
        }
        prev_open.set(is_open);
    });

    // Escape and a press outside close it at once. Focus leaving does not dismiss: `focusin`
    // cannot be cancelled, and the trigger's own blur already closes the card.
    let ctx_dismiss = context.clone();
    let dismiss_ref = crate::primitives::core::dismissable_layer::use_dismissable_layer_with(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| ctx_dismiss.close_now())),
        Signal::derive(move || !context.open.get()),
        false,
    );
    let composed_refs = crate::primitives::core::compose_refs::use_composed_refs(vec![
        node_ref,
        presence.node_ref,
        dismiss_ref,
    ]);

    let content_id = StoredValue::new(context.content_id.clone());
    let ctx_stored = StoredValue::new(context.clone());

    view! {
        <Show when=move || force_mount.get() || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                collision_boundary=popper_collision_boundary
                sticky=popper_sticky
                hide_when_detached=popper_hide_when_detached
                update_position_strategy=popper_update_position_strategy
                class=class
                style=style
                node_ref=composed_refs
                as_child=as_child
                attr:id=content_id.get_value()
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                on:pointerenter=move |e: leptos::ev::PointerEvent| {
                    if e.pointer_type() != "touch" {
                        // Moving from the trigger onto the card keeps it open.
                        ctx_stored.with_value(|c| {
                            c.clear_close_timer();
                            c.clear_open_timer();
                        });
                    }
                }
                on:pointerleave=move |e: leptos::ev::PointerEvent| {
                    if e.pointer_type() != "touch" {
                        ctx_stored.with_value(|c| c.schedule_close());
                    }
                }
            >
                {children.with_value(|children| children())}
            </PopperContent>
        </Show>
    }
}

#[component]
pub fn HoverCardArrow(
    #[prop(into, optional, default=10.0.into())] width: MaybeProp<f64>,
    #[prop(into, optional, default=5.0.into())] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <PopperArrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperArrow>
    }
}
