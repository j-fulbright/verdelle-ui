use crate::sidebar_context::use_sidebar;
use crate::{Button, ButtonVariant, Size, icons};
use leptos::{ev, prelude::*};

/// A button that toggles the sidebar ([`crate::SidebarContext::toggle`]): the panel on a large
/// screen, the sheet on a small one. A ghost icon button with a panel icon by default; pass
/// children to replace the icon. `on_click` runs first.
#[component]
pub fn SidebarTrigger(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(default = "Toggle Sidebar".to_string(), into)]
    aria_label: String,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_sidebar();
    view! {
        <Button
            variant=ButtonVariant::Ghost
            size=Size::Small
            icon_only=true
            class=format!("vdl-sidebar__trigger {class}")
            aria_label=aria_label
            on_click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
                ctx.toggle();
            }
        >
            {match children {
                Some(children) => children().into_any(),
                None => icons::panel_left().into_any(),
            }}
        </Button>
    }
}
