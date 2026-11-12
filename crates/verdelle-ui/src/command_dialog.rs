use crate::native_dialog::{
    DIALOG_EXIT_MS, notify_open_change, on_cancel, on_close as on_close_handler, sync_dialog,
};
use crate::primitives::core::id::use_id;
use leptos::{ev, html, prelude::*};

/// A [`crate::Command`] in a modal dialog: the command palette. Put a `Command` in it.
///
/// Built like [`crate::Dialog`], on the native `<dialog>` opened with `showModal()` (focus trap,
/// Escape, inert page, top layer), with no padding and no close button. `open` is the single
/// source of truth and is set to `false` when the dialog is dismissed. The contents are only
/// built while it is open, so the search starts empty every time. `title` and `description` are
/// read by screen readers and not shown. Opening a palette from a key is up to the app (see the
/// gallery's Command page for a Ctrl or Cmd K listener).
///
/// It sits a third of the way down the viewport (as shadcn's), 384px wide unless `width` is set.
#[component]
pub fn CommandDialog(
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
    #[prop(into, default = "Command palette".to_string())] title: String,
    #[prop(into, default = "Search for a command to run...".to_string())] description: String,
    /// Width in pixels. Defaults to 448.
    #[prop(optional)]
    width: Option<u32>,
    /// Close when the backdrop is clicked. On by default.
    #[prop(default = true)]
    overlay_closable: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let dialog = NodeRef::<html::Dialog>::new();
    notify_open_change(open, on_open_change, on_close);
    let description_id = format!("{}-description", use_id(None).get_untracked());
    let described_by = description_id.clone();
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let children = StoredValue::new(children);

    // The palette is built a moment after the dialog opens; focus its input then.
    sync_dialog(dialog, open, DIALOG_EXIT_MS, move |_| {
        request_animation_frame(move || {
            use leptos::wasm_bindgen::JsCast;
            let input = dialog
                .get_untracked()
                .and_then(|d| d.query_selector("input").ok().flatten())
                .and_then(|i| i.dyn_into::<leptos::web_sys::HtmlElement>().ok());
            if let Some(input) = input {
                let _ = input.focus();
            }
        });
    });

    // The palette stays built until the exit animation has finished: `open` goes false at once,
    // the dialog element closes a moment later.
    let built = RwSignal::new(false);
    Effect::new(move |_| {
        if open.get() {
            built.set(true);
        }
    });

    view! {
        <dialog
            node_ref=dialog
            class=format!("vdl-dialog vdl-command-dialog {class}")
            aria-label=title.get_value()
            aria-describedby=described_by
            style=width.map(|w| format!("--vdl-dialog-width:{w}px"))
            on:cancel=on_cancel(open, true)
            on:close=move |ev| {
                on_close_handler(dialog, open)(ev);
                built.set(false);
            }
            on:click=move |ev: ev::MouseEvent| {
                if overlay_closable && ev.target() == ev.current_target() {
                    open.set(false);
                }
            }
        >
            <p id=description_id class="vdl-sr-only">{description.get_value()}</p>
            <Show when=move || built.get()>{children.with_value(|children| children())}</Show>
        </dialog>
    }
}
