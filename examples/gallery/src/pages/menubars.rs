use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, MenuItemVariant, Menubar, MenubarCheckboxItem, MenubarContent,
    MenubarGroup, MenubarItem, MenubarLabel, MenubarMenu, MenubarRadioGroup, MenubarRadioItem,
    MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent, MenubarSubTrigger,
    MenubarTrigger,
};

#[component]
pub fn MenubarsPage() -> impl IntoView {
    view! {
        <Page
            title="Menubar"
            summary="A horizontal bar of menus. Tab reaches the bar once; Left and Right move between the triggers, and between the open menus while one is open; Down, Up, Enter or Space opens a menu; hovering another trigger switches to its menu once one is open; Escape closes the menu and returns focus to its trigger."
        >
            <FullMenubar />
            <ControlledSection />
            <OptionsSection />
        </Page>
    }
}

#[component]
fn FullMenubar() -> impl IntoView {
    let last = RwSignal::new(String::from("nothing yet"));
    let pick = move |name: &'static str| Callback::new(move |()| last.set(name.to_string()));
    let bookmarks = RwSignal::new(true);
    let full_urls = RwSignal::new(false);
    let profile = RwSignal::new(String::from("benoit"));

    view! {
        <Section
            title="Application menubar"
            description="shadcn's demo: File, Edit, View and Profiles, with sub menus (Share, Find), shortcuts, check and radio items and disabled items. \"Undo\" is disabled and is skipped by the keyboard."
        >
            <Menubar>
                <MenubarMenu value="file">
                    <MenubarTrigger>"File"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem on_select=pick("New Tab")>"New Tab" <MenubarShortcut>"⌘T"</MenubarShortcut></MenubarItem>
                        <MenubarItem on_select=pick("New Window")>"New Window" <MenubarShortcut>"⌘N"</MenubarShortcut></MenubarItem>
                        <MenubarItem disabled=true>"New Incognito Window"</MenubarItem>
                        <MenubarSeparator />
                        <MenubarSub>
                            <MenubarSubTrigger>"Share"</MenubarSubTrigger>
                            <MenubarSubContent>
                                <MenubarItem on_select=pick("Email link")>"Email link"</MenubarItem>
                                <MenubarItem on_select=pick("Messages")>"Messages"</MenubarItem>
                                <MenubarItem on_select=pick("Notes")>"Notes"</MenubarItem>
                            </MenubarSubContent>
                        </MenubarSub>
                        <MenubarSeparator />
                        <MenubarItem on_select=pick("Print")>"Print…" <MenubarShortcut>"⌘P"</MenubarShortcut></MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="edit">
                    <MenubarTrigger>"Edit"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem disabled=true>"Undo" <MenubarShortcut>"⌘Z"</MenubarShortcut></MenubarItem>
                        <MenubarItem on_select=pick("Redo")>"Redo" <MenubarShortcut>"⇧⌘Z"</MenubarShortcut></MenubarItem>
                        <MenubarSeparator />
                        <MenubarSub>
                            <MenubarSubTrigger>"Find"</MenubarSubTrigger>
                            <MenubarSubContent>
                                <MenubarItem on_select=pick("Search the web")>"Search the web"</MenubarItem>
                                <MenubarSeparator />
                                <MenubarItem on_select=pick("Find…")>"Find…"</MenubarItem>
                                <MenubarItem on_select=pick("Find Next")>"Find Next"</MenubarItem>
                                <MenubarItem on_select=pick("Find Previous")>"Find Previous"</MenubarItem>
                            </MenubarSubContent>
                        </MenubarSub>
                        <MenubarSeparator />
                        <MenubarItem on_select=pick("Cut")>"Cut"</MenubarItem>
                        <MenubarItem on_select=pick("Copy")>"Copy"</MenubarItem>
                        <MenubarItem on_select=pick("Paste")>"Paste"</MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem variant=MenuItemVariant::Destructive on_select=pick("Clear history")>
                            "Clear history"
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="view">
                    <MenubarTrigger>"View"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarCheckboxItem checked=bookmarks on_checked_change=move |v| bookmarks.set(v)>
                            "Always Show Bookmarks Bar"
                        </MenubarCheckboxItem>
                        <MenubarCheckboxItem checked=full_urls on_checked_change=move |v| full_urls.set(v)>
                            "Always Show Full URLs"
                        </MenubarCheckboxItem>
                        <MenubarSeparator />
                        <MenubarItem inset=true on_select=pick("Reload")>"Reload" <MenubarShortcut>"⌘R"</MenubarShortcut></MenubarItem>
                        <MenubarItem inset=true disabled=true>"Force Reload" <MenubarShortcut>"⇧⌘R"</MenubarShortcut></MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem inset=true on_select=pick("Toggle Fullscreen")>"Toggle Fullscreen"</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="profiles">
                    <MenubarTrigger>"Profiles"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarRadioGroup value=profile on_value_change=move |v| profile.set(v)>
                            <MenubarRadioItem value="andy">"Andy"</MenubarRadioItem>
                            <MenubarRadioItem value="benoit">"Benoit"</MenubarRadioItem>
                            <MenubarRadioItem value="luis">"Luis"</MenubarRadioItem>
                        </MenubarRadioGroup>
                        <MenubarSeparator />
                        <MenubarGroup>
                            <MenubarLabel inset=true>"Edit…"</MenubarLabel>
                            <MenubarItem inset=true on_select=pick("Add Profile…")>"Add Profile…"</MenubarItem>
                        </MenubarGroup>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
            <p class="text-sm text-muted-foreground">
                "Last chosen: " <span id="mb-last">{move || last.get()}</span> " · bookmarks "
                <span id="mb-checks">
                    {move || bookmarks.get().to_string()} ", urls " {move || full_urls.get().to_string()}
                </span>
                " · profile " <span id="mb-radio">{move || profile.get()}</span>
            </p>
        </Section>
    }
}

#[component]
fn ControlledSection() -> impl IntoView {
    let value = RwSignal::new(None::<String>);

    view! {
        <Section
            title="Controlled state"
            description="The bar's value is the open menu's value, or none. The buttons open and close menus through the same signal."
        >
            <Menubar
                value=Signal::derive(move || value.get())
                on_value_change=move |v| value.set(v)
            >
                <MenubarMenu value="one">
                    <MenubarTrigger>"One"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"Item 1.1"</MenubarItem>
                        <MenubarItem>"Item 1.2"</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="two">
                    <MenubarTrigger>"Two"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"Item 2.1"</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
            <Row>
                <Button variant=ButtonVariant::Outline on_click=move |_| value.set(Some("one".into()))>"Open One"</Button>
                <Button variant=ButtonVariant::Outline on_click=move |_| value.set(Some("two".into()))>"Open Two"</Button>
                <Button variant=ButtonVariant::Ghost on_click=move |_| value.set(None)>"Close"</Button>
                <span class="text-sm text-muted-foreground">
                    "value = " <span id="mb-value">{move || format!("{:?}", value.get())}</span>
                </span>
            </Row>
        </Section>
    }
}

#[component]
fn OptionsSection() -> impl IntoView {
    view! {
        <Section
            title="No looping and a disabled trigger"
            description="With loop off, Left on the first menu and Right on the last stay put. The disabled trigger is skipped by the arrow keys."
        >
            <Menubar r#loop=false>
                <MenubarMenu value="a">
                    <MenubarTrigger>"First"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"A"</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="b">
                    <MenubarTrigger disabled=true>"Disabled"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"B"</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="c">
                    <MenubarTrigger>"Last"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"C"</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </Section>
    }
}
