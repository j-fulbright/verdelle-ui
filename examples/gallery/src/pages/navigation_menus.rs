use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Icon, IconName, NavigationMenu, NavigationMenuContent,
    NavigationMenuIndicator, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuTrigger, Orientation, Sheet, SheetHeader, SheetTitle, Side, Size,
};

fn now_ms() -> f64 {
    leptos::web_sys::js_sys::Date::now()
}

#[component]
pub fn NavigationMenusPage() -> impl IntoView {
    view! {
        <Page
            title="Navigation menu"
            summary="A site navigation bar whose triggers open panels of links. Hover a trigger (it waits 200ms, then glides along the bar at once), click it, or use the keyboard: Left/Right move along the bar, Down opens a panel and moves into it, Escape closes it."
        >
            <BasicSection />
            <NoViewportSection />
            <IndicatorSection />
            <ControlledSection />
            <VerticalSection />
            <NarrowSection />
            <PhoneSection />
        </Page>
    }
}

/// A link with a title and a one-line description, as in shadcn's examples.
#[component]
fn ListLink(
    href: &'static str,
    title: &'static str,
    description: &'static str,
    #[prop(optional)] active: bool,
) -> impl IntoView {
    view! {
        <li>
            <NavigationMenuLink href=href active=active>
                <div class="vdl-nav__link-title">{title}</div>
                <div class="vdl-nav__link-description">{description}</div>
            </NavigationMenuLink>
        </li>
    }
}

#[component]
fn ComponentsPanel() -> impl IntoView {
    view! {
        <ul class="grid gap-2" style="width:min(500px, calc(100vw - 48px));grid-template-columns:repeat(auto-fit, minmax(200px, 1fr));list-style:none;margin:0;padding:0">
            <ListLink
                href="#alert-dialog"
                title="Alert Dialog"
                description="A modal dialog that interrupts the user with important content and expects a response."
            />
            <ListLink
                href="#hover-card"
                title="Hover Card"
                description="For sighted users to preview content available behind a link."
            />
            <ListLink
                href="#progress"
                title="Progress"
                description="Displays an indicator showing the completion progress of a task."
            />
            <ListLink
                href="#scroll-area"
                title="Scroll-area"
                description="Visually or semantically separates content."
            />
            <ListLink href="#tabs" title="Tabs" description="A set of layered sections of content." />
            <ListLink
                href="#tooltip"
                title="Tooltip"
                description="A popup that displays information related to an element."
            />
        </ul>
    }
}

#[component]
fn GettingStartedPanel() -> impl IntoView {
    view! {
        <ul class="grid gap-2" style="width:min(400px, calc(100vw - 48px));list-style:none;margin:0;padding:0">
            <ListLink
                href="#intro"
                title="Introduction"
                description="Re-usable components built with Verdelle, on headless primitives."
            />
            <ListLink
                href="#install"
                title="Installation"
                description="How to install dependencies and structure your app."
            />
            <ListLink
                href="#theming"
                title="Theming"
                description="Paste a shadcn theme; switch theme and mode independently."
            />
        </ul>
    }
}

#[component]
fn BasicSection() -> impl IntoView {
    view! {
        <Section
            title="Basic"
            description="shadcn's example: two triggers share one viewport that resizes and slides between them, with a plain link in the bar. Move from one trigger to the other: the second panel slides in from the side you came from."
        >
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem value="getting-started">
                        <NavigationMenuTrigger>"Getting started"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <GettingStartedPanel />
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="components">
                        <NavigationMenuTrigger>"Components"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ComponentsPanel />
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="compact">
                        <NavigationMenuTrigger>"Compact"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-2" style="width:200px;list-style:none;margin:0;padding:0">
                                <ListLink href="#a" title="Short" description="A small panel." />
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem>
                        <NavigationMenuLink href="#docs" trigger_style=true>"Docs"</NavigationMenuLink>
                    </NavigationMenuItem>
                    <NavigationMenuItem>
                        <NavigationMenuLink href="#current" trigger_style=true active=true>"Current page"</NavigationMenuLink>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="disabled">
                        <NavigationMenuTrigger disabled=true>"Disabled"</NavigationMenuTrigger>
                        <NavigationMenuContent>"Never shown"</NavigationMenuContent>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </Section>
    }
}

#[component]
fn NoViewportSection() -> impl IntoView {
    view! {
        <Section
            title="Without a viewport"
            description="viewport=false: each panel hangs under its own item as a popover-styled box. Leave room below, panels overlap the page."
        >
            <NavigationMenu viewport=false label="No viewport">
                <NavigationMenuList>
                    <NavigationMenuItem value="one">
                        <NavigationMenuTrigger>"Learn"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <GettingStartedPanel />
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="two">
                        <NavigationMenuTrigger>"Reference"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-1" style="width:220px;list-style:none;margin:0;padding:0">
                                <li><NavigationMenuLink href="#r1">"Components"</NavigationMenuLink></li>
                                <li><NavigationMenuLink href="#r2">"Primitives"</NavigationMenuLink></li>
                                <li><NavigationMenuLink href="#r3">"Tokens"</NavigationMenuLink></li>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem>
                        <NavigationMenuLink href="#blog" trigger_style=true>"Blog"</NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
            <div style="height:150px"></div>
        </Section>
    }
}

#[component]
fn IndicatorSection() -> impl IntoView {
    view! {
        <Section
            title="Indicator"
            description="NavigationMenuIndicator is an arrow under the bar that slides to the open trigger and points at the panel."
        >
            <NavigationMenu label="With indicator">
                <NavigationMenuList>
                    <NavigationMenuItem value="p">
                        <NavigationMenuTrigger>"Product"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <GettingStartedPanel />
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="s">
                        <NavigationMenuTrigger>"Solutions"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ComponentsPanel />
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                </NavigationMenuList>
                <NavigationMenuIndicator />
            </NavigationMenu>
        </Section>
    }
}

#[component]
fn ControlledSection() -> impl IntoView {
    let value = RwSignal::new(String::new());
    let log = RwSignal::new(Vec::<String>::new());
    let started = RwSignal::new(0.0_f64);
    let push = move |line: String| {
        log.update(|l| {
            l.insert(0, line);
            l.truncate(6);
        })
    };

    view! {
        <Section
            title="Controlled, delays"
            description="value and on_value_change drive the menu (the value is an empty string when closed). This bar waits 600ms before opening a panel and skips the wait for 1000ms after one closes; the log shows how long each change took after the pointer arrived on the bar."
        >
            <div
                class="space-y-3"
                on:pointerenter=move |_| started.set(now_ms())
            >
                <NavigationMenu
                    value=value
                    on_value_change=move |v: String| {
                        value.set(v.clone());
                        let waited = (now_ms() - started.get_untracked()).round();
                        push(format!("value = {:?} at {waited} ms", v));
                    }
                    delay_duration=600
                    skip_delay_duration=1000
                    label="Controlled"
                >
                    <NavigationMenuList>
                        <NavigationMenuItem value="alpha">
                            <NavigationMenuTrigger>"Alpha"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul style="width:240px;list-style:none;margin:0;padding:0">
                                    <li><NavigationMenuLink href="#a1">"Alpha one"</NavigationMenuLink></li>
                                    <li><NavigationMenuLink href="#a2">"Alpha two"</NavigationMenuLink></li>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                        <NavigationMenuItem value="beta">
                            <NavigationMenuTrigger>"Beta"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul style="width:320px;list-style:none;margin:0;padding:0">
                                    <li><NavigationMenuLink href="#b1">"Beta one"</NavigationMenuLink></li>
                                    <li><NavigationMenuLink href="#b2">"Beta two"</NavigationMenuLink></li>
                                    <li><NavigationMenuLink href="#b3">"Beta three"</NavigationMenuLink></li>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                        <NavigationMenuItem value="gamma">
                            <NavigationMenuTrigger>"Gamma"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul style="width:200px;list-style:none;margin:0;padding:0">
                                    <li><NavigationMenuLink href="#g1">"Gamma one"</NavigationMenuLink></li>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
                <div class="flex flex-wrap items-center gap-2">
                    <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| value.set("alpha".into())>"Open alpha"</Button>
                    <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| value.set("gamma".into())>"Open gamma"</Button>
                    <Button variant=ButtonVariant::Ghost size=Size::Small on_click=move |_| value.set(String::new())>"Close"</Button>
                    <span class="text-sm text-muted-foreground">
                        "value = " <span id="nav-value">{move || format!("{:?}", value.get())}</span>
                    </span>
                </div>
                <ul id="nav-log" class="min-h-28 text-sm text-muted-foreground">
                    {move || log.get().into_iter().map(|l| view! { <li>{l}</li> }).collect_view()}
                </ul>
                <div style="height:120px"></div>
            </div>
        </Section>
    }
}

#[component]
fn VerticalSection() -> impl IntoView {
    view! {
        <Section
            title="Vertical"
            description="orientation=Vertical: Up/Down move along the list and Right opens a panel, which appears beside the list. (The panel is not aligned to its trigger in this orientation.)"
        >
            <NavigationMenu orientation=Orientation::Vertical label="Vertical">
                <NavigationMenuList>
                    <NavigationMenuItem value="v1">
                        <NavigationMenuTrigger>"First"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul style="width:200px;list-style:none;margin:0;padding:0">
                                <li><NavigationMenuLink href="#v1a">"First one"</NavigationMenuLink></li>
                                <li><NavigationMenuLink href="#v1b">"First two"</NavigationMenuLink></li>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="v2">
                        <NavigationMenuTrigger>"Second"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul style="width:200px;list-style:none;margin:0;padding:0">
                                <li><NavigationMenuLink href="#v2a">"Second one"</NavigationMenuLink></li>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem>
                        <NavigationMenuLink href="#v3" trigger_style=true>"Third (link)"</NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
            <div style="height:100px"></div>
        </Section>
    }
}

#[component]
fn NarrowSection() -> impl IntoView {
    view! {
        <Section
            title="Narrow screens"
            description="Panels size themselves with min(width, 100vw - 48px) and the viewport slides back inside the window (8px clear) when its trigger is near the edge. Make the window narrow (or open this page on a phone) and open the last trigger of the bar below, which sits at the far right."
        >
            <div class="flex justify-end">
                <NavigationMenu label="Right aligned">
                    <NavigationMenuList>
                        <NavigationMenuItem value="n1">
                            <NavigationMenuTrigger>"Menu"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ComponentsPanel />
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                        <NavigationMenuItem value="n2">
                            <NavigationMenuTrigger>"Far right"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <GettingStartedPanel />
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
            <div style="height:260px"></div>
        </Section>
    }
}

/// The recommended phone pattern: a bar of triggers does not fit a phone, so below `sm` the bar is
/// replaced by a menu button that opens the same links in a left Sheet.
#[component]
fn PhoneSection() -> impl IntoView {
    let open = RwSignal::new(false);
    let links: [(&str, &str); 6] = [
        ("#p-intro", "Introduction"),
        ("#p-install", "Installation"),
        ("#p-theming", "Theming"),
        ("#p-components", "Components"),
        ("#p-docs", "Documentation"),
        ("#p-blog", "Blog"),
    ];
    view! {
        <Section
            title="On a phone"
            description="A bar of six triggers is wider than a phone. The bar wraps rather than clipping, but the pattern to use is to swap it below sm for a menu button that opens a Sheet with the same links (shown here: resize the window under 640px, or open the page on a phone)."
        >
            <div class="max-sm:hidden">
                <NavigationMenu label="Wide">
                    <NavigationMenuList>
                        {links
                            .into_iter()
                            .map(|(href, text)| {
                                view! {
                                    <NavigationMenuItem>
                                        <NavigationMenuLink href=href trigger_style=true>
                                            {text}
                                        </NavigationMenuLink>
                                    </NavigationMenuItem>
                                }
                            })
                            .collect_view()}
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
            <div class="sm:hidden">
                <Button
                    variant=ButtonVariant::Outline
                    icon_only=true
                    aria_label="Open menu"
                    on_click=move |_| open.set(true)
                >
                    <Icon name=IconName::Menu />
                </Button>
                <Sheet open=open label="Menu" side=Side::Left>
                    <SheetHeader>
                        <SheetTitle>"Menu"</SheetTitle>
                    </SheetHeader>
                    <nav aria-label="Menu" class="flex flex-col gap-1 px-4">
                        {links
                            .into_iter()
                            .map(|(href, text)| {
                                view! {
                                    <a
                                        href=href
                                        class="rounded-md px-3 py-2 text-sm font-medium hover:bg-muted"
                                        on:click=move |_| open.set(false)
                                    >
                                        {text}
                                    </a>
                                }
                            })
                            .collect_view()}
                    </nav>
                </Sheet>
            </div>
        </Section>
    }
}
