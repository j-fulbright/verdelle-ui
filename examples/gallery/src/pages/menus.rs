use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Align, Button, ButtonVariant, DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuRadioGroup,
    DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSub,
    DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger, MenuItemVariant, Side,
    Size,
};

/// A 16px stroke icon from a single path, for the menu examples.
#[component]
fn Icon(d: &'static str) -> impl IntoView {
    view! {
        <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <path d=d></path>
        </svg>
    }
}

#[component]
pub fn MenusPage() -> impl IntoView {
    view! {
        <Page
            title="Dropdown menus"
            summary="A list of actions that opens from a button. Arrow keys, Home, End and typeahead move through the items; Enter or Space chooses; Escape closes and returns focus to the trigger."
        >
            <FullMenu />
            <CheckboxAndRadio />
            <ItemOptions />
            <PlacementSection />
            <OpenStateSection />
            <CollisionSection />
        </Page>
    }
}

#[component]
fn FullMenu() -> impl IntoView {
    let last = RwSignal::new(String::from("nothing yet"));
    let pick = move |name: &'static str| Callback::new(move |()| last.set(name.to_string()));

    view! {
        <Section
            title="Menu with groups, shortcuts and sub menus"
            description="shadcn's account menu. Hover \"Invite users\" or press the right arrow on it to open the sub menu (which has its own sub menu); the left arrow closes it. \"API\" is disabled and is skipped by the keyboard. Try typing \"b\" for Billing."
        >
            <Row>
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Outline>"Open menu"</DropdownMenuTrigger>
                    <DropdownMenuContent width=224u32>
                        <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                        <DropdownMenuGroup>
                            <DropdownMenuItem on_select=pick("Profile")>
                                <Icon d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8z" />
                                "Profile"
                                <DropdownMenuShortcut>"⇧⌘P"</DropdownMenuShortcut>
                            </DropdownMenuItem>
                            <DropdownMenuItem on_select=pick("Billing")>
                                <Icon d="M2 5h20v14H2zM2 10h20" />
                                "Billing"
                                <DropdownMenuShortcut>"⌘B"</DropdownMenuShortcut>
                            </DropdownMenuItem>
                            <DropdownMenuItem on_select=pick("Settings")>
                                <Icon d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
                                "Settings"
                                <DropdownMenuShortcut>"⌘S"</DropdownMenuShortcut>
                            </DropdownMenuItem>
                            <DropdownMenuItem on_select=pick("Keyboard shortcuts")>
                                "Keyboard shortcuts"
                                <DropdownMenuShortcut>"⌘K"</DropdownMenuShortcut>
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuGroup>
                            <DropdownMenuItem on_select=pick("Team")>"Team"</DropdownMenuItem>
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger>"Invite users"</DropdownMenuSubTrigger>
                                <DropdownMenuSubContent>
                                    <DropdownMenuItem on_select=pick("Invite: Email")>"Email"</DropdownMenuItem>
                                    <DropdownMenuItem on_select=pick("Invite: Message")>"Message"</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuSub>
                                        <DropdownMenuSubTrigger>"More…"</DropdownMenuSubTrigger>
                                        <DropdownMenuSubContent>
                                            <DropdownMenuItem on_select=pick("Invite: Slack")>"Slack"</DropdownMenuItem>
                                            <DropdownMenuItem on_select=pick("Invite: Link")>"Copy link"</DropdownMenuItem>
                                        </DropdownMenuSubContent>
                                    </DropdownMenuSub>
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
                            <DropdownMenuItem on_select=pick("New Team")>
                                "New Team"
                                <DropdownMenuShortcut>"⌘+T"</DropdownMenuShortcut>
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem on_select=pick("GitHub")>"GitHub"</DropdownMenuItem>
                        <DropdownMenuItem on_select=pick("Support")>"Support"</DropdownMenuItem>
                        <DropdownMenuItem disabled=true on_select=pick("API")>"API"</DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem variant=MenuItemVariant::Destructive on_select=pick("Log out")>
                            <Icon d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4M16 17l5-5-5-5M21 12H9" />
                            "Log out"
                            <DropdownMenuShortcut>"⇧⌘Q"</DropdownMenuShortcut>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
                <span class="text-sm text-muted-foreground">
                    "Last chosen: " <span id="menu-last">{move || last.get()}</span>
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn CheckboxAndRadio() -> impl IntoView {
    let status_bar = RwSignal::new(true);
    let activity_bar = RwSignal::new(false);
    let panel = RwSignal::new(false);
    let position = RwSignal::new(String::from("bottom"));

    view! {
        <Section
            title="Checkbox and radio items"
            description="Controlled by signals; the readouts below show the state. They stay open when toggled, so several can be changed in one visit (close_on_select closes after the choice). The \"Remember\" item keeps its own state (default_checked)."
        >
            <Row>
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Outline>"Checkboxes"</DropdownMenuTrigger>
                    <DropdownMenuContent width=224u32>
                        <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuCheckboxItem
                            checked=status_bar
                            on_checked_change=move |v| status_bar.set(v)
                        >
                            "Status Bar"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem
                            checked=activity_bar
                            disabled=true
                            on_checked_change=move |v| activity_bar.set(v)
                        >
                            "Activity Bar"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem checked=panel on_checked_change=move |v| panel.set(v)>
                            "Panel"
                        </DropdownMenuCheckboxItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuCheckboxItem default_checked=true>"Remember (uncontrolled)"</DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem close_on_select=true>"Closes when toggled"</DropdownMenuCheckboxItem>
                    </DropdownMenuContent>
                </DropdownMenu>
                <span id="menu-checks" class="text-sm text-muted-foreground">
                    "status " {move || status_bar.get().to_string()} ", activity " {move || activity_bar.get().to_string()}
                    ", panel " {move || panel.get().to_string()}
                </span>
            </Row>
            <Row>
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Outline>"Radio group"</DropdownMenuTrigger>
                    <DropdownMenuContent width=224u32>
                        <DropdownMenuLabel>"Panel Position"</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuRadioGroup
                            value=position
                            on_value_change=move |v| position.set(v)
                        >
                            <DropdownMenuRadioItem value="top">"Top"</DropdownMenuRadioItem>
                            <DropdownMenuRadioItem value="bottom">"Bottom"</DropdownMenuRadioItem>
                            <DropdownMenuRadioItem value="right">"Right"</DropdownMenuRadioItem>
                            <DropdownMenuRadioItem value="left" disabled=true>"Left (disabled)"</DropdownMenuRadioItem>
                        </DropdownMenuRadioGroup>
                    </DropdownMenuContent>
                </DropdownMenu>
                <span id="menu-radio" class="text-sm text-muted-foreground">"position: " {move || position.get()}</span>
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Outline>"Uncontrolled radio"</DropdownMenuTrigger>
                    <DropdownMenuContent width=224u32>
                        <DropdownMenuRadioGroup default_value="medium".to_string()>
                            <DropdownMenuRadioItem value="small" close_on_select=true>"Small (closes)"</DropdownMenuRadioItem>
                            <DropdownMenuRadioItem value="medium">"Medium"</DropdownMenuRadioItem>
                            <DropdownMenuRadioItem value="large">"Large"</DropdownMenuRadioItem>
                        </DropdownMenuRadioGroup>
                    </DropdownMenuContent>
                </DropdownMenu>
            </Row>
        </Section>
    }
}

#[component]
fn ItemOptions() -> impl IntoView {
    let last = RwSignal::new(String::from("nothing yet"));

    view! {
        <Section
            title="Item variants and inset"
            description="A destructive item, inset labels and items that line up with check items, a disabled sub menu, and a menu with a fixed width."
        >
            <Row>
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Secondary>"Item options"</DropdownMenuTrigger>
                    <DropdownMenuContent width=240u32>
                        <DropdownMenuLabel inset=true>"Inset label"</DropdownMenuLabel>
                        <DropdownMenuItem inset=true on_select=move |()| last.set("Inset item".into())>"Inset item"</DropdownMenuItem>
                        <DropdownMenuItem on_select=move |()| last.set("Normal item".into())>"Normal item"</DropdownMenuItem>
                        <DropdownMenuItem disabled=true>"Disabled item"</DropdownMenuItem>
                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger disabled=true>"Disabled sub menu"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuItem>"Never shown"</DropdownMenuItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger inset=true>"Inset sub trigger"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuItem on_select=move |()| last.set("Sub item".into())>"Sub item"</DropdownMenuItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem variant=MenuItemVariant::Destructive on_select=move |()| last.set("Delete".into())>
                            "Delete"
                            <DropdownMenuShortcut>"⌫"</DropdownMenuShortcut>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Ghost icon_only=true size=Size::Small>
                        <Icon d="M12 12h.01M19 12h.01M5 12h.01" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align=Align::End>
                        <DropdownMenuItem on_select=move |()| last.set("Edit".into())>"Edit"</DropdownMenuItem>
                        <DropdownMenuItem on_select=move |()| last.set("Duplicate".into())>"Duplicate"</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
                <span class="text-sm text-muted-foreground">"Last chosen: " <span id="menu-last2">{move || last.get()}</span></span>
            </Row>
        </Section>
    }
}

#[component]
fn PlacementSection() -> impl IntoView {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    const ALIGNS: [Align; 3] = [Align::Start, Align::Center, Align::End];

    view! {
        <Section
            title="Side and align"
            description="The menu opens on the chosen side with the chosen alignment; the default is bottom, start."
        >
            <div class="space-y-3">
                {SIDES
                    .into_iter()
                    .map(|side| {
                        view! {
                            <Row label=side.as_str()>
                                {ALIGNS
                                    .into_iter()
                                    .map(|align| {
                                        view! {
                                            <DropdownMenu>
                                                <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>
                                                    {align.as_str()}
                                                </DropdownMenuTrigger>
                                                <DropdownMenuContent side=side align=align>
                                                    <DropdownMenuItem>"One"</DropdownMenuItem>
                                                    <DropdownMenuItem>"Two"</DropdownMenuItem>
                                                </DropdownMenuContent>
                                            </DropdownMenu>
                                        }
                                    })
                                    .collect_view()}
                            </Row>
                        }
                    })
                    .collect_view()}
                <Row label="offsets">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>"side_offset 20"</DropdownMenuTrigger>
                        <DropdownMenuContent side_offset=20.0>
                            <DropdownMenuItem>"One"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>"align_offset 24"</DropdownMenuTrigger>
                        <DropdownMenuContent align_offset=24.0>
                            <DropdownMenuItem>"One"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </Row>
            </div>
        </Section>
    }
}

#[component]
fn OpenStateSection() -> impl IntoView {
    let open = RwSignal::new(false);
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |line: String| {
        log.update(|l| {
            l.insert(0, line);
            l.truncate(5);
        })
    };

    view! {
        <Section
            title="Open state and callbacks"
            description="on_open_change reports each change; the buttons drive the same menu through the controlled open signal. Escape and outside presses can be intercepted."
        >
            <Row>
                <DropdownMenu
                    open=open
                    on_open_change=move |o| {
                        open.set(o);
                        push(format!("on_open_change({o})"));
                    }
                >
                    <DropdownMenuTrigger variant=ButtonVariant::Outline>"Controlled menu"</DropdownMenuTrigger>
                    <DropdownMenuContent on_escape_key_down=move |_| push("escape pressed".into())>
                        <DropdownMenuItem on_select=move |()| push("on_select: One".into())>"One"</DropdownMenuItem>
                        <DropdownMenuItem on_select=move |()| push("on_select: Two".into())>"Two"</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
                <Button variant=ButtonVariant::Ghost on_click=move |_| open.set(true)>"Open it from outside"</Button>
                <span class="text-sm text-muted-foreground">"open = " <span id="menu-open">{move || open.get().to_string()}</span></span>
            </Row>
            <ul id="menu-log" class="text-sm text-muted-foreground">
                {move || log.get().into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}
            </ul>
        </Section>
    }
}

#[component]
fn CollisionSection() -> impl IntoView {
    let show_fixed = RwSignal::new(false);

    view! {
        <Section
            title="Collisions"
            description="Near a viewport edge the menu flips and slides to stay inside the window, and a menu taller than the free space scrolls. The button pins triggers to the window corners."
        >
            <Button variant=ButtonVariant::Secondary selected=show_fixed on_click=move |_| show_fixed.update(|v| *v = !*v)>
                {move || if show_fixed.get() { "Hide the corner triggers" } else { "Show triggers pinned to the window corners" }}
            </Button>
            <Show when=move || show_fixed.get()>
                <div class="fixed bottom-4 left-[32rem] z-10">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Default>"Bottom, opens below"</DropdownMenuTrigger>
                        <DropdownMenuContent collision_padding=8.0>
                            <DropdownMenuItem>"Flips above"</DropdownMenuItem>
                            <DropdownMenuItem>"the trigger"</DropdownMenuItem>
                            <DropdownMenuItem>"when the window"</DropdownMenuItem>
                            <DropdownMenuItem>"bottom is close"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
                <div class="fixed right-4 bottom-4 z-10 flex gap-2">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Default>"Corner, opens right"</DropdownMenuTrigger>
                        <DropdownMenuContent side=Side::Right collision_padding=8.0 width=200u32>
                            <DropdownMenuItem>"Flips to the left"</DropdownMenuItem>
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger>"Sub menu"</DropdownMenuSubTrigger>
                                <DropdownMenuSubContent>
                                    <DropdownMenuItem>"Opens left, no room"</DropdownMenuItem>
                                    <DropdownMenuItem>"on the right"</DropdownMenuItem>
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
                <div class="fixed top-4 right-4 z-10">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Secondary>"Short window: long menu"</DropdownMenuTrigger>
                        <DropdownMenuContent align=Align::End collision_padding=8.0>
                            {(1..=40)
                                .map(|n| view! { <DropdownMenuItem>{format!("Item {n}")}</DropdownMenuItem> })
                                .collect_view()}
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            </Show>
        </Section>
    }
}
