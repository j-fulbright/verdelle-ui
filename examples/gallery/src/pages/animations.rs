use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, Button, ButtonVariant, Combobox, ComboboxContent,
    ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxList, Command, CommandDialog, CommandInput,
    CommandItem, CommandList, ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
    Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DropdownMenu,
    DropdownMenuContent, DropdownMenuItem, DropdownMenuSub, DropdownMenuSubContent,
    DropdownMenuSubTrigger, DropdownMenuTrigger, HoverCard, HoverCardContent, HoverCardTrigger,
    Label, Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger, Popover,
    PopoverContent, PopoverTrigger, Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
    Sheet, SheetClose, SheetDescription, SheetFooter, SheetHeader, SheetTitle, Switch, ToastData,
    Toaster, Tooltip, TooltipContent, TooltipProvider, TooltipTrigger, use_toaster,
};

/// Turns the kit's motion off for the whole document while the page is mounted
/// (`data-vdl-motion="off"` on `<html>`), so every overlay below can be compared with and
/// without its animations. Effects only run in the browser.
#[component]
fn MotionSwitch(off: RwSignal<bool>) -> impl IntoView {
    // The cleanup lives inside the effect: a cleanup registered in the component body would run
    // during SSR, where there is no document.
    Effect::new(move |_| {
        let Some(root) = document().document_element() else {
            return;
        };
        if off.get() {
            let _ = root.set_attribute("data-vdl-motion", "off");
        }
        on_cleanup(move || {
            let _ = root.remove_attribute("data-vdl-motion");
        });
    });

    view! {
        <div class="flex items-center gap-2">
            <Switch
                id="motion-off"
                checked=off
                on_checked_change=move |v| off.set(v)
                aria_label="Disable animation"
            />
            <Label html_for="motion-off">"Disable animation"</Label>
        </div>
    }
}

#[component]
fn ToastDemo() -> impl IntoView {
    let toaster = use_toaster();
    view! {
        <Button
            variant=ButtonVariant::Outline
            attr:data-testid="toast-show"
            on_click=move |_| toaster.show(ToastData::new("Saved").description("Closing plays an exit animation."))
        >
            "Show toast"
        </Button>
    }
}

#[component]
pub fn AnimationsPage() -> impl IntoView {
    let off = RwSignal::new(false);
    let dialog = RwSignal::new(false);
    let alert = RwSignal::new(false);
    let palette = RwSignal::new(false);
    let sheet = RwSignal::new(false);

    view! {
        <Page
            title="Animations"
            summary="Every overlay plays an exit as well as an entrance. Floating panels fade and zoom to 95% in 100ms and stay mounted until that has finished; dialogs, the command palette and the sheet are native <dialog>s that close after their exit. Turn animation off with the switch (data-vdl-motion=off on <html>, which the OS reduced-motion setting also does for the overlays) and the panels go away at once."
        >
            <Section title="Motion" description="Applies to the whole page while it is open.">
                <MotionSwitch off=off />
            </Section>

            <Section title="Floating panels" description="Open each one, then close it (click away, Escape, or pick an item) and watch it fade out instead of vanishing.">
                <TooltipProvider>
                    <Row label="Popover">
                        <Popover>
                            <PopoverTrigger variant=ButtonVariant::Outline attr:data-testid="popover-trigger">
                                "Popover"
                            </PopoverTrigger>
                            <PopoverContent width=220u32>
                                "A popover."
                            </PopoverContent>
                        </Popover>
                    </Row>
                    <Row label="Dropdown menu">
                        <DropdownMenu>
                            <DropdownMenuTrigger variant=ButtonVariant::Outline attr:data-testid="menu-trigger">
                                "Menu"
                            </DropdownMenuTrigger>
                            <DropdownMenuContent width=200u32>
                                <DropdownMenuItem>"Profile"</DropdownMenuItem>
                                <DropdownMenuItem>"Billing"</DropdownMenuItem>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger attr:data-testid="menu-sub-trigger">
                                        "Invite"
                                    </DropdownMenuSubTrigger>
                                    <DropdownMenuSubContent>
                                        <DropdownMenuItem>"Email"</DropdownMenuItem>
                                        <DropdownMenuItem>"Message"</DropdownMenuItem>
                                    </DropdownMenuSubContent>
                                </DropdownMenuSub>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </Row>
                    <Row label="Context menu">
                        <ContextMenu>
                            <ContextMenuTrigger class="flex h-20 w-56 items-center justify-center rounded-md border border-dashed border-border text-sm">
                                "Right click here"
                            </ContextMenuTrigger>
                            <ContextMenuContent width=200u32>
                                <ContextMenuItem>"Back"</ContextMenuItem>
                                <ContextMenuItem>"Reload"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenu>
                    </Row>
                    <Row label="Menubar">
                        <Menubar>
                            <MenubarMenu value="file">
                                <MenubarTrigger attr:data-testid="menubar-trigger">"File"</MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem>"New Tab"</MenubarItem>
                                    <MenubarItem>"Print…"</MenubarItem>
                                </MenubarContent>
                            </MenubarMenu>
                        </Menubar>
                    </Row>
                    <Row label="Select">
                        <Select>
                            <SelectTrigger class="w-[180px]" attr:data-testid="select-trigger">
                                <SelectValue placeholder="Pick a fruit" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="apple">"Apple"</SelectItem>
                                <SelectItem value="banana">"Banana"</SelectItem>
                                <SelectItem value="cherry">"Cherry"</SelectItem>
                            </SelectContent>
                        </Select>
                    </Row>
                    <Row label="Combobox">
                        <Combobox>
                            <ComboboxInput placeholder="Pick a fruit" aria_label="Fruit" class="w-[180px]" />
                            <ComboboxContent>
                                <ComboboxEmpty>"None found."</ComboboxEmpty>
                                <ComboboxList>
                                    <ComboboxItem value="apple" label="Apple">"Apple"</ComboboxItem>
                                    <ComboboxItem value="banana" label="Banana">"Banana"</ComboboxItem>
                                    <ComboboxItem value="cherry" label="Cherry">"Cherry"</ComboboxItem>
                                </ComboboxList>
                            </ComboboxContent>
                        </Combobox>
                    </Row>
                    <Row label="Tooltip">
                        <Tooltip>
                            <TooltipTrigger variant=ButtonVariant::Outline attr:data-testid="tooltip-trigger">
                                "Hover me"
                            </TooltipTrigger>
                            <TooltipContent>"A tooltip"</TooltipContent>
                        </Tooltip>
                    </Row>
                    <Row label="Hover card">
                        <HoverCard>
                            <HoverCardTrigger href="#anim" class="text-sm underline underline-offset-4" attr:data-testid="hover-trigger">
                                "Hover card"
                            </HoverCardTrigger>
                            <HoverCardContent>
                                "Details that appear on hover."
                            </HoverCardContent>
                        </HoverCard>
                    </Row>
                </TooltipProvider>
            </Section>

            <Section title="Dialogs" description="Native <dialog>s: they are marked closing, play the exit and the backdrop fade, then close. Focus goes back to the button when the dialog closes.">
                <Row>
                    <Button variant=ButtonVariant::Outline attr:data-testid="dialog-trigger" on_click=move |_| dialog.set(true)>
                        "Dialog"
                    </Button>
                    <Button variant=ButtonVariant::Outline attr:data-testid="alert-trigger" on_click=move |_| alert.set(true)>
                        "Alert dialog"
                    </Button>
                    <Button variant=ButtonVariant::Outline attr:data-testid="palette-trigger" on_click=move |_| palette.set(true)>
                        "Command dialog"
                    </Button>
                    <Button variant=ButtonVariant::Outline attr:data-testid="sheet-trigger" on_click=move |_| sheet.set(true)>
                        "Sheet"
                    </Button>
                </Row>
                <Dialog open=dialog label="Animated dialog">
                    <DialogHeader>
                        <DialogTitle>"Animated dialog"</DialogTitle>
                        <DialogDescription>"Esc, the X, the backdrop or Close plays the exit."</DialogDescription>
                    </DialogHeader>
                    <DialogFooter>
                        <Button variant=ButtonVariant::Outline on_click=move |_| dialog.set(false)>"Close"</Button>
                    </DialogFooter>
                </Dialog>
                <AlertDialog open=alert>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Delete it?"</AlertDialogTitle>
                        <AlertDialogDescription>"Cancel and Delete both play the exit."</AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction variant=ButtonVariant::Destructive>"Delete"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
                <CommandDialog open=palette>
                    <Command>
                        <CommandInput placeholder="Type a command..." aria_label="Palette search" />
                        <CommandList label="Commands">
                            <CommandItem value="calendar" label="Calendar">"Calendar"</CommandItem>
                            <CommandItem value="search" label="Search">"Search"</CommandItem>
                        </CommandList>
                    </Command>
                </CommandDialog>
                <Sheet open=sheet label="Animated sheet">
                    <SheetHeader>
                        <SheetTitle>"Animated sheet"</SheetTitle>
                        <SheetDescription>"Slides out as well as in."</SheetDescription>
                    </SheetHeader>
                    <SheetFooter>
                        <SheetClose variant=ButtonVariant::Outline>"Close"</SheetClose>
                    </SheetFooter>
                </Sheet>
            </Section>

            <Section title="Toast" description="A toast fades and shrinks out when it closes (the X, Escape, swipe or its timer).">
                <Toaster>
                    <ToastDemo />
                </Toaster>
            </Section>
        </Page>
    }
}
