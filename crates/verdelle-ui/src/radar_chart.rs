use crate::chart_common::{
    ChartFrame, Interaction, Nav, client_to_root, client_to_svg, data_table, is_focus_visible,
    nav_for_key, root_element, step_index, svg_of, svg_to_root,
};
use crate::chart_context::{ChartTooltipItem, ChartTooltipState, use_chart};
use crate::primitives::chart::{format_number, nice_scale, polar, svg_num as n};
use crate::{ChartDatum, IconName, RadarGrid};
use leptos::prelude::*;
use std::f64::consts::TAU;

#[derive(Clone, PartialEq)]
struct RSeries {
    key: String,
    name: String,
    color: String,
    icon: Option<IconName>,
    values: Vec<f64>,
}

#[derive(Clone, PartialEq)]
struct Radar {
    vw: f64,
    vh: f64,
    cx: f64,
    cy: f64,
    r: f64,
    cats: Vec<String>,
    series: Vec<RSeries>,
    ticks: Vec<f64>,
    max: f64,
}

impl Radar {
    fn angle(&self, i: usize) -> f64 {
        TAU * i as f64 / self.cats.len().max(1) as f64
    }

    fn point(&self, i: usize, v: f64) -> (f64, f64) {
        polar(
            self.cx,
            self.cy,
            self.r * (v / self.max).clamp(0.0, 1.0),
            self.angle(i),
        )
    }

    fn index_at(&self, sx: f64, sy: f64) -> Option<usize> {
        let n = self.cats.len();
        if n == 0 {
            return None;
        }
        let (dx, dy) = (sx - self.cx, sy - self.cy);
        if dx.hypot(dy) > self.r + 28.0 {
            return None;
        }
        let ang = dx.atan2(-dy).rem_euclid(TAU);
        Some((ang / (TAU / n as f64)).round() as usize % n)
    }

    fn tooltip(&self, i: usize, at: (f64, f64, bool, bool)) -> ChartTooltipState {
        ChartTooltipState {
            label: self.cats[i].clone(),
            items: self
                .series
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
                .collect(),
            x: at.0,
            y: at.1,
            flip_x: at.2,
            flip_y: at.3,
        }
    }
}

/// A radar (spider) chart: the categories of `data` are the axes and every series is a polygon.
///
/// `grid_shape` draws the rings as polygons or circles, `spokes` the axis lines, `filled` and
/// `fill_opacity` the polygon fill (off gives lines only), `dots` a marker at each vertex. `max` fixes
/// the outer ring (default: the largest value, rounded up to a tick). Hover near an axis, or focus the
/// chart and use the arrow keys, to see every series on that axis in the tooltip. Other options
/// are those of [`crate::BarChart`].
#[component]
pub fn RadarChart(
    #[prop(into)] data: Signal<Vec<ChartDatum>>,
    #[prop(optional, into)] series: Option<Vec<String>>,
    #[prop(optional)] grid_shape: RadarGrid,
    #[prop(default = true)] spokes: bool,
    #[prop(default = true)] filled: bool,
    #[prop(default = 0.6)] fill_opacity: f64,
    #[prop(optional)] dots: bool,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(default = 300.0)] width: f64,
    #[prop(default = 300.0)] height: f64,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] description: String,
) -> impl IntoView {
    let ctx = use_chart();
    let keys = series.unwrap_or_else(|| ctx.cfg().keys());
    let model = Memo::new(move |_| {
        let data = data.get();
        let cfg = ctx.cfg();
        let visible = ctx.visible(&keys);
        let series: Vec<RSeries> = keys
            .iter()
            .enumerate()
            .filter(|(_, k)| visible.contains(k))
            .map(|(pos, k)| RSeries {
                key: k.clone(),
                name: cfg.label(k),
                color: cfg.color_of(k, pos),
                icon: cfg.get(k).and_then(|c| c.icon),
                values: data.iter().map(|d| d.get(k).unwrap_or(f64::NAN)).collect(),
            })
            .collect();
        let top = series
            .iter()
            .flat_map(|s| s.values.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0f64, f64::max);
        let ns = nice_scale(0.0, max.unwrap_or(top), 4, max.is_none());
        Radar {
            vw: width,
            vh: height,
            cx: width / 2.0,
            cy: height / 2.0,
            r: (width.min(height) / 2.0 - 36.0).max(10.0),
            cats: data.iter().map(|d| cfg.label(&d.name)).collect(),
            series,
            ticks: ns.ticks.iter().copied().filter(|t| *t > 0.0).collect(),
            max: ns.max.max(f64::MIN_POSITIVE),
        }
    });
    let active = RwSignal::new(None::<usize>);

    let clear = move || {
        active.set(None);
        ctx.tooltip.set(None);
    };
    let go_key = move |svg: leptos::web_sys::Element, nav: Nav| {
        let m = model.get_untracked();
        match step_index(active.get_untracked(), nav, m.cats.len()) {
            None => clear(),
            Some(i) => {
                let top = m
                    .series
                    .iter()
                    .filter_map(|s| s.values.get(i).copied())
                    .filter(|v| v.is_finite())
                    .fold(0.0f64, f64::max);
                let (ax, ay) = m.point(i, top);
                if let Some(root) = root_element() {
                    let at = svg_to_root(&svg, &root, m.vw, m.vh, ax, ay);
                    active.set(Some(i));
                    ctx.tooltip.set(Some(m.tooltip(i, at)));
                }
            }
        }
    };
    let interaction = Interaction {
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
                    ctx.tooltip
                        .set(Some(m.tooltip(i, client_to_root(&root, cx, cy))));
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
    };

    let title = if title.is_empty() {
        "Radar chart".to_string()
    } else {
        title
    };
    let table_title = title.clone();
    let table = (move || {
        let m = model.get();
        let mut headers = vec!["Category".to_string()];
        headers.extend(m.series.iter().map(|s| s.name.clone()));
        let rows = (0..m.cats.len())
            .map(|i| {
                let mut row = vec![m.cats[i].clone()];
                row.extend(m.series.iter().map(|s| match s.values.get(i) {
                    Some(v) if v.is_finite() => format_number(*v),
                    _ => String::new(),
                }));
                row
            })
            .collect();
        data_table(table_title.clone(), headers, rows)
    })
    .into_any();

    view! {
        <ChartFrame
            kind="radar"
            vw=width
            vh=height
            title=title
            description=description
            interaction=interaction
            table=table
        >
            {move || {
                let m = model.get();
                let count = m.cats.len();
                let rings: Vec<AnyView> = m
                    .ticks
                    .iter()
                    .map(|t| {
                        let rr = m.r * (t / m.max);
                        match grid_shape {
                            RadarGrid::Circle => {
                                view! {
                                    <circle
                                        class="vdl-chart__grid"
                                        cx=n(m.cx)
                                        cy=n(m.cy)
                                        r=n(rr)
                                    />
                                }
                                    .into_any()
                            }
                            RadarGrid::Polygon => {
                                let d = (0..count)
                                    .map(|i| {
                                        let (x, y) = polar(m.cx, m.cy, rr, m.angle(i));
                                        format!("{}{},{}", if i == 0 { "M" } else { "L" }, n(x), n(y))
                                    })
                                    .collect::<String>() + "Z";
                                view! { <path class="vdl-chart__grid" d=d /> }.into_any()
                            }
                        }
                    })
                    .collect();
                let axes: Vec<AnyView> = (0..count)
                    .map(|i| {
                        let (x, y) = polar(m.cx, m.cy, m.r, m.angle(i));
                        let (lx, ly) = polar(m.cx, m.cy, m.r + 14.0, m.angle(i));
                        let s = m.angle(i).sin();
                        let c = m.angle(i).cos();
                        let anchor = if s > 0.2 { "start" } else if s < -0.2 { "end" } else { "middle" };
                        let dy = if c > 0.6 { "0" } else if c < -0.6 { "0.8em" } else { "0.32em" };
                        view! {
                            <g>
                                {spokes
                                    .then(|| view! {
                                        <line
                                            class="vdl-chart__grid"
                                            x1=n(m.cx)
                                            y1=n(m.cy)
                                            x2=n(x)
                                            y2=n(y)
                                        />
                                    })}
                                <text
                                    class="vdl-chart__tick"
                                    x=n(lx)
                                    y=n(ly)
                                    dy=dy
                                    text-anchor=anchor
                                >
                                    {m.cats[i].clone()}
                                </text>
                            </g>
                        }
                            .into_any()
                    })
                    .collect();
                let shapes: Vec<AnyView> = m
                    .series
                    .iter()
                    .map(|s| {
                        let pts: Vec<(f64, f64)> = (0..count)
                            .map(|i| m.point(i, s.values.get(i).copied().unwrap_or(0.0)))
                            .collect();
                        let d = pts
                            .iter()
                            .enumerate()
                            .map(|(i, (x, y))| format!("{}{},{}", if i == 0 { "M" } else { "L" }, n(*x), n(*y)))
                            .collect::<String>()
                            + "Z";
                        let fill = if filled {
                            format!("fill:{};fill-opacity:{}", s.color, n(fill_opacity))
                        } else {
                            "fill:none".to_string()
                        };
                        view! {
                            <g>
                                <path
                                    class="vdl-chart__radar"
                                    d=d
                                    style=format!("{fill};stroke:{}", s.color)
                                />
                                {dots
                                    .then(|| {
                                        pts.iter()
                                            .map(|(x, y)| view! {
                                                <circle
                                                    class="vdl-chart__dot"
                                                    cx=n(*x)
                                                    cy=n(*y)
                                                    r="3"
                                                    style=format!("fill:{}", s.color)
                                                />
                                            })
                                            .collect_view()
                                    })}
                            </g>
                        }
                            .into_any()
                    })
                    .collect();
                view! {
                    <g>{rings}</g>
                    <g>{axes}</g>
                    <g>{shapes}</g>
                }
            }}
            {move || {
                active
                    .get()
                    .map(|i| {
                        let m = model.get();
                        let (x, y) = polar(m.cx, m.cy, m.r, m.angle(i));
                        let marks: Vec<AnyView> = m
                            .series
                            .iter()
                            .filter_map(|s| {
                                let v = *s.values.get(i)?;
                                v.is_finite().then(|| {
                                    let (px, py) = m.point(i, v);
                                    view! {
                                        <circle
                                            class="vdl-chart__active-dot"
                                            cx=n(px)
                                            cy=n(py)
                                            r="5"
                                            style=format!("fill:{}", s.color)
                                        />
                                    }
                                        .into_any()
                                })
                            })
                            .collect();
                        view! {
                            <g>
                                <line
                                    class="vdl-chart__cursor-line"
                                    x1=n(m.cx)
                                    y1=n(m.cy)
                                    x2=n(x)
                                    y2=n(y)
                                />
                                {marks}
                            </g>
                        }
                    })
            }}
        </ChartFrame>
    }
}
