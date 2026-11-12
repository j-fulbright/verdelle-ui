use crate::ShellContent;
use crate::nav::{follow, go};
use crate::{
    AppNav, ButtonVariant, DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger, NavItem, Size,
};
use leptos::prelude::*;

/// A whole-page layout with the menu in a bar above the page. Takes the same props as
/// [`crate::SidebarShell`] and the same [`AppNav`], so an app can switch between them
/// (see [`crate::AppShell`]) without touching its menu.
///
/// Top-level links are buttons in the bar; a group is a button that opens a menu of its links.
/// Below 768px the bar keeps the logo and the user, and everything else moves into one `Menu`
/// button (a dropdown, so no drawer). The current page's link, or its group, is highlighted.
/// `header_slot` is the logo at the start of the bar, `user_slot` sits at its end and
/// `footer_slot` goes below the page.
///
/// Icons: the bar's own buttons are text only (as before); an item's `icon` is drawn in the
/// dropdown and phone-menu rows, for children and for top-level links in the phone menu.
#[component]
pub fn TopBarShell(
    nav: AppNav,
    #[prop(into)] current_path: Signal<String>,
    #[prop(default = None, into)] on_navigate: Option<Callback<String>>,
    #[prop(default = None, into)] header_slot: Option<ViewFn>,
    /// Accepted so a shell can be swapped for [`crate::SidebarShell`] without changing props;
    /// the top bar has no icon rail, so it is not used.
    #[prop(default = None, into)]
    #[allow(unused_variables)]
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
    let nav = StoredValue::new(nav);
    let children = StoredValue::new(children);

    view! {
        <div class=format!("vdl-shell vdl-shell--top {class}") data-contained=contained.then_some("")>
            <header class="vdl-shell__topbar">
                {header_slot.map(|slot| view! { <div class="vdl-shell__logo">{slot.run()}</div> })}
                <nav class="vdl-shell__topnav" aria-label="Main">
                    {move || {
                        nav.with_value(|n| n.visible_items())
                            .into_iter()
                            .map(|item| {
                                view! { <TopBarItem item=item current_path=current_path on_navigate=on_navigate /> }
                            })
                            .collect_view()
                    }}
                </nav>
                <div class="vdl-shell__phone-menu">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>
                            "Menu"
                        </DropdownMenuTrigger>
                        <DropdownMenuContent width=256u32>
                            {move || {
                                nav.with_value(|n| n.visible_items())
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, item)| {
                                        view! { <PhoneMenuItem item=item first=i == 0 current_path=current_path on_navigate=on_navigate /> }
                                    })
                                    .collect_view()
                            }}
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
                {user_slot.map(|slot| view! { <div class="vdl-shell__user">{slot.run()}</div> })}
            </header>
            <main class="vdl-shell__scroll">
                <div class="vdl-shell__content" data-width=move || content.get().attribute()>{children.with_value(|c| c())}</div>
            </main>
            {footer_slot.map(|slot| view! { <footer class="vdl-shell__bottom">{slot.run()}</footer> })}
        </div>
    }
}

/// A link button, or a group button with a dropdown, in the bar.
#[component]
fn TopBarItem(
    item: NavItem,
    current_path: Signal<String>,
    on_navigate: Option<Callback<String>>,
) -> impl IntoView {
    let item = StoredValue::new(item);
    let label = item.with_value(|i| i.label.clone());
    let active = move || {
        item.with_value(|i| i.is_active(&current_path.get()))
            .then_some("")
    };

    if !item.with_value(|i| i.is_group()) {
        let href = item.with_value(|i| i.href.clone());
        let target = href.clone().unwrap_or_default();
        return view! {
            <a
                class="vdl-btn vdl-shell__link"
                data-variant="ghost"
                data-size="sm"
                href=href
                data-active=active
                aria-current=move || active().map(|_| "page")
                aria-disabled=item.with_value(|i| i.disabled).then_some("true")
                on:click=move |ev: leptos::ev::MouseEvent| follow(on_navigate, &target, &ev)
            >
                <span class="vdl-btn__content">{label}</span>
            </a>
        }
        .into_any();
    }

    view! {
        <span class="vdl-shell__group" data-active=active>
            <DropdownMenu>
                <DropdownMenuTrigger variant=ButtonVariant::Ghost size=Size::Small>
                    {item.with_value(|i| i.label.clone())}
                </DropdownMenuTrigger>
                <DropdownMenuContent width=208u32>
                    {move || {
                        item.with_value(|i| i.visible_children())
                            .into_iter()
                            .map(|child| view! { <MenuLink item=child current_path=current_path on_navigate=on_navigate /> })
                            .collect_view()
                    }}
                </DropdownMenuContent>
            </DropdownMenu>
        </span>
    }
    .into_any()
}

/// One link inside a dropdown, with its icon if it has one; the current page is bold.
#[component]
pub(crate) fn MenuLink(
    item: NavItem,
    current_path: Signal<String>,
    on_navigate: Option<Callback<String>>,
) -> impl IntoView {
    let href = item.href.clone().unwrap_or_default();
    let item = StoredValue::new(item);
    let current = move || item.with_value(|i| i.is_active(&current_path.get()));
    view! {
        <DropdownMenuItem
            disabled=item.with_value(|i| i.disabled)
            on_select=move |()| go(on_navigate, &href)
        >
            {item.with_value(|i| i.icon.as_ref().map(|icon| icon.run()))}
            <span class="vdl-shell__menu-label" data-active=move || current().then_some("")>
                {item.with_value(|i| i.label.clone())}
            </span>
        </DropdownMenuItem>
    }
}

/// An entry of the phone menu: a top-level link, or a labelled group of links.
#[component]
fn PhoneMenuItem(
    item: NavItem,
    first: bool,
    current_path: Signal<String>,
    on_navigate: Option<Callback<String>>,
) -> impl IntoView {
    if !item.is_group() {
        return view! { <MenuLink item=item current_path=current_path on_navigate=on_navigate /> }
            .into_any();
    }
    let label = StoredValue::new(item.label.clone());
    let children = StoredValue::new(item.visible_children());
    view! {
        {(!first).then(|| view! { <DropdownMenuSeparator /> })}
        <DropdownMenuGroup>
            <DropdownMenuLabel>{label.get_value()}</DropdownMenuLabel>
            {children
                .get_value()
                .into_iter()
                .map(|child| view! { <MenuLink item=child current_path=current_path on_navigate=on_navigate /> })
                .collect_view()}
        </DropdownMenuGroup>
    }
    .into_any()
}
