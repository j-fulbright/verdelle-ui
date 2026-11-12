//! The shared parts of the circular charts (pie, donut, radial bar): the ring segment model, hit
//! testing, hover and keyboard handling, and the text in the middle. Crate-private.

use crate::IconName;
use crate::chart_common::{
    Interaction, Nav, client_to_root, client_to_svg, is_focus_visible, nav_for_key, root_element,
    step_index, svg_of, svg_to_root,
};
use crate::chart_context::{ChartContext, ChartTooltipItem, ChartTooltipState, use_chart};
use crate::primitives::chart::{arc_centroid, svg_num as n};
use leptos::prelude::*;
use std::f64::consts::TAU;

/// One ring segment: a pie slice or a radial bar, with the region that counts as hovering it.
#[derive(Clone, PartialEq)]
pub(crate) struct PolarItem {
    pub key: String,
    pub name: String,
    pub color: String,
    pub icon: Option<IconName>,
    pub value: f64,
    pub r_in: f64,
    pub r_out: f64,
    pub a0: f64,
    pub a1: f64,
}

#[derive(Clone, PartialEq)]
pub(crate) struct PolarGeom {
    pub vw: f64,
    pub vh: f64,
    pub cx: f64,
    pub cy: f64,
    pub items: Vec<PolarItem>,
}

impl PolarGeom {
    /// The item under a point in `viewBox` units.
    pub fn hit(&self, sx: f64, sy: f64) -> Option<usize> {
        let (dx, dy) = (sx - self.cx, sy - self.cy);
        let r = dx.hypot(dy);
        let angle = dx.atan2(-dy);
        self.items.iter().position(|it| {
            if r < it.r_in - 1.0 || r > it.r_out + 1.0 {
                return false;
            }
            let (lo, hi) = (it.a0.min(it.a1), it.a0.max(it.a1));
            let tol = 0.02;
            (-2..=2).any(|k| {
                let a = angle + TAU * k as f64;
                a >= lo - tol && a <= hi + tol
            })
        })
    }

    /// Where a tooltip for item `i` is anchored by keyboard.
    pub fn anchor(&self, i: usize) -> (f64, f64) {
        let it = &self.items[i];
        arc_centroid(self.cx, self.cy, it.r_in, it.r_out, it.a0, it.a1)
    }

    pub fn table_rows(&self) -> Vec<Vec<String>> {
        self.items
            .iter()
            .map(|it| {
                vec![
                    it.name.clone(),
                    crate::primitives::chart::format_number(it.value),
                ]
            })
            .collect()
    }
}

fn show(ctx: ChartContext, g: &PolarGeom, i: usize, at: (f64, f64, bool, bool)) {
    let it = &g.items[i];
    ctx.tooltip.set(Some(ChartTooltipState {
        label: String::new(),
        items: vec![ChartTooltipItem {
            key: it.key.clone(),
            name: it.name.clone(),
            value: it.value,
            color: it.color.clone(),
            icon: it.icon,
        }],
        x: at.0,
        y: at.1,
        flip_x: at.2,
        flip_y: at.3,
    }));
}

/// Pointer and keyboard behaviour for a ring chart: `active` is the item index.
pub(crate) fn interaction(geom: Memo<PolarGeom>, active: RwSignal<Option<usize>>) -> Interaction {
    let ctx = use_chart();
    let clear = move || {
        active.set(None);
        ctx.tooltip.set(None);
    };
    let go_key = move |svg: leptos::web_sys::Element, nav: Nav| {
        let g = geom.get_untracked();
        match step_index(active.get_untracked(), nav, g.items.len()) {
            None => clear(),
            Some(i) => {
                let (ax, ay) = g.anchor(i);
                if let Some(root) = root_element() {
                    let at = svg_to_root(&svg, &root, g.vw, g.vh, ax, ay);
                    active.set(Some(i));
                    show(ctx, &g, i, at);
                }
            }
        }
    };
    Interaction {
        on_move: Some(Callback::new(move |ev: leptos::web_sys::PointerEvent| {
            let (Some(svg), Some(root)) = (svg_of(&ev), root_element()) else {
                return;
            };
            let g = geom.get_untracked();
            let (cx, cy) = (ev.client_x() as f64, ev.client_y() as f64);
            let (sx, sy) = client_to_svg(&svg, g.vw, g.vh, cx, cy);
            match g.hit(sx, sy) {
                Some(i) => {
                    active.set(Some(i));
                    show(ctx, &g, i, client_to_root(&root, cx, cy));
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

/// The text in the middle of a donut or radial chart: a big value and a small caption under it.
pub(crate) fn center_text(
    cx: f64,
    cy: f64,
    value: Option<String>,
    label: Option<String>,
) -> AnyView {
    if value.is_none() && label.is_none() {
        return ().into_any();
    }
    let has_label = label.is_some();
    let value_y = if has_label { cy - 4.0 } else { cy };
    view! {
        <g class="vdl-chart__center" pointer-events="none">
            {value
                .map(|v| view! {
                    <text
                        class="vdl-chart__center-value"
                        x=n(cx)
                        y=n(value_y)
                        text-anchor="middle"
                        dominant-baseline="central"
                    >
                        {v}
                    </text>
                })}
            {label
                .map(|l| view! {
                    <text
                        class="vdl-chart__center-label"
                        x=n(cx)
                        y=n(cy + 22.0)
                        text-anchor="middle"
                        dominant-baseline="central"
                    >
                        {l}
                    </text>
                })}
        </g>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(a0: f64, a1: f64) -> PolarItem {
        PolarItem {
            key: "k".into(),
            name: "K".into(),
            color: "red".into(),
            icon: None,
            value: 1.0,
            r_in: 40.0,
            r_out: 80.0,
            a0,
            a1,
        }
    }

    #[test]
    fn hit_testing_follows_angle_and_radius() {
        let g = PolarGeom {
            vw: 200.0,
            vh: 200.0,
            cx: 100.0,
            cy: 100.0,
            items: vec![
                item(0.0, std::f64::consts::PI),
                item(std::f64::consts::PI, TAU),
            ],
        };
        // Right of the centre at radius 60: the first (clockwise from 12 to 6 o'clock) item.
        assert_eq!(g.hit(160.0, 100.0), Some(0));
        // Left of the centre: the second.
        assert_eq!(g.hit(40.0, 100.0), Some(1));
        // In the hole and outside the ring: nothing.
        assert_eq!(g.hit(100.0, 100.0), None);
        assert_eq!(g.hit(100.0, 5.0), None);
    }

    #[test]
    fn hit_testing_handles_a_sweep_that_starts_negative() {
        let g = PolarGeom {
            vw: 200.0,
            vh: 200.0,
            cx: 100.0,
            cy: 100.0,
            items: vec![item(
                -std::f64::consts::FRAC_PI_2,
                std::f64::consts::FRAC_PI_2,
            )],
        };
        // Straight up is inside -90..90 degrees; straight down is not.
        assert_eq!(g.hit(100.0, 40.0), Some(0));
        assert_eq!(g.hit(100.0, 160.0), None);
        assert_eq!(g.hit(40.0, 100.0), Some(0));
    }
}
