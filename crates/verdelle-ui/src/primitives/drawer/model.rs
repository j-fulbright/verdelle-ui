//! The pure model: snap points, release decisions, gesture ownership, velocity.

/// Fraction of the lowest snap point that must stay visible for a release to keep the drawer
/// open; dragging past it (25% of the way to closed) dismisses. Same as vaul's close threshold.
pub const CLOSE_THRESHOLD: f64 = 0.25;
/// A release faster than this (px per ms) is a flick: it moves one snap point (or dismisses).
pub const FLICK_VELOCITY: f64 = 0.4;
/// A release faster than this (px per ms) dismisses whatever the snap point.
pub const DISMISS_VELOCITY: f64 = 2.0;
/// How far (px) the finger must move before the gesture is classified.
pub const DRAG_SLOP: f64 = 6.0;
/// Only the last this many milliseconds of movement count towards the release velocity.
pub const VELOCITY_WINDOW_MS: f64 = 100.0;

/// Clean up a list of snap points: each is the visible fraction of the drawer (0 to 1], so 0.5
/// shows half of it. Out-of-range values are clamped, the list is sorted ascending and
/// de-duplicated. An empty list means one snap point, fully open.
pub fn normalize_snaps(raw: &[f64]) -> Vec<f64> {
    let mut snaps: Vec<f64> = raw
        .iter()
        .filter(|s| s.is_finite())
        .map(|s| s.clamp(0.05, 1.0))
        .collect();
    snaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    snaps.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    if snaps.is_empty() {
        snaps.push(1.0);
    }
    snaps
}

/// What a finger release decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Release {
    /// Settle on the snap point with this index (into the normalized list).
    Snap(usize),
    /// Close the drawer.
    Dismiss,
}

/// Decide where a release lands.
///
/// `snaps` is normalized (ascending), `active` the snap point the drag started from, `size` the
/// drawer's full extent in px, `drag` how far it has been pulled toward close (negative: pulled
/// open past the snap point), `velocity` px/ms toward close (negative: toward open).
///
/// - a very fast flick toward close dismisses (if `dismissible`);
/// - a flick moves one snap point that way, or dismisses from the lowest one;
/// - otherwise the nearest snap point wins, except that pulling the panel more than a quarter
///   below the lowest snap point dismisses.
pub fn resolve_release(
    snaps: &[f64],
    active: usize,
    size: f64,
    drag: f64,
    velocity: f64,
    dismissible: bool,
) -> Release {
    let snaps = if snaps.is_empty() { &[1.0][..] } else { snaps };
    let active = active.min(snaps.len() - 1);
    let visible = snaps[active] * size - drag;

    if dismissible && velocity >= DISMISS_VELOCITY {
        return Release::Dismiss;
    }

    if velocity.abs() >= FLICK_VELOCITY {
        if velocity > 0.0 {
            // The highest snap point that is still below where the panel is now.
            let lower = snaps.iter().rposition(|s| s * size < visible - 0.5);
            return match lower {
                Some(i) => Release::Snap(i),
                None if dismissible => Release::Dismiss,
                None => Release::Snap(0),
            };
        }
        let higher = snaps.iter().position(|s| s * size > visible + 0.5);
        return Release::Snap(higher.unwrap_or(snaps.len() - 1));
    }

    if dismissible && visible < snaps[0] * size * (1.0 - CLOSE_THRESHOLD) {
        return Release::Dismiss;
    }
    let nearest = snaps
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let da = (**a * size - visible).abs();
            let db = (**b * size - visible).abs();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0);
    Release::Snap(nearest)
}

/// Resistance past a limit: the more the finger overshoots, the less the panel follows
/// (logarithmic, as vaul's `dampenValue`).
pub fn damp(overshoot: f64) -> f64 {
    8.0 * (overshoot.max(0.0) + 1.0).ln()
}

/// Limit a drag. Pulling open past the highest snap point, or (when the drawer cannot be
/// dismissed) closed past the lowest, meets resistance; a dismissible drawer can be dragged all
/// the way out of view and no further.
pub fn constrain_drag(
    drag: f64,
    size: f64,
    snaps: &[f64],
    active: usize,
    dismissible: bool,
) -> f64 {
    let snaps = if snaps.is_empty() { &[1.0][..] } else { snaps };
    let active = active.min(snaps.len() - 1);
    let highest = snaps[snaps.len() - 1];
    let min_drag = -(highest - snaps[active]) * size;
    let max_drag = if dismissible {
        snaps[active] * size
    } else {
        (snaps[active] - snaps[0]) * size
    };
    if drag < min_drag {
        min_drag - damp(min_drag - drag)
    } else if drag > max_drag {
        if dismissible {
            max_drag
        } else {
            max_drag + damp(drag - max_drag)
        }
    } else {
        drag
    }
}

/// Who owns a gesture that started inside the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    /// The drawer follows the finger.
    Drawer,
    /// The scrollable content inside scrolls; the drawer stays put.
    Content,
}

/// Whether the scroll container under the finger can still scroll in each direction of the
/// drag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollRoom {
    /// It can scroll while the finger moves toward close.
    pub toward_close: bool,
    /// It can scroll while the finger moves toward open.
    pub toward_open: bool,
}

/// Decide whether a drag belongs to the drawer or to scrolling content.
///
/// `delta` is the finger movement so far (positive: toward close), `room` the scroll container
/// under the finger (`None`: there is none), `at_max_snap` whether the drawer is open as far as
/// it goes. Content only scrolls once the drawer is fully open; a drawer at a lower snap point
/// takes every drag (pulling it up expands it), and content scrolled to its edge hands the drag
/// back to the drawer, so a list scrolled to the top can be pulled down to dismiss.
pub fn gesture_owner(delta: f64, room: Option<ScrollRoom>, at_max_snap: bool) -> Owner {
    let Some(room) = room else {
        return Owner::Drawer;
    };
    if !at_max_snap {
        return Owner::Drawer;
    }
    let content_can = if delta > 0.0 {
        room.toward_close
    } else {
        room.toward_open
    };
    if content_can {
        Owner::Content
    } else {
        Owner::Drawer
    }
}

/// Finger velocity over the last [`VELOCITY_WINDOW_MS`].
#[derive(Clone, Debug, Default)]
pub struct VelocityTracker {
    samples: Vec<(f64, f64)>,
}

impl VelocityTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record the finger at `pos` at time `t` (ms).
    pub fn push(&mut self, t: f64, pos: f64) {
        self.samples.push((t, pos));
        let cutoff = t - VELOCITY_WINDOW_MS;
        self.samples.retain(|(st, _)| *st >= cutoff);
    }

    /// Px per ms at time `now`. Zero when the finger rested for a whole window before `now`
    /// (a drag that paused before letting go is not a flick) or there is too little data.
    pub fn velocity(&self, now: f64) -> f64 {
        let (Some(first), Some(last)) = (self.samples.first(), self.samples.last()) else {
            return 0.0;
        };
        if now - last.0 > VELOCITY_WINDOW_MS || last.0 - first.0 < 1.0 {
            return 0.0;
        }
        (last.1 - first.1) / (last.0 - first.0)
    }
}

/// Settings a drag starts from.
#[derive(Clone, Debug)]
pub struct DragConfig {
    /// The drawer's full extent along the drag axis, px.
    pub size: f64,
    /// Normalized snap points.
    pub snaps: Vec<f64>,
    /// Index of the active snap point.
    pub active: usize,
    pub dismissible: bool,
}

/// What a pointer move did to the gesture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DragUpdate {
    /// Still inside the slop: undecided.
    Pending,
    /// Not the drawer's: content scrolls (or the cross axis won). Ignore the rest of the gesture.
    Passed,
    /// The drawer follows the finger; this is its (constrained) offset toward close, px.
    Drag(f64),
}

/// One gesture, from finger down to finger up.
#[derive(Clone, Debug)]
pub struct DragSession {
    cfg: DragConfig,
    room: Option<ScrollRoom>,
    start_main: f64,
    start_cross: f64,
    origin: f64,
    decided: Option<Owner>,
    drag: f64,
    tracker: VelocityTracker,
}

impl DragSession {
    pub fn new(
        cfg: DragConfig,
        room: Option<ScrollRoom>,
        main: f64,
        cross: f64,
        time: f64,
    ) -> Self {
        let mut tracker = VelocityTracker::new();
        tracker.push(time, main);
        Self {
            cfg,
            room,
            start_main: main,
            start_cross: cross,
            origin: main,
            decided: None,
            drag: 0.0,
            tracker,
        }
    }

    fn at_max_snap(&self) -> bool {
        self.cfg.active + 1 >= self.cfg.snaps.len()
    }

    /// Feed a pointer move (`main`: along the drag axis, toward close positive; `cross`: the
    /// other axis).
    pub fn update(&mut self, main: f64, cross: f64, time: f64) -> DragUpdate {
        match self.decided {
            Some(Owner::Content) => return DragUpdate::Passed,
            Some(Owner::Drawer) => {}
            None => {
                let dm = main - self.start_main;
                let dc = cross - self.start_cross;
                if dm.abs() < DRAG_SLOP && dc.abs() < DRAG_SLOP {
                    return DragUpdate::Pending;
                }
                let owner = if dc.abs() > dm.abs() {
                    Owner::Content
                } else {
                    gesture_owner(dm, self.room, self.at_max_snap())
                };
                self.decided = Some(owner);
                if owner == Owner::Content {
                    return DragUpdate::Passed;
                }
                // Start from where the finger is, so the panel does not jump by the slop.
                self.origin = main;
            }
        }
        self.tracker.push(time, main);
        self.drag = constrain_drag(
            main - self.origin,
            self.cfg.size,
            &self.cfg.snaps,
            self.cfg.active,
            self.cfg.dismissible,
        );
        DragUpdate::Drag(self.drag)
    }

    /// The finger lifted at `time`. `None` when the drawer never took the gesture (a tap, or
    /// content scrolled).
    pub fn finish(&self, time: f64) -> Option<Release> {
        if self.decided != Some(Owner::Drawer) {
            return None;
        }
        Some(resolve_release(
            &self.cfg.snaps,
            self.cfg.active,
            self.cfg.size,
            self.drag,
            self.tracker.velocity(time),
            self.cfg.dismissible,
        ))
    }
}

/// The snap point a handle click moves to: the next one up, wrapping to the lowest.
pub fn next_snap(active: usize, count: usize) -> usize {
    if count == 0 { 0 } else { (active + 1) % count }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNAPS: [f64; 3] = [0.25, 0.5, 1.0];

    #[test]
    fn normalizes_snap_points() {
        assert_eq!(normalize_snaps(&[]), vec![1.0]);
        assert_eq!(normalize_snaps(&[1.0, 0.5, 0.5, 2.0]), vec![0.5, 1.0]);
        assert_eq!(normalize_snaps(&[f64::NAN, 0.3]), vec![0.3]);
        assert_eq!(normalize_snaps(&[-1.0]), vec![0.05]);
    }

    #[test]
    fn short_slow_drag_returns_to_the_snap_point() {
        assert_eq!(
            resolve_release(&[1.0], 0, 400.0, 60.0, 0.0, true),
            Release::Snap(0)
        );
    }

    #[test]
    fn long_slow_drag_dismisses() {
        assert_eq!(
            resolve_release(&[1.0], 0, 400.0, 120.0, 0.0, true),
            Release::Dismiss
        );
    }

    #[test]
    fn long_drag_cannot_dismiss_when_not_dismissible() {
        assert_eq!(
            resolve_release(&[1.0], 0, 400.0, 390.0, 0.0, false),
            Release::Snap(0)
        );
    }

    #[test]
    fn quick_flick_dismisses_even_after_a_short_drag() {
        assert_eq!(
            resolve_release(&[1.0], 0, 400.0, 30.0, 0.6, true),
            Release::Dismiss
        );
        assert_eq!(
            resolve_release(&[1.0], 0, 400.0, 30.0, 0.3, true),
            Release::Snap(0)
        );
    }

    #[test]
    fn very_fast_flick_dismisses_from_any_snap_point() {
        assert_eq!(
            resolve_release(&SNAPS, 2, 800.0, 10.0, 2.5, true),
            Release::Dismiss
        );
        assert_eq!(
            resolve_release(&SNAPS, 2, 800.0, 10.0, 2.5, false),
            Release::Snap(1)
        );
    }

    #[test]
    fn flick_moves_one_snap_point() {
        assert_eq!(
            resolve_release(&SNAPS, 2, 800.0, 20.0, 0.8, true),
            Release::Snap(1)
        );
        assert_eq!(
            resolve_release(&SNAPS, 1, 800.0, 20.0, 0.8, true),
            Release::Snap(0)
        );
        assert_eq!(
            resolve_release(&SNAPS, 0, 800.0, 20.0, 0.8, true),
            Release::Dismiss
        );
        assert_eq!(
            resolve_release(&SNAPS, 0, 800.0, 20.0, 0.8, false),
            Release::Snap(0)
        );
        assert_eq!(
            resolve_release(&SNAPS, 0, 800.0, -20.0, -0.8, true),
            Release::Snap(1)
        );
        assert_eq!(
            resolve_release(&SNAPS, 2, 800.0, -20.0, -0.8, true),
            Release::Snap(2)
        );
    }

    #[test]
    fn slow_release_picks_the_nearest_snap_point() {
        // From 0.5 (400px visible) pulled up 300px: 700 visible, nearest is 1.0.
        assert_eq!(
            resolve_release(&SNAPS, 1, 800.0, -300.0, 0.0, true),
            Release::Snap(2)
        );
        // From 1.0 pulled down 100px: 700 visible, still nearest to 1.0.
        assert_eq!(
            resolve_release(&SNAPS, 2, 800.0, 100.0, 0.0, true),
            Release::Snap(2)
        );
        // Pulled down 450px: 350 visible, nearest 0.5.
        assert_eq!(
            resolve_release(&SNAPS, 2, 800.0, 450.0, 0.0, true),
            Release::Snap(1)
        );
    }

    #[test]
    fn dragging_below_the_lowest_snap_point_dismisses() {
        // Lowest is 200px of 800; below 150px visible dismisses.
        assert_eq!(
            resolve_release(&SNAPS, 0, 800.0, 60.0, 0.0, true),
            Release::Dismiss
        );
        assert_eq!(
            resolve_release(&SNAPS, 0, 800.0, 40.0, 0.0, true),
            Release::Snap(0)
        );
    }

    #[test]
    fn out_of_range_active_index_is_clamped() {
        assert_eq!(
            resolve_release(&SNAPS, 9, 800.0, 0.0, 0.0, true),
            Release::Snap(2)
        );
        assert_eq!(
            resolve_release(&[], 0, 400.0, 0.0, 0.0, true),
            Release::Snap(0)
        );
    }

    #[test]
    fn drag_is_resisted_past_the_open_limit() {
        let d = constrain_drag(-100.0, 400.0, &[1.0], 0, true);
        assert!(d < 0.0 && d > -100.0, "{d}");
        assert!((d + damp(100.0)).abs() < 1e-9);
        // At a lower snap point the panel follows freely up to the top one.
        assert_eq!(constrain_drag(-100.0, 400.0, &SNAPS, 0, true), -100.0);
    }

    #[test]
    fn dismissible_drag_stops_at_fully_hidden_and_others_resist() {
        assert_eq!(constrain_drag(900.0, 400.0, &[1.0], 0, true), 400.0);
        let d = constrain_drag(100.0, 400.0, &[1.0], 0, false);
        assert!(d > 0.0 && d < 100.0, "{d}");
        assert_eq!(constrain_drag(200.0, 800.0, &SNAPS, 2, false), 200.0);
    }

    #[test]
    fn damp_grows_slowly() {
        assert_eq!(damp(0.0), 0.0);
        assert!(damp(100.0) < 40.0);
        assert!(damp(10.0) < damp(100.0));
        assert_eq!(damp(-5.0), 0.0);
    }

    #[test]
    fn velocity_uses_only_the_recent_window() {
        let mut v = VelocityTracker::new();
        v.push(0.0, 0.0);
        v.push(500.0, 10.0);
        v.push(540.0, 30.0);
        v.push(580.0, 70.0);
        // Window is 100ms back from 580: samples at 500..580 -> 60px in 80ms.
        assert!((v.velocity(585.0) - 0.75).abs() < 1e-9);
        // Held still for longer than the window: not a flick.
        assert_eq!(v.velocity(800.0), 0.0);
    }

    #[test]
    fn velocity_needs_two_samples() {
        let mut v = VelocityTracker::new();
        assert_eq!(v.velocity(0.0), 0.0);
        v.push(10.0, 5.0);
        assert_eq!(v.velocity(12.0), 0.0);
    }

    #[test]
    fn velocity_is_negative_toward_open() {
        let mut v = VelocityTracker::new();
        v.push(0.0, 100.0);
        v.push(50.0, 50.0);
        assert!((v.velocity(60.0) + 1.0).abs() < 1e-9);
    }

    const ROOM: ScrollRoom = ScrollRoom {
        toward_close: false,
        toward_open: true,
    };

    #[test]
    fn content_at_its_edge_hands_the_drag_to_the_drawer() {
        assert_eq!(gesture_owner(10.0, Some(ROOM), true), Owner::Drawer);
        assert_eq!(gesture_owner(-10.0, Some(ROOM), true), Owner::Content);
    }

    #[test]
    fn scrolled_content_keeps_the_gesture() {
        let mid = ScrollRoom {
            toward_close: true,
            toward_open: true,
        };
        assert_eq!(gesture_owner(10.0, Some(mid), true), Owner::Content);
    }

    #[test]
    fn lower_snap_points_take_every_drag() {
        let mid = ScrollRoom {
            toward_close: true,
            toward_open: true,
        };
        assert_eq!(gesture_owner(-10.0, Some(mid), false), Owner::Drawer);
        assert_eq!(gesture_owner(10.0, None, true), Owner::Drawer);
    }

    fn session(room: Option<ScrollRoom>, snaps: &[f64], active: usize) -> DragSession {
        DragSession::new(
            DragConfig {
                size: 400.0,
                snaps: snaps.to_vec(),
                active,
                dismissible: true,
            },
            room,
            100.0,
            50.0,
            0.0,
        )
    }

    #[test]
    fn small_moves_stay_pending_and_a_tap_never_finishes() {
        let mut s = session(None, &[1.0], 0);
        assert_eq!(s.update(103.0, 51.0, 10.0), DragUpdate::Pending);
        assert_eq!(s.finish(20.0), None);
    }

    #[test]
    fn drag_starts_from_where_it_was_decided() {
        let mut s = session(None, &[1.0], 0);
        assert_eq!(s.update(108.0, 50.0, 10.0), DragUpdate::Drag(0.0));
        assert_eq!(s.update(158.0, 50.0, 20.0), DragUpdate::Drag(50.0));
    }

    #[test]
    fn cross_axis_movement_is_passed_on() {
        let mut s = session(None, &[1.0], 0);
        assert_eq!(s.update(102.0, 80.0, 10.0), DragUpdate::Passed);
        assert_eq!(s.update(300.0, 80.0, 20.0), DragUpdate::Passed);
        assert_eq!(s.finish(30.0), None);
    }

    #[test]
    fn scrolling_content_keeps_its_gesture_for_good() {
        let mid = ScrollRoom {
            toward_close: true,
            toward_open: true,
        };
        let mut s = session(Some(mid), &[1.0], 0);
        assert_eq!(s.update(120.0, 50.0, 10.0), DragUpdate::Passed);
        assert_eq!(s.finish(20.0), None);
    }

    #[test]
    fn a_fast_pull_ends_in_a_dismiss() {
        let mut s = session(None, &[1.0], 0);
        s.update(110.0, 50.0, 10.0);
        s.update(140.0, 50.0, 30.0);
        s.update(170.0, 50.0, 50.0);
        // 60px in 40ms = 1.5px/ms, a flick.
        assert_eq!(s.finish(55.0), Some(Release::Dismiss));
    }

    #[test]
    fn a_slow_short_pull_springs_back() {
        let mut s = session(None, &[1.0], 0);
        s.update(110.0, 50.0, 100.0);
        s.update(140.0, 50.0, 600.0);
        s.update(160.0, 50.0, 1200.0);
        assert_eq!(s.finish(1250.0), Some(Release::Snap(0)));
    }

    #[test]
    fn a_pull_up_from_a_low_snap_point_expands() {
        let mut s = session(None, &SNAPS, 0);
        s.update(95.0, 50.0, 10.0);
        s.update(40.0, 50.0, 30.0);
        s.update(-20.0, 50.0, 50.0);
        assert_eq!(s.finish(55.0), Some(Release::Snap(1)));
    }

    #[test]
    fn next_snap_wraps() {
        assert_eq!(next_snap(0, 3), 1);
        assert_eq!(next_snap(2, 3), 0);
        assert_eq!(next_snap(0, 0), 0);
    }
}
