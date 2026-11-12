# Presets and themes

Verdelle themes are shadcn's: CSS custom properties (`--background`, `--primary`, `--radius`, ...) in a
`:root` block for light and a `.dark` block for dark. shadcn's `/create` page is the theme editor (there is
no Verdelle CLI for it); what it produces comes into Verdelle two ways.

## Preset codes

`/create` gives each design a short code such as `b2oE5cW2a`. `verdelle_ui::preset::Preset` reads and writes it
(pure Rust, mirrors `packages/registry/src/preset/preset.ts`, checked against shadcn's own functions run
under node: `crates/verdelle-ui/tests/fixtures/`).

```rust
let p = Preset::decode("b2oE5cW2a")?;          // or Preset::parse_input("--preset b2oE5cW2a")
p.base_color;  p.theme;  p.chart_color;  p.radius;  p.font;  p.font_heading;
p.menu_color;  p.menu_accent;  p.style;  p.icon_library;
p.encode();                                    // always a version "b" code
p.tokens();                                    // final light/dark token sets (ThemeTokens)
p.scoped_css("my-theme");                      // tokens + fonts as [data-theme="my-theme"] rules
p.google_fonts_url();                          // <link rel="stylesheet"> for the fonts
p.supported();  p.unsupported();  p.effective();
```

Every field is a typed enum with `ALL`, `slug()`, `from_slug()`, `label()`; `Preset::default()` is index 0
everywhere (`b0`, shadcn's default, which is the *nova* style).

**How tokens compose** (as shadcn's `buildRegistryTheme`): start from the neutral token set; lay the base colour
over it (a complete set); lay the accent theme over that (an accent theme such as rose changes primary,
secondary, chart and sidebar-primary only); take `--chart-1..5` from the chart colour; replace `--radius` if the
radius is not `default`. Missing tokens keep the neutral values; `gray` (which shadcn no longer ships) is neutral.
One deliberate difference: shadcn remaps `--accent` to `--primary` page-wide for a bold menu accent. Verdelle does
not: its CSS reads `data-vdl-menu-accent` and restyles only the highlighted menu items.
`verdelle_ui::remap_bold_accent` does shadcn's remap if you want it.

**What Verdelle renders.** Colours, radius, fonts and menu colour/accent are applied. Only the `nova` style (shadcn's default) and
`lucide` icons exist, so another style (vega, maia, lyra, mira, luma, sera, rhea) or icon library falls back to them: nothing errors, the code keeps its
values, and `unsupported()` says which parts fell back.

**Fonts.** Verdelle does not bundle fonts. A preset sets `--font-sans` (the body font whatever its kind),
`--font-serif`/`--font-mono` for such fonts, and `--font-heading`, with system fallbacks, and the page uses them.
Your app loads the files (`preset.google_fonts_url()`, or fontsource packages: the stack lists both the plain and
the `Variable` family names).

## Pasted shadcn CSS: `ThemeStyle`

```rust
<ThemeStyle name="ocean" css=include_str!("ocean.css") />
```

Reads the `:root` and `.dark` blocks (everything else in an export is ignored; every custom property passes
through) and writes rules scoped to `data-theme="ocean"` and `.dark[data-theme="ocean"]`. Missing tokens keep the
default theme's values. Inside a `ThemeProvider` it is also listed by `ThemeSwitcher`. Put it above the switcher.
An example export is `docs/examples/ocean-theme.css`. The parser is `verdelle_ui::theme_css::parse_theme_css`; the
`themegen` example (for the 24 built-in files and for an app's own `--themes <dir>`) uses the same one.

## `ThemeProvider`

Owns `data-theme` and the `.dark` class on `<html>`, plus `data-vdl-menu-color` / `data-vdl-menu-accent` from the
active preset (the menu CSS is separate). Put it once near the root.

```rust
<ThemeProvider preset="b2oE5cW2a" default_mode=ThemeMode::System>
    <ThemeSwitcher /> <ModeToggle /> <App/>
</ThemeProvider>
```

- Mode is light, dark or system (follows the OS live). `ModeToggle` and `ThemeSwitcher` find the provider.
- `preset` is offered as the theme `preset` and used until the person picks something else.
- **No flash**: an inline script applies the stored theme, mode, menu attributes and saved themes' CSS before
  first paint. The provider prints it; for the earliest paint pass `script=false` and put
  `theme_init_script(..)` in your document `<head>` (the gallery does). Values the server cannot know reach the
  signals in an effect after hydration, so server HTML and first client render agree.
- **Storage** is the `ThemeStorage` trait (`get`/`set`/`remove` of strings). The default `LocalStorage` uses
  `localStorage` through leptos-use-storage under the keys in `custom_theme::keys`; `SyncedStorage` (below)
  keeps them in memory and writes through to a server. Reads are synchronous. The plain no-flash script reads
  `localStorage`; an adapter that can be read on the server (`ThemeStorage::seed`) makes the provider print a
  script seeded from its values instead.

## Custom presets in the browser

`use_theme()` gives a `ThemeContext`:

| | |
|---|---|
| `try_theme(code, css)` / `try_preset(&p)` | apply as the unsaved `preview`, remembered across reloads |
| `save_theme(name, code, css)` / `save_preview(name)` | keep under a name (slug made from it, kept unique); chosen |
| `rename_theme(slug, name)` / `delete_theme(slug)` | edit the saved list |
| `export_themes()` / `import_themes(text, fallback_name)` | JSON of names, codes and pasted CSS; import also takes one code or one CSS block; all or nothing |
| `choices()`, `saved()`, `preview()`, `theme`, `mode`, `dark` | reactive state |

A theme is `code` and/or `css`; with both, the CSS overrides the preset's tokens. The built rules are stored with
it (for the no-flash script), the export holds only the inputs.

`active_custom()` is the chosen theme when the provider owns it (preview, saved, `preset`), `import_themes_max(text,
name, max)` is `import_themes` with a limit on the saved list.

The gallery's Themes page demonstrates all of it.

## An Appearance page

`ThemeCustomizer` is the whole panel in one component. It sits under the provider, takes the width it is given
(a page, a Card, a Tabs panel, a phone) and has a section per job, each switched with a `show_*` prop (all on by
default):

| Prop | |
|---|---|
| `show_theme` | `ThemeSwitcher` with swatches and a light / dark / system control |
| `show_preset` | code box (bare code, `--preset X` or a ui.shadcn.com/create link), Apply, a one-line summary with a collapsed list of what Verdelle falls back on (style other than nova, icon library other than Lucide), and a link to shadcn's creator |
| `show_fine_tuning` | base, accent and chart colours, radius, menu colour and accent, font and heading font; each change is applied as the preview |
| `show_css` | a textarea for shadcn CSS with a live preview and Apply |
| `show_saved` | saved themes (apply, rename, delete after a confirmation), "Save current as...", export and import as JSON text |
| `show_preview` | a small card: Button, Badge, Card, Input |
| `compact` | tighter spacing and no explanatory paragraphs (the line under each section title, the note about what falls back, the note under Save); labels, controls, errors and the preset summary stay. For a Settings panel |
| `max_saved` | the most themes a person may keep (saving and importing stop there) |
| `load_fonts` | add a `<link>` for the fonts of the preset in use (default false: your app loads fonts) |
| `on_apply`, `on_save` | `Callback<ThemeApplied>` (code, css, fonts URL) after something went on the page; `Callback<CustomTheme>` after saving |

Whatever is applied becomes the provider's unsaved `preview` theme (kept across reloads); saving names it. The
fine-tuning controls start from the chosen theme's preset when it has one (preview, saved, or the provider's
`preset`); with a built-in theme chosen they start from shadcn's default preset.

### Keeping a person's theme on a server

`SyncedStorage` is a `ThemeStorage` that keeps the values in memory, starting from what you already have, and calls
you on every change. Values arriving later replace the contents with `load`; the provider reads them again.

- `SyncedStorage::new(initial)`: `(key, value)` strings, the keys being `custom_theme::keys` (`THEME`, `MODE`,
  `CUSTOM`, `PREVIEW`).
- `.on_set(|key, value| ..)` and `.on_remove(|key| ..)`: write through. Setting a value it already has calls
  nothing, so what you `load` is not echoed back. The provider also does not write defaults the person never chose.
- `.debounce_ms(n)`: in the browser, changes wait `n` ms and each key is written once, with its last value.
  `flush()` writes now (call it before leaving the page if you must not lose a change).
- `load(values)`: replaces the contents without calling the hooks and tells the provider (theme, mode, saved themes
  and preview update). Changes still waiting for the debounce timer stay on top.
- `snapshot()`: a copy of the values; `into_arc()` is what the provider's `storage` prop takes.

**First paint.** The no-flash script runs before the page paints, so it must have the person's values already.
`SyncedStorage` can be read while rendering on the server, so build it from the values you loaded for this person:
the provider then starts its signals from them (the server's HTML has their theme and saved themes) and prints a
script that reads the storage's values instead of `localStorage`. If you would rather put the script in `<head>`,
pass `script=false` and print `theme_init_script_seeded(default_theme, default_mode, default_menu, &storage.snapshot())`
there. The values go into the script as JSON with `<`, `>` and `&` escaped, so saved CSS cannot close the element.
Build the storage from the same values on the server and in the browser (for example by passing them through a
resource) so the first client render matches the server's HTML; values that only arrive after hydration go
through `load`.

A copy-paste example (the server functions are yours):

```rust
use verdelle_ui::{SyncedStorage, ThemeCustomizer, ThemeMode, ThemeProvider};

#[component]
pub fn App() -> impl IntoView {
    // Settings loaded for this person (a server function / resource): [(key, value)].
    let saved = Resource::new(|| (), |_| load_theme_settings());

    let storage = SyncedStorage::new([])
        .on_set(|key, value| save_theme_setting(key.to_string(), value.to_string()))   // spawn_local a server fn
        .on_remove(|key| delete_theme_setting(key.to_string()))
        .debounce_ms(500);

    // Values arrive after hydration (or are known while rendering: pass them to `new` instead).
    Effect::new({
        let storage = storage.clone();
        move |_| {
            if let Some(values) = saved.get() {
                storage.load(values);
            }
        }
    });

    view! {
        <ThemeProvider storage=storage.into_arc() default_mode=ThemeMode::System>
            <Routes /* ... */ />
        </ThemeProvider>
    }
}

// The Settings > Appearance page (inside the provider):
#[component]
fn AppearancePage() -> impl IntoView {
    view! {
        <Card class="max-w-xl">
            <CardHeader><CardTitle>"Appearance"</CardTitle></CardHeader>
            <CardContent>
                <ThemeCustomizer compact=true max_saved=10usize />
            </CardContent>
        </Card>
    }
}
```

Load the preset's fonts with your own `<link rel="stylesheet" href=preset.google_fonts_url()>` or pass
`load_fonts=true` for the one in use. Do not add cookies for this: the server record is the source of truth and
the seeded script covers first paint.
