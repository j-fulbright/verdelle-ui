use crate::ChartDatum;
use crate::chart_common::{ChartFrame, data_table};
use crate::chart_context::use_chart;
use crate::chart_polar::{PolarGeom, PolarItem, center_text, interaction};
use crate::primitives::chart::{arc_path, format_number, pie_layout, polar, svg_num as n};
use leptos::prelude::*;

/// A pie, or with `inner_radius` a donut. Each [`ChartDatum`] is a slice: its `name` is the config key
/// (so the label and colour come from the [`crate::ChartConfig`]) and `value_key` says which of its
/// values to plot (default: the first).
///
/// `inner_radius` is the hole as a fraction of the radius (0 is a pie; 0.6 a donut). `pad_angle` is the
/// gap between slices in degrees. `start_angle` and `end_angle` are in degrees clockwise from the
/// top (0 and 360 by default; -90 and 90 draws a half circle sitting on the bottom edge, for which
/// give a shorter `height`). `labels` prints each slice's value outside it. `center_value` and
/// `center_label` put text in the middle of a donut (a total, say). Slices hidden by the
/// [`crate::ChartLegend`] drop out and the rest share the circle. Hover a slice, or focus the chart
/// and use the arrow keys, for the tooltip.
#[component]
pub fn PieChart(
    #[prop(into)] data: Signal<Vec<ChartDatum>>,
    #[prop(optional, into)] value_key: Option<String>,
    #[prop(optional)] inner_radius: f64,
    #[prop(optional)] pad_angle: f64,
    #[prop(default = 0.0)] start_angle: f64,
    #[prop(default = 360.0)] end_angle: f64,
    #[prop(optional)] labels: bool,
    #[prop(optional, into)] center_value: Option<String>,
    #[prop(optional, into)] center_label: Option<String>,
    #[prop(default = 300.0)] width: f64,
    #[prop(default = 300.0)] height: f64,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] description: String,
) -> impl IntoView {
    let ctx = use_chart();
    let (a0, a1) = (start_angle.to_radians(), end_angle.to_radians());
    let half = (a1 - a0).abs() <= std::f64::consts::PI + 1e-6;
    let value_key = StoredValue::new(value_key);
    let inner = inner_radius.clamp(0.0, 0.95);

    // Slices and the arc they are drawn with, rebuilt when the data or the hidden set changes.
    let model = Memo::new(move |_| {
        let data = data.get();
        let cfg = ctx.cfg();
        let hidden = ctx.hidden.get();
        let key = value_key.get_value();
        let rows: Vec<(usize, String, f64)> = data
            .iter()
            .enumerate()
            .filter(|(_, d)| !hidden.contains(&d.name))
            .filter_map(|(pos, d)| {
                let v = match &key {
                    Some(k) => d.get(k),
                    None => d.values.first().map(|(_, v)| *v),
                }?;
                Some((pos, d.name.clone(), v))
            })
            .collect();
        let values: Vec<f64> = rows.iter().map(|r| r.2).collect();
        let slices = pie_layout(&values, a0, a1, pad_angle.to_radians());
        let edge = if labels { 40.0 } else { 8.0 };
        let radius = (width / 2.0).min(if half { height - 12.0 } else { height / 2.0 }) - edge;
        let radius = radius.max(10.0);
        let (cx, cy) = (width / 2.0, if half { height - 12.0 } else { height / 2.0 });
        let items = rows
            .iter()
            .zip(&slices)
            .map(|((pos, name, v), s)| PolarItem {
                key: name.clone(),
                name: cfg.label(name),
                color: cfg.color_of(name, *pos),
                icon: cfg.get(name).and_then(|c| c.icon),
                value: *v,
                r_in: radius * inner,
                r_out: radius,
                a0: s.start,
                a1: s.end,
            })
            .collect();
        PolarGeom {
            vw: width,
            vh: height,
            cx,
            cy,
            items,
        }
    });
    let active = RwSignal::new(None::<usize>);
    let title = if title.is_empty() {
        "Pie chart".to_string()
    } else {
        title
    };
    let table_title = title.clone();
    let table = (move || {
        data_table(
            table_title.clone(),
            vec!["Name".into(), "Value".into()],
            model.get().table_rows(),
        )
    })
    .into_any();

    view! {
        <ChartFrame
            kind="pie"
            vw=width
            vh=height
            title=title
            description=description
            interaction=interaction(model, active)
            table=table
        >
            {move || {
                let g = model.get();
                let slices: Vec<AnyView> = g
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, it)| {
                        view! {
                            <path
                                class="vdl-chart__slice"
                                d=arc_path(g.cx, g.cy, it.r_in, it.r_out, it.a0, it.a1)
                                style=format!("fill:{}", it.color)
                                data-key=it.key.clone()
                                data-active=move || (active.get() == Some(i)).then_some("")
                                data-dim=move || {
                                    active.get().is_some_and(|a| a != i).then_some("")
                                }
                            />
                        }
                            .into_any()
                    })
                    .collect();
                let outside: Vec<AnyView> = if labels {
                    g.items
                        .iter()
                        .filter(|it| (it.a1 - it.a0).abs() > 0.12)
                        .map(|it| {
                            let mid = (it.a0 + it.a1) / 2.0;
                            let (x1, y1) = polar(g.cx, g.cy, it.r_out + 2.0, mid);
                            let (x2, y2) = polar(g.cx, g.cy, it.r_out + 12.0, mid);
                            let (tx, ty) = polar(g.cx, g.cy, it.r_out + 16.0, mid);
                            let anchor = if mid.sin() > 0.15 {
                                "start"
                            } else if mid.sin() < -0.15 {
                                "end"
                            } else {
                                "middle"
                            };
                            let dy = if mid.cos() > 0.6 {
                                "0"
                            } else if mid.cos() < -0.6 {
                                "0.8em"
                            } else {
                                "0.32em"
                            };
                            view! {
                                <g pointer-events="none">
                                    <line
                                        class="vdl-chart__leader"
                                        x1=n(x1)
                                        y1=n(y1)
                                        x2=n(x2)
                                        y2=n(y2)
                                    />
                                    <text
                                        class="vdl-chart__value-label"
                                        x=n(tx)
                                        y=n(ty)
                                        dy=dy
                                        text-anchor=anchor
                                    >
                                        {format_number(it.value)}
                                    </text>
                                </g>
                            }
                                .into_any()
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                let centre_y = if half { g.cy - 26.0 } else { g.cy };
                view! {
                    <g>{slices}</g>
                    <g>{outside}</g>
                    {center_text(g.cx, centre_y, center_value.clone(), center_label.clone())}
                }
            }}
        </ChartFrame>
    }
}
