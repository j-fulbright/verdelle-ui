use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use std::time::Duration;
use verdelle_ui::{
    Button, ButtonVariant, Combobox, ComboboxChipList, ComboboxChips, ComboboxChipsInput,
    ComboboxContent, ComboboxEmpty, ComboboxGroup, ComboboxInput, ComboboxItem, ComboboxLabel,
    ComboboxList, ComboboxSeparator, FilterQuery, Label, Side, contains_filter, fuzzy_filter,
    starts_with_filter,
};

const FRAMEWORKS: [(&str, &str); 5] = [
    ("next", "Next.js"),
    ("sveltekit", "SvelteKit"),
    ("nuxt", "Nuxt.js"),
    ("remix", "Remix"),
    ("astro", "Astro"),
];

const TIMEZONES: [(&str, &[&str]); 3] = [
    (
        "North America",
        &[
            "Eastern (EST)",
            "Central (CST)",
            "Mountain (MST)",
            "Pacific (PST)",
        ],
    ),
    (
        "Europe and Africa",
        &[
            "Greenwich (GMT)",
            "Central European (CET)",
            "Eastern European (EET)",
        ],
    ),
    (
        "Asia",
        &["India (IST)", "China (CST)", "Japan (JST)", "Korea (KST)"],
    ),
];

const REGIONS: [&str; 4] = ["us-east-1", "us-west-2", "eu-west-1", "ap-south-1"];
const ROLES: [&str; 4] = ["web", "api", "worker", "db"];

/// One server of a made-up fleet: its name, region and role, and whether it is being drained
/// (and so cannot be picked).
#[derive(Clone)]
struct Server {
    name: String,
    region: &'static str,
    role: &'static str,
    draining: bool,
}

fn fleet() -> Vec<Server> {
    let mut servers = Vec::new();
    for (r, region) in REGIONS.into_iter().enumerate() {
        for (k, role) in ROLES.into_iter().enumerate() {
            for n in 1..=4 {
                servers.push(Server {
                    name: format!("{role}-{region}-{n:02}"),
                    region,
                    role,
                    draining: (r + k + n) % 9 == 0,
                });
            }
        }
    }
    servers
}

fn server_labels() -> Vec<(String, String)> {
    fleet()
        .into_iter()
        .map(|s| (s.name.clone(), s.name))
        .collect()
}

fn framework_items() -> Vec<(String, String)> {
    FRAMEWORKS
        .into_iter()
        .map(|(v, l)| (v.to_string(), l.to_string()))
        .collect()
}

fn framework_options() -> impl IntoView {
    FRAMEWORKS
        .into_iter()
        .map(
            |(value, label)| view! { <ComboboxItem value=value label=label>{label}</ComboboxItem> },
        )
        .collect_view()
}

#[component]
fn ServerItems() -> impl IntoView {
    fleet()
        .into_iter()
        .map(|s| {
            let region = s.region;
            view! {
                <ComboboxItem
                    value=s.name.clone()
                    label=s.name.clone()
                    keywords=vec![s.region.to_string(), s.role.to_string()]
                    disabled=s.draining
                >
                    <span class="flex-1">{s.name.clone()}</span>
                    <span class="text-xs text-muted-foreground">
                        {if s.draining { "draining".to_string() } else { region.to_string() }}
                    </span>
                </ComboboxItem>
            }
        })
        .collect_view()
}

#[component]
pub fn ComboboxesPage() -> impl IntoView {
    let controlled = RwSignal::new(String::from("remix"));
    let typed = RwSignal::new(String::new());
    let open = RwSignal::new(false);
    let changes = RwSignal::new(0_u32);
    let cleared = RwSignal::new(String::from("astro"));
    let server = RwSignal::new(String::from("api-eu-west-1-02"));
    let picks = RwSignal::new(vec![String::from("next"), String::from("astro")]);
    let filter_mode = RwSignal::new("contains");
    let submitted = RwSignal::new(String::from("(not submitted)"));

    // Async: results arrive after a delay; the newest request wins.
    let results = RwSignal::new(fleet().into_iter().take(12).collect::<Vec<_>>());
    let loading = RwSignal::new(false);
    let request = RwSignal::new(0_u32);
    let on_search = Callback::new(move |query: String| {
        let id = request.get_untracked() + 1;
        request.set(id);
        loading.set(true);
        set_timeout(
            move || {
                if request.get_untracked() != id {
                    return;
                }
                let q = query.to_lowercase();
                let found: Vec<_> = fleet()
                    .into_iter()
                    .filter(|s| s.name.contains(&q))
                    .take(12)
                    .collect();
                results.set(found);
                loading.set(false);
            },
            Duration::from_millis(400),
        );
    });

    let custom_filter = Callback::new(move |q: FilterQuery| match filter_mode.get() {
        "starts with" => starts_with_filter(&q),
        "fuzzy" => fuzzy_filter(&q),
        _ => contains_filter(&q),
    });

    view! {
        <Page
            title="Comboboxes"
            summary="Combobox, ComboboxInput, ComboboxChips, ComboboxContent, ComboboxList, ComboboxItem, ComboboxGroup, ComboboxLabel, ComboboxSeparator and ComboboxEmpty, on the combobox primitive (a text input that filters a list; focus stays in the input and the list is driven with aria-activedescendant). Keyboard: type to filter; ArrowDown and ArrowUp open and move (Ctrl or Cmd jumps to the first or last); PageUp and PageDown move ten; Enter picks; Escape closes; Backspace on an empty input removes the last chip."
        >
            <Section title="Basic" description="Uncontrolled. Typing opens the list and filters it (every typed word must appear in the item's label or keywords); Enter or a click picks; the chevron toggles the list.">
                <Combobox items=framework_items()>
                    <ComboboxInput placeholder="Select a framework" aria_label="Framework" class="w-[220px]" />
                    <ComboboxContent>
                        <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                        <ComboboxList>{framework_options()}</ComboboxList>
                    </ComboboxContent>
                </Combobox>
            </Section>

            <Section title="Clearable" description="show_clear swaps the chevron for a clear button once there is something to clear. default_value starts it filled, and items gives the label for the closed input.">
                <Combobox
                    items=framework_items()
                    default_value="astro"
                    on_value_change={move |v: Option<String>| cleared.set(v.unwrap_or_else(|| "(cleared)".into()))}
                >
                    <ComboboxInput placeholder="Select a framework" aria_label="Clearable framework" show_clear=true class="w-[220px]" />
                    <ComboboxContent>
                        <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                        <ComboboxList>{framework_options()}</ComboboxList>
                    </ComboboxContent>
                </Combobox>
                <p class="text-sm text-muted-foreground">
                    "last change: " <span id="cb-cleared-readout">{move || cleared.get()}</span>
                </p>
            </Section>

            <Section title="Without the trigger button" description="show_trigger=false leaves a plain search field; the list opens as you type or click it (open_on_input_click).">
                <Combobox items=framework_items()>
                    <ComboboxInput placeholder="Search frameworks" aria_label="Search only" show_trigger=false class="w-[220px]" />
                    <ComboboxContent>
                        <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                        <ComboboxList>{framework_options()}</ComboboxList>
                    </ComboboxContent>
                </Combobox>
            </Section>

            <Section title="Groups" description="ComboboxGroup with a ComboboxLabel; a group hides while none of its items match, and separators hide while you search.">
                <Combobox>
                    <ComboboxInput placeholder="Select a timezone" aria_label="Timezone" class="w-[260px]" />
                    <ComboboxContent>
                        <ComboboxEmpty>"No timezone found."</ComboboxEmpty>
                        <ComboboxList>
                            {TIMEZONES
                                .into_iter()
                                .enumerate()
                                .map(|(i, (group, zones))| {
                                    view! {
                                        {(i > 0).then(|| view! { <ComboboxSeparator /> })}
                                        <ComboboxGroup>
                                            <ComboboxLabel>{group}</ComboboxLabel>
                                            {zones
                                                .iter()
                                                .map(|z| view! { <ComboboxItem value=*z>{*z}</ComboboxItem> })
                                                .collect_view()}
                                        </ComboboxGroup>
                                    }
                                })
                                .collect_view()}
                        </ComboboxList>
                    </ComboboxContent>
                </Combobox>
            </Section>

            <Section title="Server picker" description="A long list with a search box, the shape of a server dropdown: 64 servers, matched on name, region and role (try 'eu db' or 'worker 02'); some are draining and cannot be picked. The list scrolls and keeps the highlight in view; PageDown moves ten.">
                <Combobox
                    items=server_labels()
                    value=server
                    on_value_change={move |v: Option<String>| server.set(v.unwrap_or_default())}
                >
                    <ComboboxInput placeholder="Pick a server" aria_label="Server" show_clear=true class="w-[300px] max-w-full" />
                    <ComboboxContent>
                        <ComboboxEmpty>"No server matches."</ComboboxEmpty>
                        <ComboboxList><ServerItems /></ComboboxList>
                    </ComboboxContent>
                </Combobox>
                <p class="text-sm text-muted-foreground">
                    "selected: " <span id="cb-server-readout">{move || server.get()}</span>
                </p>
            </Section>

            <Section title="Controlled" description="value, input_value and open all controlled. The buttons set them from outside.">
                <div class="grid gap-3">
                    <Combobox
                        items=framework_items()
                        value=controlled
                        on_value_change={move |v: Option<String>| {
                            controlled.set(v.unwrap_or_default());
                            changes.update(|c| *c += 1);
                        }}
                        input_value=typed
                        on_input_value_change=move |t: String| typed.set(t)
                        open=open
                        on_open_change=move |o: bool| open.set(o)
                    >
                        <ComboboxInput placeholder="Select a framework" aria_label="Controlled framework" class="w-[220px]" />
                        <ComboboxContent>
                            <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                            <ComboboxList>{framework_options()}</ComboboxList>
                        </ComboboxContent>
                    </Combobox>
                    <Row>
                        <Button variant=ButtonVariant::Outline on_click=move |_| { controlled.set("nuxt".into()); typed.set("Nuxt.js".into()); }>
                            "Pick Nuxt"
                        </Button>
                        <Button variant=ButtonVariant::Outline on_click=move |_| { controlled.set(String::new()); typed.set(String::new()); }>
                            "Clear"
                        </Button>
                        <Button variant=ButtonVariant::Outline on_click=move |_| open.update(|o| *o = !*o)>
                            "Toggle list"
                        </Button>
                    </Row>
                    <p class="text-sm text-muted-foreground" id="cb-controlled-readout">
                        "value: " {move || format!("\"{}\"", controlled.get())}
                        " · text: " {move || format!("\"{}\"", typed.get())}
                        " · open: " {move || open.get().to_string()}
                        " · changes: " {move || changes.get()}
                    </p>
                </div>
            </Section>

            <Section title="Multiple, with chips" description="multiple holds a list; the input only searches. ComboboxChips is the frame, ComboboxChipList draws a chip per value, Backspace on the empty input removes the last. The list stays open while you pick, and picked items show a check.">
                <Combobox
                    multiple=true
                    items=framework_items()
                    values=picks
                    on_values_change={move |v: Vec<String>| picks.set(v)}
                >
                    <ComboboxChips class="w-[360px] max-w-full">
                        <ComboboxChipList />
                        <ComboboxChipsInput placeholder="Add framework" aria_label="Frameworks" />
                    </ComboboxChips>
                    <ComboboxContent chips=true>
                        <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                        <ComboboxList>{framework_options()}</ComboboxList>
                    </ComboboxContent>
                </Combobox>
                <p class="text-sm text-muted-foreground">
                    "values: " <span id="cb-multi-readout">{move || picks.get().join(", ")}</span>
                </p>
            </Section>

            <Section title="Custom filter" description="filter takes a closure over the item's label, keywords and the typed text. The three provided matchers are contains (default), starts with, and fuzzy (letters in order: 'gh' finds 'GitHub').">
                <div class="grid gap-3">
                    <Row>
                        {["contains", "starts with", "fuzzy"]
                            .into_iter()
                            .map(|mode| {
                                view! {
                                    <Button
                                        variant=ButtonVariant::Outline
                                        selected=Signal::derive(move || filter_mode.get() == mode)
                                        on_click=move |_| filter_mode.set(mode)
                                    >
                                        {mode}
                                    </Button>
                                }
                            })
                            .collect_view()}
                    </Row>
                    <Combobox filter=custom_filter>
                        <ComboboxInput placeholder="Find a tool" aria_label="Tool" class="w-[260px]" />
                        <ComboboxContent>
                            <ComboboxEmpty>"Nothing matches."</ComboboxEmpty>
                            <ComboboxList>
                                {["GitHub", "GitLab", "Grafana", "Jira", "Jenkins", "Sentry", "Slack"]
                                    .into_iter()
                                    .map(|t| view! { <ComboboxItem value=t>{t}</ComboboxItem> })
                                    .collect_view()}
                            </ComboboxList>
                        </ComboboxContent>
                    </Combobox>
                </div>
            </Section>

            <Section title="Async results" description="should_filter=false: the list shows whatever you render, here results that arrive 400ms after each keystroke (the newest request wins). on_input_value_change starts the search.">
                <Combobox should_filter=false on_input_value_change=on_search>
                    <ComboboxInput placeholder="Search servers..." aria_label="Remote server" class="w-[300px] max-w-full" />
                    <ComboboxContent>
                        <ComboboxEmpty>
                            {move || if loading.get() { "Searching..." } else { "No servers found." }}
                        </ComboboxEmpty>
                        <ComboboxList>
                            <For
                                each=move || results.get()
                                key=|s| s.name.clone()
                                children=|s| {
                                    view! {
                                        <ComboboxItem value=s.name.clone() disabled=s.draining>
                                            {s.name.clone()}
                                        </ComboboxItem>
                                    }
                                }
                            />
                        </ComboboxList>
                    </ComboboxContent>
                </Combobox>
            </Section>

            <Section title="Options" description="auto_highlight=false leaves nothing highlighted until you press an arrow; loop_around=false stops the arrows at the ends; open_on_input_click=false opens only from typing, the arrows or the chevron; side puts the list above.">
                <div class="grid gap-3">
                    <Row label="auto_highlight off">
                        <Combobox items=framework_items() auto_highlight=false>
                            <ComboboxInput placeholder="Frameworks" aria_label="No auto highlight" class="w-[220px]" />
                            <ComboboxContent>
                                <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="loop_around off">
                        <Combobox items=framework_items() loop_around=false>
                            <ComboboxInput placeholder="Frameworks" aria_label="No loop" class="w-[220px]" />
                            <ComboboxContent>
                                <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="no open on click">
                        <Combobox items=framework_items() open_on_input_click=false>
                            <ComboboxInput placeholder="Frameworks" aria_label="Click does not open" class="w-[220px]" />
                            <ComboboxContent>
                                <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="side top">
                        <Combobox items=framework_items()>
                            <ComboboxInput placeholder="Frameworks" aria_label="Opens above" class="w-[220px]" />
                            <ComboboxContent side=Side::Top>
                                <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                </div>
            </Section>

            <Section title="Rich items" description="Item children can be anything; matching uses the label and keywords, not the markup. Here the keywords carry the team name.">
                <Combobox>
                    <ComboboxInput placeholder="Assign to..." aria_label="Assignee" class="w-[300px] max-w-full" />
                    <ComboboxContent>
                        <ComboboxEmpty>"Nobody found."</ComboboxEmpty>
                        <ComboboxList>
                            {[
                                ("ada", "Ada Lovelace", "Platform"),
                                ("grace", "Grace Hopper", "Compilers"),
                                ("linus", "Linus Torvalds", "Kernel"),
                                ("margaret", "Margaret Hamilton", "Flight software"),
                            ]
                                .into_iter()
                                .map(|(value, name, team)| {
                                    view! {
                                        <ComboboxItem value=value label=name keywords=vec![team.to_string()]>
                                            <span class="grid">
                                                <span>{name}</span>
                                                <span class="text-xs text-muted-foreground">{team}</span>
                                            </span>
                                        </ComboboxItem>
                                    }
                                })
                                .collect_view()}
                        </ComboboxList>
                    </ComboboxContent>
                </Combobox>
            </Section>

            <Section title="Disabled, read-only and invalid" description="disabled dims the field and ignores input; read_only shows the value but does not open; invalid on the input draws the destructive border and sets aria-invalid.">
                <div class="grid gap-3">
                    <Row label="disabled">
                        <Combobox items=framework_items() default_value="nuxt" disabled=true>
                            <ComboboxInput aria_label="Disabled framework" class="w-[220px]" />
                            <ComboboxContent>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="read only">
                        <Combobox items=framework_items() default_value="nuxt" read_only=true>
                            <ComboboxInput aria_label="Read-only framework" class="w-[220px]" />
                            <ComboboxContent>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="invalid">
                        <Combobox items=framework_items()>
                            <ComboboxInput placeholder="Required" aria_label="Invalid framework" invalid=true class="w-[220px]" />
                            <ComboboxContent>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="disabled chips">
                        <Combobox multiple=true items=framework_items() default_values=vec!["next".to_string()] disabled=true>
                            <ComboboxChips class="w-[360px] max-w-full">
                                <ComboboxChipList />
                                <ComboboxChipsInput aria_label="Disabled chips" />
                            </ComboboxChips>
                            <ComboboxContent chips=true>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                </div>
            </Section>

            <Section title="In a form" description="With name set the selection submits with the form: one hidden field for a single value, one per pick for multiple. required (single value) blocks submitting while empty.">
                <form
                    class="grid max-w-md gap-3"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        use leptos::wasm_bindgen::JsCast;
                        if let Some(form) = ev
                            .target()
                            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                            && let Ok(data) = leptos::web_sys::FormData::new_with_form(&form)
                        {
                            let single = data.get("framework").as_string().unwrap_or_else(|| "(absent)".into());
                            let many: Vec<String> = data
                                .get_all("tools")
                                .iter()
                                .filter_map(|v| v.as_string())
                                .collect();
                            submitted.set(format!("framework={single}; tools=[{}]", many.join(", ")));
                        }
                    }
                >
                    <div class="grid gap-2">
                        <Label>"Framework"</Label>
                        <Combobox items=framework_items() name="framework" required=true>
                            <ComboboxInput placeholder="Select a framework" aria_label="Form framework" />
                            <ComboboxContent>
                                <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                                <ComboboxList>{framework_options()}</ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </div>
                    <div class="grid gap-2">
                        <Label>"Tools"</Label>
                        <Combobox multiple=true name="tools" default_values=vec!["GitHub".to_string()]>
                            <ComboboxChips>
                                <ComboboxChipList />
                                <ComboboxChipsInput placeholder="Add tool" aria_label="Form tools" />
                            </ComboboxChips>
                            <ComboboxContent chips=true>
                                <ComboboxEmpty>"No tool found."</ComboboxEmpty>
                                <ComboboxList>
                                    {["GitHub", "GitLab", "Jira", "Sentry", "Slack"]
                                        .into_iter()
                                        .map(|t| view! { <ComboboxItem value=t>{t}</ComboboxItem> })
                                        .collect_view()}
                                </ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </div>
                    <div>
                        <button type="submit" id="cb-submit" class="vdl-btn" data-variant="default" data-size="md">
                            "Save"
                        </button>
                    </div>
                    <span id="cb-form-readout" class="text-sm text-muted-foreground">
                        "submitted: " {move || submitted.get()}
                    </span>
                </form>
            </Section>
        </Page>
    }
}
