use crate::preset::title_case;
use crate::theme_css::{parse_theme_css, scoped_css, slugify};
use crate::theme_provider::{ThemeChoice, ThemeContext, ThemeSource};
use crate::themes::THEMES;
use leptos::prelude::*;

/// Adds a theme from shadcn-exported CSS: the `:root { ... }` and `.dark { ... }` blocks that
/// shadcn's `/create` page, its theme pages or tweakcn give you (oklch colours, `--radius`,
/// sidebar and chart tokens, fonts: every custom property passes through). Not part of
/// shadcn/ui.
///
/// ```ignore
/// <ThemeStyle name="rose-garden" css=include_str!("rose-garden.css") />
/// ```
///
/// It writes the tokens as rules scoped to `data-theme="rose-garden"` (and, in dark mode,
/// `.dark[data-theme="rose-garden"]`), so they apply when that theme is chosen and nothing
/// else changes. Tokens the CSS does not set keep the default theme's values. Inside a
/// [`crate::ThemeProvider`] the theme is also listed, so [`crate::ThemeSwitcher`] shows it next
/// to the built-in ones. Without one, set `data-theme` on `<html>` yourself.
///
/// The `<style>` is part of the page, so it is server-rendered and there is nothing to flash.
/// Place the component above the `ThemeSwitcher` (the list is read in render order). CSS with
/// no `:root` block is reported to the console and ignored.
///
/// `name` is made into the `data-theme` value (lower case, dashes); `label` is what the
/// switcher shows (default: the name, in title case when it is a plain slug). The name of a
/// built-in theme restyles that theme instead of adding one. `css` may be a signal.
#[component]
pub fn ThemeStyle(
    #[prop(into)] name: String,
    #[prop(into)] css: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let slug = slugify(&name);
    let label = label.unwrap_or_else(|| {
        if name.trim() == slug {
            title_case(&slug)
        } else {
            name.trim().to_string()
        }
    });

    if let Some(ctx) = ThemeContext::try_use()
        && !slug.is_empty()
        && !THEMES.iter().any(|t| t.slug == slug)
    {
        ctx.register(ThemeChoice {
            slug: slug.clone(),
            name: label,
            source: ThemeSource::Registered,
        });
        let slug = slug.clone();
        on_cleanup(move || ctx.unregister(&slug));
    }

    let rules = Memo::new(move |_| {
        if slug.is_empty() {
            leptos::logging::warn!("ThemeStyle: the name {name:?} has no letters or digits");
            return String::new();
        }
        match parse_theme_css(&css.get()) {
            Ok(parsed) => scoped_css(&slug, &parsed.light, &parsed.dark.unwrap_or_default()),
            Err(e) => {
                leptos::logging::warn!("ThemeStyle {slug:?}: {e}");
                String::new()
            }
        }
    });

    view! { <style data-vdl-theme-style="" inner_html=move || rules.get()></style> }
}
