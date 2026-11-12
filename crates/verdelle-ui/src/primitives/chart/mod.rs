//! The geometry behind Verdelle's charts: scales, tick marks, stacking, path generation (linear, monotone,
//! natural and step curves) and arc maths for pies and radial charts. Plain functions over `f64`: no DOM,
//! no Leptos, so all of it is tested natively. `verdelle_ui`'s chart components draw SVG from these.
//!
//! This is original Verdelle code. shadcn's Chart wraps Recharts (React-only), which follows d3's scale,
//! shape and stack modules; the monotone and natural curves here are the published algorithms d3-shape
//! uses (Fritsch-Carlson monotone interpolation and the natural cubic spline).

pub mod arc;
pub mod curve;
pub mod format;
pub mod scale;
pub mod stack;
pub mod ticks;

pub use arc::{Slice, arc_centroid, arc_path, arc_stroke_path, pie_layout, polar};
pub use curve::{Curve, Point, area_path, line_path, num as svg_num};
pub use format::{format_number, format_tick};
pub use scale::{BandScale, LinearScale, PointScale};
pub use stack::{Segment, StackOffset, stack};
pub use ticks::{NiceScale, nice_scale};
