use crate::drawer::DrawerOpen;
use crate::{Button, ButtonVariant, Size};
use leptos::prelude::*;

/// A button that closes the [`crate::Drawer`] it is inside (use it in the footer). It is a
/// [`crate::Button`]: pass `variant` and `size` as you would there. Outside a drawer it does
/// nothing.
#[component]
pub fn DrawerClose(
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
    let drawer = use_context::<DrawerOpen>();
    view! {
        <Button
            variant=variant
            size=size
            class=class
            on_click=move |_| {
                if let Some(DrawerOpen(open)) = drawer {
                    open.set(false);
                }
            }
        >
            {children()}
        </Button>
    }
}
