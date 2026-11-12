use leptos::prelude::*;

/// How a [`Checkbox`] gets its checked state. You do not build this: pass `checked=` any of
/// these and it converts.
///
/// - an `RwSignal<bool>`: two-way, the checkbox writes it (the form that always worked);
/// - a `Signal<bool>`, `ReadSignal<bool>`, `Memo<bool>` or a plain `bool`: controlled, the
///   checkbox only reads it and reports clicks through `on_checked_change`; whatever the
///   handler does decides the next state, and the box snaps back if the value did not change.
///   This needs no signal per row: pass `Signal::derive(move || selected.get().contains(&id))`;
/// - nothing: uncontrolled, it keeps its own state (start it with `default_checked`).
#[derive(Clone, Copy, Default)]
pub enum Checked {
    /// The checkbox manages itself.
    #[default]
    Uncontrolled,
    /// Two-way binding to a signal.
    Writable(RwSignal<bool>),
    /// Controlled by a value that something else updates.
    Controlled(Signal<bool>),
}

impl From<RwSignal<bool>> for Checked {
    fn from(signal: RwSignal<bool>) -> Self {
        Checked::Writable(signal)
    }
}
impl From<Signal<bool>> for Checked {
    fn from(signal: Signal<bool>) -> Self {
        Checked::Controlled(signal)
    }
}
impl From<ReadSignal<bool>> for Checked {
    fn from(signal: ReadSignal<bool>) -> Self {
        Checked::Controlled(signal.into())
    }
}
impl From<Memo<bool>> for Checked {
    fn from(signal: Memo<bool>) -> Self {
        Checked::Controlled(signal.into())
    }
}
impl From<bool> for Checked {
    fn from(value: bool) -> Self {
        Checked::Controlled(Signal::stored(value))
    }
}

/// A checkbox, styled as shadcn/ui's Checkbox.
///
/// It is a native `<input type="checkbox">` drawn with CSS, so it keeps the browser's
/// keyboard handling, form participation and accessibility. (shadcn's is a button with
/// `role="checkbox"`; the behaviour is the same.) Name it with a [`crate::Label`] pointing at
/// its `id`, or with `aria_label`.
///
/// `checked` takes an `RwSignal<bool>` (two-way, as before), or a read-only value (`Signal`,
/// `ReadSignal`, `Memo` or `bool`) with `on_checked_change` for controlled use, which lets a
/// list of rows share one piece of state instead of a signal each (see [`Checked`]). Leave it
/// out for an uncontrolled box starting at `default_checked`. `on_checked_change` is called with
/// the new state on every user toggle in all three modes.
#[component]
pub fn Checkbox(
    /// The checked state: an `RwSignal<bool>` (two-way) or a read-only value (controlled).
    #[prop(into, optional)]
    checked: Checked,
    /// Initial state of an uncontrolled checkbox (no `checked`).
    #[prop(optional)]
    default_checked: bool,
    /// Called with the new state whenever the user toggles it.
    #[prop(optional, into)]
    on_checked_change: Option<Callback<bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Form field name; the value is submitted under it.
    #[prop(optional, into)]
    name: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Mark the checkbox `aria-invalid` and draw the destructive border.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    /// The mixed state, for a "select all" box when only some of its rows are selected. It
    /// draws a dash and is announced as mixed. The browser clears it when the box is clicked;
    /// it comes back if this signal is still true afterwards, so set it from your own state.
    #[prop(optional, into)]
    indeterminate: Signal<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let internal = RwSignal::new(default_checked);
    let current: Signal<bool> = match checked {
        Checked::Uncontrolled => internal.into(),
        Checked::Writable(signal) => signal.into(),
        Checked::Controlled(signal) => signal,
    };

    view! {
        <input
            type="checkbox"
            class=format!("vdl-checkbox {class}")
            id=id
            name=name
            aria-label=aria_label
            aria-invalid=move || invalid.get().then_some("true")
            aria-describedby=aria_describedby
            disabled=move || disabled.get()
            prop:checked=move || current.get()
            prop:indeterminate=move || indeterminate.get()
            on:change=move |ev| {
                let next = event_target_checked(&ev);
                match checked {
                    Checked::Uncontrolled => internal.set(next),
                    Checked::Writable(signal) => signal.set(next),
                    Checked::Controlled(_) => {}
                }
                if let Some(cb) = on_checked_change {
                    cb.run(next);
                }
                let el = event_target::<leptos::web_sys::HtmlInputElement>(&ev);
                // A controlled box shows what its value says, not what the click did.
                el.set_checked(current.get_untracked());
                el.set_indeterminate(indeterminate.get_untracked());
            }
        />
    }
}
