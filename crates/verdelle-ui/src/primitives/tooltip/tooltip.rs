use crate::primitives::core::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, Sticky, UpdatePositionStrategy, parse_align,
    parse_side,
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

thread_local! {
    /// The close callback of the tooltip that is open now, so opening another one closes it
    /// (like Radix, only one tooltip shows at a time, whatever opened each: hover, focus or both).
    static OPEN_TOOLTIP: std::cell::RefCell<Option<(String, Callback<()>)>> =
        const { std::cell::RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct TooltipContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    trigger_ref: AnyNodeRef,
    content_id: String,
    delay_duration: Signal<i32>,
    open_timer_id: RwSignal<Option<i32>>,
    was_instant_open: RwSignal<bool>,
    /// Whether the content stays open while the pointer moves from the trigger onto it.
    hoverable: Signal<bool>,
    /// The pending delayed close after the pointer left the trigger of a hoverable tooltip.
    close_timer_id: RwSignal<Option<i32>>,
}

/// How long a hoverable tooltip waits after the pointer leaves the trigger, so the pointer can
/// cross the gap to the content.
const HOVER_CLOSE_DELAY_MS: i32 = 120;

impl TooltipContextValue {
    /// Cancel a pending delayed close.
    fn cancel_close(&self) {
        if let Some(id) = self.close_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
            self.close_timer_id.set(None);
        }
    }

    #[allow(dead_code)]
    fn clear_open_timer(&self) {
        if let Some(id) = self.open_timer_id.get_untracked() {
            clear_timeout(id);
            self.open_timer_id.set(None);
        }
    }
}

#[derive(Clone, Debug)]
struct TooltipProviderContextValue {
    delay_duration: Signal<i32>,
    skip_delay_duration: Signal<i32>,
    /// Timestamp (ms) of the last tooltip close, used to skip delays for rapid hover.
    last_close_time: RwSignal<f64>,
}

/// Global tooltip provider — manages delay behavior.
#[component]
pub fn TooltipProvider(
    #[prop(into, optional, default = 700.into())] delay_duration: MaybeProp<i32>,
    #[prop(into, optional, default = 300.into())] skip_delay_duration: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let provider_ctx = TooltipProviderContextValue {
        delay_duration: Signal::derive(move || delay_duration.get().unwrap_or(700)),
        skip_delay_duration: Signal::derive(move || skip_delay_duration.get().unwrap_or(300)),
        last_close_time: RwSignal::new(0.0),
    };

    view! {
        <Provider value=provider_ctx>
            {children.with_value(|children| children())}
        </Provider>
    }
}

#[component]
pub fn Tooltip(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] delay_duration: MaybeProp<i32>,
    /// Keep the tooltip open while the pointer is over its content (default true, as Radix).
    #[prop(into, optional)]
    hoverable: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let hoverable = Signal::derive(move || hoverable.get().unwrap_or(true));
    let provider_ctx = use_context::<TooltipProviderContextValue>();
    let provider_for_base_delay = provider_ctx.clone();
    let delay_duration = Signal::derive(move || {
        delay_duration.get().unwrap_or_else(|| {
            provider_for_base_delay
                .as_ref()
                .map(|ctx| ctx.delay_duration.get())
                .unwrap_or(700)
        })
    });

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

    // Compute effective delay: skip if within provider's skip_delay_duration of last close
    let provider_for_delay = provider_ctx.clone();
    let effective_delay = Signal::derive(move || {
        if let Some(ref pctx) = provider_for_delay {
            let skip_dur = pctx.skip_delay_duration.get() as f64;
            let last_close = pctx.last_close_time.get();
            let now = window().performance().map(|p| p.now()).unwrap_or(0.0);
            if last_close > 0.0 && (now - last_close) < skip_dur {
                return 0;
            }
        }
        delay_duration.get()
    });

    let provider_for_close = provider_ctx.clone();
    let registry_id = base_id.clone();
    let on_close_cb = Callback::new(move |()| {
        let was_open = open.get_untracked();
        OPEN_TOOLTIP.with(|slot| {
            let mut slot = slot.borrow_mut();
            if slot.as_ref().is_some_and(|(id, _)| *id == registry_id) {
                *slot = None;
            }
        });
        set_open.run(Some(false));
        // Only a tooltip that was actually shown starts the "skip the delay" window.
        if was_open && let Some(ref pctx) = provider_for_close {
            let now = window().performance().map(|p| p.now()).unwrap_or(0.0);
            pctx.last_close_time.set(now);
        }
    });

    let was_instant_open = RwSignal::new(false);

    let open_id = base_id.clone();
    let context_value = TooltipContextValue {
        open,
        on_open: Callback::new(move |()| {
            was_instant_open.set(effective_delay.get_untracked() == 0);
            // Close the tooltip that is open elsewhere, then take its place.
            let previous = OPEN_TOOLTIP.with(|slot| {
                slot.borrow_mut()
                    .replace((open_id.clone(), on_close_cb))
                    .filter(|(id, _)| *id != open_id)
            });
            if let Some((_, close)) = previous {
                close.try_run(());
            }
            set_open.run(Some(true));
        }),
        on_close: on_close_cb,
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-tooltip", base_id),
        delay_duration: effective_delay,
        open_timer_id,
        was_instant_open,
        hoverable,
        close_timer_id: RwSignal::new(None),
    };

    // Clean up timer on unmount
    on_cleanup({
        let close_timer = context_value.close_timer_id;
        let timer_id = context_value.open_timer_id;
        let cleanup_id = base_id.clone();
        move || {
            OPEN_TOOLTIP.with(|slot| {
                let mut slot = slot.borrow_mut();
                if slot.as_ref().is_some_and(|(id, _)| *id == cleanup_id) {
                    *slot = None;
                }
            });
            if let Some(id) = timer_id.try_get_untracked().flatten() {
                clear_timeout(id);
            }
            if let Some(id) = close_timer.try_get_untracked().flatten() {
                clear_timeout(id);
            }
        }
    });

    let context_value = StoredValue::new(context_value);

    view! {
        <Popper>
            <Provider value=context_value.get_value()>
                {children.with_value(|children| children())}
            </Provider>
        </Popper>
    }
}

#[component]
pub fn TooltipTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();
    let composed_refs = crate::primitives::core::compose_refs::use_composed_refs(vec![
        node_ref,
        context.trigger_ref,
    ]);

    let on_open = context.on_open;

    let ctx_open = context.open;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;
    let ctx_delay = context.delay_duration;
    let ctx_timer = context.open_timer_id;
    let content_id = StoredValue::new(context.content_id.clone());
    let trigger_ctx = StoredValue::new(context.clone());

    // A press on the trigger focuses it, which would open the tooltip at once; the press
    // already closes it, so a focus that follows a pointer press is ignored.
    let pointer_pressed = StoredValue::new(false);

    view! {
        <PopperAnchor as_child=true>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_refs
                attr:aria-describedby=move || ctx_open.get().then(|| content_id.get_value())
                attr:data-state=move || if ctx_open.get() {
                    if context.was_instant_open.get() { "instant-open" } else { "delayed-open" }
                } else { "closed" }
                on:pointerenter=move |_: leptos::ev::PointerEvent| {
                    trigger_ctx.get_value().cancel_close();
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    let delay = ctx_delay.get_untracked();
                    if delay <= 0 {
                        ctx_on_open.run(());
                    } else {
                        let id = set_timeout(move || on_open.run(()), delay);
                        ctx_timer.set(Some(id));
                    }
                }
                on:pointerleave=move |_: leptos::ev::PointerEvent| {
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    let ctx = trigger_ctx.get_value();
                    if ctx.hoverable.get_untracked() && ctx.open.get_untracked() {
                        // Give the pointer time to reach the content, which cancels this.
                        ctx.cancel_close();
                        let ctx_for_timer = ctx.clone();
                        let id = set_timeout(
                            move || {
                                ctx_for_timer.close_timer_id.try_set(None);
                                ctx_for_timer.on_close.try_run(());
                            },
                            HOVER_CLOSE_DELAY_MS,
                        );
                        ctx.close_timer_id.set(Some(id));
                    } else {
                        ctx_on_close.run(());
                    }
                }
                on:pointerdown=move |_: leptos::ev::PointerEvent| {
                    pointer_pressed.set_value(true);
                    trigger_ctx.get_value().cancel_close();
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    ctx_on_close.run(());
                }
                on:focus=move |_| {
                    if pointer_pressed.get_value() {
                        pointer_pressed.set_value(false);
                    } else {
                        ctx_on_open.run(());
                    }
                }
                on:blur=move |_| {
                    pointer_pressed.set_value(false);
                    trigger_ctx.get_value().cancel_close();
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    ctx_on_close.run(());
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </PopperAnchor>
    }
}

#[component]
pub fn TooltipPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();

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
pub fn TooltipContent(
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
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("top".into())));
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

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);
    keep_portal_mounted(presence.is_present);

    // Escape closes the tooltip. A press anywhere also closes it (the trigger's own press
    // is handled by the trigger).
    let on_close_for_dismiss = context.on_close;
    let dismiss_ref = crate::primitives::core::dismissable_layer::use_dismissable_layer_with(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| on_close_for_dismiss.run(()))),
        Signal::derive(move || !context.open.get()),
        false,
    );
    let composed_refs = crate::primitives::core::compose_refs::use_composed_refs(vec![
        node_ref,
        presence.node_ref,
        dismiss_ref,
    ]);

    let content_id = StoredValue::new(context.content_id.clone());
    let ctx_open = context.open;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;
    let ctx_open_timer_id = context.open_timer_id;
    let content_ctx = StoredValue::new(context.clone());

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
                as_child=as_child
                node_ref=composed_refs
                attr:id=content_id.get_value()
                attr:role="tooltip"
                attr:data-state=move || if ctx_open.get() {
                    if context.was_instant_open.get() { "instant-open" } else { "delayed-open" }
                } else { "closed" }
                on:pointerenter=move |_| {
                    let ctx = content_ctx.get_value();
                    ctx.cancel_close();
                    // Without `hoverable` the pointer on the content does not keep it open.
                    if ctx.hoverable.get_untracked() {
                        ctx_on_open.run(());
                    }
                }
                on:pointerleave=move |_| {
                    content_ctx.get_value().cancel_close();
                    if let Some(id) = ctx_open_timer_id.get_untracked() {
                        clear_timeout(id);
                        ctx_open_timer_id.set(None);
                    }
                    ctx_on_close.run(());
                }
            >
                {children.with_value(|children| children())}
            </PopperContent>
        </Show>
    }
}

#[component]
pub fn TooltipArrow(
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
