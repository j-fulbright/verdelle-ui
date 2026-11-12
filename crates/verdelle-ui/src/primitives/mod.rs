//! Headless, accessible UI primitives for Leptos.
//!
//! Ported from Leptix (<https://github.com/RantAI-dev/leptix-ui>, MIT, Copyright (c)
//! 2025 Rantai Dev) and extended. See `NOTICE` and `LICENSES/LEPTIX-MIT.txt`.
//!
//! `core` holds the shared helpers (ids, presence, portals, focus scopes,
//! dismissable layers, popper positioning, controllable state). Each primitive
//! is one module next to it.

// Each primitive keeps Leptix's layout, `toast/toast.rs` and so on, which trips this lint 31 times.
// Renaming every inner module would churn the paths for nothing.
#![allow(clippy::module_inception)]

pub mod core;

// Primitives. Each is ported from the Leptix crate of the same (kebab-case) name.
pub mod accessible_icon;
pub mod accordion;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod calendar;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod data_table;
pub mod dialog;
pub mod drawer;
pub mod dropdown_menu;
pub mod form;
pub mod hover_card;
pub mod label;
pub mod menubar;
pub mod navigation_menu;
pub mod number_field;
pub mod otp_field;
pub mod password_toggle;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod resizable;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod time_field;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;
pub mod tooltip;
pub mod visually_hidden;
