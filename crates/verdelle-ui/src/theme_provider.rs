use crate::custom_theme::{
    APP_PRESET_SLUG, CustomTheme, CustomThemeError, LocalStorage, PREVIEW_SLUG, ThemeStorage,
    check_limit, export_themes, import_themes, keys, load_custom, save_custom,
};
use crate::preset::Preset;
use crate::themes::{DEFAULT_THEME, THEMES};
use leptos::context::Provider;
use leptos::prelude::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Light, dark, or whatever the operating system is set to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

impl ThemeMode {
    pub const ALL: [ThemeMode; 3] = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::System];

    pub fn slug(self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
            ThemeMode::System => "system",
        }
    }

    pub fn from_slug(slug: &str) -> Option<ThemeMode> {
        ThemeMode::ALL.into_iter().find(|m| m.slug() == slug)
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
            ThemeMode::System => "System",
        }
    }
}

/// Where a theme in the list came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeSource {
    /// One of Verdelle's own ([`THEMES`]).
    BuiltIn,
    /// Declared in the page with a [`crate::ThemeStyle`].
    Registered,
    /// The provider's `preset` prop.
    AppPreset,
    /// Saved by the person using the app.
    Saved,
    /// The unsaved preview.
    Preview,
}

/// One entry in the list of selectable themes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeChoice {
    pub slug: String,
    pub name: String,
    pub source: ThemeSource,
}

/// What [`ThemeProvider`] gives to everything below it: the chosen theme and mode, the themes
/// people made, and the functions to change them. Get it with [`use_theme`].
///
/// It is `Copy`; the signals are shared. Setting `theme` or `mode` is enough to change the
/// page: the provider applies it to `<html>` and stores it.
#[derive(Clone, Copy)]
pub struct ThemeContext {
    /// The chosen theme's slug: `data-theme` on `<html>`.
    pub theme: RwSignal<String>,
    /// Light, dark or system.
    pub mode: RwSignal<ThemeMode>,
    /// Whether the page is dark now (the mode, with `System` resolved).
    pub dark: Signal<bool>,
    custom: RwSignal<Vec<CustomTheme>>,
    preview: RwSignal<Option<CustomTheme>>,
    registered: RwSignal<Vec<ThemeChoice>>,
    app: Memo<Option<CustomTheme>>,
    storage: StoredValue<Arc<dyn ThemeStorage>>,
    default_theme: StoredValue<String>,
    #[cfg(feature = "hydrate")]
    default_mode: ThemeMode,
}

/// The theme context of the nearest [`ThemeProvider`].
///
/// # Panics
/// When there is no provider above.
pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}

impl ThemeContext {
    /// The theme context, if there is a provider above.
    pub fn try_use() -> Option<ThemeContext> {
        use_context::<ThemeContext>()
    }

    /// Flips between light and dark (from what the page shows now, so `System` becomes the
    /// opposite of the current look and stays explicit).
    pub fn toggle_mode(&self) {
        let dark = self.dark.get_untracked();
        self.mode.set(if dark {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        });
    }

    /// Chooses a theme by slug.
    pub fn choose(&self, slug: impl Into<String>) {
        self.theme.set(slug.into());
    }

    /// Every theme that can be chosen: the built-in ones, those declared with
    /// [`crate::ThemeStyle`], the `preset` prop, the saved ones and the preview. Reactive.
    pub fn choices(&self) -> Vec<ThemeChoice> {
        let mut out: Vec<ThemeChoice> = THEMES
            .iter()
            .map(|t| ThemeChoice {
                slug: t.slug.to_string(),
                name: t.name.to_string(),
                source: ThemeSource::BuiltIn,
            })
            .collect();
        out.extend(self.registered.get());
        let entry = |t: &CustomTheme, source| ThemeChoice {
            slug: t.slug.clone(),
            name: t.name.clone(),
            source,
        };
        if let Some(app) = self.app.get() {
            out.push(entry(&app, ThemeSource::AppPreset));
        }
        out.extend(
            self.custom
                .get()
                .iter()
                .map(|t| entry(t, ThemeSource::Saved)),
        );
        if let Some(p) = self.preview.get() {
            out.push(entry(&p, ThemeSource::Preview));
        }
        out
    }

    /// The saved themes. Reactive.
    pub fn saved(&self) -> Vec<CustomTheme> {
        self.custom.get()
    }

    /// The unsaved preview, if one is being tried. Reactive.
    pub fn preview(&self) -> Option<CustomTheme> {
        self.preview.get()
    }

    /// The chosen theme when it is one the provider owns (the preview, a saved theme or the
    /// `preset` prop): what its preset code and pasted CSS were. `None` for built-in and
    /// `ThemeStyle` themes. Reactive.
    pub fn active_custom(&self) -> Option<CustomTheme> {
        let slug = self.theme.get();
        if slug == PREVIEW_SLUG {
            return self.preview.get();
        }
        if let Some(t) = self.custom.get().into_iter().find(|t| t.slug == slug) {
            return Some(t);
        }
        if slug == APP_PRESET_SLUG {
            return self.app.get();
        }
        None
    }

    /// Tries a preset code and/or pasted shadcn CSS: builds it, applies it as the page's
    /// theme (named `preview`) and remembers it across reloads, without saving it under a
    /// name. On error nothing changes. See [`CustomTheme::build`] for how the two combine.
    pub fn try_theme(&self, code: Option<&str>, css: Option<&str>) -> Result<(), CustomThemeError> {
        let built = CustomTheme::with_slug(PREVIEW_SLUG, "Preview", code, css)?;
        self.set_preview(Some(built));
        self.theme.set(PREVIEW_SLUG.to_string());
        Ok(())
    }

    /// Applies a [`Preset`] as the preview.
    pub fn try_preset(&self, preset: &Preset) -> Result<(), CustomThemeError> {
        self.try_theme(Some(&preset.encode()), None)
    }

    /// Drops the preview. If it was chosen, goes back to the default theme.
    pub fn clear_preview(&self) {
        self.set_preview(None);
        if self.theme.get_untracked() == PREVIEW_SLUG {
            self.theme.set(self.default_theme.get_value());
        }
    }

    /// Saves a theme under `name` and chooses it; returns its slug. The slug is made from the
    /// name and kept unique. The theme is stored through the [`ThemeStorage`].
    pub fn save_theme(
        &self,
        name: &str,
        code: Option<&str>,
        css: Option<&str>,
    ) -> Result<String, CustomThemeError> {
        let slug = self.free_slug(name);
        let theme = CustomTheme::with_slug(&slug, name, code, css)?;
        self.custom.update(|list| list.push(theme));
        self.persist_custom();
        self.theme.set(slug.clone());
        // What was previewed is now saved under a name.
        self.set_preview(None);
        Ok(slug)
    }

    /// Saves the preview under `name` (see [`Self::save_theme`]).
    pub fn save_preview(&self, name: &str) -> Result<String, CustomThemeError> {
        let preview = self
            .preview
            .get_untracked()
            .ok_or_else(|| CustomThemeError("there is no preview to save".into()))?;
        self.save_theme(name, preview.code.as_deref(), preview.css.as_deref())
    }

    /// Renames a saved theme (its slug stays).
    pub fn rename_theme(&self, slug: &str, name: &str) -> Result<(), CustomThemeError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(CustomThemeError("give the theme a name".into()));
        }
        let mut found = false;
        self.custom.update(|list| {
            if let Some(t) = list.iter_mut().find(|t| t.slug == slug) {
                t.name = name.to_string();
                found = true;
            }
        });
        if !found {
            return Err(CustomThemeError(format!("no saved theme `{slug}`")));
        }
        self.persist_custom();
        Ok(())
    }

    /// Deletes a saved theme. If it was chosen, goes back to the default theme.
    pub fn delete_theme(&self, slug: &str) {
        self.custom.update(|list| list.retain(|t| t.slug != slug));
        self.persist_custom();
        if self.theme.get_untracked() == slug {
            self.theme.set(self.default_theme.get_value());
        }
    }

    /// The saved themes as text (JSON of names, preset codes and pasted CSS), to copy or keep.
    pub fn export_themes(&self) -> String {
        export_themes(&self.custom.get_untracked())
    }

    /// Adds themes from text made by [`Self::export_themes`] (or a single preset code, or one
    /// block of shadcn CSS, named `fallback_name`). Names that are taken get a number added.
    /// All or nothing. Returns how many were added.
    pub fn import_themes(
        &self,
        text: &str,
        fallback_name: &str,
    ) -> Result<usize, CustomThemeError> {
        self.import_themes_max(text, fallback_name, None)
    }

    /// [`Self::import_themes`] that fails, adding nothing, when the saved list would hold more
    /// than `max` themes afterwards (no limit when `None`).
    pub fn import_themes_max(
        &self,
        text: &str,
        fallback_name: &str,
        max: Option<usize>,
    ) -> Result<usize, CustomThemeError> {
        let taken: Vec<String> = self.taken_slugs();
        let themes = import_themes(text, fallback_name, taken.iter().map(String::as_str))?;
        check_limit(self.custom.get_untracked().len(), themes.len(), max)?;
        let n = themes.len();
        self.custom.update(|list| list.extend(themes));
        self.persist_custom();
        Ok(n)
    }

    /// Lists a theme declared in the page so the switcher shows it. [`crate::ThemeStyle`] does
    /// this; call it yourself only for themes whose CSS you load another way.
    pub fn register(&self, choice: ThemeChoice) {
        self.registered.update(
            |list| match list.iter_mut().find(|c| c.slug == choice.slug) {
                Some(slot) => *slot = choice,
                None => list.push(choice),
            },
        );
    }

    /// Takes a theme back out of the list.
    pub fn unregister(&self, slug: &str) {
        self.registered
            .update(|list| list.retain(|c| c.slug != slug));
    }

    fn taken_slugs(&self) -> Vec<String> {
        let mut taken: Vec<String> = self
            .custom
            .get_untracked()
            .iter()
            .map(|t| t.slug.clone())
            .collect();
        taken.extend(
            self.registered
                .get_untracked()
                .iter()
                .map(|c| c.slug.clone()),
        );
        taken.push(APP_PRESET_SLUG.to_string());
        taken
    }

    fn free_slug(&self, name: &str) -> String {
        let taken = self.taken_slugs();
        CustomTheme::unique_slug(name, taken.iter().map(String::as_str))
    }

    fn persist_custom(&self) {
        save_custom(&*self.storage.get_value(), &self.custom.get_untracked());
    }

    fn set_preview(&self, preview: Option<CustomTheme>) {
        let storage = self.storage.get_value();
        match &preview {
            Some(p) => {
                if let Ok(json) = serde_json::to_string(p) {
                    storage.set(keys::PREVIEW, &json);
                }
            }
            None => storage.remove(keys::PREVIEW),
        }
        self.preview.set(preview);
    }

    /// The scoped CSS of everything the provider owns: the `preset` prop, the saved themes and
    /// the preview.
    fn rules(&self) -> String {
        let mut css = String::new();
        if let Some(app) = self.app.get() {
            css.push_str(&app.rules);
        }
        for t in self.custom.get() {
            css.push_str(&t.rules);
        }
        if let Some(p) = self.preview.get() {
            css.push_str(&p.rules);
        }
        css
    }

    /// The menu colour and accent of the chosen theme, when it comes from a preset.
    #[cfg(feature = "hydrate")]
    fn menu(&self) -> (Option<String>, Option<String>) {
        let slug = self.theme.get();
        let from = |t: &CustomTheme| (t.menu_color.clone(), t.menu_accent.clone());
        if slug == PREVIEW_SLUG {
            return self.preview.get().map(|t| from(&t)).unwrap_or_default();
        }
        if let Some(t) = self.custom.get().iter().find(|t| t.slug == slug) {
            return from(t);
        }
        if slug == APP_PRESET_SLUG
            && let Some(t) = self.app.get()
        {
            return from(&t);
        }
        (None, None)
    }
}

/// The inline script that puts the saved theme, mode, saved-theme CSS and menu attributes on
/// `<html>` before the first paint, so a reload does not flash the default look.
/// [`ThemeProvider`] writes it itself; if you would rather have it in `<head>` (earlier, and
/// the only place that avoids a flash for content above the provider), pass `script=false` to
/// the provider and print this where your document shell writes its `<head>`.
///
/// It reads the same keys the provider writes ([`keys`]) from `localStorage`, so it is only
/// right when the provider uses the default [`LocalStorage`]. The `default_*` values are what
/// applies when nothing is stored.
pub fn theme_init_script(
    default_theme: &str,
    default_mode: ThemeMode,
    default_menu: (Option<&str>, Option<&str>),
) -> String {
    init_script(default_theme, default_mode, default_menu, None)
}

/// [`theme_init_script`] for an app that keeps a person's theme on a server: it reads `seed`
/// (storage key to value, the same map you give [`crate::SyncedStorage::new`], see
/// [`crate::custom_theme::keys`]) instead of `localStorage`, so the first paint of a page the
/// server renders already has that person's theme, mode, menu attributes and saved themes.
/// Keys missing from `seed` mean "nothing chosen": the `default_*` values apply.
///
/// The values are embedded as JSON with `<`, `>`, `&` and line separators escaped, so a saved
/// theme's CSS cannot end the `<script>` element.
pub fn theme_init_script_seeded(
    default_theme: &str,
    default_mode: ThemeMode,
    default_menu: (Option<&str>, Option<&str>),
    seed: &BTreeMap<String, String>,
) -> String {
    init_script(default_theme, default_mode, default_menu, Some(seed))
}

/// JSON that is safe inside an inline `<script>`.
fn script_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "null".into())
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

fn init_script(
    default_theme: &str,
    default_mode: ThemeMode,
    default_menu: (Option<&str>, Option<&str>),
    seed: Option<&BTreeMap<String, String>>,
) -> String {
    let js = |s: &str| script_json(&s);
    let opt = |s: Option<&str>| match s {
        Some(s) => js(s),
        None => "null".to_string(),
    };
    let reader = match seed {
        Some(seed) => format!(
            "var S={};g=function(k){{return Object.prototype.hasOwnProperty.call(S,k)?S[k]:null}};",
            script_json(seed)
        ),
        None => "g=function(k){try{return window.localStorage.getItem(k)}catch(e){return null}};"
            .to_string(),
    };
    format!(
        "(function(){{try{{\
var d=document.documentElement,g;{reader}\
var th=g({theme_key})||{default_theme};\
var mo=g({mode_key})||{default_mode};\
var dk=mo==='dark'||(mo==='system'&&window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches);\
d.classList.toggle('dark',dk);\
d.setAttribute('data-theme',th);\
var css='',mc=null,ma=null;\
if(th==={default_theme}){{mc={default_mc};ma={default_ma}}}\
var add=function(t){{if(!t)return;css+=t.rules||'';if(t.slug===th){{mc=t.menu_color||null;ma=t.menu_accent||null}}}};\
try{{JSON.parse(g({custom_key})||'[]').forEach(add)}}catch(e){{}}\
try{{add(JSON.parse(g({preview_key})||'null'))}}catch(e){{}}\
if(mc)d.setAttribute('data-vdl-menu-color',mc);else d.removeAttribute('data-vdl-menu-color');\
if(ma)d.setAttribute('data-vdl-menu-accent',ma);else d.removeAttribute('data-vdl-menu-accent');\
if(css){{var st=document.createElement('style');st.id='vdl-themes-boot';st.textContent=css;(document.head||d).appendChild(st)}}\
}}catch(e){{}}}})();",
        theme_key = js(keys::THEME),
        mode_key = js(keys::MODE),
        custom_key = js(keys::CUSTOM),
        preview_key = js(keys::PREVIEW),
        default_theme = js(default_theme),
        default_mode = js(default_mode.slug()),
        default_mc = opt(default_menu.0),
        default_ma = opt(default_menu.1),
    )
}

/// Owns the page's theme: `data-theme` and the `.dark` class on `<html>`, the preset's menu
/// attributes, the themes people save in the browser, and where all of it is kept. Put it
/// once near the root of the app, above anything that shows a [`crate::ThemeSwitcher`] or
/// [`crate::ModeToggle`] (they find it on their own). Not part of shadcn/ui.
///
/// - **Theme and mode** stay independent, as in shadcn. `ThemeContext::theme` is a built-in
///   theme's slug, a [`crate::ThemeStyle`] name, `preset`, a saved theme or `preview`.
///   `ThemeMode::System` follows the operating system and updates live.
/// - **`preset`**: a shadcn preset code (`b2oE5cW2a`; see [`Preset`]). Its tokens, radius, chart
///   colours and fonts become the theme named [`APP_PRESET_SLUG`], which is the starting
///   theme when the person has not chosen one. Its menu colour and accent become
///   `data-vdl-menu-color` / `data-vdl-menu-accent` on `<html>`. The style and icon library
///   are not rendered (see [`Preset::supported`]); that never errors. The app loads the font
///   files: [`Preset::google_fonts_url`] gives a stylesheet URL.
/// - **Custom themes**: [`ThemeContext::try_theme`], `save_theme`, `rename_theme`,
///   `delete_theme`, `export_themes` and `import_themes` make, keep and share themes built
///   from a preset code and/or pasted shadcn CSS. They show up in the theme switcher.
/// - **Storage**: the default is `localStorage` under the keys in [`keys`]. Pass `storage`
///   (any [`ThemeStorage`]) to keep them elsewhere, a server for instance.
/// - **No flash**: unless `script` is false, the provider prints an inline script
///   ([`theme_init_script`]) that applies the stored choice before the page paints. Values set
///   after hydration (the stored lists) reach the signals in an effect, so server-rendered
///   text never disagrees with the browser's first render.
#[component]
pub fn ThemeProvider(
    /// The theme used when none is stored. Default: `neutral`, or `preset` when `preset` is given.
    #[prop(optional, into)]
    default_theme: Option<String>,
    /// A shadcn preset code to offer as the theme named `preset`.
    #[prop(optional, into)]
    preset: MaybeProp<String>,
    /// The mode used when none is stored. Default: follow the system.
    #[prop(optional)]
    default_mode: ThemeMode,
    /// Where choices and saved themes are kept. Default: `localStorage`.
    #[prop(optional)]
    storage: Option<Arc<dyn ThemeStorage>>,
    /// Print the no-flash script (default true); see [`theme_init_script`].
    #[prop(default = true)]
    script: bool,
    /// The content.
    children: Children,
) -> impl IntoView {
    let storage: Arc<dyn ThemeStorage> = storage.unwrap_or_else(|| Arc::new(LocalStorage));

    let app = Memo::new(move |_| {
        let code = preset.get()?;
        match CustomTheme::with_slug(APP_PRESET_SLUG, "Preset", Some(&code), None) {
            Ok(t) => Some(t),
            Err(e) => {
                leptos::logging::warn!("ThemeProvider: ignoring preset {code:?}: {e}");
                None
            }
        }
    });
    let default_theme = default_theme.unwrap_or_else(|| {
        if app.get_untracked().is_some() {
            APP_PRESET_SLUG.to_string()
        } else {
            DEFAULT_THEME.to_string()
        }
    });

    // The same on the server and during hydration, so the first render matches the server's HTML.
    // A storage that can be read on the server (`SyncedStorage`) gives the person's own values
    // here, as long as the app builds it from the same values on both sides.
    let seed = storage.seed();
    let stored = seed.as_ref().map(|_| read_stored(&*storage));
    let theme = RwSignal::new(
        stored
            .as_ref()
            .and_then(|s| s.theme.clone())
            .unwrap_or_else(|| default_theme.clone()),
    );
    let mode = RwSignal::new(stored.as_ref().and_then(|s| s.mode).unwrap_or(default_mode));
    let system_dark = RwSignal::new(false);
    let dark = Signal::derive(move || match mode.get() {
        ThemeMode::Light => false,
        ThemeMode::Dark => true,
        ThemeMode::System => system_dark.get(),
    });
    let ctx = ThemeContext {
        theme,
        mode,
        dark,
        custom: RwSignal::new(
            stored
                .as_ref()
                .map(|s| s.custom.clone())
                .unwrap_or_default(),
        ),
        preview: RwSignal::new(stored.and_then(|s| s.preview)),
        registered: RwSignal::new(Vec::new()),
        app,
        storage: StoredValue::new(storage),
        default_theme: StoredValue::new(default_theme.clone()),
        #[cfg(feature = "hydrate")]
        default_mode,
    };

    let init = script.then(|| {
        let menu = app.get_untracked();
        init_script(
            &default_theme,
            default_mode,
            (
                menu.as_ref().and_then(|t| t.menu_color.as_deref()),
                menu.as_ref().and_then(|t| t.menu_accent.as_deref()),
            ),
            seed.as_ref(),
        )
    });

    #[cfg(feature = "hydrate")]
    hydrate_effects(ctx, system_dark);
    #[cfg(not(feature = "hydrate"))]
    let _ = system_dark;

    view! {
        <Provider value=ctx>
            {init.map(|js| view! { <script inner_html=js></script> })}
            <style data-vdl-themes="" inner_html=move || ctx.rules()></style>
            {children()}
        </Provider>
    }
}

/// Browser-only work: read the stored state once after hydration, keep `<html>` and storage in
/// step with the signals, and follow the system's colour scheme.
#[cfg(feature = "hydrate")]
fn hydrate_effects(ctx: ThemeContext, system_dark: RwSignal<bool>) {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;

    // The system's colour scheme, live.
    if let Some(mql) = window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
    {
        system_dark.set(mql.matches());
        let listener = Closure::<dyn FnMut(web_sys::Event)>::new({
            let mql = mql.clone();
            move |_| system_dark.set(mql.matches())
        });
        let _ = mql.add_event_listener_with_callback("change", listener.as_ref().unchecked_ref());
        // The provider lives as long as the app.
        listener.forget();
    }

    // Runs first: read what was stored (the no-flash script has already applied it to <html>).
    // It runs again when the storage says its values were replaced (a server's answer).
    let changes = ctx.storage.get_value().changes();
    Effect::new(move |_| {
        if let Some(changes) = &changes {
            changes.track();
        }
        untrack(|| {
            let storage = ctx.storage.get_value();
            let stored = read_stored(&*storage);
            ctx.custom.set(stored.custom);
            ctx.preview.set(stored.preview);
            if let Some(mode) = stored.mode {
                ctx.mode.set(mode);
            }
            if let Some(theme) = stored.theme
                && ctx.choices().iter().any(|c| c.slug == theme)
            {
                ctx.theme.set(theme);
            }
            // The no-flash script's copy of the saved themes is now the provider's own.
            if let Some(boot) = document().get_element_by_id("vdl-themes-boot") {
                boot.remove();
            }
        });
    });

    // Then keep <html> and storage in step.
    Effect::new(move |_| {
        let theme = ctx.theme.get();
        let dark = ctx.dark.get();
        let (menu_color, menu_accent) = ctx.menu();
        if let Some(html) = document().document_element() {
            let _ = html.set_attribute("data-theme", &theme);
            let _ = html.class_list().toggle_with_force("dark", dark);
            for (name, value) in [
                ("data-vdl-menu-color", menu_color),
                ("data-vdl-menu-accent", menu_accent),
            ] {
                match value {
                    Some(v) => {
                        let _ = html.set_attribute(name, &v);
                    }
                    None => {
                        let _ = html.remove_attribute(name);
                    }
                }
            }
        }
        let storage = ctx.storage.get_value();
        let default = ctx.default_theme.get_value();
        write_unless_default(&*storage, keys::THEME, &theme, &default);
    });
    Effect::new(move |_| {
        let mode = ctx.mode.get();
        let storage = ctx.storage.get_value();
        write_unless_default(&*storage, keys::MODE, mode.slug(), ctx.default_mode.slug());
    });
}

/// Stores `value` unless nothing is stored under `key` and `value` is what applies anyway, so
/// opening the app does not write defaults (to a server, say) that the person never chose.
#[cfg(feature = "hydrate")]
fn write_unless_default(storage: &dyn ThemeStorage, key: &str, value: &str, default: &str) {
    if storage.get(key).is_none() && value == default {
        return;
    }
    storage.set(key, value);
}

/// What a storage holds of the provider's own state, read once.
struct StoredTheme {
    theme: Option<String>,
    mode: Option<ThemeMode>,
    custom: Vec<CustomTheme>,
    preview: Option<CustomTheme>,
}

fn read_stored(storage: &dyn ThemeStorage) -> StoredTheme {
    StoredTheme {
        theme: storage.get(keys::THEME).filter(|t| !t.is_empty()),
        mode: storage
            .get(keys::MODE)
            .and_then(|m| ThemeMode::from_slug(&m)),
        custom: load_custom(storage),
        preview: storage
            .get(keys::PREVIEW)
            .and_then(|json| serde_json::from_str::<CustomTheme>(&json).ok())
            .filter(|t| t.slug == PREVIEW_SLUG),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_theme::MemoryStorage;

    fn seed(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn the_plain_script_reads_local_storage() {
        let js = theme_init_script("neutral", ThemeMode::System, (None, None));
        assert!(js.contains("window.localStorage.getItem"));
        assert!(!js.contains("var S="));
    }

    #[test]
    fn the_seeded_script_reads_the_given_values_instead() {
        let s = seed(&[(keys::THEME, "rose"), (keys::MODE, "dark")]);
        let js = theme_init_script_seeded("neutral", ThemeMode::System, (None, None), &s);
        assert!(!js.contains("localStorage"));
        assert!(js.contains(r#"var S={"verdelle-mode":"dark","verdelle-theme":"rose"}"#));
        assert!(js.contains("d.setAttribute('data-theme',th)"));
    }

    #[test]
    fn seeded_css_cannot_close_the_script_element() {
        let s = seed(&[(
            keys::CUSTOM,
            "[{\"rules\":\"</script><script>alert(1)</script> & \u{2028}\"}]",
        )]);
        let js = theme_init_script_seeded("neutral", ThemeMode::Light, (None, None), &s);
        assert!(!js.contains("</script"), "{js}");
        assert!(!js.contains("<script"), "{js}");
        assert!(js.contains("\\u003c/script\\u003e"));
        assert!(!js.contains('\u{2028}'));
    }

    #[component]
    fn Probe() -> impl IntoView {
        let t = use_theme();
        view! {
            <p id="probe">
                {move || format!("{}|{}|{}", t.theme.get(), t.mode.get().slug(), t.saved().len())}
            </p>
        }
    }

    fn rendered(storage: Arc<dyn ThemeStorage>) -> String {
        use leptos::tachys::view::RenderHtml;
        let owner = Owner::new();
        owner.with(|| view! { <ThemeProvider storage=storage><Probe /></ThemeProvider> }.to_html())
    }

    #[test]
    fn a_seeded_storage_starts_the_server_render_with_the_persons_theme() {
        let tmp = MemoryStorage::default();
        save_custom(
            &tmp,
            &[CustomTheme::build("Brand", Some("b0"), None).unwrap()],
        );
        let values = [
            (keys::THEME.to_string(), "brand".to_string()),
            (keys::MODE.to_string(), "dark".to_string()),
            (keys::CUSTOM.to_string(), tmp.get(keys::CUSTOM).unwrap()),
        ];
        let html = rendered(crate::SyncedStorage::new(values).into_arc());
        // Theme, mode and the number of saved themes are in the server's HTML ...
        assert!(html.contains("brand|dark|1"), "{html}");
        // ... and the printed no-flash script reads the same values, not localStorage.
        assert!(html.contains("var S="), "{html}");
        assert!(!html.contains("localStorage"), "{html}");
    }

    #[test]
    fn storage_that_only_works_in_the_browser_renders_the_defaults() {
        let html = rendered(Arc::new(LocalStorage));
        assert!(html.contains("neutral|system|0"), "{html}");
        assert!(html.contains("window.localStorage.getItem"), "{html}");
    }

    #[test]
    fn the_customizer_renders_on_the_server() {
        use leptos::tachys::view::RenderHtml;
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <ThemeProvider>
                    <crate::ThemeCustomizer max_saved=3usize />
                </ThemeProvider>
            }
            .to_html()
        });
        for part in ["theme", "preset", "tuning", "css", "saved", "preview"] {
            assert!(html.contains(&format!("data-part=\"{part}\"")), "{part}");
        }
        assert!(html.contains("0 / 3"), "the saved count shows the limit");
    }

    #[test]
    fn a_compact_customizer_leaves_out_the_explanatory_paragraphs() {
        use leptos::tachys::view::RenderHtml;
        let render = |compact: bool| {
            let owner = Owner::new();
            owner.with(|| {
                view! {
                    <ThemeProvider>
                        <crate::ThemeCustomizer compact=compact />
                    </ThemeProvider>
                }
                .to_html()
            })
        };
        let paragraphs = [
            "Paste it here.",
            "Each change is applied to the page as you make it.",
            "Only those blocks are read.",
            "Apply a preset or pasted CSS to save it.",
        ];
        let full = render(false);
        for p in paragraphs {
            assert!(full.contains(p), "the full panel explains: {p}");
        }
        let compact = render(true);
        for p in paragraphs {
            assert!(!compact.contains(p), "compact keeps: {p}");
        }
        // Everything else is still there: every section with its title and controls.
        for part in ["theme", "preset", "tuning", "css", "saved", "preview"] {
            assert!(compact.contains(&format!("data-part=\"{part}\"")), "{part}");
        }
        for label in [
            "Preset code",
            "shadcn CSS",
            "Colour theme",
            "Nothing saved yet.",
        ] {
            assert!(compact.contains(label), "compact lost: {label}");
        }
    }

    #[test]
    fn stored_state_is_read_from_any_storage() {
        let store = MemoryStorage::default();
        let none = read_stored(&store);
        assert!(none.theme.is_none() && none.mode.is_none() && none.custom.is_empty());

        let saved = CustomTheme::build("Brand", Some("b0"), None).unwrap();
        save_custom(&store, std::slice::from_ref(&saved));
        store.set(keys::THEME, "brand");
        store.set(keys::MODE, "dark");
        let got = read_stored(&store);
        assert_eq!(got.theme.as_deref(), Some("brand"));
        assert_eq!(got.mode, Some(ThemeMode::Dark));
        assert_eq!(got.custom, vec![saved]);

        // Junk is ignored, not fatal.
        store.set(keys::MODE, "sepia");
        store.set(keys::PREVIEW, "{nope");
        let got = read_stored(&store);
        assert_eq!(got.mode, None);
        assert!(got.preview.is_none());
    }
}
