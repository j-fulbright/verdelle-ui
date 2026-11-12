//! shadcn preset codes: the short string (`b2oE5cW2a`) that shadcn's `/create` page and
//! `shadcn init --preset` use to name a whole design-system configuration.
//!
//! A code is a version letter (`a` or `b`) followed by a base-62 number. The number packs one
//! index per field, lowest bits first, in the order of [`Preset::FIELDS`]: menu colour, menu
//! accent, radius, font, icon library, theme, base colour, style, chart colour, heading font
//! (version `a` stops after style). This module mirrors `packages/registry/src/preset/preset.ts`
//! in shadcn-ui/ui (MIT, Copyright (c) 2023 shadcn): the value lists never reorder, a new
//! value is only ever appended, and index 0 is every field's default.
//!
//! It is pure Rust with no dependencies, so it also works on the server and in tests.
//!
//! Verdelle renders only part of a preset. [`Preset::supported`] says which parts, and
//! [`Preset::effective`] replaces the rest with what Verdelle does render. Nothing here errors on
//! an unsupported value: decoding keeps it (so the code still round-trips), the styled
//! components just fall back.
//!
//! ```
//! use verdelle_ui::preset::{BaseColor, Preset};
//! let preset = Preset::decode("b2oE5cW2a").unwrap();
//! assert_eq!(preset.base_color, BaseColor::Zinc);
//! assert_eq!(preset.encode(), "b2oE5cW2a");
//! ```

use std::fmt;

const BASE62: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Declares a field's value list: a `Copy` enum whose first variant is the default, with the
/// `slug` shadcn uses for it (in preset codes' value lists, URLs and the registry).
macro_rules! preset_enum {
    ($(#[$meta:meta])* $name:ident { $($variant:ident = $slug:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name { $($variant),+ }

        impl $name {
            /// Every value, in shadcn's order (the order that gives the bit pattern).
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The name shadcn uses for this value (`"dm-sans"`).
            pub const fn slug(self) -> &'static str {
                match self { $($name::$variant => $slug),+ }
            }

            /// The value with this slug, if there is one.
            pub fn from_slug(slug: &str) -> Option<$name> {
                match slug { $($slug => Some($name::$variant),)+ _ => None }
            }

            /// Position in [`Self::ALL`], the number that goes into the code.
            pub fn index(self) -> usize {
                Self::ALL.iter().position(|v| *v == self).unwrap_or(0)
            }

            /// The value at `index`, if there is one.
            pub fn from_index(index: usize) -> Option<$name> {
                Self::ALL.get(index).copied()
            }

            /// A display name: the slug in title case (`"default-translucent"` becomes
            /// `"Default Translucent"`).
            pub fn label(self) -> String { title_case(self.slug()) }
        }

        impl Default for $name {
            /// Index 0, as in shadcn.
            fn default() -> Self { Self::ALL[0] }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.slug()) }
        }
    };
}

pub(crate) fn title_case(slug: &str) -> String {
    slug.split('-')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

preset_enum! {
    /// How the menus and popups are coloured (`data-vdl-menu-color` on `<html>`).
    MenuColor {
        Default = "default",
        Inverted = "inverted",
        DefaultTranslucent = "default-translucent",
        InvertedTranslucent = "inverted-translucent",
    }
}

preset_enum! {
    /// How the highlighted menu item is coloured (`data-vdl-menu-accent` on `<html>`).
    MenuAccent { Subtle = "subtle", Bold = "bold" }
}

preset_enum! {
    /// The corner radius. `Default` keeps the theme's own `--radius` (0.625rem).
    Radius {
        Default = "default",
        None = "none",
        Small = "small",
        Medium = "medium",
        Large = "large",
    }
}

preset_enum! {
    /// shadcn's component styles. Verdelle renders [`Style::Nova`] only.
    Style {
        Nova = "nova",
        Vega = "vega",
        Maia = "maia",
        Lyra = "lyra",
        Mira = "mira",
        Luma = "luma",
        Sera = "sera",
        Rhea = "rhea",
    }
}

preset_enum! {
    /// The icon library. Verdelle's icons are Lucide ([`crate::Icon`]); the others fall back to it.
    IconLibrary {
        Lucide = "lucide",
        Hugeicons = "hugeicons",
        Tabler = "tabler",
        Phosphor = "phosphor",
        Remixicon = "remixicon",
    }
}

preset_enum! {
    /// A neutral palette that the whole token set is built on. `Gray` is in shadcn's list but
    /// shadcn no longer ships tokens for it; Verdelle treats it as `Neutral`.
    BaseColor {
        Neutral = "neutral",
        Stone = "stone",
        Zinc = "zinc",
        Gray = "gray",
        Mauve = "mauve",
        Olive = "olive",
        Mist = "mist",
        Taupe = "taupe",
    }
}

preset_enum! {
    /// The accent theme (primary colour and friends), layered over the base colour. A base
    /// colour's own name picks "no accent". Also the type of a preset's chart colour.
    Theme {
        Neutral = "neutral",
        Stone = "stone",
        Zinc = "zinc",
        Gray = "gray",
        Amber = "amber",
        Blue = "blue",
        Cyan = "cyan",
        Emerald = "emerald",
        Fuchsia = "fuchsia",
        Green = "green",
        Indigo = "indigo",
        Lime = "lime",
        Orange = "orange",
        Pink = "pink",
        Purple = "purple",
        Red = "red",
        Rose = "rose",
        Sky = "sky",
        Teal = "teal",
        Violet = "violet",
        Yellow = "yellow",
        Mauve = "mauve",
        Olive = "olive",
        Mist = "mist",
        Taupe = "taupe",
    }
}

preset_enum! {
    /// A font family. Verdelle does not bundle fonts: the app loads the files (see
    /// [`Font::google_fonts_url`]) and Verdelle sets `--font-sans` and friends.
    Font {
        Inter = "inter",
        NotoSans = "noto-sans",
        NunitoSans = "nunito-sans",
        Figtree = "figtree",
        Roboto = "roboto",
        Raleway = "raleway",
        DmSans = "dm-sans",
        PublicSans = "public-sans",
        Outfit = "outfit",
        JetbrainsMono = "jetbrains-mono",
        Geist = "geist",
        GeistMono = "geist-mono",
        Lora = "lora",
        Merriweather = "merriweather",
        PlayfairDisplay = "playfair-display",
        NotoSerif = "noto-serif",
        RobotoSlab = "roboto-slab",
        Oxanium = "oxanium",
        Manrope = "manrope",
        SpaceGrotesk = "space-grotesk",
        Montserrat = "montserrat",
        IbmPlexSans = "ibm-plex-sans",
        SourceSans3 = "source-sans-3",
        InstrumentSans = "instrument-sans",
        EbGaramond = "eb-garamond",
        InstrumentSerif = "instrument-serif",
    }
}

/// Whether a font is sans, serif or monospace, which decides its generic fallback and the
/// CSS variable it is kept in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontKind {
    Sans,
    Serif,
    Mono,
}

impl Font {
    /// The family's proper name (`"DM Sans"`).
    pub fn title(self) -> &'static str {
        match self {
            Font::DmSans => "DM Sans",
            Font::IbmPlexSans => "IBM Plex Sans",
            Font::EbGaramond => "EB Garamond",
            Font::JetbrainsMono => "JetBrains Mono",
            Font::SourceSans3 => "Source Sans 3",
            Font::NotoSans => "Noto Sans",
            Font::NunitoSans => "Nunito Sans",
            Font::PublicSans => "Public Sans",
            Font::NotoSerif => "Noto Serif",
            Font::RobotoSlab => "Roboto Slab",
            Font::PlayfairDisplay => "Playfair Display",
            Font::SpaceGrotesk => "Space Grotesk",
            Font::InstrumentSans => "Instrument Sans",
            Font::InstrumentSerif => "Instrument Serif",
            Font::GeistMono => "Geist Mono",
            Font::Inter => "Inter",
            Font::Figtree => "Figtree",
            Font::Roboto => "Roboto",
            Font::Raleway => "Raleway",
            Font::Outfit => "Outfit",
            Font::Geist => "Geist",
            Font::Lora => "Lora",
            Font::Merriweather => "Merriweather",
            Font::Oxanium => "Oxanium",
            Font::Manrope => "Manrope",
            Font::Montserrat => "Montserrat",
        }
    }

    pub fn kind(self) -> FontKind {
        match self {
            Font::JetbrainsMono | Font::GeistMono => FontKind::Mono,
            Font::Lora
            | Font::Merriweather
            | Font::PlayfairDisplay
            | Font::NotoSerif
            | Font::RobotoSlab
            | Font::EbGaramond
            | Font::InstrumentSerif => FontKind::Serif,
            _ => FontKind::Sans,
        }
    }

    /// The CSS variable shadcn keeps this font in: `--font-sans`, `--font-serif` or `--font-mono`.
    pub fn variable(self) -> &'static str {
        match self.kind() {
            FontKind::Sans => "--font-sans",
            FontKind::Serif => "--font-serif",
            FontKind::Mono => "--font-mono",
        }
    }

    /// A `font-family` value: the name as Google Fonts spells it, the name fontsource's
    /// variable packages use (`'Inter Variable'`, which is what shadcn's own CSS uses), then
    /// system fonts of the same kind so text is fine before the font file arrives or when the
    /// app does not load it.
    pub fn stack(self) -> String {
        let t = self.title();
        let fallback = match self.kind() {
            FontKind::Sans => "ui-sans-serif, system-ui, sans-serif",
            FontKind::Serif => "ui-serif, Georgia, serif",
            FontKind::Mono => "ui-monospace, SFMono-Regular, Menlo, monospace",
        };
        if self == Font::InstrumentSerif {
            // Fontsource ships this one as a static font, so there is no "Variable" name.
            format!("'{t}', {fallback}")
        } else {
            format!("'{t}', '{t} Variable', {fallback}")
        }
    }

    /// The `family=` part of a Google Fonts CSS2 URL (`"Inter:wght@400;500;600;700"`).
    pub fn google_family(self) -> String {
        let name = self.title().replace(' ', "+");
        // Instrument Serif exists in regular only; every other font here has 400 to 700.
        if self == Font::InstrumentSerif {
            format!("{name}:wght@400")
        } else {
            format!("{name}:wght@400;500;600;700")
        }
    }

    /// A Google Fonts stylesheet URL that loads this font; one `<link rel="stylesheet">` in
    /// the app's `<head>`. See also [`Preset::google_fonts_url`].
    pub fn google_fonts_url(self) -> String {
        google_fonts_url([self])
    }
}

/// A `fonts.googleapis.com` URL loading all of `fonts` (duplicates are dropped).
pub fn google_fonts_url(fonts: impl IntoIterator<Item = Font>) -> String {
    let mut seen: Vec<Font> = Vec::new();
    for f in fonts {
        if !seen.contains(&f) {
            seen.push(f);
        }
    }
    let families: Vec<String> = seen
        .into_iter()
        .map(|f| format!("family={}", f.google_family()))
        .collect();
    format!(
        "https://fonts.googleapis.com/css2?{}&display=swap",
        families.join("&")
    )
}

/// The heading font: the body font (`Inherit`, the default) or its own family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FontHeading {
    #[default]
    Inherit,
    Font(Font),
}

impl FontHeading {
    pub fn slug(self) -> &'static str {
        match self {
            FontHeading::Inherit => "inherit",
            FontHeading::Font(f) => f.slug(),
        }
    }

    pub fn from_slug(slug: &str) -> Option<FontHeading> {
        if slug == "inherit" {
            Some(FontHeading::Inherit)
        } else {
            Font::from_slug(slug).map(FontHeading::Font)
        }
    }

    /// 0 for `Inherit`, then each font's index plus one.
    pub fn index(self) -> usize {
        match self {
            FontHeading::Inherit => 0,
            FontHeading::Font(f) => f.index() + 1,
        }
    }

    pub fn from_index(index: usize) -> Option<FontHeading> {
        match index {
            0 => Some(FontHeading::Inherit),
            n => Font::from_index(n - 1).map(FontHeading::Font),
        }
    }

    pub fn label(self) -> String {
        match self {
            FontHeading::Inherit => "Inherit".to_string(),
            FontHeading::Font(f) => f.title().to_string(),
        }
    }

    /// Every value: `Inherit`, then every font.
    pub fn all() -> impl Iterator<Item = FontHeading> {
        std::iter::once(FontHeading::Inherit).chain(Font::ALL.iter().map(|f| FontHeading::Font(*f)))
    }
}

impl fmt::Display for FontHeading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
    }
}

/// Why a string is not a preset code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresetError {
    /// Shorter than a version letter plus one digit.
    TooShort,
    /// Longer than any code shadcn makes (10 characters).
    TooLong,
    /// Does not start with `a` or `b`.
    UnknownVersion,
    /// Contains something other than 0-9, A-Z, a-z after the version letter.
    InvalidCharacter,
}

impl fmt::Display for PresetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PresetError::TooShort => "a preset code needs a version letter and at least one digit",
            PresetError::TooLong => "a preset code has at most 10 characters",
            PresetError::UnknownVersion => "a preset code starts with `a` or `b`",
            PresetError::InvalidCharacter => "a preset code holds only letters and digits",
        })
    }
}

impl std::error::Error for PresetError {}

/// One field of a preset, for reporting ([`Preset::unsupported`]) and for [`Preset::FIELDS`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresetField {
    MenuColor,
    MenuAccent,
    Radius,
    Font,
    IconLibrary,
    Theme,
    BaseColor,
    Style,
    ChartColor,
    FontHeading,
}

impl PresetField {
    /// The name shadcn gives the field (`"iconLibrary"`).
    pub fn name(self) -> &'static str {
        match self {
            PresetField::MenuColor => "menuColor",
            PresetField::MenuAccent => "menuAccent",
            PresetField::Radius => "radius",
            PresetField::Font => "font",
            PresetField::IconLibrary => "iconLibrary",
            PresetField::Theme => "theme",
            PresetField::BaseColor => "baseColor",
            PresetField::Style => "style",
            PresetField::ChartColor => "chartColor",
            PresetField::FontHeading => "fontHeading",
        }
    }
}

/// What Verdelle renders of a preset today.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresetSupport {
    /// Verdelle only has the `nova` style (shadcn's default).
    pub style: bool,
    /// Verdelle only has Lucide icons.
    pub icon_library: bool,
}

impl PresetSupport {
    /// True when the preset renders as named.
    pub fn is_complete(&self) -> bool {
        self.style && self.icon_library
    }
}

/// A shadcn design-system configuration. `Preset::default()` is every field at index 0 (the
/// code `b0`): style nova, neutral, Inter, Lucide, default radius, default subtle menus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Preset {
    pub menu_color: MenuColor,
    pub menu_accent: MenuAccent,
    pub radius: Radius,
    pub font: Font,
    pub icon_library: IconLibrary,
    pub theme: Theme,
    pub base_color: BaseColor,
    pub style: Style,
    pub chart_color: Theme,
    pub font_heading: FontHeading,
}

/// shadcn's `V1_CHART_COLOR_MAP`: before version `b`, a base colour's charts were coloured
/// like one of the accent themes.
fn v1_chart_color(theme: Theme) -> Theme {
    match theme {
        Theme::Neutral => Theme::Blue,
        Theme::Stone => Theme::Lime,
        Theme::Zinc => Theme::Amber,
        Theme::Mauve => Theme::Emerald,
        Theme::Olive => Theme::Violet,
        Theme::Mist => Theme::Rose,
        Theme::Taupe => Theme::Cyan,
        other => other,
    }
}

impl Preset {
    /// The fields in code order, each with its width in bits (version `b`; version `a` has the
    /// first eight).
    pub const FIELDS: [(PresetField, u32); 10] = [
        (PresetField::MenuColor, 3),
        (PresetField::MenuAccent, 3),
        (PresetField::Radius, 4),
        (PresetField::Font, 6),
        (PresetField::IconLibrary, 6),
        (PresetField::Theme, 6),
        (PresetField::BaseColor, 6),
        (PresetField::Style, 6),
        (PresetField::ChartColor, 6),
        (PresetField::FontHeading, 5),
    ];

    /// Reads a code, as shadcn's `decodePreset`. An index past the end of a field's list
    /// becomes the field's default; a version `a` code gets the chart colour shadcn restores
    /// for it and an inherited heading font. Surrounding whitespace is ignored.
    pub fn decode(code: &str) -> Result<Preset, PresetError> {
        let code = code.trim();
        if code.len() < 2 {
            return Err(PresetError::TooShort);
        }
        if code.len() > 10 {
            return Err(PresetError::TooLong);
        }
        let version = code.as_bytes()[0];
        if version != b'a' && version != b'b' {
            return Err(PresetError::UnknownVersion);
        }
        let mut bits: u64 = 0;
        for b in code.bytes().skip(1) {
            let digit = BASE62
                .iter()
                .position(|c| *c == b)
                .ok_or(PresetError::InvalidCharacter)? as u64;
            // Nine digits stay under 2^54, so this cannot overflow.
            bits = bits * 62 + digit;
        }
        let field_count = if version == b'a' { 8 } else { 10 };
        let mut idx = [0usize; 10];
        let mut offset = 0;
        for (i, (_, width)) in Self::FIELDS.iter().take(field_count).enumerate() {
            idx[i] = ((bits >> offset) & ((1 << width) - 1)) as usize;
            offset += width;
        }
        let theme = Theme::from_index(idx[5]).unwrap_or_default();
        Ok(Preset {
            menu_color: MenuColor::from_index(idx[0]).unwrap_or_default(),
            menu_accent: MenuAccent::from_index(idx[1]).unwrap_or_default(),
            radius: Radius::from_index(idx[2]).unwrap_or_default(),
            font: Font::from_index(idx[3]).unwrap_or_default(),
            icon_library: IconLibrary::from_index(idx[4]).unwrap_or_default(),
            theme,
            base_color: BaseColor::from_index(idx[6]).unwrap_or_default(),
            style: Style::from_index(idx[7]).unwrap_or_default(),
            chart_color: if version == b'a' {
                v1_chart_color(theme)
            } else {
                Theme::from_index(idx[8]).unwrap_or_default()
            },
            font_heading: if version == b'a' {
                FontHeading::Inherit
            } else {
                FontHeading::from_index(idx[9]).unwrap_or_default()
            },
        })
    }

    /// Reads what a person would paste: a bare code, `--preset b2oE5cW2a` (the CLI flag), or a
    /// link with a `preset=` query parameter (the `/create` page's URL).
    pub fn parse_input(input: &str) -> Result<Preset, PresetError> {
        let input = input.trim();
        let candidate = if let Some(rest) = input.strip_prefix("--preset") {
            rest.trim()
        } else if let Some((_, query)) = input.split_once('?') {
            query
                .split('&')
                .find_map(|kv| kv.strip_prefix("preset="))
                .unwrap_or(input)
        } else {
            input
        };
        Preset::decode(candidate)
    }

    /// The version `b` code for this preset, as shadcn's `encodePreset`.
    pub fn encode(&self) -> String {
        let idx = [
            self.menu_color.index(),
            self.menu_accent.index(),
            self.radius.index(),
            self.font.index(),
            self.icon_library.index(),
            self.theme.index(),
            self.base_color.index(),
            self.style.index(),
            self.chart_color.index(),
            self.font_heading.index(),
        ];
        let mut bits: u64 = 0;
        let mut offset = 0;
        for (i, (_, width)) in Self::FIELDS.iter().enumerate() {
            bits += (idx[i] as u64) << offset;
            offset += width;
        }
        let mut digits = Vec::new();
        if bits == 0 {
            digits.push(b'0');
        }
        while bits > 0 {
            digits.push(BASE62[(bits % 62) as usize]);
            bits /= 62;
        }
        digits.push(b'b');
        digits.reverse();
        String::from_utf8(digits).unwrap_or_default()
    }

    /// Whether `s` looks like a code (shadcn's `isPresetCode`): a version letter and base-62
    /// digits, at most 10 characters.
    pub fn is_code(s: &str) -> bool {
        Preset::decode(s).is_ok()
    }

    /// Which parts Verdelle renders as named. Style and icon library are the only ones that can
    /// be unsupported: colours, radius, fonts and menus are all applied.
    pub fn supported(&self) -> PresetSupport {
        PresetSupport {
            style: self.style == Style::Nova,
            icon_library: self.icon_library == IconLibrary::Lucide,
        }
    }

    /// The parts [`Self::supported`] says are not rendered as named.
    pub fn unsupported(&self) -> Vec<PresetField> {
        let s = self.supported();
        let mut out = Vec::new();
        if !s.style {
            out.push(PresetField::Style);
        }
        if !s.icon_library {
            out.push(PresetField::IconLibrary);
        }
        out
    }

    /// What Verdelle actually renders: this preset with the unsupported parts replaced by the
    /// ones it has (style `nova`, icons `lucide`). Never fails.
    pub fn effective(&self) -> Preset {
        Preset {
            style: Style::Nova,
            icon_library: IconLibrary::Lucide,
            ..*self
        }
    }

    /// A Google Fonts stylesheet URL for the body font and the heading font, if any.
    pub fn google_fonts_url(&self) -> String {
        let mut fonts = vec![self.font];
        if let FontHeading::Font(f) = self.font_heading {
            fonts.push(f);
        }
        google_fonts_url(fonts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_known_code() {
        let p = Preset::decode("b2oE5cW2a").unwrap();
        assert_eq!(p.menu_color, MenuColor::Default);
        assert_eq!(p.menu_accent, MenuAccent::Subtle);
        assert_eq!(p.radius, Radius::Default);
        assert_eq!(p.font, Font::Figtree);
        assert_eq!(p.icon_library, IconLibrary::Lucide);
        assert_eq!(p.theme, Theme::Zinc);
        assert_eq!(p.base_color, BaseColor::Zinc);
        assert_eq!(p.style, Style::Nova);
        assert_eq!(p.chart_color, Theme::Green);
        assert_eq!(p.font_heading, FontHeading::Inherit);
        assert_eq!(p.encode(), "b2oE5cW2a");
    }

    #[test]
    fn default_is_index_zero_everywhere() {
        let p = Preset::default();
        assert_eq!(p.encode(), "b0");
        assert_eq!(Preset::decode("b0").unwrap(), p);
        assert_eq!(p.style, Style::Nova);
        assert_eq!(p.font, Font::Inter);
        assert_eq!(p.font_heading, FontHeading::Inherit);
    }

    #[test]
    fn value_lists_match_shadcns_field_widths() {
        // A list longer than its field would silently lose values.
        assert!(MenuColor::ALL.len() <= 1 << 3);
        assert!(MenuAccent::ALL.len() <= 1 << 3);
        assert!(Radius::ALL.len() <= 1 << 4);
        assert!(Font::ALL.len() <= 1 << 6);
        assert!(IconLibrary::ALL.len() <= 1 << 6);
        assert!(Theme::ALL.len() <= 1 << 6);
        assert!(BaseColor::ALL.len() <= 1 << 6);
        assert!(Style::ALL.len() <= 1 << 6);
        assert!(FontHeading::all().count() <= 1 << 5);
        // The version b code is at most 51 bits: 9 base-62 digits plus the letter.
        let widths: u32 = Preset::FIELDS.iter().map(|(_, w)| w).sum();
        assert_eq!(widths, 51);
    }

    #[test]
    fn slugs_round_trip() {
        for t in Theme::ALL {
            assert_eq!(Theme::from_slug(t.slug()), Some(*t));
        }
        for f in FontHeading::all() {
            assert_eq!(FontHeading::from_slug(f.slug()), Some(f));
            assert_eq!(FontHeading::from_index(f.index()), Some(f));
        }
        assert_eq!(Font::from_slug("nope"), None);
        assert_eq!(
            MenuColor::InvertedTranslucent.label(),
            "Inverted Translucent"
        );
    }

    #[test]
    fn every_value_of_every_field_round_trips() {
        let base = Preset::default();
        for v in MenuColor::ALL {
            let p = Preset {
                menu_color: *v,
                ..base
            };
            assert_eq!(Preset::decode(&p.encode()).unwrap(), p);
        }
        for v in Radius::ALL {
            let p = Preset { radius: *v, ..base };
            assert_eq!(Preset::decode(&p.encode()).unwrap(), p);
        }
        for v in FontHeading::all() {
            let p = Preset {
                font_heading: v,
                ..base
            };
            assert_eq!(Preset::decode(&p.encode()).unwrap(), p);
        }
        for v in Theme::ALL {
            let p = Preset {
                chart_color: *v,
                theme: *v,
                ..base
            };
            assert_eq!(Preset::decode(&p.encode()).unwrap(), p);
        }
        let all = Preset {
            menu_color: MenuColor::InvertedTranslucent,
            menu_accent: MenuAccent::Bold,
            radius: Radius::Large,
            font: Font::InstrumentSerif,
            icon_library: IconLibrary::Remixicon,
            theme: Theme::Taupe,
            base_color: BaseColor::Taupe,
            style: Style::Rhea,
            chart_color: Theme::Taupe,
            font_heading: FontHeading::Font(Font::InstrumentSerif),
        };
        assert_eq!(Preset::decode(&all.encode()).unwrap(), all);
    }

    #[test]
    fn rejects_what_is_not_a_code() {
        assert_eq!(Preset::decode(""), Err(PresetError::TooShort));
        assert_eq!(Preset::decode("b"), Err(PresetError::TooShort));
        assert_eq!(Preset::decode("c123"), Err(PresetError::UnknownVersion));
        assert_eq!(Preset::decode("b12-4"), Err(PresetError::InvalidCharacter));
        assert_eq!(Preset::decode("b12345678901"), Err(PresetError::TooLong));
        assert!(!Preset::is_code("hello"));
        assert!(Preset::is_code(" b2oE5cW2a "));
    }

    #[test]
    fn version_a_codes_get_restored_defaults() {
        // "a" stops after style: chart colour comes from shadcn's v1 map, heading inherits.
        let p = Preset::decode("a2oE5c").unwrap();
        assert_eq!(p.font_heading, FontHeading::Inherit);
        let zinc = Preset {
            theme: Theme::Zinc,
            ..Preset::default()
        };
        let coded = Preset::decode(&format!("a{}", &zinc.encode()[1..])).unwrap();
        assert_eq!(coded.theme, Theme::Zinc);
        assert_eq!(coded.chart_color, Theme::Amber);
        // Re-encoding gives a version b code.
        assert!(coded.encode().starts_with('b'));
    }

    #[test]
    fn out_of_range_indexes_fall_back_to_default() {
        // All bits set: every index is past its list except where the list is full.
        let p = Preset::decode("bZZZZZZZZ").unwrap();
        assert_eq!(p.style, Style::Nova);
        assert_eq!(p.icon_library, IconLibrary::Lucide);
    }

    #[test]
    fn parse_input_accepts_flags_and_links() {
        let want = Preset::decode("b2oE5cW2a").unwrap();
        assert_eq!(Preset::parse_input("  b2oE5cW2a ").unwrap(), want);
        assert_eq!(Preset::parse_input("--preset b2oE5cW2a").unwrap(), want);
        assert_eq!(
            Preset::parse_input("https://ui.shadcn.com/create?base=base&preset=b2oE5cW2a").unwrap(),
            want
        );
        assert!(Preset::parse_input("not a code").is_err());
    }

    #[test]
    fn support_names_what_verdelle_does_not_render() {
        let p = Preset::decode("b2oE5cW2a").unwrap();
        assert_eq!(p.style, Style::Nova);
        assert!(p.unsupported().is_empty());
        assert!(Preset::default().supported().is_complete());
        // The other seven styles, vega included, fall back to nova.
        let vega = Preset {
            style: Style::Vega,
            ..p
        };
        assert_eq!(vega.unsupported(), vec![PresetField::Style]);
        assert_eq!(vega.effective().style, Style::Nova);
        let odd = Preset {
            style: Style::Luma,
            icon_library: IconLibrary::Phosphor,
            ..p
        };
        assert_eq!(
            odd.unsupported(),
            vec![PresetField::Style, PresetField::IconLibrary]
        );
        let eff = odd.effective();
        assert_eq!(
            (eff.style, eff.icon_library),
            (Style::Nova, IconLibrary::Lucide)
        );
        // Colours and fonts are untouched.
        assert_eq!((eff.theme, eff.font), (odd.theme, odd.font));
        assert!(eff.supported().is_complete());
    }

    #[test]
    fn fonts_give_stacks_and_google_urls() {
        assert_eq!(
            Font::DmSans.stack(),
            "'DM Sans', 'DM Sans Variable', ui-sans-serif, system-ui, sans-serif"
        );
        assert!(
            Font::InstrumentSerif
                .stack()
                .starts_with("'Instrument Serif', ui-serif")
        );
        assert_eq!(Font::JetbrainsMono.variable(), "--font-mono");
        assert_eq!(Font::Lora.variable(), "--font-serif");
        assert_eq!(
            Font::Inter.google_fonts_url(),
            "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
        );
        let p = Preset {
            font: Font::Inter,
            font_heading: FontHeading::Font(Font::PlayfairDisplay),
            ..Preset::default()
        };
        assert!(
            p.google_fonts_url()
                .contains("family=Playfair+Display:wght@400;500;600;700")
        );
        // Heading equal to the body font is listed once.
        let same = Preset {
            font_heading: FontHeading::Font(Font::Inter),
            ..p
        };
        assert_eq!(same.google_fonts_url(), Font::Inter.google_fonts_url());
    }

    /// Codes made by shadcn's own `encodePreset` / `decodePreset` (run under node from
    /// shadcn-ui/ui's `packages/registry/src/preset/preset.ts`): every field at every value,
    /// 80 random presets, and a few odd codes.
    #[test]
    fn agrees_with_shadcns_own_functions() {
        #[derive(serde::Deserialize)]
        struct Config {
            style: String,
            #[serde(rename = "baseColor")]
            base_color: String,
            theme: String,
            #[serde(rename = "chartColor")]
            chart_color: Option<String>,
            #[serde(rename = "iconLibrary")]
            icon_library: String,
            font: String,
            #[serde(rename = "fontHeading")]
            font_heading: String,
            radius: String,
            #[serde(rename = "menuAccent")]
            menu_accent: String,
            #[serde(rename = "menuColor")]
            menu_color: String,
        }
        #[derive(serde::Deserialize)]
        struct Entry {
            code: String,
            config: Config,
        }
        #[derive(serde::Deserialize)]
        struct Fixture {
            codes: Vec<Entry>,
            raw: Vec<Entry>,
        }
        let fixture: Fixture =
            serde_json::from_str(include_str!("../tests/fixtures/preset_codes.json")).unwrap();
        assert!(fixture.codes.len() > 150);
        for e in fixture.codes.iter().chain(fixture.raw.iter()) {
            let c = &e.config;
            let want = Preset {
                menu_color: MenuColor::from_slug(&c.menu_color).unwrap(),
                menu_accent: MenuAccent::from_slug(&c.menu_accent).unwrap(),
                radius: Radius::from_slug(&c.radius).unwrap(),
                font: Font::from_slug(&c.font).unwrap(),
                icon_library: IconLibrary::from_slug(&c.icon_library).unwrap(),
                theme: Theme::from_slug(&c.theme).unwrap(),
                base_color: BaseColor::from_slug(&c.base_color).unwrap(),
                style: Style::from_slug(&c.style).unwrap(),
                // shadcn leaves chartColor out of a version a code and then maps it.
                chart_color: c
                    .chart_color
                    .as_deref()
                    .map(|s| Theme::from_slug(s).unwrap())
                    .unwrap_or_else(|| v1_chart_color(Theme::from_slug(&c.theme).unwrap())),
                font_heading: FontHeading::from_slug(&c.font_heading).unwrap(),
            };
            let got = Preset::decode(&e.code).unwrap();
            assert_eq!(got, want, "decoding {}", e.code);
        }
        // Codes shadcn itself encoded come back out of `encode` unchanged.
        for e in &fixture.codes {
            let got = Preset::decode(&e.code).unwrap();
            assert_eq!(got.encode(), e.code, "encoding {}", e.code);
        }
    }
}
