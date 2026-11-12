use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, DialogHost, Input, Label, Sheet, SheetClose, SheetDescription,
    SheetFooter, SheetHeader, SheetTitle, Side,
};

const SIDES: [(Side, &str); 4] = [
    (Side::Top, "Top"),
    (Side::Right, "Right"),
    (Side::Bottom, "Bottom"),
    (Side::Left, "Left"),
];

/// One sheet with a button that opens it, for the sides section.
#[component]
fn SideExample(side: Side, name: &'static str) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <Button variant=ButtonVariant::Outline on_click=move |_| open.set(true)>
            {name}
        </Button>
        <Sheet open=open label=format!("{name} sheet") side=side>
            <SheetHeader>
                <SheetTitle>{format!("{name} sheet")}</SheetTitle>
                <SheetDescription>
                    "It slides in from the " {side.as_str()} " edge. Esc, the X, a click outside or Close dismisses it."
                </SheetDescription>
            </SheetHeader>
            <SheetFooter>
                <SheetClose variant=ButtonVariant::Outline>"Close"</SheetClose>
            </SheetFooter>
        </Sheet>
    }
}

#[component]
pub fn SheetsPage() -> impl IntoView {
    let profile_open = RwSignal::new(false);
    let name = RwSignal::new(String::from("Pedro Duarte"));
    let username = RwSignal::new(String::from("@peduarte"));
    let saved = RwSignal::new(String::from("not saved"));

    let scroll_open = RwSignal::new(false);

    let locked_open = RwSignal::new(false);
    let no_x_open = RwSignal::new(false);
    let no_overlay_open = RwSignal::new(false);
    let no_esc_open = RwSignal::new(false);
    let wide_open = RwSignal::new(false);

    view! {
        <Page
            title="Sheet"
            summary="A panel that slides in from an edge over the page. Like the dialog it is a native <dialog>, so the browser traps focus, handles Escape and makes the page inert; unlike it, it plays an exit animation before closing."
        >
            <Section title="Sides" description="Left, right (the default), top and bottom. The panel moves 2.5rem and fades, as shadcn's.">
                <Row>
                    {SIDES
                        .into_iter()
                        .map(|(side, name)| view! { <SideExample side=side name=name /> })
                        .collect_view()}
                </Row>
            </Section>

            <Section title="With a form" description="Header, a body and a footer; the footer sticks to the bottom and Save closes the sheet through its open signal.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| profile_open.set(true)>
                        "Edit profile"
                    </Button>
                    <span class="text-sm text-muted-foreground">"Profile: " {move || saved.get()}</span>
                </Row>
                <Sheet open=profile_open label="Edit profile">
                    <SheetHeader>
                        <SheetTitle>"Edit profile"</SheetTitle>
                        <SheetDescription>
                            "Make changes to your profile here. Click save when you're done."
                        </SheetDescription>
                    </SheetHeader>
                    <div class="grid flex-1 auto-rows-min gap-6 px-4">
                        <div class="grid gap-3">
                            <Label>"Name"</Label>
                            <Input value=name aria_label="Name" />
                        </div>
                        <div class="grid gap-3">
                            <Label>"Username"</Label>
                            <Input value=username aria_label="Username" />
                        </div>
                    </div>
                    <SheetFooter>
                        <Button on_click=move |_| {
                            saved.set(format!("{} ({})", name.get(), username.get()));
                            profile_open.set(false);
                        }>"Save changes"</Button>
                        <SheetClose variant=ButtonVariant::Outline>"Close"</SheetClose>
                    </SheetFooter>
                </Sheet>
            </Section>

            <Section title="Long content" description="The panel is full height; put the scrolling region between header and footer.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| scroll_open.set(true)>
                        "Open long sheet"
                    </Button>
                </Row>
                <Sheet open=scroll_open label="Release notes">
                    <SheetHeader>
                        <SheetTitle>"Release notes"</SheetTitle>
                        <SheetDescription>"Scroll the list; the header and footer stay put."</SheetDescription>
                    </SheetHeader>
                    <div class="min-h-0 flex-1 space-y-3 overflow-y-auto px-4 text-sm">
                        {(1..=40)
                            .map(|n| view! { <p>{format!("Change {n}: a line of the changelog that needs a little room.")}</p> })
                            .collect_view()}
                    </div>
                    <SheetFooter>
                        <SheetClose>"Done"</SheetClose>
                    </SheetFooter>
                </Sheet>
            </Section>

            <Section title="Dismissal options" description="Each of the corner button, Escape and a click on the backdrop can be turned off; a sheet that must be answered turns all three off.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| no_x_open.set(true)>
                        "No close button"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| no_overlay_open.set(true)>
                        "Backdrop click ignored"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| no_esc_open.set(true)>
                        "Escape ignored"
                    </Button>
                    <Button variant=ButtonVariant::Destructive on_click=move |_| locked_open.set(true)>
                        "Must be answered"
                    </Button>
                </Row>
                <Sheet open=no_x_open label="No close button" show_close_button=false>
                    <SheetHeader>
                        <SheetTitle>"No corner button"</SheetTitle>
                        <SheetDescription>"Esc and a click outside still close it."</SheetDescription>
                    </SheetHeader>
                </Sheet>
                <Sheet open=no_overlay_open label="Backdrop click ignored" overlay_closable=false>
                    <SheetHeader>
                        <SheetTitle>"Backdrop click ignored"</SheetTitle>
                        <SheetDescription>"Clicking outside does nothing; Esc and the X close it."</SheetDescription>
                    </SheetHeader>
                </Sheet>
                <Sheet open=no_esc_open label="Escape ignored" close_on_escape=false>
                    <SheetHeader>
                        <SheetTitle>"Escape ignored"</SheetTitle>
                        <SheetDescription>"Esc does nothing; the X and a click outside close it."</SheetDescription>
                    </SheetHeader>
                </Sheet>
                <Sheet
                    open=locked_open
                    label="Must be answered"
                    show_close_button=false
                    overlay_closable=false
                    close_on_escape=false
                    side=Side::Bottom
                >
                    <SheetHeader>
                        <SheetTitle>"Discard your changes?"</SheetTitle>
                        <SheetDescription>"Only the buttons close this one."</SheetDescription>
                    </SheetHeader>
                    <SheetFooter>
                        <Button variant=ButtonVariant::Destructive on_click=move |_| locked_open.set(false)>
                            "Discard"
                        </Button>
                        <SheetClose variant=ButtonVariant::Outline>"Keep editing"</SheetClose>
                    </SheetFooter>
                </Sheet>
            </Section>

            <Section title="Sizing" description="The width is 75% up to 24rem; pass a class to change it (here Tailwind's max-w-xl).">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| wide_open.set(true)>
                        "Wide sheet"
                    </Button>
                </Row>
                <Sheet open=wide_open label="Wide sheet" class="sm:max-w-xl!">
                    <SheetHeader>
                        <SheetTitle>"Wide sheet"</SheetTitle>
                        <SheetDescription>"Up to 36rem wide."</SheetDescription>
                    </SheetHeader>
                </Sheet>
            </Section>
            <SheetLifecycleExamples />
        </Page>
    }
}

/// `on_open_change` / `on_close`, and a sheet mounted only while needed (`DialogHost`).
#[component]
fn SheetLifecycleExamples() -> impl IntoView {
    let open = RwSignal::new(false);
    let log = RwSignal::new(Vec::<String>::new());
    let wanted = RwSignal::new(false);

    view! {
        <Section
            title="Callbacks and DialogHost"
            description="on_open_change and on_close replace an Effect on the open signal. The second sheet is built only while `wanted` is true and still slides out, because DialogHost removes it after its exit animation."
        >
            <Row>
                <Button
                    attr:id="sheet-events-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| open.set(true)
                >
                    "With callbacks"
                </Button>
                <Button
                    attr:id="sheet-host-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| wanted.set(true)
                >
                    "Hosted sheet"
                </Button>
                <span id="sheet-events-readout" class="text-sm text-muted-foreground">
                    {move || log.get().join(", ")}
                </span>
            </Row>
            <Sheet
                open=open
                label="Callbacks"
                on_open_change=move |o: bool| log.update(|l| l.push(format!("change {o}")))
                on_close=move |_| log.update(|l| l.push("close".to_string()))
            >
                <SheetHeader>
                    <SheetTitle>"Callbacks"</SheetTitle>
                    <SheetDescription>"Closing it logs a change and a close."</SheetDescription>
                </SheetHeader>
            </Sheet>
            <DialogHost
                when=Signal::derive(move || wanted.get())
                on_close=move |_| wanted.set(false)
                let:open
            >
                <Sheet open=open label="Hosted">
                    <SheetHeader>
                        <SheetTitle>"Hosted sheet"</SheetTitle>
                        <SheetDescription>"Built only while wanted; slides out on close."</SheetDescription>
                    </SheetHeader>
                </Sheet>
            </DialogHost>
        </Section>
    }
}
