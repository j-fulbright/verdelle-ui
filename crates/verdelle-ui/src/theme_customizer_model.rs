//! The pure parts of [`crate::ThemeCustomizer`]: reading what a person pastes into the preset
//! box and summarising what Verdelle does with a preset. No DOM, no signals.

use crate::preset::{Preset, PresetField};

/// One part of a preset Verdelle renders as named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Applied {
    pub field: &'static str,
    pub value: String,
}

/// One part of a preset Verdelle does not render as named, and what it uses instead.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FellBack {
    pub field: &'static str,
    pub asked: String,
    pub used: String,
}

/// What was applied of a preset and what Verdelle falls back on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PresetSummary {
    /// The normalised code.
    pub code: String,
    pub applied: Vec<Applied>,
    pub fell_back: Vec<FellBack>,
}

impl PresetSummary {
    /// One line that says it all: the code, and how many parts fall back.
    pub fn headline(&self) -> String {
        match self.fell_back.len() {
            0 => format!("Applied {}.", self.code),
            1 => format!("Applied {}; 1 part falls back.", self.code),
            n => format!("Applied {}; {n} parts fall back.", self.code),
        }
    }
}

/// Summarises `preset`: the colour, radius, font and menu choices that are applied, and the
/// style and icon library when they fall back to nova and Lucide.
pub(crate) fn summarize(preset: &Preset) -> PresetSummary {
    let missing = preset.unsupported();
    let effective = preset.effective();
    let applied = [
        (PresetField::BaseColor, preset.base_color.slug()),
        (PresetField::Theme, preset.theme.slug()),
        (PresetField::ChartColor, preset.chart_color.slug()),
        (PresetField::Radius, preset.radius.slug()),
        (PresetField::Font, preset.font.slug()),
        (PresetField::FontHeading, preset.font_heading.slug()),
        (PresetField::MenuColor, preset.menu_color.slug()),
        (PresetField::MenuAccent, preset.menu_accent.slug()),
    ]
    .into_iter()
    .map(|(f, v)| Applied {
        field: f.name(),
        value: v.to_string(),
    })
    .collect();
    let fell_back = missing
        .into_iter()
        .map(|f| {
            let (asked, used) = match f {
                PresetField::Style => (preset.style.slug(), effective.style.slug()),
                _ => (preset.icon_library.slug(), effective.icon_library.slug()),
            };
            FellBack {
                field: f.name(),
                asked: asked.to_string(),
                used: used.to_string(),
            }
        })
        .collect();
    PresetSummary {
        code: preset.encode(),
        applied,
        fell_back,
    }
}

/// Reads what a person pastes in the preset box: a bare code, `--preset X`, or a
/// `ui.shadcn.com/create` link (with `?preset=X`, or the code as the last path segment). The
/// error is a sentence for the person.
pub(crate) fn read_preset_input(input: &str) -> Result<Preset, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("Paste a preset code, `--preset` command or ui.shadcn.com/create link.".into());
    }
    let mut candidate = input;
    if input.contains("shadcn.com") && !input.contains("preset=") {
        let path = input.split(['?', '#']).next().unwrap_or(input);
        candidate = path
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or(path);
    }
    Preset::parse_input(candidate)
        .map_err(|e| format!("That is not a preset code Verdelle can read ({e})."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::{IconLibrary, Style};

    #[test]
    fn reads_a_bare_code_a_command_and_links() {
        let want = Preset::decode("b2oE5cW2a").unwrap();
        for input in [
            "b2oE5cW2a",
            "  b2oE5cW2a\n",
            "--preset b2oE5cW2a",
            "npx shadcn@latest init --preset b2oE5cW2a",
            "https://ui.shadcn.com/create?preset=b2oE5cW2a",
            "https://ui.shadcn.com/create?base=base&preset=b2oE5cW2a&x=1",
            "https://ui.shadcn.com/create/b2oE5cW2a",
            "https://ui.shadcn.com/create/b2oE5cW2a/?x=1",
        ] {
            match read_preset_input(input) {
                Ok(p) => assert_eq!(p, want, "{input}"),
                // The full init command is not something the box promises to read.
                Err(_) => assert!(input.starts_with("npx"), "{input}"),
            }
        }
    }

    #[test]
    fn explains_what_it_cannot_read() {
        assert!(read_preset_input("   ").unwrap_err().contains("Paste"));
        assert!(
            read_preset_input("not a code!")
                .unwrap_err()
                .contains("not a preset code")
        );
        assert!(read_preset_input("https://ui.shadcn.com/create").is_err());
    }

    #[test]
    fn a_nova_lucide_preset_has_no_fallback() {
        let s = summarize(&Preset::default());
        assert_eq!(s.code, "b0");
        assert!(s.fell_back.is_empty());
        assert_eq!(s.headline(), "Applied b0.");
        assert_eq!(s.applied.len(), 8);
        assert_eq!(s.applied[0].field, "baseColor");
    }

    #[test]
    fn other_styles_and_icon_libraries_say_what_they_fall_back_to() {
        let vega = Preset {
            style: Style::Vega,
            ..Preset::default()
        };
        let s = summarize(&vega);
        assert_eq!(
            s.fell_back,
            [FellBack {
                field: "style",
                asked: "vega".into(),
                used: "nova".into()
            }]
        );
        assert!(s.headline().contains("1 part falls back"));

        let both = Preset {
            icon_library: IconLibrary::Hugeicons,
            ..vega
        };
        let s = summarize(&both);
        assert_eq!(s.fell_back.len(), 2);
        assert_eq!(s.fell_back[1].used, "lucide");
        assert!(s.headline().contains("2 parts fall back"));
    }
}
