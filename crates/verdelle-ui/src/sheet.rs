use crate::native_dialog::{
    SHEET_EXIT_MS, notify_open_change, on_cancel, on_close as on_close_handler, sync_dialog,
};
use crate::{Button, ButtonVariant, Side, Size, icons};
use leptos::{ev, html, prelude::*};

/// Tells [`crate::SheetClose`] which sheet it belongs to.
#[derive(Clone, Copy)]
pub(crate) struct SheetOpen(pub RwSignal<bool>);

/// A panel that slides in from an edge of the screen over the page, styled as shadcn/ui's
/// Sheet. Use it for a form, a menu or details that should not leave the page. It is
/// the same idea as [`crate::Dialog`] and is built the same way: a native `<dialog>` opened with
/// `showModal()`, so the browser provides the focus trap, Escape, the inert page behind it,
/// top-layer stacking and the `dialog` role, and returns focus to what opened it.
///
/// `open` is the single source of truth: set it to show or hide the sheet, and it is set to
/// `false` when the user dismisses it. `label` is the accessible name. `side` is the edge it
/// comes from (right by default, as shadcn). Unlike the dialog it has an exit animation: closing
/// plays it before the dialog element is closed. The panel moves 2.5rem and fades, as shadcn's.
///
/// Compose the body with [`crate::SheetHeader`], [`crate::SheetTitle`],
/// [`crate::SheetDescription`] and [`crate::SheetFooter`]; anything else goes in between as it
/// is. [`crate::SheetClose`] makes any button close it. Ways to dismiss it, each switchable:
/// the corner button (`show_close_button`), a click on the backdrop (`overlay_closable`) and
/// Escape (`close_on_escape`).
///
/// The sheet is 75% of the screen wide on the left and right (at most 24rem from 640px up),
/// full height; on the top and bottom it is full width and as tall as its content. Override
/// either with `class`.
#[component]
pub fn Sheet(
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
    #[prop(default = Side::Right)] side: Side,
    /// Close when the backdrop is clicked. On by default.
    #[prop(default = true)]
    overlay_closable: bool,
    /// Close on Escape. On by default.
    #[prop(default = true)]
    close_on_escape: bool,
    /// Show an X button in the corner that closes the sheet. On by default, as shadcn.
    #[prop(default = true)]
    show_close_button: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<html::Dialog>::new();
    notify_open_change(open, on_open_change, on_close);
    provide_context(SheetOpen(open));

    // Keep the element in step with `open`. Closing first marks the element `data-closing` (the
    // CSS plays the exit animation), then closes it when that has run.
    sync_dialog(dialog, open, SHEET_EXIT_MS, |_| {});

    view! {
        <dialog
            node_ref=dialog
            class=format!("vdl-sheet {class}")
            data-side=side.as_str()
            aria-label=label
            // Fires for Escape. Always cancelled: closing goes through `open`, so the exit
            // animation plays.
            on:cancel=on_cancel(open, close_on_escape)
            // Fires after any close, including the one above.
            on:close=on_close_handler(dialog, open)
            // The sheet has no padding of its own, so a click whose target is the dialog
            // itself landed on the backdrop.
            on:click=move |ev: ev::MouseEvent| {
                if overlay_closable && ev.target() == ev.current_target() {
                    open.set(false);
                }
            }
        >
            {children()}
            {show_close_button
                .then(|| {
                    view! {
                        <Button
                            variant=ButtonVariant::Ghost
                            size=Size::Small
                            icon_only=true
                            class="vdl-sheet__close"
                            aria_label="Close"
                            on_click=move |_| open.set(false)
                        >
                            {icons::x()}
                        </Button>
                    }
                })}
        </dialog>
    }
}
