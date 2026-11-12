//! `ResizablePanelGroup`, `ResizablePanel` and `ResizableHandle`.
//!
//! A group lays its panels out in a row (`Horizontal`) or a column (`Vertical`); a handle
//! between two panels is a `role="separator"` that can be dragged (with pointer capture) or
//! moved with the arrow keys. Sizes are percentages (see [`super::model`]); a panel's
//! `flex-grow` is its size, so the browser does the layout and only the handle's own width
//! is left out of the shared space.
//!
//! Panels and handles register with the group while they are built, in order, so they must
//! not appear or disappear later (use a collapsible panel to hide one). Groups nest: a
//! panel may hold another group.

use super::model::{self, PanelConstraints, Snap};
use crate::primitives::separator::Orientation;
use leptos::{
    context::Provider,
    ev::{KeyboardEvent, PointerEvent},
    html,
    prelude::*,
};
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone)]
struct Drag {
    pointer_id: i32,
    start_px: f64,
    start_sizes: Vec<f64>,
    /// Pixels the panels share along the group's axis, measured when the drag began.
    total_px: f64,
}

#[derive(Clone)]
struct GroupContext {
    group_id: String,
    orientation: Signal<Orientation>,
    sizes: RwSignal<Vec<f64>>,
    constraints: StoredValue<Vec<PanelConstraints>>,
    defaults: StoredValue<Vec<Option<f64>>>,
    /// The last size each panel had while open, for expanding it again.
    last_open: StoredValue<Vec<f64>>,
    next_handle: StoredValue<usize>,
    group_ref: NodeRef<html::Div>,
    /// The handle being dragged.
    active: RwSignal<Option<usize>>,
    drag: StoredValue<Option<Drag>>,
    keyboard_step: f64,
    on_layout_change: Option<Callback<Vec<f64>>>,
    on_layout_end: Option<Callback<Vec<f64>>>,
}

impl GroupContext {
    fn panel_id(&self, idx: usize) -> String {
        format!("{}-panel-{idx}", self.group_id)
    }

    /// Take `new` as the layout if it differs from the current one.
    fn apply(&self, new: Vec<f64>) {
        let changed = self.sizes.with_untracked(|cur| {
            cur.len() != new.len()
                || cur
                    .iter()
                    .zip(&new)
                    .any(|(a, b)| (a - b).abs() > model::EPS)
        });
        if !changed {
            return;
        }
        self.remember_open(&new);
        self.sizes.set(new.clone());
        if let Some(cb) = self.on_layout_change {
            cb.run(new);
        }
    }

    fn remember_open(&self, sizes: &[f64]) {
        let c = self.constraints.get_value();
        self.last_open.update_value(|last| {
            last.resize(sizes.len(), 0.0);
            for (i, s) in sizes.iter().enumerate() {
                if !c[i].is_collapsed(*s) && *s > model::EPS {
                    last[i] = *s;
                }
            }
        });
    }

    fn commit(&self) {
        if let Some(cb) = self.on_layout_end {
            cb.run(self.sizes.get_untracked());
        }
    }

    fn collapse(&self, idx: usize) {
        let c = self.constraints.get_value();
        let sizes = self.sizes.get_untracked();
        self.apply(model::collapse_panel(&sizes, &c, idx));
    }

    fn expand(&self, idx: usize) {
        let c = self.constraints.get_value();
        let sizes = self.sizes.get_untracked();
        let to = self
            .last_open
            .with_value(|l| l.get(idx).copied())
            .filter(|v| *v > 0.0)
            .unwrap_or(100.0 / sizes.len().max(1) as f64);
        self.apply(model::expand_panel(&sizes, &c, idx, to));
    }

    fn panel_is_collapsed(&self, idx: usize) -> bool {
        let c = self.constraints.get_value();
        self.sizes
            .with_untracked(|s| s.get(idx).is_some_and(|v| c[idx].is_collapsed(*v)))
    }

    /// Enter on a handle: open a collapsed neighbour, else collapse a collapsible one.
    fn toggle_at_handle(&self, handle: usize) {
        let neighbours = [handle, handle + 1];
        if let Some(i) = neighbours.into_iter().find(|i| self.panel_is_collapsed(*i)) {
            self.expand(i);
            return;
        }
        let c = self.constraints.get_value();
        if let Some(i) = neighbours
            .into_iter()
            .find(|i| c.get(*i).is_some_and(|c| c.collapsible))
        {
            self.collapse(i);
        }
    }
}

/// A row or column of resizable panels.
///
/// `orientation` is the way the panels are laid out: `Horizontal` puts them side by side
/// (handles are vertical lines), `Vertical` stacks them. `sizes` is an optional two-way
/// signal holding the layout in percent (one entry per panel, adding up to 100): start it
/// with a saved layout to restore one, or leave it empty to use the panels' `default_size`;
/// the group keeps it up to date, and writing to it moves the panels. `on_layout_change`
/// runs on every change, `on_layout_end` once when a drag or key press is done (save the
/// layout there). `keyboard_step` is the percent one arrow press moves a handle (Shift
/// moves ten times as much).
#[component]
pub fn ResizablePanelGroup(
    #[prop(into, optional)] orientation: Signal<Orientation>,
    #[prop(default = None)] sizes: Option<RwSignal<Vec<f64>>>,
    #[prop(default = None)] on_layout_change: Option<Callback<Vec<f64>>>,
    #[prop(default = None)] on_layout_end: Option<Callback<Vec<f64>>>,
    #[prop(default = 1.0)] keyboard_step: f64,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] style: String,
    #[prop(default = None)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let internal = RwSignal::new(Vec::<f64>::new());
    let group_ref = NodeRef::<html::Div>::new();
    let ctx = GroupContext {
        group_id: crate::primitives::core::id::use_id(None).get_untracked(),
        orientation,
        sizes: internal,
        constraints: StoredValue::new(Vec::new()),
        defaults: StoredValue::new(Vec::new()),
        last_open: StoredValue::new(Vec::new()),
        next_handle: StoredValue::new(0),
        group_ref,
        active: RwSignal::new(None),
        drag: StoredValue::new(None),
        keyboard_step,
        on_layout_change,
        on_layout_end,
    };

    // Building the children registers every panel with the context.
    let inner = view! { <Provider value=ctx.clone()>{children()}</Provider> };

    let c = ctx.constraints.get_value();
    let initial = sizes
        .and_then(|s| model::valid_layout(&c, &s.get_untracked()))
        .unwrap_or_else(|| model::initial_sizes(&c, &ctx.defaults.get_value()));
    let n = c.len().max(1) as f64;
    let open = ctx.defaults.with_value(|d| {
        c.iter()
            .enumerate()
            .map(|(i, c)| {
                let v = d.get(i).copied().flatten().unwrap_or(100.0 / n);
                c.clamp(if v > model::EPS { v } else { 100.0 / n })
            })
            .collect::<Vec<_>>()
    });
    ctx.last_open.set_value(open);
    ctx.remember_open(&initial);
    internal.set(initial);

    if let Some(external) = sizes {
        let ctx = ctx.clone();
        Effect::new(move |_| {
            let wanted = external.get();
            let c = ctx.constraints.get_value();
            if let Some(layout) = model::valid_layout(&c, &wanted) {
                ctx.apply(layout);
            }
        });
        Effect::new(move |_| {
            let current = internal.get();
            let same = external.with_untracked(|e| {
                e.len() == current.len()
                    && e.iter()
                        .zip(&current)
                        .all(|(a, b)| (a - b).abs() <= model::EPS)
            });
            if !same {
                external.set(current);
            }
        });
    }

    let active = ctx.active;
    let style_fn = move || {
        let direction = match orientation.get() {
            Orientation::Horizontal => "row",
            Orientation::Vertical => "column",
        };
        format!("display:flex;flex-direction:{direction};{style}")
    };
    view! {
        <div
            node_ref=group_ref
            role="group"
            aria-label=aria_label
            class=class
            style=style_fn
            data-vdl-resizable-group=""
            data-orientation=move || orientation.get().to_string()
            data-dragging=move || active.get().is_some().then_some("")
        >
            {inner}
        </div>
    }
}

/// One panel of a [`ResizablePanelGroup`]. Its `flex-grow` is its current size, so it fills
/// its share of the group.
///
/// `default_size` (percent) is the starting size; panels without one share what is left.
/// `min_size` and `max_size` limit it. A `collapsible` panel that is dragged (or stepped)
/// below `min_size` collapses to `collapsed_size` (0 by default). `collapsed` is an optional
/// two-way signal for that state: set it to collapse or expand the panel. All of these are
/// read once, when the panel is built.
#[component]
pub fn ResizablePanel(
    #[prop(default = None)] default_size: Option<f64>,
    #[prop(default = 0.0)] min_size: f64,
    #[prop(default = 100.0)] max_size: f64,
    #[prop(optional)] collapsible: bool,
    #[prop(optional)] collapsed_size: f64,
    #[prop(default = None)] collapsed: Option<RwSignal<bool>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] style: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<GroupContext>();
    let c = PanelConstraints {
        min: min_size,
        max: max_size,
        collapsible,
        collapsed: collapsed_size,
    };
    let starts_collapsed = collapsible && collapsed.is_some_and(|s| s.get_untracked());
    let mut idx = 0;
    ctx.constraints.update_value(|v| {
        idx = v.len();
        v.push(c);
    });
    ctx.defaults.update_value(|v| {
        v.push(if starts_collapsed {
            Some(collapsed_size)
        } else {
            default_size
        })
    });

    if let Some(signal) = collapsed {
        let sync = ctx.clone();
        Effect::new(move |_| {
            // Follow the layout.
            let is = sync.sizes.with(|s| {
                s.get(idx)
                    .is_some_and(|v| sync.constraints.with_value(|c| c[idx].is_collapsed(*v)))
            });
            if signal.get_untracked() != is {
                signal.set(is);
            }
        });
        let drive = ctx.clone();
        Effect::new(move |_| {
            // Follow the signal.
            let want = signal.get();
            if want != drive.panel_is_collapsed(idx) {
                if want {
                    drive.collapse(idx);
                } else {
                    drive.expand(idx);
                }
            }
        });
    }

    let id = ctx.panel_id(idx);
    let grow = {
        let ctx = ctx.clone();
        move || {
            ctx.sizes
                .with(|s| s.get(idx).copied())
                .or(default_size)
                .unwrap_or(1.0)
        }
    };
    let state = {
        let ctx = ctx.clone();
        move || {
            if ctx.sizes.with(|s| {
                s.get(idx)
                    .is_some_and(|v| ctx.constraints.with_value(|c| c[idx].is_collapsed(*v)))
            }) {
                "collapsed"
            } else {
                "expanded"
            }
        }
    };
    let size_text = {
        let ctx = ctx.clone();
        move || ctx.sizes.with(|s| s.get(idx).map(|v| pct(*v)))
    };
    let orientation = ctx.orientation;
    view! {
        <div
            id=id
            class=class
            style=move || format!("flex:{} 1 0px;min-width:0;min-height:0;{style}", grow())
            data-vdl-resizable-panel=""
            data-orientation=move || orientation.get().to_string()
            data-state=state
            data-size=size_text
        >
            {children.map(|c| c())}
        </div>
    }
}

/// The draggable separator between two panels. It is the focusable control
/// (`role="separator"`, `aria-valuenow` the size of the panel before it, `aria-valuemin` and
/// `aria-valuemax` how far that panel can go): the arrow keys along the group's axis move it
/// (Shift moves ten steps), Home and End send it to either end, Enter collapses or opens a
/// collapsible neighbour. Children draw a grip; it carries no size of its own, so style its
/// thickness with CSS. A handle between panels `n` and `n + 1` is the `n`th handle built.
#[component]
pub fn ResizableHandle(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(default = None)] aria_label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<GroupContext>();
    let mut idx = 0;
    ctx.next_handle.update_value(|n| {
        idx = *n;
        *n += 1;
    });
    let orientation = ctx.orientation;

    let value_now = {
        let ctx = ctx.clone();
        move || ctx.sizes.with(|s| s.get(idx).map(|v| pct(*v)))
    };
    let bounds = {
        let ctx = ctx.clone();
        move || {
            ctx.sizes.with(|s| {
                ctx.constraints
                    .with_value(|c| model::handle_bounds(s, c, idx))
            })
        }
    };
    let controls = {
        let ctx = ctx.clone();
        move || format!("{} {}", ctx.panel_id(idx), ctx.panel_id(idx + 1))
    };

    let on_down = {
        let ctx = ctx.clone();
        move |ev: PointerEvent| {
            if disabled.get_untracked() || ev.button() != 0 {
                return;
            }
            let Some(group) = ctx.group_ref.get_untracked() else {
                return;
            };
            let horizontal = ctx.orientation.get_untracked() == Orientation::Horizontal;
            let total_px = panels_extent(&group, horizontal);
            if total_px <= 0.0 {
                return;
            }
            if let Some(target) = ev
                .current_target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            {
                let _ = target.set_pointer_capture(ev.pointer_id());
                let _ = target.focus();
            }
            ev.prevent_default();
            ctx.drag.set_value(Some(Drag {
                pointer_id: ev.pointer_id(),
                start_px: coord(&ev, horizontal),
                start_sizes: ctx.sizes.get_untracked(),
                total_px,
            }));
            ctx.active.set(Some(idx));
        }
    };
    let on_move = {
        let ctx = ctx.clone();
        move |ev: PointerEvent| {
            let Some(drag) = ctx.drag.get_value() else {
                return;
            };
            if ev.pointer_id() != drag.pointer_id || ctx.active.get_untracked() != Some(idx) {
                return;
            }
            let horizontal = ctx.orientation.get_untracked() == Orientation::Horizontal;
            let delta = (coord(&ev, horizontal) - drag.start_px) / drag.total_px * 100.0;
            let c = ctx.constraints.get_value();
            ctx.apply(model::resize_pair(
                &drag.start_sizes,
                &c,
                idx,
                delta,
                Snap::Midpoint,
            ));
        }
    };
    let on_end = {
        let ctx = ctx.clone();
        move |_: PointerEvent| {
            if ctx.drag.get_value().is_none() {
                return;
            }
            ctx.drag.set_value(None);
            ctx.active.set(None);
            ctx.commit();
        }
    };
    let on_key = {
        let ctx = ctx.clone();
        move |ev: KeyboardEvent| {
            if disabled.get_untracked() {
                return;
            }
            let horizontal = ctx.orientation.get_untracked() == Orientation::Horizontal;
            let Some(action) =
                model::key_action(&ev.key(), ev.shift_key(), horizontal, ctx.keyboard_step)
            else {
                return;
            };
            ev.prevent_default();
            match action {
                model::KeyAction::Move(delta) => {
                    let c = ctx.constraints.get_value();
                    let sizes = ctx.sizes.get_untracked();
                    ctx.apply(model::resize_pair(&sizes, &c, idx, delta, Snap::Eager));
                }
                model::KeyAction::Toggle => ctx.toggle_at_handle(idx),
            }
            ctx.commit();
        }
    };

    view! {
        <div
            role="separator"
            class=class
            tabindex=move || if disabled.get() { "-1" } else { "0" }
            aria-label=aria_label
            aria-disabled=move || disabled.get().then_some("true")
            aria-orientation=move || match orientation.get() {
                Orientation::Horizontal => "vertical",
                Orientation::Vertical => "horizontal",
            }
            aria-controls=controls
            aria-valuenow=value_now
            aria-valuemin=move || pct(bounds().0)
            aria-valuemax=move || pct(bounds().1)
            style="flex:none;touch-action:none;"
            data-vdl-resizable-handle=""
            data-orientation=move || orientation.get().to_string()
            data-state=move || if ctx.active.get() == Some(idx) { "drag" } else { "inactive" }
            data-disabled=move || disabled.get().then_some("")
            on:pointerdown=on_down
            on:pointermove=on_move
            on:pointerup=on_end.clone()
            on:pointercancel=on_end.clone()
            on:lostpointercapture=on_end
            on:keydown=on_key
        >
            {children.map(|c| c())}
        </div>
    }
}

/// A size as text for ARIA and data attributes: one decimal at most.
fn pct(v: f64) -> String {
    ((v * 10.0).round() / 10.0).to_string()
}

fn coord(ev: &PointerEvent, horizontal: bool) -> f64 {
    if horizontal {
        ev.client_x() as f64
    } else {
        ev.client_y() as f64
    }
}

/// The pixels the group's own panels take along its axis (the handles are not counted).
fn panels_extent(group: &web_sys::Element, horizontal: bool) -> f64 {
    let Ok(list) = group.query_selector_all(":scope > [data-vdl-resizable-panel]") else {
        return 0.0;
    };
    (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
        .map(|el| {
            let r = el.get_bounding_client_rect();
            if horizontal { r.width() } else { r.height() }
        })
        .sum()
}
