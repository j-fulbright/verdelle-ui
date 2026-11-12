//! A storybook-style gallery for Verdelle. Run it with `cargo leptos watch` in this folder.

// Leptos view trees nest deeply; the default limit is too low for a full build of the pages.
#![recursion_limit = "512"]

pub mod app;
mod pages;
mod registry;
mod section;
mod server_demo;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
