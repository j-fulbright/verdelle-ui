use crate::native_dialog::notify_open_change;
use leptos::{leptos_dom::helpers::set_timeout, prelude::*};
use std::time::Duration;

/// Mounts a dialog only while it is needed, and still lets it animate out.
///
/// Every dialog in Verdelle ([`crate::Dialog`], [`crate::AlertDialog`], [`crate::Sheet`],
/// [`crate::Drawer`], [`crate::CommandDialog`]) plays an exit animation after `open` turns false.
/// If an app removes the dialog from the page at the same moment (`{move ||
/// selected.get().map(|row| view! { <Dialog .../> })}`, or a `<Show>` around it), the element is
/// gone before it can animate and the dialog just vanishes. `DialogHost` is the fix: it builds the
/// dialog when `when` turns true, hands the children an `open` signal that is already true, sets
/// it false when `when` turns false, and removes the dialog only after the exit animation has
/// run (`exit_ms`, 350ms by default, longer than any of the dialogs' exits).
///
/// If the user dismisses the dialog (Escape, backdrop, a button), `open` goes false by itself:
/// the dialog animates out and is removed, and `on_close` runs so the app can reset whatever
/// `when` reads (for example `selected.set(None)`). Set `when` to true again to reopen.
///
/// ```ignore
/// let selected = RwSignal::new(None::<Row>);
/// view! {
///     <DialogHost
///         when=Signal::derive(move || selected.get().is_some())
///         on_close=move |_| selected.set(None)
///         let:open
///     >
///         <Dialog open=open label="Row details">...</Dialog>
///     </DialogHost>
/// }
/// ```
///
/// The children run again each time the dialog is built, not while it animates out. They are
/// still on screen during the exit, so do not read state that `when` clears (a
/// `selected.get().unwrap()` would panic): keep the data until `on_close`, or copy it first.
#[component]
pub fn DialogHost<F, V>(
    /// Whether the dialog should be shown. Turning it false closes the dialog with its exit
    /// animation and then removes it.
    #[prop(into)]
    when: Signal<bool>,
    /// Milliseconds to keep the dialog mounted after it closes. The default, 350, covers the
    /// longest exit animation (the drawer's 300ms).
    #[prop(default = 350)]
    exit_ms: u64,
    /// Called when the dialog closes, however that happened: after the user dismissed it, or after
    /// `when` turned false.
    #[prop(optional, into)]
    on_close: Option<Callback<()>>,
    /// Builds the dialog from the `open` signal to pass to it.
    children: F,
) -> impl IntoView
where
    F: Fn(RwSignal<bool>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let open = RwSignal::new(when.get_untracked());
    let mounted = RwSignal::new(when.get_untracked());

    // `when` drives `open`; the dialog may also close itself.
    Effect::new(move |_| {
        let want = when.get();
        if want {
            mounted.set(true);
        }
        if open.get_untracked() != want {
            open.set(want);
        }
    });

    // Remove the dialog once its exit has had time to run, unless it was reopened.
    Effect::new(move |_| {
        if !open.get() && mounted.get_untracked() {
            set_timeout(
                move || {
                    if !open.try_get_untracked().unwrap_or(true) {
                        mounted.try_set(false);
                    }
                },
                Duration::from_millis(exit_ms),
            );
        }
    });

    notify_open_change(open, None, on_close);

    move || mounted.get().then(|| children(open))
}
