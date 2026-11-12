use crate::{
    AppNav, ShellContent, ShellLayout, SidebarCollapsible, SidebarShell, SidebarShellProps,
    TopBarShell, TopBarShellProps,
};
use leptos::prelude::*;

/// The page frame of an application: one [`AppNav`], drawn by whichever shell `layout` names.
/// Changing `layout` (a signal, so it can be a user setting) swaps the shell and leaves the
/// menu, the slots and the page content as they are.
///
/// Props are those of [`crate::SidebarShell`] and [`crate::TopBarShell`]; `collapsible` only
/// applies to the sidebar. Switching shells rebuilds the page content, so keep state you want
/// to survive in signals above the shell.
#[component]
pub fn AppShell(
    nav: AppNav,
    #[prop(into)] current_path: Signal<String>,
    #[prop(optional, into)] layout: Signal<ShellLayout>,
    #[prop(default = None, into)] on_navigate: Option<Callback<String>>,
    #[prop(default = None)] collapsible: Option<SidebarCollapsible>,
    #[prop(default = None, into)] header_slot: Option<ViewFn>,
    /// Shown instead of `header_slot` while the sidebar is collapsed to the icon rail; the top
    /// bar ignores it.
    #[prop(default = None, into)]
    header_collapsed_slot: Option<ViewFn>,
    #[prop(default = None, into)] user_slot: Option<ViewFn>,
    #[prop(default = None, into)] footer_slot: Option<ViewFn>,
    #[prop(optional)] contained: bool,
    /// Framing of the page content: centred and padded (the default) or filling the area (`Full`).
    #[prop(optional, into)]
    content: Signal<ShellContent>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    move || match layout.get() {
        ShellLayout::Sidebar => SidebarShell(
            SidebarShellProps::builder()
                .nav(nav.clone())
                .current_path(current_path)
                .on_navigate(on_navigate)
                .collapsible(collapsible)
                .header_slot(header_slot.clone())
                .header_collapsed_slot(header_collapsed_slot.clone())
                .user_slot(user_slot.clone())
                .footer_slot(footer_slot.clone())
                .contained(contained)
                .content(content)
                .class(class.clone())
                .children(children.clone())
                .build(),
        )
        .into_any(),
        ShellLayout::TopBar => TopBarShell(
            TopBarShellProps::builder()
                .nav(nav.clone())
                .current_path(current_path)
                .on_navigate(on_navigate)
                .header_slot(header_slot.clone())
                .header_collapsed_slot(header_collapsed_slot.clone())
                .user_slot(user_slot.clone())
                .footer_slot(footer_slot.clone())
                .contained(contained)
                .content(content)
                .class(class.clone())
                .children(children.clone())
                .build(),
        )
        .into_any(),
    }
}
