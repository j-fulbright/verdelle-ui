use crate::sidebar_context::use_sidebar;
use crate::{Sheet, Side, SidebarCollapsible, SidebarSide, SidebarVariant};
use leptos::prelude::*;

/// The sidebar, as shadcn/ui's `Sidebar`: a panel of navigation next to the page content. Put it
/// in a [`crate::SidebarProvider`], before the [`crate::SidebarInset`], and fill it with
/// [`crate::SidebarHeader`], [`crate::SidebarContent`] (groups of menus) and [`crate::SidebarFooter`].
///
/// - `side`: `Left` (default) or `Right`.
/// - `variant`: `Sidebar` (default), `Floating` or `Inset`; see [`SidebarVariant`].
/// - `collapsible`: what collapsing does, see [`SidebarCollapsible`]. `Offcanvas` is the default.
/// - `label`: the accessible name of the small-screen sheet.
///
/// Below 768px (`use_sidebar().is_mobile`) the sidebar is drawn as a [`crate::Sheet`], 18rem wide,
/// opened by the trigger or the shortcut; with `collapsible=None` it stays a panel. On larger
/// screens the panel is `position: fixed`, with an empty gap of the same width in the layout that
/// animates when it collapses (`contained` on the provider pins it to the provider's box instead).
/// `class` lands on the fixed container.
///
/// The root carries `data-state`, `data-collapsible` (empty while expanded), `data-variant` and
/// `data-side`, which the styles of everything inside read.
#[component]
pub fn Sidebar(
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(default = "Sidebar".to_string(), into)] label: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_sidebar();
    let children = StoredValue::new(children);
    let sheet_side = match side {
        SidebarSide::Left => Side::Left,
        SidebarSide::Right => Side::Right,
    };

    move || {
        if collapsible == SidebarCollapsible::None {
            return view! {
                <div class=format!("vdl-sidebar-static {class}") data-sidebar="sidebar">
                    {children.with_value(|children| children())}
                </div>
            }
            .into_any();
        }
        if ctx.is_mobile.get() {
            return view! {
                <Sheet
                    open=ctx.open_mobile
                    label=label.clone()
                    side=sheet_side
                    show_close_button=false
                    class="vdl-sidebar__sheet"
                >
                    <div class="vdl-sidebar__sheet-inner" data-sidebar="sidebar" data-mobile="true">
                        {children.with_value(|children| children())}
                    </div>
                </Sheet>
            }
            .into_any();
        }
        view! {
            <div
                class="vdl-sidebar"
                data-state=move || ctx.state()
                data-collapsible=move || {
                    if ctx.expanded() { "" } else { collapsible.as_str() }
                }
                data-variant=variant.as_str()
                data-side=side.as_str()
            >
                <div class="vdl-sidebar__gap"></div>
                <div class=format!("vdl-sidebar__container {class}")>
                    <div class="vdl-sidebar__inner" data-sidebar="sidebar">
                        {children.with_value(|children| children())}
                    </div>
                </div>
            </div>
        }
        .into_any()
    }
}
