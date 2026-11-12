use crate::ShellContent;
use crate::nav::follow;
use crate::primitives::dropdown_menu as menu_primitive;
use crate::top_bar_shell::MenuLink;
use crate::{
    Align, AppNav, Collapsible, CollapsibleContent, DropdownMenu, DropdownMenuContent,
    DropdownMenuLabel, NavItem, Side, Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter,
    SidebarGroup, SidebarGroupContent, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuBadge,
    SidebarMenuButton, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem,
    SidebarProvider, SidebarRail, SidebarTrigger, icons, use_sidebar,
};
use leptos::prelude::*;

/// A whole-page layout with the menu in a column beside the page, as an operations console
/// has it: a header (your logo), the menu with top-level links and groups that fold, a user
/// area and a version line at the bottom, and the page in a centred wrapper. Built on
/// [`crate::Sidebar`], so below 768px the column becomes a [`crate::Sheet`] opened by a menu
/// button in a bar above the page.
///
/// The menu is an [`AppNav`]; `current_path` is the page you are on (for example
/// `use_location().pathname`). The group that holds the current page opens by itself, and the
/// current page is highlighted. Links go through `on_navigate` when it is given (connect it to
/// the router's `use_navigate`), otherwise they are plain anchors.
///
/// The column is fixed and always shown on wide screens. Pass `collapsible=Some(Icon)` (a rail
/// of icons) or `Some(Offcanvas)` to let people collapse it: that adds a trigger to the bar,
/// the Ctrl/Cmd+B shortcut and a rail on the edge, and remembers the choice in a cookie.
///
/// `header_slot`, `user_slot` and `footer_slot` are views for the top, the user area and the
/// bottom of the column (see [`crate::ShellUser`]). `contained` keeps the shell inside its
/// parent (give the parent a height) instead of filling the window, for demos.
///
/// **The icon rail.** The rail is 3rem wide, so a header written for the full column does not
/// fit. Give `header_collapsed_slot` (usually just the logo mark) and the shell shows it in the
/// rail instead of `header_slot`. Without it the rail keeps the first part of the header's top-level
/// row (the logo) and drops the rest, so there is no half-cut text; parts of the header that should vanish in the rail (the app name) can carry
/// the class `vdl-shell__hide-collapsed`. The user area shows its avatar only, centred.
///
/// In the rail a group (a [`NavItem`] with children) opens a menu to the side of its icon with
/// its links (icons, the current page marked): on click, Enter, Space or ArrowRight. The group
/// tooltip is not shown while that menu is open, and only one tooltip shows at a time.
#[component]
pub fn SidebarShell(
    nav: AppNav,
    #[prop(into)] current_path: Signal<String>,
    #[prop(default = None, into)] on_navigate: Option<Callback<String>>,
    /// `None` (the default): a fixed column. `Some(..)`: people can collapse it.
    #[prop(default = None)]
    collapsible: Option<SidebarCollapsible>,
    #[prop(default = None, into)] header_slot: Option<ViewFn>,
    /// Shown instead of `header_slot` while the sidebar is collapsed to the icon rail (see above).
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
    let children = StoredValue::new(children);
    let nav = StoredValue::new(nav);
    let header_slot = StoredValue::new(header_slot);
    let header_collapsed_slot = StoredValue::new(header_collapsed_slot);
    let user_slot = StoredValue::new(user_slot);
    let footer_slot = StoredValue::new(footer_slot);
    let collapsible_by_user = collapsible.is_some();
    let rail = collapsible == Some(SidebarCollapsible::Icon);

    view! {
        <SidebarProvider
            contained=contained
            persist=collapsible_by_user
            keyboard_shortcut=if collapsible_by_user { "b" } else { "" }
            class=format!("vdl-shell {class}")
        >
            <Sidebar collapsible=collapsible.unwrap_or(SidebarCollapsible::Offcanvas) label="Main">
                {(header_slot.get_value().is_some() || header_collapsed_slot.get_value().is_some())
                    .then(|| {
                        let has_rail_slot = header_collapsed_slot.get_value().is_some();
                        view! {
                            <SidebarHeader class="vdl-shell__header">
                                <div class="vdl-shell__header-full" data-rail-slot=has_rail_slot.then_some("")>
                                    {header_slot.get_value().map(|slot| slot.run())}
                                </div>
                                {header_collapsed_slot
                                    .get_value()
                                    .map(|slot| view! { <div class="vdl-shell__header-rail">{slot.run()}</div> })}
                            </SidebarHeader>
                        }
                    })}
                <SidebarContent>
                    <SidebarGroup>
                        <SidebarGroupContent>
                            <SidebarMenu>
                                {move || {
                                    nav.with_value(|n| n.visible_items())
                                        .into_iter()
                                        .map(|item| {
                                            view! {
                                                <SidebarNavItem
                                                    item=item
                                                    current_path=current_path
                                                    on_navigate=on_navigate
                                                    rail=rail
                                                />
                                            }
                                        })
                                        .collect_view()
                                }}
                            </SidebarMenu>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
                {(user_slot.get_value().is_some() || footer_slot.get_value().is_some())
                    .then(|| {
                        view! {
                            <SidebarFooter class="vdl-shell__footer">
                                {user_slot.get_value().map(|slot| slot.run())}
                                {footer_slot
                                    .get_value()
                                    .map(|slot| view! { <div class="vdl-shell__version">{slot.run()}</div> })}
                            </SidebarFooter>
                        }
                    })}
                {collapsible_by_user.then(|| view! { <SidebarRail /> })}
            </Sidebar>
            <SidebarInset class="vdl-shell__inset">
                <header class="vdl-shell__bar">
                    <SidebarTrigger class=if collapsible_by_user { "" } else { "vdl-shell__mobile-only" } />
                    <nav class="vdl-shell__trail" aria-label="Breadcrumb">
                        {move || {
                            let trail = nav.with_value(|n| n.trail(&current_path.get()));
                            let last = trail.len().saturating_sub(1);
                            trail
                                .into_iter()
                                .enumerate()
                                .map(|(i, label)| {
                                    view! {
                                        {(i > 0).then(|| view! { <span class="vdl-shell__trail-sep" aria-hidden="true">"/"</span> })}
                                        <span aria-current=(i == last).then_some("page")>{label}</span>
                                    }
                                })
                                .collect_view()
                        }}
                    </nav>
                </header>
                <div class="vdl-shell__scroll">
                    <div class="vdl-shell__content" data-width=move || content.get().attribute()>{children.with_value(|c| c())}</div>
                </div>
            </SidebarInset>
        </SidebarProvider>
    }
}

/// Whether the icon rail is what people see: the sidebar is collapsible to icons, is collapsed
/// and the screen is wide (a small screen shows the sheet instead).
fn rail_visible(collapsible_to_icon: bool, expanded: bool, mobile: bool) -> bool {
    collapsible_to_icon && !expanded && !mobile
}

/// One row of the sidebar: a link, or a button that folds a group of links (in the icon rail,
/// a button that opens a menu of them beside the rail).
#[component]
fn SidebarNavItem(
    item: NavItem,
    current_path: Signal<String>,
    on_navigate: Option<Callback<String>>,
    /// The sidebar collapses to an icon rail.
    rail: bool,
) -> impl IntoView {
    let ctx = use_sidebar();
    let item = StoredValue::new(item);
    let label = item.with_value(|i| i.label.clone());
    let href = item.with_value(|i| i.href.clone());
    let disabled = item.with_value(|i| i.disabled);
    let badge = item.with_value(|i| i.badge.clone());
    let icon = move || item.with_value(|i| i.icon.as_ref().map(|icon| icon.run()));
    let active = Signal::derive(move || item.with_value(|i| i.is_active(&current_path.get())));
    let open_mobile = ctx.open_mobile;

    if !item.with_value(|i| i.is_group()) {
        let link = href.clone().unwrap_or_default();
        return view! {
            <SidebarMenuItem>
                <SidebarMenuButton
                    href=href.unwrap_or_default()
                    tooltip=label.clone()
                    is_active=active
                    disabled=disabled
                    on_click=move |ev: leptos::ev::MouseEvent| {
                        follow(on_navigate, &link, &ev);
                        open_mobile.set(false);
                    }
                >
                    {icon}
                    <span>{label.clone()}</span>
                </SidebarMenuButton>
                {badge.map(|b| view! { <SidebarMenuBadge>{b}</SidebarMenuBadge> })}
            </SidebarMenuItem>
        }
        .into_any();
    }

    // A group opens by itself when the current page is inside it, and otherwise stays as
    // the person left it.
    let expanded =
        RwSignal::new(item.with_value(|i| i.has_active_descendant(&current_path.get_untracked())));
    Effect::new(move |_| {
        let inside = item.with_value(|i| i.has_active_descendant(&current_path.get()));
        if inside && !expanded.get_untracked() {
            expanded.set(true);
        }
    });

    let in_rail = move || rail_visible(rail, ctx.expanded(), ctx.is_mobile.get());
    // The group's flyout menu, open only in the rail.
    let flyout_open = RwSignal::new(false);
    Effect::new(move |_| {
        if !in_rail() {
            flyout_open.set(false);
        }
    });
    let label = StoredValue::new(label);

    view! {
        <SidebarMenuItem>
            {move || {
                if in_rail() {
                    view! {
                        <DropdownMenu
                            open=flyout_open
                            on_open_change=move |o: bool| flyout_open.set(o && in_rail())
                        >
                            <menu_primitive::DropdownMenuTrigger as_child=true arrow_right_opens=true>
                                // The wrapper takes the menu's trigger role so the button can keep
                                // its tooltip; focus sent to it (the menu closing) goes to the button.
                                <div
                                    class="vdl-shell__flyout-anchor"
                                    tabindex="-1"
                                    on:focus=move |ev: leptos::ev::FocusEvent| {
                                        use leptos::wasm_bindgen::JsCast;
                                        if let Some(anchor) = ev
                                            .current_target()
                                            .and_then(|t| t.dyn_into::<leptos::web_sys::Element>().ok())
                                            && let Ok(Some(button)) = anchor.query_selector("[data-sidebar=\"menu-button\"]")
                                            && let Ok(button) = button.dyn_into::<leptos::web_sys::HtmlElement>()
                                        {
                                            let _ = button.focus();
                                        }
                                    }
                                >
                                    <SidebarMenuButton
                                        tooltip=label.get_value()
                                        tooltip_hidden=flyout_open
                                        is_active=active
                                        disabled=disabled
                                    >
                                        {icon}
                                        <span>{label.get_value()}</span>
                                    </SidebarMenuButton>
                                </div>
                            </menu_primitive::DropdownMenuTrigger>
                            <DropdownMenuContent
                                side=Side::Right
                                align=Align::Start
                                side_offset=8.0
                                width=224u32
                                class="vdl-shell__flyout"
                            >
                                <DropdownMenuLabel>{label.get_value()}</DropdownMenuLabel>
                                {move || {
                                    item.with_value(|i| i.visible_children())
                                        .into_iter()
                                        .map(|child| {
                                            view! { <MenuLink item=child current_path=current_path on_navigate=on_navigate /> }
                                        })
                                        .collect_view()
                                }}
                            </DropdownMenuContent>
                        </DropdownMenu>
                    }
                    .into_any()
                } else {
                    view! {
                        <SidebarMenuButton
                            tooltip=label.get_value()
                            is_active=active
                            disabled=disabled
                            expanded=Signal::from(expanded)
                            on_click=move |_| expanded.update(|v| *v = !*v)
                        >
                            {icon}
                            <span>{label.get_value()}</span>
                            <span class="vdl-shell__chevron" aria-hidden="true">{icons::chevron_right()}</span>
                        </SidebarMenuButton>
                    }
                    .into_any()
                }
            }}
            <Collapsible open=expanded on_open_change=move |v| expanded.set(v)>
                <CollapsibleContent>
                    <SidebarMenuSub>
                        {move || {
                            item.with_value(|i| i.visible_children())
                                .into_iter()
                                .map(|child| {
                                    let target = child.href.clone().unwrap_or_default();
                                    let link = target.clone();
                                    let child_active = {
                                        let child = child.clone();
                                        Signal::derive(move || child.is_active(&current_path.get()))
                                    };
                                    view! {
                                        <SidebarMenuSubItem>
                                            <SidebarMenuSubButton
                                                href=link
                                                is_active=child_active
                                                on_click=move |ev: leptos::ev::MouseEvent| {
                                                    follow(on_navigate, &target, &ev);
                                                    open_mobile.set(false);
                                                }
                                            >
                                                {child.icon.as_ref().map(|icon| icon.run())}
                                                <span>{child.label.clone()}</span>
                                            </SidebarMenuSubButton>
                                        </SidebarMenuSubItem>
                                    }
                                })
                                .collect_view()
                        }}
                    </SidebarMenuSub>
                </CollapsibleContent>
            </Collapsible>
        </SidebarMenuItem>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::rail_visible;

    #[test]
    fn rail_shows_only_when_collapsed_to_icons_on_a_wide_screen() {
        assert!(rail_visible(true, false, false));
        assert!(!rail_visible(true, true, false), "expanded");
        assert!(
            !rail_visible(true, false, true),
            "the small-screen sheet is not the rail"
        );
        assert!(
            !rail_visible(false, false, false),
            "offcanvas or fixed has no rail"
        );
    }
}
