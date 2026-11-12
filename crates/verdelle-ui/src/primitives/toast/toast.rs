use std::sync::Arc;

use crate::primitives::core::presence::use_presence;
use crate::primitives::core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

const SWIPE_THRESHOLD_PX: f64 = 100.0;

fn window() -> web_sys::Window {
    web_sys::window().expect("should have a Window")
}

fn set_timeout(callback: &Closure<dyn Fn()>, ms: i32) -> Option<i32> {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            ms,
        )
        .ok()
}

fn clear_timeout(id: i32) {
    window().clear_timeout_with_handle(id);
}

/// Milliseconds on a monotonic clock, for measuring how much of a toast's duration has run.
fn now_ms() -> f64 {
    window().performance().map(|p| p.now()).unwrap_or_default()
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastProviderContextValue {
    toasts: RwSignal<Vec<ToastEntry>>,
    add_toast: Callback<ToastEntry>,
    remove_toast: Callback<String>,
    swipe_direction: Signal<String>,
    default_duration: Signal<i32>,
}

#[derive(Clone, Debug)]
struct ToastEntry {
    id: String,
    /// Closes the toast the same way its own close button does (sets it closed and runs `on_open_change`).
    close: Callback<()>,
    /// When true, the auto-close timer is paused.
    paused: RwSignal<bool>,
}

/// Handle for acting on every toast under a [`ToastProvider`]. Get one with [`use_toast_controller`].
#[derive(Clone, Copy, Debug)]
pub struct ToastController {
    toasts: RwSignal<Vec<ToastEntry>>,
}

impl ToastController {
    /// Closes every toast, running each one's `on_open_change(false)`.
    pub fn dismiss_all(&self) {
        // Collect first: closing a toast can unregister it and mutate the list.
        let closers: Vec<Callback<()>> = self
            .toasts
            .with_untracked(|list| list.iter().map(|t| t.close).collect());
        for close in closers {
            close.run(());
        }
    }
}

/// Returns a [`ToastController`] for the nearest [`ToastProvider`], or `None` outside one.
pub fn use_toast_controller() -> Option<ToastController> {
    use_context::<ToastProviderContextValue>().map(|ctx| ToastController { toasts: ctx.toasts })
}

#[component]
pub fn ToastProvider(
    #[prop(into, optional, default = 5000.into())] duration: MaybeProp<i32>,
    #[prop(into, optional)] swipe_direction: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let default_duration = Signal::derive(move || duration.get().unwrap_or(5000));
    let swipe_dir = Signal::derive(move || swipe_direction.get().unwrap_or("right".into()));
    let toasts: RwSignal<Vec<ToastEntry>> = RwSignal::new(vec![]);

    let ctx = ToastProviderContextValue {
        toasts,
        add_toast: Callback::new(move |entry: ToastEntry| {
            toasts.update(|t| t.push(entry));
        }),
        remove_toast: Callback::new(move |id: String| {
            toasts.update(|t| t.retain(|e| e.id != id));
        }),
        swipe_direction: swipe_dir,
        default_duration,
    };

    view! { <Provider value=ctx>{children.with_value(|c| c())}</Provider> }
}

#[derive(Clone, Debug)]
struct ToastContextValue {
    on_close: Callback<()>,
}

#[component]
pub fn Toast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] duration: MaybeProp<i32>,
    /// Announce to screen readers at once (`aria-live="assertive"`, for errors) instead of
    /// politely (the default). Radix calls this `type="foreground"`.
    #[prop(into, optional)]
    assertive: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let provider_ctx = use_context::<ToastProviderContextValue>();
    let duration = Signal::derive(move || {
        duration.get().unwrap_or_else(|| {
            provider_ctx
                .as_ref()
                .map(|ctx| ctx.default_duration.get())
                .unwrap_or(5000)
        })
    });
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(true));

    // Get swipe direction from provider context (default "right")
    let provider_ctx = use_context::<ToastProviderContextValue>();
    let swipe_direction = Signal::derive(move || {
        provider_ctx
            .as_ref()
            .map(|ctx| ctx.swipe_direction.get())
            .unwrap_or_else(|| "right".to_string())
    });

    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_state.set(o);
        }
    });

    let id = crate::primitives::core::id::use_id(None).get_untracked();

    let on_close_cb = Callback::new(move |()| {
        // Already closing (for instance dismiss-all during an exit animation): nothing to do.
        if !open_state.get_untracked() {
            return;
        }
        open_state.set(false);
        if let Some(cb) = on_open_change {
            cb.run(false);
        }
    });

    let toast_ctx = ToastContextValue {
        on_close: on_close_cb,
    };

    // Self-register with the provider so "pause all" / "dismiss all" is possible.
    let provider_for_reg = use_context::<ToastProviderContextValue>();
    // Paused by the viewport (pointer over it, or focus inside it).
    let paused = RwSignal::new(false);
    // Paused because the pointer is over this toast, or it is being swiped.
    let hovered = RwSignal::new(false);
    let swiping = RwSignal::new(false);
    if let Some(ref pctx) = provider_for_reg {
        pctx.add_toast.run(ToastEntry {
            id: id.clone(),
            close: on_close_cb,
            paused,
        });
    }
    {
        let id_for_cleanup = id.clone();
        on_cleanup(move || {
            if let Some(ref pctx) = provider_for_reg {
                pctx.remove_toast.run(id_for_cleanup.clone());
            }
        });
    }

    // Auto-close timer. While any pause applies the timer is cleared and the time it had
    // already run is subtracted, so resuming continues with what was left rather than
    // starting the full duration again.
    let close_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_close_cb.run(());
        })));
    let close_closure_stored = StoredValue::new(close_closure);
    let timer_id: StoredValue<Option<i32>> = StoredValue::new(None);
    let remaining_ms: StoredValue<Option<f64>> = StoredValue::new(None);
    let started_at: StoredValue<f64> = StoredValue::new(0.0);
    let running =
        Memo::new(move |_| open_state.get() && !paused.get() && !hovered.get() && !swiping.get());

    Effect::new(move |_| {
        if running.get() {
            let total = duration.get_untracked();
            if total <= 0 {
                return;
            }
            let left = remaining_ms.get_value().unwrap_or(total as f64).max(1.0);
            remaining_ms.set_value(Some(left));
            started_at.set_value(now_ms());
            close_closure_stored.with_value(|cls| {
                timer_id.set_value(set_timeout(cls, left.ceil() as i32));
            });
        } else if let Some(id) = timer_id.get_value() {
            clear_timeout(id);
            timer_id.set_value(None);
            let ran = now_ms() - started_at.get_value();
            remaining_ms.update_value(|left| {
                if let Some(left) = left {
                    *left -= ran;
                }
            });
        }
    });

    on_cleanup(move || {
        if let Some(Some(id)) = timer_id.try_get_value() {
            clear_timeout(id);
        }
    });

    let present = Signal::derive(move || open_state.get());
    let presence = use_presence(present);
    // Presence needs the element to see its exit animation before unmounting it.
    let refs =
        crate::primitives::core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref]);

    // --- Swipe state ---
    // The offset along the swipe axis is exposed as `--vdl-swipe-x` / `--vdl-swipe-y` so the
    // stylesheet decides how to move (and how to animate out from where the swipe ended).
    let swipe_start_x = RwSignal::new(0.0f64);
    let swipe_start_y = RwSignal::new(0.0f64);
    let swipe_x = RwSignal::new(0.0f64);
    let swipe_y = RwSignal::new(0.0f64);
    let swipe_data_attr: RwSignal<Option<&'static str>> = RwSignal::new(None);

    // The distance travelled in the swipe direction (negative or zero when against it).
    let along = move |dx: f64, dy: f64| match swipe_direction.get_untracked().as_str() {
        "right" => dx,
        "left" => -dx,
        "up" => -dy,
        "down" => dy,
        _ => 0.0,
    };

    view! {
        <Provider value=toast_ctx>
            <Show when=move || presence.is_present.get()>
                <Primitive element=html::li as_child=as_child node_ref=refs
                    attr:role=move || if assertive.get().unwrap_or(false) { "alert" } else { "status" }
                    attr:aria-live=move || if assertive.get().unwrap_or(false) { "assertive" } else { "polite" }
                    attr:aria-atomic="true"
                    attr:data-state=move || if open_state.get() { "open" } else { "closed" }
                    attr:data-swipe=move || swipe_data_attr.get()
                    attr:data-swipe-direction=move || swipe_direction.get()
                    attr:tabindex="0"
                    style:--vdl-swipe-x=move || format!("{}px", swipe_x.get())
                    style:--vdl-swipe-y=move || format!("{}px", swipe_y.get())
                    style:user-select=move || {
                        if swiping.get() { "none" } else { "" }
                    }
                    style:touch-action=move || {
                        match swipe_direction.get().as_str() {
                            "right" | "left" => "pan-y",
                            "up" | "down" => "pan-x",
                            _ => "auto",
                        }
                    }
                    on:keydown=move |event: leptos::ev::KeyboardEvent| {
                        if event.key() == "Escape" {
                            event.stop_propagation();
                            on_close_cb.run(());
                        }
                    }
                    on:pointerdown=move |event: leptos::ev::PointerEvent| {
                        swipe_start_x.set(event.client_x() as f64);
                        swipe_start_y.set(event.client_y() as f64);
                        swipe_x.set(0.0);
                        swipe_y.set(0.0);
                        swiping.set(true);
                        swipe_data_attr.set(Some("start"));
                        // Capture pointer for reliable tracking
                        if let Some(target) = event.target()
                            && let Ok(el) = target.dyn_into::<web_sys::Element>()
                        {
                            let _ = el.set_pointer_capture(event.pointer_id());
                        }
                    }
                    on:pointermove=move |event: leptos::ev::PointerEvent| {
                        if !swiping.get_untracked() {
                            return;
                        }
                        let dx = event.client_x() as f64 - swipe_start_x.get_untracked();
                        let dy = event.client_y() as f64 - swipe_start_y.get_untracked();
                        let d = along(dx, dy);
                        // Move along the swipe axis only, and only with the direction.
                        let horizontal = matches!(
                            swipe_direction.get_untracked().as_str(),
                            "right" | "left"
                        );
                        let d = d.max(0.0);
                        let sign = match swipe_direction.get_untracked().as_str() {
                            "left" | "up" => -1.0,
                            _ => 1.0,
                        };
                        if horizontal {
                            swipe_x.set(d * sign);
                            swipe_y.set(0.0);
                        } else {
                            swipe_y.set(d * sign);
                            swipe_x.set(0.0);
                        }
                        if d > 0.0 {
                            swipe_data_attr.set(Some("move"));
                        }
                    }
                    on:pointerup=move |event: leptos::ev::PointerEvent| {
                        if !swiping.get_untracked() {
                            return;
                        }
                        if let Some(target) = event.target()
                            && let Ok(el) = target.dyn_into::<web_sys::Element>()
                        {
                            let _ = el.release_pointer_capture(event.pointer_id());
                        }
                        let d = along(
                            event.client_x() as f64 - swipe_start_x.get_untracked(),
                            event.client_y() as f64 - swipe_start_y.get_untracked(),
                        );
                        if d > SWIPE_THRESHOLD_PX {
                            // Swipe completed: dismiss. The offset stays so the exit animation
                            // can continue from where the finger left the toast.
                            swipe_data_attr.set(Some("end"));
                            swiping.set(false);
                            on_close_cb.run(());
                        } else {
                            swipe_data_attr.set(Some("cancel"));
                            swipe_x.set(0.0);
                            swipe_y.set(0.0);
                            swiping.set(false);
                        }
                    }
                    on:pointercancel=move |_: leptos::ev::PointerEvent| {
                        if swiping.get_untracked() {
                            swipe_data_attr.set(Some("cancel"));
                            swipe_x.set(0.0);
                            swipe_y.set(0.0);
                            swiping.set(false);
                        }
                    }
                    on:pointerenter=move |_: leptos::ev::PointerEvent| hovered.set(true)
                    on:pointerleave=move |_: leptos::ev::PointerEvent| hovered.set(false)
                >
                    {children.with_value(|c| c())}
                </Primitive>
            </Show>
        </Provider>
    }
}

#[component]
pub fn ToastTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref>{children.with_value(|c| c())}</Primitive> }
}

#[component]
pub fn ToastDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref>{children.with_value(|c| c())}</Primitive> }
}

#[component]
pub fn ToastAction(
    #[prop(into)] alt_text: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = use_context::<ToastContextValue>();
    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:aria-label=alt_text
            // As Radix: acting on a toast closes it.
            on:click=move |_| {
                if let Some(ctx) = ctx.as_ref() {
                    ctx.on_close.run(());
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ToastClose(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ToastContextValue>();

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            on:click=move |_| ctx.on_close.run(())
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ToastViewport(
    #[prop(into, optional)] label: MaybeProp<String>,
    /// Keyboard shortcut(s) to focus the viewport. Defaults to `["F8"]`.
    #[prop(into, optional)]
    hotkey: Option<Vec<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let label = Signal::derive(move || label.get().unwrap_or("Notifications".into()));
    let hotkeys = hotkey.unwrap_or_else(|| vec!["F8".into()]);
    let viewport_ref = AnyNodeRef::new();
    let refs =
        crate::primitives::core::compose_refs::use_composed_refs(vec![node_ref, viewport_ref]);

    // Focus the viewport on the hotkey. The listener is removed when the viewport unmounts.
    let hotkeys = StoredValue::new(hotkeys);
    Effect::new(move |_| {
        let handle = leptos::leptos_dom::helpers::window_event_listener(
            leptos::ev::keydown,
            move |event: web_sys::KeyboardEvent| {
                let pressed = event.key();
                if hotkeys
                    .get_value()
                    .iter()
                    .any(|k| k.eq_ignore_ascii_case(&pressed))
                    && let Some(el) = viewport_ref.get()
                    && let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>()
                {
                    let _ = html_el.focus();
                }
            },
        );
        on_cleanup(move || handle.remove());
    });

    // Access provider registry for pause-all / resume-all on viewport focus.
    // Extract the signal (Copy) so closures can share it without move conflicts.
    let toasts_signal = use_context::<ToastProviderContextValue>().map(|ctx| ctx.toasts);

    let set_all_paused = move |paused: bool| {
        if let Some(toasts) = toasts_signal {
            toasts.with_untracked(|list| {
                for toast in list {
                    toast.paused.set(paused);
                }
            });
        }
    };

    view! {
        <Primitive element=html::ol as_child=as_child node_ref=refs
            attr:role="region"
            attr:aria-label=move || label.get()
            attr:tabindex="-1"
            on:focusin=move |_| set_all_paused(true)
            on:focusout=move |_| set_all_paused(false)
            on:pointerenter=move |_| set_all_paused(true)
            on:pointerleave=move |_| set_all_paused(false)
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn dismiss_all_runs_every_toast_closer() {
        let owner = Owner::new();
        owner.set();
        let toasts: RwSignal<Vec<ToastEntry>> = RwSignal::new(vec![]);
        let closed = Arc::new(AtomicUsize::new(0));
        for i in 0..3 {
            let closed = closed.clone();
            toasts.update(|t| {
                t.push(ToastEntry {
                    id: format!("t{i}"),
                    close: Callback::new(move |()| {
                        closed.fetch_add(1, Ordering::SeqCst);
                    }),
                    paused: RwSignal::new(false),
                })
            });
        }
        ToastController { toasts }.dismiss_all();
        assert_eq!(closed.load(Ordering::SeqCst), 3);
    }
}
