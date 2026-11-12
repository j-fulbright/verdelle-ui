use crate::Size;
use crate::native_dialog::{
    DIALOG_EXIT_MS, notify_open_change, on_cancel, on_close as on_close_handler, sync_dialog,
};
use leptos::wasm_bindgen::JsCast;
use leptos::{context::Provider, ev, html, prelude::*};

/// What the parts of an alert dialog share: the open flag (the buttons close it) and the
/// ids that `aria-labelledby` / `aria-describedby` point at.
#[derive(Clone)]
pub(crate) struct AlertDialogContext {
    pub open: RwSignal<bool>,
    pub title_id: String,
    pub description_id: String,
}

/// A modal that interrupts the user for a decision, styled as shadcn/ui's Alert Dialog.
///
/// Native `<dialog>` opened with `showModal()` (the same choice as [`crate::Dialog`]),
/// with `role="alertdialog"`. The browser gives the focus trap, the inert page, top-layer
/// stacking and returns focus to what opened it. We add what makes it an alert dialog:
/// initial focus goes to [`crate::AlertDialogCancel`] (the safe choice), Escape and a
/// backdrop click do not dismiss it, and there is no close button. `close_on_escape` and
/// `overlay_closable` switch the first two back on.
///
/// `open` is the single source of truth, as for Dialog. Compose with
/// [`crate::AlertDialogHeader`] (holding [`crate::AlertDialogMedia`],
/// [`crate::AlertDialogTitle`] and [`crate::AlertDialogDescription`]) and
/// [`crate::AlertDialogFooter`] (holding [`crate::AlertDialogCancel`] and
/// [`crate::AlertDialogAction`]). Title and Description are expected: the dialog is
/// labelled and described by them.
///
/// `size` is `Size::Small` (a 320px dialog with centred text and a two-column footer) or
/// the default (up to 384px wide, text at the start); other sizes draw as the default.
/// Closing plays a short exit animation before the element closes. Not provided: a trigger
/// part (open it from any button by setting `open`).
#[component]
pub fn AlertDialog(
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
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Close on Escape. Off by default.
    #[prop(optional)]
    close_on_escape: bool,
    /// Close when the backdrop is clicked. Off by default.
    #[prop(optional)]
    overlay_closable: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<html::Dialog>::new();
    notify_open_change(open, on_open_change, on_close);
    let ids = crate::primitives::core::id::use_id(None).get_untracked();
    let ctx = AlertDialogContext {
        open,
        title_id: format!("vdl-alert-dialog-title-{ids}"),
        description_id: format!("vdl-alert-dialog-description-{ids}"),
    };
    let (title_id, description_id) = (ctx.title_id.clone(), ctx.description_id.clone());

    // Start on Cancel, not on the first button in the DOM.
    sync_dialog(dialog, open, DIALOG_EXIT_MS, |el| {
        if let Ok(Some(cancel)) = el.query_selector(".vdl-alert-dialog__cancel")
            && let Ok(cancel) = cancel.dyn_into::<leptos::web_sys::HtmlElement>()
        {
            let _ = cancel.focus();
        }
    });

    view! {
        <Provider value=ctx>
        <dialog
            node_ref=dialog
            class=format!("vdl-alert-dialog {class}")
            role="alertdialog"
            aria-labelledby=title_id
            aria-describedby=description_id
            data-size=size.as_str()
            // Escape fires `cancel`; cancelling it keeps the dialog open. The keydown guard
            // covers Chrome closing on a second Escape that no interaction separated.
            on:cancel=on_cancel(open, close_on_escape)
            on:keydown=move |ev: ev::KeyboardEvent| {
                if !close_on_escape && ev.key() == "Escape" {
                    ev.prevent_default();
                }
            }
            on:close=on_close_handler(dialog, open)
            on:click=move |ev: ev::MouseEvent| {
                if overlay_closable && ev.target() == ev.current_target() {
                    open.set(false);
                }
            }
        >
            <div class="vdl-alert-dialog__surface">{children()}</div>
        </dialog>
        </Provider>
    }
}
