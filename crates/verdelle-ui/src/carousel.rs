use crate::calendar_state::use_synced;
use crate::primitives::carousel::{
    self as model, Direction, ITEM_ATTRIBUTE, KeyAction, LayoutWatcher,
};
use crate::{CarouselGap, Orientation};
use leptos::leptos_dom::helpers::{
    IntervalHandle, TimeoutHandle, set_interval_with_handle, set_timeout_with_handle,
};
use leptos::wasm_bindgen::JsCast;
use leptos::{context::Provider, ev, html, prelude::*};
use std::time::Duration;

/// A pointer drag must move this far before it counts as a drag (and not a click).
const DRAG_START: f64 = 5.0;
/// A drag at least this long turns the page even if it ends nearer the slide it started on.
const DRAG_TURN: f64 = 40.0;
/// How long the scroll must be still before it counts as settled.
const SETTLE_MS: u64 = 120;

#[derive(Clone, Copy)]
struct Drag {
    start: f64,
    scroll: f64,
    index: usize,
    active: bool,
}

/// A handle on a `Carousel`: the current index, the number of snap positions, and ways to move.
/// It is what Embla calls the API. Get it from `Carousel`'s `on_api` callback, or inside the
/// carousel with `use_carousel()`. It is `Copy`.
///
/// An index is a snap position, not a slide: with three slides in view and five slides there are
/// three snaps (see `crate::primitives::carousel`).
#[derive(Clone, Copy)]
pub struct CarouselApi {
    orientation: Orientation,
    looping: bool,
    step: usize,
    draggable: Signal<bool>,
    autoplaying: bool,
    pub(crate) viewport: NodeRef<html::Div>,
    state: RwSignal<Option<usize>>,
    count: RwSignal<usize>,
    snaps: StoredValue<Vec<f64>>,
    target: StoredValue<Option<usize>>,
    settle: StoredValue<Option<TimeoutHandle>>,
    drag: StoredValue<Option<Drag>>,
    on_select: Option<Callback<usize>>,
}

/// The `CarouselApi` of the enclosing `Carousel`, if any.
pub fn use_carousel() -> Option<CarouselApi> {
    use_context::<CarouselApi>()
}

impl CarouselApi {
    pub(crate) fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn vertical(&self) -> bool {
        self.orientation == Orientation::Vertical
    }

    /// The current snap index (reactive).
    pub fn index(&self) -> usize {
        let last = self.count.get().saturating_sub(1);
        self.state.get().unwrap_or(0).min(last)
    }

    /// The number of snap positions (reactive). Zero until the page is interactive.
    pub fn count(&self) -> usize {
        self.count.get()
    }

    /// Whether a step back leads anywhere (reactive): not at the start, unless looping.
    pub fn can_scroll_prev(&self) -> bool {
        model::can_step(
            self.index(),
            self.count(),
            self.looping,
            Direction::Previous,
        )
    }

    /// Whether a step forward leads anywhere (reactive): not at the end, unless looping.
    pub fn can_scroll_next(&self) -> bool {
        model::can_step(self.index(), self.count(), self.looping, Direction::Next)
    }

    pub fn scroll_prev(&self) {
        self.step_towards(Direction::Previous, self.looping);
    }

    pub fn scroll_next(&self) {
        self.step_towards(Direction::Next, self.looping);
    }

    /// Scroll to a snap index (smoothly, unless the user prefers reduced motion).
    pub fn scroll_to(&self, index: usize) {
        let Some(position) = self.snaps.with_value(|s| s.get(index).copied()) else {
            return;
        };
        self.select(index);
        let Some(viewport) = self.viewport.get_untracked() else {
            return;
        };
        let vertical = self.vertical();
        let current = model::scroll_position(&viewport, vertical);
        if (current - position).abs() >= 1.0 {
            self.target.set_value(Some(index));
            model::scroll_to(&viewport, vertical, position, !model::motion_reduced());
        }
        self.schedule_settle();
    }

    fn step_towards(&self, direction: Direction, wrap: bool) {
        let count = self.count.get_untracked();
        let index = self.state.get_untracked().unwrap_or(0);
        self.scroll_to(model::step_index(index, count, self.step, wrap, direction));
    }

    /// Record `index` as the current one and tell the owner.
    fn select(&self, index: usize) {
        if self.state.get_untracked() != Some(index) {
            self.state.set(Some(index));
            if let Some(callback) = self.on_select {
                callback.run(index);
            }
        }
    }

    /// Measure the slides again: snap positions, count, labels.
    fn refresh(&self) {
        let Some(viewport) = self.viewport.get_untracked() else {
            return;
        };
        let snaps = model::measure_snaps(&viewport, self.vertical());
        let last = snaps.len().saturating_sub(1);
        self.count.set(snaps.len());
        self.snaps.set_value(snaps);
        label_slides(&viewport);
        if self.state.get_untracked().is_some_and(|i| i > last) {
            self.select(last);
        }
    }

    /// The scroll position changed: follow it, unless we are scrolling to a chosen target.
    pub(crate) fn on_scroll(&self) {
        let Some(viewport) = self.viewport.get_untracked() else {
            return;
        };
        let position = model::scroll_position(&viewport, self.vertical());
        let snaps = self.snaps.get_value();
        if let Some(target) = self.target.get_value() {
            if snaps
                .get(target)
                .is_some_and(|s| (position - s).abs() < 2.0)
            {
                self.target.set_value(None);
            }
        } else if let Some(index) = model::nearest_snap(&snaps, position) {
            self.select(index);
        }
        self.schedule_settle();
    }

    /// Once the scroll has been still for a moment: forget the target, snap back on, and take
    /// the nearest snap as the index.
    fn schedule_settle(&self) {
        if let Some(handle) = self.settle.get_value() {
            handle.clear();
        }
        let api = *self;
        let handle = set_timeout_with_handle(
            move || {
                if api.drag.get_value().is_some_and(|d| d.active) {
                    return;
                }
                api.target.set_value(None);
                let Some(viewport) = api.viewport.get_untracked() else {
                    return;
                };
                let _ = viewport.remove_attribute("data-snap-off");
                let position = model::scroll_position(&viewport, api.vertical());
                if let Some(index) = api.snaps.with_value(|s| model::nearest_snap(s, position)) {
                    api.select(index);
                }
            },
            Duration::from_millis(SETTLE_MS),
        )
        .ok();
        self.settle.set_value(handle);
    }

    /// The user took over (wheel, touch): stop steering to a chosen target.
    pub(crate) fn interrupt(&self) {
        self.target.set_value(None);
    }

    pub(crate) fn live_region(&self) -> &'static str {
        if self.autoplaying { "off" } else { "polite" }
    }

    // ------------------------------------------------------------- mouse drag

    pub(crate) fn pointer_down(&self, ev: &ev::PointerEvent) {
        if !self.draggable.get_untracked() || ev.pointer_type() != "mouse" || ev.button() != 0 {
            return;
        }
        let Some(viewport) = self.viewport.get_untracked() else {
            return;
        };
        self.interrupt();
        self.drag.set_value(Some(Drag {
            start: self.coordinate(ev),
            scroll: model::scroll_position(&viewport, self.vertical()),
            index: self.state.get_untracked().unwrap_or(0),
            active: false,
        }));
    }

    pub(crate) fn pointer_move(&self, ev: &ev::PointerEvent) {
        let Some(mut drag) = self.drag.get_value() else {
            return;
        };
        let Some(viewport) = self.viewport.get_untracked() else {
            return;
        };
        let delta = self.coordinate(ev) - drag.start;
        if !drag.active {
            if delta.abs() < DRAG_START {
                return;
            }
            drag.active = true;
            self.drag.set_value(Some(drag));
            let _ = viewport.set_attribute("data-dragging", "");
            let _ = viewport.set_attribute("data-snap-off", "");
            let _ = viewport.set_pointer_capture(ev.pointer_id());
        }
        model::scroll_to(&viewport, self.vertical(), drag.scroll - delta, false);
    }

    pub(crate) fn pointer_end(&self, ev: &ev::PointerEvent) {
        let Some(drag) = self.drag.get_value() else {
            return;
        };
        self.drag.set_value(None);
        if !drag.active {
            return;
        }
        let Some(viewport) = self.viewport.get_untracked() else {
            return;
        };
        let _ = viewport.release_pointer_capture(ev.pointer_id());
        let _ = viewport.remove_attribute("data-dragging");
        let delta = self.coordinate(ev) - drag.start;
        let position = model::scroll_position(&viewport, self.vertical());
        let target = self
            .snaps
            .with_value(|s| model::drag_target(s, drag.index, position, delta, DRAG_TURN));
        match target {
            Some(index) => self.scroll_to(index),
            None => self.schedule_settle(),
        }
    }

    fn coordinate(&self, ev: &ev::PointerEvent) -> f64 {
        if self.vertical() {
            ev.client_y() as f64
        } else {
            ev.client_x() as f64
        }
    }

    // --------------------------------------------------------------- keyboard

    fn key_down(&self, ev: &ev::KeyboardEvent) {
        if ev.default_prevented() || ev.alt_key() || ev.ctrl_key() || ev.meta_key() {
            return;
        }
        // Arrow keys belong to text fields and the like.
        let in_field = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|el| {
                el.closest("input, textarea, select, [contenteditable]")
                    .ok()
                    .flatten()
                    .is_some()
            });
        if in_field {
            return;
        }
        let Some(action) = model::key_action(&ev.key(), self.orientation) else {
            return;
        };
        ev.prevent_default();
        match action {
            KeyAction::Step(direction) => self.step_towards(direction, self.looping),
            KeyAction::First => self.scroll_to(0),
            KeyAction::Last => self.scroll_to(self.count.get_untracked().saturating_sub(1)),
        }
    }
}

/// Give each slide an "n of m" name, unless it was given one.
fn label_slides(viewport: &web_sys::HtmlElement) {
    let Ok(items) = viewport.query_selector_all(&format!("[{ITEM_ATTRIBUTE}]")) else {
        return;
    };
    let total = items.length();
    for i in 0..total {
        let Some(el) = items
            .item(i)
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
        else {
            continue;
        };
        if !el.has_attribute("aria-label") || el.has_attribute("data-vdl-auto-label") {
            let _ = el.set_attribute("aria-label", &format!("{} of {}", i + 1, total));
            let _ = el.set_attribute("data-vdl-auto-label", "");
        }
    }
}

/// A carousel on CSS scroll-snap, styled as shadcn/ui's Carousel. shadcn wraps Embla (a
/// JavaScript library); here the browser scrolls and snaps, and a small Rust model
/// (`crate::primitives::carousel`) decides indices, steps and wrapping. Slides swipe natively on
/// touch, drag with the mouse, and step with the arrow keys (Home and End jump to the ends) when
/// focus is inside the carousel.
///
/// Compose it as `Carousel` > `CarouselContent` > `CarouselItem`s, with `CarouselPrevious` and
/// `CarouselNext` (placed outside the slides, so leave room at the sides) and `CarouselDots`.
/// The root is a `role="region"` with `aria-roledescription="carousel"`; give it a name with
/// `label`.
///
/// Slides fill the viewport. Show several with `slides_per_view`, or with basis classes on each
/// `CarouselItem` (`basis-1/2`, responsive ones too). `gap` is the space between slides.
/// `orientation` is horizontal (default) or vertical; a vertical carousel needs a height on
/// `CarouselContent`.
///
/// - `looping`: stepping past either end wraps to the other (a smooth scroll back across the
///   slides, not an endless strip). The buttons stay enabled.
/// - `step`: how many snap positions a button, arrow key or autoplay moves (default 1).
/// - `autoplay`: advance every this many milliseconds, wrapping at the end. It pauses while the
///   pointer is over the carousel, while keyboard focus is inside it and while dragging, and it
///   does not run at all when the user prefers reduced motion.
/// - `draggable`: mouse dragging (default true; touch scrolling is always native).
/// - `index` / `default_index` / `on_select`: the current snap index, controlled with a signal
///   (changing it scrolls there) or seeded; `on_select` reports changes by the user.
/// - `on_api`: receives the `CarouselApi` once the page is interactive.
///
/// The number of snaps, the "n of m" slide labels and the dots come from measuring the slides,
/// which needs the browser, so they appear after hydration.
#[component]
pub fn Carousel(
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] gap: CarouselGap,
    #[prop(optional)] looping: bool,
    /// Slides shown at once (default 1; may be fractional, `2.5`).
    #[prop(default = 1.0)]
    slides_per_view: f64,
    #[prop(default = 1)] step: usize,
    /// Milliseconds between automatic advances; none (default) is off.
    #[prop(optional, into)]
    autoplay: Option<u32>,
    #[prop(default = true.into(), into)] draggable: Signal<bool>,
    #[prop(optional, into)] index: MaybeProp<usize>,
    #[prop(optional, into)] default_index: MaybeProp<usize>,
    #[prop(optional, into)] on_select: Option<Callback<usize>>,
    #[prop(optional, into)] on_api: Option<Callback<CarouselApi>>,
    /// The region's accessible name.
    #[prop(optional, into)]
    label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let viewport = NodeRef::<html::Div>::new();
    let root = NodeRef::<html::Div>::new();
    let pending: StoredValue<Option<CarouselApi>> = StoredValue::new(None);
    let state = use_synced(index, default_index, move |next| {
        if let Some(api) = pending.get_value() {
            api.scroll_to(next.unwrap_or(0));
        }
    });
    let api = CarouselApi {
        orientation,
        looping,
        step,
        draggable,
        autoplaying: autoplay.is_some(),
        viewport,
        state,
        count: RwSignal::new(0),
        snaps: StoredValue::new(Vec::new()),
        target: StoredValue::new(None),
        settle: StoredValue::new(None),
        drag: StoredValue::new(None),
        on_select,
    };
    pending.set_value(Some(api));

    // Measure once the viewport exists, then keep measuring as sizes or slides change.
    let watcher: StoredValue<Option<LayoutWatcher>, LocalStorage> = StoredValue::new_local(None);
    Effect::new(move |_| {
        let Some(element) = viewport.get() else {
            return;
        };
        api.refresh();
        if let Some(position) = api
            .state
            .get_untracked()
            .filter(|i| *i > 0)
            .and_then(|i| api.snaps.with_value(|s| s.get(i).copied()))
        {
            model::scroll_to(&element, api.vertical(), position, false);
        }
        let observed = element.clone();
        let created = LayoutWatcher::new(&element, move || {
            api.refresh();
            watcher.with_value(|w| {
                if let Some(w) = w {
                    w.observe_items(&observed);
                }
            });
        });
        watcher.set_value(created);
        on_cleanup(move || {
            watcher.with_value(|w| {
                if let Some(w) = w {
                    w.disconnect();
                }
            });
        });
        if let Some(callback) = on_api {
            callback.run(api);
        }
    });

    // Autoplay: paused by the pointer, by keyboard focus and by a drag; off for reduced motion.
    let hovered = RwSignal::new(false);
    let focused = RwSignal::new(false);
    if let Some(interval) = autoplay.filter(|ms| *ms > 0) {
        Effect::new(move |_| {
            if model::motion_reduced() {
                return;
            }
            let handle = set_interval_with_handle(
                move || {
                    let busy = hovered.get_untracked()
                        || focused.get_untracked()
                        || api.drag.get_value().is_some();
                    if !busy {
                        api.step_towards(Direction::Next, true);
                    }
                },
                Duration::from_millis(interval as u64),
            )
            .ok();
            on_cleanup(move || {
                let handle: Option<IntervalHandle> = handle;
                if let Some(handle) = handle {
                    handle.clear();
                }
            });
        });
    }

    let per_view = (slides_per_view != 1.0 && slides_per_view > 0.0)
        .then(|| format!("--vdl-carousel-per-view: {slides_per_view}"));

    view! {
        <div
            class=format!("vdl-carousel {class}")
            node_ref=root
            role="region"
            aria-roledescription="carousel"
            aria-label=label
            data-orientation=orientation.to_string()
            data-gap=gap.as_str()
            data-loop=looping.then_some("")
            style=per_view
            on:keydown=move |ev| api.key_down(&ev)
            on:pointerenter=move |ev: ev::PointerEvent| {
                if ev.pointer_type() == "mouse" {
                    hovered.set(true);
                }
            }
            on:pointerleave=move |_| hovered.set(false)
            on:focusin=move |ev: ev::FocusEvent| {
                let keyboard = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .and_then(|el| el.matches(":focus-visible").ok())
                    .unwrap_or(false);
                if keyboard {
                    focused.set(true);
                }
            }
            on:focusout=move |ev: ev::FocusEvent| {
                let inside = ev
                    .related_target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .zip(root.get_untracked())
                    .is_some_and(|(related, root)| root.contains(Some(&related)));
                if !inside {
                    focused.set(false);
                }
            }
        >
            <Provider value=api>{children()}</Provider>
        </div>
    }
}
