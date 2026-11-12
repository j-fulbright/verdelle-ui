//! The pure model of a resizable panel group. Sizes are percentages of the space the panels
//! share (the handles' own width is not part of it) and always add up to 100.
//!
//! Every operation takes the sizes you start from and returns new ones; nothing is stored.
//! A drag keeps the sizes from when it began and passes the total movement so far, so a
//! panel that snapped shut part-way can come back when the pointer returns.

/// Rounding tolerance: sizes closer than this are the same size.
pub const EPS: f64 = 1e-6;

/// The limits of one panel, all in percent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelConstraints {
    pub min: f64,
    pub max: f64,
    /// Whether the panel may shrink to `collapsed` (instead of stopping at `min`).
    pub collapsible: bool,
    /// The size of a collapsed panel (0 hides it; a larger value leaves a strip).
    pub collapsed: f64,
}

impl Default for PanelConstraints {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            collapsible: false,
            collapsed: 0.0,
        }
    }
}

impl PanelConstraints {
    /// Is a panel of `size` collapsed?
    pub fn is_collapsed(&self, size: f64) -> bool {
        self.collapsible && size <= self.collapsed + EPS
    }

    /// `size` kept within `min` and `max`.
    pub fn clamp(&self, size: f64) -> f64 {
        size.clamp(self.min, self.max.max(self.min))
    }
}

/// How a panel that is pushed below its minimum decides to collapse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Snap {
    /// Collapse once past halfway between the minimum and the collapsed size (dragging).
    Midpoint,
    /// Collapse as soon as the minimum is passed (a key press, which moves in small steps).
    Eager,
}

impl Snap {
    /// Whether to collapse: `towards` is true when the move heads for the collapsed end,
    /// `past_middle` when it went past halfway there.
    fn collapses(self, towards: bool, past_middle: bool) -> bool {
        match self {
            Snap::Eager => towards,
            Snap::Midpoint => past_middle,
        }
    }
}

fn round(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

fn rounded(sizes: Vec<f64>) -> Vec<f64> {
    sizes.into_iter().map(round).collect()
}

/// Add `diff` (positive grows, negative shrinks) to the panels in `order`, first to last,
/// each within its limits. Collapsed panels stay as they are. Returns what is left over.
fn distribute(
    sizes: &mut [f64],
    c: &[PanelConstraints],
    order: impl IntoIterator<Item = usize>,
    mut diff: f64,
) -> f64 {
    for i in order {
        if diff.abs() <= EPS {
            break;
        }
        if c[i].is_collapsed(sizes[i]) {
            continue;
        }
        let target = c[i].clamp(sizes[i] + diff).max(0.0);
        // A panel already outside its limits is not pushed further out.
        let target = if diff > 0.0 {
            target.max(sizes[i])
        } else {
            target.min(sizes[i])
        };
        diff -= target - sizes[i];
        sizes[i] = target;
    }
    diff
}

/// Starting sizes for panels with optional default sizes: panels without one share what is
/// left of 100 equally, then everything is clamped and brought back to a total of 100.
pub fn initial_sizes(c: &[PanelConstraints], defaults: &[Option<f64>]) -> Vec<f64> {
    let n = c.len();
    if n == 0 {
        return Vec::new();
    }
    let default_of = |i: usize| defaults.get(i).copied().flatten();
    let given: f64 = (0..n).filter_map(default_of).sum();
    let unset = (0..n).filter(|&i| default_of(i).is_none()).count();
    let mut sizes: Vec<f64> = (0..n)
        .map(|i| match default_of(i) {
            Some(v) => v.max(0.0),
            None => (100.0 - given).max(0.0) / unset as f64,
        })
        .collect();
    // Everything given and not adding up: scale to fit.
    let total: f64 = sizes.iter().sum();
    if total > EPS && (total - 100.0).abs() > EPS {
        for s in &mut sizes {
            *s = *s / total * 100.0;
        }
    }
    for (s, c) in sizes.iter_mut().zip(c) {
        if c.is_collapsed(*s) {
            *s = c.collapsed;
        } else {
            *s = c.clamp(*s);
        }
    }
    let diff = 100.0 - sizes.iter().sum::<f64>();
    // Grow or shrink the last panels first, as the panels at the start are usually fixed.
    distribute(&mut sizes, c, (0..n).rev(), diff);
    rounded(sizes)
}

/// The sizes a group should use given a candidate list (a saved layout, a controlled value):
/// the list itself if it fits the panels and adds up to 100, else `None`.
pub fn valid_layout(c: &[PanelConstraints], sizes: &[f64]) -> Option<Vec<f64>> {
    if sizes.len() != c.len() || sizes.iter().any(|s| !s.is_finite() || *s < 0.0) {
        return None;
    }
    ((sizes.iter().sum::<f64>() - 100.0).abs() < 0.01).then(|| sizes.to_vec())
}

/// The range panel `handle` (the one before the handle) can take, as `(low, high)`, including
/// the collapsed size when collapsing is possible at that end. Used for `aria-valuemin` and
/// `aria-valuemax`.
pub fn handle_bounds(sizes: &[f64], c: &[PanelConstraints], handle: usize) -> (f64, f64) {
    let (a, b) = (handle, handle + 1);
    if b >= sizes.len() || c.len() != sizes.len() {
        return (0.0, 100.0);
    }
    let total = sizes[a] + sizes[b];
    let lo = c[a].min.max(total - c[b].max);
    let hi = c[a].max.min(total - c[b].min);
    let mut low = lo.min(hi);
    let mut high = hi.max(lo);
    if c[a].collapsible && (c[b].min..=c[b].max).contains(&(total - c[a].collapsed)) {
        low = low.min(c[a].collapsed);
    }
    if c[b].collapsible && (c[a].min..=c[a].max).contains(&(total - c[b].collapsed)) {
        high = high.max(total - c[b].collapsed);
    }
    (round(low), round(high))
}

/// Move the handle between panel `handle` and `handle + 1` by `delta` percent (positive
/// grows the first panel). The two panels trade size; the others stay. The result respects
/// both panels' limits, and a collapsible panel pushed past its minimum collapses.
pub fn resize_pair(
    start: &[f64],
    c: &[PanelConstraints],
    handle: usize,
    delta: f64,
    snap: Snap,
) -> Vec<f64> {
    let (a, b) = (handle, handle + 1);
    if b >= start.len() || c.len() != start.len() {
        return start.to_vec();
    }
    let total = start[a] + start[b];
    let lo = c[a].min.max(total - c[b].max);
    let hi = c[a].max.min(total - c[b].min);
    let wanted = start[a] + delta;

    // The sizes the first panel may take apart from the continuous range: collapsed itself,
    // or with the second panel collapsed.
    let a_collapsed = (c[a].collapsible
        && (c[b].min..=c[b].max).contains(&(total - c[a].collapsed)))
    .then_some(c[a].collapsed);
    let b_collapsed = (c[b].collapsible
        && (c[a].min..=c[a].max).contains(&(total - c[b].collapsed)))
    .then_some(total - c[b].collapsed);

    let first = if lo > hi + EPS {
        // The limits cannot be met together: leave things alone.
        start[a]
    } else if wanted < lo - EPS {
        match a_collapsed {
            Some(col) if snap.collapses(delta < 0.0, wanted < (col + lo) / 2.0) => col,
            _ => lo,
        }
    } else if wanted > hi + EPS {
        match b_collapsed {
            Some(col) if snap.collapses(delta > 0.0, wanted > (col + hi) / 2.0) => col,
            _ => hi,
        }
    } else {
        wanted
    };
    let mut out = start.to_vec();
    out[a] = first;
    out[b] = total - first;
    rounded(out)
}

/// The panels nearest first: those after `idx`, then those before it going backwards.
fn neighbours(n: usize, idx: usize) -> Vec<usize> {
    (idx + 1..n).chain((0..idx).rev()).collect()
}

/// Collapse panel `idx` (which must be collapsible), handing its space to the panels around
/// it. Unchanged if the others cannot take it all.
pub fn collapse_panel(sizes: &[f64], c: &[PanelConstraints], idx: usize) -> Vec<f64> {
    if idx >= sizes.len() || !c[idx].collapsible || c[idx].is_collapsed(sizes[idx]) {
        return sizes.to_vec();
    }
    let mut out = sizes.to_vec();
    let freed = out[idx] - c[idx].collapsed;
    out[idx] = c[idx].collapsed;
    let left = distribute(&mut out, c, neighbours(sizes.len(), idx), freed);
    if left > EPS {
        return sizes.to_vec();
    }
    rounded(out)
}

/// Expand a collapsed panel `idx` to `size` (kept within its limits), taking the space from
/// the panels around it. If they cannot give all of it the panel gets what they can give,
/// as long as that reaches its minimum; otherwise nothing changes.
pub fn expand_panel(sizes: &[f64], c: &[PanelConstraints], idx: usize, size: f64) -> Vec<f64> {
    if idx >= sizes.len() || !c[idx].is_collapsed(sizes[idx]) {
        return sizes.to_vec();
    }
    let want = c[idx].clamp(size).max(c[idx].min);
    let mut out = sizes.to_vec();
    let need = want - out[idx];
    // `left` is negative by what the others could not give.
    let left = distribute(&mut out, c, neighbours(sizes.len(), idx), -need);
    let got = need + left;
    if out[idx] + got < c[idx].min - EPS {
        return sizes.to_vec();
    }
    out[idx] += got;
    rounded(out)
}

/// What a key press on a handle asks for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyAction {
    /// Move the handle by this many percent (negative is towards the start).
    Move(f64),
    /// Collapse or open a collapsible neighbour.
    Toggle,
}

/// The effect of `key` on a handle of a group laid out `horizontal`ly (or in a column).
/// Only the arrows along the group's axis count; Shift multiplies `step` by ten; Home and
/// End move all the way.
pub fn key_action(key: &str, shift: bool, horizontal: bool, step: f64) -> Option<KeyAction> {
    let step = if shift { step * 10.0 } else { step };
    let (back, forward) = if horizontal {
        ("ArrowLeft", "ArrowRight")
    } else {
        ("ArrowUp", "ArrowDown")
    };
    match key {
        k if k == back => Some(KeyAction::Move(-step)),
        k if k == forward => Some(KeyAction::Move(step)),
        "Home" => Some(KeyAction::Move(-100.0)),
        "End" => Some(KeyAction::Move(100.0)),
        "Enter" => Some(KeyAction::Toggle),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_along_the_axis_move_the_handle() {
        assert_eq!(
            key_action("ArrowRight", false, true, 2.0),
            Some(KeyAction::Move(2.0))
        );
        assert_eq!(
            key_action("ArrowLeft", false, true, 2.0),
            Some(KeyAction::Move(-2.0))
        );
        assert_eq!(
            key_action("ArrowDown", false, false, 2.0),
            Some(KeyAction::Move(2.0))
        );
        assert_eq!(
            key_action("ArrowUp", false, false, 2.0),
            Some(KeyAction::Move(-2.0))
        );
        assert_eq!(
            key_action("ArrowRight", true, true, 2.0),
            Some(KeyAction::Move(20.0))
        );
    }

    #[test]
    fn arrows_across_the_axis_are_ignored() {
        assert_eq!(key_action("ArrowUp", false, true, 1.0), None);
        assert_eq!(key_action("ArrowLeft", false, false, 1.0), None);
        assert_eq!(key_action("a", false, true, 1.0), None);
    }

    #[test]
    fn home_end_and_enter() {
        assert_eq!(
            key_action("Home", false, true, 1.0),
            Some(KeyAction::Move(-100.0))
        );
        assert_eq!(
            key_action("End", false, false, 1.0),
            Some(KeyAction::Move(100.0))
        );
        assert_eq!(
            key_action("Enter", false, true, 1.0),
            Some(KeyAction::Toggle)
        );
    }

    #[test]
    fn home_and_end_stop_at_the_limits_or_collapse() {
        let c = [limits(15.0, 80.0), open()];
        assert_eq!(
            resize_pair(&[40.0, 60.0], &c, 0, -100.0, Snap::Eager),
            vec![15.0, 85.0]
        );
        assert_eq!(
            resize_pair(&[40.0, 60.0], &c, 0, 100.0, Snap::Eager),
            vec![80.0, 20.0]
        );
        let c = [collapsible(15.0, 0.0), open()];
        assert_eq!(
            resize_pair(&[40.0, 60.0], &c, 0, -100.0, Snap::Eager),
            vec![0.0, 100.0]
        );
    }

    fn free(n: usize) -> Vec<PanelConstraints> {
        vec![PanelConstraints::default(); n]
    }

    fn open() -> PanelConstraints {
        PanelConstraints::default()
    }

    fn limits(min: f64, max: f64) -> PanelConstraints {
        PanelConstraints {
            min,
            max,
            ..Default::default()
        }
    }

    fn collapsible(min: f64, collapsed: f64) -> PanelConstraints {
        PanelConstraints {
            min,
            collapsible: true,
            collapsed,
            ..Default::default()
        }
    }

    #[test]
    fn unset_panels_share_the_rest_equally() {
        assert_eq!(
            initial_sizes(&free(3), &[Some(40.0), None, None]),
            vec![40.0, 30.0, 30.0]
        );
        assert_eq!(initial_sizes(&free(4), &[]), vec![25.0; 4]);
    }

    #[test]
    fn given_sizes_that_do_not_add_up_are_scaled() {
        assert_eq!(
            initial_sizes(&free(2), &[Some(20.0), Some(20.0)]),
            vec![50.0, 50.0]
        );
    }

    #[test]
    fn initial_sizes_respect_the_limits_and_still_total_100() {
        let c = [limits(0.0, 30.0), limits(0.0, 100.0)];
        let s = initial_sizes(&c, &[Some(60.0), None]);
        assert_eq!(s, vec![30.0, 70.0]);
        let c = [limits(20.0, 100.0), limits(20.0, 100.0)];
        let s = initial_sizes(&c, &[Some(5.0), Some(95.0)]);
        assert_eq!(s, vec![20.0, 80.0]);
    }

    #[test]
    fn a_collapsed_default_starts_collapsed() {
        let c = [collapsible(15.0, 0.0), open()];
        assert_eq!(initial_sizes(&c, &[Some(0.0), None]), vec![0.0, 100.0]);
    }

    #[test]
    fn valid_layout_needs_the_right_count_and_a_total_of_100() {
        let c = free(2);
        assert_eq!(valid_layout(&c, &[30.0, 70.0]), Some(vec![30.0, 70.0]));
        assert_eq!(valid_layout(&c, &[30.0, 60.0]), None);
        assert_eq!(valid_layout(&c, &[100.0]), None);
        assert_eq!(valid_layout(&c, &[f64::NAN, 100.0]), None);
    }

    #[test]
    fn a_handle_moves_the_two_neighbours_only() {
        let s = resize_pair(&[30.0, 30.0, 40.0], &free(3), 0, 10.0, Snap::Midpoint);
        assert_eq!(s, vec![40.0, 20.0, 40.0]);
        let s = resize_pair(&[30.0, 30.0, 40.0], &free(3), 1, -10.0, Snap::Midpoint);
        assert_eq!(s, vec![30.0, 20.0, 50.0]);
    }

    #[test]
    fn resizing_stops_at_min_and_max() {
        let c = [limits(20.0, 70.0), limits(10.0, 100.0)];
        assert_eq!(
            resize_pair(&[50.0, 50.0], &c, 0, 40.0, Snap::Midpoint),
            vec![70.0, 30.0]
        );
        assert_eq!(
            resize_pair(&[50.0, 50.0], &c, 0, -40.0, Snap::Midpoint),
            vec![20.0, 80.0]
        );
        // The neighbour's minimum limits it too.
        let c = [open(), limits(40.0, 100.0)];
        assert_eq!(
            resize_pair(&[50.0, 50.0], &c, 0, 30.0, Snap::Midpoint),
            vec![60.0, 40.0]
        );
    }

    #[test]
    fn the_total_never_changes() {
        let c = [limits(7.0, 80.0), limits(0.0, 55.0), collapsible(10.0, 0.0)];
        for h in 0..2 {
            for d in [-100.0, -33.3, -1.0, 0.0, 0.4, 12.5, 100.0] {
                for snap in [Snap::Midpoint, Snap::Eager] {
                    let s = resize_pair(&[30.0, 30.0, 40.0], &c, h, d, snap);
                    assert!(
                        (s.iter().sum::<f64>() - 100.0).abs() < 0.01,
                        "{h} {d}: {s:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_collapsible_panel_snaps_shut_past_halfway_and_back() {
        let c = [collapsible(20.0, 0.0), open()];
        let start = [30.0, 70.0];
        // 30 - 8 = 22: above the minimum, no collapse.
        assert_eq!(
            resize_pair(&start, &c, 0, -8.0, Snap::Midpoint),
            vec![22.0, 78.0]
        );
        // 30 - 15 = 15: below min, but above halfway (10) so it stays at the minimum.
        assert_eq!(
            resize_pair(&start, &c, 0, -15.0, Snap::Midpoint),
            vec![20.0, 80.0]
        );
        // 30 - 25 = 5: past halfway, collapsed.
        assert_eq!(
            resize_pair(&start, &c, 0, -25.0, Snap::Midpoint),
            vec![0.0, 100.0]
        );
        // Dragging from the collapsed state (start 0) back out: small moves stay shut.
        let shut = [0.0, 100.0];
        assert_eq!(
            resize_pair(&shut, &c, 0, 8.0, Snap::Midpoint),
            vec![0.0, 100.0]
        );
        assert_eq!(
            resize_pair(&shut, &c, 0, 12.0, Snap::Midpoint),
            vec![20.0, 80.0]
        );
        assert_eq!(
            resize_pair(&shut, &c, 0, 45.0, Snap::Midpoint),
            vec![45.0, 55.0]
        );
    }

    #[test]
    fn eager_snapping_collapses_on_the_first_step_below_the_minimum() {
        let c = [collapsible(20.0, 0.0), open()];
        assert_eq!(
            resize_pair(&[20.0, 80.0], &c, 0, -1.0, Snap::Eager),
            vec![0.0, 100.0]
        );
        assert_eq!(
            resize_pair(&[20.0, 80.0], &c, 0, -1.0, Snap::Midpoint),
            vec![20.0, 80.0]
        );
        // And opens again on the first step back.
        assert_eq!(
            resize_pair(&[0.0, 100.0], &c, 0, 1.0, Snap::Eager),
            vec![20.0, 80.0]
        );
    }

    #[test]
    fn the_second_panel_can_collapse_too() {
        let c = [open(), collapsible(20.0, 4.0)];
        assert_eq!(
            resize_pair(&[70.0, 30.0], &c, 0, 30.0, Snap::Midpoint),
            vec![96.0, 4.0]
        );
        assert_eq!(
            resize_pair(&[70.0, 30.0], &c, 0, 6.0, Snap::Midpoint),
            vec![76.0, 24.0]
        );
    }

    #[test]
    fn a_collapsed_strip_size_is_kept() {
        let c = [collapsible(20.0, 5.0), open()];
        assert_eq!(
            resize_pair(&[30.0, 70.0], &c, 0, -30.0, Snap::Midpoint),
            vec![5.0, 95.0]
        );
        assert!(c[0].is_collapsed(5.0));
        assert!(!c[0].is_collapsed(20.0));
    }

    #[test]
    fn impossible_limits_leave_the_sizes_alone() {
        // Both panels must be at least 60.
        let c = [limits(60.0, 100.0), limits(60.0, 100.0)];
        assert_eq!(
            resize_pair(&[50.0, 50.0], &c, 0, 10.0, Snap::Midpoint),
            vec![50.0, 50.0]
        );
    }

    #[test]
    fn handle_bounds_cover_the_continuous_range_and_collapsed_ends() {
        let c = [limits(20.0, 70.0), limits(10.0, 100.0)];
        assert_eq!(handle_bounds(&[50.0, 50.0], &c, 0), (20.0, 70.0));
        // The second panel may not exceed 60, so the first is at least 40; collapsing the
        // first would leave 100 for the second, which is over its maximum.
        let c = [collapsible(20.0, 0.0), limits(10.0, 60.0)];
        assert_eq!(handle_bounds(&[50.0, 50.0], &c, 0), (40.0, 90.0));
        let c = [collapsible(20.0, 0.0), open()];
        assert_eq!(handle_bounds(&[50.0, 50.0], &c, 0), (0.0, 100.0));
    }

    #[test]
    fn collapse_gives_the_space_to_the_next_panel() {
        let c = [collapsible(15.0, 0.0), open(), open()];
        assert_eq!(
            collapse_panel(&[25.0, 35.0, 40.0], &c, 0),
            vec![0.0, 60.0, 40.0]
        );
    }

    #[test]
    fn the_last_panel_gives_its_space_back_to_the_one_before() {
        let c = [open(), open(), collapsible(15.0, 0.0)];
        assert_eq!(
            collapse_panel(&[35.0, 40.0, 25.0], &c, 2),
            vec![35.0, 65.0, 0.0]
        );
    }

    #[test]
    fn collapse_overflows_to_the_next_panel_when_one_is_full() {
        let c = [collapsible(15.0, 0.0), limits(0.0, 50.0), open()];
        assert_eq!(
            collapse_panel(&[25.0, 40.0, 35.0], &c, 0),
            vec![0.0, 50.0, 50.0]
        );
    }

    #[test]
    fn collapse_is_refused_when_nobody_can_take_the_space() {
        let c = [collapsible(15.0, 0.0), limits(0.0, 60.0)];
        assert_eq!(collapse_panel(&[30.0, 60.0], &c, 0), vec![30.0, 60.0]);
        // Not collapsible at all.
        assert_eq!(collapse_panel(&[50.0, 50.0], &free(2), 0), vec![50.0, 50.0]);
    }

    #[test]
    fn expand_takes_the_space_back_from_the_neighbour() {
        let c = [collapsible(15.0, 0.0), open()];
        assert_eq!(expand_panel(&[0.0, 100.0], &c, 0, 25.0), vec![25.0, 75.0]);
        // Asked for less than the minimum: the minimum.
        assert_eq!(expand_panel(&[0.0, 100.0], &c, 0, 5.0), vec![15.0, 85.0]);
        // Not collapsed: nothing to do.
        assert_eq!(expand_panel(&[40.0, 60.0], &c, 0, 25.0), vec![40.0, 60.0]);
    }

    #[test]
    fn expand_is_limited_by_what_the_neighbours_can_give() {
        // The neighbour keeps at least 95, so only 5 can be taken: below the minimum 15.
        let c = [collapsible(15.0, 0.0), limits(95.0, 100.0)];
        assert_eq!(expand_panel(&[0.0, 100.0], &c, 0, 25.0), vec![0.0, 100.0]);
        // 20 to give of the 40 wanted: the panel gets 20, above its minimum.
        let c = [collapsible(15.0, 0.0), limits(80.0, 100.0)];
        assert_eq!(expand_panel(&[0.0, 100.0], &c, 0, 40.0), vec![20.0, 80.0]);
    }

    #[test]
    fn collapse_then_expand_returns_to_the_same_layout() {
        let c = [collapsible(15.0, 0.0), open(), open()];
        let before = [25.0, 35.0, 40.0];
        let shut = collapse_panel(&before, &c, 0);
        // Everything went to panel 1, so it pays it back.
        assert_eq!(expand_panel(&shut, &c, 0, 25.0), before.to_vec());
    }
}
