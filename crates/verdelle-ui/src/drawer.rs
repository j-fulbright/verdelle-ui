use crate::DrawerDirection;
use crate::native_dialog::{
    DRAWER_EXIT_MS, notify_open_change, on_cancel, on_close as on_close_handler, sync_dialog_with,
};
use crate::primitives::drawer::{
    DragConfig, DragSession, DragUpdate, Release, ScrollRoom, next_snap, normalize_snaps,
};
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{Element, HtmlElement};
use leptos::{ev, html, prelude::*};

/// Tells [`crate::DrawerClose`] which drawer it belongs to.
#[derive(Clone, Copy)]
pub(crate) struct DrawerOpen(pub RwSignal<bool>);

const VAR_DRAG: &str = "--vdl-drawer-drag";
const VAR_SNAP: &str = "--vdl-drawer-snap";
const VAR_PROGRESS: &str = "--vdl-drawer-progress";

fn set_var(el: &HtmlElement, name: &str, value: &str) {
    let _ = el.style().set_property(name, value);
}

/// Whether a press that starts on `target` should not start a drag (text fields: the pointer is
/// there to place the caret and select).
fn is_text_field(target: &Element) -> bool {
    target
        .closest("input, textarea, select, [contenteditable]")
        .ok()
        .flatten()
        .is_some()
}

/// The nearest scrolling ancestor of `target` inside `dialog` along the drag axis, and how much
/// room it has left in each direction of the finger.
fn scroll_room(
    target: &Element,
    dialog: &Element,
    direction: DrawerDirection,
) -> Option<ScrollRoom> {
    let vertical = direction.is_vertical();
    let mut node = Some(target.clone());
    while let Some(el) = node {
        if &el == dialog {
            break;
        }
        let (scroll, client, pos) = if vertical {
            (el.scroll_height(), el.client_height(), el.scroll_top())
        } else {
            (el.scroll_width(), el.client_width(), el.scroll_left())
        };
        if scroll > client + 1 {
            let overflow = leptos::prelude::window()
                .get_computed_style(&el)
                .ok()
                .flatten()
                .and_then(|s| {
                    s.get_property_value(if vertical { "overflow-y" } else { "overflow-x" })
                        .ok()
                })
                .unwrap_or_default();
            if matches!(overflow.as_str(), "auto" | "scroll" | "overlay") {
                let max = scroll - client;
                let can_decrease = pos > 0;
                let can_increase = pos < max - 1;
                // The finger moving toward larger coordinates scrolls the content toward its
                // start (smaller scroll position) and the other way round.
                return Some(if direction.close_sign() > 0.0 {
                    ScrollRoom {
                        toward_close: can_decrease,
                        toward_open: can_increase,
                    }
                } else {
                    ScrollRoom {
                        toward_close: can_increase,
                        toward_open: can_decrease,
                    }
                });
            }
        }
        node = el.parent_element();
    }
    None
}

/// Everything the gesture handlers share. All fields are `Copy` handles, so the struct is too and
/// each event handler takes its own copy.
#[derive(Clone, Copy)]
struct Gesture {
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<bool>,
    active: RwSignal<usize>,
    snaps: StoredValue<Vec<f64>>,
    session: StoredValue<Option<DragSession>>,
    /// The drawer's extent along the drag axis, measured when a gesture starts.
    size: StoredValue<f64>,
    /// A drag just happened: the click that follows it must not count as a backdrop click.
    dragged: StoredValue<bool>,
    direction: DrawerDirection,
    dismissible: bool,
    scale: bool,
}

impl Gesture {
    fn snap_value(&self) -> f64 {
        let i = self.active.get_untracked();
        self.snaps
            .with_value(|s| s.get(i).or(s.last()).copied().unwrap_or(1.0))
    }

    /// Write the resting state of the CSS variables: the active snap point, no drag.
    fn sync_vars(&self, snap: f64) {
        if let Some(el) = self.dialog.get_untracked() {
            set_var(&el, VAR_SNAP, &snap.to_string());
            set_var(&el, VAR_DRAG, "0px");
            set_var(&el, VAR_PROGRESS, &(1.0 - snap).to_string());
            let _ = el.remove_attribute("data-dragging");
        }
        self.sync_page_progress(1.0 - snap, false);
    }

    /// Mirror the progress on `<html>` for the scaled background.
    fn sync_page_progress(&self, progress: f64, dragging: bool) {
        if !self.scale {
            return;
        }
        if let Some(root) = document().document_element()
            && let Ok(root) = root.dyn_into::<HtmlElement>()
        {
            set_var(&root, VAR_PROGRESS, &progress.to_string());
            if dragging {
                let _ = root.set_attribute("data-vdl-drawer-dragging", "");
            } else {
                let _ = root.remove_attribute("data-vdl-drawer-dragging");
            }
        }
    }

    /// Start tracking a gesture at (`x`, `y`) on `target`.
    fn begin(&self, x: f64, y: f64, time: f64, target: Option<Element>) {
        self.dragged.set_value(false);
        let Some(el) = self.dialog.get_untracked() else {
            return;
        };
        let size = if self.direction.is_vertical() {
            el.offset_height()
        } else {
            el.offset_width()
        } as f64;
        if size <= 0.0 {
            return;
        }
        self.size.set_value(size);
        let room = target.and_then(|t| scroll_room(&t, &el, self.direction));
        let (main, cross) = self.axes(x, y);
        let snaps = self.snaps.get_value();
        let active = self.active.get_untracked().min(snaps.len() - 1);
        self.session.set_value(Some(DragSession::new(
            DragConfig {
                size,
                snaps,
                active,
                dismissible: self.dismissible,
            },
            room,
            main,
            cross,
            time,
        )));
    }

    /// Position in "toward close" coordinates, and the cross axis.
    fn axes(&self, x: f64, y: f64) -> (f64, f64) {
        let sign = self.direction.close_sign();
        if self.direction.is_vertical() {
            (y * sign, x)
        } else {
            (x * sign, y)
        }
    }

    /// Feed a move. Returns whether the drawer is following the finger (the caller then stops
    /// the browser from scrolling).
    fn moved(&self, x: f64, y: f64, time: f64) -> bool {
        let (main, cross) = self.axes(x, y);
        let mut update = DragUpdate::Passed;
        self.session.update_value(|s| {
            if let Some(s) = s {
                update = s.update(main, cross, time);
            }
        });
        let DragUpdate::Drag(drag) = update else {
            return false;
        };
        self.dragged.set_value(true);
        let size = self.size.get_value().max(1.0);
        let progress = ((1.0 - self.snap_value()) + drag / size).clamp(0.0, 1.0);
        if let Some(el) = self.dialog.get_untracked() {
            let _ = el.set_attribute("data-dragging", "");
            set_var(&el, VAR_DRAG, &format!("{drag:.1}px"));
            set_var(&el, VAR_PROGRESS, &progress.to_string());
        }
        self.sync_page_progress(progress, true);
        true
    }

    /// The finger lifted: snap, or dismiss.
    fn end(&self, time: f64) {
        let session = self.session.try_update_value(Option::take).flatten();
        let Some(release) = session.and_then(|s| s.finish(time)) else {
            return;
        };
        match release {
            Release::Snap(i) => {
                // Clearing the dragging attribute turns the transition back on, so the panel
                // glides to the snap point.
                if self.active.get_untracked() != i {
                    self.active.set(i);
                }
                self.sync_vars(self.snaps.with_value(|s| s[i.min(s.len() - 1)]));
            }
            Release::Dismiss => {
                if let Some(el) = self.dialog.get_untracked() {
                    // The exit animation continues from where the finger left the panel.
                    let _ = el.remove_attribute("data-dragging");
                }
                self.sync_page_progress(1.0, false);
                self.open.set(false);
            }
        }
    }

    /// The gesture was interrupted (the browser took it, the touch was cancelled).
    fn cancel(&self) {
        if self
            .session
            .try_update_value(Option::take)
            .flatten()
            .is_some()
        {
            self.sync_vars(self.snap_value());
        }
    }
}

/// A panel that is pulled in from an edge and swiped away, styled after shadcn/ui's Drawer
/// (vaul). Use it for a task that should feel like a layer of the page, most of all on a phone:
/// the panel follows the finger, a flick or a long pull dismisses it, and it can rest at several
/// heights (snap points).
///
/// It is built like [`crate::Sheet`]: a native `<dialog>`, so a modal drawer gets the browser's
/// focus trap, Escape, inert page, top-layer stacking and focus restore, and one component is
/// the panel, the backdrop and the portal. `open` is the single source of truth: set it to show
/// or hide the drawer, and it is set to `false` when the user dismisses it. `label` is the
/// accessible name. Closing plays an exit animation (300ms) before the element is closed, from
/// wherever the panel was let go.
///
/// Compose the body with [`crate::DrawerHeader`], [`crate::DrawerTitle`],
/// [`crate::DrawerDescription`] and [`crate::DrawerFooter`]; [`crate::DrawerClose`] and
/// [`crate::DrawerTrigger`] are buttons for the open signal.
///
/// - `direction`: the edge it comes from, bottom by default. A top or bottom drawer is as tall as
///   its content (at most the viewport minus 6rem); a left or right one is 75% wide (24rem from
///   640px up).
/// - `snap_points`: visible fractions of the drawer, for example `[0.4, 1.0]`. With snap points
///   the drawer is `100dvh - 3rem` tall (or wide, for left and right) and rests at one of them;
///   `default_snap` (an index into the sorted list) is where it opens, `active_snap` makes the
///   active index controlled. A flick moves one snap point, a slow release picks the nearest,
///   and a click on the handle cycles through them. The decisions are in
///   [`crate::primitives::drawer`].
/// - `show_handle`: the grab handle, on by default.
/// - `dismissible`: when off, the drawer cannot be closed by swiping, Escape or the backdrop, only
///   by your own buttons; a swipe meets resistance.
/// - `modal`: on by default. Off opens the drawer without a backdrop, focus trap or inert page,
///   so the page stays usable behind it; it is still dismissed by swiping, Escape (while focus is
///   inside) or your buttons, and focus returns to what had it.
/// - `scale_background`: while a modal drawer is open the [`crate::DrawerWrapper`] around the page
///   shrinks a little, following the drag.
///
/// Scrolling content inside the drawer scrolls first; once it is at its edge (a list at the top)
/// the same swipe moves the drawer. A drawer at a lower snap point takes every swipe. Touch is
/// handled with touch events (so the browser can be told not to scroll), the mouse and pen with
/// pointer events.
#[component]
pub fn Drawer(
    /// The open state, read and written by the component.
    open: RwSignal<bool>,
    #[prop(into)] label: String,
    /// Called with the new state whenever it opens or closes (after the first render), so you
    /// need not watch `open` in an `Effect`. It fires when the change is requested.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    /// Called when it closes, however that happened (Escape, backdrop, swipe, a button,
    /// `open.set(false)`). It fires when the close is requested, at the start of the exit animation.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    #[prop(default = DrawerDirection::Bottom)] direction: DrawerDirection,
    /// Visible fractions of the drawer it may rest at, e.g. `[0.4, 1.0]`.
    #[prop(optional, into)]
    snap_points: Vec<f64>,
    /// The index (into the sorted snap points) the drawer opens at. 0 by default.
    #[prop(default = 0)]
    default_snap: usize,
    /// Controlled active snap point index. Leave out to let the drawer manage it.
    #[prop(optional)]
    active_snap: Option<RwSignal<usize>>,
    /// Show the grab handle. On by default.
    #[prop(default = true)]
    show_handle: bool,
    /// Whether swiping, Escape and a backdrop click may close it. On by default.
    #[prop(default = true)]
    dismissible: bool,
    /// Modal (backdrop, focus trap, inert page). On by default.
    #[prop(default = true)]
    modal: bool,
    /// Shrink the [`crate::DrawerWrapper`] while open.
    #[prop(default = false)]
    scale_background: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<html::Dialog>::new();
    provide_context(DrawerOpen(open));
    notify_open_change(open, on_open_change, on_close);

    let snaps = normalize_snaps(&snap_points);
    let has_snaps = !snap_points.is_empty();
    let default_snap = default_snap.min(snaps.len() - 1);
    let controlled = active_snap.is_some();
    let active = active_snap.unwrap_or_else(|| RwSignal::new(default_snap));

    let gesture = Gesture {
        dialog,
        open,
        active,
        snaps: StoredValue::new(snaps),
        session: StoredValue::new(None),
        size: StoredValue::new(0.0),
        dragged: StoredValue::new(false),
        direction,
        dismissible,
        scale: scale_background && modal,
    };
    let previous_focus = StoredValue::new_local(None::<HtmlElement>);

    // Keep the element in step with `open`. A new opening starts at the default snap point.
    sync_dialog_with(dialog, open, DRAWER_EXIT_MS, modal, move |_| {
        if !controlled {
            active.set(default_snap);
        }
        gesture.session.set_value(None);
        gesture.sync_vars(gesture.snap_value());
        if !modal {
            let focused = document()
                .active_element()
                .and_then(|e| e.dyn_into::<HtmlElement>().ok());
            previous_focus.set_value(focused);
        }
    });

    // The snap point in the CSS, and its backdrop strength.
    Effect::new(move |_| {
        let _ = dialog.get();
        let i = active.get();
        let snap = gesture
            .snaps
            .with_value(|s| s.get(i).or(s.last()).copied().unwrap_or(1.0));
        if gesture.session.with_value(Option::is_none) {
            gesture.sync_vars(snap);
        }
    });

    // The scaled page behind: an attribute on <html> while a modal drawer is open.
    if gesture.scale {
        Effect::new(move |_| {
            let on = open.get();
            if let Some(root) = document().document_element() {
                if on {
                    let _ = root.set_attribute("data-vdl-drawer-scaled", "");
                } else {
                    let _ = root.remove_attribute("data-vdl-drawer-scaled");
                    let _ = root.remove_attribute("data-vdl-drawer-dragging");
                }
            }
            on_cleanup(|| {
                if let Some(root) = document().document_element() {
                    let _ = root.remove_attribute("data-vdl-drawer-scaled");
                }
            });
        });
    }

    // Mouse and pen: pointer events on the window while a gesture is live. A press on the
    // backdrop (outside the panel's rectangle) does not start one.
    // (An effect: it only runs in the browser, and `window` does not exist during SSR.)
    Effect::new(move |_| {
        let handles = [
            window_event_listener(ev::pointermove, move |e: ev::PointerEvent| {
                if e.pointer_type() != "touch" && gesture.session.with_value(Option::is_some) {
                    gesture.moved(e.client_x() as f64, e.client_y() as f64, e.time_stamp());
                }
            }),
            window_event_listener(ev::pointerup, move |e: ev::PointerEvent| {
                if e.pointer_type() != "touch" {
                    gesture.end(e.time_stamp());
                }
            }),
            window_event_listener(ev::pointercancel, move |e: ev::PointerEvent| {
                if e.pointer_type() != "touch" {
                    gesture.cancel();
                }
            }),
        ];
        on_cleanup(move || handles.into_iter().for_each(|h| h.remove()));
    });

    let on_pointer_down = move |e: ev::PointerEvent| {
        // A new press: whatever drag came before no longer explains the click that follows.
        gesture.dragged.set_value(false);
        if e.pointer_type() == "touch" || e.button() != 0 {
            return;
        }
        let target = e.target().and_then(|t| t.dyn_into::<Element>().ok());
        if target.as_ref().is_some_and(is_text_field) {
            return;
        }
        // Inside the panel only: the dialog element also receives presses on its backdrop.
        let Some(el) = dialog.get_untracked() else {
            return;
        };
        let rect = el.get_bounding_client_rect();
        let (x, y) = (e.client_x() as f64, e.client_y() as f64);
        if x < rect.left() || x > rect.right() || y < rect.top() || y > rect.bottom() {
            return;
        }
        gesture.begin(x, y, e.time_stamp(), target);
    };

    let first_touch = |e: &ev::TouchEvent| {
        e.touches()
            .get(0)
            .map(|t| (t.client_x() as f64, t.client_y() as f64))
    };
    let on_touch_start = move |e: ev::TouchEvent| {
        if e.touches().length() != 1 {
            gesture.cancel();
            return;
        }
        let target = e.target().and_then(|t| t.dyn_into::<Element>().ok());
        if target.as_ref().is_some_and(is_text_field) {
            return;
        }
        if let Some((x, y)) = first_touch(&e) {
            gesture.begin(x, y, e.time_stamp(), target);
        }
    };
    let on_touch_move = move |e: ev::TouchEvent| {
        if let Some((x, y)) = first_touch(&e)
            && gesture.moved(x, y, e.time_stamp())
            && e.cancelable()
        {
            // The drawer has the gesture: do not let the page or the content scroll.
            e.prevent_default();
        }
    };
    let on_touch_end = move |e: ev::TouchEvent| gesture.end(e.time_stamp());
    let on_touch_cancel = move |_: ev::TouchEvent| gesture.cancel();

    let on_keydown = move |e: ev::KeyboardEvent| {
        // A modal dialog gets Escape as `cancel`; a non-modal one does not.
        if !modal && e.key() == "Escape" && dismissible {
            e.prevent_default();
            open.set(false);
        }
    };

    let on_closed = on_close_handler(dialog, open);
    let cancel_handler = on_cancel(open, dismissible);

    view! {
        <dialog
            node_ref=dialog
            class=format!("vdl-drawer {class}")
            data-direction=direction.as_str()
            data-snap-points=has_snaps.then_some("")
            data-modal=modal.then_some("")
            aria-label=label
            on:cancel=cancel_handler
            on:close=move |e: ev::Event| {
                on_closed(e);
                if !modal
                    && let Some(el) = previous_focus.try_update_value(Option::take).flatten()
                {
                    let _ = el.focus();
                }
            }
            on:keydown=on_keydown
            on:pointerdown=on_pointer_down
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
            on:touchcancel=on_touch_cancel
            // A press on the backdrop lands on the dialog itself (it has no padding). A click
            // that ends a drag is not one.
            on:click=move |e: ev::MouseEvent| {
                let was_drag = gesture.dragged.try_get_value().unwrap_or(false);
                gesture.dragged.set_value(false);
                if modal && dismissible && !was_drag && e.target() == e.current_target() {
                    open.set(false);
                }
            }
        >
            {show_handle
                .then(|| {
                    view! {
                        <div
                            class="vdl-drawer__handle"
                            aria-hidden="true"
                            on:click=move |e: ev::MouseEvent| {
                                e.stop_propagation();
                                let was_drag = gesture.dragged.try_get_value().unwrap_or(false);
                                gesture.dragged.set_value(false);
                                let count = gesture.snaps.with_value(Vec::len);
                                if !was_drag && count > 1 {
                                    active.set(next_snap(active.get_untracked(), count));
                                }
                            }
                        ></div>
                    }
                })}
            <div class="vdl-drawer__content">{children()}</div>
        </dialog>
    }
}
