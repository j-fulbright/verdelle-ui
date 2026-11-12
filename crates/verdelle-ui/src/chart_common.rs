//! What every chart type shares: the SVG frame, the pointer and keyboard plumbing, the hidden data
//! table and the live announcement. Crate-private.

use crate::ChartTooltipState;
use crate::chart_context::use_chart;
use crate::primitives::chart::format_number;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::Element;

/// A key press that moves between data points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nav {
    Prev,
    Next,
    First,
    Last,
    Clear,
}

pub(crate) fn nav_for_key(key: &str) -> Option<Nav> {
    match key {
        "ArrowLeft" | "ArrowUp" => Some(Nav::Prev),
        "ArrowRight" | "ArrowDown" => Some(Nav::Next),
        "Home" => Some(Nav::First),
        "End" => Some(Nav::Last),
        "Escape" => Some(Nav::Clear),
        _ => None,
    }
}

/// The data point after a key press: arrows stop at the ends, and with nothing active the first
/// press lands on the first point.
pub(crate) fn step_index(current: Option<usize>, nav: Nav, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    match nav {
        Nav::Clear => None,
        Nav::First => Some(0),
        Nav::Last => Some(len - 1),
        Nav::Next => Some(current.map_or(0, |i| (i + 1).min(len - 1))),
        Nav::Prev => Some(current.map_or(0, |i| i.saturating_sub(1))),
    }
}

/// The chart's reactions to the pointer and keyboard. Each is optional.
#[derive(Clone, Copy, Default)]
pub(crate) struct Interaction {
    pub on_move: Option<Callback<web_sys::PointerEvent>>,
    pub on_key: Option<Callback<web_sys::KeyboardEvent>>,
    pub on_focus: Option<Callback<web_sys::FocusEvent>>,
    pub on_leave: Option<Callback<()>>,
}

/// The `<svg>` that holds an event's current target.
pub(crate) fn svg_of(ev: &web_sys::Event) -> Option<Element> {
    let el = ev.current_target()?.dyn_into::<Element>().ok()?;
    el.closest("svg").ok().flatten()
}

/// The size and offset of the drawing inside the `<svg>` box (the `viewBox` is fitted and centred).
fn fit(svg: &Element, vw: f64, vh: f64) -> (f64, f64, f64, f64, f64) {
    let r = svg.get_bounding_client_rect();
    let scale = (r.width() / vw).min(r.height() / vh);
    let scale = if scale.is_finite() && scale > 0.0 {
        scale
    } else {
        1.0
    };
    (
        r.left() + (r.width() - vw * scale) / 2.0,
        r.top() + (r.height() - vh * scale) / 2.0,
        scale,
        r.width(),
        r.height(),
    )
}

/// A pointer position (client pixels) as a point in the chart's own `viewBox` units.
pub(crate) fn client_to_svg(svg: &Element, vw: f64, vh: f64, cx: f64, cy: f64) -> (f64, f64) {
    let (left, top, scale, _, _) = fit(svg, vw, vh);
    ((cx - left) / scale, (cy - top) / scale)
}

/// A client pixel position as tooltip coordinates: from the container's corner, and whether the
/// tooltip should open leftwards / upwards (the point is in the far half).
pub(crate) fn client_to_root(root: &Element, cx: f64, cy: f64) -> (f64, f64, bool, bool) {
    let r = root.get_bounding_client_rect();
    let (x, y) = (cx - r.left(), cy - r.top());
    (x, y, x > r.width() * 0.55, y > r.height() * 0.6)
}

/// A point in the chart's `viewBox` units as tooltip coordinates (for keyboard focus).
pub(crate) fn svg_to_root(
    svg: &Element,
    root: &Element,
    vw: f64,
    vh: f64,
    x: f64,
    y: f64,
) -> (f64, f64, bool, bool) {
    let (left, top, scale, _, _) = fit(svg, vw, vh);
    client_to_root(root, left + x * scale, top + y * scale)
}

/// The container element, if it is in the page.
pub(crate) fn root_element() -> Option<Element> {
    use_chart().root.get_untracked().map(|d| {
        let el: &Element = d.as_ref();
        el.clone()
    })
}

/// Whether focus came from the keyboard (not a click).
pub(crate) fn is_focus_visible(ev: &web_sys::FocusEvent) -> bool {
    ev.current_target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .and_then(|el| el.matches(":focus-visible").ok())
        .unwrap_or(false)
}

/// Read a tooltip text aloud: `January: Desktop 186, Mobile 80`.
pub(crate) fn announce(state: &ChartTooltipState) -> String {
    let items = state
        .items
        .iter()
        .map(|i| format!("{} {}", i.name, format_number(i.value)))
        .collect::<Vec<_>>()
        .join(", ");
    if state.label.is_empty() {
        items
    } else {
        format!("{}: {items}", state.label)
    }
}

/// The hidden table every chart carries for assistive technology and for copy and paste.
pub(crate) fn data_table(caption: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> AnyView {
    view! {
        <table class="vdl-sr-only vdl-chart__table">
            <caption>{caption}</caption>
            <thead>
                <tr>{headers.into_iter().map(|h| view! { <th scope="col">{h}</th> }).collect_view()}</tr>
            </thead>
            <tbody>
                {rows
                    .into_iter()
                    .map(|row| {
                        let mut cells = row.into_iter();
                        let first = cells.next().unwrap_or_default();
                        view! {
                            <tr>
                                <th scope="row">{first}</th>
                                {cells.map(|c| view! { <td>{c}</td> }).collect_view()}
                            </tr>
                        }
                    })
                    .collect_view()}
            </tbody>
        </table>
    }
    .into_any()
}

/// The SVG of a chart with its accessible name, keyboard and pointer handlers, the hidden table and a
/// live region that reads out the focused data point.
#[component]
pub(crate) fn ChartFrame(
    kind: &'static str,
    vw: f64,
    vh: f64,
    title: String,
    description: String,
    interaction: Interaction,
    table: AnyView,
    children: Children,
) -> impl IntoView {
    let ctx = use_chart();
    let live = move || ctx.tooltip.get().map(|s| announce(&s)).unwrap_or_default();
    let on_move = interaction.on_move;
    let on_key = interaction.on_key;
    let on_focus = interaction.on_focus;
    let on_leave = interaction.on_leave;
    let label = title.clone();
    view! {
        <svg
            class="vdl-chart__svg"
            data-chart-kind=kind
            viewBox=format!("0 0 {} {}", format_dim(vw), format_dim(vh))
            role="group"
            aria-roledescription="chart"
            aria-label=label
            tabindex="0"
            on:pointermove=move |e| {
                if let Some(cb) = on_move {
                    cb.run(e);
                }
            }
            on:pointerleave=move |_| {
                if let Some(cb) = on_leave {
                    cb.run(());
                }
            }
            on:keydown=move |e| {
                if let Some(cb) = on_key {
                    cb.run(e);
                }
            }
            on:focus=move |e| {
                if let Some(cb) = on_focus {
                    cb.run(e);
                }
            }
            on:blur=move |_| {
                if let Some(cb) = on_leave {
                    cb.run(());
                }
            }
        >
            <title>{title}</title>
            <desc>{description}</desc>
            {children()}
        </svg>
        {table}
        <div class="vdl-sr-only" role="status" aria-live="polite">
            {live}
        </div>
    }
}

fn format_dim(v: f64) -> String {
    crate::primitives::chart::svg_num(v)
}

/// A rectangle with each corner rounded by its own radius, as path data. `r` is top left, top right,
/// bottom right, bottom left; each is capped by half the smaller side.
pub(crate) fn rounded_rect(x: f64, y: f64, w: f64, h: f64, r: [f64; 4]) -> String {
    use crate::primitives::chart::svg_num as n;
    if w <= 0.0 || h <= 0.0 {
        return String::new();
    }
    let cap = (w.min(h)) / 2.0;
    let [tl, tr, br, bl] = r.map(|v| v.clamp(0.0, cap));
    format!(
        "M{},{}H{}{}V{}{}H{}{}V{}{}Z",
        n(x + tl),
        n(y),
        n(x + w - tr),
        corner(tr, x + w, y + tr),
        n(y + h - br),
        corner(br, x + w - br, y + h),
        n(x + bl),
        corner(bl, x, y + h - bl),
        n(y + tl),
        corner(tl, x + tl, y),
    )
}

fn corner(r: f64, x: f64, y: f64) -> String {
    use crate::primitives::chart::svg_num as n;
    if r <= 0.0 {
        String::new()
    } else {
        format!("A{0},{0},0,0,1,{1},{2}", n(r), n(x), n(y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_stop_at_the_ends() {
        assert_eq!(step_index(Some(0), Nav::Prev, 5), Some(0));
        assert_eq!(step_index(Some(4), Nav::Next, 5), Some(4));
        assert_eq!(step_index(Some(1), Nav::Next, 5), Some(2));
        assert_eq!(step_index(Some(3), Nav::Prev, 5), Some(2));
    }

    #[test]
    fn the_first_key_press_lands_on_the_first_point() {
        assert_eq!(step_index(None, Nav::Next, 5), Some(0));
        assert_eq!(step_index(None, Nav::Prev, 5), Some(0));
    }

    #[test]
    fn home_end_and_escape() {
        assert_eq!(step_index(Some(2), Nav::First, 5), Some(0));
        assert_eq!(step_index(Some(2), Nav::Last, 5), Some(4));
        assert_eq!(step_index(Some(2), Nav::Clear, 5), None);
        assert_eq!(step_index(None, Nav::Next, 0), None);
    }

    #[test]
    fn keys_map_to_moves() {
        assert_eq!(nav_for_key("ArrowRight"), Some(Nav::Next));
        assert_eq!(nav_for_key("ArrowUp"), Some(Nav::Prev));
        assert_eq!(nav_for_key("Home"), Some(Nav::First));
        assert_eq!(nav_for_key("a"), None);
    }

    #[test]
    fn rounded_rect_rounds_only_the_asked_corners() {
        let d = rounded_rect(0.0, 0.0, 20.0, 40.0, [4.0, 4.0, 0.0, 0.0]);
        assert_eq!(d, "M4,0H16A4,4,0,0,1,20,4V40H0V4A4,4,0,0,1,4,0Z");
        assert_eq!(rounded_rect(0.0, 0.0, 0.0, 5.0, [1.0; 4]), "");
        // A radius larger than the bar is capped, so a tiny bar stays a valid shape.
        let tiny = rounded_rect(0.0, 0.0, 4.0, 2.0, [9.0; 4]);
        assert!(tiny.contains("A1,1,0,0,1"));
    }
}
