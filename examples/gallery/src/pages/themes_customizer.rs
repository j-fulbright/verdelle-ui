//! The Themes page's sections for `ThemeCustomizer` (every option) and `SyncedStorage`.

use crate::section::{Row, Section};
use crate::server_demo::{ServerDemo, short};
use leptos::prelude::*;
use std::collections::BTreeMap;
use verdelle_ui::custom_theme::{CustomTheme, MemoryStorage, ThemeStorage, keys, save_custom};
use verdelle_ui::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, Size, Switch,
    SyncedStorage, Tabs, TabsContent, TabsList, TabsTrigger, ThemeApplied, ThemeCustomizer,
    ThemeMode, theme_init_script_seeded,
};

/// One section per option of `ThemeCustomizer`, then `SyncedStorage`.
#[component]
pub fn CustomizerSections() -> impl IntoView {
    view! {
        <FullPanel />
        <Placements />
        <ShowProps />
        <Playground />
        <MaxSaved />
        <ProviderStorage />
        <DebounceDemo />
        <FirstPaint />
    }
}

/// The whole panel with every section, plus the callbacks and font loading.
#[component]
fn FullPanel() -> impl IntoView {
    let applied = RwSignal::new("nothing yet".to_string());
    let saved = RwSignal::new("nothing yet".to_string());
    view! {
        <Section
            title="ThemeCustomizer"
            description="An Appearance panel in one component, under the ThemeProvider: theme and mode, a preset code box, fine tuning, pasted CSS, saved themes and a preview. Everything you apply becomes the page's theme (the provider's unsaved preview, kept across reloads) and Save keeps it under a name. on_apply and on_save are called back; load_fonts adds a stylesheet link for the preset's fonts."
        >
            <div id="customizer-full" class="max-w-3xl">
                <ThemeCustomizer
                    load_fonts=true
                    on_apply=move |a: ThemeApplied| {
                        applied
                            .set(
                                format!(
                                    "code {} / css {} / fonts {}",
                                    a.code.unwrap_or_else(|| "none".into()),
                                    if a.css.is_some() { "yes" } else { "none" },
                                    if a.fonts_url.is_some() { "url" } else { "none" },
                                ),
                            )
                    }
                    on_save=move |t: CustomTheme| saved.set(format!("{} ({})", t.name, t.slug))
                />
            </div>
            <p class="text-sm text-muted-foreground">
                "on_apply: " <code id="customizer-on-apply" class="font-mono">{move || applied.get()}</code>
                " / on_save: " <code id="customizer-on-save" class="font-mono">{move || saved.get()}</code>
            </p>
        </Section>
    }
}

/// compact, inside a Card, in a Tabs panel, and at phone width.
#[component]
fn Placements() -> impl IntoView {
    view! {
        <Section
            title="ThemeCustomizer: compact, in a Card, in Tabs, on a phone"
            description="compact tightens the spacing and drops the explanatory paragraphs (the line under each section title and the notes about what falls back and about Save); labels, controls, errors and the preset summary stay. First the same panel without and with compact, side by side; then a 28rem Card and a Tabs panel, and one exactly 390px wide."
        >
            <div class="grid items-start gap-6 lg:grid-cols-2">
                <div id="customizer-side-full" class="min-w-0 rounded-lg border p-4">
                    <p class="mb-3 text-xs font-medium text-muted-foreground">"compact = false (default)"</p>
                    <ThemeCustomizer show_preview=false />
                </div>
                <div id="customizer-side-compact" class="min-w-0 rounded-lg border p-4">
                    <p class="mb-3 text-xs font-medium text-muted-foreground">"compact = true"</p>
                    <ThemeCustomizer compact=true show_preview=false />
                </div>
            </div>
            <div class="grid items-start gap-6 lg:grid-cols-2">
                <Card class="max-w-md">
                    <CardHeader>
                        <CardTitle>"Appearance"</CardTitle>
                        <CardDescription>"compact, inside a Card."</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div id="customizer-card">
                            <ThemeCustomizer compact=true />
                        </div>
                    </CardContent>
                </Card>
                <Tabs default_value="appearance" class="max-w-md">
                    <TabsList>
                        <TabsTrigger value="general">"General"</TabsTrigger>
                        <TabsTrigger value="appearance">"Appearance"</TabsTrigger>
                    </TabsList>
                    <TabsContent value="general">
                        <p class="p-4 text-sm text-muted-foreground">"Other settings."</p>
                    </TabsContent>
                    <TabsContent value="appearance">
                        <div id="customizer-tabs" class="pt-4">
                            <ThemeCustomizer compact=true show_preview=false />
                        </div>
                    </TabsContent>
                </Tabs>
            </div>
            <div id="customizer-phone" class="w-[390px] max-w-full rounded-lg border p-4">
                <ThemeCustomizer />
            </div>
        </Section>
    }
}

/// Each `show_*` prop on its own.
#[component]
fn ShowProps() -> impl IntoView {
    let parts: [(&str, &str); 6] = [
        (
            "show_theme",
            "The theme switcher and the light / dark / system control.",
        ),
        (
            "show_preset",
            "The preset code box, its summary and the link to shadcn's creator.",
        ),
        (
            "show_fine_tuning",
            "Colours, radius, menu colour and accent, fonts.",
        ),
        ("show_css", "The pasted-CSS box with its live preview."),
        (
            "show_saved",
            "Save current as, the saved list, export and import.",
        ),
        ("show_preview", "The small card of components."),
    ];
    view! {
        <Section
            title="ThemeCustomizer: show_* props"
            description="Every section is on by default and has a bool prop. Here each one is the only section on (the others are false). The props are signals, so a section can come and go while the panel stays mounted: see the playground below."
        >
            <div class="grid items-start gap-6 md:grid-cols-2">
                {parts
                    .into_iter()
                    .enumerate()
                    .map(|(i, (name, what))| {
                        view! {
                            <div class="space-y-2 rounded-lg border p-4" id=format!("customizer-only-{name}")>
                                <div>
                                    <code class="font-mono text-sm font-medium">{name} "=true"</code>
                                    <p class="text-xs text-muted-foreground">{what}</p>
                                </div>
                                <ThemeCustomizer
                                    show_theme={i == 0}
                                    show_preset={i == 1}
                                    show_fine_tuning={i == 2}
                                    show_css={i == 3}
                                    show_saved={i == 4}
                                    show_preview={i == 5}
                                />
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </Section>
    }
}

/// Switches for every bool prop, on one live panel.
#[component]
fn Playground() -> impl IntoView {
    let theme = RwSignal::new(true);
    let preset = RwSignal::new(true);
    let tuning = RwSignal::new(false);
    let css = RwSignal::new(false);
    let saved = RwSignal::new(true);
    let preview = RwSignal::new(true);
    let compact = RwSignal::new(true);
    let toggle = |label: &'static str, sig: RwSignal<bool>| {
        view! {
            <label class="flex items-center gap-2 text-sm">
                <Switch checked=sig on_checked_change=move |v| sig.set(v) aria_label=label />
                <code class="font-mono text-xs">{label}</code>
            </label>
        }
    };
    view! {
        <Section
            title="ThemeCustomizer: playground"
            description="All the bool props as switches on one panel. State inside the panel (typed text, the draft preset) survives a section being hidden and shown."
        >
            <div id="customizer-playground" class="grid items-start gap-4 md:grid-cols-[14rem_1fr]">
                <div class="flex flex-col gap-3">
                    {toggle("show_theme", theme)} {toggle("show_preset", preset)}
                    {toggle("show_fine_tuning", tuning)} {toggle("show_css", css)}
                    {toggle("show_saved", saved)} {toggle("show_preview", preview)}
                    {toggle("compact", compact)}
                </div>
                <div class="max-w-xl min-w-0">
                    <ThemeCustomizer
                        show_theme=theme
                        show_preset=preset
                        show_fine_tuning=tuning
                        show_css=css
                        show_saved=saved
                        show_preview=preview
                        compact=compact
                    />
                </div>
            </div>
        </Section>
    }
}

/// `max_saved`.
#[component]
fn MaxSaved() -> impl IntoView {
    view! {
        <Section
            title="ThemeCustomizer: max_saved"
            description="max_saved=2: Save stops at two themes (the hint says why, and the count shows 2 / 2), and an import that would go over the limit adds nothing. Deleting one makes room."
        >
            <div id="customizer-max" class="max-w-xl">
                <ThemeCustomizer
                    max_saved=2usize
                    show_theme=false
                    show_fine_tuning=false
                    show_css=false
                    show_preview=false
                />
            </div>
        </Section>
    }
}

/// The page's provider is on a SyncedStorage with a visible log.
#[component]
fn ProviderStorage() -> impl IntoView {
    let server = use_context::<ServerDemo>().expect("the gallery provides ServerDemo");
    let log = server.log;
    let from_other_device = {
        let server = server.clone();
        move |_| {
            // What another device saved: the Rose theme, dark mode and one saved theme.
            let mut values: BTreeMap<String, String> = BTreeMap::new();
            values.insert(keys::THEME.into(), "from-the-server".into());
            values.insert(keys::MODE.into(), "dark".into());
            let theme = CustomTheme::build("From the server", Some("b501U2Nxg"), None)
                .expect("a valid preset code");
            let tmp = MemoryStorage::default();
            save_custom(&tmp, &[theme]);
            if let Some(json) = tmp.get(keys::CUSTOM) {
                values.insert(keys::CUSTOM.into(), json);
            }
            server.storage.load(values);
            server.log.update(|l| {
                l.push("load(3 values) simulated: values arrived from the server".into())
            });
        }
    };
    let reload = {
        let server = server.clone();
        move |_| server.load_stored()
    };
    view! {
        <Section
            title="SyncedStorage on this page's provider"
            description="This page's ThemeProvider uses a SyncedStorage: an in-memory map that calls on_set(key, value) and on_remove(key) for every change, so an app can write it to its database. Here the hooks append to the log below and write to localStorage (the pretend server), so a reload still finds your theme. Changes you make anywhere on this page show up in the log; loading what was stored sets nothing back (no echo)."
        >
            <div class="flex flex-wrap gap-2">
                <Button variant=ButtonVariant::Outline size=Size::Small on_click=from_other_device>
                    "Load from server (another device's theme)"
                </Button>
                <Button variant=ButtonVariant::Outline size=Size::Small on_click=reload>
                    "Reload what the pretend server stored"
                </Button>
                <Button variant=ButtonVariant::Ghost size=Size::Small on_click=move |_| log.set(Vec::new())>
                    "Clear log"
                </Button>
            </div>
            <pre
                id="synced-log"
                class="max-h-56 min-h-16 overflow-auto rounded-md border bg-muted p-3 font-mono text-xs"
                aria-live="polite"
            >
                {move || {
                    let lines = log.get();
                    if lines.is_empty() { "(no calls yet)".to_string() } else { lines.join("\n") }
                }}
            </pre>
        </Section>
    }
}

/// A standalone SyncedStorage with `debounce_ms`.
#[component]
fn DebounceDemo() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let storage = SyncedStorage::new([("seed".to_string(), "initial".to_string())])
        .on_set(move |k, v| log.update(|l| l.push(format!("on_set({k}, {})", short(v)))))
        .on_remove(move |k| log.update(|l| l.push(format!("on_remove({k})"))))
        .debounce_ms(800);
    let set = {
        let storage = storage.clone();
        move |k: &'static str, v: &'static str| {
            let storage = storage.clone();
            move |_| storage.set(k, v)
        }
    };
    let remove = {
        let storage = storage.clone();
        move |_| storage.remove("size")
    };
    let flush = {
        let storage = storage.clone();
        move |_| storage.flush()
    };
    let snapshot = {
        let storage = storage.clone();
        move || format!("{:?}", storage.snapshot())
    };
    view! {
        <Section
            title="SyncedStorage: debounce_ms, flush, snapshot"
            description="A standalone SyncedStorage with debounce_ms(800). Click quickly: the hooks run once per key, 800ms after the last change, with that key's last value. Setting a value it already has calls nothing. flush() calls the hooks now. snapshot() copies the contents."
        >
            <Row>
                <Button size=Size::Small variant=ButtonVariant::Outline on_click=set("color", "red")>
                    "set color=red"
                </Button>
                <Button size=Size::Small variant=ButtonVariant::Outline on_click=set("color", "blue")>
                    "set color=blue"
                </Button>
                <Button size=Size::Small variant=ButtonVariant::Outline on_click=set("size", "lg")>
                    "set size=lg"
                </Button>
                <Button size=Size::Small variant=ButtonVariant::Outline on_click=remove>
                    "remove size"
                </Button>
                <Button size=Size::Small on_click=flush>"flush()"</Button>
                <Button size=Size::Small variant=ButtonVariant::Ghost on_click=move |_| log.set(Vec::new())>
                    "Clear"
                </Button>
            </Row>
            <pre
                id="debounce-log"
                class="min-h-16 overflow-auto rounded-md border bg-muted p-3 font-mono text-xs"
            >
                {move || {
                    let lines = log.get();
                    if lines.is_empty() { "(no calls yet)".to_string() } else { lines.join("\n") }
                }}
            </pre>
            <p class="text-xs text-muted-foreground">
                "snapshot() = "
                <code id="debounce-snapshot" class="font-mono">
                    {move || {
                        let _ = log.get();
                        snapshot()
                    }}
                </code>
            </p>
        </Section>
    }
}

/// Seeding the first paint from the server.
#[component]
fn FirstPaint() -> impl IntoView {
    let mut seed: BTreeMap<String, String> = BTreeMap::new();
    seed.insert(keys::THEME.into(), "rose".into());
    seed.insert(keys::MODE.into(), "dark".into());
    let script = theme_init_script_seeded("neutral", ThemeMode::System, (None, None), &seed);
    view! {
        <Section
            title="SyncedStorage: first paint from the server"
            description="The no-flash script normally reads localStorage. For a theme kept on a server, build the SyncedStorage from the server's values while rendering, and either leave script=true on the provider (it prints a script seeded from the storage) or print theme_init_script_seeded in the document head with the same map. This is what that script looks like for a person whose server record says theme=rose, mode=dark (this gallery keeps localStorage and prints the plain script instead)."
        >
            <pre class="overflow-x-auto rounded-md border bg-muted p-3 text-xs"><code>{FIRST_PAINT_EXAMPLE}</code></pre>
            <pre
                id="seeded-script"
                class="max-h-40 overflow-auto whitespace-pre-wrap break-all rounded-md border bg-muted p-3 font-mono text-xs"
            >
                {script}
            </pre>
        </Section>
    }
}

const FIRST_PAINT_EXAMPLE: &str = r#"// On the server, while rendering a person's page (their saved settings from your database):
let values: Vec<(String, String)> = load_theme_settings(user_id);   // keys from verdelle_ui::custom_theme::keys
let storage = SyncedStorage::new(values.clone())
    .on_set(|key, value| save_theme_setting(key, value))             // write through (browser side)
    .on_remove(|key| delete_theme_setting(key))
    .debounce_ms(500);

view! {
    // The provider starts from the storage (theme, mode, saved themes) and prints a seeded
    // no-flash script, so the server's HTML already has the person's look.
    <ThemeProvider storage=storage.clone().into_arc()> <App /> </ThemeProvider>
}

// Or print the script yourself in <head> (earliest) and pass script=false to the provider:
let js = theme_init_script_seeded("neutral", ThemeMode::System, (None, None), &storage.snapshot());"#;
