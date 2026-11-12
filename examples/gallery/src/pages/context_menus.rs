use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Align, ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuGroup,
    ContextMenuItem, ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem,
    ContextMenuSeparator, ContextMenuShortcut, ContextMenuSub, ContextMenuSubContent,
    ContextMenuSubTrigger, ContextMenuTrigger, MenuItemVariant, Side,
};

const AREA: &str =
    "flex h-36 w-72 items-center justify-center rounded-md border border-dashed text-sm";

#[component]
pub fn ContextMenusPage() -> impl IntoView {
    view! {
        <Page
            title="Context menus"
            summary="A menu that opens at the pointer on right-click, or when a touch is held for 700ms. It behaves like the dropdown menu: arrow keys, Home, End and typeahead move through the items; Enter or Space chooses; Escape closes."
        >
            <FullMenu />
            <CheckboxAndRadio />
            <ItemOptions />
            <StateSection />
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
            description="shadcn's demo. Right-click the box (or press and hold on a touch screen). \"More Tools\" opens a sub menu by hover, click or the right arrow; the left arrow closes it. \"Forward\" is disabled and is skipped by the keyboard."
        >
            <Row>
                <ContextMenu>
                    <ContextMenuTrigger class=AREA>"Right click here"</ContextMenuTrigger>
                    <ContextMenuContent width=256u32>
                        <ContextMenuItem inset=true on_select=pick("Back")>
                            "Back"
                            <ContextMenuShortcut>"⌘["</ContextMenuShortcut>
                        </ContextMenuItem>
                        <ContextMenuItem inset=true disabled=true on_select=pick("Forward")>
                            "Forward"
                            <ContextMenuShortcut>"⌘]"</ContextMenuShortcut>
                        </ContextMenuItem>
                        <ContextMenuItem inset=true on_select=pick("Reload")>
                            "Reload"
                            <ContextMenuShortcut>"⌘R"</ContextMenuShortcut>
                        </ContextMenuItem>
                        <ContextMenuSub>
                            <ContextMenuSubTrigger inset=true>"More Tools"</ContextMenuSubTrigger>
                            <ContextMenuSubContent>
                                <ContextMenuItem on_select=pick("Save Page...")>
                                    "Save Page..."
                                    <ContextMenuShortcut>"⇧⌘S"</ContextMenuShortcut>
                                </ContextMenuItem>
                                <ContextMenuItem on_select=pick("Create Shortcut...")>"Create Shortcut..."</ContextMenuItem>
                                <ContextMenuItem on_select=pick("Name Window...")>"Name Window..."</ContextMenuItem>
                                <ContextMenuSeparator />
                                <ContextMenuItem on_select=pick("Developer Tools")>"Developer Tools"</ContextMenuItem>
                                <ContextMenuSeparator />
                                <ContextMenuItem variant=MenuItemVariant::Destructive on_select=pick("Delete")>
                                    "Delete"
                                </ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                        <ContextMenuSeparator />
                        <ContextMenuGroup>
                            <ContextMenuLabel inset=true>"People"</ContextMenuLabel>
                            <ContextMenuItem inset=true on_select=pick("Pedro Duarte")>"Pedro Duarte"</ContextMenuItem>
                            <ContextMenuItem inset=true on_select=pick("Colm Tuite")>"Colm Tuite"</ContextMenuItem>
                        </ContextMenuGroup>
                    </ContextMenuContent>
                </ContextMenu>
                <span class="text-sm text-muted-foreground">
                    "Last chosen: " <span id="ctx-last">{move || last.get()}</span>
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn CheckboxAndRadio() -> impl IntoView {
    let bookmarks = RwSignal::new(true);
    let urls = RwSignal::new(false);
    let person = RwSignal::new(String::from("pedro"));

    view! {
        <Section
            title="Checkbox and radio items"
            description="Controlled by signals; the readout shows the state. Toggling does not close the menu."
        >
            <Row>
                <ContextMenu>
                    <ContextMenuTrigger class=AREA>"Right click for options"</ContextMenuTrigger>
                    <ContextMenuContent width=256u32>
                        <ContextMenuCheckboxItem
                            checked=bookmarks
                            on_checked_change=move |v| bookmarks.set(v)
                        >
                            "Show Bookmarks"
                            <ContextMenuShortcut>"⌘⇧B"</ContextMenuShortcut>
                        </ContextMenuCheckboxItem>
                        <ContextMenuCheckboxItem checked=urls on_checked_change=move |v| urls.set(v)>
                            "Show Full URLs"
                        </ContextMenuCheckboxItem>
                        <ContextMenuCheckboxItem disabled=true checked=true>"Disabled (checked)"</ContextMenuCheckboxItem>
                        <ContextMenuSeparator />
                        <ContextMenuLabel inset=true>"People"</ContextMenuLabel>
                        <ContextMenuRadioGroup value=person on_value_change=move |v| person.set(v)>
                            <ContextMenuRadioItem value="pedro">"Pedro Duarte"</ContextMenuRadioItem>
                            <ContextMenuRadioItem value="colm">"Colm Tuite"</ContextMenuRadioItem>
                            <ContextMenuRadioItem value="ada" disabled=true>"Ada (disabled)"</ContextMenuRadioItem>
                        </ContextMenuRadioGroup>
                        <ContextMenuSeparator />
                        <ContextMenuCheckboxItem default_checked=true close_on_select=true>
                            "Closes when toggled"
                        </ContextMenuCheckboxItem>
                    </ContextMenuContent>
                </ContextMenu>
                <span id="ctx-checks" class="text-sm text-muted-foreground">
                    "bookmarks " {move || bookmarks.get().to_string()} ", urls " {move || urls.get().to_string()}
                    ", person " {move || person.get()}
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn ItemOptions() -> impl IntoView {
    view! {
        <Section
            title="Placement, inset and disabled sub menus"
            description="side and align place the menu relative to the pointer (default: right, start). The second box opens to the left, aligned to the end. It also has a disabled sub menu and a sub menu that is open to the keyboard only (focus \"Sub\" and press Right)."
        >
            <Row>
                <ContextMenu>
                    <ContextMenuTrigger class=AREA>"side=left, align=end"</ContextMenuTrigger>
                    <ContextMenuContent side=Side::Left align=Align::End>
                        <ContextMenuItem>"One"</ContextMenuItem>
                        <ContextMenuItem>"Two"</ContextMenuItem>
                        <ContextMenuSub>
                            <ContextMenuSubTrigger disabled=true>"Disabled sub menu"</ContextMenuSubTrigger>
                            <ContextMenuSubContent>
                                <ContextMenuItem>"Never shown"</ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                        <ContextMenuSub>
                            <ContextMenuSubTrigger>"Sub"</ContextMenuSubTrigger>
                            <ContextMenuSubContent>
                                <ContextMenuItem>"Sub one"</ContextMenuItem>
                                <ContextMenuItem>"Sub two"</ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                    </ContextMenuContent>
                </ContextMenu>
            </Row>
        </Section>
    }
}

#[component]
fn StateSection() -> impl IntoView {
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
            title="Callbacks and a disabled trigger"
            description="on_open_change reports each change. The disabled area leaves the browser's own context menu alone."
        >
            <Row>
                <ContextMenu on_open_change=move |o| {
                    open.set(o);
                    push(format!("on_open_change({o})"));
                }>
                    <ContextMenuTrigger class=AREA>"Right click (reports state)"</ContextMenuTrigger>
                    <ContextMenuContent on_escape_key_down=move |_| push("escape pressed".into())>
                        <ContextMenuItem on_select=move |()| push("on_select: One".into())>"One"</ContextMenuItem>
                        <ContextMenuItem on_select=move |()| push("on_select: Two".into())>"Two"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
                <ContextMenu>
                    <ContextMenuTrigger class=AREA disabled=true>
                        "Disabled: native menu"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Never shown"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
                <span class="text-sm text-muted-foreground">"open = " <span id="ctx-open">{move || open.get().to_string()}</span></span>
            </Row>
            <ul id="ctx-log" class="text-sm text-muted-foreground">
                {move || log.get().into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}
            </ul>
        </Section>
    }
}

#[component]
fn CollisionSection() -> impl IntoView {
    view! {
        <Section
            title="Collisions"
            description="Right-click near the right or bottom edge of the window: the menu (and its sub menu) flips and slides to stay inside it. The box covers a tall area; a short window shows the long menu scrolling."
        >
            <ContextMenu>
                <ContextMenuTrigger class="flex h-96 w-full items-center justify-center rounded-md border border-dashed text-sm">
                    "Right click anywhere in this box, near the edges of the window"
                </ContextMenuTrigger>
                <ContextMenuContent collision_padding=8.0>
                    <ContextMenuItem>"Flips to the left"</ContextMenuItem>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Sub menu"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Opens left when"</ContextMenuItem>
                            <ContextMenuItem>"there is no room"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                    <ContextMenuSeparator />
                    {(1..=30)
                        .map(|n| view! { <ContextMenuItem>{format!("Item {n}")}</ContextMenuItem> })
                        .collect_view()}
                </ContextMenuContent>
            </ContextMenu>
        </Section>
    }
}
