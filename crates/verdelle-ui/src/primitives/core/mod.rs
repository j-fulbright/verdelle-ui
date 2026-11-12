//! Shared infrastructure for the primitives: hooks, helper components and
//! internal utilities. Ported from `leptix-core` (MIT, see the crate's NOTICE).

/// The arrow that points a floating panel at its anchor.
pub mod arrow;
/// Combine several node refs into one.
pub mod compose_refs;
/// Text direction (`ltr`/`rtl`) plumbing. Kept from Leptix; RTL is out of scope and untested.
pub mod direction;
/// Closes a layer on Escape, a press outside or focus leaving it; layers stack, so only the top one reacts.
pub mod dismissable_layer;
/// Sentinel elements at the edges of the page that keep focus scopes from losing focus to the browser chrome.
pub mod focus_guards;
/// Trap focus inside a container and restore it on unmount.
pub mod focus_scope;
pub mod id;
pub mod listbox;
/// Small numeric helpers (clamp).
pub mod number;
/// Floating-ui positioning: an anchor, a content wrapper placed with `transform`, and an arrow.
pub mod popper;
/// Render children into another element, `<body>` by default.
pub mod portal;
/// Keep an element mounted until its exit animation has finished (`PortalKeep`, `keep_portal_mounted`).
pub mod presence;
/// The element-with-`as_child` building block the primitives render through.
pub mod primitive;
/// Controlled or uncontrolled state for a prop that may or may not be passed.
pub mod use_controllable_state;
/// Run a handler when Escape is pressed anywhere on the page.
pub mod use_escape_keydown;
/// The previous value of a signal.
pub mod use_previous;
/// Observe an element's size with a `ResizeObserver`.
pub mod use_size;
/// Content hidden from sight but still read by assistive technology.
pub mod visually_hidden;
