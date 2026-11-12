//! shadcn-format theme CSS as data: reading it, layering token sets, and writing it back out
//! scoped to a `data-theme` name.
//!
//! A theme file is what the shadcn theme tools (tweakcn, shadcn's `/create` page, the docs'
//! theme pages) export: a `:root { --x: v; }` block for light and a `.dark { ... }` block for
//! dark. Everything else in the file (`@theme inline`, `@layer base`, ...) is ignored, and
//! every `--custom-property` in the two blocks is passed through, so `--radius`, fonts,
//! shadows and any extra tokens survive.
//!
//! Pure Rust with no dependencies on the browser, so it runs on the server, in tests and in
//! the `themegen` example.

use crate::themes::THEMES;
use std::fmt;
use std::sync::OnceLock;

/// `(property, value)` pairs, in file order, last duplicate wins. Properties keep their `--`.
pub type Decls = Vec<(String, String)>;

/// Why CSS could not be read as a theme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeCssError(pub String);

impl fmt::Display for ThemeCssError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ThemeCssError {}

/// The `:root` and `.dark` blocks of a theme file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedTheme {
    pub light: Decls,
    pub dark: Option<Decls>,
}

/// Reads shadcn-format theme CSS. Fails when there is no `:root` block with custom properties
/// (a `.dark` block is optional).
pub fn parse_theme_css(css: &str) -> Result<ParsedTheme, ThemeCssError> {
    let css = strip_comments(css);
    let mut light = None;
    let mut dark = None;

    let mut rest = css.as_str();
    while let Some(open) = rest.find('{') {
        // Skip preceding statements without a block (`@import url(...);`).
        let selector = rest[..open].rsplit(';').next().unwrap_or("").trim();
        let (body, after) = split_block(&rest[open..])?;
        match selector {
            ":root" => light = Some(merge(light, parse_decls(body))),
            ".dark" | ":root.dark" | "html.dark" => dark = Some(merge(dark, parse_decls(body))),
            // At-rules and everything else, including their nested blocks.
            _ => {}
        }
        rest = after;
    }

    match light {
        Some(light) if !light.is_empty() => Ok(ParsedTheme { light, dark }),
        _ => Err(ThemeCssError(
            "no `:root { --token: value; }` block with custom properties found".into(),
        )),
    }
}

fn merge(existing: Option<Decls>, new: Decls) -> Decls {
    let mut out = existing.unwrap_or_default();
    set_all(&mut out, new);
    out
}

/// Sets each declaration, replacing one with the same name or appending.
pub fn set_all(target: &mut Decls, new: impl IntoIterator<Item = (String, String)>) {
    for (k, v) in new {
        set(target, &k, v);
    }
}

/// Sets one declaration, replacing one with the same name or appending.
pub fn set(target: &mut Decls, name: &str, value: String) {
    match target.iter_mut().find(|(ek, _)| ek == name) {
        Some(slot) => slot.1 = value,
        None => target.push((name.to_string(), value)),
    }
}

/// The value of a declaration.
pub fn get<'a>(decls: &'a Decls, name: &str) -> Option<&'a str> {
    decls
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

/// Given text starting at `{`, returns the body between the braces and the text after the
/// matching `}`. Nested braces (inside `@layer` and the like) are balanced.
fn split_block(s: &str) -> Result<(&str, &str), ThemeCssError> {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Ok((&s[1..i], &s[i + 1..]));
                }
            }
            _ => {}
        }
    }
    Err(ThemeCssError("unbalanced braces".into()))
}

fn parse_decls(body: &str) -> Decls {
    // Split on `;` outside parentheses and quotes, so values like `url("a;b")` or font
    // stacks survive.
    let mut parts = Vec::new();
    let (mut depth, mut quote, mut start) = (0i32, None::<char>, 0usize);
    for (i, c) in body.char_indices() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '(') => depth += 1,
            (None, ')') => depth -= 1,
            (None, ';') if depth == 0 => {
                parts.push(&body[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&body[start..]);

    parts
        .into_iter()
        .filter_map(|p| {
            let (name, value) = p.split_once(':')?;
            let name = name.trim();
            if !name.starts_with("--") {
                return None;
            }
            let value = normalize_value(value);
            (!value.is_empty()).then(|| (name.to_string(), value))
        })
        .collect()
}

/// Collapses whitespace, and wraps the older shadcn colour format (`222.2 84% 4.9%`, bare
/// HSL channels meant for `hsl(var(--x))`) into a usable `hsl(...)` value.
fn normalize_value(v: &str) -> String {
    let v = v.split_whitespace().collect::<Vec<_>>().join(" ");
    if is_hsl_channels(&v) {
        format!("hsl({v})")
    } else {
        v
    }
}

fn is_hsl_channels(v: &str) -> bool {
    let (colour, alpha) = match v.split_once('/') {
        Some((c, a)) => (c.trim(), Some(a.trim())),
        None => (v, None),
    };
    let toks: Vec<&str> = colour.split(' ').collect();
    let num = |s: &str| s.trim_end_matches("deg").parse::<f32>().is_ok();
    let pct = |s: &str| {
        s.strip_suffix('%')
            .is_some_and(|n| n.parse::<f32>().is_ok())
    };
    toks.len() == 3
        && num(toks[0])
        && pct(toks[1])
        && pct(toks[2])
        && alpha.is_none_or(|a| num(a.trim_end_matches('%')))
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        match rest[start + 2..].find("*/") {
            Some(end) => rest = &rest[start + 2 + end + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

/// Makes a string usable as a `data-theme` name: lower case letters, digits and single
/// dashes (`"My Brand!"` becomes `"my-brand"`). Empty when nothing is left.
pub fn slugify(name: &str) -> String {
    let mut slug = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if !slug.ends_with('-') && !slug.is_empty() {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').to_string()
}

/// A light and a dark token set.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThemeTokens {
    pub light: Decls,
    pub dark: Decls,
}

impl ThemeTokens {
    /// Lays `other` over these tokens: its declarations replace the ones with the same name
    /// and the rest are added. When `other` has no dark block, dark is left as it was.
    pub fn overlay(&mut self, other: &ParsedTheme) {
        set_all(&mut self.light, other.light.iter().cloned());
        if let Some(dark) = &other.dark {
            set_all(&mut self.dark, dark.iter().cloned());
        }
    }

    /// The complete default theme (neutral): every token Verdelle's components read.
    pub fn neutral() -> ThemeTokens {
        let mut t = ThemeTokens::default();
        if let Some(n) = builtin_theme("neutral") {
            t.overlay(&n);
        }
        t
    }

    /// The light value of a token (`"--primary"`).
    pub fn light(&self, name: &str) -> Option<&str> {
        get(&self.light, name)
    }

    /// The dark value of a token.
    pub fn dark(&self, name: &str) -> Option<&str> {
        get(&self.dark, name)
    }

    /// CSS that applies these tokens when `data-theme="<slug>"` is on `<html>` (or an element
    /// inside a dark page that sets its own `data-theme`, as the theme picker's cards do).
    /// The slug is made safe with [`slugify`]. An empty dark set writes no dark rule.
    pub fn scoped_css(&self, slug: &str) -> String {
        scoped_css(slug, &self.light, &self.dark)
    }
}

/// See [`ThemeTokens::scoped_css`].
pub fn scoped_css(slug: &str, light: &Decls, dark: &Decls) -> String {
    let slug = slugify(slug);
    let mut css = String::new();
    css.push_str(&format!("[data-theme=\"{slug}\"]{{"));
    write_decls(&mut css, light);
    css.push('}');
    if !dark.is_empty() {
        // The last selector: popups the "inverted" menu colour option keeps dark in a light page.
        css.push_str(&format!(
            ".dark[data-theme=\"{slug}\"],.dark [data-theme=\"{slug}\"],\
             :root[data-theme=\"{slug}\"][data-vdl-menu-color^=\"inverted\"] \
             :is(.vdl-menu,.vdl-select__content,.vdl-combobox__content){{"
        ));
        write_decls(&mut css, dark);
        css.push('}');
    }
    css
}

fn write_decls(out: &mut String, decls: &Decls) {
    for (k, v) in decls {
        // A `}` or `;` smuggled into a value would end the rule early.
        let v = v.replace(['{', '}', '<'], "");
        out.push_str(&format!("{k}:{v};"));
    }
}

/// The parsed token set of one of the built-in themes ([`THEMES`]), by slug. Parsed once.
pub fn builtin_theme(slug: &str) -> Option<ParsedTheme> {
    static PARSED: OnceLock<Vec<(&'static str, Option<ParsedTheme>)>> = OnceLock::new();
    PARSED
        .get_or_init(|| {
            THEMES
                .iter()
                .map(|t| (t.slug, parse_theme_css(t.source).ok()))
                .collect()
        })
        .iter()
        .find(|(s, _)| *s == slug)
        .and_then(|(_, p)| p.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(pairs: &[(&str, &str)]) -> Decls {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn parses_light_and_dark_blocks() {
        let p = parse_theme_css(
            ":root { --background: oklch(1 0 0); --radius: 0.625rem; }
             .dark { --background: oklch(0.145 0 0); }",
        )
        .unwrap();
        assert_eq!(
            p.light,
            d(&[("--background", "oklch(1 0 0)"), ("--radius", "0.625rem")])
        );
        assert_eq!(p.dark, Some(d(&[("--background", "oklch(0.145 0 0)")])));
    }

    #[test]
    fn dark_is_optional() {
        let p = parse_theme_css(":root { --primary: red; }").unwrap();
        assert_eq!(p.dark, None);
    }

    #[test]
    fn ignores_other_rules_including_nested_at_rules() {
        let p = parse_theme_css(
            "@theme inline { --color-background: var(--background); }
             @layer base { * { @apply border-border; } body { --not-a-token: 1; } }
             :root { --background: #fff; }
             .dark { --background: #000; }
             .other { --background: hotpink; }",
        )
        .unwrap();
        assert_eq!(p.light, d(&[("--background", "#fff")]));
        assert_eq!(p.dark, Some(d(&[("--background", "#000")])));
    }

    #[test]
    fn statements_before_a_block_do_not_hide_it() {
        let p = parse_theme_css(
            "@import url(\"https://fonts.example/css?family=Inter\");
             @import \"tailwindcss\";
             :root { --a: 1; }",
        )
        .unwrap();
        assert_eq!(p.light, d(&[("--a", "1")]));
    }

    #[test]
    fn strips_comments_and_keeps_non_colour_tokens() {
        let p = parse_theme_css(
            "/* :root { --evil: 1; } */
             :root {
               --font-sans: Inter, \"Helvetica Neue\", sans-serif; /* body */
               --shadow-sm: 0 1px 2px 0 hsl(0 0% 0% / 0.05);
               --tracking-normal: 0em;
             }",
        )
        .unwrap();
        assert_eq!(
            p.light,
            d(&[
                ("--font-sans", "Inter, \"Helvetica Neue\", sans-serif"),
                ("--shadow-sm", "0 1px 2px 0 hsl(0 0% 0% / 0.05)"),
                ("--tracking-normal", "0em"),
            ])
        );
    }

    #[test]
    fn semicolons_inside_parens_and_quotes_do_not_split() {
        let p = parse_theme_css(":root { --a: url(\"x;y\"); --b: 'p;q'; --c: 1; }").unwrap();
        assert_eq!(
            p.light,
            d(&[("--a", "url(\"x;y\")"), ("--b", "'p;q'"), ("--c", "1")])
        );
    }

    #[test]
    fn wraps_legacy_hsl_channel_values() {
        let p = parse_theme_css(
            ":root { --background: 0 0% 100%; --ring: 222.2 84% 4.9% / 0.5; --x: 10px 20px; }",
        )
        .unwrap();
        assert_eq!(
            p.light,
            d(&[
                ("--background", "hsl(0 0% 100%)"),
                ("--ring", "hsl(222.2 84% 4.9% / 0.5)"),
                ("--x", "10px 20px")
            ])
        );
    }

    #[test]
    fn later_duplicates_win_and_blocks_merge() {
        let p = parse_theme_css(":root { --a: 1; --b: 2; } :root { --a: 3; }").unwrap();
        assert_eq!(p.light, d(&[("--a", "3"), ("--b", "2")]));
    }

    #[test]
    fn rejects_a_file_with_no_root_block() {
        assert!(parse_theme_css(".dark { --a: 1; }").is_err());
        assert!(parse_theme_css(":root { color: red; }").is_err());
        assert!(parse_theme_css(":root { --a: 1; ").is_err());
    }

    #[test]
    fn overlay_replaces_and_adds() {
        let mut t = ThemeTokens {
            light: d(&[("--a", "1"), ("--b", "2")]),
            dark: d(&[("--a", "9")]),
        };
        t.overlay(&ParsedTheme {
            light: d(&[("--b", "3"), ("--c", "4")]),
            dark: None,
        });
        assert_eq!(t.light, d(&[("--a", "1"), ("--b", "3"), ("--c", "4")]));
        assert_eq!(t.dark, d(&[("--a", "9")]));
    }

    #[test]
    fn scoped_css_has_a_light_and_a_dark_rule() {
        let css = scoped_css(
            "Rose Garden!",
            &d(&[("--primary", "red")]),
            &d(&[("--primary", "blue")]),
        );
        assert_eq!(
            css,
            "[data-theme=\"rose-garden\"]{--primary:red;}\
             .dark[data-theme=\"rose-garden\"],.dark [data-theme=\"rose-garden\"],\
             :root[data-theme=\"rose-garden\"][data-vdl-menu-color^=\"inverted\"] \
             :is(.vdl-menu,.vdl-select__content,.vdl-combobox__content){--primary:blue;}"
        );
        // No dark tokens, no dark rule.
        assert!(!scoped_css("x", &d(&[("--a", "1")]), &Vec::new()).contains(".dark"));
    }

    #[test]
    fn scoped_css_cannot_be_broken_out_of() {
        let css = scoped_css(
            "x\"] {} body{",
            &d(&[("--a", "red;}</style><b>{")]),
            &Vec::new(),
        );
        assert!(!css.contains('<'));
        assert_eq!(css.matches('{').count(), 1, "{css}");
        assert_eq!(css.matches('}').count(), 1, "{css}");
    }

    #[test]
    fn slugs() {
        assert_eq!(slugify("My Brand!"), "my-brand");
        assert_eq!(slugify("  --a--b-- "), "a-b");
        assert_eq!(slugify("***"), "");
    }

    #[test]
    fn built_in_themes_parse() {
        let neutral = ThemeTokens::neutral();
        assert!(neutral.light("--primary").is_some());
        assert!(neutral.dark("--primary").is_some());
        for t in THEMES {
            assert!(builtin_theme(t.slug).is_some(), "{}", t.slug);
        }
        assert!(builtin_theme("gray").is_none());
    }
}
