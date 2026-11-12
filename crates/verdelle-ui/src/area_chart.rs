use crate::chart_cartesian::{Cartesian, Spec, axes, build, interaction};
use crate::chart_common::{ChartFrame, data_table};
use crate::chart_context::use_chart;
use crate::chart_overlay::{active_dots, cursor};
use crate::primitives::chart::{Point, StackOffset, area_path, line_path, svg_num as n};
use crate::{ChartCurve, ChartDatum, ChartGrid};
use leptos::prelude::*;

/// An area chart: each series filled down to the baseline, or `stacked` on the one below (`percent`
/// stacks to 100%). `curve` joins the points as in [`crate::LineChart`]; `fill_opacity` (0.4 by
/// default) is how strong the fill is, and `line` draws the stroke along each top edge. Areas run
/// edge to edge unless `point_padding` leaves space at the ends. The other options are those of
/// [`crate::BarChart`].
#[component]
pub fn AreaChart(
    #[prop(into)] data: Signal<Vec<ChartDatum>>,
    #[prop(optional, into)] series: Option<Vec<String>>,
    #[prop(optional)] curve: ChartCurve,
    #[prop(optional)] stacked: bool,
    #[prop(optional)] percent: bool,
    #[prop(default = 0.4)] fill_opacity: f64,
    #[prop(default = true)] line: bool,
    #[prop(optional)] point_padding: f64,
    #[prop(default = true)] category_axis: bool,
    #[prop(default = true)] value_axis: bool,
    #[prop(optional)] grid: ChartGrid,
    #[prop(default = true)] cursor_line: bool,
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
            horizontal: false,
            band: false,
            point_padding,
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
        "Area chart".to_string()
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
            kind="area"
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
            {move || areas(&model.get(), curve, fill_opacity, line)}
            {move || {
                if !cursor_line {
                    return None;
                }
                active.get().map(|i| cursor(&model.get(), i))
            }}
            {move || active.get().map(|i| active_dots(&model.get(), i))}
        </ChartFrame>
    }
}

fn areas(m: &Cartesian, curve: ChartCurve, fill_opacity: f64, line: bool) -> AnyView {
    let (d_lo, d_hi) = (m.val.d0.min(m.val.d1), m.val.d0.max(m.val.d1));
    let floor = 0.0f64.clamp(d_lo, d_hi);
    let mut out: Vec<AnyView> = Vec::new();
    for s in &m.series {
        let mut top: Vec<Point> = Vec::new();
        let mut bottom: Vec<Point> = Vec::new();
        for i in 0..m.count() {
            let (Some(v), Some(seg)) = (s.values.get(i), s.segs.get(i)) else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            let (hi, lo) = if m.stacked {
                (seg.end, seg.start)
            } else {
                (*v, floor)
            };
            let (tx, ty) = m.point(i, hi);
            let (bx, by) = m.point(i, lo);
            top.push(Point::new(tx, ty));
            bottom.push(Point::new(bx, by));
        }
        out.push(
            view! {
                <path
                    class="vdl-chart__area"
                    d=area_path(&top, &bottom, curve)
                    style=format!("fill:{};fill-opacity:{}", s.color, n(fill_opacity))
                    data-series=s.key.clone()
                />
            }
            .into_any(),
        );
        if line {
            out.push(
                view! {
                    <path
                        class="vdl-chart__line"
                        d=line_path(&top, curve)
                        style=format!("stroke:{}", s.color)
                        stroke-width="2"
                    />
                }
                .into_any(),
            );
        }
    }
    view! { <g>{out}</g> }.into_any()
}
