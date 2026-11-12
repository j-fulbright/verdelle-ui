use crate::chart_cartesian::{Cartesian, Spec, axes, build, interaction};
use crate::chart_common::{ChartFrame, data_table};
use crate::chart_context::use_chart;
use crate::chart_overlay::{active_dots, cursor};
use crate::primitives::chart::{Point, line_path, svg_num as n};
use crate::{ChartCurve, ChartDatum, ChartGrid};
use leptos::prelude::*;

/// A line chart: one line per series over the categories of `data` (one [`ChartDatum`] each).
///
/// `curve` is how points are joined (monotone by default, which never overshoots the data; also
/// linear, natural and three step types). `dots` marks every point. Hovering or focusing a category
/// draws a crosshair and a ring on each line, and the tooltip lists every series there. A missing
/// value leaves a gap in the tooltip and the line joins across it. `point_padding` is the space
/// before the first and after the last point as a fraction of the spacing (0.3 by default).
/// The other options are those of [`crate::BarChart`].
#[component]
pub fn LineChart(
    #[prop(into)] data: Signal<Vec<ChartDatum>>,
    #[prop(optional, into)] series: Option<Vec<String>>,
    #[prop(optional)] curve: ChartCurve,
    #[prop(optional)] dots: bool,
    #[prop(default = 2.0)] stroke_width: f64,
    #[prop(default = 0.3)] point_padding: f64,
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
    let model = Memo::new(move |_| {
        let data = data.get();
        let visible = ctx.visible(&keys);
        build(&Spec {
            vw: width,
            vh: height,
            horizontal: false,
            band: false,
            point_padding,
            stack: None,
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
        "Line chart".to_string()
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
            kind="line"
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
                if !cursor_line {
                    return None;
                }
                active.get().map(|i| cursor(&model.get(), i))
            }}
            {move || lines(&model.get(), curve, dots, stroke_width)}
            {move || active.get().map(|i| active_dots(&model.get(), i))}
        </ChartFrame>
    }
}

fn lines(m: &Cartesian, curve: ChartCurve, dots: bool, stroke_width: f64) -> AnyView {
    let mut out: Vec<AnyView> = Vec::new();
    for s in &m.series {
        let pts: Vec<Point> = (0..m.count())
            .filter_map(|i| {
                let v = *s.values.get(i)?;
                v.is_finite().then(|| {
                    let (x, y) = m.point(i, v);
                    Point::new(x, y)
                })
            })
            .collect();
        out.push(
            view! {
                <path
                    class="vdl-chart__line"
                    d=line_path(&pts, curve)
                    style=format!("stroke:{}", s.color)
                    stroke-width=n(stroke_width)
                    data-series=s.key.clone()
                />
            }
            .into_any(),
        );
        if dots {
            for p in &pts {
                out.push(
                    view! {
                        <circle
                            class="vdl-chart__dot"
                            cx=n(p.x)
                            cy=n(p.y)
                            r="3.5"
                            style=format!("fill:{}", s.color)
                        />
                    }
                    .into_any(),
                );
            }
        }
    }
    view! { <g>{out}</g> }.into_any()
}
