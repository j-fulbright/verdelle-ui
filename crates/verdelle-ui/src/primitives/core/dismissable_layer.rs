use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

// Global counter for unique layer IDs.
static NEXT_LAYER_ID: AtomicU64 = AtomicU64::new(1);

fn next_layer_id() -> u64 {
    NEXT_LAYER_ID.fetch_add(1, Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Global layer stack, shared across all dismissable layers in the app.
//
// A plain thread-local vector, not a signal: a signal created inside the first layer's
// reactive owner is disposed with that owner, and every later layer then panicked on it.
// Nothing needs to react to the stack changing.
// ---------------------------------------------------------------------------

thread_local! {
    static LAYER_STACK: std::cell::RefCell<Vec<u64>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn stack_push(id: u64) {
    LAYER_STACK.with(|v| {
        let mut v = v.borrow_mut();
        if !v.contains(&id) {
            v.push(id);
        }
    });
}

fn stack_remove(id: u64) {
    LAYER_STACK.with(|v| v.borrow_mut().retain(|&x| x != id));
}

fn stack_is_topmost(id: u64) -> bool {
    LAYER_STACK.with(|v| v.borrow().last().copied() == Some(id))
}

/// Hook that detects interactions outside a given element and escape key presses.
///
/// Implements a layer stack: only the **topmost** active layer responds to
/// Escape and outside-pointer/focus events. This prevents all open overlays
/// from closing simultaneously.
///
/// Returns a node ref to attach to the dismissable layer element.
pub fn use_dismissable_layer(
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    on_focus_outside: Option<Callback<web_sys::FocusEvent>>,
    on_interact_outside: Option<Callback<web_sys::Event>>,
    on_dismiss: Option<Callback<()>>,
    disabled: Signal<bool>,
) -> AnyNodeRef {
    use_dismissable_layer_with(
        on_escape_key_down,
        on_pointer_down_outside,
        on_focus_outside,
        on_interact_outside,
        on_dismiss,
        disabled,
        true,
    )
}

/// Like [`use_dismissable_layer`], with a switch for whether focus moving outside the layer
/// dismisses it. `focusin` cannot be cancelled, so a callback cannot veto it; a layer that
/// stays open while focus is elsewhere (a sub menu opened by hover) turns it off here.
pub fn use_dismissable_layer_with(
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    on_focus_outside: Option<Callback<web_sys::FocusEvent>>,
    on_interact_outside: Option<Callback<web_sys::Event>>,
    on_dismiss: Option<Callback<()>>,
    disabled: Signal<bool>,
    dismiss_on_focus_outside: bool,
) -> AnyNodeRef {
    let node_ref = AnyNodeRef::new();
    let layer_id = next_layer_id();

    // Register this layer on the stack when enabled, remove when disabled/cleanup.
    Effect::new(move |_| {
        // `try_get`: this can run once more just after the layer's owner is disposed (the layer
        // closes as `disabled` flips), when the signal is already gone.
        let Some(is_disabled) = disabled.try_get() else {
            return;
        };
        if is_disabled {
            stack_remove(layer_id);
        } else {
            stack_push(layer_id);
        }
    });

    // --- Escape key: only fire for the topmost layer ---
    type KeyHandler = dyn Fn(web_sys::KeyboardEvent);
    let handle_keydown: Arc<SendWrapper<Closure<KeyHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::KeyboardEvent| {
            if disabled.try_get_untracked().unwrap_or(true) {
                return;
            }
            let is_topmost = stack_is_topmost(layer_id);
            if !is_topmost {
                return;
            }
            if event.key() == "Escape" {
                if let Some(cb) = on_escape_key_down {
                    cb.run(event.clone());
                }
                if !event.default_prevented()
                    && let Some(dismiss) = on_dismiss
                {
                    dismiss.run(());
                }
            }
        }),
    ));

    // --- Pointer down outside: only fire for topmost layer ---
    type PointerHandler = dyn Fn(web_sys::PointerEvent);
    let handle_pointerdown: Arc<SendWrapper<Closure<PointerHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::PointerEvent| {
            if disabled.try_get_untracked().unwrap_or(true) {
                return;
            }
            // Ignore right-clicks — context menus should not dismiss layers.
            if event.button() == 2 {
                return;
            }
            let is_topmost = stack_is_topmost(layer_id);
            if !is_topmost {
                return;
            }
            let target = event.target();
            let is_outside = target
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>())
                .map(|target_node| {
                    node_ref
                        .get_untracked()
                        .map(|container| !container.contains(Some(target_node)))
                        .unwrap_or(false)
                })
                .unwrap_or(false);

            if is_outside {
                if let Some(cb) = on_pointer_down_outside {
                    cb.run(event.clone());
                }
                let event_as_event: &web_sys::Event = event.as_ref();
                if let Some(cb) = on_interact_outside {
                    cb.run(event_as_event.clone());
                }
                if !event.default_prevented()
                    && let Some(dismiss) = on_dismiss
                {
                    dismiss.run(());
                }
            }
        }),
    ));

    // --- Focus outside: only fire for topmost layer ---
    type FocusHandler = dyn Fn(web_sys::FocusEvent);
    let handle_focusin: Arc<SendWrapper<Closure<FocusHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::FocusEvent| {
            if disabled.try_get_untracked().unwrap_or(true) {
                return;
            }
            let is_topmost = stack_is_topmost(layer_id);
            if !is_topmost || !dismiss_on_focus_outside {
                return;
            }
            let target = event.target();
            let is_outside = target
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>())
                .map(|target_node| {
                    node_ref
                        .get_untracked()
                        .map(|container| !container.contains(Some(target_node)))
                        .unwrap_or(false)
                })
                .unwrap_or(false);

            if is_outside {
                if let Some(cb) = on_focus_outside {
                    cb.run(event.clone());
                }
                let event_as_event: &web_sys::Event = event.as_ref();
                if let Some(cb) = on_interact_outside {
                    cb.run(event_as_event.clone());
                }
                if !event.default_prevented()
                    && let Some(dismiss) = on_dismiss
                {
                    dismiss.run(());
                }
            }
        }),
    ));

    // Set when the owner is disposed. The listener-attaching effect below can still run after
    // that (open then close in quick succession), and would leave listeners behind that read
    // disposed signals.
    let cleaned_up = Arc::new(std::sync::atomic::AtomicBool::new(false));

    // Attach document listeners.
    Effect::new({
        let cleaned_up = cleaned_up.clone();
        let handle_keydown = handle_keydown.clone();
        let handle_pointerdown = handle_pointerdown.clone();
        let handle_focusin = handle_focusin.clone();
        move |_| {
            if cleaned_up.load(Ordering::Relaxed) {
                return;
            }
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            let _ = document.add_event_listener_with_callback(
                "keydown",
                (**handle_keydown).as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "pointerdown",
                (**handle_pointerdown).as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "focusin",
                (**handle_focusin).as_ref().unchecked_ref(),
            );
        }
    });

    on_cleanup(move || {
        cleaned_up.store(true, Ordering::Relaxed);
        // Remove from layer stack.
        stack_remove(layer_id);

        // Remove document listeners.
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let _ = document.remove_event_listener_with_callback(
            "keydown",
            (**handle_keydown).as_ref().unchecked_ref(),
        );
        let _ = document.remove_event_listener_with_callback(
            "pointerdown",
            (**handle_pointerdown).as_ref().unchecked_ref(),
        );
        let _ = document.remove_event_listener_with_callback(
            "focusin",
            (**handle_focusin).as_ref().unchecked_ref(),
        );
    });

    node_ref
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_last_pushed_layer_is_topmost_and_removal_hands_it_back() {
        let (a, b) = (next_layer_id(), next_layer_id());
        stack_push(a);
        stack_push(b);
        stack_push(a); // pushing a layer twice must not reorder or duplicate it
        assert!(stack_is_topmost(b));
        assert!(!stack_is_topmost(a));
        stack_remove(b);
        assert!(stack_is_topmost(a));
        stack_remove(a);
        assert!(!stack_is_topmost(a));
    }
}
