//! What a cartesian chart draws for the active category: a band behind bars, a line through lines
//! and areas, and a ring on every series. Crate-private.

use crate::chart_cartesian::{Cartesian, CatScale};
use crate::primitives::chart::svg_num as n;
use leptos::prelude::*;

/// The highlight behind (bars) or through (lines, areas) the active category.
pub(crate) fn cursor(m: &Cartesian, i: usize) -> AnyView {
    match &m.cat {
        CatScale::Band(b) => {
            let (low, width) = b.extent(i);
            let step = b.step().abs();
            let c = low + width / 2.0;
            let (x, y, w, h) = if m.horizontal {
                (m.x0, c - step / 2.0, m.x1 - m.x0, step)
            } else {
                (c - step / 2.0, m.y0, step, m.y1 - m.y0)
            };
            view! { <rect class="vdl-chart__cursor-band" x=n(x) y=n(y) width=n(w) height=n(h) /> }
                .into_any()
        }
        CatScale::Point(_) => {
            let c = m.cat_center(i);
            view! { <line class="vdl-chart__cursor-line" x1=n(c) y1=n(m.y0) x2=n(c) y2=n(m.y1) /> }
                .into_any()
        }
    }
}

/// A ring on each series at the active category (lines and areas).
pub(crate) fn active_dots(m: &Cartesian, i: usize) -> AnyView {
    let dots: Vec<AnyView> = m
        .series
        .iter()
        .filter_map(|s| {
            let seg = s.segs.get(i)?;
            let v = *s.values.get(i)?;
            if !v.is_finite() {
                return None;
            }
            let at = if m.stacked { seg.end } else { v };
            let (x, y) = m.point(i, at);
            Some(
                view! {
                    <circle
                        class="vdl-chart__active-dot"
                        cx=n(x)
                        cy=n(y)
                        r="5"
                        style=format!("fill:{}", s.color)
                    />
                }
                .into_any(),
            )
        })
        .collect();
    view! { <g>{dots}</g> }.into_any()
}
