use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{ScrollArea, ScrollAreaOrientation, ScrollAreaType, Separator};

#[component]
fn Tags(count: u32) -> impl IntoView {
    (1..=count)
        .map(|i| {
            view! {
                <div class="text-sm">{format!("v1.2.0-beta.{}", count + 1 - i)}</div>
                <Separator class="my-2" />
            }
        })
        .collect_view()
}

#[component]
fn Tile(index: u32) -> impl IntoView {
    view! {
        <div
            class="flex h-24 w-32 shrink-0 items-center justify-center rounded-md bg-muted text-sm text-muted-foreground"
        >
            {format!("Tile {index}")}
        </div>
    }
}

#[component]
pub fn ScrollAreasPage() -> impl IntoView {
    view! {
        <Page
            title="Scroll area"
            summary="Native scrolling with themed bars. The browser's own bars are hidden and a bar and thumb are drawn from the measured sizes; the viewport takes focus, so arrow keys, Page Up/Down and Home/End scroll it."
        >
            <Section title="Vertical" description="shadcn's default. Bars shown always, so they are visible without hovering.">
                <ScrollArea r#type=ScrollAreaType::Always class="h-72 w-48 rounded-md border border-border">
                    <div class="p-4">
                        <h4 class="mb-4 text-sm font-medium leading-none">"Tags"</h4>
                        <Tags count=50 />
                    </div>
                </ScrollArea>
            </Section>

            <Section title="Horizontal" description="orientation=Horizontal; the row does not wrap.">
                <ScrollArea
                    orientation=ScrollAreaOrientation::Horizontal
                    r#type=ScrollAreaType::Always
                    class="w-96 max-w-full whitespace-nowrap rounded-md border border-border"
                >
                    <div class="flex w-max gap-4 p-4">
                        {(1..=12).map(|i| view! { <Tile index=i /> }).collect_view()}
                    </div>
                </ScrollArea>
            </Section>

            <Section title="Both" description="Both bars and the corner where they meet.">
                <ScrollArea
                    orientation=ScrollAreaOrientation::Both
                    r#type=ScrollAreaType::Always
                    class="h-56 w-80 max-w-full rounded-md border border-border"
                >
                    <div class="grid w-max grid-cols-6 gap-3 p-4">
                        {(1..=36).map(|i| view! { <Tile index=i /> }).collect_view()}
                    </div>
                </ScrollArea>
            </Section>

            <Section
                title="When the bars show"
                description="Hover (the default) shows them while the pointer is over the area; Auto whenever the content overflows; Scroll for a second after scrolling. A short list below does not overflow, so Auto shows no bar there."
            >
                <div class="flex flex-wrap gap-6">
                    <div class="space-y-2">
                        <p class="text-xs text-muted-foreground">"Hover"</p>
                        <ScrollArea class="h-40 w-40 rounded-md border border-border">
                            <div class="p-3">
                                <Tags count=20 />
                            </div>
                        </ScrollArea>
                    </div>
                    <div class="space-y-2">
                        <p class="text-xs text-muted-foreground">"Auto"</p>
                        <ScrollArea r#type=ScrollAreaType::Auto class="h-40 w-40 rounded-md border border-border">
                            <div class="p-3">
                                <Tags count=20 />
                            </div>
                        </ScrollArea>
                    </div>
                    <div class="space-y-2">
                        <p class="text-xs text-muted-foreground">"Scroll"</p>
                        <ScrollArea r#type=ScrollAreaType::Scroll class="h-40 w-40 rounded-md border border-border">
                            <div class="p-3">
                                <Tags count=20 />
                            </div>
                        </ScrollArea>
                    </div>
                    <div class="space-y-2">
                        <p class="text-xs text-muted-foreground">"Auto, no overflow"</p>
                        <ScrollArea r#type=ScrollAreaType::Auto class="h-40 w-40 rounded-md border border-border">
                            <div class="p-3">
                                <Tags count=3 />
                            </div>
                        </ScrollArea>
                    </div>
                </div>
            </Section>
        </Page>
    }
}
