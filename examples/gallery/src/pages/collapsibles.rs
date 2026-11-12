use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Collapsible, CollapsibleContent, CollapsibleTrigger, Icon, IconName,
    Size, Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};

#[component]
fn ChevronsIcon() -> impl IntoView {
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
            <path d="m7 15 5 5 5-5M7 9l5-5 5 5"></path>
        </svg>
    }
}

#[component]
fn Repo(name: &'static str) -> impl IntoView {
    view! { <div class="rounded-md border border-border px-4 py-2 font-mono text-sm">{name}</div> }
}

#[component]
pub fn CollapsiblesPage() -> impl IntoView {
    let open = RwSignal::new(false);
    let changes = RwSignal::new(0u32);

    view! {
        <Page
            title="Collapsible"
            summary="One trigger shows and hides one region. The trigger has aria-expanded and aria-controls; the content is role=region and hidden while closed."
        >
            <Section title="Uncontrolled" description="Starts closed; the trigger is a Button-styled icon button, as in shadcn's example.">
                <Collapsible class="flex w-80 max-w-full flex-col gap-2">
                    <div class="flex items-center justify-between gap-4">
                        <h4 class="text-sm font-semibold">"@peduarte starred 3 repositories"</h4>
                        <CollapsibleTrigger
                            variant=ButtonVariant::Ghost
                            size=Size::Small
                            icon_only=true
                        >
                            <ChevronsIcon />
                            <span class="sr-only">"Toggle"</span>
                        </CollapsibleTrigger>
                    </div>
                    <Repo name="@radix-ui/primitives" />
                    <CollapsibleContent class="flex flex-col gap-2">
                        <Repo name="@radix-ui/colors" />
                        <Repo name="@stitches/react" />
                    </CollapsibleContent>
                </Collapsible>
            </Section>

            <Section title="Controlled" description="open and on_open_change; the outside button and the trigger drive the same signal.">
                <Collapsible
                    open=open
                    on_open_change=move |v| {
                        changes.update(|n| *n += 1);
                        open.set(v);
                    }
                    class="flex w-80 max-w-full flex-col gap-2"
                >
                    <CollapsibleTrigger variant=ButtonVariant::Outline class="self-start">"Details"</CollapsibleTrigger>
                    <CollapsibleContent>
                        <div class="rounded-md border border-border px-4 py-3 text-sm">"Hidden until opened."</div>
                    </CollapsibleContent>
                </Collapsible>
                <Row>
                    <Button
                        variant=ButtonVariant::Secondary
                        size=Size::Small
                        on_click=move |_| open.update(|o| *o = !*o)
                    >
                        "Toggle from outside"
                    </Button>
                    <span id="collapsible-readout" class="text-sm text-muted-foreground">
                        "open=" {move || open.get().to_string()} ", on_open_change calls=" {move || changes.get()}
                    </span>
                </Row>
            </Section>

            <Section title="Default open, plain trigger, disabled" description="default_open starts it open. A trigger without a variant is unstyled text. disabled blocks the trigger.">
                <div class="grid max-w-sm gap-6">
                    <Collapsible default_open=true class="flex flex-col gap-2">
                        <CollapsibleTrigger class="text-sm font-medium underline">
                            "Plain text trigger"
                        </CollapsibleTrigger>
                        <CollapsibleContent>
                            <p class="text-sm text-muted-foreground">"Open on first render."</p>
                        </CollapsibleContent>
                    </Collapsible>
                    <Collapsible disabled=true class="flex flex-col gap-2">
                        <CollapsibleTrigger variant=ButtonVariant::Outline class="self-start">"Disabled"</CollapsibleTrigger>
                        <CollapsibleContent>
                            <p class="text-sm">"You cannot reach this."</p>
                        </CollapsibleContent>
                    </Collapsible>
                    <Collapsible class="flex flex-col gap-2">
                        <CollapsibleTrigger variant=ButtonVariant::Outline class="self-start">"force_mount"</CollapsibleTrigger>
                        <CollapsibleContent force_mount=true>
                            <p class="text-sm">"Kept in the page (hidden) while closed."</p>
                        </CollapsibleContent>
                    </Collapsible>
                </div>
            </Section>
            <SharedStateExamples />
        </Page>
    }
}

/// A shared state for a trigger and content that do not share one wrapper: a table row and the
/// row under it. Also the rotating icon hook.
#[component]
fn SharedStateExamples() -> impl IntoView {
    let orders = [
        ("A-100", "Pending"),
        ("A-101", "Shipped"),
        ("A-102", "Delivered"),
    ];
    let icon_state = verdelle_ui::use_collapsible(false);

    view! {
        <Section
            title="Shared state: trigger and content in different table rows"
            description="use_collapsible makes a CollapsibleState; pass it as `state` to the trigger and to the content and they need no common wrapper. The content here is a <tr> (as_child), hidden while closed; the chevron has the class vdl-collapsible__icon, so it turns with the trigger's data-state."
        >
            <div class="rounded-lg border">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>""</TableHead>
                            <TableHead>"Order"</TableHead>
                            <TableHead>"Status"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {orders
                            .into_iter()
                            .map(|(order, status)| {
                                let state = verdelle_ui::use_collapsible(order == "A-101");
                                view! {
                                    <TableRow>
                                        <TableCell>
                                            <CollapsibleTrigger
                                                state=state
                                                attr:id=format!("row-trigger-{order}")
                                                class="flex size-6 items-center justify-center"
                                            >
                                                <span class="sr-only">{format!("Toggle {order}")}</span>
                                                <Icon name=IconName::ChevronDown class="vdl-collapsible__icon" />
                                            </CollapsibleTrigger>
                                        </TableCell>
                                        <TableCell>{order}</TableCell>
                                        <TableCell>
                                            <span id=format!("row-state-{order}")>
                                                {status} " (" {move || state.data_state()} ")"
                                            </span>
                                        </TableCell>
                                    </TableRow>
                                    <CollapsibleContent state=state as_child=true>
                                        <TableRow>
                                            <TableCell col_span=3>
                                                {format!("Details for {order}: line items, carrier and tracking number.")}
                                            </TableCell>
                                        </TableRow>
                                    </CollapsibleContent>
                                }
                            })
                            .collect_view()}
                    </TableBody>
                </Table>
            </div>
        </Section>

        <Section
            title="A handle can also drive the state from code"
            description="The handle is Copy: read state.open, set it, or call toggle(). This one is opened by a button that is not a CollapsibleTrigger, and its icon is outside both parts."
        >
            <Row>
                <Button attr:id="handle-toggle" variant=ButtonVariant::Outline on_click=move |_| icon_state.toggle()>
                    "Toggle from outside"
                </Button>
                <Icon
                    name=IconName::ChevronDown
                    class="vdl-collapsible__icon"
                    attr:data-state=move || icon_state.data_state()
                />
                <span id="handle-readout" class="text-sm text-muted-foreground">
                    "open = " {move || icon_state.open.get().to_string()}
                </span>
            </Row>
            <CollapsibleContent state=icon_state>
                <p id="handle-content" class="rounded-md border p-3 text-sm">
                    "This content follows a handle that no wrapper owns."
                </p>
            </CollapsibleContent>
        </Section>
    }
}
