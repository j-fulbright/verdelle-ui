//! Navigation Menu: a site-navigation bar whose triggers open panels of links.
//!
//! Behaviour follows Radix's navigation menu (hover intent with an open delay, a "skip
//! delay" so gliding along the bar opens panels at once, a short close delay so the pointer can
//! cross onto the panel, content motion direction between items, a shared `Viewport` that
//! sizes itself to the open panel, an `Indicator`, roving focus along the bar) with Base UI's
//! naming where they differ. Values are strings; the empty string is "closed", as in Radix.
//!
//! Timers are set with `once_into_js` so nothing browser-only is built while rendering
//! (building a wasm-bindgen closure on the server panics).

use crate::primitives::core::direction::{Direction, use_direction};
use crate::primitives::core::id::use_id;
use crate::primitives::core::primitive::Primitive;
use crate::primitives::core::use_controllable_state::{
    UseControllableStateParams, use_controllable_state,
};
use crate::primitives::core::use_size::use_size;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/// How long the pointer may be away from both trigger and panel before the menu closes.
const CLOSE_DELAY_MS: i32 = 150;
/// Longest exit animation we wait for before removing a closed panel (a guard for the case
/// where `animationend` never arrives).
const EXIT_FALLBACK_MS: i32 = 450;
/// Space kept between the viewport and the window edge when it is slid back into view.
const WINDOW_PAD: f64 = 8.0;

fn window() -> web_sys::Window {
    web_sys::window().expect("should have a Window")
}

/// Run `f` once after `ms` milliseconds; returns the timer id.
fn set_timeout(f: impl FnOnce() + 'static, ms: i32) -> i32 {
    let callback = Closure::once_into_js(f);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), ms)
        .expect("set_timeout should succeed")
}

fn clear_timeout(id: i32) {
    window().clear_timeout_with_handle(id);
}

fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false)
}

/// Which way the panel of `me` moves, given the open value, the previous one and the
/// direction of travel along the bar (`Some(true)` when the new item is later than the old
/// one). Entering panels come `from-start`/`from-end`, leaving ones go `to-start`/`to-end`;
/// opening from closed or closing entirely has no direction.
fn motion_for(
    me: &str,
    value: &str,
    previous: &str,
    forward: Option<bool>,
) -> Option<&'static str> {
    let forward = forward?;
    if value == me && !previous.is_empty() {
        Some(if forward { "from-end" } else { "from-start" })
    } else if previous == me && !value.is_empty() {
        Some(if forward { "to-start" } else { "to-end" })
    } else {
        None
    }
}

/// The left edge (window coordinates) for a panel `width` wide that wants to start at
/// `desired`, kept inside a window `window_width` wide.
fn clamp_left(desired: f64, width: f64, window_width: f64) -> f64 {
    let max = window_width - WINDOW_PAD - width;
    desired.min(max).max(WINDOW_PAD)
}

#[derive(Clone, Copy)]
struct Nav {
    /// The open item's value; empty when closed.
    value: Signal<String>,
    set_raw: Callback<String>,
    previous: RwSignal<String>,
    /// Direction of the last move between two open items (see [`motion_for`]).
    forward: RwSignal<Option<bool>>,
    /// Whether opening waits for `delay` (false for a moment after a panel closes).
    is_open_delayed: RwSignal<bool>,
    delay: Signal<i32>,
    skip_delay: Signal<i32>,
    open_timer: RwSignal<Option<i32>>,
    close_timer: RwSignal<Option<i32>>,
    skip_timer: RwSignal<Option<i32>>,
    orientation: Signal<String>,
    direction: Signal<Direction>,
    base_id: StoredValue<String>,
    root_ref: AnyNodeRef,
    viewport_ref: AnyNodeRef,
    has_viewport: RwSignal<bool>,
    /// Size of the open panel, which the viewport takes.
    viewport_size: RwSignal<Option<(f64, f64)>>,
    /// Left edge of the viewport relative to the menu root, in pixels.
    viewport_x: RwSignal<f64>,
    /// Whether the viewport may animate its position and size. Off while it opens from
    /// closed, so it appears in place instead of sliding from where it last was.
    layout_animate: RwSignal<bool>,
    /// The open panel was opened by hovering (so a click on its trigger should not close it).
    hover_opened: RwSignal<bool>,
    layout_timer: RwSignal<Option<i32>>,
}

fn clear(timer: RwSignal<Option<i32>>) {
    if let Some(id) = timer.try_get_untracked().flatten() {
        clear_timeout(id);
        timer.try_set(None);
    }
}

impl Nav {
    fn clear_timers(&self) {
        clear(self.open_timer);
        clear(self.close_timer);
        clear(self.skip_timer);
        clear(self.layout_timer);
    }

    /// Position of a trigger in the bar (document order, reversed in RTL).
    fn index_of(&self, value: &str) -> Option<usize> {
        let root = self.root_ref.get_untracked()?;
        let root: &web_sys::Element = root.unchecked_ref();
        let mut values = trigger_values(root);
        if self.direction.get_untracked() == Direction::Rtl {
            values.reverse();
        }
        values.iter().position(|v| v == value)
    }

    fn set_value(&self, next: String) {
        let current = self.value.get_untracked();
        if current == next {
            return;
        }
        let forward = if !current.is_empty() && !next.is_empty() {
            match (self.index_of(&current), self.index_of(&next)) {
                (Some(a), Some(b)) if a != b => Some(b > a),
                _ => None,
            }
        } else {
            None
        };
        if current.is_empty() && !next.is_empty() {
            // Opening from closed: start from nothing, in place, and only allow the box to
            // animate between items once it has settled.
            self.viewport_size.set(None);
            self.place_viewport(&next, 0.0);
            self.layout_animate.set(false);
            clear(self.layout_timer);
            let (animate, timer) = (self.layout_animate, self.layout_timer);
            let id = set_timeout(
                move || {
                    timer.try_set(None);
                    animate.try_set(true);
                },
                100,
            );
            self.layout_timer.set(Some(id));
        }
        self.forward.set(forward);
        self.previous.set(current);
        clear(self.skip_timer);
        if next.is_empty() {
            let flag = self.is_open_delayed;
            let timer = self.skip_timer;
            let id = set_timeout(
                move || {
                    timer.try_set(None);
                    flag.try_set(true);
                },
                self.skip_delay.get_untracked(),
            );
            self.skip_timer.set(Some(id));
        } else if self.skip_delay.get_untracked() > 0 {
            self.is_open_delayed.set(false);
        }
        self.set_raw.run(next);
    }

    fn open_now(&self, value: String) {
        clear(self.close_timer);
        clear(self.open_timer);
        self.set_value(value);
    }

    fn delayed_open(&self, value: String) {
        if self.value.get_untracked() == value {
            clear(self.close_timer);
            return;
        }
        clear(self.open_timer);
        let nav = *self;
        let timer = self.open_timer;
        let id = set_timeout(
            move || {
                timer.try_set(None);
                clear(nav.close_timer);
                nav.hover_opened.try_set(true);
                nav.set_value(value);
            },
            self.delay.get_untracked(),
        );
        self.open_timer.set(Some(id));
    }

    fn start_close_timer(&self) {
        clear(self.close_timer);
        let nav = *self;
        let timer = self.close_timer;
        let id = set_timeout(
            move || {
                timer.try_set(None);
                nav.set_value(String::new());
            },
            CLOSE_DELAY_MS,
        );
        self.close_timer.set(Some(id));
    }

    fn trigger_enter(&self, value: String) {
        clear(self.open_timer);
        if self.is_open_delayed.get_untracked() {
            self.delayed_open(value);
        } else {
            self.hover_opened.set(true);
            self.open_now(value);
        }
    }

    fn trigger_leave(&self) {
        clear(self.open_timer);
        self.start_close_timer();
    }

    fn content_enter(&self) {
        clear(self.close_timer);
    }

    fn content_leave(&self) {
        self.start_close_timer();
    }

    fn select(&self, value: String) {
        clear(self.open_timer);
        clear(self.close_timer);
        if self.value.get_untracked() == value {
            if self.hover_opened.get_untracked() {
                // Hover opened it and the click is just the user confirming: keep it open.
                self.hover_opened.set(false);
            } else {
                self.set_value(String::new());
            }
        } else {
            self.hover_opened.set(false);
            self.set_value(value);
        }
    }

    fn dismiss(&self) {
        clear(self.open_timer);
        clear(self.close_timer);
        self.set_value(String::new());
    }

    fn trigger_id(&self, value: &str) -> String {
        format!("{}-trigger-{}", self.base_id.get_value(), value)
    }

    fn content_id(&self, value: &str) -> String {
        format!("{}-content-{}", self.base_id.get_value(), value)
    }

    /// Put the viewport under the open item's trigger, kept inside the window.
    fn place_viewport(&self, value: &str, width: f64) {
        let Some(root) = self.root_ref.get_untracked() else {
            return;
        };
        let root: &web_sys::Element = root.unchecked_ref();
        let Some(trigger) = find_trigger(root, value) else {
            return;
        };
        let window_width = window()
            .document()
            .and_then(|d| d.document_element())
            .map(|e| e.client_width() as f64)
            .unwrap_or(0.0);
        let root_left = root.get_bounding_client_rect().left();
        let desired = trigger.get_bounding_client_rect().left();
        let left = if window_width > 0.0 {
            clamp_left(desired, width, window_width)
        } else {
            desired
        };
        self.viewport_x.set(left - root_left);
    }
}

fn trigger_elements(root: &web_sys::Element) -> Vec<web_sys::Element> {
    let mut out = vec![];
    if let Ok(list) = root.query_selector_all("[data-vdl-nav-trigger]") {
        for i in 0..list.length() {
            if let Some(el) = list
                .item(i)
                .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
            {
                out.push(el);
            }
        }
    }
    out
}

fn trigger_values(root: &web_sys::Element) -> Vec<String> {
    trigger_elements(root)
        .iter()
        .filter_map(|el| el.get_attribute("data-nav-value"))
        .collect()
}

fn find_trigger(root: &web_sys::Element, value: &str) -> Option<web_sys::Element> {
    trigger_elements(root)
        .into_iter()
        .find(|el| el.get_attribute("data-nav-value").as_deref() == Some(value))
}

const FOCUSABLE: &str = "a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";

fn focusables(container: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let mut out = vec![];
    if let Ok(list) = container.query_selector_all(FOCUSABLE) {
        for i in 0..list.length() {
            if let Some(el) = list
                .item(i)
                .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
            {
                out.push(el);
            }
        }
    }
    out
}

fn focus(el: &web_sys::HtmlElement) {
    let _ = el.focus();
}

/// A keyboard-reachable item of the bar: a trigger or a link outside any panel.
fn bar_items(current: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let Ok(Some(list)) = current.closest("[data-vdl-nav-list]") else {
        return vec![];
    };
    let mut out = vec![];
    if let Ok(nodes) = list.query_selector_all("[data-vdl-nav-item]") {
        for i in 0..nodes.length() {
            if let Some(el) = nodes
                .item(i)
                .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
            {
                let same_list = el
                    .closest("[data-vdl-nav-list]")
                    .ok()
                    .flatten()
                    .is_some_and(|l| l == list);
                if same_list && !el.has_attribute("disabled") {
                    out.push(el);
                }
            }
        }
    }
    out
}

/// Arrow keys, Home and End move focus along the bar. Returns whether the key was used.
fn bar_keydown(event: &KeyboardEvent, nav: &Nav) -> bool {
    let Some(current) = event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    else {
        return false;
    };
    let horizontal = nav.orientation.get_untracked() != "vertical";
    let rtl = nav.direction.get_untracked() == Direction::Rtl;
    let (prev_key, next_key) = if horizontal {
        if rtl {
            ("ArrowRight", "ArrowLeft")
        } else {
            ("ArrowLeft", "ArrowRight")
        }
    } else {
        ("ArrowUp", "ArrowDown")
    };
    let key = event.key();
    let items = bar_items(&current);
    let Some(idx) = items.iter().position(|el| {
        let node: &web_sys::Node = el.unchecked_ref();
        node == current.unchecked_ref::<web_sys::Node>()
    }) else {
        return false;
    };
    let len = items.len();
    let target = if key == next_key {
        (idx + 1) % len
    } else if key == prev_key {
        (idx + len - 1) % len
    } else if key == "Home" {
        0
    } else if key == "End" {
        len - 1
    } else {
        return false;
    };
    event.prevent_default();
    focus(&items[target]);
    true
}

/// Keeps something mounted while its exit animation plays. `present` turns true as soon as
/// `open` does and false only once the animation ends (or a fallback timer fires, at once
/// under reduced motion). Attach `on_end` to the element's `animationend`.
fn use_exit_presence(open: Signal<bool>) -> (Signal<bool>, Callback<()>) {
    let present = RwSignal::new(open.get_untracked());
    let timer: RwSignal<Option<i32>> = RwSignal::new(None);

    Effect::new(move |_| {
        let is_open = open.get();
        clear(timer);
        if is_open {
            present.set(true);
        } else if present.get_untracked() {
            let ms = if prefers_reduced_motion() {
                0
            } else {
                EXIT_FALLBACK_MS
            };
            let id = set_timeout(
                move || {
                    timer.try_set(None);
                    present.try_set(false);
                },
                ms,
            );
            timer.set(Some(id));
        }
    });
    on_cleanup(move || clear(timer));

    let on_end = Callback::new(move |()| {
        if !open.get_untracked() {
            clear(timer);
            present.try_set(false);
        }
    });
    (present.into(), on_end)
}

/// Run `on_end` for an `animationend` that belongs to the element itself, not a descendant.
fn own_animation_end(event: &web_sys::AnimationEvent, on_end: Callback<()>) {
    if event.target() == event.current_target() {
        on_end.run(());
    }
}

#[allow(clippy::too_many_arguments)]
fn make_nav(
    value: Signal<String>,
    set_raw: Callback<String>,
    delay: Signal<i32>,
    skip_delay: Signal<i32>,
    orientation: Signal<String>,
    direction: Signal<Direction>,
    root_ref: AnyNodeRef,
) -> Nav {
    Nav {
        value,
        set_raw,
        previous: RwSignal::new(String::new()),
        forward: RwSignal::new(None),
        is_open_delayed: RwSignal::new(true),
        delay,
        skip_delay,
        open_timer: RwSignal::new(None),
        close_timer: RwSignal::new(None),
        skip_timer: RwSignal::new(None),
        orientation,
        direction,
        base_id: StoredValue::new(use_id(None).get_untracked()),
        root_ref,
        viewport_ref: AnyNodeRef::new(),
        has_viewport: RwSignal::new(false),
        viewport_size: RwSignal::new(None),
        viewport_x: RwSignal::new(0.0),
        layout_animate: RwSignal::new(true),
        hover_opened: RwSignal::new(false),
        layout_timer: RwSignal::new(None),
    }
}

#[component]
pub fn NavigationMenu(
    /// The open item's value; empty when nothing is open.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    /// Called with the new open value, or an empty string when the menu closes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Milliseconds the pointer must rest on a trigger before its panel opens. Default 200.
    #[prop(into, optional)]
    delay_duration: MaybeProp<i32>,
    /// Milliseconds after a panel closes during which another opens without the delay.
    /// Default 300.
    #[prop(into, optional)]
    skip_delay_duration: MaybeProp<i32>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    /// Accessible name of the `nav` landmark. Default "Main".
    #[prop(into, optional)]
    label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let direction = use_direction(dir);
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let root_ref = AnyNodeRef::new();
    let composed =
        crate::primitives::core::compose_refs::use_composed_refs(vec![node_ref, root_ref]);

    let (state, set_state) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change
            .map(|cb| Callback::new(move |v: Option<String>| cb.run(v.unwrap_or_default()))),
        default_prop: default_value,
    });
    let value = Signal::derive(move || state.get().unwrap_or_default());

    let nav = make_nav(
        value,
        Callback::new(move |v: String| set_state.run(Some(v))),
        Signal::derive(move || delay_duration.get().unwrap_or(200)),
        Signal::derive(move || skip_delay_duration.get().unwrap_or(300)),
        orientation,
        direction,
        root_ref,
    );
    on_cleanup(move || nav.clear_timers());

    view! {
        <Provider value=nav>
            <Primitive element=html::nav as_child=as_child node_ref=composed
                attr:aria-label=move || label.get().unwrap_or("Main".into())
                attr:data-orientation=move || orientation.get()
                attr:data-state=move || if value.get().is_empty() { "closed" } else { "open" }
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let nav = expect_context::<Nav>();

    view! {
        <Primitive element=html::ul as_child=as_child node_ref=node_ref
            attr:data-vdl-nav-list=""
            attr:data-orientation=move || nav.orientation.get()
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[derive(Clone)]
struct ItemContext {
    value: String,
}

/// Marks that we are inside a panel, so links there are not part of the bar's focus order.
#[derive(Clone, Copy)]
struct InContent;

#[component]
pub fn NavigationMenuItem(
    /// Identifies the item; its trigger and content share it. Generated when omitted.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = value
        .get_untracked()
        .unwrap_or_else(|| use_id(None).get_untracked());

    view! {
        <Provider value=ItemContext { value }>
            <Primitive element=html::li as_child=as_child node_ref=node_ref>
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let nav = expect_context::<Nav>();
    let item = expect_context::<ItemContext>();
    let value = StoredValue::new(item.value.clone());
    let is_open = Signal::derive(move || nav.value.get() == value.get_value());
    let trigger_id = nav.trigger_id(&item.value);
    let content_id = StoredValue::new(nav.content_id(&item.value));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    // A press that closed the panel must not be undone by the pointer move that follows.
    let was_click_close = StoredValue::new(false);
    let has_pointer_move_opened = StoredValue::new(false);

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:id=trigger_id
            attr:r#type="button"
            attr:disabled=move || disabled.get().then_some("")
            attr:data-vdl-nav-trigger=""
            attr:data-vdl-nav-item=""
            attr:data-nav-value=item.value.clone()
            attr:aria-expanded=move || is_open.get().to_string()
            attr:aria-controls=move || is_open.get().then(|| content_id.get_value())
            attr:data-state=move || if is_open.get() { "open" } else { "closed" }
            on:pointerenter=move |_: leptos::ev::PointerEvent| was_click_close.set_value(false)
            on:pointermove=move |e: leptos::ev::PointerEvent| {
                if e.pointer_type() != "mouse" {
                    return;
                }
                if disabled.get_untracked()
                    || was_click_close.get_value()
                    || has_pointer_move_opened.get_value()
                {
                    return;
                }
                nav.trigger_enter(value.get_value());
                has_pointer_move_opened.set_value(true);
            }
            on:pointerleave=move |e: leptos::ev::PointerEvent| {
                if e.pointer_type() != "mouse" || disabled.get_untracked() {
                    return;
                }
                nav.trigger_leave();
                has_pointer_move_opened.set_value(false);
            }
            on:click=move |_| {
                nav.select(value.get_value());
                was_click_close.set_value(!is_open.get_untracked());
            }
            on:keydown=move |event: KeyboardEvent| {
                let entry_key = if nav.orientation.get_untracked() == "vertical" {
                    if nav.direction.get_untracked() == Direction::Rtl { "ArrowLeft" } else { "ArrowRight" }
                } else {
                    "ArrowDown"
                };
                let key = event.key();
                if key == entry_key {
                    // Open the panel and move into it.
                    event.prevent_default();
                    nav.hover_opened.set(false);
                    nav.open_now(value.get_value());
                    let id = content_id.get_value();
                    set_timeout(move || focus_content(&id, false), 30);
                } else if key == "Escape" && is_open.get_untracked() {
                    event.prevent_default();
                    nav.dismiss();
                } else {
                    bar_keydown(&event, &nav);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// Focus the first (or last) focusable thing in the panel with this id.
fn focus_content(id: &str, last: bool) {
    let Some(content) = window().document().and_then(|d| d.get_element_by_id(id)) else {
        return;
    };
    let items = focusables(&content);
    let target = if last { items.last() } else { items.first() };
    if let Some(el) = target {
        focus(el);
    }
}

#[component]
pub fn NavigationMenuContent(
    /// Keep the panel mounted while closed.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    /// Class for the panel element (`attr:class` cannot reach it: this component's root is a
    /// `Show`).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let nav = expect_context::<Nav>();
    let item = expect_context::<ItemContext>();
    let value = StoredValue::new(item.value.clone());
    let is_open = Signal::derive(move || nav.value.get() == value.get_value());
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let (present, on_end) = use_exit_presence(is_open);

    // Motion attribute: only the entering and the leaving panel change it, so a panel that
    // is still leaving keeps its direction while a third one opens.
    let motion_state: StoredValue<Option<&'static str>> = StoredValue::new(None);
    let motion = Signal::derive(move || {
        let (v, p, f) = (nav.value.get(), nav.previous.get(), nav.forward.get());
        let me = value.get_value();
        if v == me || p == me {
            motion_state.set_value(motion_for(&me, &v, &p, f));
        }
        motion_state.get_value()
    });

    // The panel itself (which builds browser-only hooks) is created only after hydration, and
    // only while present, so the server never renders it.
    let mounted = RwSignal::new(false);
    Effect::new(move |_| mounted.set(true));

    view! {
        <Show when=move || mounted.get() && (force_mount.get() || present.get())>
            {move || {
                let panel = move || {
                    view! {
                        <NavPanel
                            value=value.get_value()
                            class=class
                            as_child=as_child
                            node_ref=node_ref
                            is_open=is_open
                            motion=motion
                            on_end=on_end
                        >
                            {children.with_value(|c| c())}
                        </NavPanel>
                    }
                };
                if nav.has_viewport.get() {
                    // The panel lives inside the viewport so it can size and animate it.
                    nav.viewport_ref
                        .get()
                        .map(|_| {
                            view! {
                                <crate::primitives::core::portal::Portal container_ref=nav.viewport_ref>
                                    {panel()}
                                </crate::primitives::core::portal::Portal>
                            }
                        })
                        .into_any()
                } else {
                    panel().into_any()
                }
            }}
        </Show>
    }
}

/// One panel's element: dismissal, sizing the viewport, keyboard and motion.
#[component]
fn NavPanel(
    value: String,
    class: MaybeProp<String>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    is_open: Signal<bool>,
    motion: Signal<Option<&'static str>>,
    on_end: Callback<()>,
    children: ChildrenFn,
) -> impl IntoView {
    let nav = expect_context::<Nav>();
    let children = StoredValue::new(children);
    let content_id_stored = StoredValue::new(nav.content_id(&value));
    let trigger_id = StoredValue::new(nav.trigger_id(&value));
    let value = StoredValue::new(value);

    let content_ref = AnyNodeRef::new();
    let size = use_size(content_ref);
    // Tell the viewport how big the open panel is and where to stand.
    Effect::new(move |_| {
        if !is_open.get() || !nav.has_viewport.get() {
            return;
        }
        if let Some(s) = size.get() {
            nav.viewport_size.set(Some((s.width, s.height)));
            nav.place_viewport(&value.get_value(), s.width);
        }
    });

    // Escape closes the menu and returns focus to the trigger. A press outside closes it too,
    // unless it landed in the menu itself (the trigger toggles on its own click).
    let dismiss_ref = crate::primitives::core::dismissable_layer::use_dismissable_layer_with(
        Some(Callback::new(move |_: web_sys::KeyboardEvent| {
            if let Some(el) = window()
                .document()
                .and_then(|d| d.get_element_by_id(&trigger_id.get_value()))
                .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
            {
                focus(&el);
            }
        })),
        Some(Callback::new(move |e: web_sys::PointerEvent| {
            let inside_menu = nav
                .root_ref
                .get_untracked()
                .zip(e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()))
                .is_some_and(|(root, target)| root.contains(Some(&target)));
            if inside_menu {
                e.prevent_default();
            }
        })),
        None,
        None,
        Some(Callback::new(move |()| nav.dismiss())),
        Signal::derive(move || !is_open.get()),
        false,
    );
    let composed = crate::primitives::core::compose_refs::use_composed_refs(vec![
        node_ref,
        content_ref,
        dismiss_ref,
    ]);

    let panel = move || {
        view! {
            <Primitive element=html::div as_child=as_child node_ref=composed
                attr:class=move || class.get()
                attr:id=content_id_stored.get_value()
                attr:role="group"
                attr:aria-labelledby=trigger_id.get_value()
                attr:data-vdl-nav-content=""
                attr:data-state=move || if is_open.get() { "open" } else { "closed" }
                attr:data-motion=move || motion.get()
                attr:data-orientation=move || nav.orientation.get()
                on:pointerenter=move |e: leptos::ev::PointerEvent| {
                    if e.pointer_type() == "mouse" {
                        nav.content_enter();
                    }
                }
                on:pointerleave=move |e: leptos::ev::PointerEvent| {
                    if e.pointer_type() == "mouse" {
                        nav.content_leave();
                    }
                }
                on:animationend=move |e: web_sys::AnimationEvent| own_animation_end(&e, on_end)
                on:animationcancel=move |e: web_sys::AnimationEvent| own_animation_end(&e, on_end)
                on:keydown=move |event: KeyboardEvent| {
                    let vertical = nav.orientation.get_untracked() == "vertical";
                    let (prev_key, next_key) = if vertical {
                        ("ArrowLeft", "ArrowRight")
                    } else {
                        ("ArrowUp", "ArrowDown")
                    };
                    let key = event.key();
                    if key != prev_key && key != next_key {
                        return;
                    }
                    let Some(content) = window()
                        .document()
                        .and_then(|d| d.get_element_by_id(&content_id_stored.get_value()))
                    else {
                        return;
                    };
                    let items = focusables(&content);
                    let active = window().document().and_then(|d| d.active_element());
                    let idx = active.and_then(|a| {
                        items.iter().position(|el| {
                            let node: &web_sys::Node = el.unchecked_ref();
                            node == a.unchecked_ref::<web_sys::Node>()
                        })
                    });
                    event.prevent_default();
                    event.stop_propagation();
                    if key == next_key {
                        let next = idx.map(|i| (i + 1) % items.len()).unwrap_or(0);
                        if let Some(el) = items.get(next) {
                            focus(el);
                        }
                    } else if idx.unwrap_or(0) == 0 {
                        // Up from the first item goes back to the trigger.
                        if let Some(el) = window()
                            .document()
                            .and_then(|d| d.get_element_by_id(&trigger_id.get_value()))
                            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            focus(&el);
                        }
                    } else if let Some(el) = idx.and_then(|i| items.get(i - 1)) {
                        focus(el);
                    }
                }
            >
                <Provider value=InContent>
                    {children.with_value(|c| c())}
                </Provider>
            </Primitive>
        }
    };
    panel()
}

#[component]
pub fn NavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let nav = expect_context::<Nav>();
    let active = Signal::derive(move || active.get().unwrap_or(false));
    let in_content = use_context::<InContent>().is_some();

    view! {
        <Primitive element=html::a as_child=as_child node_ref=node_ref
            attr:data-active=move || active.get().then_some("")
            attr:aria-current=move || active.get().then_some("page")
            attr:data-vdl-nav-item=(!in_content).then_some("")
            on:click=move |_| {
                if let Some(cb) = on_select {
                    cb.run(());
                }
                nav.dismiss();
            }
            on:keydown=move |event: KeyboardEvent| {
                if !in_content {
                    bar_keydown(&event, &nav);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// The shared panel the open item's content is shown in. It takes the open panel's size as
/// `--vdl-nav-viewport-width` / `--vdl-nav-viewport-height` and its left edge (relative to
/// the menu) as `--vdl-nav-viewport-x`, so CSS can animate between items.
///
/// `class` goes on the outer positioning element, `viewport_class` on the inner clipping
/// element that holds the panels.
#[component]
pub fn NavigationMenuViewport(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] viewport_class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let nav = expect_context::<Nav>();
    nav.has_viewport.set(true);
    on_cleanup(move || {
        nav.has_viewport.try_set(false);
    });
    let is_open = Signal::derive(move || !nav.value.get().is_empty());
    let (present, on_end) = use_exit_presence(is_open);
    let composed =
        crate::primitives::core::compose_refs::use_composed_refs(vec![node_ref, nav.viewport_ref]);

    view! {
        <Show when=move || present.get()>
            <div
                class=move || class.get()
                data-vdl-nav-viewport-position=""
                data-instant=move || (!nav.layout_animate.get()).then_some("")
                data-state=move || if is_open.get() { "open" } else { "closed" }
                data-orientation=move || nav.orientation.get()
                style=move || format!("--vdl-nav-viewport-x:{}px", nav.viewport_x.get())
                on:pointerenter=move |e: leptos::ev::PointerEvent| {
                    if e.pointer_type() == "mouse" {
                        nav.content_enter();
                    }
                }
                on:pointerleave=move |e: leptos::ev::PointerEvent| {
                    if e.pointer_type() == "mouse" {
                        nav.content_leave();
                    }
                }
                on:animationend=move |e: web_sys::AnimationEvent| own_animation_end(&e, on_end)
                on:animationcancel=move |e: web_sys::AnimationEvent| own_animation_end(&e, on_end)
            >
                <Primitive element=html::div node_ref=composed
                    attr:class=move || viewport_class.get()
                    attr:data-vdl-nav-viewport=""
                    attr:data-state=move || if is_open.get() { "open" } else { "closed" }
                    attr:data-orientation=move || nav.orientation.get()
                    attr:style=move || {
                        nav.viewport_size
                            .get()
                            .map(|(w, h)| {
                                format!("--vdl-nav-viewport-width:{w}px;--vdl-nav-viewport-height:{h}px")
                            })
                    }
                >
                    {None::<String>}
                </Primitive>
            </div>
        </Show>
    }
}

/// A nested menu inside a panel: its own open value, delays and focus group, no viewport.
#[component]
pub fn NavigationMenuSub(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let parent = expect_context::<Nav>();
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let root_ref = AnyNodeRef::new();
    let composed =
        crate::primitives::core::compose_refs::use_composed_refs(vec![node_ref, root_ref]);

    let (state, set_state) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change
            .map(|cb| Callback::new(move |v: Option<String>| cb.run(v.unwrap_or_default()))),
        default_prop: default_value,
    });
    let value = Signal::derive(move || state.get().unwrap_or_default());
    let nav = make_nav(
        value,
        Callback::new(move |v: String| set_state.run(Some(v))),
        parent.delay,
        parent.skip_delay,
        orientation,
        parent.direction,
        root_ref,
    );
    on_cleanup(move || nav.clear_timers());

    view! {
        <Provider value=nav>
            <Primitive element=html::div as_child=as_child node_ref=composed
                attr:data-orientation=move || orientation.get()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

/// An arrow that follows the open trigger and points at the panel. Position and size come as
/// `--vdl-nav-indicator-pos` / `--vdl-nav-indicator-size` (pixels, relative to the menu).
#[component]
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let nav = expect_context::<Nav>();
    let is_visible = Signal::derive(move || !nav.value.get().is_empty());
    let (present, on_end) = use_exit_presence(is_visible);
    let position: RwSignal<Option<(f64, f64)>> = RwSignal::new(None);

    // Forget the old spot once hidden, so it reappears in place rather than sliding there.
    Effect::new(move |_| {
        if !present.get() {
            position.set(None);
        }
    });
    Effect::new(move |_| {
        let value = nav.value.get();
        if value.is_empty() {
            return;
        }
        let Some(root) = nav.root_ref.get_untracked() else {
            return;
        };
        let root: &web_sys::Element = root.unchecked_ref();
        if let Some(trigger) = find_trigger(root, &value) {
            let r = trigger.get_bounding_client_rect();
            position.set(Some((
                r.left() - root.get_bounding_client_rect().left(),
                r.width(),
            )));
        }
    });

    view! {
        <Show when=move || present.get() && position.get().is_some()>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-state=move || if is_visible.get() { "visible" } else { "hidden" }
                attr:data-orientation=move || nav.orientation.get()
                attr:class=move || class.get()
                attr:aria-hidden="true"
                attr:style=move || {
                    position
                        .get()
                        .map(|(x, w)| format!("--vdl-nav-indicator-pos:{x}px;--vdl-nav-indicator-size:{w}px"))
                }
                on:animationend=move |e: web_sys::AnimationEvent| own_animation_end(&e, on_end)
                on:animationcancel=move |e: web_sys::AnimationEvent| own_animation_end(&e, on_end)
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </Primitive>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_follows_the_direction_of_travel() {
        // Moving from "a" to a later item "b": b enters from the end, a leaves toward the start.
        assert_eq!(motion_for("b", "b", "a", Some(true)), Some("from-end"));
        assert_eq!(motion_for("a", "b", "a", Some(true)), Some("to-start"));
        // Moving back to an earlier item reverses both.
        assert_eq!(motion_for("a", "a", "b", Some(false)), Some("from-start"));
        assert_eq!(motion_for("b", "a", "b", Some(false)), Some("to-end"));
    }

    #[test]
    fn opening_from_closed_or_closing_has_no_motion() {
        // Nothing was open before, or nothing is open now: no direction is ever computed.
        assert_eq!(motion_for("a", "a", "", None), None);
        assert_eq!(motion_for("a", "", "a", None), None);
        // Even with a stale direction, a panel entering from closed does not slide.
        assert_eq!(motion_for("a", "a", "", Some(true)), None);
        assert_eq!(motion_for("a", "", "a", Some(true)), None);
    }

    #[test]
    fn unrelated_panels_are_untouched() {
        assert_eq!(motion_for("c", "b", "a", Some(true)), None);
    }

    #[test]
    fn viewport_is_kept_inside_the_window() {
        // Fits where it wants to be.
        assert_eq!(clamp_left(100.0, 300.0, 1000.0), 100.0);
        // Would overflow on the right: slide left, keeping 8px clear.
        assert_eq!(clamp_left(900.0, 300.0, 1000.0), 692.0);
        // Wider than the window: pinned to the left padding.
        assert_eq!(clamp_left(50.0, 2000.0, 1000.0), 8.0);
        // Desired position left of the padding: pushed in.
        assert_eq!(clamp_left(-20.0, 100.0, 1000.0), 8.0);
    }
}
