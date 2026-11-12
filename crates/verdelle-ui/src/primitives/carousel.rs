//! The headless model behind a carousel built on CSS scroll-snap: which scroll positions the
//! slides snap to, which snap is current, where a step, a key press or the end of a drag leads,
//! and whether the ends wrap. It is plain functions over numbers, so it is testable natively;
//! the few DOM helpers at the bottom only read geometry and call `scrollTo`.
//! `verdelle_ui::Carousel` is the styled component built on it.
//!
//! This is original Verdelle code (not a Leptix port). shadcn's Carousel wraps Embla, a JavaScript
//! engine; here the browser does the scrolling and snapping and this model only decides indices.
//!
//! An *index* always means a snap, not a slide: with three slides in view and five slides there
//! are three snaps (the last slide offsets are clamped to the end of the track and merge).

use crate::primitives::separator::Orientation;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    HtmlElement, MutationObserver, MutationObserverInit, ResizeObserver, ScrollBehavior,
    ScrollToOptions,
};

/// Which way a step goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Previous,
    Next,
}

/// A request from the keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyAction {
    Step(Direction),
    First,
    Last,
}

/// Snaps closer together than this (in pixels) are one snap. Absorbs sub-pixel layout rounding.
const SNAP_TOLERANCE: f64 = 1.0;

/// The scroll positions to snap to, given each slide's start offset (from the start of the
/// scrolled content), the viewport's size and the content's total size along the scroll axis.
///
/// Each offset is clamped to the scrollable range (`0..=content - viewport`) and neighbours that
/// end up at the same place are merged, so the last snap is always the end of the track.
pub fn compute_snaps(offsets: &[f64], viewport: f64, content: f64) -> Vec<f64> {
    let max = (content - viewport).max(0.0);
    let mut snaps: Vec<f64> = Vec::with_capacity(offsets.len());
    for offset in offsets {
        let snap = offset.clamp(0.0, max);
        match snaps.last() {
            Some(last) if (snap - last).abs() < SNAP_TOLERANCE => {}
            _ => snaps.push(snap),
        }
    }
    snaps
}

/// How many snaps to expect before the DOM is measured (for example during server rendering):
/// every slide except the last `per_view - 1` can be a snap.
pub fn snap_count_estimate(slides: usize, per_view: f64) -> usize {
    if slides == 0 {
        return 0;
    }
    let visible = per_view.max(1.0).floor() as usize;
    slides.saturating_sub(visible - 1).max(1)
}

/// The index of the snap nearest to a scroll position. `None` when there are no snaps.
pub fn nearest_snap(snaps: &[f64], position: f64) -> Option<usize> {
    snaps
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (*a - position)
                .abs()
                .partial_cmp(&(*b - position).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
}

/// Whether a step in `direction` leads anywhere from `index`. With `wrap` (a looping carousel)
/// it always does, as long as there is more than one snap.
pub fn can_step(index: usize, count: usize, wrap: bool, direction: Direction) -> bool {
    if count < 2 {
        return false;
    }
    if wrap {
        return true;
    }
    match direction {
        Direction::Previous => index > 0,
        Direction::Next => index + 1 < count,
    }
}

/// The index a step of `step` snaps in `direction` leads to. It stops at the ends, except that
/// with `wrap` a step from an end goes round to the other one (a step that would overshoot an end
/// lands on the end first). `step` below 1 counts as 1.
pub fn step_index(
    index: usize,
    count: usize,
    step: usize,
    wrap: bool,
    direction: Direction,
) -> usize {
    if count == 0 {
        return 0;
    }
    let last = count - 1;
    let index = index.min(last);
    let step = step.max(1);
    match direction {
        Direction::Next if index == last => {
            if wrap {
                0
            } else {
                last
            }
        }
        Direction::Next => (index + step).min(last),
        Direction::Previous if index == 0 => {
            if wrap {
                last
            } else {
                0
            }
        }
        Direction::Previous => index.saturating_sub(step),
    }
}

/// Which action a key means, if it is a carousel key for this orientation. Arrow keys along
/// the scroll axis step, `Home` and `End` jump to the ends.
pub fn key_action(key: &str, orientation: Orientation) -> Option<KeyAction> {
    match (key, orientation) {
        ("ArrowLeft", Orientation::Horizontal) | ("ArrowUp", Orientation::Vertical) => {
            Some(KeyAction::Step(Direction::Previous))
        }
        ("ArrowRight", Orientation::Horizontal) | ("ArrowDown", Orientation::Vertical) => {
            Some(KeyAction::Step(Direction::Next))
        }
        ("Home", _) => Some(KeyAction::First),
        ("End", _) => Some(KeyAction::Last),
        _ => None,
    }
}

/// Where a drag that ended at scroll position `position` should settle. It is the nearest snap,
/// except that a drag of at least `threshold` pixels that would settle back where it started goes
/// on to the neighbour in the dragged direction (a short flick turns the page). `delta` is how
/// far the pointer moved: negative (towards the start) means the content moved forward.
pub fn drag_target(
    snaps: &[f64],
    start_index: usize,
    position: f64,
    delta: f64,
    threshold: f64,
) -> Option<usize> {
    let nearest = nearest_snap(snaps, position)?;
    if nearest == start_index && delta.abs() >= threshold {
        let towards = if delta < 0.0 {
            (start_index + 1).min(snaps.len() - 1)
        } else {
            start_index.saturating_sub(1)
        };
        return Some(towards);
    }
    Some(nearest)
}

// ---------------------------------------------------------------- DOM helpers

/// The attribute the styled component puts on each slide.
pub const ITEM_ATTRIBUTE: &str = "data-vdl-carousel-item";

/// The current scroll position of the viewport along its axis.
pub fn scroll_position(viewport: &HtmlElement, vertical: bool) -> f64 {
    if vertical {
        viewport.scroll_top() as f64
    } else {
        viewport.scroll_left() as f64
    }
}

/// The slide's `scroll-margin` at the start of the scroll axis, in pixels.
fn scroll_margin(el: &web_sys::Element, vertical: bool) -> f64 {
    let property = if vertical {
        "scroll-margin-top"
    } else {
        "scroll-margin-left"
    };
    web_sys::window()
        .and_then(|w| w.get_computed_style(el).ok().flatten())
        .and_then(|style| style.get_property_value(property).ok())
        .and_then(|v| v.trim().trim_end_matches("px").parse::<f64>().ok())
        .unwrap_or(0.0)
}

/// The viewport's snap positions, read from the slides (`ITEM_ATTRIBUTE`) inside it.
pub fn measure_snaps(viewport: &HtmlElement, vertical: bool) -> Vec<f64> {
    let rect = viewport.get_bounding_client_rect();
    let origin = if vertical { rect.top() } else { rect.left() };
    let scrolled = scroll_position(viewport, vertical);
    let mut offsets = Vec::new();
    if let Ok(items) = viewport.query_selector_all(&format!("[{ITEM_ATTRIBUTE}]")) {
        for i in 0..items.length() {
            if let Some(el) = items
                .item(i)
                .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
            {
                let r = el.get_bounding_client_rect();
                let start = if vertical { r.top() } else { r.left() };
                // The browser snaps the slide's snap area (its box plus `scroll-margin`) to the
                // viewport's start, so the margin moves where this slide snaps.
                offsets.push(start - origin + scrolled - scroll_margin(&el, vertical));
            }
        }
    }
    let (view, content) = if vertical {
        (viewport.client_height(), viewport.scroll_height())
    } else {
        (viewport.client_width(), viewport.scroll_width())
    };
    compute_snaps(&offsets, view as f64, content as f64)
}

/// Scroll the viewport to `position` along its axis, smoothly or at once.
pub fn scroll_to(viewport: &HtmlElement, vertical: bool, position: f64, smooth: bool) {
    let options = ScrollToOptions::new();
    if vertical {
        options.set_top(position);
    } else {
        options.set_left(position);
    }
    options.set_behavior(if smooth {
        ScrollBehavior::Smooth
    } else {
        ScrollBehavior::Instant
    });
    viewport.scroll_to_with_scroll_to_options(&options);
}

/// Whether motion should be skipped: the user asked for less (`prefers-reduced-motion`) or the
/// page switched the kit's animation off (`data-vdl-motion="off"` on `<html>`).
pub fn motion_reduced() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let media = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches());
    let off = window
        .document()
        .and_then(|d| d.document_element())
        .and_then(|e| e.get_attribute("data-vdl-motion"))
        .is_some_and(|v| v == "off");
    media || off
}

/// Calls back when the viewport or its slides change size, or slides are added or removed, so
/// the snaps can be measured again. Disconnect it when the carousel unmounts.
pub struct LayoutWatcher {
    resize: ResizeObserver,
    mutation: MutationObserver,
    _callback: Closure<dyn Fn()>,
}

impl LayoutWatcher {
    pub fn new(viewport: &HtmlElement, on_change: impl Fn() + 'static) -> Option<Self> {
        let callback: Closure<dyn Fn()> = Closure::new(on_change);
        let resize = ResizeObserver::new(callback.as_ref().unchecked_ref()).ok()?;
        let mutation = MutationObserver::new(callback.as_ref().unchecked_ref()).ok()?;
        resize.observe(viewport);
        let options = MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        let _ = mutation.observe_with_options(viewport, &options);
        let watcher = Self {
            resize,
            mutation,
            _callback: callback,
        };
        watcher.observe_items(viewport);
        Some(watcher)
    }

    /// Watch every slide's size too (call again after slides were added).
    pub fn observe_items(&self, viewport: &HtmlElement) {
        if let Ok(items) = viewport.query_selector_all(&format!("[{ITEM_ATTRIBUTE}]")) {
            for i in 0..items.length() {
                if let Some(el) = items
                    .item(i)
                    .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
                {
                    self.resize.observe(&el);
                }
            }
        }
    }

    pub fn disconnect(&self) {
        self.resize.disconnect();
        self.mutation.disconnect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::{Next, Previous};

    #[test]
    fn snaps_merge_at_the_end_of_the_track() {
        // Five 100px slides in a 300px viewport: slides 3 and 4 start past the end (200).
        let offsets = [0.0, 100.0, 200.0, 300.0, 400.0];
        assert_eq!(
            compute_snaps(&offsets, 300.0, 500.0),
            vec![0.0, 100.0, 200.0]
        );
    }

    #[test]
    fn one_slide_per_view_has_a_snap_per_slide() {
        let offsets = [0.0, 300.0, 600.0];
        assert_eq!(
            compute_snaps(&offsets, 300.0, 900.0),
            vec![0.0, 300.0, 600.0]
        );
    }

    #[test]
    fn content_that_fits_has_a_single_snap() {
        assert_eq!(compute_snaps(&[0.0, 50.0], 300.0, 100.0), vec![0.0]);
        assert!(compute_snaps(&[], 300.0, 0.0).is_empty());
    }

    #[test]
    fn negative_offsets_from_a_leading_gap_clamp_to_zero() {
        // A track pulled left by its gap: the first slide box starts at -16.
        let offsets = [-16.0, 284.0, 584.0];
        assert_eq!(
            compute_snaps(&offsets, 300.0, 916.0),
            vec![0.0, 284.0, 584.0]
        );
    }

    #[test]
    fn rounding_noise_does_not_make_a_snap() {
        assert_eq!(
            compute_snaps(&[0.0, 200.0, 200.4], 100.0, 300.0),
            vec![0.0, 200.0]
        );
    }

    #[test]
    fn estimate_follows_slides_per_view() {
        assert_eq!(snap_count_estimate(5, 1.0), 5);
        assert_eq!(snap_count_estimate(5, 2.0), 4);
        assert_eq!(snap_count_estimate(5, 2.5), 4);
        assert_eq!(snap_count_estimate(5, 3.0), 3);
        assert_eq!(snap_count_estimate(2, 3.0), 1);
        assert_eq!(snap_count_estimate(0, 1.0), 0);
        assert_eq!(snap_count_estimate(4, 0.2), 4);
    }

    #[test]
    fn estimate_matches_the_measured_snaps() {
        // The same five slides, three per view, measured: three snaps, as estimated.
        let measured = compute_snaps(&[0.0, 100.0, 200.0, 300.0, 400.0], 300.0, 500.0);
        assert_eq!(measured.len(), snap_count_estimate(5, 3.0));
    }

    #[test]
    fn nearest_picks_the_closest_snap() {
        let snaps = [0.0, 300.0, 600.0];
        assert_eq!(nearest_snap(&snaps, 10.0), Some(0));
        assert_eq!(nearest_snap(&snaps, 160.0), Some(1));
        assert_eq!(nearest_snap(&snaps, 149.0), Some(0));
        assert_eq!(nearest_snap(&snaps, 9000.0), Some(2));
        assert_eq!(nearest_snap(&[], 5.0), None);
    }

    #[test]
    fn stepping_stops_at_the_ends_without_wrap() {
        assert_eq!(step_index(0, 4, 1, false, Previous), 0);
        assert_eq!(step_index(3, 4, 1, false, Next), 3);
        assert_eq!(step_index(1, 4, 1, false, Next), 2);
        assert_eq!(step_index(1, 4, 1, false, Previous), 0);
        assert_eq!(step_index(0, 0, 1, false, Next), 0);
    }

    #[test]
    fn stepping_wraps_from_an_end() {
        assert_eq!(step_index(3, 4, 1, true, Next), 0);
        assert_eq!(step_index(0, 4, 1, true, Previous), 3);
        assert_eq!(step_index(2, 4, 1, true, Next), 3);
    }

    #[test]
    fn a_larger_step_lands_on_the_end_before_it_wraps() {
        assert_eq!(step_index(0, 5, 2, false, Next), 2);
        assert_eq!(step_index(3, 5, 2, false, Next), 4);
        assert_eq!(step_index(4, 5, 2, true, Next), 0);
        assert_eq!(step_index(1, 5, 2, true, Previous), 0);
        assert_eq!(step_index(0, 5, 2, true, Previous), 4);
        assert_eq!(step_index(2, 5, 0, false, Next), 3);
    }

    #[test]
    fn an_out_of_range_index_is_clamped() {
        assert_eq!(step_index(9, 4, 1, false, Previous), 2);
    }

    #[test]
    fn can_step_matches_the_ends() {
        assert!(!can_step(0, 4, false, Previous));
        assert!(can_step(0, 4, false, Next));
        assert!(can_step(3, 4, false, Previous));
        assert!(!can_step(3, 4, false, Next));
        assert!(can_step(0, 4, true, Previous));
        assert!(can_step(3, 4, true, Next));
        assert!(!can_step(0, 1, true, Next));
        assert!(!can_step(0, 0, false, Next));
    }

    #[test]
    fn keys_follow_the_orientation() {
        use KeyAction::*;
        let h = Orientation::Horizontal;
        let v = Orientation::Vertical;
        assert_eq!(key_action("ArrowLeft", h), Some(Step(Previous)));
        assert_eq!(key_action("ArrowRight", h), Some(Step(Next)));
        assert_eq!(key_action("ArrowDown", h), None);
        assert_eq!(key_action("ArrowUp", v), Some(Step(Previous)));
        assert_eq!(key_action("ArrowDown", v), Some(Step(Next)));
        assert_eq!(key_action("ArrowLeft", v), None);
        assert_eq!(key_action("Home", v), Some(First));
        assert_eq!(key_action("End", h), Some(Last));
        assert_eq!(key_action("a", h), None);
    }

    #[test]
    fn a_drag_settles_on_the_nearest_snap_or_turns_the_page() {
        let snaps = [0.0, 300.0, 600.0];
        // Dragged far enough to be nearer the next snap.
        assert_eq!(drag_target(&snaps, 0, 200.0, -200.0, 40.0), Some(1));
        // A short drag that would snap back still turns the page when past the threshold.
        assert_eq!(drag_target(&snaps, 1, 320.0, -50.0, 40.0), Some(2));
        assert_eq!(drag_target(&snaps, 1, 280.0, 50.0, 40.0), Some(0));
        // Below the threshold it settles back.
        assert_eq!(drag_target(&snaps, 1, 310.0, -10.0, 40.0), Some(1));
        // At the ends a flick outwards stays put.
        assert_eq!(drag_target(&snaps, 2, 590.0, -60.0, 40.0), Some(2));
        assert_eq!(drag_target(&snaps, 0, 0.0, 60.0, 40.0), Some(0));
        assert_eq!(drag_target(&[], 0, 0.0, 60.0, 40.0), None);
    }
}
