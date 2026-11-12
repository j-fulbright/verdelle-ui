use crate::registry::{Group, PAGES};
use crate::section::{Page, Section};
use leptos::prelude::*;
use leptos_router::components::A;
use verdelle_ui::{ThemePicker, use_theme};

#[component]
pub fn OverviewPage() -> impl IntoView {
    let theme = use_theme();

    view! {
        <Page
            title="Verdelle"
            summary="shadcn/ui components for Leptos, on Base UI-style headless primitives. Every component renders here, under the theme and mode you pick."
        >
            <Section
                title="Theme"
                description="A theme is a shadcn-format CSS file in crates/verdelle-ui/themes/. Light or dark is separate; use the button in the sidebar."
            >
                <ThemePicker value=theme.theme />
            </Section>
            {Group::ALL
                .into_iter()
                .map(|group| {
                    let pages = PAGES.iter().filter(|page| page.group == group).collect::<Vec<_>>();
                    view! {
                        <Section title=group.title() description=format!("{} pages", pages.len())>
                            <ul class="grid gap-3 sm:grid-cols-2">
                                {pages
                                    .into_iter()
                                    .map(|page| {
                                        view! {
                                            <li>
                                                <A
                                                    href=page.path
                                                    attr:class="block rounded-lg border p-3 hover:bg-accent"
                                                >
                                                    <span class="block text-sm font-medium">{page.title}</span>
                                                    <span class="block text-xs text-muted-foreground">{page.shows}</span>
                                                </A>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </Section>
                    }
                })
                .collect_view()}
        </Page>
    }
}
