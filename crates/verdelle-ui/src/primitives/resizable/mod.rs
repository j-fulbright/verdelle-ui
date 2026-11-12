//! Resizable panel groups: panels that share a row or column, separated by draggable,
//! keyboard-operable handles. Verdelle's own (not a Leptix port); the behaviour follows
//! `react-resizable-panels`, which shadcn's Resizable wraps.
//!
//! [`model`] is the pure part (sizes in percent, min and max, collapsing, the effect of a
//! drag or a key press) and is where the tests live. `resizable` holds the components.

pub mod model;
mod resizable;

pub use model::{PanelConstraints, Snap};
pub use resizable::*;
