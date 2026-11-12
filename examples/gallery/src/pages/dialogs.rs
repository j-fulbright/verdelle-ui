use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogHost, DialogTitle,
};

#[component]
pub fn DialogsPage() -> impl IntoView {
    let dialog_open = RwSignal::new(false);
    let confirm_open = RwSignal::new(false);
    let last_action = RwSignal::new(String::from("none yet"));

    view! {
        <Page
            title="Dialogs"
            summary="A native <dialog>: the browser traps focus, handles Escape, makes the page inert and stacks it on top. These sit inside a space-y container on purpose, to prove they stay centred."
        >
            <Section title="Dismissal options" description="Each of the close button, Escape and a click on the backdrop can be turned off.">
                <div class="space-y-3">
                    <Row>
                        <Button variant=ButtonVariant::Outline on_click=move |_| dialog_open.set(true)>
                            "Open dialog"
                        </Button>
                        <Button variant=ButtonVariant::Destructive on_click=move |_| confirm_open.set(true)>
                            "Open confirm (no Esc / backdrop / X)"
                        </Button>
                        <span class="text-sm text-muted-foreground">"Last action: " {move || last_action.get()}</span>
                    </Row>

                    <Dialog open=dialog_open label="Edit branch">
                        <DialogHeader>
                            <DialogTitle>"Edit branch"</DialogTitle>
                            <DialogDescription>
                                "Esc, the X, a backdrop click or either button closes this. Tab stays inside the dialog."
                            </DialogDescription>
                        </DialogHeader>
                        <DialogContent>
                            <p class="text-sm">"Dialog body. It scrolls if it grows taller than the viewport."</p>
                        </DialogContent>
                        <DialogFooter>
                            <Button
                                variant=ButtonVariant::Outline
                                on_click=move |_| {
                                    last_action.set("cancelled".into());
                                    dialog_open.set(false);
                                }
                            >
                                "Cancel"
                            </Button>
                            <Button on_click=move |_| {
                                last_action.set("saved".into());
                                dialog_open.set(false);
                            }>"Save"</Button>
                        </DialogFooter>
                    </Dialog>

                    <Dialog
                        open=confirm_open
                        label="Delete branch"
                        width=380
                        overlay_closable=false
                        close_on_escape=false
                        show_close_button=false
                    >
                        <DialogHeader>
                            <DialogTitle>"Delete branch?"</DialogTitle>
                            <DialogDescription>
                                "This one ignores Esc, the backdrop and has no X, so you must choose."
                            </DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <Button
                                variant=ButtonVariant::Outline
                                on_click=move |_| {
                                    last_action.set("kept".into());
                                    confirm_open.set(false);
                                }
                            >
                                "Keep"
                            </Button>
                            <Button
                                variant=ButtonVariant::Destructive
                                on_click=move |_| {
                                    last_action.set("deleted".into());
                                    confirm_open.set(false);
                                }
                            >
                                "Delete"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                </div>
            </Section>
            <DialogLifecycleExamples />
        </Page>
    }
}

/// `on_open_change` / `on_close` callbacks and `DialogHost` (mount a dialog only while needed,
/// and still animate it out).
#[component]
fn DialogLifecycleExamples() -> impl IntoView {
    // on_open_change / on_close
    let events_open = RwSignal::new(false);
    let changes = RwSignal::new(Vec::<String>::new());
    let closes = RwSignal::new(0u32);

    // DialogHost with a row of data
    let selected = RwSignal::new(None::<u32>);
    // Kept after the selection is cleared, so the content is still right while it animates out.
    let shown = RwSignal::new(0u32);
    // The pattern DialogHost replaces: a Show around the dialog, which cuts the exit animation.
    let naive = RwSignal::new(false);

    view! {
        <Section
            title="on_open_change and on_close"
            description="No Effect on the open signal needed: on_open_change gets every change (true or false) and on_close runs when it closes, whether by Escape, the backdrop, the X or a button. Both fire when the change is requested, at the start of the exit animation."
        >
            <Row>
                <Button
                    attr:id="events-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| events_open.set(true)
                >
                    "Open"
                </Button>
                <span id="events-readout" class="text-sm text-muted-foreground">
                    "changes: " {move || changes.get().join(", ")} " | closes: " {move || closes.get()}
                </span>
            </Row>
            <Dialog
                open=events_open
                label="Callbacks"
                on_open_change=move |o: bool| changes.update(|c| c.push(o.to_string()))
                on_close=move |_| closes.update(|n| *n += 1)
            >
                <DialogHeader>
                    <DialogTitle>"Callbacks"</DialogTitle>
                    <DialogDescription>
                        "Close it any way you like and watch the readout beside the button."
                    </DialogDescription>
                </DialogHeader>
            </Dialog>
        </Section>

        <Section
            title="DialogHost: mount while needed, still animate out"
            description="Removing a dialog from the page when its state changes (a Show or an Option in the view) skips the exit animation: the element is gone before it can play. DialogHost builds the dialog when `when` turns true, passes it an open signal, and removes it only after the exit has run. Compare the two buttons."
        >
            <div class="space-y-3">
                <Row label="With DialogHost">
                    {[1u32, 2, 3]
                        .into_iter()
                        .map(|id| {
                            view! {
                                <Button
                                    attr:id=format!("host-open-{id}")
                                    variant=ButtonVariant::Outline
                                    on_click=move |_| {
                                        shown.set(id);
                                        selected.set(Some(id));
                                    }
                                >
                                    {format!("Row {id}")}
                                </Button>
                            }
                        })
                        .collect_view()}
                    <span id="host-readout" class="text-sm text-muted-foreground">
                        "selected: " {move || selected.get().map(|n| n.to_string()).unwrap_or("none".into())}
                    </span>
                </Row>
                <DialogHost
                    when=Signal::derive(move || selected.get().is_some())
                    on_close=move |_| selected.set(None)
                    let:open
                >
                    <Dialog open=open label="Row details">
                        <DialogHeader>
                            <DialogTitle>{move || format!("Row {}", shown.get())}</DialogTitle>
                            <DialogDescription>
                                "Mounted only while a row is selected, yet it fades out when you close it. The dialog is removed about 350ms after it closes."
                            </DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <Button on_click=move |_| open.set(false)>"Done"</Button>
                        </DialogFooter>
                    </Dialog>
                </DialogHost>

                <Row label="Without it">
                    <Button
                        attr:id="naive-open"
                        variant=ButtonVariant::Outline
                        on_click=move |_| naive.set(true)
                    >
                        "Show-wrapped dialog"
                    </Button>
                    <span class="text-sm text-muted-foreground">
                        "Closing this one cuts the animation short: it disappears at once."
                    </span>
                </Row>
                <Show when=move || naive.get()>
                    <Dialog open=RwSignal::new(true) label="No exit animation">
                        <DialogHeader>
                            <DialogTitle>"No exit animation"</DialogTitle>
                            <DialogDescription>
                                "The Show around this dialog removes it the moment the Done button runs."
                            </DialogDescription>
                        </DialogHeader>
                        <DialogFooter>
                            <Button on_click=move |_| naive.set(false)>"Done"</Button>
                        </DialogFooter>
                    </Dialog>
                </Show>
            </div>
        </Section>
    }
}
