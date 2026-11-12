//! Themes an app's users make in the browser: a name plus a preset code and/or pasted shadcn
//! CSS, kept as data that can be stored, listed, renamed, deleted, exported and imported.
//!
//! This file is the pure part (no browser): the [`CustomTheme`] record, how it is built, the
//! text export/import format, and the [`ThemeStorage`] adapter an app can swap. The
//! components that use it are [`crate::ThemeProvider`] and [`crate::ThemeSwitcher`].

use crate::preset::{MenuAccent, MenuColor, Preset};
use crate::theme_css::{ThemeTokens, parse_theme_css, scoped_css, slugify};
use crate::themes::THEMES;
use leptos::prelude::ArcReadSignal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// The theme name the live preview uses: it is the one that is applied while a person is still
/// trying a preset code or pasted CSS, before saving it under a name.
pub const PREVIEW_SLUG: &str = "preview";

/// The `data-theme` name a [`crate::ThemeProvider`]'s `preset` prop is registered under.
pub const APP_PRESET_SLUG: &str = "preset";

/// Why a theme could not be built or imported.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomThemeError(pub String);

impl fmt::Display for CustomThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CustomThemeError {}

/// A theme made by the app's user.
///
/// `code` and `css` are the inputs; `rules` is what they build to (the scoped CSS that is
/// injected) and `menu_color` / `menu_accent` the preset's menu choices, kept so the page can
/// be styled before the app's code has run (the no-flash script reads them).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomTheme {
    /// The `data-theme` value. Stable: renaming changes only `name`.
    pub slug: String,
    /// What the theme switcher shows.
    pub name: String,
    /// A shadcn preset code (`b2oE5cW2a`), when the theme is built from one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Pasted shadcn-format CSS (`:root` and `.dark` blocks). Laid over the preset's tokens
    /// when both are given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    /// The scoped CSS built from `code` and `css`.
    #[serde(default)]
    pub rules: String,
    /// `data-vdl-menu-color` for this theme (a preset's menu colour), if it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_color: Option<String>,
    /// `data-vdl-menu-accent` for this theme, if it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu_accent: Option<String>,
}

impl CustomTheme {
    /// Builds a theme from a preset code and/or pasted CSS. The slug is made from `name`
    /// ([`slugify`]); use [`CustomTheme::with_slug`] to choose it.
    ///
    /// With a code the tokens start from the preset (base colour, accent, chart colour,
    /// radius, fonts), and pasted CSS then overrides individual tokens. With CSS alone only
    /// the pasted tokens are written and everything else keeps the default theme's values.
    /// The preset's style and icon library are not rendered (see [`Preset::supported`]); that
    /// never fails the build.
    pub fn build(
        name: &str,
        code: Option<&str>,
        css: Option<&str>,
    ) -> Result<CustomTheme, CustomThemeError> {
        Self::with_slug(&slugify(name), name, code, css)
    }

    /// [`CustomTheme::build`] with the slug given. The slug is cleaned with [`slugify`].
    pub fn with_slug(
        slug: &str,
        name: &str,
        code: Option<&str>,
        css: Option<&str>,
    ) -> Result<CustomTheme, CustomThemeError> {
        let slug = slugify(slug);
        if slug.is_empty() {
            return Err(CustomThemeError("give the theme a name".into()));
        }
        let code = code.map(str::trim).filter(|c| !c.is_empty());
        let css = css.map(str::trim).filter(|c| !c.is_empty());
        if code.is_none() && css.is_none() {
            return Err(CustomThemeError(
                "give a preset code, shadcn CSS, or both".into(),
            ));
        }

        let preset = match code {
            Some(c) => Some(Preset::parse_input(c).map_err(|e| CustomThemeError(e.to_string()))?),
            None => None,
        };
        let mut tokens = match &preset {
            Some(p) => p.tokens_with_fonts(),
            None => ThemeTokens::default(),
        };
        if let Some(css) = css {
            let parsed = parse_theme_css(css).map_err(|e| CustomThemeError(e.to_string()))?;
            tokens.overlay(&parsed);
        }
        let mut rules = scoped_css(&slug, &tokens.light, &tokens.dark);
        if let Some(p) = &preset {
            // The font rules of `Preset::scoped_css`, after the tokens.
            let full = p.scoped_css(&slug);
            if let Some(at) = full.find(&format!("[data-theme=\"{slug}\"] body")) {
                rules.push_str(&full[at..]);
            }
        }
        Ok(CustomTheme {
            rules,
            menu_color: preset.map(|p| p.menu_color.slug().to_string()),
            menu_accent: preset.map(|p| p.menu_accent.slug().to_string()),
            slug,
            name: name.trim().to_string(),
            // Keep what was typed, normalised to the bare code.
            code: preset.map(|p| p.encode()),
            css: css.map(str::to_string),
        })
    }

    /// The preset this theme was made from, if any.
    pub fn preset(&self) -> Option<Preset> {
        self.code.as_deref().and_then(|c| Preset::decode(c).ok())
    }

    /// Whether `slug` belongs to one of Verdelle's built-in themes or is reserved.
    pub fn is_reserved(slug: &str) -> bool {
        slug == PREVIEW_SLUG || slug == APP_PRESET_SLUG || THEMES.iter().any(|t| t.slug == slug)
    }

    /// A slug for `name` that no built-in theme and none of `taken` uses: the plain slug when
    /// free, else `name-2`, `name-3`, ...
    pub fn unique_slug<'a>(name: &str, taken: impl IntoIterator<Item = &'a str>) -> String {
        let taken: Vec<&str> = taken.into_iter().collect();
        let base = match slugify(name) {
            s if s.is_empty() => "theme".to_string(),
            s => s,
        };
        let free = |s: &str| !Self::is_reserved(s) && !taken.contains(&s);
        if free(&base) {
            return base;
        }
        (2..)
            .map(|n| format!("{base}-{n}"))
            .find(|s| free(s))
            .unwrap_or(base)
    }

    /// Rebuilds `rules` and the menu fields from `code` and `css` (after importing, or when
    /// Verdelle's composition changed). Keeps the slug and name.
    pub fn rebuilt(&self) -> Result<CustomTheme, CustomThemeError> {
        Self::with_slug(
            &self.slug,
            &self.name,
            self.code.as_deref(),
            self.css.as_deref(),
        )
    }

    /// The menu colour as the typed value, if valid.
    pub fn menu_color_value(&self) -> Option<MenuColor> {
        self.menu_color.as_deref().and_then(MenuColor::from_slug)
    }

    /// The menu accent as the typed value, if valid.
    pub fn menu_accent_value(&self) -> Option<MenuAccent> {
        self.menu_accent.as_deref().and_then(MenuAccent::from_slug)
    }
}

/// The text form themes are exported as and imported from: a small JSON document holding each
/// theme's name, preset code and pasted CSS (not the built rules, so an export keeps working
/// when Verdelle's composition improves).
#[derive(Serialize, Deserialize)]
struct Export {
    verdelle_themes: u32,
    themes: Vec<ExportTheme>,
}

#[derive(Serialize, Deserialize)]
struct ExportTheme {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    css: Option<String>,
}

/// Writes themes as text, to copy or save.
pub fn export_themes(themes: &[CustomTheme]) -> String {
    let export = Export {
        verdelle_themes: 1,
        themes: themes
            .iter()
            .map(|t| ExportTheme {
                name: t.name.clone(),
                code: t.code.clone(),
                css: t.css.clone(),
            })
            .collect(),
    };
    serde_json::to_string_pretty(&export).unwrap_or_default()
}

/// Reads text made by [`export_themes`]. For convenience it also takes a single preset code
/// or a block of shadcn CSS, which becomes one theme named `fallback_name`. Slugs are made
/// unique against `taken` and the built-in themes, and every theme is built, so a bad entry
/// fails the whole import (nothing is half-added).
pub fn import_themes<'a>(
    text: &str,
    fallback_name: &str,
    taken: impl IntoIterator<Item = &'a str>,
) -> Result<Vec<CustomTheme>, CustomThemeError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(CustomThemeError("nothing to import".into()));
    }
    let entries: Vec<ExportTheme> = if text.starts_with('{') {
        let export: Export = serde_json::from_str(text)
            .map_err(|e| CustomThemeError(format!("not a Verdelle theme export: {e}")))?;
        export.themes
    } else if Preset::is_code(text) || text.starts_with("--preset") {
        vec![ExportTheme {
            name: fallback_name.to_string(),
            code: Some(text.to_string()),
            css: None,
        }]
    } else {
        vec![ExportTheme {
            name: fallback_name.to_string(),
            code: None,
            css: Some(text.to_string()),
        }]
    };

    let mut used: Vec<String> = taken.into_iter().map(str::to_string).collect();
    let mut out = Vec::new();
    for e in entries {
        let slug = CustomTheme::unique_slug(&e.name, used.iter().map(String::as_str));
        let theme = CustomTheme::with_slug(&slug, &e.name, e.code.as_deref(), e.css.as_deref())
            .map_err(|err| CustomThemeError(format!("{}: {err}", e.name)))?;
        used.push(slug);
        out.push(theme);
    }
    Ok(out)
}

/// Checks that adding `adding` themes to the `existing` ones stays within `max` (no limit when
/// `max` is `None`). The error says how many may still be saved.
pub fn check_limit(
    existing: usize,
    adding: usize,
    max: Option<usize>,
) -> Result<(), CustomThemeError> {
    let Some(max) = max else { return Ok(()) };
    if existing + adding <= max {
        return Ok(());
    }
    let room = max.saturating_sub(existing);
    Err(CustomThemeError(match room {
        0 => format!("you can keep {max} saved theme(s); delete one first"),
        n => format!("you can keep {max} saved theme(s); there is room for {n} more"),
    }))
}

/// Where the theme provider keeps what a person chose and made. The default,
/// [`LocalStorage`], is the browser's `localStorage`; give [`crate::ThemeProvider`] another
/// one (say, one that loads the values with the page from a server and writes changes back
/// through a server function) with its `storage` prop.
///
/// Reads are synchronous, so an adapter backed by a server must have its values in hand when
/// the provider is created. Methods are called from the browser only, never during SSR.
/// Values are short strings: the theme name and mode, and JSON for the lists.
///
/// An adapter whose values can change from outside (a server's answer arriving after
/// hydration) says so through [`ThemeStorage::changes`], and one that can be read while
/// rendering on the server gives its values through [`ThemeStorage::seed`]; see
/// [`crate::SyncedStorage`], which does both.
pub trait ThemeStorage: Send + Sync + 'static {
    /// The stored value of `key`.
    fn get(&self, key: &str) -> Option<String>;
    /// Stores `value` under `key`.
    fn set(&self, key: &str, value: &str);
    /// Forgets `key`.
    fn remove(&self, key: &str);
    /// A counter that goes up whenever the stored values were replaced from outside (not by
    /// this provider's own `set` and `remove`). The provider reads the storage again when it
    /// changes. `None` (the default) means the values never change behind its back.
    fn changes(&self) -> Option<ArcReadSignal<u64>> {
        None
    }
    /// All the values, when they can be read during server rendering too. The provider then
    /// starts from them (so the server's HTML already has the person's theme and saved
    /// themes) and prints a no-flash script that uses them instead of `localStorage`.
    /// `None` (the default) is for adapters that only work in the browser.
    fn seed(&self) -> Option<BTreeMap<String, String>> {
        None
    }
}

/// The browser's `localStorage`. Every call is a no-op or `None` where storage is missing or
/// blocked (private windows, server rendering).
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalStorage;

impl LocalStorage {
    #[cfg(feature = "hydrate")]
    fn storage() -> Option<web_sys::Storage> {
        leptos_use_storage::StorageType::Local
            .into_storage()
            .ok()
            .flatten()
    }
}

impl ThemeStorage for LocalStorage {
    fn get(&self, key: &str) -> Option<String> {
        #[cfg(feature = "hydrate")]
        {
            Self::storage()?.get_item(key).ok().flatten()
        }
        #[cfg(not(feature = "hydrate"))]
        {
            let _ = key;
            None
        }
    }

    fn set(&self, key: &str, value: &str) {
        #[cfg(feature = "hydrate")]
        if let Some(s) = Self::storage() {
            let _ = s.set_item(key, value);
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = (key, value);
    }

    fn remove(&self, key: &str) {
        #[cfg(feature = "hydrate")]
        if let Some(s) = Self::storage() {
            let _ = s.remove_item(key);
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = key;
    }
}

/// An in-memory [`ThemeStorage`], for tests and for apps that want nothing persisted.
#[derive(Debug, Default)]
pub struct MemoryStorage(std::sync::Mutex<std::collections::BTreeMap<String, String>>);

impl ThemeStorage for MemoryStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.0.lock().ok()?.get(key).cloned()
    }
    fn set(&self, key: &str, value: &str) {
        if let Ok(mut m) = self.0.lock() {
            m.insert(key.to_string(), value.to_string());
        }
    }
    fn remove(&self, key: &str) {
        if let Ok(mut m) = self.0.lock() {
            m.remove(key);
        }
    }
}

/// The storage keys the provider uses (and the no-flash script reads).
pub mod keys {
    /// The chosen theme's slug.
    pub const THEME: &str = "verdelle-theme";
    /// `light`, `dark` or `system`.
    pub const MODE: &str = "verdelle-mode";
    /// JSON array of saved [`super::CustomTheme`]s.
    pub const CUSTOM: &str = "verdelle-custom-themes";
    /// JSON of the unsaved preview [`super::CustomTheme`].
    pub const PREVIEW: &str = "verdelle-preview-theme";
}

/// Reads the saved themes from storage. Entries that no longer build are dropped.
pub fn load_custom(storage: &dyn ThemeStorage) -> Vec<CustomTheme> {
    storage
        .get(keys::CUSTOM)
        .and_then(|json| serde_json::from_str::<Vec<CustomTheme>>(&json).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|t| !CustomTheme::is_reserved(&t.slug) && !t.slug.is_empty())
        .collect()
}

/// Writes the saved themes.
pub fn save_custom(storage: &dyn ThemeStorage, themes: &[CustomTheme]) {
    if themes.is_empty() {
        storage.remove(keys::CUSTOM);
    } else if let Ok(json) = serde_json::to_string(themes) {
        storage.set(keys::CUSTOM, &json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::{BaseColor, Theme};

    const ROSE_CSS: &str = r#"
        :root { --primary: oklch(0.6 0.2 10); --radius: 1rem; }
        .dark { --primary: oklch(0.4 0.2 10); }
    "#;

    #[test]
    fn builds_from_css_alone() {
        let t = CustomTheme::build("My Rose", None, Some(ROSE_CSS)).unwrap();
        assert_eq!(t.slug, "my-rose");
        assert_eq!(t.name, "My Rose");
        assert!(
            t.rules
                .contains("[data-theme=\"my-rose\"]{--primary:oklch(0.6 0.2 10);--radius:1rem;}")
        );
        assert!(t.rules.contains(".dark[data-theme=\"my-rose\"]"));
        // Only the pasted tokens are written: the rest comes from the default theme.
        assert!(!t.rules.contains("--background"));
        assert_eq!(t.code, None);
        assert_eq!(t.menu_color, None);
    }

    #[test]
    fn builds_from_a_code_and_css_overrides_it() {
        let code = Preset {
            theme: Theme::Rose,
            base_color: BaseColor::Zinc,
            menu_accent: MenuAccent::Bold,
            ..Preset::default()
        }
        .encode();
        let t = CustomTheme::build("Brand", Some(&code), Some(ROSE_CSS)).unwrap();
        assert_eq!(t.code.as_deref(), Some(code.as_str()));
        // A full token set, with the pasted primary on top.
        assert!(t.rules.contains("--background:"));
        assert!(t.rules.contains("--primary:oklch(0.6 0.2 10)"));
        assert!(t.rules.contains("--radius:1rem"));
        assert!(t.rules.contains("--font-sans:"));
        assert_eq!(t.menu_accent.as_deref(), Some("bold"));
        assert_eq!(t.menu_accent_value(), Some(MenuAccent::Bold));
        assert_eq!(t.menu_color_value(), Some(MenuColor::Default));
        assert_eq!(t.preset().unwrap().theme, Theme::Rose);
    }

    #[test]
    fn accepts_what_people_paste_for_a_code() {
        let t = CustomTheme::build("A", Some("--preset b2oE5cW2a"), None).unwrap();
        assert_eq!(t.code.as_deref(), Some("b2oE5cW2a"));
    }

    #[test]
    fn rejects_what_cannot_be_built() {
        assert!(CustomTheme::build("", Some("b0"), None).is_err());
        assert!(CustomTheme::build("x", None, None).is_err());
        assert!(CustomTheme::build("x", Some("   "), Some(" ")).is_err());
        assert!(CustomTheme::build("x", Some("zzz"), None).is_err());
        assert!(CustomTheme::build("x", None, Some("body { color: red }")).is_err());
    }

    #[test]
    fn unsupported_style_and_icons_do_not_fail() {
        // The docs' code is nova; the same preset in the vega style is not rendered as named.
        let vega = Preset {
            style: crate::preset::Style::Vega,
            ..Preset::decode("b2oE5cW2a").unwrap()
        };
        let code = vega.encode();
        assert!(!Preset::decode(&code).unwrap().supported().is_complete());
        assert!(CustomTheme::build("x", Some(&code), None).is_ok());
    }

    #[test]
    fn slugs_stay_unique_and_off_the_built_ins() {
        assert_eq!(CustomTheme::unique_slug("Brand", []), "brand");
        assert_eq!(CustomTheme::unique_slug("Brand", ["brand"]), "brand-2");
        assert_eq!(
            CustomTheme::unique_slug("Brand", ["brand", "brand-2"]),
            "brand-3"
        );
        // A built-in name or the preview name is never handed out.
        assert_eq!(CustomTheme::unique_slug("Rose", []), "rose-2");
        assert_eq!(CustomTheme::unique_slug("Preview", []), "preview-2");
        assert_eq!(CustomTheme::unique_slug("Preset", []), "preset-2");
        assert_eq!(CustomTheme::unique_slug("!!!", []), "theme");
    }

    #[test]
    fn export_then_import_gives_the_same_themes() {
        let a = CustomTheme::build("Brand", Some("b2oE5cW2a"), None).unwrap();
        let b = CustomTheme::build("Pasted", None, Some(ROSE_CSS)).unwrap();
        let text = export_themes(&[a.clone(), b.clone()]);
        // The export holds inputs, not built rules.
        assert!(!text.contains("data-theme"));
        let back = import_themes(&text, "Imported", []).unwrap();
        assert_eq!(back, vec![a, b]);
    }

    #[test]
    fn import_renames_on_collision_and_takes_loose_input() {
        let a = CustomTheme::build("Brand", Some("b0"), None).unwrap();
        let text = export_themes(&[a]);
        let back = import_themes(&text, "Imported", ["brand"]).unwrap();
        assert_eq!(back[0].slug, "brand-2");
        assert_eq!(back[0].name, "Brand");

        let one = import_themes("b2oE5cW2a", "From code", []).unwrap();
        assert_eq!(
            (one[0].name.as_str(), one[0].code.as_deref()),
            ("From code", Some("b2oE5cW2a"))
        );
        let css = import_themes(ROSE_CSS, "From css", []).unwrap();
        assert!(css[0].css.is_some() && css[0].code.is_none());
    }

    #[test]
    fn a_bad_entry_fails_the_whole_import() {
        let text = r#"{"verdelle_themes":1,"themes":[{"name":"ok","code":"b0"},{"name":"bad","code":"nope"}]}"#;
        let err = import_themes(text, "x", []).unwrap_err();
        assert!(err.0.starts_with("bad:"), "{err}");
        assert!(import_themes("{ not json", "x", []).is_err());
        assert!(import_themes("  ", "x", []).is_err());
    }

    #[test]
    fn the_saved_limit_counts_what_is_added() {
        assert!(check_limit(5, 5, None).is_ok());
        assert!(check_limit(2, 1, Some(3)).is_ok());
        let full = check_limit(3, 1, Some(3)).unwrap_err();
        assert!(full.0.contains("delete one"), "{full}");
        let some = check_limit(1, 4, Some(3)).unwrap_err();
        assert!(some.0.contains("room for 2"), "{some}");
    }

    #[test]
    fn storage_round_trips_and_drops_reserved_slugs() {
        let store = MemoryStorage::default();
        assert!(load_custom(&store).is_empty());
        let a = CustomTheme::build("Brand", Some("b0"), None).unwrap();
        save_custom(&store, std::slice::from_ref(&a));
        assert_eq!(load_custom(&store), vec![a.clone()]);
        // A hand-edited entry that shadows a built-in theme is ignored.
        let mut sneaky = a.clone();
        sneaky.slug = "rose".into();
        save_custom(&store, &[sneaky, a.clone()]);
        assert_eq!(load_custom(&store), vec![a]);
        save_custom(&store, &[]);
        assert_eq!(store.get(keys::CUSTOM), None);
        store.set(keys::CUSTOM, "not json");
        assert!(load_custom(&store).is_empty());
    }
}
