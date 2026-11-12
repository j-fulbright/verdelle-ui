use crate::section::{Page, Row, Section};
use leptos::{ev, prelude::*};
use verdelle_ui::{
    Button, ButtonVariant, Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput,
    CommandItem, CommandList, CommandSeparator, CommandShortcut, FilterQuery, Kbd, Popover,
    PopoverContent, PopoverTrigger, fuzzy_filter,
};

/// A 16px stroke icon from the paths of Lucide's (ISC licence) glyphs.
#[component]
fn Icon(path: &'static str) -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d=path />
        </svg>
    }
}

const CALENDAR: &str =
    "M8 2v4M16 2v4M3 10h18M5 4h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z";
const SMILE: &str =
    "M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0zM8 14s1.5 2 4 2 4-2 4-2M9 9h.01M15 9h.01";
const CALC: &str = "M6 2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2zM8 6h8M8 10h.01M12 10h.01M16 10h.01M8 14h.01M12 14h.01M16 14h.01M8 18h.01M12 18h.01M16 18h.01";
const USER: &str = "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8z";
const CARD: &str =
    "M3 5h18a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1zM2 10h20";
const COG: &str = "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h0a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h0a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v0a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z";

/// The same commands every example uses: two groups, shortcuts, one disabled item.
#[component]
fn Suggestions(on_select: Callback<String>) -> impl IntoView {
    view! {
        <CommandEmpty>"No results found."</CommandEmpty>
        <CommandGroup heading="Suggestions">
            <CommandItem value="calendar" keywords=vec!["date".to_string(), "schedule".to_string()] on_select=on_select>
                <Icon path=CALENDAR />
                "Calendar"
            </CommandItem>
            <CommandItem value="search-emoji" label="Search Emoji" keywords=vec!["smile".to_string()] on_select=on_select>
                <Icon path=SMILE />
                "Search Emoji"
            </CommandItem>
            <CommandItem value="calculator" disabled=true on_select=on_select>
                <Icon path=CALC />
                "Calculator"
                <CommandShortcut>"soon"</CommandShortcut>
            </CommandItem>
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Settings">
            <CommandItem value="profile" label="Profile" on_select=on_select>
                <Icon path=USER />
                "Profile"
                <CommandShortcut>"⌘P"</CommandShortcut>
            </CommandItem>
            <CommandItem value="billing" label="Billing" on_select=on_select>
                <Icon path=CARD />
                "Billing"
                <CommandShortcut>"⌘B"</CommandShortcut>
            </CommandItem>
            <CommandItem value="settings" label="Settings" on_select=on_select>
                <Icon path=COG />
                "Settings"
                <CommandShortcut>"⌘S"</CommandShortcut>
            </CommandItem>
        </CommandGroup>
    }
}

#[component]
pub fn CommandsPage() -> impl IntoView {
    let last = RwSignal::new(String::from("(none yet)"));
    let on_select = Callback::new(move |v: String| last.set(v));
    let highlighted = RwSignal::new(String::new());
    let dialog_open = RwSignal::new(false);
    let search = RwSignal::new(String::new());
    let fuzzy_search = RwSignal::new(String::new());
    let status_open = RwSignal::new(false);
    let status = RwSignal::new(String::from("Backlog"));

    // Ctrl or Cmd K opens the palette. Browser only: effects do not run while rendering on the server.
    Effect::new(move |_| {
        let handle = window_event_listener(ev::keydown, move |event| {
            if (event.meta_key() || event.ctrl_key()) && event.key().eq_ignore_ascii_case("k") {
                event.prevent_default();
                dialog_open.update(|o| *o = !*o);
            }
        });
        on_cleanup(move || handle.remove());
    });

    let fuzzy = Callback::new(|q: FilterQuery| fuzzy_filter(&q));

    view! {
        <Page
            title="Command"
            summary="Command, CommandInput, CommandList, CommandGroup, CommandItem, CommandShortcut, CommandSeparator, CommandEmpty and CommandDialog, on the command primitive (cmdk's idea: a search box over actions, focus stays in the input, one item is highlighted). Keyboard: type to filter; ArrowDown and ArrowUp move; Home and End jump to the first and last; PageUp and PageDown move ten; Ctrl or Cmd with an arrow jumps to the end; Enter runs the highlighted item."
        >
            <Section title="Basic" description="Groups, shortcuts, a disabled item and an empty state. The first match is highlighted as you type. Items are matched on their label and keywords ('date' finds Calendar; 'smile' finds Search Emoji). Separators hide while you search and a group hides when none of its items match.">
                <Command class="max-w-md rounded-xl border shadow-md">
                    <CommandInput placeholder="Type a command or search..." aria_label="Command" />
                    <CommandList label="Commands">
                        <Suggestions on_select=on_select />
                    </CommandList>
                </Command>
                <p class="text-sm text-muted-foreground">
                    "last run: " <span id="cmd-last-readout">{move || last.get()}</span>
                </p>
            </Section>

            <Section title="Command dialog" description="CommandDialog is a Command in a modal dialog, a third of the way down the page. Open it with the button or Ctrl/⌘ K; Escape or a click outside closes it, and the search starts empty each time.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| dialog_open.set(true)>
                        "Open command palette"
                    </Button>
                    <span class="text-sm text-muted-foreground">
                        "or press " <Kbd>"Ctrl"</Kbd> " " <Kbd>"K"</Kbd>
                    </span>
                </Row>
                <CommandDialog open=dialog_open>
                    <Command>
                        <CommandInput placeholder="Type a command or search..." aria_label="Palette search" />
                        <CommandList label="Palette commands">
                            <Suggestions
                                on_select=Callback::new(move |v: String| {
                                    last.set(v);
                                    dialog_open.set(false);
                                })
                            />
                        </CommandList>
                    </Command>
                </CommandDialog>
            </Section>

            <Section title="Controlled search and the highlighted value" description="search is controlled here (the buttons type for you) and on_value_change reports the highlighted item's value as it moves.">
                <div class="grid gap-3">
                    <Command
                        class="max-w-md rounded-xl border shadow-md"
                        search=search
                        on_search_change=move |s: String| search.set(s)
                        on_value_change={move |v: Option<String>| highlighted.set(v.unwrap_or_default())}
                    >
                        <CommandInput placeholder="Search..." aria_label="Controlled command" />
                        <CommandList label="Controlled commands">
                            <Suggestions on_select=on_select />
                        </CommandList>
                    </Command>
                    <Row>
                        <Button variant=ButtonVariant::Outline on_click=move |_| search.set("set".into())>
                            "Search 'set'"
                        </Button>
                        <Button variant=ButtonVariant::Outline on_click=move |_| search.set(String::new())>
                            "Clear"
                        </Button>
                    </Row>
                    <p class="text-sm text-muted-foreground" id="cmd-controlled-readout">
                        "search: " {move || format!("\"{}\"", search.get())}
                        " · highlighted: " {move || format!("\"{}\"", highlighted.get())}
                    </p>
                </div>
            </Section>

            <Section title="Custom filter and force_match" description="filter replaces the matcher (fuzzy here: letters in order, so 'pf' finds Profile). A force_match item stays while nothing matches, the way an 'ask AI' fallback does.">
                <Command class="max-w-md rounded-xl border shadow-md" filter=fuzzy search=fuzzy_search on_search_change=move |s: String| fuzzy_search.set(s)>
                    <CommandInput placeholder="Fuzzy search..." aria_label="Fuzzy command" />
                    <CommandList label="Fuzzy commands">
                        <CommandEmpty>"No results found."</CommandEmpty>
                        <CommandGroup heading="Commands">
                            <CommandItem value="profile" label="Profile" on_select=on_select>"Profile"</CommandItem>
                            <CommandItem value="billing" label="Billing" on_select=on_select>"Billing"</CommandItem>
                            <CommandItem value="settings" label="Settings" on_select=on_select>"Settings"</CommandItem>
                        </CommandGroup>
                        <CommandSeparator always_render=true />
                        <CommandGroup>
                            <CommandItem value="ask-ai" force_match=true on_select=on_select>
                                {move || format!("Ask AI about \"{}\"", fuzzy_search.get())}
                            </CommandItem>
                        </CommandGroup>
                    </CommandList>
                </Command>
            </Section>

            <Section title="Looping" description="loop_around lets the arrows wrap from the last item to the first and back (off by default).">
                <Command class="max-w-md rounded-xl border shadow-md" loop_around=true>
                    <CommandInput placeholder="Search..." aria_label="Looping command" />
                    <CommandList label="Looping commands">
                        <CommandGroup>
                            <CommandItem value="one" on_select=on_select>"One"</CommandItem>
                            <CommandItem value="two" on_select=on_select>"Two"</CommandItem>
                            <CommandItem value="three" on_select=on_select>"Three"</CommandItem>
                        </CommandGroup>
                    </CommandList>
                </Command>
            </Section>

            <Section title="In a popover" description="A Command in a Popover is a combobox-like menu: pick a status. The popover moves focus to the search field when it opens.">
                <Row>
                    <span class="text-sm text-muted-foreground">"Status"</span>
                    <Popover open=status_open on_open_change=move |o: bool| status_open.set(o)>
                        <PopoverTrigger variant=ButtonVariant::Outline class="w-[160px] justify-start">
                            {move || status.get()}
                        </PopoverTrigger>
                        <PopoverContent class="p-0" width=200 label="Set status">
                            <Command>
                                <CommandInput placeholder="Change status..." aria_label="Status search" />
                                <CommandList label="Statuses">
                                    <CommandEmpty>"No results found."</CommandEmpty>
                                    <CommandGroup>
                                        {["Backlog", "Todo", "In progress", "Done", "Canceled"]
                                            .into_iter()
                                            .map(|s| {
                                                view! {
                                                    <CommandItem
                                                        value=s
                                                        on_select=move |_: String| {
                                                            status.set(s.to_string());
                                                            status_open.set(false);
                                                        }
                                                    >
                                                        {s}
                                                    </CommandItem>
                                                }
                                            })
                                            .collect_view()}
                                    </CommandGroup>
                                </CommandList>
                            </Command>
                        </PopoverContent>
                    </Popover>
                </Row>
            </Section>
            <CommandDialogCallbacks />
        </Page>
    }
}

/// `on_open_change` / `on_close` on a CommandDialog.
#[component]
fn CommandDialogCallbacks() -> impl IntoView {
    let open = RwSignal::new(false);
    let log = RwSignal::new(Vec::<String>::new());

    view! {
        <Section
            title="Command dialog callbacks"
            description="on_open_change and on_close on CommandDialog, so the app need not watch the open signal."
        >
            <Row>
                <Button
                    attr:id="cmd-events-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| open.set(true)
                >
                    "Open palette"
                </Button>
                <span id="cmd-events-readout" class="text-sm text-muted-foreground">
                    {move || log.get().join(", ")}
                </span>
            </Row>
            <CommandDialog
                open=open
                on_open_change=move |o: bool| log.update(|l| l.push(format!("change {o}")))
                on_close=move |_| log.update(|l| l.push("close".to_string()))
            >
                <Command>
                    <CommandInput placeholder="Type a command..." aria_label="Command" />
                    <CommandList label="Commands">
                        <CommandEmpty>"No results found."</CommandEmpty>
                        <CommandGroup>
                            <CommandItem value="one" on_select=move |_: String| open.set(false)>
                                "Run one"
                            </CommandItem>
                            <CommandItem value="two" on_select=move |_: String| open.set(false)>
                                "Run two"
                            </CommandItem>
                        </CommandGroup>
                    </CommandList>
                </Command>
            </CommandDialog>
        </Section>
    }
}
