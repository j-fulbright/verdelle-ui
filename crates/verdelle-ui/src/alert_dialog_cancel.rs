use crate::alert_dialog::AlertDialogContext;
use crate::{Button, ButtonVariant, Size};
use leptos::{ev, prelude::*};

/// The safe choice: closes the dialog without acting. The dialog puts focus here when it
/// opens. An outline [`Button`] by default, as shadcn.
#[component]
pub fn AlertDialogCancel(
    /// Visual variant: see `ButtonVariant`.
    #[prop(default = ButtonVariant::Outline)]
    variant: ButtonVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Runs before the dialog closes.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Disables it: no focus or input, and it is styled as disabled. Disable it while an
    /// action is running, so the dialog cannot be cancelled halfway.
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
            class=format!("vdl-alert-dialog__cancel {class}")
            disabled=disabled
            on_click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
                open.set(false);
            }
        >
            {children()}
        </Button>
    }
}
