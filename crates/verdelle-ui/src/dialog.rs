use crate::native_dialog::{
    DIALOG_EXIT_MS, notify_open_change, on_cancel, on_close as on_close_handler, sync_dialog,
};
use crate::{Button, ButtonVariant, Size};
use crate::{Icon, IconName};
use leptos::{ev, html, prelude::*};

/// A modal dialog, styled as shadcn/ui's Dialog.
///
/// Built on the native `<dialog>` element opened with `showModal()`, so the
/// browser provides the focus trap, Escape handling, an inert page behind it,
/// top-layer stacking and the `dialog` role. `open` is the single source of
/// truth: set it to show or hide the dialog, and it is set to `false` when the
/// user dismisses it (Escape, backdrop click, or a `<form method="dialog">`).
///
/// `label` is the accessible name. Compose the body with [`crate::DialogHeader`],
/// [`crate::DialogTitle`], [`crate::DialogDescription`], [`crate::DialogContent`]
/// and [`crate::DialogFooter`].
///
/// Ways to dismiss it, each switchable: the close button (`show_close_button`), a click
/// on the backdrop (`overlay_closable`) and Escape (`close_on_escape`). A dialog that
/// must be answered turns all three off and offers its own buttons.
///
/// Closing plays a short exit animation (fade and zoom out) before the element closes; a
/// `<form method="dialog">` submit is closed by the browser at once and skips it. Not provided:
/// stacked layers. Put [`crate::Button`]s in a
/// `DialogFooter` for the actions. The dialog is 384px wide unless `width` is set,
/// never wider than the viewport minus 2rem. It is centred with `top`/`left` and a
/// transform, not margins, so a parent's margin utilities (Tailwind's `space-y-*`)
/// cannot push it off-centre.
#[component]
pub fn Dialog(
    /// The open state, read and written by the component.
    open: RwSignal<bool>,
    /// Called with the new state whenever it opens or closes (after the first render), so you
    /// need not watch `open` in an `Effect`. It fires when the change is requested.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    /// Called when it closes, however that happened (Escape, backdrop, a button, `open.set(false)`).
    /// It fires when the close is requested, at the start of the exit animation.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    #[prop(into)] label: String,
    /// Width in pixels. Defaults to 448.
    #[prop(optional)]
    width: Option<u32>,
    /// Close when the backdrop is clicked. On by default.
    #[prop(default = true)]
    overlay_closable: bool,
    /// Close on Escape. On by default.
    #[prop(default = true)]
    close_on_escape: bool,
    /// Show an X button in the corner that closes the dialog. On by default, as shadcn.
    #[prop(default = true)]
    show_close_button: bool,
    /// The content.
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<html::Dialog>::new();
    notify_open_change(open, on_open_change, on_close);

    // Keep the element in step with `open`; closing plays the exit animation first.
    sync_dialog(dialog, open, DIALOG_EXIT_MS, |_| {});

    view! {
        <dialog
            node_ref=dialog
            class="vdl-dialog"
            aria-label=label
            style=width.map(|w| format!("--vdl-dialog-width:{w}px"))
            // Fires for Escape. Always cancelled: closing goes through `open`, so the exit
            // animation plays.
            on:cancel=on_cancel(open, close_on_escape)
            // Fires after any close, including the one above.
            on:close=on_close_handler(dialog, open)
            // The dialog has no padding of its own, so a click whose target is
            // the dialog itself landed on the backdrop.
            on:click=move |ev: ev::MouseEvent| {
                if overlay_closable && ev.target() == ev.current_target() {
                    open.set(false);
                }
            }
        >
            <div class="vdl-dialog__surface">{children()}</div>
            {show_close_button
                .then(|| {
                    view! {
                        <Button
                            variant=ButtonVariant::Ghost
                            size=Size::Small
                            icon_only=true
                            class="vdl-dialog__close"
                            aria_label="Close"
                            on_click=move |_| open.set(false)
                        >
                            <Icon name=IconName::X />
                        </Button>
                    }
                })}
        </dialog>
    }
}
