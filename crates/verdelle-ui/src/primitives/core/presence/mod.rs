use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

pub(crate) mod use_state_machine;
use use_state_machine::use_state_machine;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MachineState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MachineEvent {
    Mount,
    AnimationOut,
    AnimationEnd,
    Unmount,
}

/// Return value from `use_presence`.
pub struct UsePresenceReturn {
    /// Whether the element should currently be present in the DOM.
    pub is_present: Signal<bool>,
    /// Attach this ref to the animated element so presence can track animation state.
    pub node_ref: AnyNodeRef,
}

/// Hook that manages presence state with CSS animation awareness.
///
/// Returns whether the element should be mounted, delaying unmount
/// until any exit animation completes.
pub fn use_presence(present: Signal<bool>) -> UsePresenceReturn {
    let node_ref = AnyNodeRef::new();
    let styles: RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>> = RwSignal::new(None);
    let prev_present = RwSignal::new(present.get_untracked());
    let prev_animation_name = RwSignal::new("none".to_string());

    let initial_state = match present.get_untracked() {
        true => MachineState::Mounted,
        false => MachineState::Unmounted,
    };

    let (state, send) = use_state_machine(
        initial_state,
        HashMap::from([
            (
                MachineState::Mounted,
                HashMap::from([
                    (MachineEvent::Unmount, MachineState::Unmounted),
                    (MachineEvent::AnimationOut, MachineState::UnmountSuspended),
                ]),
            ),
            (
                MachineState::UnmountSuspended,
                HashMap::from([
                    (MachineEvent::Mount, MachineState::Mounted),
                    (MachineEvent::AnimationEnd, MachineState::Unmounted),
                ]),
            ),
            (
                MachineState::Unmounted,
                HashMap::from([(MachineEvent::Mount, MachineState::Mounted)]),
            ),
        ]),
    );

    // Track current animation name when state changes
    Effect::new(move |_| {
        let current_animation_name = get_animation_name(styles.get().as_deref());
        prev_animation_name.set(match state.get() {
            MachineState::Mounted => current_animation_name,
            _ => "none".into(),
        });
    });

    // React to present changes
    Effect::new(move |_| {
        let was_present = prev_present.get();
        let is_present = present.get();

        if was_present != is_present {
            prev_present.set(is_present);

            if is_present {
                send.run(MachineEvent::Mount);
                return;
            }

            // Whether there is an exit animation is only known once the element's `data-state`
            // (and with it the animation the CSS gives a closing element) has been written, and
            // that happens after this effect runs. So look a frame later; the element stays
            // mounted until then.
            request_animation_frame(move || {
                // Reopened in the meantime: the effect above has already sent `Mount`.
                if present.try_get_untracked().unwrap_or(true) {
                    return;
                }
                let current_styles = styles.try_get_untracked().flatten();
                let prev_anim = prev_animation_name
                    .try_get_untracked()
                    .unwrap_or_else(|| "none".into());
                let current_animation_name = get_animation_name(current_styles.as_deref());

                if current_animation_name == "none"
                    || current_styles
                        .as_deref()
                        .and_then(|s| s.get_property_value("display").ok())
                        == Some("none".into())
                {
                    send.try_run(MachineEvent::Unmount);
                } else if prev_anim != current_animation_name {
                    send.try_run(MachineEvent::AnimationOut);
                    // `animationend` always follows, but never leave the element stuck if it
                    // does not (an animation cancelled by `display: none`, say).
                    set_timeout(
                        move || {
                            if !present.try_get_untracked().unwrap_or(true) {
                                send.try_run(MachineEvent::AnimationEnd);
                            }
                        },
                        std::time::Duration::from_millis(1000),
                    );
                } else {
                    send.try_run(MachineEvent::Unmount);
                }
            });
        }
    });

    // Listen for animation events on the node
    type AnimHandler = dyn Fn(web_sys::AnimationEvent);

    let handle_animation_end: Arc<SendWrapper<Closure<AnimHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::AnimationEvent| {
            let current_animation_name = get_animation_name(styles.get_untracked().as_deref());
            let is_current_animation = current_animation_name.contains(&event.animation_name());
            if is_current_animation
                && event.target().as_ref()
                    == node_ref
                        .get_untracked()
                        .as_ref()
                        .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
            {
                send.run(MachineEvent::AnimationEnd);
            }
        }),
    ));

    let handle_animation_start: Arc<SendWrapper<Closure<AnimHandler>>> = Arc::new(
        SendWrapper::new(Closure::new(move |event: web_sys::AnimationEvent| {
            if event.target().as_ref()
                == node_ref
                    .get_untracked()
                    .as_ref()
                    .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
            {
                prev_animation_name.set(get_animation_name(styles.get_untracked().as_deref()));
            }
        })),
    );

    Effect::new({
        let handle_animation_end = handle_animation_end.clone();
        let handle_animation_start = handle_animation_start.clone();
        move |_| {
            if let Some(node) = node_ref.get() {
                let _ = node.add_event_listener_with_callback(
                    "animationstart",
                    (**handle_animation_start).as_ref().unchecked_ref(),
                );
                let _ = node.add_event_listener_with_callback(
                    "animationcancel",
                    (**handle_animation_end).as_ref().unchecked_ref(),
                );
                let _ = node.add_event_listener_with_callback(
                    "animationend",
                    (**handle_animation_end).as_ref().unchecked_ref(),
                );
            } else {
                send.run(MachineEvent::AnimationEnd);
            }
        }
    });

    // Get computed styles
    Effect::new(move |_| {
        if let Some(node) = node_ref.get()
            && let Some(window) = web_sys::window()
        {
            styles.set(
                window
                    .get_computed_style(&node)
                    .expect("Element is valid.")
                    .map(SendWrapper::new),
            );
        }
    });

    on_cleanup(move || {
        if let Some(node) = node_ref.get_untracked() {
            let _ = node.remove_event_listener_with_callback(
                "animationstart",
                (**handle_animation_start).as_ref().unchecked_ref(),
            );
            let _ = node.remove_event_listener_with_callback(
                "animationcancel",
                (**handle_animation_end).as_ref().unchecked_ref(),
            );
            let _ = node.remove_event_listener_with_callback(
                "animationend",
                (**handle_animation_end).as_ref().unchecked_ref(),
            );
        }
    });

    UsePresenceReturn {
        is_present: Signal::derive(move || {
            [MachineState::Mounted, MachineState::UnmountSuspended].contains(&state.get())
        }),
        node_ref,
    }
}

fn get_animation_name(styles: Option<&web_sys::CssStyleDeclaration>) -> String {
    styles
        .and_then(|styles| styles.get_property_value("animation-name").ok())
        .unwrap_or("none".into())
}

/// Lets a portal stay mounted while its content plays an exit animation.
///
/// Portals are `<Show when=open>`, which would remove the content the moment `open` turns
/// false and leave no time for [`use_presence`] to wait for the exit. A portal provides one of
/// these (`<Provider value=keep>`) and shows while `open || keep`; its content calls
/// [`keep_portal_mounted`] with its presence, so the portal goes away only after the exit.
#[derive(Clone, Copy)]
pub(crate) struct PortalKeep(RwSignal<bool>);

impl PortalKeep {
    pub(crate) fn new() -> Self {
        Self(RwSignal::new(false))
    }

    pub(crate) fn get(&self) -> bool {
        self.0.get()
    }
}

/// Called by a content component, next to [`use_presence`], with that presence: tells the
/// enclosing portal (if any) to stay mounted until the content has gone.
pub(crate) fn keep_portal_mounted(is_present: Signal<bool>) {
    if let Some(PortalKeep(keep)) = use_context::<PortalKeep>() {
        Effect::new(move |_| keep.set(is_present.get()));
        on_cleanup(move || {
            keep.try_set(false);
        });
    }
}
