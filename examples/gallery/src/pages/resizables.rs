use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Orientation, ResizableHandle, ResizablePanel, ResizablePanelGroup, Size,
};

/// A panel's content: a centred label.
#[component]
fn Pane(label: &'static str) -> impl IntoView {
    view! {
        <div class="flex h-full items-center justify-center p-4">
            <span class="font-semibold">{label}</span>
        </div>
    }
}

fn fmt_sizes(sizes: &[f64]) -> String {
    sizes
        .iter()
        .map(|s| format!("{s:.1}%"))
        .collect::<Vec<_>>()
        .join(" / ")
}

#[component]
pub fn ResizablesPage() -> impl IntoView {
    let live = RwSignal::new(Vec::<f64>::new());
    let step = RwSignal::new(Vec::<f64>::new());
    let controlled = RwSignal::new(vec![25.0, 75.0]);
    let saves = RwSignal::new(0usize);
    let saved = RwSignal::new(String::from("nothing yet"));
    let sidebar_collapsed = RwSignal::new(false);
    let rail_collapsed = RwSignal::new(true);
    let vertical = RwSignal::new(false);
    let orientation = Signal::derive(move || {
        if vertical.get() {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        }
    });

    view! {
        <Page
            title="Resizable"
            summary="Panels that share a row or column, split by draggable handles (shadcn's Resizable, after react-resizable-panels). Sizes are percentages. Drag a handle, or focus it (Tab) and use the arrow keys along the layout, Shift for bigger steps, Home and End, and Enter on a collapsible neighbour."
        >
            <Section
                title="Horizontal"
                description="The default: panels side by side, handles are vertical lines. default_size sets a start; panels without one share the rest."
            >
                <ResizablePanelGroup
                    aria_label="Horizontal example"
                    class="max-w-md rounded-lg border min-h-[200px]"
                >
                    <ResizablePanel default_size=25.0>
                        <Pane label="Sidebar" />
                    </ResizablePanel>
                    <ResizableHandle aria_label="Resize sidebar" />
                    <ResizablePanel default_size=75.0>
                        <Pane label="Content" />
                    </ResizablePanel>
                </ResizablePanelGroup>
            </Section>

            <Section
                title="Vertical"
                description="orientation=Vertical stacks the panels; the handles are horizontal lines and the Up and Down arrows move them."
            >
                <ResizablePanelGroup
                    orientation=Orientation::Vertical
                    aria_label="Vertical example"
                    class="min-h-[200px] max-w-md rounded-lg border"
                >
                    <ResizablePanel default_size=25.0>
                        <Pane label="Header" />
                    </ResizablePanel>
                    <ResizableHandle aria_label="Resize header" />
                    <ResizablePanel default_size=75.0>
                        <Pane label="Content" />
                    </ResizablePanel>
                </ResizablePanelGroup>
            </Section>

            <Section
                title="Nested"
                description="A panel can hold another group, here a column inside the right half. Each group keeps its own sizes."
            >
                <ResizablePanelGroup
                    aria_label="Nested example"
                    class="min-h-[220px] max-w-md rounded-lg border"
                >
                    <ResizablePanel default_size=50.0>
                        <Pane label="One" />
                    </ResizablePanel>
                    <ResizableHandle aria_label="Resize one" />
                    <ResizablePanel default_size=50.0>
                        <ResizablePanelGroup orientation=Orientation::Vertical aria_label="Right column">
                            <ResizablePanel default_size=25.0>
                                <Pane label="Two" />
                            </ResizablePanel>
                            <ResizableHandle aria_label="Resize two" />
                            <ResizablePanel default_size=75.0>
                                <Pane label="Three" />
                            </ResizablePanel>
                        </ResizablePanelGroup>
                    </ResizablePanel>
                </ResizablePanelGroup>
            </Section>

            <Section
                title="With handle"
                description="with_handle adds the grip in the middle of the handle (turned on its side in a vertical group)."
            >
                <div class="grid gap-4 md:grid-cols-2">
                    <ResizablePanelGroup aria_label="Grip example" class="min-h-[200px] rounded-lg border">
                        <ResizablePanel default_size=50.0>
                            <Pane label="One" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize" />
                        <ResizablePanel default_size=50.0>
                            <Pane label="Two" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                    <ResizablePanelGroup
                        orientation=Orientation::Vertical
                        aria_label="Vertical grip example"
                        class="min-h-[200px] rounded-lg border"
                    >
                        <ResizablePanel default_size=50.0>
                            <Pane label="One" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize" />
                        <ResizablePanel default_size=50.0>
                            <Pane label="Two" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </div>
            </Section>

            <Section
                title="Min and max size"
                description="The first panel stays between 20% and 40%, the last at least 30%. The handle stops there (aria-valuemin and aria-valuemax say where)."
            >
                <ResizablePanelGroup aria_label="Limits example" class="min-h-[160px] max-w-lg rounded-lg border">
                    <ResizablePanel default_size=30.0 min_size=20.0 max_size=40.0>
                        <Pane label="20 to 40" />
                    </ResizablePanel>
                    <ResizableHandle with_handle=true aria_label="Resize limited panel" />
                    <ResizablePanel default_size=40.0>
                        <Pane label="Free" />
                    </ResizablePanel>
                    <ResizableHandle with_handle=true aria_label="Resize last panel" />
                    <ResizablePanel default_size=30.0 min_size=30.0>
                        <Pane label="30 or more" />
                    </ResizablePanel>
                </ResizablePanelGroup>
            </Section>

            <Section
                title="Collapsible"
                description="A collapsible panel dragged past halfway under its min_size snaps shut (a key press collapses it at once); drag it back out to open. Enter on the handle toggles it, and the collapsed signal does the same from outside."
            >
                <div class="space-y-3">
                    <Row label="Controls">
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| sidebar_collapsed.update(|c| *c = !*c)
                        >
                            {move || if sidebar_collapsed.get() { "Expand sidebar" } else { "Collapse sidebar" }}
                        </Button>
                        <span class="text-sm text-muted-foreground" data-testid="collapsed-state">
                            {move || if sidebar_collapsed.get() { "collapsed" } else { "open" }}
                        </span>
                    </Row>
                    <ResizablePanelGroup aria_label="Collapsible example" class="min-h-[160px] max-w-lg rounded-lg border">
                        <ResizablePanel
                            default_size=30.0
                            min_size=15.0
                            max_size=45.0
                            collapsible=true
                            collapsed=sidebar_collapsed
                        >
                            <Pane label="Sidebar" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize collapsible sidebar" />
                        <ResizablePanel default_size=70.0>
                            <Pane label="Content" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                    <Row label="Strip">
                        <span class="text-sm text-muted-foreground">
                            "collapsed_size=6 leaves a narrow strip when shut; it starts collapsed here."
                        </span>
                    </Row>
                    <ResizablePanelGroup aria_label="Strip example" class="min-h-[120px] max-w-lg rounded-lg border">
                        <ResizablePanel
                            default_size=25.0
                            min_size=15.0
                            collapsible=true
                            collapsed_size=6.0
                            collapsed=rail_collapsed
                        >
                            <Pane label="Rail" />
                        </ResizablePanel>
                        <ResizableHandle aria_label="Resize rail" />
                        <ResizablePanel>
                            <Pane label="Content" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </div>
            </Section>

            <Section
                title="Keyboard"
                description="Tab to a handle. Arrow keys along the layout move it by keyboard_step (here 5%), Shift moves ten times as much, Home and End go to either end. on_layout_change reports every change."
            >
                <div class="space-y-3">
                    <ResizablePanelGroup
                        aria_label="Keyboard example"
                        keyboard_step=5.0
                        on_layout_change=Callback::new(move |s: Vec<f64>| step.set(s))
                        class="min-h-[120px] max-w-lg rounded-lg border"
                    >
                        <ResizablePanel default_size=20.0>
                            <Pane label="A" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize A and B" />
                        <ResizablePanel default_size=50.0>
                            <Pane label="B" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize B and C" />
                        <ResizablePanel default_size=30.0>
                            <Pane label="C" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                    <p class="text-sm text-muted-foreground" data-testid="keyboard-sizes">
                        "Last change: "
                        {move || {
                            let s = step.get();
                            if s.is_empty() { "none yet".to_string() } else { fmt_sizes(&s) }
                        }}
                    </p>
                </div>
            </Section>

            <Section
                title="Controlled sizes and saving"
                description="sizes is a two-way signal: set it to move the panels, read it for the layout. on_layout_end fires once when a drag or key press ends, which is where to save a layout (the kit has no storage of its own; start the signal from a saved value to restore one). on_layout_change fires on every step."
            >
                <div class="space-y-3">
                    <Row label="Set">
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(vec![25.0, 75.0])
                        >
                            "25 / 75"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(vec![50.0, 50.0])
                        >
                            "50 / 50"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| controlled.set(vec![80.0, 20.0])
                        >
                            "80 / 20"
                        </Button>
                    </Row>
                    <ResizablePanelGroup
                        aria_label="Controlled example"
                        sizes=controlled
                        on_layout_end=Callback::new(move |s: Vec<f64>| {
                            saves.update(|n| *n += 1);
                            saved.set(fmt_sizes(&s));
                        })
                        class="min-h-[120px] max-w-lg rounded-lg border"
                    >
                        <ResizablePanel>
                            <Pane label="Left" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize controlled" />
                        <ResizablePanel>
                            <Pane label="Right" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                    <p class="text-sm text-muted-foreground" data-testid="controlled-sizes">
                        "Signal: " {move || fmt_sizes(&controlled.get())}
                    </p>
                    <p class="text-sm text-muted-foreground" data-testid="saved-sizes">
                        "Saved " {move || saves.get()} " times, last: " {move || saved.get()}
                    </p>
                    <p class="text-sm text-muted-foreground">
                        "A layout that does not fit the panels (wrong count, not adding up to 100) is ignored."
                    </p>
                    <Row label="Live">
                        <span class="text-sm text-muted-foreground" data-testid="live-sizes">
                            {move || fmt_sizes(&live.get())}
                        </span>
                    </Row>
                    <ResizablePanelGroup
                        aria_label="Live example"
                        on_layout_change=Callback::new(move |s: Vec<f64>| live.set(s))
                        class="min-h-[80px] max-w-lg rounded-lg border"
                    >
                        <ResizablePanel default_size=40.0>
                            <Pane label="on_layout_change" />
                        </ResizablePanel>
                        <ResizableHandle aria_label="Resize live" />
                        <ResizablePanel default_size=60.0>
                            <Pane label="reports each step" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </div>
            </Section>

            <Section
                title="Disabled handle"
                description="disabled fixes the boundary: the handle leaves the tab order and ignores the pointer and keys. The other handle still works."
            >
                <ResizablePanelGroup aria_label="Disabled example" class="min-h-[120px] max-w-lg rounded-lg border">
                    <ResizablePanel default_size=30.0>
                        <Pane label="Fixed" />
                    </ResizablePanel>
                    <ResizableHandle disabled=true aria_label="Fixed boundary" />
                    <ResizablePanel default_size=40.0>
                        <Pane label="Middle" />
                    </ResizablePanel>
                    <ResizableHandle with_handle=true aria_label="Free boundary" />
                    <ResizablePanel default_size=30.0>
                        <Pane label="Free" />
                    </ResizablePanel>
                </ResizablePanelGroup>
            </Section>

            <Section
                title="Reactive orientation"
                description="orientation takes a signal; the layout, the handle direction and the arrow keys follow it (sizes are kept)."
            >
                <div class="space-y-3">
                    <Row label="Layout">
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| vertical.update(|v| *v = !*v)
                        >
                            {move || if vertical.get() { "Make horizontal" } else { "Make vertical" }}
                        </Button>
                    </Row>
                    <ResizablePanelGroup
                        orientation=orientation
                        aria_label="Reactive example"
                        class="min-h-[200px] max-w-md rounded-lg border"
                    >
                        <ResizablePanel default_size=35.0>
                            <Pane label="One" />
                        </ResizablePanel>
                        <ResizableHandle with_handle=true aria_label="Resize reactive" />
                        <ResizablePanel default_size=65.0>
                            <Pane label="Two" />
                        </ResizablePanel>
                    </ResizablePanelGroup>
                </div>
            </Section>
        </Page>
    }
}
