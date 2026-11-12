use crate::custom_theme::{CustomTheme, check_limit};
use crate::preset::{BaseColor, Font, FontHeading, MenuAccent, MenuColor, Preset, Radius, Theme};
use crate::theme_css::slugify;
use crate::theme_customizer_model::{PresetSummary, read_preset_input, summarize};
use crate::theme_provider::{ThemeContext, ThemeMode};
use crate::{
    Alert, AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertVariant, Badge, BadgeVariant,
    Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, Collapsible,
    CollapsibleContent, CollapsibleTrigger, Field, FieldLabel, Icon, IconName, Input, Item,
    ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle, ItemVariant, Select,
    SelectContent, SelectItem, SelectTrigger, SelectValue, Size, Textarea, ThemeSwitcher,
    ToggleGroup, ToggleGroupItem, ToggleVariant,
};
use leptos::prelude::*;

/// What [`ThemeCustomizer`] tells its `on_apply` callback: the theme that was just put on the
/// page as the (unsaved) preview.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeApplied {
    /// The preset code (`b2oE5cW2a`), when the theme came from one.
    pub code: Option<String>,
    /// The pasted shadcn CSS, when the theme came from some.
    pub css: Option<String>,
    /// A stylesheet URL for the preset's fonts, for apps that load them themselves.
    pub fonts_url: Option<String>,
}

/// Everything the parts share. It is `Copy`: the parts take it by value.
#[derive(Clone, Copy)]
struct Env {
    ctx: ThemeContext,
    prefix: StoredValue<String>,
    on_apply: Callback<ThemeApplied>,
    on_save: Callback<CustomTheme>,
    max_saved: MaybeProp<usize>,
    /// The `compact` prop: sections leave out their explanatory paragraphs.
    compact: Signal<bool>,
    /// The preset the fine-tuning controls edit.
    draft: RwSignal<Preset>,
}

impl Env {
    fn id(&self, part: &str) -> String {
        format!("{}-{part}", self.prefix.get_value())
    }

    /// Puts `preset` on the page as the preview and tells the app.
    fn apply_preset(&self, preset: &Preset) -> Result<(), String> {
        self.ctx.try_preset(preset).map_err(|e| e.to_string())?;
        self.on_apply.run(ThemeApplied {
            code: Some(preset.encode()),
            css: None,
            fonts_url: Some(preset.google_fonts_url()),
        });
        Ok(())
    }
}

fn options<T: Copy>(
    all: &[T],
    slug: impl Fn(T) -> String,
    label: impl Fn(T) -> String,
) -> Vec<(String, String)> {
    all.iter().map(|v| (slug(*v), label(*v))).collect()
}

/// A stylesheet link for the preset's fonts. The app owns font loading; this is the opt-in
/// helper (`load_fonts`).
#[cfg(feature = "hydrate")]
fn load_font_link(url: &str) {
    let doc = document();
    let Some(head) = doc.head() else { return };
    let link = doc.get_element_by_id("vdl-customizer-fonts").or_else(|| {
        let el = doc.create_element("link").ok()?;
        el.set_id("vdl-customizer-fonts");
        let _ = el.set_attribute("rel", "stylesheet");
        head.append_child(&el).ok()?;
        Some(el)
    });
    if let Some(link) = link {
        let _ = link.set_attribute("href", url);
    }
}

/// An "Appearance" panel for an app: choose a theme and mode, paste a shadcn preset code,
/// fine-tune a preset, paste shadcn CSS, keep named themes, and see the result on a small
/// preview. Not part of shadcn/ui. It must sit under a [`crate::ThemeProvider`] (it uses
/// [`crate::use_theme`]) and works in a page, a Card or a Tabs panel: it needs no particular
/// width, down to a phone's.
///
/// Sections, each switched with a `show_*` prop (all on by default):
///
/// - **`show_theme`**: [`crate::ThemeSwitcher`] (with swatches) and the light / dark / system
///   control.
/// - **`show_preset`**: a box that takes a bare code, `--preset X` or a ui.shadcn.com/create
///   link, an Apply button, a one-line summary of what was applied with a collapsed list of
///   the parts Verdelle falls back on (a style other than nova, an icon library other than
///   Lucide), and a link to <https://ui.shadcn.com/create> to make codes.
/// - **`show_fine_tuning`**: base, accent and chart colours, radius, menu colour and accent,
///   font and heading font. Every change puts the resulting preset on the page as the preview.
///   The controls start from the chosen theme's preset when it has one (the preview, a saved
///   theme or the provider's `preset`), else from shadcn's default.
/// - **`show_css`**: a textarea for CSS exported from shadcn (`:root` and `.dark` blocks) with a
///   live preview as you type, and Apply.
/// - **`show_saved`**: the saved themes (apply, rename, delete after a confirmation), "Save
///   current as...", and export / import as JSON text.
/// - **`show_preview`**: a small card of components (Button, Badge, Card, Input) in the page's
///   theme.
///
/// Whatever is applied becomes the provider's unsaved `preview` theme (kept across reloads);
/// saving makes it a named theme. Everything is stored through the provider's
/// [`crate::custom_theme::ThemeStorage`], so a [`crate::SyncedStorage`] there keeps it all on
/// a server. Fonts are the app's to load (see [`Preset::google_fonts_url`]); `load_fonts` adds
/// a `<link>` for the preset in use, as a convenience.
#[component]
pub fn ThemeCustomizer(
    /// Show the theme switcher and mode control. Default: true.
    #[prop(into, default = Signal::stored(true))]
    show_theme: Signal<bool>,
    /// Show the preset code box with its summary. Default: true.
    #[prop(into, default = Signal::stored(true))]
    show_preset: Signal<bool>,
    /// Show the fine-tuning controls. Default: true.
    #[prop(into, default = Signal::stored(true))]
    show_fine_tuning: Signal<bool>,
    /// Show the pasted-CSS box. Default: true.
    #[prop(into, default = Signal::stored(true))]
    show_css: Signal<bool>,
    /// Show the saved themes, "Save current as..." and export / import. Default: true.
    #[prop(into, default = Signal::stored(true))]
    show_saved: Signal<bool>,
    /// Show the small preview card. Default: true.
    #[prop(into, default = Signal::stored(true))]
    show_preview: Signal<bool>,
    /// A tighter layout for a Settings panel: smaller gaps and no explanatory paragraphs (the
    /// intro line under each section title, the note about what falls back and the note under
    /// "Save"). Labels, controls, errors, the preset summary and the "limit reached" message stay.
    #[prop(optional, into)]
    compact: Signal<bool>,
    /// The most themes a person may keep (saving and importing stop there). Default: no limit.
    #[prop(optional, into)]
    max_saved: MaybeProp<usize>,
    /// Add a stylesheet `<link>` for the fonts of the preset in use. Default: false (the app
    /// loads them).
    #[prop(optional, into)]
    load_fonts: Signal<bool>,
    /// Called after a preset or CSS was applied to the page (also after each fine-tuning
    /// change).
    #[prop(optional, into)]
    on_apply: Option<Callback<ThemeApplied>>,
    /// Called with the theme after it was saved.
    #[prop(optional, into)]
    on_save: Option<Callback<CustomTheme>>,
    /// Extra classes, for layout.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let ctx = expect_context::<ThemeContext>();
    let prefix = format!(
        "vdl-tc-{}",
        crate::primitives::core::id::use_id(None).get_untracked()
    );
    let env = Env {
        ctx,
        prefix: StoredValue::new(prefix),
        on_apply: on_apply.unwrap_or_else(|| Callback::new(|_| {})),
        on_save: on_save.unwrap_or_else(|| Callback::new(|_| {})),
        max_saved,
        compact,
        draft: RwSignal::new(Preset::default()),
    };

    // The fine-tuning controls follow whatever theme the provider has chosen.
    Effect::new(move |_| {
        if let Some(p) = ctx.active_custom().and_then(|t| t.preset())
            && p != env.draft.get_untracked()
        {
            env.draft.set(p);
        }
    });

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        if load_fonts.get()
            && let Some(p) = ctx.active_custom().and_then(|t| t.preset())
        {
            load_font_link(&p.google_fonts_url());
        }
    });
    #[cfg(not(feature = "hydrate"))]
    let _ = load_fonts;

    view! {
        <div
            class=format!("vdl-theme-customizer {class}")
            data-vdl-theme-customizer=""
            data-compact=move || compact.get().then_some("true")
        >
            <Show when=move || show_theme.get()>
                <ThemeAndMode env=env />
            </Show>
            <Show when=move || show_preset.get()>
                <PresetBox env=env />
            </Show>
            <Show when=move || show_fine_tuning.get()>
                <FineTuning env=env />
            </Show>
            <Show when=move || show_css.get()>
                <PastedCss env=env />
            </Show>
            <Show when=move || show_saved.get()>
                <SavedThemes env=env />
            </Show>
            <Show when=move || show_preview.get()>
                <PreviewCard env=env />
            </Show>
        </div>
    }
}

/// A titled section.
#[component]
fn Block(
    env: Env,
    title: &'static str,
    part: &'static str,
    #[prop(optional)] hint: &'static str,
    children: Children,
) -> impl IntoView {
    let id = env.id(&format!("{part}-title"));
    view! {
        <section class="vdl-theme-customizer__section" data-part=part aria-labelledby=id.clone()>
            <h3 id=id.clone() class="vdl-theme-customizer__title">
                {title}
            </h3>
            {move || {
                (!hint.is_empty() && !env.compact.get())
                    .then(|| view! { <p class="vdl-theme-customizer__hint">{hint}</p> })
            }}
            {children()}
        </section>
    }
}

/// The theme switcher and the light / dark / system control.
#[component]
fn ThemeAndMode(env: Env) -> impl IntoView {
    let ctx = env.ctx;
    let switcher_id = env.id("theme");
    let on_mode = Callback::new(move |v: Vec<String>| {
        if let Some(mode) = v.first().and_then(|s| ThemeMode::from_slug(s)) {
            ctx.mode.set(mode);
        }
    });
    view! {
        <Block env=env title="Theme" part="theme">
            <div class="vdl-theme-customizer__grid vdl-theme-customizer__grid--wide">
                <Field>
                    <FieldLabel html_for=switcher_id.clone()>"Colour theme"</FieldLabel>
                    <ThemeSwitcher id=switcher_id class="w-full" />
                </Field>
                <Field>
                    <FieldLabel>"Mode"</FieldLabel>
                    <ToggleGroup
                        aria_label="Mode"
                        variant=ToggleVariant::Outline
                        deselectable=false
                        value=Signal::derive(move || vec![ctx.mode.get().slug().to_string()])
                        on_value_change=on_mode
                    >
                        <ToggleGroupItem value="light">
                            <Icon name=IconName::Sun size=14 />
                            "Light"
                        </ToggleGroupItem>
                        <ToggleGroupItem value="dark">
                            <Icon name=IconName::Moon size=14 />
                            "Dark"
                        </ToggleGroupItem>
                        <ToggleGroupItem value="system">
                            <Icon name=IconName::Monitor size=14 />
                            "System"
                        </ToggleGroupItem>
                    </ToggleGroup>
                </Field>
            </div>
        </Block>
    }
}

/// The preset code box, its summary and the link to shadcn's creator.
#[component]
fn PresetBox(env: Env) -> impl IntoView {
    let code = RwSignal::new(String::new());
    let outcome = RwSignal::new(None::<Result<PresetSummary, String>>);
    let input_id = env.id("code");

    let apply = move || match read_preset_input(&code.get_untracked()) {
        Ok(preset) => match env.apply_preset(&preset) {
            Ok(()) => outcome.set(Some(Ok(summarize(&preset)))),
            Err(e) => outcome.set(Some(Err(e))),
        },
        Err(e) => outcome.set(Some(Err(e))),
    };

    view! {
        <Block
            env=env
            title="Preset"
            part="preset"
            hint="shadcn's create page gives each design a short code. Paste it here."
        >
            <Field>
                <FieldLabel html_for=input_id.clone()>"Preset code"</FieldLabel>
                <div class="vdl-theme-customizer__row vdl-theme-customizer__row--nowrap">
                    <div class="vdl-theme-customizer__grow">
                        <Input
                            id=input_id
                            value=code
                            placeholder="b2oE5cW2a, --preset b2oE5cW2a or a link"
                            autocomplete="off"
                            on_enter=move |_| apply()
                        />
                    </div>
                    <Button on_click=move |_| apply()>"Apply"</Button>
                </div>
            </Field>
            <a
                class="vdl-theme-customizer__link"
                href="https://ui.shadcn.com/create"
                target="_blank"
                rel="noopener noreferrer"
            >
                "Make a code on ui.shadcn.com/create"
                <Icon name=IconName::ExternalLink size=12 />
            </a>
            <div role="status" aria-live="polite" class="vdl-theme-customizer__status">
                {move || {
                    outcome
                        .get()
                        .map(|r| match r {
                            Err(e) => {
                                view! {
                                    <Alert variant=AlertVariant::Destructive>
                                        <p class="text-sm">{e}</p>
                                    </Alert>
                                }
                                    .into_any()
                            }
                            Ok(summary) => view! { <SummaryView summary=summary compact=env.compact /> }.into_any(),
                        })
                }}
            </div>
        </Block>
    }
}

/// "Applied X; 1 part falls back", with the details collapsed.
#[component]
fn SummaryView(summary: PresetSummary, compact: Signal<bool>) -> impl IntoView {
    let headline = summary.headline();
    let has_fallback = !summary.fell_back.is_empty();
    let summary = StoredValue::new(summary);
    view! {
        <div class="vdl-theme-customizer__summary" data-fallback=has_fallback.then_some("true")>
            <p class="text-sm" data-testid="summary-headline">
                {headline}
            </p>
            <Collapsible>
                <CollapsibleTrigger variant=ButtonVariant::Ghost size=Size::XSmall>
                    "What was applied"
                    <Icon name=IconName::ChevronDown size=12 />
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <div class="vdl-theme-customizer__details">
                        {move || {
                            let s = summary.get_value();
                            let fell_back = s.fell_back.clone();
                            view! {
                                <ul class="vdl-theme-customizer__chips" aria-label="Applied">
                                    {s
                                        .applied
                                        .into_iter()
                                        .map(|a| {
                                            view! {
                                                <li>
                                                    <Badge variant=BadgeVariant::Secondary>
                                                        {a.field} ": " {a.value}
                                                    </Badge>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                                {(!fell_back.is_empty())
                                    .then(move || {
                                        view! {
                                            {(!compact.get())
                                                .then(|| {
                                                    view! {
                                                        <p class="vdl-theme-customizer__hint">
                                                            "Verdelle renders the nova style with Lucide icons. These parts of the code fall back:"
                                                        </p>
                                                    }
                                                })}
                                            <ul class="vdl-theme-customizer__chips" aria-label="Falls back">
                                                {fell_back
                                                    .into_iter()
                                                    .map(|f| {
                                                        view! {
                                                            <li>
                                                                <Badge variant=BadgeVariant::Outline>
                                                                    {f.field} ": " {f.asked.clone()} " uses " {f.used.clone()}
                                                                </Badge>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </ul>
                                        }
                                    })}
                            }
                        }}
                    </div>
                </CollapsibleContent>
            </Collapsible>
        </div>
    }
}

/// One labelled Select for one field of the draft preset.
#[component]
fn PresetSelect(
    env: Env,
    label: &'static str,
    part: &'static str,
    options: Vec<(String, String)>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    let id = env.id(part);
    let items = options.clone();
    let options = StoredValue::new(options);
    view! {
        <Field>
            <FieldLabel html_for=id.clone()>{label}</FieldLabel>
            <Select
                value=Signal::derive(move || Some(value.get()))
                on_value_change=on_change
                items=items
            >
                <SelectTrigger id=id.clone() class="w-full">
                    <SelectValue />
                </SelectTrigger>
                <SelectContent>
                    {move || {
                        options
                            .get_value()
                            .into_iter()
                            .map(|(slug, name)| view! { <SelectItem value=slug>{name.clone()}</SelectItem> })
                            .collect_view()
                    }}
                </SelectContent>
            </Select>
        </Field>
    }
}

/// Controls for the fields of a preset that Verdelle renders; each change is applied at once.
#[component]
fn FineTuning(env: Env) -> impl IntoView {
    let draft = env.draft;
    let error = RwSignal::new(None::<String>);
    let push = move || {
        error.set(env.apply_preset(&draft.get_untracked()).err());
    };

    macro_rules! field {
        ($label:literal, $part:literal, $ty:ident, $field:ident) => {
            view! {
                <PresetSelect
                    env=env
                    label=$label
                    part=$part
                    options=options($ty::ALL, |v| v.slug().to_string(), |v| v.label())
                    value=Signal::derive(move || draft.get().$field.slug().to_string())
                    on_change=move |slug: String| {
                        if let Some(v) = $ty::from_slug(&slug) {
                            draft.update(|p| p.$field = v);
                            push();
                        }
                    }
                />
            }
        };
    }

    view! {
        <Block
            env=env
            title="Fine tuning"
            part="tuning"
            hint="Each change is applied to the page as you make it. Your app loads the font files."
        >
            <div class="vdl-theme-customizer__grid">
                {field!("Base colour", "base", BaseColor, base_color)}
                {field!("Accent colour", "accent", Theme, theme)}
                {field!("Chart colours", "chart", Theme, chart_color)}
                {field!("Radius", "radius", Radius, radius)}
                {field!("Menu colour", "menu-color", MenuColor, menu_color)}
                {field!("Menu accent", "menu-accent", MenuAccent, menu_accent)}
                <PresetSelect
                    env=env
                    label="Font"
                    part="font"
                    options=options(Font::ALL, |v| v.slug().to_string(), |v| v.title().to_string())
                    value=Signal::derive(move || draft.get().font.slug().to_string())
                    on_change=move |slug: String| {
                        if let Some(v) = Font::from_slug(&slug) {
                            draft.update(|p| p.font = v);
                            push();
                        }
                    }
                />
                <PresetSelect
                    env=env
                    label="Heading font"
                    part="heading-font"
                    options=FontHeading::all().map(|v| (v.slug().to_string(), v.label())).collect()
                    value=Signal::derive(move || draft.get().font_heading.slug().to_string())
                    on_change=move |slug: String| {
                        if let Some(v) = FontHeading::from_slug(&slug) {
                            draft.update(|p| p.font_heading = v);
                            push();
                        }
                    }
                />
            </div>
            <div class="vdl-theme-customizer__row">
                <code class="vdl-theme-customizer__code" data-testid="draft-code">
                    {move || draft.get().encode()}
                </code>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    on_click=move |_| {
                        draft.set(Preset::default());
                        push();
                    }
                >
                    "Reset to default"
                </Button>
            </div>
            {move || {
                error
                    .get()
                    .map(|e| view! { <Alert variant=AlertVariant::Destructive><p class="text-sm">{e}</p></Alert> })
            }}
        </Block>
    }
}

/// A few components drawn with the tokens of `theme` (a `data-theme` value), to see pasted CSS
/// before it is applied.
#[component]
fn MiniSample(#[prop(into)] theme: String) -> impl IntoView {
    view! {
        <div data-theme=theme class="vdl-theme-customizer__mini" aria-label="Preview of the pasted CSS" role="group">
            <Button size=Size::Small>"Primary"</Button>
            <Button size=Size::Small variant=ButtonVariant::Secondary>
                "Secondary"
            </Button>
            <Badge>"Badge"</Badge>
            <span class="vdl-theme-customizer__bars" aria-hidden="true">
                <i style="background:var(--chart-1)"></i>
                <i style="background:var(--chart-2)"></i>
                <i style="background:var(--chart-3)"></i>
                <i style="background:var(--chart-4)"></i>
                <i style="background:var(--chart-5)"></i>
            </span>
        </div>
    }
}

/// A textarea for shadcn CSS with a live preview and Apply.
#[component]
fn PastedCss(env: Env) -> impl IntoView {
    let css = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);
    let id = env.id("css");
    let draft_slug = slugify(&env.id("css-draft"));
    let built = {
        let slug = draft_slug.clone();
        Memo::new(move |_| {
            let text = css.get();
            if text.trim().is_empty() {
                return None;
            }
            Some(
                CustomTheme::with_slug(&slug, "Draft", None, Some(&text))
                    .map(|t| t.rules)
                    .map_err(|e| e.to_string()),
            )
        })
    };
    let apply = move |_| {
        let text = css.get_untracked();
        match env.ctx.try_theme(None, Some(&text)) {
            Ok(()) => {
                error.set(None);
                env.on_apply.run(ThemeApplied {
                    code: None,
                    css: Some(text),
                    fonts_url: None,
                });
            }
            Err(e) => error.set(Some(e.to_string())),
        }
    };
    let can_apply = Signal::derive(move || matches!(built.get(), Some(Ok(_))));

    view! {
        <Block
            env=env
            title="Pasted CSS"
            part="css"
            hint="Paste the :root and .dark blocks from shadcn's create page, its theme pages or tweakcn. Only those blocks are read."
        >
            <Field>
                <FieldLabel html_for=id.clone()>"shadcn CSS"</FieldLabel>
                <Textarea
                    id=id
                    value=css
                    rows=6
                    placeholder=":root { --primary: oklch(0.6 0.2 30); } .dark { ... }"
                />
            </Field>
            {move || {
                match built.get() {
                    Some(Ok(rules)) => {
                        view! {
                            <style data-vdl-customizer-draft="" inner_html=rules></style>
                            <MiniSample theme=draft_slug.clone() />
                        }
                            .into_any()
                    }
                    Some(Err(e)) => {
                        view! {
                            <Alert variant=AlertVariant::Destructive>
                                <p class="text-sm" data-testid="css-parse-error">{e}</p>
                            </Alert>
                        }
                            .into_any()
                    }
                    None => ().into_any(),
                }
            }}
            <div class="vdl-theme-customizer__row">
                <Button disabled=Signal::derive(move || !can_apply.get()) on_click=apply>
                    "Apply to the page"
                </Button>
                <Button
                    variant=ButtonVariant::Ghost
                    disabled=Signal::derive(move || css.get().is_empty())
                    on_click=move |_| {
                        css.set(String::new());
                        error.set(None);
                    }
                >
                    "Clear"
                </Button>
            </div>
            {move || {
                error
                    .get()
                    .map(|e| view! { <Alert variant=AlertVariant::Destructive><p class="text-sm">{e}</p></Alert> })
            }}
        </Block>
    }
}

/// The saved themes, saving the current look, export and import.
#[component]
fn SavedThemes(env: Env) -> impl IntoView {
    let ctx = env.ctx;
    let name = RwSignal::new(String::new());
    let message = RwSignal::new(None::<(bool, String)>);
    let editing = RwSignal::new(None::<String>);
    let rename_to = RwSignal::new(String::new());
    let confirm_open = RwSignal::new(false);
    let to_delete = RwSignal::new(None::<(String, String)>);
    let export_text = RwSignal::new(String::new());
    let import_text = RwSignal::new(String::new());
    let name_id = env.id("save-name");

    let count = Memo::new(move |_| ctx.saved().len());
    let full = Signal::derive(move || env.max_saved.get().is_some_and(|m| count.get() >= m));
    let current = Memo::new(move |_| ctx.active_custom().map(|t| (t.code, t.css)));
    let can_save = Signal::derive(move || {
        !name.get().trim().is_empty() && current.get().is_some() && !full.get()
    });

    let report = move |r: Result<String, String>| match r {
        Ok(m) => message.set(Some((false, m))),
        Err(e) => message.set(Some((true, e))),
    };

    let save = move || {
        let Some((code, css)) = current.get_untracked() else {
            return report(Err(
                "Apply a preset or pasted CSS first, then save it.".into()
            ));
        };
        let trimmed = name.get_untracked();
        let result = check_limit(count.get_untracked(), 1, env.max_saved.get_untracked())
            .and_then(|_| ctx.save_theme(&trimmed, code.as_deref(), css.as_deref()))
            .map_err(|e| e.to_string());
        match result {
            Ok(slug) => {
                name.set(String::new());
                if let Some(theme) = untrack(|| ctx.saved().into_iter().find(|t| t.slug == slug)) {
                    env.on_save.run(theme);
                }
                report(Ok(format!("Saved \"{}\".", trimmed.trim())));
            }
            Err(e) => report(Err(e)),
        }
    };

    let rename = move |slug: String| match ctx.rename_theme(&slug, &rename_to.get_untracked()) {
        Ok(()) => {
            editing.set(None);
            report(Ok("Renamed.".into()));
        }
        Err(e) => report(Err(e.to_string())),
    };

    view! {
        <Block env=env title="My themes" part="saved">
            <div class="vdl-theme-customizer__save">
                <Field>
                    <FieldLabel html_for=name_id.clone()>"Save current as..."</FieldLabel>
                    <div class="vdl-theme-customizer__row vdl-theme-customizer__row--nowrap">
                        <div class="vdl-theme-customizer__grow">
                            <Input
                                id=name_id
                                value=name
                                placeholder="Name this look"
                                autocomplete="off"
                                on_enter=move |_| {
                                    if can_save.get_untracked() {
                                        save()
                                    }
                                }
                            />
                        </div>
                        <Button
                            disabled=Signal::derive(move || !can_save.get())
                            on_click=move |_| save()
                        >
                            "Save"
                        </Button>
                    </div>
                </Field>
                {move || {
                    let text = if full.get() {
                        format!(
                            "The limit of {} saved themes is reached; delete one to save another.",
                            env.max_saved.get().unwrap_or(0),
                        )
                    } else if env.compact.get() {
                        return None;
                    } else if current.get().is_none() {
                        "Apply a preset or pasted CSS to save it.".to_string()
                    } else {
                        "Saves the preset or CSS on the page now.".to_string()
                    };
                    Some(view! { <p class="vdl-theme-customizer__hint" data-testid="save-hint">{text}</p> })
                }}
            </div>

            <div class="vdl-theme-customizer__saved-head">
                <h4 class="vdl-theme-customizer__subtitle">"Saved"</h4>
                <Badge variant=BadgeVariant::Secondary>
                    <span data-testid="saved-count">
                        {move || match env.max_saved.get() {
                            Some(m) => format!("{} / {m}", count.get()),
                            None => count.get().to_string(),
                        }}
                    </span>
                </Badge>
            </div>
            <ul class="vdl-theme-customizer__list" data-testid="saved-list">
                {move || {
                    let saved = ctx.saved();
                    if saved.is_empty() {
                        return view! { <li class="vdl-theme-customizer__hint">"Nothing saved yet."</li> }
                            .into_any();
                    }
                    saved
                        .into_iter()
                        .map(|t| {
                            let slug = StoredValue::new(t.slug.clone());
                            let current_name = StoredValue::new(t.name.clone());
                            let swatch_slug = t.slug.clone();
                            let detail = StoredValue::new(format!(
                                "{}{}",
                                t.code.clone().unwrap_or_default(),
                                if t.css.is_some() {
                                    if t.code.is_some() { " + CSS" } else { "CSS" }
                                } else {
                                    ""
                                },
                            ));
                            let is_editing = Signal::derive(move || {
                                editing.get().as_deref() == Some(slug.get_value().as_str())
                            });
                            let active = Signal::derive(move || ctx.theme.get() == slug.get_value());
                            view! {
                                <li data-slug=t.slug.clone()>
                                    <Item variant=ItemVariant::Outline size=Size::Small>
                                        <ItemMedia>
                                            <span class="vdl-theme-swatch" data-theme=swatch_slug aria-hidden="true">
                                                <crate::theme_switcher::SwatchDots />
                                            </span>
                                        </ItemMedia>
                                        <ItemContent>
                                            {move || {
                                                if is_editing.get() {
                                                    view! {
                                                        <Input
                                                            value=rename_to
                                                            aria_label="New name"
                                                            size=Size::Small
                                                            autocomplete="off"
                                                            on_enter=move |_| rename(slug.get_value())
                                                        />
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <ItemTitle>{current_name.get_value()}</ItemTitle>
                                                        <ItemDescription>{detail.get_value()}</ItemDescription>
                                                    }
                                                        .into_any()
                                                }
                                            }}
                                        </ItemContent>
                                        <ItemActions>
                                            {move || {
                                                if is_editing.get() {
                                                    view! {
                                                        <Button size=Size::Small on_click=move |_| rename(slug.get_value())>
                                                            "Save name"
                                                        </Button>
                                                        <Button
                                                            size=Size::Small
                                                            variant=ButtonVariant::Ghost
                                                            on_click=move |_| editing.set(None)
                                                        >
                                                            "Cancel"
                                                        </Button>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <Button
                                                            size=Size::Small
                                                            variant=ButtonVariant::Secondary
                                                            selected=active
                                                            on_click=move |_| ctx.choose(slug.get_value())
                                                        >
                                                            {move || if active.get() { "Applied" } else { "Apply" }}
                                                        </Button>
                                                        <Button
                                                            size=Size::Small
                                                            variant=ButtonVariant::Outline
                                                            aria_label=format!("Rename {}", current_name.get_value())
                                                            on_click={
                                                                move |_| {
                                                                    rename_to.set(current_name.get_value());
                                                                    editing.set(Some(slug.get_value()));
                                                                }
                                                            }
                                                        >
                                                            "Rename"
                                                        </Button>
                                                        <Button
                                                            size=Size::Small
                                                            variant=ButtonVariant::Outline
                                                            aria_label=format!("Delete {}", current_name.get_value())
                                                            on_click={
                                                                move |_| {
                                                                    to_delete
                                                                        .set(Some((slug.get_value(), current_name.get_value())));
                                                                    confirm_open.set(true);
                                                                }
                                                            }
                                                        >
                                                            "Delete"
                                                        </Button>
                                                    }
                                                        .into_any()
                                                }
                                            }}
                                        </ItemActions>
                                    </Item>
                                </li>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </ul>

            <div role="status" aria-live="polite" class="vdl-theme-customizer__status">
                {move || {
                    message
                        .get()
                        .map(|(is_error, text)| {
                            if is_error {
                                view! {
                                    <Alert variant=AlertVariant::Destructive>
                                        <p class="text-sm" data-testid="saved-message">{text}</p>
                                    </Alert>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <p class="vdl-theme-customizer__ok" data-testid="saved-message">{text}</p>
                                }
                                    .into_any()
                            }
                        })
                }}
            </div>

            <Collapsible>
                <CollapsibleTrigger variant=ButtonVariant::Ghost size=Size::Small>
                    "Export or import"
                    <Icon name=IconName::ChevronDown size=12 />
                </CollapsibleTrigger>
                <CollapsibleContent>
                    <div class="vdl-theme-customizer__grid vdl-theme-customizer__transfer">
                        <div class="vdl-theme-customizer__stack">
                            <Textarea
                                value=export_text
                                rows=5
                                readonly=true
                                aria_label="Exported themes"
                                placeholder="Exported themes appear here"
                            />
                            <Button
                                variant=ButtonVariant::Outline
                                size=Size::Small
                                disabled=Signal::derive(move || count.get() == 0)
                                on_click=move |_| {
                                    export_text.set(ctx.export_themes());
                                }
                            >
                                <Icon name=IconName::Download size=14 />
                                "Export all"
                            </Button>
                        </div>
                        <div class="vdl-theme-customizer__stack">
                            <Textarea
                                value=import_text
                                rows=5
                                aria_label="Themes to import"
                                placeholder="Exported text, a preset code or shadcn CSS"
                            />
                            <Button
                                variant=ButtonVariant::Outline
                                size=Size::Small
                                disabled=Signal::derive(move || import_text.get().trim().is_empty())
                                on_click=move |_| {
                                    report(
                                        ctx
                                            .import_themes_max(
                                                &import_text.get_untracked(),
                                                "Imported",
                                                env.max_saved.get_untracked(),
                                            )
                                            .map(|n| {
                                                import_text.set(String::new());
                                                format!("Imported {n} theme(s).")
                                            })
                                            .map_err(|e| e.to_string()),
                                    )
                                }
                            >
                                <Icon name=IconName::Upload size=14 />
                                "Import"
                            </Button>
                        </div>
                    </div>
                </CollapsibleContent>
            </Collapsible>

            <AlertDialog open=confirm_open>
                <AlertDialogHeader>
                    <AlertDialogTitle>"Delete this theme?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        {move || {
                            let name = to_delete.get().map(|(_, n)| n).unwrap_or_default();
                            format!("\"{name}\" will be removed from your saved themes. This cannot be undone.")
                        }}
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                    <AlertDialogAction
                        variant=ButtonVariant::Destructive
                        on_click=move |_| {
                            if let Some((slug, name)) = to_delete.get_untracked() {
                                ctx.delete_theme(&slug);
                                report(Ok(format!("Deleted \"{name}\".")));
                            }
                        }
                    >
                        "Delete"
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialog>
        </Block>
    }
}

/// A small card of components, in the page's theme.
#[component]
fn PreviewCard(env: Env) -> impl IntoView {
    let text = RwSignal::new("Sample input".to_string());
    view! {
        <Block env=env title="Preview" part="preview">
            <Card class="vdl-theme-customizer__preview">
                <CardHeader>
                    <CardTitle>"Card title"</CardTitle>
                    <CardDescription>"Body text in the theme's font."</CardDescription>
                </CardHeader>
                <CardContent class="vdl-theme-customizer__stack">
                    <div class="vdl-theme-customizer__row">
                        <Button>"Primary"</Button>
                        <Button variant=ButtonVariant::Outline>"Outline"</Button>
                        <Badge>"Badge"</Badge>
                        <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                    </div>
                    <Input value=text aria_label="Sample input" />
                    <span class="vdl-theme-customizer__bars" aria-hidden="true">
                        <i style="background:var(--chart-1)"></i>
                        <i style="background:var(--chart-2)"></i>
                        <i style="background:var(--chart-3)"></i>
                        <i style="background:var(--chart-4)"></i>
                        <i style="background:var(--chart-5)"></i>
                    </span>
                </CardContent>
            </Card>
        </Block>
    }
}
