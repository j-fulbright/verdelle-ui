//! Tells an app's own build script where this crate's stylesheets are, whether the crate comes
//! from a path, git or the registry. An app's `build.rs` reads `DEP_VERDELLE_UI_CSS_DIR`.
//!
//! This is not the recommended way to get the CSS into an app: cargo-leptos starts Tailwind before
//! the app's build script has run, so on a clean checkout an `@import` of the copied files fails
//! once. Serve `verdelle_ui::kit_css()` instead. See `docs/new-app.md` and `docs/using-verdelle.md`.

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let css = std::path::Path::new(&manifest_dir).join("css");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::metadata=css_dir={}", css.display());
}
