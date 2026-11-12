use super::themes_customizer::CustomizerSections;
use crate::section::{OCEAN_CSS, Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::custom_theme::keys;
use verdelle_ui::preset::{
    BaseColor, Font, FontHeading, IconLibrary, MenuAccent, MenuColor, Preset, PresetField, Radius,
    Style, Theme,
};
use verdelle_ui::themes::{DEFAULT_THEME, THEMES};
use verdelle_ui::{
    Alert, AlertVariant, Badge, BadgeVariant, Button, ButtonVariant, Field, FieldDescription,
    FieldGroup, FieldLabel, FieldOrientation, Input, ModeToggle, Select, SelectContent, SelectItem,
    SelectTrigger, SelectValue, Size, Switch, Textarea, ThemeMode, ThemePicker, ThemeStyle,
    ThemeSwitcher, use_theme,
};

/// Presets made with shadcn's own `encodePreset` (the oracle script under
/// `crates/verdelle-ui/tests/fixtures/`), for trying out the parts.
pub const REFERENCE_PRESETS: [(&str, &str); 7] = [
    ("Vega, defaults", "bIkeymG"),
    ("Rose, large radius", "b501U2Nxg"),
    ("Emerald, bold inverted menus", "b2BoX62ix"),
    ("Stone, mono font", "b3REsR4cq"),
    ("Violet, Hugeicons", "b5w7ASzvU"),
    ("Taupe serif (Sera style)", "b4xFeBLg4O"),
    ("The code shadcn's docs use", "b2oE5cW2a"),
];

const SIZES: [(Size, &str); 4] = [
    (Size::XSmall, "xs (24px)"),
    (Size::Small, "sm (28px)"),
    (Size::Medium, "md (32px, default)"),
    (Size::Large, "lg (36px)"),
];

fn options<T: Copy>(
    all: &[T],
    slug: impl Fn(T) -> String,
    label: impl Fn(T) -> String,
) -> Vec<(String, String)> {
    all.iter().map(|v| (slug(*v), label(*v))).collect()
}

/// One labelled Select for one field of a preset.
#[component]
fn PresetSelect(
    label: &'static str,
    id: &'static str,
    options: Vec<(String, String)>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    let items = options.clone();
    let options = StoredValue::new(options);
    view! {
        <Field>
            <FieldLabel html_for=id>{label}</FieldLabel>
            <Select
                value=Signal::derive(move || Some(value.get()))
                on_value_change=on_change
                items=items
            >
                <SelectTrigger id=id class="w-full">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {options
                        .get_value()
                        .into_iter()
                        .map(|(slug, name)| view! { <SelectItem value=slug>{name.clone()}</SelectItem> })
                        .collect_view()}
                </SelectContent>
            </Select>
        </Field>
    }
}

/// What Verdelle renders of each part of a preset, and what it uses when it does not.
#[component]
fn SupportTable(#[prop(into)] preset: Signal<Preset>) -> impl IntoView {
    view! {
        <div class="overflow-x-auto rounded-md border">
            <table class="w-full text-sm" id="themes-support">
                <thead class="bg-muted text-left text-xs text-muted-foreground">
                    <tr>
                        <th class="px-3 py-2">"Field"</th>
                        <th class="px-3 py-2">"Value"</th>
                        <th class="px-3 py-2">"Verdelle"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let p = preset.get();
                        let unsupported = p.unsupported();
                        let eff = p.effective();
                        let rows = [
                            (PresetField::Style, p.style.slug(), eff.style.slug()),
                            (PresetField::IconLibrary, p.icon_library.slug(), eff.icon_library.slug()),
                            (PresetField::BaseColor, p.base_color.slug(), p.base_color.slug()),
                            (PresetField::Theme, p.theme.slug(), p.theme.slug()),
                            (PresetField::ChartColor, p.chart_color.slug(), p.chart_color.slug()),
                            (PresetField::Radius, p.radius.slug(), p.radius.slug()),
                            (PresetField::Font, p.font.slug(), p.font.slug()),
                            (PresetField::FontHeading, p.font_heading.slug(), p.font_heading.slug()),
                            (PresetField::MenuColor, p.menu_color.slug(), p.menu_color.slug()),
                            (PresetField::MenuAccent, p.menu_accent.slug(), p.menu_accent.slug()),
                        ];
                        rows.into_iter()
                            .map(|(field, value, used)| {
                                let ok = !unsupported.contains(&field);
                                view! {
                                    <tr class="border-t" data-field=field.name() data-supported=ok.to_string()>
                                        <td class="px-3 py-2 font-mono text-xs">{field.name()}</td>
                                        <td class="px-3 py-2">{value}</td>
                                        <td class="px-3 py-2">
                                            {if ok {
                                                view! { <Badge variant=BadgeVariant::Secondary>"rendered"</Badge> }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <Badge variant=BadgeVariant::Outline>
                                                        "falls back to " {used}
                                                    </Badge>
                                                }
                                                    .into_any()
                                            }}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>
        </div>
    }
}

/// A sample of components that shows a theme: colours, radius, fonts, charts.
#[component]
fn Sample(#[prop(into)] theme: String) -> impl IntoView {
    let on = RwSignal::new(true);
    let text = RwSignal::new("Sample input".to_string());
    view! {
        <div
            data-theme=theme
            class="space-y-3 rounded-lg border bg-background p-4 text-foreground"
            style="font-family: var(--font-sans, inherit)"
        >
            <h3 class="text-base font-semibold" style="font-family: var(--font-heading, inherit)">
                "Heading text"
            </h3>
            <p class="text-sm text-muted-foreground">"Body text in the preset's font."</p>
            <div class="flex flex-wrap items-center gap-2">
                <Button>"Primary"</Button>
                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                <Badge>"Badge"</Badge>
                <Switch checked=on aria_label="Sample switch" />
            </div>
            <Input value=text aria_label="Sample input" />
            <div class="flex h-8 gap-1" aria-label="Chart colours">
                <div class="flex-1 rounded-sm" style="background:var(--chart-1)"></div>
                <div class="flex-1 rounded-sm" style="background:var(--chart-2)"></div>
                <div class="flex-1 rounded-sm" style="background:var(--chart-3)"></div>
                <div class="flex-1 rounded-sm" style="background:var(--chart-4)"></div>
                <div class="flex-1 rounded-sm" style="background:var(--chart-5)"></div>
            </div>
        </div>
    }
}

/// The app does the font loading; Verdelle only gives the URL. The gallery adds the link for the
/// preset on the page.
#[cfg(feature = "hydrate")]
fn load_fonts(url: &str) {
    let doc = document();
    let Some(head) = doc.head() else { return };
    let link = doc.get_element_by_id("gallery-preset-fonts").or_else(|| {
        let el = doc.create_element("link").ok()?;
        el.set_id("gallery-preset-fonts");
        let _ = el.set_attribute("rel", "stylesheet");
        head.append_child(&el).ok()?;
        Some(el)
    });
    if let Some(link) = link {
        let _ = link.set_attribute("href", url);
    }
}

#[component]
pub fn ThemesPage() -> impl IntoView {
    let app = use_theme();
    // The other examples use their own state, so they do not restyle the page.
    let preview = RwSignal::new(DEFAULT_THEME.to_string());
    let field_value = RwSignal::new(DEFAULT_THEME.to_string());
    let cards = RwSignal::new(DEFAULT_THEME.to_string());
    let mode_items: Vec<(String, String)> = ThemeMode::ALL
        .iter()
        .map(|m| (m.slug().to_string(), m.label().to_string()))
        .collect();

    view! {
        <Page
            title="Themes"
            summary="A theme is shadcn's token set. ThemeProvider owns the page's theme and mode, applies shadcn presets (the short codes from shadcn's /create page), takes pasted shadcn CSS, and keeps themes people make in the browser. ThemeCustomizer is a ready-made Appearance panel for all of it; SyncedStorage keeps a person's theme on a server. The sections below show the panel and every option, then the lower-level pieces."
        >
            <CustomizerSections />
            <Section
                title="ThemeSwitcher with the provider"
                description="No props: it lists the built-in themes, those declared with ThemeStyle (Ocean, below), the preset prop, the themes you save on this page and the unsaved preview, and it changes the page's theme. Same control as the sidebar's. Each swatch shows its own theme's background, primary, accent and first chart colour: apply a preset code in the customizer above, open this list, and the Custom entry (and a saved one) show that preset's colours, in light and dark."
            >
                <Row label="Provider">
                    <ThemeSwitcher id="themes-provider-switcher" />
                    <p class="text-sm text-muted-foreground">
                        "theme = "
                        <code id="themes-readout" class="font-mono">{move || app.theme.get()}</code>
                        " (" {THEMES.len()} " built in)"
                    </p>
                </Row>
            </Section>

            <Section
                title="Mode"
                description="Light, dark or the system's setting, independent of the theme. ModeToggle flips light and dark; the Select can also pick System, which follows the OS live. The stored choice is applied before first paint."
            >
                <Row label="ModeToggle">
                    <ModeToggle />
                    <ModeToggle size=Size::Small />
                    <ModeToggle size=Size::Large />
                    <span class="text-sm text-muted-foreground">
                        "mode = " <code id="themes-mode" class="font-mono">{move || app.mode.get().slug()}</code>
                        ", dark = " <code id="themes-dark" class="font-mono">{move || app.dark.get().to_string()}</code>
                    </span>
                </Row>
                <Row label="Mode select">
                    <Select
                        value=Signal::derive(move || Some(app.mode.get().slug().to_string()))
                        on_value_change=move |slug: String| {
                            if let Some(mode) = ThemeMode::from_slug(&slug) {
                                app.mode.set(mode);
                            }
                        }
                        items=mode_items
                    >
                        <SelectTrigger id="themes-mode-select" aria_label="Mode" class="w-40">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            {ThemeMode::ALL
                                .into_iter()
                                .map(|m| view! { <SelectItem value=m.slug()>{m.label()}</SelectItem> })
                                .collect_view()}
                        </SelectContent>
                    </Select>
                </Row>
            </Section>

            <PresetCodes />
            <PresetBuilder />
            <PastedCss />

            <Section
                title="ThemeProvider"
                description="One component near the root of the app, above anything that shows a ThemeSwitcher or ModeToggle."
            >
                <pre class="overflow-x-auto rounded-md border bg-muted p-3 text-xs"><code>{PROVIDER_EXAMPLE}</code></pre>
                <ul class="list-disc space-y-1 pl-5 text-sm text-muted-foreground">
                    <li>
                        <code class="font-mono">"preset"</code>
                        ": a shadcn code, offered as the theme named \"preset\" and used until someone picks another. Radius, tokens, chart colours and fonts are applied; the menu colour and accent become data-vdl-menu-color and data-vdl-menu-accent on html."
                    </li>
                    <li>
                        <code class="font-mono">"storage"</code>
                        ": any ThemeStorage (this gallery's provider uses a SyncedStorage, see above); the default keeps "
                        <code class="font-mono">{keys::THEME}</code> ", " <code class="font-mono">{keys::MODE}</code> ", "
                        <code class="font-mono">{keys::CUSTOM}</code> " and " <code class="font-mono">{keys::PREVIEW}</code>
                        " in localStorage."
                    </li>
                    <li>
                        <code class="font-mono">"script"</code>
                        ": the no-flash inline script. This gallery turns it off in the provider and prints theme_init_script in the document head instead, which is earlier."
                    </li>
                    <li>
                        <code class="font-mono">"default_theme, default_mode"</code>
                        ": what applies when nothing is stored (neutral, system)."
                    </li>
                </ul>
            </Section>

            <Section
                title="ThemeSwitcher without the provider"
                description="Controlled: value and on_change, and an optional themes list. The Select's sizes, each swatch drawn from that theme's own tokens."
            >
                {SIZES
                    .into_iter()
                    .map(|(size, label)| {
                        view! {
                            <Row label=label>
                                <ThemeSwitcher
                                    value=preview
                                    on_change=move |slug| preview.set(slug)
                                    size=size
                                />
                            </Row>
                        }
                    })
                    .collect_view()}
                <Row label="Own list">
                    <ThemeSwitcher
                        id="themes-own-list"
                        value=preview
                        on_change=move |slug| preview.set(slug)
                        themes=vec![
                            ("neutral".to_string(), "Neutral".to_string()),
                            ("rose".to_string(), "Rose".to_string()),
                            ("blue".to_string(), "Blue".to_string()),
                        ]
                    />
                    <span class="text-xs text-muted-foreground">"themes = three of them"</span>
                </Row>
                <Row label="Disabled">
                    <ThemeSwitcher value=preview on_change=move |slug| preview.set(slug) disabled=true />
                </Row>
                <div class="grid max-w-md gap-6">
                    <FieldGroup>
                        <Field orientation=FieldOrientation::Horizontal>
                            <FieldLabel html_for="themes-field">"Colour theme"</FieldLabel>
                            <ThemeSwitcher
                                id="themes-field"
                                value=field_value
                                on_change=move |slug| field_value.set(slug)
                            />
                        </Field>
                        <Field>
                            <FieldLabel html_for="themes-field-v">"Colour theme"</FieldLabel>
                            <ThemeSwitcher
                                id="themes-field-v"
                                class="w-full"
                                value=field_value
                                on_change=move |slug| field_value.set(slug)
                            />
                            <FieldDescription>"With a visible label the trigger is named by it (id and html_for)."</FieldDescription>
                        </Field>
                    </FieldGroup>
                </div>
            </Section>

            <Section
                title="ThemePicker (cards) with the provider"
                description="No props: the same list as ThemeSwitcher (built-in, Ocean from ThemeStyle, the preset, saved themes and the unsaved preview) and it changes the page's theme. Each card draws its swatch from its own theme's tokens, so registered and custom themes show their own colours in light and dark. Save a theme in the customizer above and its card appears here."
            >
                <div id="themes-picker-provider">
                    <ThemePicker />
                </div>
            </Section>

            <Section
                title="ThemePicker: controlled, own list"
                description="With value the picker only edits that signal (apply it yourself); themes replaces the list with (slug, name) pairs. Without a provider the built-in themes are the default list."
            >
                <div id="themes-picker-controlled" class="space-y-3">
                    <ThemePicker value=cards />
                    <p class="text-sm text-muted-foreground">
                        "value = " <code class="font-mono">{move || cards.get()}</code>
                    </p>
                    <div id="themes-picker-own">
                        <ThemePicker
                            value=cards
                            legend="Theme (short list)"
                            themes=vec![
                                ("neutral".to_string(), "Neutral".to_string()),
                                ("ocean".to_string(), "Ocean".to_string()),
                                ("rose".to_string(), "Rose".to_string()),
                            ]
                        />
                    </div>
                </div>
            </Section>
        </Page>
    }
}

const PROVIDER_EXAMPLE: &str = r#"// Near the root of the app. Everything below can call use_theme().
view! {
    <ThemeProvider preset="b2oE5cW2a" default_mode=ThemeMode::System>
        <ThemeStyle name="ocean" css=include_str!("ocean.css") />
        <ThemeSwitcher />   // built-ins, "ocean", the preset, themes people saved
        <ModeToggle />
        <App />
    </ThemeProvider>
}

// In the document <head> (the app loads the font files):
//   <link rel="stylesheet" href=preset.google_fonts_url()>
//   <script>{theme_init_script("neutral", ThemeMode::System, (None, None))}</script>

let theme = use_theme();
theme.try_theme(Some("b2oE5cW2a"), None)?;       // preview a code and/or pasted CSS
let slug = theme.save_theme("Brand", Some(code), Some(css))?;  // keep it
theme.rename_theme(&slug, "Brand v2")?;
let text = theme.export_themes();                // JSON text to copy
theme.import_themes(&text, "Imported")?;"#;

/// Reference codes, and any code a person pastes.
#[component]
fn PresetCodes() -> impl IntoView {
    let app = use_theme();
    let code = RwSignal::new(String::new());
    let decoded = RwSignal::new(Preset::default());
    let message = RwSignal::new(String::new());
    let error = RwSignal::new(false);

    let apply = move |input: String| match Preset::parse_input(&input) {
        Ok(preset) => {
            decoded.set(preset);
            match app.try_preset(&preset) {
                Ok(()) => {
                    error.set(false);
                    message.set(format!("Applied {}", preset.encode()));
                }
                Err(e) => {
                    error.set(true);
                    message.set(e.to_string());
                }
            }
        }
        Err(e) => {
            error.set(true);
            message.set(e.to_string());
        }
    };

    view! {
        <Section
            title="Preset codes"
            description="shadcn's /create page gives every design a short code (b2oE5cW2a). Paste one, or use these, which were made with shadcn's own encoder. Applying replaces the page's theme with the preset (named \"preview\"; it is remembered across reloads until you save or clear it)."
        >
            <div class="flex flex-wrap gap-2" id="themes-references">
                {REFERENCE_PRESETS
                    .into_iter()
                    .map(|(name, preset_code)| {
                        view! {
                            <Button
                                variant=ButtonVariant::Outline
                                size=Size::Small
                                on_click=move |_| {
                                    code.set(preset_code.to_string());
                                    apply(preset_code.to_string());
                                }
                            >
                                {name} " "
                                <code class="font-mono text-xs text-muted-foreground">{preset_code}</code>
                            </Button>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="flex max-w-xl flex-wrap items-center gap-2">
                <div class="flex-1">
                    <Input
                        id="themes-code-input"
                        value=code
                        aria_label="Preset code"
                        placeholder="b2oE5cW2a, --preset b2oE5cW2a, or a /create link"
                    />
                </div>
                <Button on_click=move |_| apply(code.get_untracked())>"Apply"</Button>
                <Button
                    variant=ButtonVariant::Outline
                    on_click=move |_| {
                        match Preset::parse_input(&code.get_untracked()) {
                            Ok(p) => {
                                decoded.set(p);
                                error.set(false);
                                message.set(format!("Decoded {}", p.encode()));
                            }
                            Err(e) => {
                                error.set(true);
                                message.set(e.to_string());
                            }
                        }
                    }
                >
                    "Decode only"
                </Button>
                <Button variant=ButtonVariant::Ghost on_click=move |_| app.clear_preview()>
                    "Clear preview"
                </Button>
            </div>
            <p
                id="themes-code-message"
                class="text-sm"
                class:text-destructive=move || error.get()
                class:text-muted-foreground=move || !error.get()
            >
                {move || message.get()}
            </p>
            <h3 class="text-sm font-medium">"What the code says, and what Verdelle renders"</h3>
            <p class="text-sm text-muted-foreground">
                "Verdelle has the nova style (shadcn's default) and Lucide icons. A preset that names another style (vega, maia, lyra, mira, luma, sera, rhea) or icon library still applies (colours, radius, fonts and menus all work); those two parts fall back, and nothing errors. The default code b0 is shadcn's own default, so it shows no fallback."
            </p>
            <SupportTable preset=decoded />
        </Section>
    }
}

/// Every field of a preset, one control each, with a live sample.
#[component]
fn PresetBuilder() -> impl IntoView {
    let app = use_theme();
    let builder = RwSignal::new(Preset {
        style: Style::Nova,
        ..Preset::default()
    });
    let result = RwSignal::new(String::new());

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| load_fonts(&builder.get().google_fonts_url()));

    macro_rules! field {
        ($label:literal, $id:literal, $ty:ident, $field:ident) => {
            view! {
                <PresetSelect
                    label=$label
                    id=$id
                    options=options($ty::ALL, |v| v.slug().to_string(), |v| v.label())
                    value=Signal::derive(move || builder.get().$field.slug().to_string())
                    on_change=move |slug: String| {
                        if let Some(v) = $ty::from_slug(&slug) {
                            builder.update(|p| p.$field = v);
                        }
                    }
                />
            }
        };
    }

    view! {
        <Section
            title="Preset builder: every field"
            description="One control per field of the code, in shadcn's order. The sample below is styled with the tokens the preset composes (base colour, then the accent theme over it, then the chart colour, then radius and menu accent), scoped to the sample only. Apply puts it on the whole page."
        >
            <style inner_html=move || builder.get().effective().scoped_css("builder")></style>
            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {field!("Style", "pb-style", Style, style)}
                {field!("Base colour", "pb-base", BaseColor, base_color)}
                {field!("Theme (accent)", "pb-theme", Theme, theme)}
                {field!("Chart colour", "pb-chart", Theme, chart_color)}
                {field!("Radius", "pb-radius", Radius, radius)}
                <PresetSelect
                    label="Font"
                    id="pb-font"
                    options=options(Font::ALL, |v| v.slug().to_string(), |v| v.title().to_string())
                    value=Signal::derive(move || builder.get().font.slug().to_string())
                    on_change=move |slug: String| {
                        if let Some(v) = Font::from_slug(&slug) {
                            builder.update(|p| p.font = v);
                        }
                    }
                />
                <PresetSelect
                    label="Heading font"
                    id="pb-heading"
                    options=FontHeading::all().map(|v| (v.slug().to_string(), v.label())).collect()
                    value=Signal::derive(move || builder.get().font_heading.slug().to_string())
                    on_change=move |slug: String| {
                        if let Some(v) = FontHeading::from_slug(&slug) {
                            builder.update(|p| p.font_heading = v);
                        }
                    }
                />
                {field!("Icon library", "pb-icons", IconLibrary, icon_library)}
                {field!("Menu colour", "pb-menu-color", MenuColor, menu_color)}
                {field!("Menu accent", "pb-menu-accent", MenuAccent, menu_accent)}
            </div>
            <Row label="Code">
                <code id="themes-builder-code" class="rounded bg-muted px-2 py-1 font-mono text-sm">
                    {move || builder.get().encode()}
                </code>
                <Button
                    on_click=move |_| {
                        result.set(match app.try_preset(&builder.get_untracked()) {
                            Ok(()) => "Applied to the page.".to_string(),
                            Err(e) => e.to_string(),
                        });
                    }
                >
                    "Apply to the page"
                </Button>
                <Button
                    variant=ButtonVariant::Outline
                    on_click=move |_| builder.set(Preset::default())
                >
                    "Reset"
                </Button>
                <span id="themes-builder-result" class="text-sm text-muted-foreground">{move || result.get()}</span>
            </Row>
            <Row label="Fonts">
                <span class="text-xs text-muted-foreground">
                    "The app loads the font files; Verdelle gives the stack and a URL. This page adds a link for the preset's fonts: "
                </span>
                <code id="themes-fonts-url" class="break-all font-mono text-xs">{move || builder.get().google_fonts_url()}</code>
            </Row>
            {move || {
                let missing = builder.get().unsupported();
                (!missing.is_empty())
                    .then(|| {
                        view! {
                            <Alert variant=AlertVariant::Default class="max-w-2xl">
                                <p id="themes-builder-fallback" class="text-sm">
                                    "Verdelle does not render "
                                    {missing.iter().map(|f| f.name()).collect::<Vec<_>>().join(" and ")}
                                    " yet: the sample uses nova and Lucide instead. The code keeps the values."
                                </p>
                            </Alert>
                        }
                    })
            }}
            <Sample theme="builder" />
            <SupportTable preset=builder />
        </Section>
    }
}

/// Pasted shadcn CSS through ThemeStyle.
#[component]
fn PastedCss() -> impl IntoView {
    let css = RwSignal::new(OCEAN_CSS.to_string());
    view! {
        <Section
            title="ThemeStyle: pasted shadcn CSS"
            description="Paste CSS exported from shadcn's /create page, its theme pages or tweakcn: a :root block and a .dark block (oklch values, --radius, sidebar and chart tokens). ThemeStyle scopes it to data-theme and registers it, so ThemeSwitcher lists it: the Ocean theme in every theme list on this page is the CSS below, declared once in the app. Edit the text to change the sample."
        >
            <div class="grid gap-4 lg:grid-cols-2">
                <div class="space-y-2">
                    <Textarea id="themes-css-input" value=css rows=14 aria_label="Pasted shadcn CSS" />
                    <p class="text-xs text-muted-foreground">
                        "Only the :root and .dark blocks are read; the @import, @theme and @layer rules are ignored. Tokens the CSS leaves out keep the default theme's values."
                    </p>
                </div>
                <div class="space-y-2">
                    <ThemeStyle name="pasted-demo" label="Pasted (demo)" css=css />
                    <Sample theme="pasted-demo" />
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| {
                                let _ = use_theme().try_theme(None, Some(&css.get_untracked()));
                            }
                        >
                            "Put it on the whole page"
                        </Button>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=Size::Small
                            on_click=move |_| css.set(OCEAN_CSS.to_string())
                        >
                            "Reset"
                        </Button>
                    </Row>
                </div>
            </div>
        </Section>
    }
}
