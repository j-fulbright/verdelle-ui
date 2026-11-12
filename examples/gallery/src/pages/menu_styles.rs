use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem,
    ComboboxList, ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem,
    ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator,
    ContextMenuShortcut, ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger,
    ContextMenuTrigger, DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuRadioGroup,
    DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSub,
    DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger, Icon, IconName, Label,
    MenuItemVariant, Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu,
    MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent, MenubarSubTrigger,
    MenubarTrigger, NativeSelect, NativeSelectOption, Popover, PopoverContent, PopoverTrigger,
    Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger,
    SelectValue, Tooltip, TooltipContent, TooltipProvider, TooltipTrigger,
};

const COLORS: [&str; 4] = [
    "default",
    "inverted",
    "default-translucent",
    "inverted-translucent",
];
const ACCENTS: [&str; 2] = ["subtle", "bold"];

const COLOR_ATTR: &str = "data-vdl-menu-color";
const ACCENT_ATTR: &str = "data-vdl-menu-accent";

/// A busy backdrop, so translucent popups have something to blur.
const BACKDROP: &str = "rounded-lg border p-6 [background:repeating-linear-gradient(135deg,var(--chart-2)_0_16px,var(--chart-4)_16px_32px)]";

/// Sets the two menu attributes on `<html>` while the page is mounted and puts back what was
/// there on leave (the theme provider will own them in an app). Effects only run in the browser.
#[component]
fn MenuStyleControl(color: RwSignal<String>, accent: RwSignal<String>) -> impl IntoView {
    Effect::new(move |_| {
        let Some(root) = document().document_element() else {
            return;
        };
        let _ = root.set_attribute(COLOR_ATTR, &color.get());
        let _ = root.set_attribute(ACCENT_ATTR, &accent.get());
    });
    // Restore on leave; the cleanup lives in an effect so it does not run during SSR.
    Effect::new(move |_| {
        let Some(root) = document().document_element() else {
            return;
        };
        let before = (
            root.get_attribute(COLOR_ATTR),
            root.get_attribute(ACCENT_ATTR),
        );
        on_cleanup(move || {
            for (name, value) in [(COLOR_ATTR, &before.0), (ACCENT_ATTR, &before.1)] {
                match value {
                    Some(v) => {
                        let _ = root.set_attribute(name, v);
                    }
                    None => {
                        let _ = root.remove_attribute(name);
                    }
                }
            }
        });
    });

    view! {
        <div class="sticky top-0 z-10 flex flex-wrap items-center gap-4 rounded-lg border bg-background p-3">
            <div class="flex items-center gap-2">
                <Label html_for="menu-color">"Menu color"</Label>
                <NativeSelect id="menu-color" value=color aria_label="Menu color">
                    {COLORS.map(|c| view! { <NativeSelectOption value=c>{c}</NativeSelectOption> }).collect_view()}
                </NativeSelect>
            </div>
            <div class="flex items-center gap-2">
                <Label html_for="menu-accent">"Menu accent"</Label>
                <NativeSelect id="menu-accent" value=accent aria_label="Menu accent">
                    {ACCENTS.map(|a| view! { <NativeSelectOption value=a>{a}</NativeSelectOption> }).collect_view()}
                </NativeSelect>
            </div>
            <span class="text-xs text-muted-foreground">
                "<html " {COLOR_ATTR} "=\"" {move || color.get()} "\" " {ACCENT_ATTR} "=\"" {move || accent.get()} "\">"
            </span>
        </div>
    }
}

#[component]
pub fn MenuStylesPage() -> impl IntoView {
    let color = RwSignal::new(String::from("default"));
    let accent = RwSignal::new(String::from("subtle"));

    view! {
        <Page
            title="Menu styles"
            summary="shadcn's two menu options: the menu color (default, inverted, default-translucent, inverted-translucent) and the menu accent (subtle, bold). They are two attributes on <html>, data-vdl-menu-color and data-vdl-menu-accent, which the theme provider sets; with neither, menus look as they always did. The control below sets them for the page and puts them back when you leave. They apply to dropdown, context and menubar menus (and sub menus), the select popup and the combobox popup, as in shadcn; the tooltip and popover at the end are shown for comparison and do not change."
        >
            <MenuStyleControl color=color accent=accent />
            <DropdownSection />
            <ContextSection />
            <MenubarSection />
            <SelectSection />
            <ComboboxSection />
            <UnaffectedSection />
        </Page>
    }
}

#[component]
fn DropdownSection() -> impl IntoView {
    let status = RwSignal::new(true);
    let panel = RwSignal::new(false);
    let position = RwSignal::new(String::from("bottom"));

    view! {
        <Section
            title="Dropdown menu"
            description="Highlight an item with the keyboard or pointer. Items: icon and shortcut, check and radio items, a destructive item, a disabled item, a label, separators, and a sub menu (hover or press the right arrow on \"Invite users\")."
        >
            <div class=BACKDROP>
                <Row>
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline attr:data-testid="ms-dropdown">
                            "Open menu"
                        </DropdownMenuTrigger>
                        <DropdownMenuContent width=240u32>
                            <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                            <DropdownMenuGroup>
                                <DropdownMenuItem>
                                    <Icon name=IconName::User />
                                    "Profile"
                                    <DropdownMenuShortcut>"⇧⌘P"</DropdownMenuShortcut>
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    <Icon name=IconName::CreditCard />
                                    "Billing"
                                    <DropdownMenuShortcut>"⌘B"</DropdownMenuShortcut>
                                </DropdownMenuItem>
                                <DropdownMenuItem disabled=true>"API (disabled)"</DropdownMenuItem>
                            </DropdownMenuGroup>
                            <DropdownMenuSeparator />
                            <DropdownMenuSub>
                                <DropdownMenuSubTrigger>"Invite users"</DropdownMenuSubTrigger>
                                <DropdownMenuSubContent>
                                    <DropdownMenuItem>"Email"</DropdownMenuItem>
                                    <DropdownMenuItem>"Message"</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem variant=MenuItemVariant::Destructive>"Revoke all"</DropdownMenuItem>
                                </DropdownMenuSubContent>
                            </DropdownMenuSub>
                            <DropdownMenuSeparator />
                            <DropdownMenuCheckboxItem checked=status on_checked_change=move |v| status.set(v)>
                                "Status Bar"
                            </DropdownMenuCheckboxItem>
                            <DropdownMenuCheckboxItem checked=panel on_checked_change=move |v| panel.set(v)>
                                "Panel"
                            </DropdownMenuCheckboxItem>
                            <DropdownMenuSeparator />
                            <DropdownMenuRadioGroup value=position on_value_change=move |v| position.set(v)>
                                <DropdownMenuRadioItem value="top">"Top"</DropdownMenuRadioItem>
                                <DropdownMenuRadioItem value="bottom">"Bottom"</DropdownMenuRadioItem>
                            </DropdownMenuRadioGroup>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem variant=MenuItemVariant::Destructive>
                                <Icon name=IconName::LogOut />
                                "Log out"
                                <DropdownMenuShortcut>"⇧⌘Q"</DropdownMenuShortcut>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </Row>
            </div>
        </Section>
    }
}

#[component]
fn ContextSection() -> impl IntoView {
    let bookmarks = RwSignal::new(true);
    let person = RwSignal::new(String::from("pedro"));

    view! {
        <Section title="Context menu" description="Right-click the area.">
            <div class=BACKDROP>
                <ContextMenu>
                    <ContextMenuTrigger
                        class="flex h-28 items-center justify-center rounded-md border border-dashed bg-background text-sm"
                        attr:data-testid="ms-context"
                    >
                        "Right click here"
                    </ContextMenuTrigger>
                    <ContextMenuContent width=224u32>
                        <ContextMenuItem>"Back" <ContextMenuShortcut>"⌘["</ContextMenuShortcut></ContextMenuItem>
                        <ContextMenuItem disabled=true>"Forward"</ContextMenuItem>
                        <ContextMenuSub>
                            <ContextMenuSubTrigger>"More Tools"</ContextMenuSubTrigger>
                            <ContextMenuSubContent>
                                <ContextMenuItem>"Save Page..."</ContextMenuItem>
                                <ContextMenuItem variant=MenuItemVariant::Destructive>"Delete"</ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                        <ContextMenuSeparator />
                        <ContextMenuCheckboxItem checked=bookmarks on_checked_change=move |v| bookmarks.set(v)>
                            "Show Bookmarks"
                        </ContextMenuCheckboxItem>
                        <ContextMenuSeparator />
                        <ContextMenuLabel>"People"</ContextMenuLabel>
                        <ContextMenuRadioGroup value=person on_value_change=move |v| person.set(v)>
                            <ContextMenuRadioItem value="pedro">"Pedro Duarte"</ContextMenuRadioItem>
                            <ContextMenuRadioItem value="colm">"Colm Tuite"</ContextMenuRadioItem>
                        </ContextMenuRadioGroup>
                    </ContextMenuContent>
                </ContextMenu>
            </div>
        </Section>
    }
}

#[component]
fn MenubarSection() -> impl IntoView {
    let bookmarks = RwSignal::new(true);

    view! {
        <Section title="Menubar" description="Menus open under the bar; the open trigger and the items follow the same options.">
            <div class=BACKDROP>
                <Menubar>
                    <MenubarMenu value="file">
                        <MenubarTrigger attr:data-testid="ms-menubar">"File"</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>"New Tab" <MenubarShortcut>"⌘T"</MenubarShortcut></MenubarItem>
                            <MenubarItem disabled=true>"New Incognito Window"</MenubarItem>
                            <MenubarSeparator />
                            <MenubarSub>
                                <MenubarSubTrigger>"Share"</MenubarSubTrigger>
                                <MenubarSubContent>
                                    <MenubarItem>"Email link"</MenubarItem>
                                    <MenubarItem>"Messages"</MenubarItem>
                                </MenubarSubContent>
                            </MenubarSub>
                            <MenubarSeparator />
                            <MenubarCheckboxItem checked=bookmarks on_checked_change=move |v| bookmarks.set(v)>
                                "Always Show Bookmarks Bar"
                            </MenubarCheckboxItem>
                            <MenubarItem variant=MenuItemVariant::Destructive>"Quit"</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                    <MenubarMenu value="edit">
                        <MenubarTrigger>"Edit"</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>"Undo" <MenubarShortcut>"⌘Z"</MenubarShortcut></MenubarItem>
                            <MenubarItem>"Redo" <MenubarShortcut>"⇧⌘Z"</MenubarShortcut></MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                </Menubar>
            </div>
        </Section>
    }
}

#[component]
fn SelectSection() -> impl IntoView {
    view! {
        <Section title="Select" description="The popup, its group label, separator, items and the check on the selected one.">
            <div class=BACKDROP>
                <Select default_value="banana">
                    <SelectTrigger class="w-[200px] bg-background" attr:data-testid="ms-select">
                        <SelectValue placeholder="Select a fruit" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel>"Fruits"</SelectLabel>
                            <SelectItem value="apple">"Apple"</SelectItem>
                            <SelectItem value="banana">"Banana"</SelectItem>
                            <SelectItem value="grapes" disabled=true>"Grapes (disabled)"</SelectItem>
                        </SelectGroup>
                        <SelectSeparator />
                        <SelectGroup>
                            <SelectLabel>"Vegetables"</SelectLabel>
                            <SelectItem value="carrot">"Carrot"</SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            </div>
        </Section>
    }
}

#[component]
fn ComboboxSection() -> impl IntoView {
    view! {
        <Section title="Combobox" description="The popup with its search field, a highlighted item (arrow keys), the selected item's check and a disabled item.">
            <div class=BACKDROP>
                <Combobox default_value="Remix">
                    <ComboboxInput
                        placeholder="Select a framework"
                        aria_label="Framework"
                        class="w-[220px] bg-background"
                        attr:data-testid="ms-combobox"
                    />
                    <ComboboxContent>
                        <ComboboxEmpty>"No framework found."</ComboboxEmpty>
                        <ComboboxList>
                            <ComboboxItem value="Next.js">"Next.js"</ComboboxItem>
                            <ComboboxItem value="Remix">"Remix"</ComboboxItem>
                            <ComboboxItem value="Astro">"Astro"</ComboboxItem>
                            <ComboboxItem value="Nuxt.js" disabled=true>"Nuxt.js (disabled)"</ComboboxItem>
                        </ComboboxList>
                    </ComboboxContent>
                </Combobox>
            </div>
        </Section>
    }
}

#[component]
fn UnaffectedSection() -> impl IntoView {
    view! {
        <Section
            title="Not affected"
            description="The tooltip is already inverted in shadcn, and the popover is not a menu; neither reads the two attributes."
        >
            <TooltipProvider>
                <Row>
                    <Tooltip>
                        <TooltipTrigger variant=ButtonVariant::Outline>"Hover me"</TooltipTrigger>
                        <TooltipContent>"Add to library"</TooltipContent>
                    </Tooltip>
                    <Popover>
                        <PopoverTrigger variant=ButtonVariant::Outline>"Open popover"</PopoverTrigger>
                        <PopoverContent label="Example" width=240u32>
                            <p class="text-sm">"A popover is a panel, not a menu."</p>
                        </PopoverContent>
                    </Popover>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                </Row>
            </TooltipProvider>
        </Section>
    }
}
