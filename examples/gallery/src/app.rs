use crate::pages::PageHeadersPage;
use crate::pages::{
    AccordionsPage, AlertDialogsPage, AnimationsPage, AppShellsPage, AvatarsPage, BadgesPage,
    ButtonsPage, CalendarsPage, CarouselsPage, ChartsPage, ChartsPolarPage, CollapsiblesPage,
    ComboboxesPage, CommandsPage, ContextMenusPage, DataTablesMorePage, DataTablesPage,
    DialogsPage, DrawersPage, EmptyItemsPage, FieldsPage, FormsPage, GroupsPage, HoverCardsPage,
    IconsPage, LayoutPage, MenuStylesPage, MenubarsPage, MenusPage, MetersPage,
    NavigationMenusPage, NavigationPage, NumberFieldsPage, OtpPage, OverviewPage,
    PasswordInputsPage, PopoversPage, ProgressPage, ResizablesPage, ScrollAreasPage, SelectsPage,
    SheetsPage, SidebarsPage, SkeletonsMiscPage, SlidersPage, SwitchesPage, TablesPage, ThemesPage,
    TimeInputsPage, ToastsPage, TogglesPage, ToolbarsPage, TooltipsPage, TypographyPage,
};
use crate::registry::{Group, PAGES};
use crate::section::OCEAN_CSS;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{A, Route, Router, Routes};
use verdelle_ui::{
    Button, ButtonVariant, Icon, IconName, ModeToggle, Size, ThemeMode, ThemeProvider, ThemeStyle,
    ThemeSwitcher, theme_init_script,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <script inner_html=theme_init_script(
                verdelle_ui::themes::DEFAULT_THEME,
                ThemeMode::System,
                (None, None),
            )></script>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    // Shared state for verdelle-ui (the id counter). Provided here so each server request gets its own.
    verdelle_ui::provide_ui_context();
    // The theme is kept in a SyncedStorage that pretends to be a server (see server_demo.rs).
    // It starts empty on the server and in the browser's first render, so they agree; the
    // stored values "arrive" in an effect after hydration, like a fetch on login.
    let server = crate::server_demo::ServerDemo::new();
    provide_context(server.clone());
    Effect::new({
        let server = server.clone();
        move |_| server.load_stored()
    });
    view! {
        <Stylesheet id="leptos" href="/pkg/verdelle-gallery.css" />
        <Title text="Verdelle gallery" />
        // The head script (see `shell`) already applied the stored theme and mode, so the provider
        // does not print its own.
        <ThemeProvider script=false storage=server.storage.clone().into_arc()>
        // A theme declared in the page from shadcn-exported CSS: the switcher lists it with the
        // built-in ones.
        <ThemeStyle name="ocean" css=OCEAN_CSS />
        <Router>
            <div class="flex min-h-screen">
                <Sidebar />
                <main class="min-w-0 flex-1 p-4 pt-16 md:p-8">
                    <Routes fallback=|| view! { <p>"Page not found."</p> }>
                        <Route path=StaticSegment("") view=OverviewPage />
                        <Route path=StaticSegment("buttons") view=ButtonsPage />
                        <Route path=StaticSegment("calendars") view=CalendarsPage />
                        <Route path=StaticSegment("badges") view=BadgesPage />
                        <Route path=StaticSegment("forms") view=FormsPage />
                        <Route path=StaticSegment("tables") view=TablesPage />
                        <Route path=StaticSegment("data-tables") view=DataTablesPage />
                        <Route path=StaticSegment("data-tables-layout") view=DataTablesMorePage />
                        <Route path=StaticSegment("layout") view=LayoutPage />
                        <Route path=StaticSegment("page-headers") view=PageHeadersPage />
                        <Route path=StaticSegment("animations") view=AnimationsPage />
                        <Route path=StaticSegment("app-shells") view=AppShellsPage />
                        <Route path=StaticSegment("switches") view=SwitchesPage />
                        <Route path=StaticSegment("accordions") view=AccordionsPage />
                        <Route path=StaticSegment("dialogs") view=DialogsPage />
                        <Route path=StaticSegment("selects") view=SelectsPage />
                        <Route path=StaticSegment("comboboxes") view=ComboboxesPage />
                        <Route path=StaticSegment("commands") view=CommandsPage />
                        <Route path=StaticSegment("alert-dialogs") view=AlertDialogsPage />
                        <Route path=StaticSegment("sheets") view=SheetsPage />
                        <Route path=StaticSegment("drawers") view=DrawersPage />
                        <Route path=StaticSegment("sidebars") view=SidebarsPage />
                        <Route path=StaticSegment("avatars") view=AvatarsPage />
                        <Route path=StaticSegment("progress") view=ProgressPage />
                        <Route path=StaticSegment("collapsibles") view=CollapsiblesPage />
                        <Route path=StaticSegment("scroll-areas") view=ScrollAreasPage />
                        <Route path=StaticSegment("carousels") view=CarouselsPage />
                        <Route path=StaticSegment("popovers") view=PopoversPage />
                        <Route path=StaticSegment("menus") view=MenusPage />
                        <Route path=StaticSegment("context-menus") view=ContextMenusPage />
                        <Route path=StaticSegment("menubars") view=MenubarsPage />
                        <Route path=StaticSegment("menu-styles") view=MenuStylesPage />
                        <Route path=StaticSegment("meters") view=MetersPage />
                        <Route path=StaticSegment("resizables") view=ResizablesPage />
                        <Route path=StaticSegment("charts") view=ChartsPage />
                        <Route path=StaticSegment("charts-polar") view=ChartsPolarPage />
                        <Route path=StaticSegment("typography") view=TypographyPage />
                        <Route path=StaticSegment("number-fields") view=NumberFieldsPage />
                        <Route path=StaticSegment("time-inputs") view=TimeInputsPage />
                        <Route path=StaticSegment("password-inputs") view=PasswordInputsPage />
                        <Route path=StaticSegment("toolbars") view=ToolbarsPage />
                        <Route path=StaticSegment("tooltips") view=TooltipsPage />
                        <Route path=StaticSegment("toasts") view=ToastsPage />
                        <Route path=StaticSegment("hover-cards") view=HoverCardsPage />
                        <Route path=StaticSegment("navigation-menus") view=NavigationMenusPage />
                        <Route path=StaticSegment("skeletons-misc") view=SkeletonsMiscPage />
                        <Route path=StaticSegment("empty-items") view=EmptyItemsPage />
                        <Route path=StaticSegment("navigation") view=NavigationPage />
                        <Route path=StaticSegment("groups") view=GroupsPage />
                        <Route path=StaticSegment("icons") view=IconsPage />
                        <Route path=StaticSegment("toggles") view=TogglesPage />
                        <Route path=StaticSegment("sliders") view=SlidersPage />
                        <Route path=StaticSegment("otp") view=OtpPage />
                        <Route path=StaticSegment("fields") view=FieldsPage />
                        <Route path=StaticSegment("themes") view=ThemesPage />
                    </Routes>
                </main>
            </div>
        </Router>
        </ThemeProvider>
    }
}

#[component]
fn NavLink(path: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=path
            exact=true
            attr:class="rounded-md px-3 py-1 max-md:py-2 text-sm hover:bg-sidebar-accent aria-[current=page]:bg-sidebar-accent aria-[current=page]:font-medium"
        >
            {label}
        </A>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    // Below the md breakpoint the sidebar is a drawer, opened from the top bar.
    let open = RwSignal::new(false);

    view! {
        <header class="fixed inset-x-0 top-0 z-30 flex h-12 items-center gap-2 border-b bg-background px-3 md:hidden">
            <Button
                variant=ButtonVariant::Ghost
                size=Size::Small
                icon_only=true
                aria_label="Open navigation"
                on_click=move |_| open.set(true)
            >
                <Icon name=IconName::Menu />
            </Button>
            <span class="font-semibold">"Verdelle"</span>
        </header>
        <div
            class="fixed inset-0 z-30 bg-black/50 md:hidden"
            class:hidden=move || !open.get()
            on:click=move |_| open.set(false)
        ></div>
        <aside
            class="fixed inset-y-0 left-0 z-40 flex w-60 shrink-0 flex-col gap-6 border-r bg-sidebar p-4 text-sidebar-foreground transition-transform md:sticky md:top-0 md:h-screen md:translate-x-0"
            class=("-translate-x-full", move || !open.get())
            class=("max-md:invisible", move || !open.get())
            on:keydown=move |ev| {
                if ev.key() == "Escape" {
                    open.set(false);
                }
            }
        >
            <div class="flex items-start justify-between">
                <div>
                    <div class="text-lg font-semibold">"Verdelle"</div>
                    <div class="text-xs text-muted-foreground">"component gallery"</div>
                </div>
                <div class="md:hidden">
                    <Button
                        variant=ButtonVariant::Ghost
                        size=Size::Small
                        icon_only=true
                        aria_label="Close navigation"
                        on_click=move |_| open.set(false)
                    >
                        <Icon name=IconName::X />
                    </Button>
                </div>
            </div>
            <nav
                class="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto"
                aria-label="Components"
                on:click=move |_| open.set(false)
            >
                <NavLink path="/" label="Overview" />
                {Group::ALL
                    .into_iter()
                    .map(|group| {
                        view! {
                            <div class="flex flex-col gap-1">
                                <div class="px-3 text-xs font-medium text-muted-foreground">{group.title()}</div>
                                {PAGES
                                    .iter()
                                    .filter(|page| page.group == group)
                                    .map(|page| view! { <NavLink path=page.path label=page.title /> })
                                    .collect_view()}
                            </div>
                        }
                    })
                    .collect_view()}
            </nav>
            <div class="flex items-center gap-2">
                <ThemeSwitcher class="min-w-0 flex-1" size=Size::Small />
                <ModeToggle variant=ButtonVariant::Outline size=Size::Small />
            </div>
        </aside>
    }
}
