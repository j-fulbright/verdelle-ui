//! Ported from leptix-visually_hidden (MIT, Copyright (c) 2025 Rantai Dev; see the crate NOTICE).
//! VisuallyHidden component.
//!
//! Hides content visually while keeping it accessible to screen readers.
//!
//! The upstream `leptix-visually_hidden` crate is a near-duplicate of the one in
//! `leptix-core` (same `span` + bootstrap-style hiding styles; the only difference is
//! that the standalone crate takes optional `ChildrenFn` and sets one `style` string,
//! while core takes `TypedChildrenFn` and uses `style:` props). To avoid two
//! `VisuallyHidden` components, this module re-exports the core one.

pub use crate::primitives::core::visually_hidden::*;
