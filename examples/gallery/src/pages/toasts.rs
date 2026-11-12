use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::primitives::toast::use_toast_controller;
use verdelle_ui::{
    Button, ButtonVariant, Toast, ToastAction, ToastData, ToastDescription, ToastPosition,
    ToastProvider, ToastTitle, ToastVariant, ToastViewport, Toaster, use_toaster,
};

const POSITIONS: [(ToastPosition, &str); 6] = [
    (ToastPosition::TopLeft, "Top left"),
    (ToastPosition::TopCenter, "Top center"),
    (ToastPosition::TopRight, "Top right"),
    (ToastPosition::BottomLeft, "Bottom left"),
    (ToastPosition::BottomCenter, "Bottom center"),
    (ToastPosition::BottomRight, "Bottom right"),
];

const VARIANTS: [(ToastVariant, &str); 5] = [
    (ToastVariant::Default, "Default"),
    (ToastVariant::Success, "Success"),
    (ToastVariant::Info, "Info"),
    (ToastVariant::Warning, "Warning"),
    (ToastVariant::Destructive, "Destructive"),
];

#[component]
pub fn ToastsPage() -> impl IntoView {
    let position = RwSignal::new(ToastPosition::BottomRight);

    view! {
        <Page
            title="Toasts"
            summary="Notifications that close themselves. Hovering or focusing the viewport (F8) pauses every timer and resumes with the time that was left; drag a toast towards the screen edge, press Escape on it, or use the X to dismiss it. Built on the headless Radix-shaped toast primitive and styled like shadcn's Sonner."
        >
            <Toaster position=position>
                <ImperativeDemo position=position />
            </Toaster>
            <DeclarativeDemo />
        </Page>
    }
}

/// `use_toaster()`: no list to manage, one call per toast.
#[component]
fn ImperativeDemo(position: RwSignal<ToastPosition>) -> impl IntoView {
    let toaster = use_toaster();
    let duration = RwSignal::new(5000_i32);
    let opened = RwSignal::new(0_u32);
    let closed = RwSignal::new(0_u32);

    let show = move |data: ToastData| {
        opened.update(|n| *n += 1);
        toaster.show(
            data.duration(duration.get_untracked())
                .on_close(move || closed.update(|n| *n += 1)),
        );
    };

    view! {
        <Section
            title="Toaster and use_toaster"
            description="Wrap the app in Toaster, call use_toaster() anywhere below it and show a toast with one call. Clear all closes every open toast, each running its on_close."
        >
            <div class="space-y-3">
                <Row label="Duration">
                    <Button
                        variant=Signal::derive(move || if duration.get() == 3000 { ButtonVariant::Default } else { ButtonVariant::Outline })
                        on_click=move |_| duration.set(3000)
                    >
                        "3 s"
                    </Button>
                    <Button
                        variant=Signal::derive(move || if duration.get() == 5000 { ButtonVariant::Default } else { ButtonVariant::Outline })
                        on_click=move |_| duration.set(5000)
                    >
                        "5 s"
                    </Button>
                    <Button
                        variant=Signal::derive(move || if duration.get() == 0 { ButtonVariant::Default } else { ButtonVariant::Outline })
                        on_click=move |_| duration.set(0)
                    >
                        "Stay until closed"
                    </Button>
                </Row>
                <Row label="Variant">
                    {VARIANTS
                        .into_iter()
                        .map(|(variant, name)| {
                            view! {
                                <Button
                                    variant=ButtonVariant::Outline
                                    on_click=move |_| {
                                        show(
                                            ToastData::new(format!("{name} toast"))
                                                .description("Closes on its timer, the X, a swipe, Escape, or Clear all.")
                                                .variant(variant),
                                        )
                                    }
                                >
                                    {name}
                                </Button>
                            }
                        })
                        .collect_view()}
                </Row>
                <Row label="Position">
                    {POSITIONS
                        .into_iter()
                        .map(|(value, name)| {
                            view! {
                                <Button
                                    variant=Signal::derive(move || if position.get() == value { ButtonVariant::Default } else { ButtonVariant::Outline })
                                    on_click=move |_| position.set(value)
                                >
                                    {name}
                                </Button>
                            }
                        })
                        .collect_view()}
                </Row>
                <Row>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=move |_| show(ToastData::new("Event created").description("Sunday, December 03, 2023 at 9:00 AM"))
                    >
                        "Add toast"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=move |_| {
                            show(
                                ToastData::new("Event moved to the trash")
                                    .description("You can bring it back.")
                                    .action("Undo", "Undo moving the event to the trash", || {}),
                            )
                        }
                    >
                        "Add toast with action"
                    </Button>
                    <Button variant=ButtonVariant::Destructive on_click=move |_| toaster.dismiss_all()>
                        "Clear all"
                    </Button>
                    <span class="text-sm text-muted-foreground">
                        "Open: " {move || opened.get().saturating_sub(closed.get())} " · on_close calls: " {move || closed.get()}
                    </span>
                </Row>
            </div>
        </Section>
    }
}

/// One toast of the declarative demo: an id, and whether it carries an action button.
#[derive(Clone, PartialEq)]
struct Item {
    id: u32,
    with_action: bool,
}

/// The parts composed by hand: its own provider and viewport (bottom left, so it does not
/// overlap the Toaster above), and a list the page manages itself.
#[component]
fn DeclarativeDemo() -> impl IntoView {
    view! {
        <Section
            title="Composed by hand"
            description="ToastProvider, ToastViewport and Toast with a list the page keeps. This viewport is bottom left. Use it when a toast needs custom content."
        >
            <ToastProvider position=ToastPosition::BottomLeft>
                <DeclarativeBody />
            </ToastProvider>
        </Section>
    }
}

#[component]
fn DeclarativeBody() -> impl IntoView {
    let controller = use_toast_controller();
    let items = RwSignal::new(Vec::<Item>::new());
    let next_id = RwSignal::new(1_u32);
    let closed = RwSignal::new(0_u32);

    let add = move |with_action: bool| {
        let id = next_id.get_untracked();
        next_id.set(id + 1);
        items.update(|v| v.push(Item { id, with_action }));
    };

    view! {
        <Row>
            <Button variant=ButtonVariant::Outline on_click=move |_| add(false)>"Add toast"</Button>
            <Button variant=ButtonVariant::Outline on_click=move |_| add(true)>"Add toast with action"</Button>
            <Button
                variant=ButtonVariant::Destructive
                on_click=move |_| {
                    if let Some(c) = controller {
                        c.dismiss_all();
                    }
                }
            >
                "Clear all"
            </Button>
            <span class="text-sm text-muted-foreground">
                "Open: " {move || items.get().len()} " · on_open_change(false) calls: " {move || closed.get()}
            </span>
        </Row>

        <ToastViewport>
            <For each=move || items.get() key=|item| item.id let:item>
                {
                    let id = item.id;
                    let with_action = item.with_action;
                    view! {
                        <Toast
                            default_open=true
                            on_open_change=Callback::new(move |open: bool| {
                                if !open {
                                    closed.update(|n| *n += 1);
                                    // Keep the toast in the list while its exit animation plays.
                                    set_timeout(
                                        move || items.update(|v| v.retain(|i| i.id != id)),
                                        std::time::Duration::from_millis(400),
                                    );
                                }
                            })
                        >
                            <ToastTitle>{format!("Toast #{id}")}</ToastTitle>
                            <ToastDescription>"Closes on its timer, the X, a swipe left, or Clear all."</ToastDescription>
                            {with_action.then(|| view! {
                                <ToastAction alt_text="Undo the last change">"Undo"</ToastAction>
                            })}
                        </Toast>
                    }
                }
            </For>
        </ToastViewport>
    }
}
