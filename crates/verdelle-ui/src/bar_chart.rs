use crate::chart_cartesian::{Cartesian, CatScale, Spec, axes, build, interaction};
use crate::chart_common::{ChartFrame, data_table, rounded_rect};
use crate::chart_context::use_chart;
use crate::chart_overlay::cursor;
use crate::primitives::chart::{StackOffset, format_number, svg_num as n};
use crate::{ChartDatum, ChartGrid, ChartOrientation};
use leptos::prelude::*;

/// A bar chart: grouped (one bar per series side by side), `stacked`, or `percent` (stacked to 100%),
/// upright or `orientation=Horizontal`. Put it in a [`crate::ChartContainer`] with a
/// [`crate::ChartConfig`] naming the series; add [`crate::ChartTooltip`] and [`crate::ChartLegend`]
/// beside it.
///
/// `data` is one [`ChartDatum`] per category; `series` picks which config keys to plot (default: all,
/// in config order). The value axis includes zero and ends on a round number unless `domain` is
/// given. `radius` rounds the outer end of each bar (stacked bars round only the outermost segment);
/// `bar_labels` prints the value at the end of every grouped bar. `grid` chooses the grid lines,
/// `category_axis` and `value_axis` switch the axes. `value_formatter` and `category_formatter`
/// change the axis labels. The chart is an SVG with the given `width` and `height` as its coordinate
/// system (it scales with the container). `title` and `description` name it for screen readers, which
/// also get a hidden data table and read the tooltip aloud. Focus the chart and use the arrow keys,
/// Home, End and Escape to move between categories.
#[component]
pub fn BarChart(
    #[prop(into)] data: Signal<Vec<ChartDatum>>,
    #[prop(optional, into)] series: Option<Vec<String>>,
    #[prop(optional)] orientation: ChartOrientation,
    #[prop(optional)] stacked: bool,
    #[prop(optional)] percent: bool,
    #[prop(default = true)] category_axis: bool,
    #[prop(default = true)] value_axis: bool,
    #[prop(optional)] grid: ChartGrid,
    #[prop(default = 4.0)] radius: f64,
    #[prop(optional)] bar_labels: bool,
    #[prop(default = true)] cursor_band: bool,
    #[prop(optional, into)] domain: Option<(f64, f64)>,
    #[prop(optional, into)] value_formatter: Option<Callback<f64, String>>,
    #[prop(optional, into)] category_formatter: Option<Callback<String, String>>,
    #[prop(default = 480.0)] width: f64,
    #[prop(default = 270.0)] height: f64,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] description: String,
) -> impl IntoView {
    let ctx = use_chart();
    let keys = series.unwrap_or_else(|| ctx.cfg().keys());
    let horizontal = orientation == ChartOrientation::Horizontal;
    let stack = if percent {
        Some(StackOffset::Expand)
    } else if stacked {
        Some(StackOffset::None)
    } else {
        None
    };
    let model = Memo::new(move |_| {
        let data = data.get();
        let visible = ctx.visible(&keys);
        build(&Spec {
            vw: width,
            vh: height,
            horizontal,
            band: true,
            point_padding: 0.0,
            stack,
            category_axis,
            value_axis,
            domain,
            keys: &keys,
            visible: &visible,
            data: &data,
            config: &ctx.cfg(),
            value_fmt: value_formatter,
            cat_fmt: category_formatter,
        })
    });
    let active = RwSignal::new(None::<usize>);
    let title = if title.is_empty() {
        "Bar chart".to_string()
    } else {
        title
    };
    let table_title = title.clone();
    let table = (move || {
        let m = model.get();
        let mut headers = vec!["Category".to_string()];
        headers.extend(m.series.iter().map(|s| s.name.clone()));
        data_table(table_title.clone(), headers, m.table_rows())
    })
    .into_any();

    view! {
        <ChartFrame
            kind="bar"
            vw=width
            vh=height
            title=title
            description=description
            interaction=interaction(model, active)
            table=table
        >
            {move || {
                let m = model.get();
                axes(&m, grid, category_axis, value_axis)
            }}
            {move || {
                if !cursor_band {
                    return None;
                }
                active.get().map(|i| cursor(&model.get(), i))
            }}
            {move || {
                let m = model.get();
                bars(&m, radius, bar_labels && !m.stacked, value_formatter)
            }}
        </ChartFrame>
    }
}

fn bars(
    m: &Cartesian,
    radius: f64,
    labels: bool,
    value_fmt: Option<Callback<f64, String>>,
) -> AnyView {
    let CatScale::Band(band) = &m.cat else {
        return ().into_any();
    };
    let ns = m.series.len();
    let mut out: Vec<AnyView> = Vec::new();
    for i in 0..m.count() {
        let (low, width) = band.extent(i);
        let gap = if ns > 1 && !m.stacked { 3.0 } else { 0.0 };
        let slot = if m.stacked {
            width
        } else {
            ((width - gap * (ns as f64 - 1.0)) / ns.max(1) as f64).max(1.0)
        };
        let bar_w = slot.min(48.0);
        for (si, s) in m.series.iter().enumerate() {
            let (v, seg) = match (s.values.get(i), s.segs.get(i)) {
                (Some(v), Some(seg)) if v.is_finite() && *v != 0.0 => (*v, *seg),
                _ => continue,
            };
            let cat_low = if m.stacked {
                low + (width - bar_w) / 2.0
            } else {
                low + si as f64 * (slot + gap) + (slot - bar_w) / 2.0
            };
            // Stacked segments leave a thin gap between them.
            let (inset_start, inset_end) = if m.stacked {
                (if seg.start == 0.0 { 0.0 } else { 0.75 }, 0.75)
            } else {
                (0.0, 0.0)
            };
            let (p0, p1) = (m.val_px(seg.start), m.val_px(seg.end));
            let dir = if p1 >= p0 { 1.0 } else { -1.0 };
            let (a, b) = (p0 + dir * inset_start, p1 - dir * inset_end);
            let (lo, hi) = (a.min(b), a.max(b));
            if hi - lo < 0.5 {
                continue;
            }
            // Only the outermost segment of a stack gets rounded ends.
            let outer = !m.stacked
                || !m.series[si + 1..].iter().any(|later| {
                    later
                        .values
                        .get(i)
                        .is_some_and(|lv| lv.is_finite() && *lv != 0.0 && (*lv > 0.0) == (v > 0.0))
                });
            let r = if outer { radius } else { 0.0 };
            let positive = v > 0.0;
            let (x, y, w, h, radii) = if m.horizontal {
                let radii = if positive {
                    [0.0, r, r, 0.0]
                } else {
                    [r, 0.0, 0.0, r]
                };
                (lo, cat_low, hi - lo, bar_w, radii)
            } else {
                let radii = if positive {
                    [r, r, 0.0, 0.0]
                } else {
                    [0.0, 0.0, r, r]
                };
                (cat_low, lo, bar_w, hi - lo, radii)
            };
            out.push(
                view! {
                    <path
                        class="vdl-chart__bar"
                        d=rounded_rect(x, y, w, h, radii)
                        style=format!("fill:{}", s.color)
                        data-series=s.key.clone()
                    />
                }
                .into_any(),
            );
            if labels {
                let text = match value_fmt {
                    Some(f) => f.run(v),
                    None => format_number(v),
                };
                let (tx, ty, anchor, dy) = if m.horizontal {
                    if positive {
                        (hi + 6.0, cat_low + bar_w / 2.0, "start", "0.32em")
                    } else {
                        (lo - 6.0, cat_low + bar_w / 2.0, "end", "0.32em")
                    }
                } else if positive {
                    (cat_low + bar_w / 2.0, lo - 6.0, "middle", "0")
                } else {
                    (cat_low + bar_w / 2.0, hi + 14.0, "middle", "0")
                };
                out.push(
                    view! {
                        <text
                            class="vdl-chart__value-label"
                            x=n(tx)
                            y=n(ty)
                            dy=dy
                            text-anchor=anchor
                        >
                            {text}
                        </text>
                    }
                    .into_any(),
                );
            }
        }
    }
    view! { <g>{out}</g> }.into_any()
}
