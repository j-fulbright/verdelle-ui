use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Avatar, AvatarFallback, Breadcrumb, BreadcrumbItem, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, Button, ButtonVariant, Card, CardContent, CardHeader, CardTitle,
    Separator, Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup,
    SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInput,
    SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton,
    SidebarMenuButtonVariant, SidebarMenuItem, SidebarMenuSkeleton, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSeparator,
    SidebarSide, SidebarTrigger, SidebarVariant, Size, Skeleton, use_sidebar,
};

/// A 16px stroke icon from one or more paths (Lucide shapes), for the examples.
#[component]
fn Glyph(paths: &'static [&'static str]) -> impl IntoView {
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

const SERVER: &[&str] = &[
    "M2 4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2z",
    "M2 14a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2z",
    "M6 6h.01",
    "M6 18h.01",
];
const BRANCH: &[&str] = &[
    "M6 3v12",
    "M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6",
    "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6",
    "M18 9a9 9 0 0 1-9 9",
];
const HOME: &[&str] = &[
    "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8",
    "M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z",
];
const USERS: &[&str] = &[
    "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2",
    "M9 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8",
    "M22 21v-2a4 4 0 0 0-3-3.87",
    "M16 3.13a4 4 0 0 1 0 7.75",
];
const SETTINGS: &[&str] = &[
    "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6",
    "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z",
];
const PLUS: &[&str] = &["M5 12h14", "M12 5v14"];
const MORE: &[&str] = &[
    "M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2",
    "M19 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2",
    "M5 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2",
];
const UPDOWN: &[&str] = &["m7 15 5 5 5-5", "m7 9 5-5 5 5"];
const CHEVRON: &[&str] = &["m9 18 6-6-6-6"];

/// The sidebar's contents, as an app like an operations console would have them.
#[component]
fn AppNav(active: RwSignal<&'static str>) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let item = move |id: &'static str| Signal::derive(move || active.get() == id);

    view! {
        <SidebarHeader>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton size=Size::Large tooltip="Ops Console">
                        <span class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary text-primary-foreground">
                            <Glyph paths=SERVER />
                        </span>
                        <span class="grid flex-1 text-left text-sm leading-tight">
                            <span class="truncate font-medium">"Ops Console"</span>
                            <span class="truncate text-xs">"Production"</span>
                        </span>
                        <Glyph paths=UPDOWN />
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
            <SidebarInput value=query placeholder="Search..." />
        </SidebarHeader>
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupLabel>"Platform"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                tooltip="Home"
                                is_active=item("home")
                                on_click=move |_| active.set("home")
                            >
                                <Glyph paths=HOME />
                                <span>"Home"</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                tooltip="Servers"
                                is_active=item("servers")
                                on_click=move |_| active.set("servers")
                            >
                                <Glyph paths=SERVER />
                                <span>"Servers"</span>
                            </SidebarMenuButton>
                            <SidebarMenuBadge>"24"</SidebarMenuBadge>
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                tooltip="Workflows"
                                is_active=item("workflows")
                                on_click=move |_| active.set("workflows")
                            >
                                <Glyph paths=BRANCH />
                                <span>"Workflows"</span>
                            </SidebarMenuButton>
                            <SidebarMenuAction show_on_hover=true aria_label="More">
                                <Glyph paths=MORE />
                            </SidebarMenuAction>
                            <SidebarMenuSub>
                                <SidebarMenuSubItem>
                                    <SidebarMenuSubButton
                                        is_active=item("runs")
                                        on_click=move |_| active.set("runs")
                                    >
                                        <span>"Runs"</span>
                                    </SidebarMenuSubButton>
                                </SidebarMenuSubItem>
                                <SidebarMenuSubItem>
                                    <SidebarMenuSubButton
                                        is_active=item("schedules")
                                        on_click=move |_| active.set("schedules")
                                    >
                                        <span>"Schedules"</span>
                                    </SidebarMenuSubButton>
                                </SidebarMenuSubItem>
                            </SidebarMenuSub>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </SidebarGroup>
            <SidebarSeparator />
            <SidebarGroup>
                <SidebarGroupLabel>"Admin"</SidebarGroupLabel>
                <SidebarGroupAction aria_label="Add a team">
                    <Glyph paths=PLUS />
                </SidebarGroupAction>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                tooltip="Teams"
                                is_active=item("teams")
                                on_click=move |_| active.set("teams")
                            >
                                <Glyph paths=USERS />
                                <span>"Teams"</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                            <SidebarMenuButton
                                tooltip="Settings"
                                is_active=item("settings")
                                on_click=move |_| active.set("settings")
                            >
                                <Glyph paths=SETTINGS />
                                <span>"Settings"</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarContent>
        <SidebarFooter>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton size=Size::Large tooltip="Ada Lovelace">
                        <Avatar>
                            <AvatarFallback>"AL"</AvatarFallback>
                        </Avatar>
                        <span class="grid flex-1 text-left text-sm leading-tight">
                            <span class="truncate font-medium">"Ada Lovelace"</span>
                            <span class="truncate text-xs">"ada@example.com"</span>
                        </span>
                        <Glyph paths=UPDOWN />
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarFooter>
        <SidebarRail />
    }
}

/// The page next to the sidebar: a top bar with the trigger and a breadcrumb, and some content.
#[component]
fn AppPage(active: RwSignal<&'static str>) -> impl IntoView {
    let ctx = use_sidebar();
    view! {
        <header class="flex h-12 shrink-0 items-center gap-2 border-b px-4">
            <SidebarTrigger />
            <Separator orientation=verdelle_ui::Orientation::Vertical class="h-4" />
            <Breadcrumb>
                <BreadcrumbList>
                    <BreadcrumbItem>"Ops Console"</BreadcrumbItem>
                    <BreadcrumbSeparator />
                    <BreadcrumbItem>
                        <BreadcrumbPage>{move || active.get()}</BreadcrumbPage>
                    </BreadcrumbItem>
                </BreadcrumbList>
            </Breadcrumb>
        </header>
        <div class="grid flex-1 content-start gap-4 p-4 sm:grid-cols-3">
            <Card>
                <CardHeader>
                    <CardTitle>"Sidebar state"</CardTitle>
                </CardHeader>
                <CardContent>
                    <p class="text-sm text-muted-foreground">
                        {move || ctx.state()} " - Ctrl or Cmd + B toggles it."
                    </p>
                </CardContent>
            </Card>
            <Skeleton class="h-24" />
            <Skeleton class="h-24" />
            <Skeleton class="h-40 sm:col-span-3" />
        </div>
    }
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

#[component]
pub fn SidebarsPage() -> impl IntoView {
    let variant = RwSignal::new(SidebarVariant::Sidebar);
    let side = RwSignal::new(SidebarSide::Left);
    let collapsible = RwSignal::new(SidebarCollapsible::Icon);
    let active = RwSignal::new("servers");

    let controlled = RwSignal::new(true);
    let menu_query = RwSignal::new(String::new());
    let plain_menu = RwSignal::new("home");

    view! {
        <Page
            title="Sidebar"
            summary="A navigation panel next to the page. Below 768px it turns into a Sheet; shrink the window to see it. The demos are contained in boxes here (the `contained` prop); in an app the sidebar is fixed to the window."
        >
            <Section
                title="Application layout"
                description="Header with a switcher and a search field, grouped menus (an active page, a count, a hover action, a sub menu, a group action), a footer with the user, and a rail on the edge. The trigger, the rail, the tooltips on the collapsed icons and Ctrl/Cmd + B all work. Changing an option rebuilds the demo."
            >
                <div class="space-y-3">
                    <Row label="Variant">
                        <Choice value=variant this=SidebarVariant::Sidebar label="sidebar" />
                        <Choice value=variant this=SidebarVariant::Floating label="floating" />
                        <Choice value=variant this=SidebarVariant::Inset label="inset" />
                    </Row>
                    <Row label="Side">
                        <Choice value=side this=SidebarSide::Left label="left" />
                        <Choice value=side this=SidebarSide::Right label="right" />
                    </Row>
                    <Row label="Collapsible">
                        <Choice value=collapsible this=SidebarCollapsible::Offcanvas label="offcanvas" />
                        <Choice value=collapsible this=SidebarCollapsible::Icon label="icon" />
                        <Choice value=collapsible this=SidebarCollapsible::None label="none" />
                    </Row>
                </div>
                {move || {
                    let (variant, side, collapsible) = (variant.get(), side.get(), collapsible.get());
                    // The sidebar's gap takes its place in the row, so a right-hand sidebar goes
                    // after the page in the source. (As in shadcn, the inset page's card styling
                    // only applies when the sidebar comes first, so `Inset` is for the left.)
                    if side == SidebarSide::Right {
                        view! {
                            <SidebarProvider contained=true persist=false class="h-[560px] rounded-lg border">
                                <SidebarInset>
                                    <AppPage active=active />
                                </SidebarInset>
                                <Sidebar variant=variant side=side collapsible=collapsible>
                                    <AppNav active=active />
                                </Sidebar>
                            </SidebarProvider>
                        }
                            .into_any()
                    } else {
                        view! {
                            <SidebarProvider contained=true persist=false class="h-[560px] rounded-lg border">
                                <Sidebar variant=variant side=side collapsible=collapsible>
                                    <AppNav active=active />
                                </Sidebar>
                                <SidebarInset>
                                    <AppPage active=active />
                                </SidebarInset>
                            </SidebarProvider>
                        }
                            .into_any()
                    }
                }}
            </Section>

            <Section
                title="Menu button options"
                description="Variant default and outline, sizes small / default / large, the active page, disabled, a link (href renders an anchor), a badge, a hover action and loading skeletons. This sidebar never collapses (collapsible none)."
            >
                <SidebarProvider contained=true persist=false keyboard_shortcut="" class="h-[520px] rounded-lg border">
                    <Sidebar collapsible=SidebarCollapsible::None>
                        <SidebarHeader>
                            <SidebarInput value=menu_query placeholder="Filter..." />
                        </SidebarHeader>
                        <SidebarContent>
                            <SidebarGroup>
                                <SidebarGroupLabel>"Sizes"</SidebarGroupLabel>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton size=Size::Small>
                                            <Glyph paths=HOME />
                                            <span>"Small"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            <Glyph paths=HOME />
                                            <span>"Default"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton size=Size::Large>
                                            <Glyph paths=HOME />
                                            <span>"Large"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroup>
                            <SidebarGroup>
                                <SidebarGroupLabel>"States and variants"</SidebarGroupLabel>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton
                                            is_active=Signal::derive(move || plain_menu.get() == "home")
                                            on_click=move |_| plain_menu.set("home")
                                        >
                                            <Glyph paths=HOME />
                                            <span>"Active (click to move)"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton
                                            is_active=Signal::derive(move || plain_menu.get() == "other")
                                            on_click=move |_| plain_menu.set("other")
                                        >
                                            <Glyph paths=SERVER />
                                            <span>"Second item"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton variant=SidebarMenuButtonVariant::Outline>
                                            <Glyph paths=BRANCH />
                                            <span>"Outline"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton disabled=true>
                                            <Glyph paths=USERS />
                                            <span>"Disabled"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton href="#sidebar-link">
                                            <Glyph paths=CHEVRON />
                                            <span>"A link (href)"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            <Glyph paths=SETTINGS />
                                            <span>"With a badge"</span>
                                        </SidebarMenuButton>
                                        <SidebarMenuBadge>"12"</SidebarMenuBadge>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            <Glyph paths=SETTINGS />
                                            <span>"Action on hover"</span>
                                        </SidebarMenuButton>
                                        <SidebarMenuAction show_on_hover=true aria_label="More">
                                            <Glyph paths=MORE />
                                        </SidebarMenuAction>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            <Glyph paths=PLUS />
                                            <span>"Action always shown"</span>
                                        </SidebarMenuButton>
                                        <SidebarMenuAction aria_label="Add">
                                            <Glyph paths=PLUS />
                                        </SidebarMenuAction>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroup>
                            <SidebarGroup>
                                <SidebarGroupLabel>"Loading"</SidebarGroupLabel>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuSkeleton show_icon=true width=80 />
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuSkeleton show_icon=true width=55 />
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuSkeleton width=68 />
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroup>
                            <SidebarGroup>
                                <SidebarGroupLabel>"Sub menu sizes"</SidebarGroupLabel>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>
                                            <Glyph paths=BRANCH />
                                            <span>"Parent"</span>
                                        </SidebarMenuButton>
                                        <SidebarMenuSub>
                                            <SidebarMenuSubItem>
                                                <SidebarMenuSubButton href="#medium" is_active=true>
                                                    <span>"Medium (default)"</span>
                                                </SidebarMenuSubButton>
                                            </SidebarMenuSubItem>
                                            <SidebarMenuSubItem>
                                                <SidebarMenuSubButton href="#small" size=Size::Small>
                                                    <span>"Small"</span>
                                                </SidebarMenuSubButton>
                                            </SidebarMenuSubItem>
                                        </SidebarMenuSub>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroup>
                        </SidebarContent>
                        <SidebarFooter>
                            <p class="px-2 text-xs text-muted-foreground">"v1.0.0"</p>
                        </SidebarFooter>
                    </Sidebar>
                    <SidebarInset>
                        <div class="p-4 text-sm text-muted-foreground">"The page."</div>
                    </SidebarInset>
                </SidebarProvider>
            </Section>

            <Section
                title="Controlled state"
                description="Pass a signal as `open` to own the state (and its storage). Without it the provider keeps its own and remembers it in the sidebar_state cookie (persist). The buttons here set the signal; the trigger and Ctrl/Cmd + B set it too."
            >
                <Row>
                    <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| controlled.set(true)>
                        "Expand"
                    </Button>
                    <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| controlled.set(false)>
                        "Collapse"
                    </Button>
                    <span class="text-sm text-muted-foreground">
                        "open = " {move || controlled.get().to_string()}
                    </span>
                </Row>
                <SidebarProvider
                    contained=true
                    open=controlled
                    keyboard_shortcut=""
                    class="h-60 rounded-lg border"
                >
                    <Sidebar collapsible=SidebarCollapsible::Icon>
                        <SidebarContent>
                            <SidebarGroup>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton tooltip="Home">
                                            <Glyph paths=HOME />
                                            <span>"Home"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton tooltip="Servers">
                                            <Glyph paths=SERVER />
                                            <span>"Servers"</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroup>
                        </SidebarContent>
                    </Sidebar>
                    <SidebarInset>
                        <div class="flex items-center gap-2 p-2">
                            <SidebarTrigger />
                            <span class="text-sm text-muted-foreground">"Collapsed to icons; hover one for its tooltip."</span>
                        </div>
                    </SidebarInset>
                </SidebarProvider>
            </Section>
        </Page>
    }
}
