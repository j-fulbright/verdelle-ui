use crate::sheet::SheetOpen;
use crate::{Button, ButtonVariant, Size};
use leptos::prelude::*;

/// A button that closes the [`crate::Sheet`] it is inside (use it in the footer). It is a
/// [`crate::Button`]: pass `variant` and `size` as you would there. Outside a sheet it does
/// nothing.
#[component]
pub fn SheetClose(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional, into)]
    variant: Signal<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let sheet = use_context::<SheetOpen>();
    view! {
        <Button
            variant=variant
            size=size
            class=class
            on_click=move |_| {
                if let Some(SheetOpen(open)) = sheet {
                    open.set(false);
                }
            }
        >
            {children()}
        </Button>
    }
}
