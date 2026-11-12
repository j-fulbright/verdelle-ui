//! Turns a [`Preset`] into the token sets shadcn would write for it.
//!
//! shadcn builds a preset's tokens in `buildRegistryTheme` (`apps/v4/registry/config.ts` in
//! shadcn-ui/ui): the **base colour** is a complete token set (neutral, stone, zinc, ...), the
//! **theme** is layered over it (an accent theme such as rose changes only the primary,
//! secondary, chart and sidebar-primary tokens; a theme named like the base colour changes
//! nothing), the five `--chart-*` tokens are then taken from the **chart colour** theme, a
//! and a non-default **radius** replaces `--radius`. (shadcn also remaps `--accent` to
//! `--primary` page-wide for a bold menu accent; Verdelle does not: its CSS reads
//! `data-vdl-menu-accent` and restyles only the highlighted menu items. [`remap_bold_accent`]
//! does shadcn's remap for anyone who wants it.) Verdelle's built-in themes are those same token sets
//! ([`crate::themes::THEMES`]), so the layering works on them as they are.
//!
//! Anything a token set lacks keeps the default neutral value: composition starts from the
//! neutral theme, so the result is always complete. Colours Verdelle has no tokens for (shadcn's
//! `gray`) behave as neutral.

use crate::preset::{FontHeading, FontKind, Preset, Radius};
use crate::theme_css::{self, Decls, ParsedTheme, ThemeTokens, builtin_theme};

impl Radius {
    /// The `--radius` this choice sets, or `None` for `Default` (keep the theme's own).
    pub fn value(self) -> Option<&'static str> {
        match self {
            Radius::Default => None,
            Radius::None => Some("0"),
            Radius::Small => Some("0.45rem"),
            Radius::Medium => Some("0.625rem"),
            Radius::Large => Some("0.875rem"),
        }
    }
}

impl Preset {
    /// The final light and dark colour tokens (and `--radius`) for this preset, as shadcn's
    /// `buildRegistryTheme` produces them. The style and icon library do not matter here.
    pub fn tokens(&self) -> ThemeTokens {
        let mut t = ThemeTokens::neutral();
        if let Some(base) = builtin_theme(self.base_color.slug()) {
            t.overlay(&base);
        }
        if let Some(theme) = builtin_theme(self.theme.slug()) {
            t.overlay(&theme);
        }
        // The chart colour only supplies the five chart tokens.
        if let Some(chart) = builtin_theme(self.chart_color.slug()) {
            let charts = |decls: &Decls| -> Decls {
                decls
                    .iter()
                    .filter(|(k, _)| is_chart_token(k))
                    .cloned()
                    .collect()
            };
            t.overlay(&ParsedTheme {
                light: charts(&chart.light),
                dark: chart.dark.as_ref().map(charts),
            });
        }
        if let Some(radius) = self.radius.value() {
            theme_css::set(&mut t.light, "--radius", radius.to_string());
        }
        t
    }

    /// The font variables for this preset: `--font-sans` is the body font (whatever its kind,
    /// since Verdelle's text reads it), `--font-serif` / `--font-mono` are set when the font is of
    /// that kind, and `--font-heading` is the heading font, or `var(--font-sans)` when it
    /// inherits. Fonts do not change with light or dark, so these belong with the light tokens.
    pub fn font_vars(&self) -> Decls {
        let mut out: Decls = vec![("--font-sans".to_string(), self.font.stack())];
        if self.font.kind() != FontKind::Sans {
            out.push((self.font.variable().to_string(), self.font.stack()));
        }
        let heading = match self.font_heading {
            FontHeading::Font(f) if f != self.font => f.stack(),
            _ => "var(--font-sans)".to_string(),
        };
        out.push(("--font-heading".to_string(), heading));
        out
    }

    /// [`Self::tokens`] with [`Self::font_vars`] added to the light set: everything a preset
    /// puts into CSS.
    pub fn tokens_with_fonts(&self) -> ThemeTokens {
        let mut t = self.tokens();
        theme_css::set_all(&mut t.light, self.font_vars());
        t
    }

    /// CSS that applies this preset when `data-theme="<slug>"` is on `<html>`. The menu colour
    /// and accent are not in it: they are the `data-vdl-menu-color` / `data-vdl-menu-accent`
    /// attributes on `<html>`, which `ThemeProvider` sets.
    pub fn scoped_css(&self, slug: &str) -> String {
        let slug = theme_css::slugify(slug);
        // The tokens only define the font variables; these rules make the page use them: the
        // body font, and the heading font for headings and component titles.
        format!(
            "{}[data-theme=\"{slug}\"] body{{font-family:var(--font-sans)}}\
             [data-theme=\"{slug}\"] :is(h1,h2,h3,h4,h5,h6,[class*=\"__title\"]){{font-family:var(--font-heading)}}",
            self.tokens_with_fonts().scoped_css(&slug)
        )
    }
}

/// shadcn's bold-menu-accent remap: `--accent` and `--accent-foreground` take the primary's
/// values, in light and dark. Not applied by [`Preset::tokens`].
pub fn remap_bold_accent(t: &mut ThemeTokens) {
    for decls in [&mut t.light, &mut t.dark] {
        if let Some(p) = theme_css::get(decls, "--primary").map(str::to_string) {
            theme_css::set(decls, "--accent", p);
        }
        if let Some(p) = theme_css::get(decls, "--primary-foreground").map(str::to_string) {
            theme_css::set(decls, "--accent-foreground", p);
        }
    }
}

fn is_chart_token(name: &str) -> bool {
    matches!(
        name,
        "--chart-1" | "--chart-2" | "--chart-3" | "--chart-4" | "--chart-5"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::{BaseColor, Font, MenuAccent, Radius, Theme};
    use std::collections::BTreeMap;

    #[derive(serde::Deserialize)]
    struct Composed {
        code: String,
        tokens: Tokens,
    }
    #[derive(serde::Deserialize)]
    struct Tokens {
        light: BTreeMap<String, String>,
        dark: BTreeMap<String, String>,
    }

    fn as_map(decls: &Decls) -> BTreeMap<String, String> {
        decls
            .iter()
            .map(|(k, v)| (k.trim_start_matches("--").to_string(), v.clone()))
            .collect()
    }

    /// shadcn's own `buildRegistryTheme` (run under node over `apps/v4/registry/themes.ts`)
    /// gives these token sets for the same codes; ours must be identical, token for token.
    #[test]
    fn matches_shadcns_build_registry_theme() {
        let fixture: Vec<Composed> =
            serde_json::from_str(include_str!("../tests/fixtures/preset_tokens.json")).unwrap();
        assert!(fixture.len() >= 25);
        let mut accents = 0;
        for c in &fixture {
            let preset = Preset::decode(&c.code).unwrap();
            let mut got = preset.tokens();
            if preset.menu_accent == MenuAccent::Bold {
                remap_bold_accent(&mut got);
            }
            assert_eq!(as_map(&got.light), c.tokens.light, "light of {}", c.code);
            assert_eq!(as_map(&got.dark), c.tokens.dark, "dark of {}", c.code);
            if preset.theme != Theme::Neutral && preset.theme.slug() != preset.base_color.slug() {
                accents += 1;
            }
        }
        assert!(accents > 5, "the fixture should cover accent themes");
    }

    #[test]
    fn default_preset_is_the_neutral_theme() {
        let t = Preset::default().tokens();
        assert_eq!(t, ThemeTokens::neutral());
        assert_eq!(t.light("--radius"), Some("0.625rem"));
    }

    #[test]
    fn accent_theme_layers_over_the_base_colour() {
        let zinc_rose = Preset {
            base_color: BaseColor::Zinc,
            theme: Theme::Rose,
            chart_color: Theme::Rose,
            ..Preset::default()
        };
        let t = zinc_rose.tokens();
        let rose = builtin_theme("rose").unwrap();
        let zinc = builtin_theme("zinc").unwrap();
        // Accent tokens come from rose, the rest from zinc.
        assert_eq!(
            t.light("--primary"),
            theme_css::get(&rose.light, "--primary")
        );
        assert_eq!(
            t.light("--background"),
            theme_css::get(&zinc.light, "--background")
        );
        assert_eq!(
            t.light("--chart-1"),
            theme_css::get(&rose.light, "--chart-1")
        );
    }

    #[test]
    fn chart_colour_only_replaces_the_chart_tokens() {
        let p = Preset {
            theme: Theme::Rose,
            chart_color: Theme::Blue,
            ..Preset::default()
        };
        let t = p.tokens();
        let rose = builtin_theme("rose").unwrap();
        let blue = builtin_theme("blue").unwrap();
        assert_eq!(
            t.light("--primary"),
            theme_css::get(&rose.light, "--primary")
        );
        assert_eq!(
            t.light("--chart-3"),
            theme_css::get(&blue.light, "--chart-3")
        );
        assert_ne!(
            t.light("--chart-3"),
            theme_css::get(&rose.light, "--chart-3")
        );
        // The blue theme's primary did not leak in.
        assert_ne!(
            t.light("--primary"),
            theme_css::get(&blue.light, "--primary")
        );
    }

    #[test]
    fn bold_menu_accent_leaves_tokens_alone_but_can_be_remapped() {
        let p = Preset {
            theme: Theme::Rose,
            menu_accent: MenuAccent::Bold,
            ..Preset::default()
        };
        let mut t = p.tokens();
        assert_ne!(t.light("--accent"), t.light("--primary"));
        remap_bold_accent(&mut t);
        assert_eq!(t.light("--accent"), t.light("--primary"));
        assert_eq!(t.dark("--accent"), t.dark("--primary"));
        assert_eq!(
            t.light("--accent-foreground"),
            t.light("--primary-foreground")
        );
    }

    #[test]
    fn radius_replaces_the_light_radius_only_when_chosen() {
        for (radius, want) in [
            (Radius::Default, "0.625rem"),
            (Radius::None, "0"),
            (Radius::Small, "0.45rem"),
            (Radius::Medium, "0.625rem"),
            (Radius::Large, "0.875rem"),
        ] {
            let t = Preset {
                radius,
                ..Preset::default()
            }
            .tokens();
            assert_eq!(t.light("--radius"), Some(want), "{radius:?}");
        }
    }

    #[test]
    fn gray_falls_back_to_neutral() {
        let p = Preset {
            base_color: BaseColor::Gray,
            theme: Theme::Gray,
            chart_color: Theme::Gray,
            ..Preset::default()
        };
        assert_eq!(p.tokens(), ThemeTokens::neutral());
    }

    #[test]
    fn font_vars_follow_the_kind_and_the_heading() {
        let sans = Preset {
            font: Font::Figtree,
            ..Preset::default()
        }
        .font_vars();
        assert_eq!(
            theme_css::get(&sans, "--font-sans"),
            Some(Font::Figtree.stack().as_str())
        );
        assert_eq!(
            theme_css::get(&sans, "--font-heading"),
            Some("var(--font-sans)")
        );
        assert_eq!(theme_css::get(&sans, "--font-serif"), None);

        let serif = Preset {
            font: Font::Lora,
            font_heading: FontHeading::Font(Font::PlayfairDisplay),
            ..Preset::default()
        }
        .font_vars();
        assert_eq!(
            theme_css::get(&serif, "--font-serif"),
            Some(Font::Lora.stack().as_str())
        );
        assert_eq!(
            theme_css::get(&serif, "--font-sans"),
            Some(Font::Lora.stack().as_str())
        );
        assert_eq!(
            theme_css::get(&serif, "--font-heading"),
            Some(Font::PlayfairDisplay.stack().as_str())
        );
        // A heading equal to the body font is the same as inheriting.
        let same = Preset {
            font: Font::Lora,
            font_heading: FontHeading::Font(Font::Lora),
            ..Preset::default()
        }
        .font_vars();
        assert_eq!(
            theme_css::get(&same, "--font-heading"),
            Some("var(--font-sans)")
        );
    }

    #[test]
    fn scoped_css_carries_colours_and_fonts() {
        let p = Preset::decode("b2oE5cW2a").unwrap();
        let css = p.scoped_css("preset");
        assert!(css.starts_with("[data-theme=\"preset\"]{"));
        assert!(css.contains("--primary:"));
        assert!(css.contains("--font-sans:'Figtree'"));
        assert!(css.contains(".dark[data-theme=\"preset\"]"));
    }
}
