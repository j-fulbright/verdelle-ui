use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    AppNav, AppShell, Avatar, AvatarFallback, Button, ButtonVariant, Collapsible,
    CollapsibleContent, CollapsibleTrigger, DropdownMenu, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger, Icon,
    IconName, ModeToggle, NavItem, PageHeader, ShellContent, ShellLayout, ShellUser,
    SidebarCollapsible, Size, Tabs, TabsList, TabsTrigger,
};

/// An internal-tool menu shaped like a real one: a few top-level pages, three products that
/// repeat the same pages (GitHub, Jira, Servers, ...), and an admin group. Names are generic.
struct Group {
    label: &'static str,
    items: &'static [&'static str],
}

const TOP: &[&str] = &["Dashboard", "Activity", "Chat"];
const GROUPS: &[Group] = &[
    Group {
        label: "Alpha",
        items: &["GitHub", "Jira", "Servers", "Workers", "Reports"],
    },
    Group {
        label: "Beta",
        items: &["GitHub", "Jira", "Servers", "Activity"],
    },
    Group {
        label: "QA",
        items: &["GitHub"],
    },
    Group {
        label: "Admin",
        items: &[
            "Sessions",
            "Permissions",
            "Autoscaler",
            "Scheduled tasks",
            "Database",
            "Token generator",
            "Hook tokens",
        ],
    },
];

/// The current page, as "Group / Item" (or just the item for a top-level page).
type Current = RwSignal<String>;

fn key(group: &str, item: &str) -> String {
    if group.is_empty() {
        item.to_string()
    } else {
        format!("{group} / {item}")
    }
}

/// A fake app window: fixed height, so each layout can be judged next to the others.
#[component]
fn Frame(#[prop(optional, into)] width: String, children: Children) -> impl IntoView {
    let style = if width.is_empty() {
        String::new()
    } else {
        format!("width: {width}; max-width: 100%;")
    };
    view! {
        <div
            class="flex h-[460px] overflow-hidden rounded-lg border border-border bg-background text-sm"
            style=style
        >
            {children()}
        </div>
    }
}

/// The page area every layout shares.
#[component]
fn Content(current: Current) -> impl IntoView {
    view! {
        <main class="min-w-0 flex-1 overflow-auto p-5">
            <PageHeader
                level=3
                title=Signal::derive(move || {
                    current.get().rsplit(" / ").next().unwrap_or_default().to_string()
                })
                description=Signal::derive(move || Some(current.get()))
                actions=|| {
                    view! {
                        <Button variant=ButtonVariant::Outline size=Size::Small>"Export"</Button>
                        <Button size=Size::Small>"New"</Button>
                    }
                }
            />
            <div class="mt-4 space-y-2">
                <div class="h-9 rounded-md bg-muted"></div>
                <div class="h-9 rounded-md bg-muted"></div>
                <div class="h-9 w-2/3 rounded-md bg-muted"></div>
            </div>
        </main>
    }
}

#[component]
fn NavLink(
    current: Current,
    group: &'static str,
    item: &'static str,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let k = key(group, item);
    let k2 = k.clone();
    view! {
        <button
            type="button"
            class=format!(
                "flex w-full items-center rounded-md px-2 py-1.5 text-left hover:bg-accent hover:text-accent-foreground data-[active]:bg-accent data-[active]:font-medium {class}"
            )
            attr:data-active=move || (current.get() == k).then_some("")
            aria-current=move || (current.get() == k2).then_some("page")
            on:click={
                let k = key(group, item);
                move |_| current.set(k.clone())
            }
        >
            {item}
        </button>
    }
}

#[component]
fn UserChip() -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 border-t border-border p-3">
            <Avatar size=Size::Small>
                <AvatarFallback>"JF"</AvatarFallback>
            </Avatar>
            <div class="min-w-0 text-xs">
                <div class="truncate font-medium">"Jane Doe"</div>
                <div class="truncate text-muted-foreground">"jane@example.com"</div>
            </div>
        </div>
    }
}

/// A. The sidebar as it is today, minus collapsing: always 224px wide, groups fold.
#[component]
fn PersistentSidebar() -> impl IntoView {
    let current: Current = RwSignal::new("Dashboard".to_string());
    view! {
        <Frame>
            <aside class="flex w-56 shrink-0 flex-col border-r border-border">
                <div class="px-4 py-3 text-sm font-semibold">"Console"</div>
                <nav class="flex-1 space-y-1 overflow-auto px-2 pb-2">
                    {TOP
                        .iter()
                        .map(|t| view! { <NavLink current=current group="" item=*t /> })
                        .collect_view()}
                    {GROUPS
                        .iter()
                        .map(|g| {
                            view! {
                                <Collapsible default_open=g.label == "Alpha">
                                    <CollapsibleTrigger class="mt-1 flex w-full items-center justify-between rounded-md px-2 py-1.5 text-left font-medium hover:bg-accent">
                                        {g.label}
                                    </CollapsibleTrigger>
                                    <CollapsibleContent class="ml-3 space-y-0.5 border-l border-border pl-2">
                                        {g
                                            .items
                                            .iter()
                                            .map(|i| view! { <NavLink current=current group=g.label item=*i /> })
                                            .collect_view()}
                                    </CollapsibleContent>
                                </Collapsible>
                            }
                        })
                        .collect_view()}
                </nav>
                <UserChip />
            </aside>
            <Content current=current />
        </Frame>
    }
}

/// B. The same menu on a phone: a top bar whose menu button opens a dropdown, so no
/// drawer is needed. Everything fits in one list because the menu is only 20 items.
#[component]
fn PhoneDropdown() -> impl IntoView {
    let current: Current = RwSignal::new("Dashboard".to_string());
    view! {
        <Frame width="340px">
            <div class="flex min-w-0 flex-1 flex-col">
                <header class="flex items-center gap-2 border-b border-border px-3 py-2">
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>
                            "Menu"
                        </DropdownMenuTrigger>
                        <DropdownMenuContent width=240u32>
                            {TOP
                                .iter()
                                .map(|t| {
                                    let k = key("", t);
                                    view! {
                                        <DropdownMenuItem on_select=move |()| current.set(k.clone())>
                                            {*t}
                                        </DropdownMenuItem>
                                    }
                                })
                                .collect_view()}
                            {GROUPS
                                .iter()
                                .map(|g| {
                                    view! {
                                        <DropdownMenuSeparator />
                                        <DropdownMenuGroup>
                                            <DropdownMenuLabel>{g.label}</DropdownMenuLabel>
                                            {g
                                                .items
                                                .iter()
                                                .map(|i| {
                                                    let k = key(g.label, i);
                                                    view! {
                                                        <DropdownMenuItem on_select=move |()| current.set(k.clone())>
                                                            {*i}
                                                        </DropdownMenuItem>
                                                    }
                                                })
                                                .collect_view()}
                                        </DropdownMenuGroup>
                                    }
                                })
                                .collect_view()}
                        </DropdownMenuContent>
                    </DropdownMenu>
                    <span class="min-w-0 flex-1 truncate font-medium">{move || current.get()}</span>
                    <Avatar size=Size::Small>
                        <AvatarFallback>"JF"</AvatarFallback>
                    </Avatar>
                </header>
                <Content current=current />
            </div>
        </Frame>
    }
}

/// C. A top bar. Top-level pages are links, each product and Admin is a dropdown.
#[component]
fn TopBar() -> impl IntoView {
    let current: Current = RwSignal::new("Dashboard".to_string());
    view! {
        <Frame>
            <div class="flex min-w-0 flex-1 flex-col">
                <header class="flex flex-wrap items-center gap-1 border-b border-border px-3 py-2">
                    <span class="mr-3 font-semibold">"Console"</span>
                    {TOP
                        .iter()
                        .map(|t| {
                            let k = key("", t);
                            view! {
                                <Button
                                    variant=ButtonVariant::Ghost
                                    size=Size::Small
                                    on_click=move |_| current.set(k.clone())
                                >
                                    {*t}
                                </Button>
                            }
                        })
                        .collect_view()}
                    {GROUPS
                        .iter()
                        .map(|g| {
                            view! {
                                <DropdownMenu>
                                    <DropdownMenuTrigger variant=ButtonVariant::Ghost size=Size::Small>
                                        {g.label}
                                    </DropdownMenuTrigger>
                                    <DropdownMenuContent width=200u32>
                                        {g
                                            .items
                                            .iter()
                                            .map(|i| {
                                                let k = key(g.label, i);
                                                view! {
                                                    <DropdownMenuItem on_select=move |()| current.set(k.clone())>
                                                        {*i}
                                                    </DropdownMenuItem>
                                                }
                                            })
                                            .collect_view()}
                                    </DropdownMenuContent>
                                </DropdownMenu>
                            }
                        })
                        .collect_view()}
                    <span class="ml-auto">
                        <Avatar size=Size::Small>
                            <AvatarFallback>"JF"</AvatarFallback>
                        </Avatar>
                    </span>
                </header>
                <Content current=current />
            </div>
        </Frame>
    }
}

/// D. Products repeat the same pages, so pick the product once (a menu) and keep one
/// short row of tabs for the pages. The sidebar disappears and the tree becomes flat.
#[component]
fn ProductTabs() -> impl IntoView {
    let product = RwSignal::new("Alpha".to_string());
    let page = RwSignal::new("GitHub".to_string());
    let current: Current = RwSignal::new("Alpha / GitHub".to_string());
    Effect::new(move |_| current.set(key(&product.get(), &page.get())));

    view! {
        <Frame>
            <div class="flex min-w-0 flex-1 flex-col">
                <header class="flex items-center gap-3 border-b border-border px-3 py-2">
                    <span class="font-semibold">"Console"</span>
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>
                            {move || product.get()}
                        </DropdownMenuTrigger>
                        <DropdownMenuContent width=180u32>
                            <DropdownMenuLabel>"Product"</DropdownMenuLabel>
                            {GROUPS
                                .iter()
                                .filter(|g| g.label != "Admin")
                                .map(|g| {
                                    view! {
                                        <DropdownMenuItem on_select=move |()| {
                                            product.set(g.label.to_string());
                                            page.set(g.items[0].to_string());
                                        }>{g.label}</DropdownMenuItem>
                                    }
                                })
                                .collect_view()}
                        </DropdownMenuContent>
                    </DropdownMenu>
                    <span class="ml-auto flex items-center gap-1">
                        {TOP
                            .iter()
                            .map(|t| {
                                view! {
                                    <Button
                                        variant=ButtonVariant::Ghost
                                        size=Size::Small
                                        on_click=move |_| current.set(t.to_string())
                                    >
                                        {*t}
                                    </Button>
                                }
                            })
                            .collect_view()}
                        <Avatar size=Size::Small>
                            <AvatarFallback>"JF"</AvatarFallback>
                        </Avatar>
                    </span>
                </header>
                <div class="border-b border-border px-3 py-2">
                    {move || {
                        let items = GROUPS
                            .iter()
                            .find(|g| g.label == product.get())
                            .map(|g| g.items)
                            .unwrap_or(&[]);
                        view! {
                            <Tabs
                                value=page.get_untracked()
                                on_value_change=move |v: String| page.set(v)
                            >
                                <TabsList>
                                    {items
                                        .iter()
                                        .map(|i| {
                                            view! { <TabsTrigger value=i.to_string()>{*i}</TabsTrigger> }
                                        })
                                        .collect_view()}
                                </TabsList>
                            </Tabs>
                        }
                    }}
                </div>
                <Content current=current />
            </div>
        </Frame>
    }
}

/// E. A narrow icon rail picks the group; the second column lists its pages. Always
/// visible, so nothing slides in and nothing is hidden behind a button.
#[component]
fn RailAndPanel() -> impl IntoView {
    let group = RwSignal::new("Alpha");
    let current: Current = RwSignal::new("Alpha / GitHub".to_string());
    view! {
        <Frame>
            <aside class="flex w-14 shrink-0 flex-col items-center gap-1 border-r border-border py-2">
                {GROUPS
                    .iter()
                    .map(|g| {
                        let label = g.label;
                        view! {
                            <button
                                type="button"
                                title=label
                                class="flex size-9 items-center justify-center rounded-md text-xs font-semibold hover:bg-accent data-[active]:bg-accent"
                                attr:data-active=move || (group.get() == label).then_some("")
                                on:click=move |_| group.set(label)
                            >
                                {label.chars().take(2).collect::<String>()}
                            </button>
                        }
                    })
                    .collect_view()}
            </aside>
            <aside class="w-44 shrink-0 space-y-0.5 overflow-auto border-r border-border p-2">
                <div class="px-2 pb-1 text-xs font-medium text-muted-foreground">
                    {move || group.get()}
                </div>
                {move || {
                    let g = group.get();
                    GROUPS
                        .iter()
                        .find(|x| x.label == g)
                        .map(|x| {
                            x.items
                                .iter()
                                .map(|i| view! { <NavLink current=current group=x.label item=*i /> })
                                .collect_view()
                        })
                }}
            </aside>
            <Content current=current />
        </Frame>
    }
}

/// A 16px stroke icon from Lucide path data.
fn glyph(paths: &'static [&'static str]) -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            {paths.iter().map(|d| view! { <path d=*d /> }).collect_view()}
        </svg>
    }
}

const ICON_HOME: &[&str] = &[
    "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8",
    "M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z",
];
const ICON_SERVER: &[&str] = &[
    "M2 4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2z",
    "M2 14a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2z",
    "M6 6h.01",
    "M6 18h.01",
];
const ICON_BRANCH: &[&str] = &[
    "M6 3v12",
    "M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6",
    "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6",
    "M18 9a9 9 0 0 1-9 9",
];
const ICON_FLASK: &[&str] = &[
    "M10 2v7.527a2 2 0 0 1-.211.896L4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-5.069-10.127A2 2 0 0 1 14 9.527V2",
    "M8.5 2h7",
];
const ICON_SHIELD: &[&str] = &[
    "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z",
];
const ICON_CHAT: &[&str] = &["M7.9 20A9 9 0 1 0 4 16.1L2 22Z"];

/// The one menu every shell below is drawn from. `admin` shows or hides the Admin group.
/// `child_icons` gives the second level icons too (`NavItem::icon` on a child); without it the
/// children are text only, which is the default.
fn demo_nav(admin: Signal<bool>, child_icons: bool) -> AppNav {
    let sub = move |item: NavItem, name: IconName| {
        if child_icons {
            item.icon(move || view! { <Icon name=name /> })
        } else {
            item
        }
    };
    let icon_server = || glyph(ICON_SERVER);
    AppNav::new()
        .item(NavItem::new("Dashboard", "/dashboard").icon(|| glyph(ICON_HOME)))
        .item(NavItem::new("Activity", "/activity").icon(|| glyph(ICON_BRANCH)))
        .item(
            NavItem::new("Chat", "/chat")
                .icon(|| glyph(ICON_CHAT))
                .badge("3"),
        )
        .item(
            NavItem::group("Alpha")
                .icon(icon_server)
                .child(sub(
                    NavItem::new("GitHub", "/alpha/github"),
                    IconName::Github,
                ))
                .child(sub(NavItem::new("Jira", "/alpha/jira"), IconName::Bug))
                .child(sub(
                    NavItem::new("Servers", "/alpha/servers"),
                    IconName::Server,
                ))
                .child(sub(
                    NavItem::new("Workers", "/alpha/workers"),
                    IconName::Cpu,
                ))
                .child(sub(
                    NavItem::new("Reports", "/alpha/reports"),
                    IconName::FileText,
                )),
        )
        .item(
            NavItem::group("Beta")
                .icon(icon_server)
                .child(sub(
                    NavItem::new("GitHub", "/beta/github"),
                    IconName::Github,
                ))
                .child(sub(NavItem::new("Jira", "/beta/jira"), IconName::Bug))
                .child(sub(
                    NavItem::new("Servers", "/beta/servers"),
                    IconName::Server,
                ))
                .child(sub(
                    NavItem::new("Activity", "/beta/activity"),
                    IconName::Activity,
                )),
        )
        .item(
            NavItem::group("QA")
                .icon(|| glyph(ICON_FLASK))
                .child(sub(NavItem::new("GitHub", "/qa/github"), IconName::Github)),
        )
        .item(
            NavItem::group("Admin")
                .icon(|| glyph(ICON_SHIELD))
                .visible(admin)
                .child(sub(
                    NavItem::new("Sessions", "/admin/sessions"),
                    IconName::Users,
                ))
                .child(sub(
                    NavItem::new("Permissions", "/admin/permissions"),
                    IconName::ShieldCheck,
                ))
                .child(sub(
                    NavItem::new("Autoscaler", "/admin/autoscaler"),
                    IconName::Layers,
                ))
                .child(sub(
                    NavItem::new("Scheduled tasks", "/admin/scheduled-tasks"),
                    IconName::Clock,
                ))
                .child(sub(
                    NavItem::new("Database", "/admin/database"),
                    IconName::Database,
                ))
                .child(sub(
                    NavItem::new("Token generator", "/admin/token-generator"),
                    IconName::Key,
                ))
                .child(sub(
                    NavItem::new("Hook tokens", "/admin/hook-tokens"),
                    IconName::Zap,
                )),
        )
}

/// A button in a row of choices; the chosen one looks pressed.
#[component]
fn Choice<T: Copy + PartialEq + Send + Sync + 'static>(
    value: RwSignal<T>,
    this: T,
    label: &'static str,
) -> impl IntoView {
    view! {
        <Button
            variant=ButtonVariant::Outline
            size=Size::Small
            selected=Signal::derive(move || value.get() == this)
            on_click=move |_| value.set(this)
        >
            {label}
        </Button>
    }
}

/// How the demo's header behaves in the icon rail.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RailHeader {
    /// Logo and name with no help: centred and clipped with a fade in the rail.
    Clipped,
    /// The name carries `vdl-shell__hide-collapsed`, so only the logo is left in the rail.
    HideClass,
    /// `header_collapsed_slot`: a dedicated logo mark for the rail.
    Slot,
}

/// A stand-in logo mark (an image in a real app).
#[component]
fn LogoMark() -> impl IntoView {
    view! {
        <span class="inline-flex size-8 shrink-0 items-center justify-center rounded-md bg-primary text-sm font-bold text-primary-foreground">
            "C"
        </span>
    }
}

/// The real `AppShell`, driven by the nav above. Switch the layout, the collapse mode and the
/// Admin permission; the menu data never changes.
#[component]
fn LiveShell() -> impl IntoView {
    let layout = RwSignal::new(ShellLayout::Sidebar);
    let collapse = RwSignal::new(None::<SidebarCollapsible>);
    let admin = RwSignal::new(true);
    let path = RwSignal::new("/alpha/servers".to_string());
    let signed_in = RwSignal::new(true);
    let child_icons = RwSignal::new(false);
    let rail_header = RwSignal::new(RailHeader::Clipped);
    let content = RwSignal::new(ShellContent::Contained);
    // The person loads after the shell is on screen: ShellUser takes signals, so the name,
    // the email and the initials follow.
    let user = RwSignal::new(("Loading...".to_string(), String::new()));
    Effect::new(move |_| {
        set_timeout(
            move || user.set(("Jane Doe".to_string(), "jane@example.com".to_string())),
            std::time::Duration::from_millis(1200),
        );
    });
    let user_name = Signal::derive(move || user.get().0);
    let user_email = Signal::derive(move || user.get().1);

    view! {
        <div class="space-y-3">
            <Row label="Layout">
                <Choice value=layout this=ShellLayout::Sidebar label="Sidebar" />
                <Choice value=layout this=ShellLayout::TopBar label="Top bar" />
            </Row>
            <Row label="Collapse (sidebar)">
                <Choice value=collapse this=None label="Fixed" />
                <Choice value=collapse this=Some(SidebarCollapsible::Icon) label="Icon rail" />
                <Choice value=collapse this=Some(SidebarCollapsible::Offcanvas) label="Offcanvas" />
            </Row>
            <Row label="Child icons">
                <Choice value=child_icons this=false label="None (default)" />
                <Choice value=child_icons this=true label="Icons" />
                <span class="text-xs text-muted-foreground">
                    "second level: sidebar rows and dropdown rows; in the icon rail they appear in the group's flyout menu"
                </span>
            </Row>
            <Row label="Rail header">
                <Choice value=rail_header this=RailHeader::Clipped label="Clipped (no help)" />
                <Choice value=rail_header this=RailHeader::HideClass label="hide-collapsed class" />
                <Choice value=rail_header this=RailHeader::Slot label="header_collapsed_slot" />
                <span class="text-xs text-muted-foreground">
                    "collapse with the trigger or Ctrl/Cmd+B; hover the rail icons for tooltips, click a group icon for its flyout menu"
                </span>
            </Row>
            <Row label="Page framing">
                <Choice value=content this=ShellContent::Contained label="Contained (default)" />
                <Choice value=content this=ShellContent::Full label="Full" />
                <span class="text-xs text-muted-foreground">
                    "Full: the page draws its own full-bleed background (the tinted area) with no padding or width limit"
                </span>
            </Row>
            <Row label="User">
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    attr:id="shell-user-rename"
                    on_click=move |_| {
                        user.set(("Alexandra Quincy-Smith".to_string(), "alexandra.quincy-smith@example.com".to_string()))
                    }
                >
                    "Rename (signal)"
                </Button>
            </Row>
            <Row label="Permission">
                <Choice value=admin this=true label="Admin" />
                <Choice value=admin this=false label="Not admin" />
            </Row>
        </div>
        <div class="h-[560px] overflow-hidden rounded-lg border border-border">
            {move || {
                view! {
                    <AppShell
                        nav=demo_nav(admin.into(), child_icons.get())
                        current_path=path
                        layout=layout
                        on_navigate=Callback::new(move |href: String| path.set(href))
                        collapsible=collapse.get()
                        contained=true
                        content=content
                        header_slot=ViewFn::from(move || {
                            let hide = rail_header.get_untracked() == RailHeader::HideClass;
                            view! {
                                <div class="flex items-center gap-2">
                                    <LogoMark />
                                    <span
                                        class=if hide { "vdl-shell__hide-collapsed text-sm font-semibold" } else { "text-sm font-semibold" }
                                    >
                                        "Console Admin"
                                    </span>
                                </div>
                            }
                        })
                        header_collapsed_slot=(rail_header.get() == RailHeader::Slot)
                            .then(|| ViewFn::from(|| view! { <LogoMark /> }))
                        user_slot=ViewFn::from(move || {
                            view! {
                                <ShellUser name=user_name email=user_email>
                                    <ModeToggle variant=ButtonVariant::Outline size=Size::Small />
                                    <Button
                                        variant=ButtonVariant::Ghost
                                        size=Size::Small
                                        on_click=move |_| signed_in.update(|s| *s = !*s)
                                    >
                                        {move || if signed_in.get() { "Sign out" } else { "Signed out" }}
                                    </Button>
                                </ShellUser>
                            }
                        })
                        footer_slot=ViewFn::from(|| view! { "v1.4.2" })
                    >
                        <div
                            id="shell-page"
                            class="min-h-full space-y-3 bg-primary/10 p-4"
                        >
                            <h3 class="text-lg font-semibold">{move || path.get()}</h3>
                            <p class="text-sm text-muted-foreground">
                                "The page. Only the shell around it changes between layouts. Its own tinted background shows the framing."
                            </p>
                            <div class="h-9 rounded-md bg-muted"></div>
                            <div class="h-9 rounded-md bg-muted"></div>
                            <div class="h-9 w-2/3 rounded-md bg-muted"></div>
                        </div>
                    </AppShell>
                }
            }}
        </div>
    }
}

/// ShellUser with plain props, with signals, and with explicit initials (plain and signal).
#[component]
fn ShellUserDemo() -> impl IntoView {
    let name = RwSignal::new("Jane Doe".to_string());
    let email = RwSignal::new("jane@example.com".to_string());
    let initials = RwSignal::new(Some("JD".to_string()));
    view! {
        <Row label="Plain">
            <ShellUser name="Jane Doe" email="jane@example.com" />
            <ShellUser name="No Email" />
        </Row>
        <Row label="Signals">
            <div id="shell-user-reactive">
                <ShellUser name=name email=email />
            </div>
            <Button
                variant=ButtonVariant::Outline
                size=Size::Small
                on_click=move |_| name.set("Sam Rivera".to_string())
            >
                "Change name"
            </Button>
            <Button
                variant=ButtonVariant::Outline
                size=Size::Small
                on_click=move |_| email.set(String::new())
            >
                "Clear email"
            </Button>
        </Row>
        <Row label="Long name and email, with actions">
            <div id="shell-user-long" class="w-64 rounded-md border border-border p-2">
                <ShellUser
                    name="Alexandra Quincy-Smith the Third"
                    email="alexandra.quincy-smith@a-rather-long-company-domain.example.com"
                >
                    <ModeToggle variant=ButtonVariant::Outline size=Size::Small />
                    <Button variant=ButtonVariant::Ghost size=Size::Small>"Logout"</Button>
                    <Button variant=ButtonVariant::Ghost size=Size::Small>"Settings"</Button>
                </ShellUser>
            </div>
            <div class="w-64 rounded-md border border-border p-2">
                <ShellUser name="Jane Doe" email="jane@example.com">
                    <ModeToggle variant=ButtonVariant::Outline size=Size::Small />
                    <Button variant=ButtonVariant::Ghost size=Size::Small>"Logout"</Button>
                </ShellUser>
            </div>
        </Row>
        <Row label="inline_actions (icon buttons beside the name)">
            <div class="w-64 rounded-md border border-border p-2">
                <ShellUser name="Jane Doe" email="jane@example.com" inline_actions=true>
                    <Button
                        variant=ButtonVariant::Ghost
                        size=Size::Small
                        icon_only=true
                        aria_label="Sign out"
                    >
                        <Icon name=IconName::LogOut />
                    </Button>
                </ShellUser>
            </div>
            <div class="w-64 rounded-md border border-border p-2">
                <ShellUser
                    name="Alexandra Quincy-Smith the Third"
                    email="alexandra.quincy-smith@a-rather-long-company-domain.example.com"
                    inline_actions=true
                >
                    <Button
                        variant=ButtonVariant::Ghost
                        size=Size::Small
                        icon_only=true
                        aria_label="Sign out"
                    >
                        <Icon name=IconName::LogOut />
                    </Button>
                </ShellUser>
            </div>
        </Row>
        <Row label="Initials">
            <ShellUser name="Jane Doe" initials="XX" />
            <ShellUser name=name initials=initials />
            <Button
                variant=ButtonVariant::Outline
                size=Size::Small
                on_click=move |_| initials.set(None)
            >
                "Initials from the name"
            </Button>
        </Row>
    }
}

#[component]
pub fn AppShellsPage() -> impl IntoView {
    view! {
        <Page
            title="App shells"
            summary="One menu (three top-level pages, three products that repeat the same pages, an admin group) drawn by the real AppShell first, then laid out five ways by hand with plain components (no Sheet, Drawer or Sidebar). Click around; the content area just echoes where you are."
        >
            <Section
                title="AppShell: one menu, either layout"
                description="The real components: an AppNav (plain data: links, groups, icons, a badge, a permission signal) drawn by AppShell as a SidebarShell or a TopBarShell. The active page, the open group, the breadcrumb and the mobile menu (a sheet for the sidebar, a dropdown for the top bar; narrow the window) all come from the data and the current path. Switching layouts changes one prop."
            >
                <LiveShell />
            </Section>
            <Section
                title="ShellUser: plain or reactive"
                description="name, email and initials take plain values or signals. The initials follow the name unless given; the email line disappears while the email is empty. The avatar and text take the first row (a long value is cut with an ellipsis, its full text is the title); actions go on a row below and wrap."
            >
                <ShellUserDemo />
            </Section>
            <Section
                title="A. Persistent sidebar"
                description="The familiar layout without the collapse button: a fixed-width column, groups fold with Collapsible, active page highlighted. Best when people live in the app and the menu is the map."
            >
                <PersistentSidebar />
            </Section>
            <Section
                title="B. The same menu on a phone"
                description="Below the md breakpoint the sidebar becomes a top bar with one Menu button that opens a DropdownMenu. 20 items fit in one scrolling list, so no drawer is needed."
            >
                <PhoneDropdown />
            </Section>
            <Section
                title="C. Top bar"
                description="Pages as buttons, each product and Admin as a dropdown. Gives the page the full width; every page is two clicks away and nothing is hidden by default."
            >
                <TopBar />
            </Section>
            <Section
                title="D. Product menu and tabs"
                description="Alpha, Beta and QA all repeat GitHub, Jira and Servers, so choose the product once and keep a single row of tabs. The 20-item tree becomes a menu of three plus about five tabs. Admin is a separate area (reachable here from a menu if you add it)."
            >
                <ProductTabs />
            </Section>
            <Section
                title="E. Icon rail and page list"
                description="A narrow rail picks the group and a second column lists its pages. Always visible, two clicks, and it scales if more products arrive."
            >
                <RailAndPanel />
            </Section>
            <Section title="Which to use">
                <Row>
                    <p class="max-w-prose text-sm text-muted-foreground">
                        "For this menu A is the safest (it is what exists), D removes the most clutter because the products repeat, and B shows that the phone case needs no drawer. All three plus a command palette for jumping (when Command lands) cover the app."
                    </p>
                </Row>
            </Section>
        </Page>
    }
}
