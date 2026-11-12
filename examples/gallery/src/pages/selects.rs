use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Label, Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator,
    SelectTrigger, SelectValue, Size,
};

const SIZES: [(Size, &str); 4] = [
    (Size::XSmall, "xs (24px)"),
    (Size::Small, "sm (28px)"),
    (Size::Medium, "md (32px, default)"),
    (Size::Large, "lg (36px)"),
];

const FRUITS: [&str; 5] = ["Apple", "Banana", "Blueberry", "Grapes", "Pineapple"];

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

#[component]
pub fn SelectsPage() -> impl IntoView {
    let fruit = RwSignal::new(String::from("banana"));
    let changes = RwSignal::new(0_u32);
    let open = RwSignal::new(false);
    let plan = RwSignal::new(String::new());
    let scrolled = RwSignal::new(String::from("item-30"));

    view! {
        <Page
            title="Selects"
            summary="Select, SelectTrigger, SelectValue, SelectContent, SelectItem, SelectGroup, SelectLabel and SelectSeparator, on the select primitive. Keyboard: open with Enter, Space or the arrow keys; arrows, Home, End, PageUp and PageDown move; type to jump to an item; Enter picks; Escape closes and returns focus to the trigger."
        >
            <Section title="Basic" description="Uncontrolled with a placeholder. The trigger turns muted while nothing is selected.">
                <Select>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Select a fruit" />
                    </SelectTrigger>
                    <SelectContent>
                        {FRUITS
                            .into_iter()
                            .map(|f| view! { <SelectItem value=f.to_lowercase()>{f}</SelectItem> })
                            .collect_view()}
                    </SelectContent>
                </Select>
            </Section>

            <Section title="Sizes" description="The size prop is data-size on the trigger.">
                <div class="grid gap-3">
                    {SIZES
                        .into_iter()
                        .map(|(size, label)| {
                            view! {
                                <Row label=label>
                                    <Select default_value="banana" items=fruit_items()>
                                        <SelectTrigger size=size class="w-[180px]">
                                            <SelectValue placeholder="Select a fruit" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            {FRUITS
                                                .into_iter()
                                                .map(|f| view! { <SelectItem value=f.to_lowercase()>{f}</SelectItem> })
                                                .collect_view()}
                                        </SelectContent>
                                    </Select>
                                </Row>
                            }
                        })
                        .collect_view()}
                </div>
            </Section>

            <Section
                title="Groups, labels and separators"
                description="Grouped items with a label each and a separator between groups. Disabled items are skipped by the keyboard."
            >
                <Select default_value="cet" items=timezone_items()>
                    <SelectTrigger class="w-[280px] max-w-full">
                        <SelectValue placeholder="Select a time zone" />
                    </SelectTrigger>
                    <SelectContent>
                        {TIMEZONES
                            .into_iter()
                            .enumerate()
                            .map(|(i, (group, zones))| {
                                view! {
                                    {(i > 0).then(|| view! { <SelectSeparator /> })}
                                    <SelectGroup>
                                        <SelectLabel>{group}</SelectLabel>
                                        {zones
                                            .iter()
                                            .map(|z| {
                                                view! {
                                                    <SelectItem value=zone_value(z) disabled=z.starts_with("Mountain")>
                                                        {*z}
                                                    </SelectItem>
                                                }
                                            })
                                            .collect_view()}
                                    </SelectGroup>
                                }
                            })
                            .collect_view()}
                    </SelectContent>
                </Select>
            </Section>

            <Section
                title="Controlled, with a readout"
                description="value and on_value_change drive it from a signal; the readout and the change counter show what the page holds. open is controlled too."
            >
                <div class="grid gap-3">
                    <Select
                        value=fruit
                        on_value_change=Callback::new(move |v: String| {
                            changes.update(|c| *c += 1);
                            fruit.set(v);
                        })
                        open=open
                        on_open_change=Callback::new(move |o: bool| open.set(o))
                        items=fruit_items()
                    >
                        <SelectTrigger class="w-[180px]">
                            <SelectValue placeholder="Select a fruit" />
                        </SelectTrigger>
                        <SelectContent>
                            {FRUITS
                                .into_iter()
                                .map(|f| view! { <SelectItem value=f.to_lowercase()>{f}</SelectItem> })
                                .collect_view()}
                        </SelectContent>
                    </Select>
                    <span class="text-sm text-muted-foreground" data-testid="readout">
                        "value: " {move || fruit.get()} " | changes: " {move || changes.get()} " | open: "
                        {move || open.get().to_string()}
                    </span>
                </div>
            </Section>

            <Section title="States" description="Disabled, a disabled item, and invalid (aria-invalid).">
                <div class="flex flex-wrap items-center gap-3">
                    <Select default_value="apple" disabled=true items=fruit_items()>
                        <SelectTrigger class="w-[180px]" aria_label="Disabled select">
                            <SelectValue placeholder="Disabled" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="apple">"Apple"</SelectItem>
                        </SelectContent>
                    </Select>
                    <Select>
                        <SelectTrigger class="w-[180px]" invalid=true>
                            <SelectValue placeholder="Invalid" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="a">"Option A"</SelectItem>
                            <SelectItem value="b" disabled=true>"Option B (disabled)"</SelectItem>
                            <SelectItem value="c">"Option C"</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </Section>

            <Section
                title="Long list"
                description="Fifty items: the list stops at the space the page has and scrolls. Opening scrolls the selected item (30) into view, and typing a number jumps to it."
            >
                <Select
                    value=scrolled
                    on_value_change=Callback::new(move |v: String| scrolled.set(v))
                    items=long_items()
                >
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Pick an item" />
                    </SelectTrigger>
                    <SelectContent>
                        {(1..=50)
                            .map(|i| view! { <SelectItem value=format!("item-{i}")>{format!("Item {i}")}</SelectItem> })
                            .collect_view()}
                    </SelectContent>
                </Select>
            </Section>

            <Section title="In a form row" description="A label above the select, with name set so the value is submitted with the form (as a hidden input).">
                <form class="grid max-w-sm gap-3" on:submit=|ev| ev.prevent_default()>
                    <div class="grid gap-2">
                        <Label>"Plan"</Label>
                        <Select
                            name="plan"
                            required=true
                            value=plan
                            on_value_change=Callback::new(move |v: String| plan.set(v))
                            items=plan_items()
                        >
                            <SelectTrigger class="w-full" aria_label="Plan">
                                <SelectValue placeholder="Choose a plan" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="free">"Free"</SelectItem>
                                <SelectItem value="pro">"Pro"</SelectItem>
                                <SelectItem value="team">"Team"</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                    <span class="text-sm text-muted-foreground" data-testid="plan-readout">
                        "plan: " {move || if plan.get().is_empty() { "(none)".to_string() } else { plan.get() }}
                    </span>
                </form>
            </Section>
            <SelectLabelExamples />
        </Page>
    }
}

fn fruit_items() -> Vec<(String, String)> {
    FRUITS
        .iter()
        .map(|f| (f.to_lowercase(), f.to_string()))
        .collect()
}

fn zone_value(label: &str) -> String {
    label
        .split('(')
        .nth(1)
        .map(|s| s.trim_end_matches(')').to_lowercase())
        .unwrap_or_else(|| label.to_lowercase())
}

fn timezone_items() -> Vec<(String, String)> {
    TIMEZONES
        .iter()
        .flat_map(|(_, zones)| zones.iter().map(|z| (zone_value(z), z.to_string())))
        .collect()
}

fn long_items() -> Vec<(String, String)> {
    (1..=50)
        .map(|i| (format!("item-{i}"), format!("Item {i}")))
        .collect()
}

fn plan_items() -> Vec<(String, String)> {
    [("free", "Free"), ("pro", "Pro"), ("team", "Team")]
        .into_iter()
        .map(|(v, l)| (v.to_string(), l.to_string()))
        .collect()
}

/// The trigger label before the list is first opened, an empty-value "All" item, `clearable`
/// and how the placeholder behaves with an empty value.
#[component]
fn SelectLabelExamples() -> impl IntoView {
    let with_items = RwSignal::new(String::from("pro"));
    let without_items = RwSignal::new(String::from("pro"));

    let all_value = RwSignal::new(String::new());
    let all_items = vec![
        (String::new(), "All plans".to_string()),
        ("free".to_string(), "Free".to_string()),
        ("pro".to_string(), "Pro".to_string()),
    ];

    let clearable_value = RwSignal::new(String::from("banana"));
    let clearable_log = RwSignal::new(String::from("none"));
    let uncontrolled_clear = RwSignal::new(String::from("(none)"));

    let empty_no_item = RwSignal::new(String::new());
    let fruit_items: Vec<(String, String)> = FRUITS
        .iter()
        .map(|f| (f.to_lowercase(), f.to_string()))
        .collect();

    view! {
        <Section
            title="Trigger label before the list opens"
            description="Items only exist in the page while the list is open, so without `items` a Select shows the raw value until the first open (a debug build also logs a console warning). Pass `items` as (value, label) pairs and the label shows from the start."
        >
            <div class="grid max-w-md gap-3">
                <Row label="With items">
                    <Select
                        attr:id="label-with-items"
                        value=with_items
                        on_value_change=move |v: String| with_items.set(v)
                        items=plan_items()
                    >
                        <SelectTrigger aria_label="With items" class="w-40">
                            <SelectValue placeholder="Plan" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="free">"Free"</SelectItem>
                            <SelectItem value="pro">"Pro"</SelectItem>
                            <SelectItem value="team">"Team"</SelectItem>
                        </SelectContent>
                    </Select>
                </Row>
                <Row label="Without items">
                    <Select
                        value=without_items
                        on_value_change=move |v: String| without_items.set(v)
                    >
                        <SelectTrigger aria_label="Without items" class="w-40">
                            <SelectValue placeholder="Plan" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="free">"Free"</SelectItem>
                            <SelectItem value="pro">"Pro"</SelectItem>
                            <SelectItem value="team">"Team"</SelectItem>
                        </SelectContent>
                    </Select>
                    <span class="text-xs text-muted-foreground">"shows `pro` until opened once"</span>
                </Row>
            </div>
        </Section>

        <Section
            title="Empty-string value (an \"All\" item)"
            description="An item may have the value \"\", so an All or Any choice needs no sentinel string. With an item or an `items` pair for \"\", the trigger shows its label (here All plans) while the value is empty."
        >
            <Row>
                <Select
                    value=all_value
                    on_value_change=move |v: String| all_value.set(v)
                    items=all_items
                >
                    <SelectTrigger attr:id="all-trigger" aria_label="Plan filter" class="w-44">
                        <SelectValue placeholder="Pick a plan" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="">"All plans"</SelectItem>
                        <SelectItem value="free">"Free"</SelectItem>
                        <SelectItem value="pro">"Pro"</SelectItem>
                    </SelectContent>
                </Select>
                <span id="all-readout" class="text-sm text-muted-foreground">
                    "value = " {move || format!("{:?}", all_value.get())}
                </span>
            </Row>
        </Section>

        <Section
            title="Placeholder with an empty value"
            description="With no item for the empty value, an empty string (or no value at all) shows the placeholder, muted. A select that has an empty-valued item shows that item's label instead, so the two never both appear: put the label you want in the \"\" item."
        >
            <Row>
                <Select
                    value=empty_no_item
                    on_value_change=move |v: String| empty_no_item.set(v)
                >
                    <SelectTrigger attr:id="empty-trigger" aria_label="Fruit" class="w-44">
                        <SelectValue placeholder="Select a fruit" />
                    </SelectTrigger>
                    <SelectContent>
                        {FRUITS
                            .iter()
                            .map(|f| view! { <SelectItem value=f.to_lowercase()>{*f}</SelectItem> })
                            .collect_view()}
                    </SelectContent>
                </Select>
                <span id="empty-readout" class="text-sm text-muted-foreground">
                    "value = " {move || format!("{:?}", empty_no_item.get())}
                </span>
            </Row>
        </Section>

        <Section
            title="clearable"
            description="Backspace or Delete on the focused trigger resets it, and a small X shows while there is a value. Clearing leaves nothing chosen (the placeholder shows) and calls on_value_change with an empty string. Works controlled and uncontrolled."
        >
            <div class="grid max-w-md gap-3">
                <Row label="Controlled">
                    <Select
                        clearable=true
                        value=clearable_value
                        on_value_change=move |v: String| {
                            clearable_log.set(format!("{v:?}"));
                            clearable_value.set(v);
                        }
                    >
                        <SelectTrigger attr:id="clear-trigger" aria_label="Fruit" class="w-44">
                            <SelectValue placeholder="Select a fruit" />
                        </SelectTrigger>
                        <SelectContent>
                            {FRUITS
                                .iter()
                                .map(|f| view! { <SelectItem value=f.to_lowercase()>{*f}</SelectItem> })
                                .collect_view()}
                        </SelectContent>
                    </Select>
                    <span id="clear-readout" class="text-sm text-muted-foreground">
                        "last change: " {move || clearable_log.get()}
                    </span>
                </Row>
                <Row label="Uncontrolled">
                    <Select
                        clearable=true
                        default_value="grapes"
                        items=fruit_items
                        on_value_change=move |v: String| {
                            uncontrolled_clear.set(if v.is_empty() { "(none)".into() } else { v })
                        }
                    >
                        <SelectTrigger attr:id="clear-trigger-2" aria_label="Fruit" class="w-44">
                            <SelectValue placeholder="Select a fruit" />
                        </SelectTrigger>
                        <SelectContent>
                            {FRUITS
                                .iter()
                                .map(|f| view! { <SelectItem value=f.to_lowercase()>{*f}</SelectItem> })
                                .collect_view()}
                        </SelectContent>
                    </Select>
                    <span class="text-sm text-muted-foreground">{move || uncontrolled_clear.get()}</span>
                </Row>
            </div>
        </Section>
    }
}
