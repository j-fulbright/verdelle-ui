use crate::Size;
use crate::theme_provider::{ThemeChoice, ThemeContext, ThemeSource};
use crate::themes::THEMES;
use leptos::prelude::*;

/// The tokens a theme swatch shows, in order: the surface, the main colour, the accent and the
/// first chart colour. Presets mostly change the primary and chart colours (the accent and
/// secondary tokens stay neutral grey, destructive is the same red everywhere), so those are
/// what tells two themes apart.
pub(crate) const SWATCH_TOKENS: [&str; 4] = ["--background", "--primary", "--accent", "--chart-1"];

/// The dots of a swatch (see [`SWATCH_TOKENS`]). They read the tokens of the nearest
/// `data-theme` ancestor, so the wrapper decides which theme they show.
#[component]
pub(crate) fn SwatchDots() -> impl IntoView {
    SWATCH_TOKENS
        .into_iter()
        .map(|t| view! { <i style=format!("background:var({t})")></i> })
        .collect_view()
}

/// Four small colour dots (background, primary, accent, first chart colour) drawn from the
/// tokens of the theme named by `data-theme`, in whatever mode the page is in. The scoped CSS
/// of every theme the list shows (built-in, `ThemeStyle`, and the provider's preview, preset and
/// saved themes) is in the page, so the dots read that theme's own values.
#[component]
pub(crate) fn ThemeSwatch(#[prop(into)] theme: Signal<String>) -> impl IntoView {
    view! {
        <span class="vdl-theme-swatch" data-theme=move || theme.get() aria-hidden="true">
            <SwatchDots />
        </span>
    }
}

/// A compact theme chooser: a Select whose trigger and items show the theme's colour
/// swatches next to its name. Not part of shadcn/ui; the recommended control, where
/// [`crate::ThemePicker`] is the roomier card grid.
///
/// **With a [`crate::ThemeProvider`]** it needs no props: it lists the built-in themes, then
/// the ones declared with [`crate::ThemeStyle`], the provider's `preset`, the themes people
/// saved in the browser and the unsaved preview (under a "Custom" heading), shows the chosen
/// one and changes it. The list updates as themes are saved, renamed or deleted.
///
/// **Without one** it is controlled: `value` is the selected theme slug and `on_change`
/// receives the slug the user picks; applying it (`data-theme` on `<html>`, persistence) is
/// the caller's job. `themes` replaces the list with your own `(slug, name)` pairs (the
/// built-in ones when absent): a theme added with `themes` must have CSS that scopes it to
/// `data-theme="<slug>"` (see [`crate::ThemeStyle`] or the `themegen` example).
///
/// Keyboard behaviour is the Select's (arrows, Home/End, typeahead, Enter, Escape). Light or
/// dark is separate: see [`crate::ModeToggle`].
#[component]
pub fn ThemeSwitcher(
    /// The selected theme slug. Default: the provider's.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Called with the slug the user picks. Default: choose it on the provider.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Replace the list of themes with these `(slug, name)` pairs.
    #[prop(optional, into)]
    themes: MaybeProp<Vec<(String, String)>>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: MaybeProp<bool>,
    /// The accessible name, when no visible label points at the trigger.
    #[prop(default = "Theme".to_string(), into)]
    aria_label: String,
    /// The element's `id`.
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let class = StoredValue::new(class);
    let aria_label = StoredValue::new(aria_label);
    let ctx = ThemeContext::try_use();

    let choices = Signal::derive(move || -> Vec<ThemeChoice> {
        if let Some(own) = themes.get() {
            return own
                .into_iter()
                .map(|(slug, name)| ThemeChoice {
                    slug,
                    name,
                    source: ThemeSource::BuiltIn,
                })
                .collect();
        }
        match ctx {
            Some(ctx) => ctx.choices(),
            None => THEMES
                .iter()
                .map(|t| ThemeChoice {
                    slug: t.slug.to_string(),
                    name: t.name.to_string(),
                    source: ThemeSource::BuiltIn,
                })
                .collect(),
        }
    });
    let current = Signal::derive(move || {
        value
            .get()
            .or_else(|| ctx.map(|c| c.theme.get()))
            .unwrap_or_else(|| crate::themes::DEFAULT_THEME.to_string())
    });
    let on_change = on_change.unwrap_or_else(|| {
        Callback::new(move |slug: String| {
            if let Some(ctx) = ctx {
                ctx.choose(slug);
            }
        })
    });
    let items = Signal::derive(move || {
        Some(
            choices
                .get()
                .into_iter()
                .map(|c| (c.slug, c.name))
                .collect::<Vec<_>>(),
        )
    });

    let item = |c: ThemeChoice| {
        view! {
            <crate::SelectItem value=c.slug.clone()>
                <ThemeSwatch theme=c.slug.clone() />
                {c.name.clone()}
            </crate::SelectItem>
        }
    };

    view! {
        <crate::Select
            value=Signal::derive(move || Some(current.get()))
            on_value_change=on_change
            disabled=disabled
            items=items
        >
            <crate::SelectTrigger
                size=size
                aria_label=aria_label.get_value()
                id=id
                class=format!("vdl-theme-switcher {}", class.get_value())
            >
                <ThemeSwatch theme=current />
                <crate::SelectValue />
            </crate::SelectTrigger>
            <crate::SelectContent>
                {move || {
                    let (builtin, custom): (Vec<_>, Vec<_>) = choices
                        .get()
                        .into_iter()
                        .partition(|c| c.source == ThemeSource::BuiltIn);
                    view! {
                        {builtin.into_iter().map(item).collect_view()}
                        {(!custom.is_empty())
                            .then(|| {
                                view! {
                                    <crate::SelectSeparator />
                                    <crate::SelectGroup>
                                        <crate::SelectLabel>"Custom"</crate::SelectLabel>
                                        {custom.clone().into_iter().map(item).collect_view()}
                                    </crate::SelectGroup>
                                }
                            })}
                    }
                }}
            </crate::SelectContent>
        </crate::Select>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_theme::{CustomTheme, MemoryStorage, ThemeStorage, keys, save_custom};
    use crate::theme_css::ThemeTokens;
    use crate::{ThemePicker, ThemeProvider};
    use leptos::tachys::view::RenderHtml;

    /// The declarations of the rule `[data-theme="slug"]{...}` in `html`.
    fn rule(html: &str, slug: &str) -> String {
        let head = format!("[data-theme=\"{slug}\"]{{");
        let at = html
            .find(&head)
            .unwrap_or_else(|| panic!("no scoped CSS for {slug}: {html}"));
        let rest = &html[at + head.len()..];
        rest[..rest.find('}').unwrap()].to_string()
    }

    fn value(decls: &str, token: &str) -> Option<String> {
        decls
            .split(';')
            .find_map(|d| d.strip_prefix(&format!("{token}:")))
            .map(str::to_string)
    }

    /// A provider holding a saved theme (a green accent preset) and an unsaved preview (a preset
    /// plus pasted CSS), the preview chosen, with a swatch and a picker.
    fn render() -> String {
        let store = MemoryStorage::default();
        let saved = CustomTheme::with_slug("forest", "Forest", Some("b2oE7bhx2"), None).unwrap();
        save_custom(&store, &[saved]);
        let preview = CustomTheme::with_slug(
            "preview",
            "Preview",
            Some("b2oE5cW2a"),
            Some(":root{--primary:oklch(0.55 0.2 300);--chart-1:oklch(0.6 0.2 320)}"),
        )
        .unwrap();
        store.set(keys::PREVIEW, &serde_json::to_string(&preview).unwrap());
        store.set(keys::THEME, "preview");
        // A seeded storage is what the server renders from (a browser one only exists after
        // hydration).
        let values: Vec<(String, String)> = [keys::CUSTOM, keys::PREVIEW, keys::THEME]
            .into_iter()
            .map(|k| (k.to_string(), store.get(k).unwrap()))
            .collect();
        let storage = crate::SyncedStorage::new(values).into_arc();
        let owner = Owner::new();
        owner.with(|| {
            view! {
                <ThemeProvider storage=storage>
                    <ThemeSwatch theme="preview" />
                    <ThemePicker />
                </ThemeProvider>
            }
            .to_html()
        })
    }

    #[test]
    fn preview_and_saved_themes_have_scoped_css_with_every_swatch_token() {
        let html = render();
        let neutral = ThemeTokens::neutral();
        for slug in ["preview", "forest"] {
            let decls = rule(&html, slug);
            for token in SWATCH_TOKENS {
                assert!(value(&decls, token).is_some(), "{slug} lacks {token}");
            }
            // The colour the theme changed is the theme's own, not the neutral default.
            let primary = value(&decls, "--primary").unwrap();
            assert_ne!(Some(primary.as_str()), neutral.light("--primary"), "{slug}");
        }
        // The chart dot: the preview's pasted colour, the saved preset's chart colour.
        assert_eq!(
            value(&rule(&html, "preview"), "--chart-1").as_deref(),
            Some("oklch(0.6 0.2 320)")
        );
        assert_ne!(
            value(&rule(&html, "forest"), "--chart-1").as_deref(),
            neutral.light("--chart-1")
        );
    }

    #[test]
    fn the_dots_read_the_swatch_tokens_of_their_own_theme() {
        // Without the hydration markers.
        let html = render().replace("<!>", "");
        let dots: String = SWATCH_TOKENS
            .iter()
            .map(|t| format!("<i style=\"background:var({t});\"></i>"))
            .collect();
        assert!(
            html.contains(&format!(
                "<span data-theme=\"preview\" aria-hidden=\"true\" class=\"vdl-theme-swatch\">{dots}"
            )),
            "{html}"
        );
        for slug in ["preview", "forest"] {
            let card = html
                .find(&format!(
                    "<label data-theme=\"{slug}\" class=\"vdl-theme-card\""
                ))
                .unwrap_or_else(|| panic!("no card for {slug}: {html}"));
            assert!(
                html[card..].contains(&format!(
                    "<span aria-hidden=\"true\" class=\"vdl-theme-card__swatch\">{dots}"
                )),
                "{slug}"
            );
        }
    }
}
