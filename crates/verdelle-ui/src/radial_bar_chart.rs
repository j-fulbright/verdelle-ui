use crate::ChartDatum;
use crate::chart_common::{ChartFrame, data_table};
use crate::chart_context::use_chart;
use crate::chart_polar::{PolarGeom, PolarItem, center_text, interaction};
use crate::primitives::chart::{arc_path, arc_stroke_path, pie_layout, svg_num as n};
use leptos::prelude::*;

/// A radial bar chart. By default every [`ChartDatum`] is a ring (the first outermost) whose arc
/// is its value as a share of `max` (default: the largest value); `track` draws the empty ring behind
/// it. With `stacked` the first datum is one ring split between its series (the config keys in
/// `series`, default all), each arc its share of the total, as in shadcn's stacked radial chart.
///
/// `start_angle` and `end_angle` are degrees clockwise from the top (0 and 360; -90 and 90 is a half
/// ring sitting on the bottom edge). `inner_radius` is the hole as a fraction of the radius and
/// `bar_gap` the space between rings as a fraction of a ring's width. `center_value` and
/// `center_label` put text in the middle. Other options are those of [`crate::PieChart`].
#[component]
pub fn RadialBarChart(
    #[prop(into)] data: Signal<Vec<ChartDatum>>,
    #[prop(optional, into)] value_key: Option<String>,
    #[prop(optional, into)] series: Option<Vec<String>>,
    #[prop(optional)] stacked: bool,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(default = 0.3)] inner_radius: f64,
    #[prop(default = 0.25)] bar_gap: f64,
    #[prop(default = true)] track: bool,
    #[prop(default = 0.0)] start_angle: f64,
    #[prop(default = 360.0)] end_angle: f64,
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
    let series = StoredValue::new(series);
    let inner = inner_radius.clamp(0.0, 0.9);
    let gap = bar_gap.clamp(0.0, 0.8);

    let model = Memo::new(move |_| {
        let data = data.get();
        let cfg = ctx.cfg();
        let hidden = ctx.hidden.get();
        let (cx, cy) = (width / 2.0, if half { height - 12.0 } else { height / 2.0 });
        let radius =
            ((width / 2.0).min(if half { height - 12.0 } else { height / 2.0 }) - 8.0).max(10.0);

        let items: Vec<PolarItem> = if stacked {
            let keys = series.get_value().unwrap_or_else(|| cfg.keys());
            let first = data.first().cloned().unwrap_or_default();
            let rows: Vec<(usize, String, f64)> = keys
                .iter()
                .enumerate()
                .filter(|(_, k)| !hidden.contains(k))
                .filter_map(|(pos, k)| Some((pos, k.clone(), first.get(k)?)))
                .collect();
            let values: Vec<f64> = rows.iter().map(|r| r.2).collect();
            let slices = pie_layout(&values, a0, a1, 1.5f64.to_radians());
            let r_out = radius;
            let r_in = radius * (1.0 - 0.3f64).max(inner);
            rows.iter()
                .zip(&slices)
                .map(|((pos, key, v), s)| PolarItem {
                    key: key.clone(),
                    name: cfg.label(key),
                    color: cfg.color_of(key, *pos),
                    icon: cfg.get(key).and_then(|c| c.icon),
                    value: *v,
                    r_in,
                    r_out,
                    a0: s.start,
                    a1: s.end,
                })
                .collect()
        } else {
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
            let r_hole = radius * inner;
            let band = (radius - r_hole) / rows.len().max(1) as f64;
            rows.iter()
                .enumerate()
                .map(|(i, (pos, name, v))| {
                    let mid = radius - band * (i as f64 + 0.5);
                    PolarItem {
                        key: name.clone(),
                        name: cfg.label(name),
                        color: cfg.color_of(name, *pos),
                        icon: cfg.get(name).and_then(|c| c.icon),
                        value: *v,
                        r_in: mid - band / 2.0,
                        r_out: mid + band / 2.0,
                        a0,
                        a1,
                    }
                })
                .collect()
        };
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
        "Radial bar chart".to_string()
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
            kind="radial"
            vw=width
            vh=height
            title=title
            description=description
            interaction=interaction(model, active)
            table=table
        >
            {move || {
                let g = model.get();
                let top = g.items.iter().map(|i| i.value).fold(0.0f64, f64::max);
                let scale_max = max.unwrap_or(top).max(f64::MIN_POSITIVE);
                let marks: Vec<AnyView> = g
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, it)| {
                        let attrs = move || (active.get() == Some(i)).then_some("");
                        let dim = move || active.get().is_some_and(|a| a != i).then_some("");
                        if stacked {
                            return view! {
                                <path
                                    class="vdl-chart__slice"
                                    d=arc_path(g.cx, g.cy, it.r_in, it.r_out, it.a0, it.a1)
                                    style=format!("fill:{}", it.color)
                                    data-active=attrs
                                    data-dim=dim
                                />
                            }
                                .into_any();
                        }
                        let band = it.r_out - it.r_in;
                        let thickness = band * (1.0 - gap);
                        let mid = (it.r_in + it.r_out) / 2.0;
                        let end = it.a0 + (it.a1 - it.a0) * (it.value / scale_max).clamp(0.0, 1.0);
                        view! {
                            <g>
                                {track
                                    .then(|| view! {
                                        <path
                                            class="vdl-chart__track"
                                            d=arc_stroke_path(g.cx, g.cy, mid, it.a0, it.a1)
                                            stroke-width=n(thickness)
                                        />
                                    })}
                                <path
                                    class="vdl-chart__radial-bar"
                                    d=arc_stroke_path(g.cx, g.cy, mid, it.a0, end)
                                    stroke-width=n(thickness)
                                    style=format!("stroke:{}", it.color)
                                    data-active=attrs
                                    data-dim=dim
                                />
                            </g>
                        }
                            .into_any()
                    })
                    .collect();
                let centre_y = if half { g.cy - 26.0 } else { g.cy };
                view! {
                    <g>{marks}</g>
                    {center_text(g.cx, centre_y, center_value.clone(), center_label.clone())}
                }
            }}
        </ChartFrame>
    }
}
