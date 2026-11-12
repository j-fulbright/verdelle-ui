//! The shared model and drawing of the cartesian charts (bar, line, area): layout, scales, ticks,
//! grid, axes, and hover / keyboard handling. Crate-private.

use crate::chart_common::{
    Interaction, Nav, client_to_root, client_to_svg, is_focus_visible, nav_for_key, root_element,
    step_index, svg_of, svg_to_root,
};
use crate::chart_context::{ChartContext, ChartTooltipItem, ChartTooltipState, use_chart};
use crate::primitives::chart::{
    BandScale, LinearScale, PointScale, Segment, StackOffset, format_tick, nice_scale, stack,
    svg_num as n,
};
use crate::{ChartConfig, ChartDatum, ChartGrid, IconName};
use leptos::prelude::*;

/// Where categories sit along their axis.
#[derive(Clone, PartialEq)]
pub(crate) enum CatScale {
    Band(BandScale),
    Point(PointScale),
}

/// One plotted series with everything the drawing needs.
#[derive(Clone, PartialEq)]
pub(crate) struct CSeries {
    pub key: String,
    pub name: String,
    pub color: String,
    pub icon: Option<IconName>,
    pub values: Vec<f64>,
    pub segs: Vec<Segment>,
}

/// The laid-out chart: plot rectangle, scales, ticks, labels and series.
#[derive(Clone, PartialEq)]
pub(crate) struct Cartesian {
    pub vw: f64,
    pub vh: f64,
    pub horizontal: bool,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    /// Category names as in the data.
    pub cats: Vec<String>,
    /// Category names as the axis shows them.
    pub cat_labels: Vec<String>,
    pub cat: CatScale,
    pub val: LinearScale,
    pub ticks: Vec<f64>,
    pub tick_labels: Vec<String>,
    pub series: Vec<CSeries>,
    /// Draw every n-th category label so they do not overlap.
    pub label_every: usize,
    pub stacked: bool,
}

/// Everything that decides the layout. Callbacks are run while building, so the model holds only text.
pub(crate) struct Spec<'a> {
    pub vw: f64,
    pub vh: f64,
    pub horizontal: bool,
    /// Bars sit in bands; lines and areas on points.
    pub band: bool,
    pub point_padding: f64,
    pub stack: Option<StackOffset>,
    pub category_axis: bool,
    pub value_axis: bool,
    pub domain: Option<(f64, f64)>,
    pub keys: &'a [String],
    pub visible: &'a [String],
    pub data: &'a [ChartDatum],
    pub config: &'a ChartConfig,
    pub value_fmt: Option<Callback<f64, String>>,
    pub cat_fmt: Option<Callback<String, String>>,
}

const CHAR_W: f64 = 6.4;

pub(crate) fn build(spec: &Spec) -> Cartesian {
    let cats: Vec<String> = spec.data.iter().map(|d| d.name.clone()).collect();
    let count = cats.len();
    let percent = spec.stack == Some(StackOffset::Expand);

    let mut series: Vec<CSeries> = Vec::new();
    for (pos, key) in spec.keys.iter().enumerate() {
        if !spec.visible.contains(key) {
            continue;
        }
        let values: Vec<f64> = spec
            .data
            .iter()
            .map(|d| d.get(key).unwrap_or(f64::NAN))
            .collect();
        series.push(CSeries {
            key: key.clone(),
            name: spec.config.label(key),
            color: spec.config.color_of(key, pos),
            icon: spec.config.get(key).and_then(|s| s.icon),
            values,
            segs: Vec::new(),
        });
    }
    let raw: Vec<Vec<f64>> = series.iter().map(|s| s.values.clone()).collect();
    match spec.stack {
        Some(offset) => {
            for (s, segs) in series.iter_mut().zip(stack(&raw, offset)) {
                s.segs = segs;
            }
        }
        None => {
            for s in &mut series {
                s.segs = s
                    .values
                    .iter()
                    .map(|v| Segment {
                        start: 0.0,
                        end: if v.is_finite() { *v } else { 0.0 },
                    })
                    .collect();
            }
        }
    }

    // The value domain always includes zero unless the caller gives one.
    let (mut lo, mut hi) = (0.0f64, 0.0f64);
    for s in &series {
        for seg in &s.segs {
            lo = lo.min(seg.low());
            hi = hi.max(seg.high());
        }
    }
    if series.is_empty() || (lo == 0.0 && hi == 0.0) {
        hi = 1.0;
    }
    let ns = match (spec.domain, percent) {
        (Some((a, b)), _) => nice_scale(a, b, 4, false),
        (None, true) => nice_scale(0.0, 1.0, 4, true),
        (None, false) => nice_scale(lo, hi, 4, true),
    };
    let (d0, d1) = match spec.domain {
        Some(d) => (d.0.min(d.1), d.0.max(d.1)),
        None => (ns.min, ns.max),
    };
    let tick_labels: Vec<String> = ns
        .ticks
        .iter()
        .map(|t| match spec.value_fmt {
            Some(f) => f.run(*t),
            None if percent => format!("{}%", (t * 100.0).round()),
            None => format_tick(*t, ns.step),
        })
        .collect();
    let cat_labels: Vec<String> = cats
        .iter()
        .map(|c| match spec.cat_fmt {
            Some(f) => f.run(c.clone()),
            None => spec.config.label(c),
        })
        .collect();

    let tick_w = tick_labels
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0) as f64
        * CHAR_W;
    let cat_w = (cat_labels
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0) as f64
        * CHAR_W)
        .min(120.0);

    let (vw, vh) = (spec.vw, spec.vh);
    let left = 8.0
        + if !spec.horizontal && spec.value_axis {
            tick_w + 8.0
        } else if spec.horizontal && spec.category_axis {
            cat_w + 8.0
        } else {
            0.0
        };
    let bottom = 8.0
        + if (!spec.horizontal && spec.category_axis) || (spec.horizontal && spec.value_axis) {
            22.0
        } else {
            0.0
        };
    // Room above for value labels on bars; a little extra at the right for horizontal bar labels.
    let right = if spec.horizontal { 36.0 } else { 16.0 };
    let (x0, x1, y0, y1) = (left, vw - right, 14.0, vh - bottom);

    let (cat_range, val_range) = if spec.horizontal {
        ((y0, y1), (x0, x1))
    } else {
        ((x0, x1), (y1, y0))
    };
    let cat = if spec.band {
        CatScale::Band(BandScale::new(count, cat_range, 0.25, 0.1))
    } else {
        CatScale::Point(PointScale::new(count, cat_range, spec.point_padding))
    };
    let step = match &cat {
        CatScale::Band(b) => b.step(),
        CatScale::Point(p) => p.step(),
    };
    let label_extent = if spec.horizontal {
        14.0
    } else {
        cat_labels
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(1) as f64
            * CHAR_W
            + 8.0
    };
    let label_every = if step > 0.0 {
        (label_extent / step).ceil().max(1.0) as usize
    } else {
        1
    };

    Cartesian {
        vw,
        vh,
        horizontal: spec.horizontal,
        x0,
        y0,
        x1,
        y1,
        cats,
        cat_labels,
        cat,
        val: LinearScale::new((d0, d1), val_range),
        ticks: ns.ticks,
        tick_labels,
        series,
        label_every,
        stacked: spec.stack.is_some(),
    }
}

impl Cartesian {
    pub fn count(&self) -> usize {
        self.cats.len()
    }

    pub fn cat_center(&self, i: usize) -> f64 {
        match &self.cat {
            CatScale::Band(b) => b.center(i),
            CatScale::Point(p) => p.position(i),
        }
    }

    pub fn val_px(&self, v: f64) -> f64 {
        self.val.apply(v)
    }

    /// The category under a point in `viewBox` units, if the point is in the plot.
    pub fn index_at(&self, sx: f64, sy: f64) -> Option<usize> {
        if sx < self.x0 || sx > self.x1 || sy < self.y0 || sy > self.y1 {
            return None;
        }
        let coord = if self.horizontal { sy } else { sx };
        match &self.cat {
            CatScale::Band(b) => b.index_at(coord),
            CatScale::Point(p) => p.index_at(coord),
        }
    }

    /// The (x, y) of the point of value `v` in category `i`.
    pub fn point(&self, i: usize, v: f64) -> (f64, f64) {
        let c = self.cat_center(i);
        let p = self.val_px(v);
        if self.horizontal { (p, c) } else { (c, p) }
    }

    /// Where a tooltip for category `i` is anchored when reached by keyboard (viewBox units).
    fn keyboard_anchor(&self, i: usize) -> (f64, f64) {
        let top = self
            .series
            .iter()
            .filter_map(|s| s.segs.get(i))
            .map(|s| s.high())
            .fold(0.0f64, f64::max);
        self.point(i, top)
    }

    fn tooltip_items(&self, i: usize) -> Vec<ChartTooltipItem> {
        self.series
            .iter()
            .filter_map(|s| {
                let v = *s.values.get(i)?;
                v.is_finite().then(|| ChartTooltipItem {
                    key: s.key.clone(),
                    name: s.name.clone(),
                    value: v,
                    color: s.color.clone(),
                    icon: s.icon,
                })
            })
            .collect()
    }

    /// Rows for the hidden data table: one per category, a column per plotted series.
    pub fn table_rows(&self) -> Vec<Vec<String>> {
        (0..self.count())
            .map(|i| {
                let mut row = vec![self.cats[i].clone()];
                row.extend(self.series.iter().map(|s| match s.values.get(i) {
                    Some(v) if v.is_finite() => crate::primitives::chart::format_number(*v),
                    _ => String::new(),
                }));
                row
            })
            .collect()
    }
}

/// Grid lines, the baseline and the axis labels.
pub(crate) fn axes(
    m: &Cartesian,
    grid: ChartGrid,
    category_axis: bool,
    value_axis: bool,
) -> AnyView {
    let mut out: Vec<AnyView> = Vec::new();
    if grid.has_value() {
        for t in &m.ticks {
            let p = m.val_px(*t);
            let (a, b, c, d) = if m.horizontal {
                (p, m.y0, p, m.y1)
            } else {
                (m.x0, p, m.x1, p)
            };
            out.push(
                view! { <line class="vdl-chart__grid" x1=n(a) y1=n(b) x2=n(c) y2=n(d) /> }
                    .into_any(),
            );
        }
    }
    if grid.has_category() {
        for i in 0..m.count() {
            let c = m.cat_center(i);
            let (a, b, cc, d) = if m.horizontal {
                (m.x0, c, m.x1, c)
            } else {
                (c, m.y0, c, m.y1)
            };
            out.push(
                view! { <line class="vdl-chart__grid" x1=n(a) y1=n(b) x2=n(cc) y2=n(d) /> }
                    .into_any(),
            );
        }
    }
    // The line categories stand on: the plot's bottom (or left) edge, plus a zero line when the
    // values cross zero.
    if category_axis {
        let (a, b, c, d) = if m.horizontal {
            (m.x0, m.y0, m.x0, m.y1)
        } else {
            (m.x0, m.y1, m.x1, m.y1)
        };
        out.push(
            view! { <line class="vdl-chart__axis-line" x1=n(a) y1=n(b) x2=n(c) y2=n(d) /> }
                .into_any(),
        );
    }
    let (v0, v1) = (m.val.d0.min(m.val.d1), m.val.d0.max(m.val.d1));
    if v0 < 0.0 && v1 > 0.0 {
        let p = m.val_px(0.0);
        let (a, b, c, d) = if m.horizontal {
            (p, m.y0, p, m.y1)
        } else {
            (m.x0, p, m.x1, p)
        };
        out.push(
            view! { <line class="vdl-chart__zero" x1=n(a) y1=n(b) x2=n(c) y2=n(d) /> }.into_any(),
        );
    }
    if value_axis {
        for (t, label) in m.ticks.iter().zip(&m.tick_labels) {
            let p = m.val_px(*t);
            if m.horizontal {
                out.push(
                    view! {
                        <text class="vdl-chart__tick" x=n(p) y=n(m.y1 + 16.0) text-anchor="middle">
                            {label.clone()}
                        </text>
                    }
                    .into_any(),
                );
            } else {
                out.push(
                    view! {
                        <text
                            class="vdl-chart__tick"
                            x=n(m.x0 - 8.0)
                            y=n(p)
                            dy="0.32em"
                            text-anchor="end"
                        >
                            {label.clone()}
                        </text>
                    }
                    .into_any(),
                );
            }
        }
    }
    if category_axis {
        let last = m.count().saturating_sub(1);
        for (i, label) in m.cat_labels.iter().enumerate() {
            if i % m.label_every != 0 {
                continue;
            }
            let c = m.cat_center(i);
            if m.horizontal {
                out.push(
                    view! {
                        <text
                            class="vdl-chart__tick"
                            x=n(m.x0 - 8.0)
                            y=n(c)
                            dy="0.32em"
                            text-anchor="end"
                        >
                            {label.clone()}
                        </text>
                    }
                    .into_any(),
                );
            } else {
                // The first and last label on a chart that runs to its edges lean inwards.
                let anchor = if i == 0 && c - m.x0 < 24.0 {
                    "start"
                } else if i == last && m.x1 - c < 24.0 {
                    "end"
                } else {
                    "middle"
                };
                out.push(
                    view! {
                        <text class="vdl-chart__tick" x=n(c) y=n(m.y1 + 16.0) text-anchor=anchor>
                            {label.clone()}
                        </text>
                    }
                    .into_any(),
                );
            }
        }
    }
    view! { <g>{out}</g> }.into_any()
}

fn show(ctx: ChartContext, m: &Cartesian, i: usize, at: (f64, f64, bool, bool)) {
    ctx.tooltip.set(Some(ChartTooltipState {
        label: ctx.cfg().label(&m.cats[i]),
        items: m.tooltip_items(i),
        x: at.0,
        y: at.1,
        flip_x: at.2,
        flip_y: at.3,
    }));
}

/// The pointer and keyboard behaviour of a cartesian chart: the active category is `active`.
pub(crate) fn interaction(model: Memo<Cartesian>, active: RwSignal<Option<usize>>) -> Interaction {
    let ctx = use_chart();
    let clear = move || {
        active.set(None);
        ctx.tooltip.set(None);
    };
    let go_key = move |svg: leptos::web_sys::Element, nav: Nav| {
        let m = model.get_untracked();
        match step_index(active.get_untracked(), nav, m.count()) {
            None => clear(),
            Some(i) => {
                let (ax, ay) = m.keyboard_anchor(i);
                if let Some(root) = root_element() {
                    let at = svg_to_root(&svg, &root, m.vw, m.vh, ax, ay);
                    active.set(Some(i));
                    show(ctx, &m, i, at);
                }
            }
        }
    };
    Interaction {
        on_move: Some(Callback::new(move |ev: leptos::web_sys::PointerEvent| {
            let (Some(svg), Some(root)) = (svg_of(&ev), root_element()) else {
                return;
            };
            let m = model.get_untracked();
            let (cx, cy) = (ev.client_x() as f64, ev.client_y() as f64);
            let (sx, sy) = client_to_svg(&svg, m.vw, m.vh, cx, cy);
            match m.index_at(sx, sy) {
                Some(i) => {
                    active.set(Some(i));
                    show(ctx, &m, i, client_to_root(&root, cx, cy));
                }
                None => clear(),
            }
        })),
        on_key: Some(Callback::new(move |ev: leptos::web_sys::KeyboardEvent| {
            let Some(nav) = nav_for_key(&ev.key()) else {
                return;
            };
            ev.prevent_default();
            if let Some(svg) = svg_of(&ev) {
                go_key(svg, nav);
            }
        })),
        on_focus: Some(Callback::new(move |ev: leptos::web_sys::FocusEvent| {
            if active.get_untracked().is_none()
                && is_focus_visible(&ev)
                && let Some(svg) = svg_of(&ev)
            {
                go_key(svg, Nav::First);
            }
        })),
        on_leave: Some(Callback::new(move |_| clear())),
    }
}
