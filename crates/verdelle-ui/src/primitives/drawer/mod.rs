//! Drawer: the headless decisions behind a swipe-to-dismiss panel.
//!
//! This is Verdelle-original (Leptix has no drawer). The browser wiring (a native `<dialog>`, pointer
//! and touch events, CSS) lives in `verdelle-ui`; everything that *decides* something is here as
//! plain functions and a small state machine with no DOM, so it can be tested: which snap point
//! a release lands on, when a release dismisses, whether a gesture belongs to the drawer or to
//! the scrolling content inside it, how fast the finger was moving, and how far past its limits
//! the panel may be dragged.
//!
//! Every position is in "toward close" coordinates: a positive number is the finger moving the
//! way that closes the drawer (down for a bottom drawer, right for a right one). The caller
//! flips the sign per direction.

mod model;

pub use model::*;
