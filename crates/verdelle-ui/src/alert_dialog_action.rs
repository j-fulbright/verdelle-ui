use crate::alert_dialog::AlertDialogContext;
use crate::{Button, ButtonVariant, Size};
use leptos::{ev, prelude::*};

/// The confirming choice: runs `on_click`, then closes the dialog. A [`Button`], so pass
/// `variant=ButtonVariant::Destructive` for a destructive action.
///
/// For an action that must finish first (an async call that may fail), pass
/// `close_on_click=false`: the dialog stays open after `on_click`, and you close it (or show an
/// error) yourself when the work is done. Drive `loading` from your own state: a loading action
/// shows a spinner and ignores clicks, so it cannot be fired twice. `disabled` greys it out.
#[component]
pub fn AlertDialogAction(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional)]
    variant: ButtonVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Close the dialog after `on_click`. On by default; turn it off to keep the dialog open
    /// while an async action runs (or shows an error) and close it yourself with `open.set(false)`.
    #[prop(default = true)]
    close_on_click: bool,
    /// Show a spinner and ignore clicks (the action stays focusable), as `Button`'s `loading`.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// The content.
    children: Children,
) -> impl IntoView {
    let open = expect_context::<AlertDialogContext>().open;
    view! {
        <Button
            variant=variant
            size=size
            class=format!("vdl-alert-dialog__action {class}")
            loading=loading
            disabled=disabled
            on_click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
                if close_on_click {
                    open.set(false);
                }
            }
        >
            {children()}
        </Button>
    }
}
