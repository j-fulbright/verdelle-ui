use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogMedia, AlertDialogTitle, Button, ButtonVariant, DialogHost, Size,
};

#[component]
fn TrashIcon() -> impl IntoView {
    view! {
        <svg
            width="32"
            height="32"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M3 6h18M8 6V4h8v2M19 6l-1 14H6L5 6M10 11v6M14 11v6"></path>
        </svg>
    }
}

#[component]
pub fn AlertDialogsPage() -> impl IntoView {
    let confirm_open = RwSignal::new(false);
    let small_open = RwSignal::new(false);
    let media_open = RwSignal::new(false);
    let escape_open = RwSignal::new(false);
    let backdrop_open = RwSignal::new(false);
    let log = RwSignal::new(String::from("nothing yet"));
    let confirmed = RwSignal::new(0u32);

    view! {
        <Page
            title="Alert Dialog"
            summary="A native <dialog> with role=alertdialog for a decision the user must make. Focus starts on Cancel, Escape and a backdrop click do not dismiss it, and there is no close button."
        >
            <Section
                title="Destructive confirmation"
                description="Cancel and Action both close it and report through their callbacks. Tab stays inside; focus returns to the opening button."
            >
                <Row>
                    <Button
                        attr:id="open-confirm"
                        variant=ButtonVariant::Destructive
                        on_click=move |_| confirm_open.set(true)
                    >
                        "Delete account"
                    </Button>
                    <span id="alert-readout" class="text-sm text-muted-foreground">
                        "Last: " {move || log.get()} " | confirmed " {move || confirmed.get()} " time(s) | open="
                        {move || confirm_open.get().to_string()}
                    </span>
                </Row>
                <AlertDialog open=confirm_open>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel on_click=move |_| log.set("cancel".into())>
                            "Cancel"
                        </AlertDialogCancel>
                        <AlertDialogAction
                            variant=ButtonVariant::Destructive
                            on_click=move |_| {
                                log.set("delete".into());
                                confirmed.update(|n| *n += 1);
                            }
                        >
                            "Delete"
                        </AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
            </Section>

            <Section
                title="Sizes and media"
                description="Size::Small is a 320px dialog with centred text and a two-column footer. AlertDialogMedia adds an icon tile beside (default) or above (small) the title."
            >
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| small_open.set(true)>
                        "Small"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| media_open.set(true)>
                        "Default with media"
                    </Button>
                </Row>
                <AlertDialog open=small_open size=Size::Small>
                    <AlertDialogHeader>
                        <AlertDialogMedia>
                            <TrashIcon />
                        </AlertDialogMedia>
                        <AlertDialogTitle>"Delete chat?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "This will permanently delete this chat conversation."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction variant=ButtonVariant::Destructive>"Delete"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
                <AlertDialog open=media_open>
                    <AlertDialogHeader>
                        <AlertDialogMedia>
                            <TrashIcon />
                        </AlertDialogMedia>
                        <AlertDialogTitle>"Delete chat?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "This will permanently delete this chat conversation. View settings or archive it instead."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction>"Continue"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
            </Section>

            <Section
                title="Dismissal options"
                description="Escape and a backdrop click are off by default; each can be switched on."
            >
                <Row>
                    <Button variant=ButtonVariant::Outline on_click=move |_| escape_open.set(true)>
                        "close_on_escape"
                    </Button>
                    <Button variant=ButtonVariant::Outline on_click=move |_| backdrop_open.set(true)>
                        "overlay_closable"
                    </Button>
                </Row>
                <AlertDialog open=escape_open close_on_escape=true>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Escape closes this"</AlertDialogTitle>
                        <AlertDialogDescription>"Press Escape to dismiss it."</AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction>"OK"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
                <AlertDialog open=backdrop_open overlay_closable=true>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"The backdrop closes this"</AlertDialogTitle>
                        <AlertDialogDescription>"Click outside the dialog to dismiss it."</AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction>"OK"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
            </Section>
            <AlertDialogAsyncExamples />
        </Page>
    }
}

/// An action that has to finish first: it keeps the dialog open, shows a loading state, and
/// reports an error in the footer.
#[component]
fn AlertDialogAsyncExamples() -> impl IntoView {
    let open = RwSignal::new(false);
    let loading = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let attempts = RwSignal::new(0u32);
    let done = RwSignal::new(0u32);

    let disabled_open = RwSignal::new(false);
    let host_wanted = RwSignal::new(false);

    view! {
        <Section
            title="Async action: stays open, loading, error"
            description="AlertDialogAction with close_on_click=false keeps the dialog open after on_click. Drive loading from your own state (spinner, clicks ignored) and disable Cancel while it runs. The first attempt here fails, so the error shows in the footer; the second succeeds and closes the dialog."
        >
            <Row>
                <Button
                    attr:id="async-open"
                    variant=ButtonVariant::Destructive
                    on_click=move |_| {
                        error.set(None);
                        attempts.set(0);
                        open.set(true);
                    }
                >
                    "Delete project"
                </Button>
                <span id="async-readout" class="text-sm text-muted-foreground">
                    "attempts " {move || attempts.get()} " | deleted " {move || done.get()}
                </span>
            </Row>
            <AlertDialog open=open>
                <AlertDialogHeader>
                    <AlertDialogTitle>"Delete this project?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "The request runs for about a second. The dialog stays open until it succeeds."
                    </AlertDialogDescription>
                </AlertDialogHeader>
                {move || {
                    error
                        .get()
                        .map(|message| {
                            view! {
                                <p id="async-error" role="alert" class="text-sm text-destructive">
                                    {message}
                                </p>
                            }
                        })
                }}
                <AlertDialogFooter>
                    <AlertDialogCancel attr:id="async-cancel" disabled=loading>
                        "Cancel"
                    </AlertDialogCancel>
                    <AlertDialogAction
                        attr:id="async-action"
                        variant=ButtonVariant::Destructive
                        close_on_click=false
                        loading=loading
                        on_click=move |_| {
                            error.set(None);
                            loading.set(true);
                            attempts.update(|n| *n += 1);
                            set_timeout(
                                move || {
                                    loading.set(false);
                                    if attempts.get_untracked() < 2 {
                                        error.set(Some("The server did not respond. Try again.".into()));
                                    } else {
                                        done.update(|n| *n += 1);
                                        open.set(false);
                                    }
                                },
                                std::time::Duration::from_millis(900),
                            );
                        }
                    >
                        "Delete"
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialog>
        </Section>

        <Section
            title="Disabled action and cancel"
            description="`disabled` greys out and blocks either button, for a choice that is not available yet."
        >
            <Row>
                <Button variant=ButtonVariant::Outline on_click=move |_| disabled_open.set(true)>
                    "Open"
                </Button>
            </Row>
            <AlertDialog open=disabled_open close_on_escape=true>
                <AlertDialogHeader>
                    <AlertDialogTitle>"Cannot confirm yet"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "Delete is disabled. Escape still closes this dialog (close_on_escape)."
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                    <AlertDialogAction attr:id="disabled-action" variant=ButtonVariant::Destructive disabled=true>
                        "Delete"
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialog>
        </Section>

        <Section
            title="With DialogHost"
            description="An alert dialog built only while needed. It still animates out because DialogHost removes it after the exit; on_close tells the app it closed."
        >
            <Row>
                <Button
                    attr:id="alert-host-open"
                    variant=ButtonVariant::Outline
                    on_click=move |_| host_wanted.set(true)
                >
                    "Open hosted alert"
                </Button>
            </Row>
            <DialogHost
                when=Signal::derive(move || host_wanted.get())
                on_close=move |_| host_wanted.set(false)
                let:open
            >
                <AlertDialog open=open>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Hosted alert"</AlertDialogTitle>
                        <AlertDialogDescription>"Mounted only while it is wanted."</AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction>"OK"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialog>
            </DialogHost>
        </Section>
    }
}
