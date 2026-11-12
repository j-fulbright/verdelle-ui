use crate::section::{Page, Row, Section};
use leptos::{ev, prelude::*};
use verdelle_ui::{
    Button, ButtonVariant, Dialog, DialogDescription, DialogFooter, DialogHeader, DialogHost,
    DialogTitle, Drawer, DrawerClose, DrawerDescription, DrawerDirection, DrawerFooter,
    DrawerHeader, DrawerTitle, DrawerTrigger, DrawerWrapper, Input, Label,
};

const DIRECTIONS: [(DrawerDirection, &str); 4] = [
    (DrawerDirection::Bottom, "Bottom"),
    (DrawerDirection::Top, "Top"),
    (DrawerDirection::Left, "Left"),
    (DrawerDirection::Right, "Right"),
];

/// One drawer per direction, each with its own button.
#[component]
fn DirectionExample(direction: DrawerDirection, name: &'static str) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <DrawerTrigger open=open variant=ButtonVariant::Outline>
            {name}
        </DrawerTrigger>
        <Drawer open=open label=format!("{name} drawer") direction=direction>
            <DrawerHeader>
                <DrawerTitle>{format!("{name} drawer")}</DrawerTitle>
                <DrawerDescription>
                    "Swipe it toward the " {direction.as_str()} " edge, press Esc or click outside to dismiss."
                </DrawerDescription>
            </DrawerHeader>
            <DrawerFooter>
                <DrawerClose variant=ButtonVariant::Outline>"Close"</DrawerClose>
            </DrawerFooter>
        </Drawer>
    }
}

/// A long list: scrolls inside the drawer, and hands the swipe to the drawer when at the top.
#[component]
fn LongList(count: usize) -> impl IntoView {
    view! {
        <div class="min-h-0 flex-1 space-y-3 overflow-y-auto px-4 py-4 text-sm" data-testid="scroller">
            {(1..=count)
                .map(|n| view! { <p>{format!("Entry {n}: a line of content that needs a little room.")}</p> })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn DrawersPage() -> impl IntoView {
    let basic_open = RwSignal::new(false);
    let goal = RwSignal::new(350i32);

    let snap_open = RwSignal::new(false);
    let snap_default_open = RwSignal::new(false);
    let controlled_snap = RwSignal::new(0usize);
    let controlled_snap_open = RwSignal::new(false);
    let snap_side_open = RwSignal::new(false);

    let no_handle_open = RwSignal::new(false);
    let scroll_open = RwSignal::new(false);
    let scroll_snap_open = RwSignal::new(false);
    let form_open = RwSignal::new(false);
    let name = RwSignal::new(String::from("Alpha"));

    let sticky_open = RwSignal::new(false);
    let non_modal_open = RwSignal::new(false);
    let clicks = RwSignal::new(0u32);

    let controlled_open = RwSignal::new(false);
    let scale_open = RwSignal::new(false);

    let responsive_open = RwSignal::new(false);
    let is_desktop = RwSignal::new(true);
    Effect::new(move |_| {
        let check = move || {
            let wide = window()
                .inner_width()
                .ok()
                .and_then(|w| w.as_f64())
                .unwrap_or(1024.0);
            is_desktop.set(wide >= 768.0);
        };
        check();
        let handle = window_event_listener(ev::resize, move |_| check());
        on_cleanup(move || handle.remove());
    });

    view! {
        <Page
            title="Drawer"
            summary="A panel pulled in from an edge that follows the finger: swipe it away, flick it, or let it rest at snap points. It is a native <dialog> like Sheet, so a modal drawer traps focus, handles Escape and makes the page inert; the swipe is Verdelle's own, with the release rules (snap, flick, dismiss) in a tested model in headless."
        >
            <Section title="Directions" description="Bottom (the default), top, left and right. Drag the panel, or its handle, toward its edge: past a quarter of the way or fast enough and it goes. On a phone use a touch drag.">
                <Row>
                    {DIRECTIONS
                        .into_iter()
                        .map(|(direction, name)| view! { <DirectionExample direction=direction name=name /> })
                        .collect_view()}
                </Row>
            </Section>

            <Section title="Basic" description="Header, a body and a footer, as shadcn's goal-setting example. The buttons close it with DrawerClose.">
                <Row>
                    <DrawerTrigger open=basic_open variant=ButtonVariant::Outline>
                        "Open drawer"
                    </DrawerTrigger>
                </Row>
                <Drawer open=basic_open label="Move goal">
                    <div class="mx-auto w-full max-w-sm">
                        <DrawerHeader>
                            <DrawerTitle>"Move goal"</DrawerTitle>
                            <DrawerDescription>"Set your daily activity goal."</DrawerDescription>
                        </DrawerHeader>
                        <div class="flex items-center justify-center gap-4 p-4">
                            <Button
                                variant=ButtonVariant::Outline
                                aria_label="Decrease"
                                on_click=move |_| goal.update(|g| *g = (*g - 10).max(200))
                            >
                                "-"
                            </Button>
                            <div class="text-center">
                                <div class="text-6xl font-bold tracking-tighter" data-testid="goal">
                                    {move || goal.get()}
                                </div>
                                <div class="text-xs uppercase text-muted-foreground">"Calories/day"</div>
                            </div>
                            <Button
                                variant=ButtonVariant::Outline
                                aria_label="Increase"
                                on_click=move |_| goal.update(|g| *g = (*g + 10).min(400))
                            >
                                "+"
                            </Button>
                        </div>
                        <DrawerFooter>
                            <DrawerClose>"Submit"</DrawerClose>
                            <DrawerClose variant=ButtonVariant::Outline>"Cancel"</DrawerClose>
                        </DrawerFooter>
                    </div>
                </Drawer>
            </Section>

            <Section title="Snap points" description="snap_points are visible fractions of the drawer (here 30%, 60% and 100%; it is 100dvh - 3rem tall). It opens at the first by default. A flick moves one point, a slow release picks the nearest, below the lowest dismisses, and a click on the handle cycles. At a lower point every swipe moves the drawer; once fully open, content scrolls first.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| snap_open.set(true)>
                        "Three snap points"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| snap_default_open.set(true)>
                        "default_snap = 2 (fully open)"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| snap_side_open.set(true)>
                        "Right, two snap points"
                    </Button>
                </Row>
                <Drawer open=snap_open label="Snap points" snap_points=vec![0.3, 0.6, 1.0]>
                    <DrawerHeader>
                        <DrawerTitle>"Three snap points"</DrawerTitle>
                        <DrawerDescription>"Drag the handle up and down, or click it."</DrawerDescription>
                    </DrawerHeader>
                    <LongList count=30 />
                    <DrawerFooter>
                        <DrawerClose variant=ButtonVariant::Outline>"Close"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
                <Drawer
                    open=snap_default_open
                    label="Starts open"
                    snap_points=vec![0.3, 0.6, 1.0]
                    default_snap=2
                >
                    <DrawerHeader>
                        <DrawerTitle>"default_snap = 2"</DrawerTitle>
                        <DrawerDescription>"Opens at 100%. Pull it down to land on 60%, then 30%."</DrawerDescription>
                    </DrawerHeader>
                    <LongList count=30 />
                </Drawer>
                <Drawer
                    open=snap_side_open
                    label="Right with snap points"
                    direction=DrawerDirection::Right
                    snap_points=vec![0.5, 1.0]
                >
                    <DrawerHeader>
                        <DrawerTitle>"Right drawer"</DrawerTitle>
                        <DrawerDescription>"Opens half way out; pull it left to open fully."</DrawerDescription>
                    </DrawerHeader>
                </Drawer>
            </Section>

            <Section title="Controlled snap point" description="active_snap is a signal holding the index of the snap point. The buttons set it, and swiping writes it back.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| controlled_snap_open.set(true)>
                        "Open"
                    </Button>
                    <span class="text-sm text-muted-foreground" data-testid="snap-state">
                        "Active snap index: " {move || controlled_snap.get()}
                    </span>
                </Row>
                <Drawer
                    open=controlled_snap_open
                    label="Controlled snap"
                    snap_points=vec![0.25, 0.5, 1.0]
                    active_snap=controlled_snap
                >
                    <DrawerHeader>
                        <DrawerTitle>"Controlled"</DrawerTitle>
                        <DrawerDescription>"Index " {move || controlled_snap.get()}</DrawerDescription>
                    </DrawerHeader>
                    <div class="flex gap-2 p-4">
                        {(0..3usize)
                            .map(|i| {
                                view! {
                                    <Button
                                        variant=ButtonVariant::Outline
                                        on_click=move |_| controlled_snap.set(i)
                                    >
                                        {format!("Snap {i}")}
                                    </Button>
                                }
                            })
                            .collect_view()}
                    </div>
                </Drawer>
            </Section>

            <Section title="Handle" description="The grab handle is on by default (shadcn shows it for every direction); show_handle=false removes it. The whole panel still drags.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| no_handle_open.set(true)>
                        "No handle"
                    </Button>
                </Row>
                <Drawer open=no_handle_open label="No handle" show_handle=false>
                    <DrawerHeader>
                        <DrawerTitle>"No handle"</DrawerTitle>
                        <DrawerDescription>"Drag anywhere on the panel."</DrawerDescription>
                    </DrawerHeader>
                    <DrawerFooter>
                        <DrawerClose variant=ButtonVariant::Outline>"Close"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
            </Section>

            <Section title="Scrolling content" description="A list inside the drawer scrolls with the finger; when it is at the top, the same downward swipe moves the drawer instead. With snap points the content only scrolls once the drawer is fully open.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| scroll_open.set(true)>
                        "Long list"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| scroll_snap_open.set(true)>
                        "Long list with snap points"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| form_open.set(true)>
                        "With a form"
                    </Button>
                </Row>
                <Drawer open=scroll_open label="Release notes">
                    <DrawerHeader>
                        <DrawerTitle>"Release notes"</DrawerTitle>
                        <DrawerDescription>"Scroll the list; pull down from the top to dismiss."</DrawerDescription>
                    </DrawerHeader>
                    <LongList count=60 />
                    <DrawerFooter>
                        <DrawerClose>"Done"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
                <Drawer open=scroll_snap_open label="Release notes, snap points" snap_points=vec![0.5, 1.0]>
                    <DrawerHeader>
                        <DrawerTitle>"Snap points and a list"</DrawerTitle>
                        <DrawerDescription>"At 50% the drawer takes every swipe; at 100% the list scrolls."</DrawerDescription>
                    </DrawerHeader>
                    <LongList count=60 />
                </Drawer>
                <Drawer open=form_open label="Edit name">
                    <DrawerHeader>
                        <DrawerTitle>"Edit name"</DrawerTitle>
                        <DrawerDescription>"Pressing in a field edits it, it does not start a drag."</DrawerDescription>
                    </DrawerHeader>
                    <div class="grid gap-3 p-4">
                        <Label>"Name"</Label>
                        <Input value=name aria_label="Name" />
                    </div>
                    <DrawerFooter>
                        <DrawerClose>"Save"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
            </Section>

            <Section title="Not dismissible" description="dismissible=false: swiping meets resistance and springs back, and Esc and the backdrop do nothing; only your buttons close it.">
                <Row>
                    <Button variant=ButtonVariant::Destructive on_click=move |_| sticky_open.set(true)>
                        "Must be answered"
                    </Button>
                </Row>
                <Drawer open=sticky_open label="Must be answered" dismissible=false>
                    <DrawerHeader>
                        <DrawerTitle>"Discard your changes?"</DrawerTitle>
                        <DrawerDescription>"Only the buttons close this one."</DrawerDescription>
                    </DrawerHeader>
                    <DrawerFooter>
                        <DrawerClose variant=ButtonVariant::Destructive>"Discard"</DrawerClose>
                        <DrawerClose variant=ButtonVariant::Outline>"Keep editing"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
            </Section>

            <Section title="Non-modal" description="modal=false: no backdrop, no focus trap, the page behind stays usable (try the counter while it is open). Still swipes, closes on Esc while focus is inside, and returns focus.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| non_modal_open.set(true)>
                        "Open non-modal"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=move |_| clicks.update(|c| *c += 1)
                    >
                        "Page button: " {move || clicks.get()}
                    </Button>
                </Row>
                <Drawer open=non_modal_open label="Non-modal drawer" modal=false snap_points=vec![0.35]>
                    <DrawerHeader>
                        <DrawerTitle>"Non-modal"</DrawerTitle>
                        <DrawerDescription>"The page behind still works."</DrawerDescription>
                    </DrawerHeader>
                    <DrawerFooter>
                        <DrawerClose variant=ButtonVariant::Outline>"Close"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
            </Section>

            <Section title="Controlled open" description="open is a signal: any code can set it. The drawer writes false back when it is dismissed.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| controlled_open.update(|o| *o = !*o)>
                        "Toggle"
                    </Button>
                    <span class="text-sm text-muted-foreground" data-testid="open-state">
                        "open = " {move || controlled_open.get().to_string()}
                    </span>
                </Row>
                <Drawer open=controlled_open label="Controlled drawer">
                    <DrawerHeader>
                        <DrawerTitle>"Controlled"</DrawerTitle>
                        <DrawerDescription>"Dismiss it any way you like; the text on the page follows."</DrawerDescription>
                    </DrawerHeader>
                </Drawer>
            </Section>

            <Section title="Scaled background" description="scale_background shrinks the DrawerWrapper around the page content while the drawer is open, following the drag (the wrapper here is just this card, the page would normally be wrapped in it).">
                <DrawerWrapper class="rounded-lg border p-4">
                    <Row>
                        <Button variant=ButtonVariant::Outline on_click=move |_| scale_open.set(true)>
                            "Open with scaled background"
                        </Button>
                        <span class="text-sm text-muted-foreground">"This card recedes."</span>
                    </Row>
                </DrawerWrapper>
                <Drawer open=scale_open label="Scaled background" scale_background=true>
                    <DrawerHeader>
                        <DrawerTitle>"Scaled background"</DrawerTitle>
                        <DrawerDescription>"Pull the handle and watch the card behind."</DrawerDescription>
                    </DrawerHeader>
                    <DrawerFooter>
                        <DrawerClose variant=ButtonVariant::Outline>"Close"</DrawerClose>
                    </DrawerFooter>
                </Drawer>
            </Section>

            <Section title="Responsive Dialog / Drawer" description="shadcn's pattern: a Dialog from 768px up, a Drawer below it, sharing one open signal and one body.">
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| responsive_open.set(true)>
                        "Edit profile"
                    </Button>
                    <span class="text-sm text-muted-foreground" data-testid="responsive-kind">
                        {move || if is_desktop.get() { "Desktop: Dialog" } else { "Mobile: Drawer" }}
                    </span>
                </Row>
                {move || {
                    if is_desktop.get() {
                        view! {
                            <Dialog open=responsive_open label="Edit profile">
                                <DialogHeader>
                                    <DialogTitle>"Edit profile"</DialogTitle>
                                    <DialogDescription>"Make changes to your profile here."</DialogDescription>
                                </DialogHeader>
                                <div class="grid gap-3">
                                    <Label>"Name"</Label>
                                    <Input value=name aria_label="Name" />
                                </div>
                                <DialogFooter>
                                    <Button on_click=move |_| responsive_open.set(false)>"Save changes"</Button>
                                </DialogFooter>
                            </Dialog>
                        }
                            .into_any()
                    } else {
                        view! {
                            <Drawer open=responsive_open label="Edit profile">
                                <DrawerHeader>
                                    <DrawerTitle>"Edit profile"</DrawerTitle>
                                    <DrawerDescription>"Make changes to your profile here."</DrawerDescription>
                                </DrawerHeader>
                                <div class="grid gap-3 p-4">
                                    <Label>"Name"</Label>
                                    <Input value=name aria_label="Name" />
                                </div>
                                <DrawerFooter>
                                    <DrawerClose>"Save changes"</DrawerClose>
                                    <DrawerClose variant=ButtonVariant::Outline>"Cancel"</DrawerClose>
                                </DrawerFooter>
                            </Drawer>
                        }
                            .into_any()
                    }
                }}
            </Section>
            <DrawerLifecycleExamples />
        </Page>
    }
}

/// `on_open_change` / `on_close`, and a drawer mounted only while needed (`DialogHost`).
#[component]
fn DrawerLifecycleExamples() -> impl IntoView {
    let open = RwSignal::new(false);
    let log = RwSignal::new(Vec::<String>::new());
    let wanted = RwSignal::new(false);

    view! {
        <Section
            title="Callbacks and DialogHost"
            description="on_open_change and on_close fire however the drawer closes (swipe, Escape, backdrop, a button). The second drawer is built only while `wanted` is true and still slides down, because DialogHost keeps it mounted through the exit."
        >
            <Row>
                <Button
                    attr:id="drawer-events-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| open.set(true)
                >
                    "With callbacks"
                </Button>
                <Button
                    attr:id="drawer-host-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| wanted.set(true)
                >
                    "Hosted drawer"
                </Button>
                <span id="drawer-events-readout" class="text-sm text-muted-foreground">
                    {move || log.get().join(", ")}
                </span>
            </Row>
            <Drawer
                open=open
                label="Callbacks"
                on_open_change=move |o: bool| log.update(|l| l.push(format!("change {o}")))
                on_close=move |_| log.update(|l| l.push("close".to_string()))
            >
                <DrawerHeader>
                    <DrawerTitle>"Callbacks"</DrawerTitle>
                    <DrawerDescription>"Swipe it down or press Escape."</DrawerDescription>
                </DrawerHeader>
            </Drawer>
            <DialogHost
                when=Signal::derive(move || wanted.get())
                on_close=move |_| wanted.set(false)
                let:open
            >
                <Drawer open=open label="Hosted">
                    <DrawerHeader>
                        <DrawerTitle>"Hosted drawer"</DrawerTitle>
                        <DrawerDescription>"Built only while wanted; slides out on close."</DrawerDescription>
                    </DrawerHeader>
                </Drawer>
            </DialogHost>
        </Section>
    }
}
