//! Verdelle-original (Leptix has no command palette): a search input over a list of actions,
//! after cmdk. Built on `core::listbox`; the list is always visible (put it in a dialog or a
//! popover to make a palette).

mod command;
pub use command::*;
