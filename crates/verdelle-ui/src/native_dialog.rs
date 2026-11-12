//! Shared plumbing for the components built on a native `<dialog>` (Dialog, Alert Dialog,
//! Command Dialog, Sheet): keep the element in step with an `open` signal, and play an exit
//! animation before closing it.
//!
//! The browser closes a modal `<dialog>` at once (on Escape, `close()` or a form with
//! `method="dialog"`), which leaves no time to animate. So closing goes through `open`: the
//! element is first marked `data-closing` (the CSS plays the exit animation and ignores the
//! pointer meanwhile) and `close()` is called when that has run. Escape's `cancel` event is
//! always cancelled for the same reason and turned into `open.set(false)`.
//!
//! If the element has no animation while closing (reduced motion, `data-vdl-motion="off"`, or a
//! test that turns animation off) it closes straight away. A fallback timer closes it even if
//! `animationend` never arrives.

use leptos::{
    ev, html,
    leptos_dom::helpers::{set_timeout, window},
    prelude::*,
};
use std::time::Duration;

/// How long to wait for the exit animation of a dialog (100ms in the CSS) before closing it.
pub(crate) const DIALOG_EXIT_MS: u64 = 110;
/// The same for a sheet, whose exit is 200ms.
pub(crate) const SHEET_EXIT_MS: u64 = 210;
/// The same for a drawer, whose exit is 300ms.
pub(crate) const DRAWER_EXIT_MS: u64 = 310;

/// Whether `el` currently plays a CSS animation.
fn is_animating(el: &leptos::web_sys::HtmlDialogElement) -> bool {
    window()
        .get_computed_style(el)
        .ok()
        .flatten()
        .and_then(|style| style.get_property_value("animation-name").ok())
        .is_some_and(|name| !name.is_empty() && name != "none")
}

/// Keep `dialog` in step with `open`. `exit_ms` is how long the CSS exit animation runs (plus a
/// frame). `on_opened` runs right after `showModal()`.
///
/// Effects do not run during SSR and the node does not exist until mount, so this is
/// browser-only by nature.
pub(crate) fn sync_dialog(
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<bool>,
    exit_ms: u64,
    on_opened: impl Fn(&leptos::web_sys::HtmlDialogElement) + 'static,
) {
    sync_dialog_with(dialog, open, exit_ms, true, on_opened);
}

/// [`sync_dialog`] with a choice of `modal`: `false` opens with `show()` (no backdrop, no focus
/// trap, the page stays interactive), as the non-modal Drawer needs.
pub(crate) fn sync_dialog_with(
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<bool>,
    exit_ms: u64,
    modal: bool,
    on_opened: impl Fn(&leptos::web_sys::HtmlDialogElement) + 'static,
) {
    // Bumped on every close request so a stale timer from an earlier close cannot cut a later
    // exit short.
    let generation = StoredValue::new(0u64);

    Effect::new(move |_| {
        let want_open = open.get();
        let Some(el) = dialog.get() else { return };
        if want_open {
            // Reopened during the exit: cancel it and carry on.
            let _ = el.remove_attribute("data-closing");
            generation.update_value(|g| *g += 1);
            if !el.open() {
                if modal {
                    let _ = el.show_modal();
                } else {
                    el.show();
                }
                on_opened(&el);
            }
        } else if el.open() && !el.has_attribute("data-closing") {
            let _ = el.set_attribute("data-closing", "");
            if !is_animating(&el) {
                let _ = el.remove_attribute("data-closing");
                el.close();
                return;
            }
            generation.update_value(|g| *g += 1);
            let mine = generation.get_value();
            set_timeout(
                move || {
                    // Reopened or closed again in the meantime: that request owns the element.
                    let current = generation.try_get_value().unwrap_or(mine);
                    if current == mine && !open.try_get_untracked().unwrap_or(true) {
                        el.close();
                    }
                },
                Duration::from_millis(exit_ms),
            );
        }
    });
}

/// The `cancel` handler (Escape). Always cancelled, so the exit animation plays: closing goes
/// through `open`. With `close_on_escape` off, Escape does nothing.
pub(crate) fn on_cancel(
    open: RwSignal<bool>,
    close_on_escape: bool,
) -> impl Fn(ev::Event) + 'static {
    move |ev: ev::Event| {
        ev.prevent_default();
        if close_on_escape {
            open.set(false);
        }
    }
}

/// The `close` handler: fires after any close, including `close()` above and a
/// `<form method="dialog">`.
pub(crate) fn on_close(
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<bool>,
) -> impl Fn(ev::Event) + 'static {
    move |_| {
        if let Some(el) = dialog.get_untracked() {
            let _ = el.remove_attribute("data-closing");
        }
        open.set(false);
    }
}

/// Call `on_open_change(next)` whenever `open` changes (not for its first value), and
/// `on_close(())` when it turns false. They fire when the change is requested, so a closing
/// dialog reports at the start of its exit animation, not after it. This lets an app react
/// to a dialog being dismissed (Escape, backdrop, a button) without an `Effect` of its own.
pub(crate) fn notify_open_change(
    open: RwSignal<bool>,
    on_open_change: Option<Callback<bool>>,
    on_close: Option<Callback<()>>,
) {
    if on_open_change.is_none() && on_close.is_none() {
        return;
    }
    let previous = StoredValue::new(open.get_untracked());
    Effect::new(move |_| {
        let now = open.get();
        if previous.get_value() == now {
            return;
        }
        previous.set_value(now);
        if let Some(cb) = on_open_change {
            cb.run(now);
        }
        if !now && let Some(cb) = on_close {
            cb.run(());
        }
    });
}
