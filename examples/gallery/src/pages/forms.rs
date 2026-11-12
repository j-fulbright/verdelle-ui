use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{Checkbox, Input, Label, Size, Textarea};

const SIZES: [(Size, &str); 4] = [
    (Size::XSmall, "xs (24px)"),
    (Size::Small, "sm (28px)"),
    (Size::Medium, "md (32px, default)"),
    (Size::Large, "lg (36px)"),
];

#[component]
pub fn FormsPage() -> impl IntoView {
    let name = RwSignal::new(String::from("feat/new-input"));
    let search = RwSignal::new(String::new());
    let count = RwSignal::new(String::from("2"));
    let notes = RwSignal::new(String::new());
    let agreed = RwSignal::new(true);
    let newsletter = RwSignal::new(false);

    view! {
        <Page title="Forms" summary="Input, textarea, label and checkbox. Inputs and textareas are native elements in a styled frame.">
            <Section title="Input sizes" description="One signal drives all four, so typing in one updates the rest.">
                <div class="grid max-w-md gap-3">
                    {SIZES
                        .into_iter()
                        .map(|(s, label)| {
                            view! {
                                <div class="space-y-1">
                                    <span class="text-xs text-muted-foreground">{label}</span>
                                    <Input
                                        value=name
                                        size=s
                                        aria_label=format!("Name, {}", s.as_str())
                                        placeholder="Name"
                                    />
                                </div>
                            }
                        })
                        .collect_view()}
                    <span class="text-sm text-muted-foreground">"value: " {move || name.get()}</span>
                </div>
            </Section>

            <Section title="Affixes, clear button, number, states">
                <div class="grid max-w-md gap-3">
                    <Input
                        value=search
                        kind="search"
                        cleanable=true
                        aria_label="Search"
                        placeholder="Search (prefix, suffix, clear button)"
                        prefix=|| view! { "⌕" }
                        suffix=|| view! { <kbd>"⌘K"</kbd> }
                    />
                    <div class="grid gap-2">
                        <Label html_for="gallery-count">"Number input"</Label>
                        <Input id="gallery-count" kind="number" min="0" max="10" step="1" value=count />
                    </div>
                    <Input value=name disabled=true aria_label="Disabled" />
                    <Input value=name readonly=true aria_label="Read only" />
                </div>
            </Section>

            <Section title="Textarea">
                <div class="max-w-md">
                    <Textarea value=notes aria_label="Notes" placeholder="Resizable vertically" />
                </div>
            </Section>

            <InputEvents />
            <TextareaEvents />

            <Section title="Label and checkbox" description="Clicking a label toggles its checkbox.">
                <div class="grid max-w-md gap-3">
                    <Row>
                        <Checkbox id="gallery-agree" checked=agreed />
                        <Label html_for="gallery-agree">"Accept the terms"</Label>
                    </Row>
                    <Row>
                        <Checkbox id="gallery-news" checked=newsletter />
                        <Label html_for="gallery-news">"Subscribe to the newsletter"</Label>
                    </Row>
                    <Row>
                        <Checkbox id="gallery-off" checked=RwSignal::new(false) disabled=true />
                        <Label html_for="gallery-off">"Disabled"</Label>
                    </Row>
                    <span class="text-sm text-muted-foreground">
                        "terms: " {move || agreed.get().to_string()} ", newsletter: "
                        {move || newsletter.get().to_string()}
                    </span>
                </div>
            </Section>

            <Section title="Checkbox, mixed state" description="indeterminate draws a dash for a select-all box when only some of its items are on. Clicking the parent sets all of them.">
                <MixedCheckboxes />
            </Section>
        </Page>
    }
}

/// A select-all checkbox over three items: it shows the mixed state when only some are on.
#[component]
fn MixedCheckboxes() -> impl IntoView {
    let items = [
        ("Email", RwSignal::new(true)),
        ("Push", RwSignal::new(false)),
        ("SMS", RwSignal::new(false)),
    ];
    let parent = RwSignal::new(false);
    let all = move || items.iter().all(|(_, on)| on.get());
    let any = move || items.iter().any(|(_, on)| on.get());
    // Items -> parent.
    Effect::new(move |_| parent.set(all()));
    // Parent -> items, only when the user toggled the parent (the items do not already agree).
    Effect::new(move |_| {
        let on = parent.get();
        if on != items.iter().all(|(_, item)| item.get_untracked()) {
            for (_, item) in items {
                item.set(on);
            }
        }
    });

    view! {
        <div class="grid max-w-md gap-3">
            <Row>
                <Checkbox
                    id="gallery-mixed-all"
                    checked=parent
                    indeterminate=Signal::derive(move || any() && !all())
                />
                <Label html_for="gallery-mixed-all">"All notifications"</Label>
            </Row>
            {items
                .into_iter()
                .map(|(label, on)| {
                    let id = format!("gallery-mixed-{label}");
                    view! {
                        <Row>
                            <span class="w-4"></span>
                            <Checkbox id=id.clone() checked=on />
                            <Label html_for=id>{label}</Label>
                        </Row>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Input `class`, `field_class` and the event callbacks.
#[component]
fn InputEvents() -> impl IntoView {
    let command = RwSignal::new(String::new());
    let inputs = RwSignal::new(0_u32);
    let committed = RwSignal::new(String::from("(none)"));
    let last_key = RwSignal::new(String::from("(none)"));
    let ran = RwSignal::new(Vec::<String>::new());

    view! {
        <Section
            title="Input: class, monospace and events"
            description="class goes on the frame (here a width), field_class on the native input (here font-mono). on_input runs on every edit, on_change when the browser commits (blur), on_keydown on every key and on_enter on Enter (not while an IME is composing). Type a command and press Enter."
        >
            <div class="grid max-w-md gap-3">
                <Input
                    value=command
                    class="max-w-xs"
                    field_class="font-mono"
                    aria_label="Command"
                    placeholder="git status, then Enter"
                    on_input=move |_| inputs.update(|n| *n += 1)
                    on_change=move |text: String| committed.set(text)
                    on_keydown=move |ev: leptos::ev::KeyboardEvent| last_key.set(ev.key())
                    on_enter=move |_| {
                        let text = command.get_untracked();
                        if !text.is_empty() {
                            ran.update(|v| v.push(text));
                            command.set(String::new());
                        }
                    }
                />
                <p class="text-xs text-muted-foreground">
                    "input events: " <span id="input-count">{move || inputs.get()}</span>
                    ", last key: " <span id="input-key">{move || last_key.get()}</span>
                    ", committed: " <span id="input-committed">{move || committed.get()}</span>
                </p>
                <p class="text-xs text-muted-foreground">
                    "ran: " <span id="input-ran">{move || ran.get().join(" | ")}</span>
                </p>
            </div>
        </Section>
    }
}

/// Textarea grow, max_rows and the event callbacks.
#[component]
fn TextareaEvents() -> impl IntoView {
    let chat = RwSignal::new(String::new());
    let sent = RwSignal::new(Vec::<String>::new());
    let pasted = RwSignal::new(String::from("(nothing yet)"));
    let grown = RwSignal::new(String::from("one\ntwo\nthree\nfour"));
    let capped = RwSignal::new(String::from(
        "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight",
    ));
    let fixed = RwSignal::new(String::new());

    view! {
        <Section
            title="Textarea: grow, max_rows and events"
            description="grow makes the field as tall as its text (a script measures it where CSS field-sizing is missing) and removes the resize handle; max_rows caps the height in lines, then the text scrolls. on_keydown, on_enter (Enter without Shift), on_paste and on_input are callbacks. The chat box sends on Enter and keeps Shift+Enter for a new line."
        >
            <div class="grid max-w-md gap-4">
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"grow (no limit)"</p>
                    <Textarea value=grown rows=2 grow=true aria_label="Grows" />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"grow with max_rows=5 (starts at 8 lines: scrolls)"</p>
                    <Textarea value=capped rows=2 grow=true max_rows=5 aria_label="Capped" />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"max_rows=4 without grow (still resizable by hand, capped by the field-sizing growth)"</p>
                    <Textarea value=fixed rows=2 max_rows=4 aria_label="Max rows only" />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Chat: Enter sends, Shift+Enter new line, mono font, grows to 6 lines"</p>
                    <Textarea
                        value=chat
                        rows=1
                        grow=true
                        max_rows=6
                        field_class="font-mono"
                        aria_label="Message"
                        placeholder="Write a message"
                        on_keydown=move |ev: leptos::ev::KeyboardEvent| {
                            if ev.key() == "Enter" && !ev.shift_key() {
                                ev.prevent_default();
                            }
                        }
                        on_enter=move |_| {
                            let text = chat.get_untracked();
                            if !text.trim().is_empty() {
                                sent.update(|v| v.push(text));
                                chat.set(String::new());
                            }
                        }
                        on_paste=move |ev: leptos::ev::ClipboardEvent| {
                            let text = ev.clipboard_data().and_then(|d| d.get_data("text").ok()).unwrap_or_default();
                            pasted.set(format!("{} characters", text.chars().count()));
                        }
                    />
                    <p class="text-xs text-muted-foreground">
                        "sent: " <span id="chat-sent">{move || sent.get().join(" | ").replace('\n', "\u{23ce}")}</span>
                        ", pasted: " <span id="chat-pasted">{move || pasted.get()}</span>
                    </p>
                </div>
            </div>
        </Section>
    }
}
