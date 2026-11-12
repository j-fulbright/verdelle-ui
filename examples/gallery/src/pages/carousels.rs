use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, Carousel,
    CarouselApi, CarouselContent, CarouselDots, CarouselGap, CarouselItem, CarouselNext,
    CarouselPrevious, Orientation, Size, Switch,
};

/// A numbered square, the usual carousel demo slide.
#[component]
fn Slide(n: usize, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!("flex aspect-square items-center justify-center rounded-xl border bg-card p-6 {class}")>
            <span class="text-4xl font-semibold">{n}</span>
        </div>
    }
}

/// A slide with a fixed height, for vertical carousels.
#[component]
fn WideSlide(n: usize) -> impl IntoView {
    view! {
        <div class="flex h-full items-center justify-center rounded-xl border bg-card p-6">
            <span class="text-3xl font-semibold">{n}</span>
        </div>
    }
}

#[component]
pub fn CarouselsPage() -> impl IntoView {
    // Controlled: the page owns the index, the carousel follows it and reports user changes.
    let index = RwSignal::new(2usize);
    let selected = RwSignal::new(0usize);
    // The API handle, kept to read the count and drive the carousel from outside.
    let api: RwSignal<Option<CarouselApi>> = RwSignal::new(None);
    let drag = RwSignal::new(true);

    view! {
        <Page
            title="Carousel"
            summary="Slides that scroll and snap, on native CSS scroll-snap with a small Rust model instead of Embla: touch swipes natively, the mouse drags, arrow keys and the buttons step. role=region with aria-roledescription=carousel; each slide is a group named \"n of m\" after hydration."
        >
            <Section
                title="Default"
                description="One slide per view. Previous and Next sit outside the slides and are disabled at the ends. Focus the carousel and use the arrow keys; Home and End jump to the ends."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-xs" label="Default carousel">
                        <CarouselContent>
                            {(1..=5).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="With cards"
                description="Any content can be a slide; here each one is a Card."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-sm" label="Cards carousel">
                        <CarouselContent>
                            {["Plan", "Build", "Test", "Ship"]
                                .into_iter()
                                .enumerate()
                                .map(|(i, name)| {
                                    view! {
                                        <CarouselItem>
                                            <Card>
                                                <CardHeader>
                                                    <CardTitle>{name}</CardTitle>
                                                    <CardDescription>{format!("Step {} of 4", i + 1)}</CardDescription>
                                                </CardHeader>
                                                <CardContent>
                                                    <p class="text-sm text-muted-foreground">
                                                        "Cards keep their own padding and border inside the slide."
                                                    </p>
                                                </CardContent>
                                            </Card>
                                        </CarouselItem>
                                    }
                                })
                                .collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Several slides per view (basis classes)"
                description="Size each CarouselItem with a basis class, as in shadcn: basis-1/2 here, and md:basis-1/3 on wider screens. With five slides and two in view there are four snap positions, so the last step shows slides 4 and 5."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-md" label="Basis classes carousel">
                        <CarouselContent>
                            {(1..=5)
                                .map(|n| {
                                    view! {
                                        <CarouselItem class="basis-1/2 md:basis-1/3">
                                            <Slide n=n />
                                        </CarouselItem>
                                    }
                                })
                                .collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Slides per view (prop)"
                description="slides_per_view=2.5 sets the basis without classes, and a fractional value shows a peek of the next slide."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-md" slides_per_view=2.5 label="Slides per view carousel">
                        <CarouselContent>
                            {(1..=6).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Step"
                description="step=2 moves two snap positions with a button, an arrow key or autoplay. A step that would overshoot the end lands on the end."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-md" slides_per_view=2.0 step=2 label="Step carousel">
                        <CarouselContent>
                            {(1..=8).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                        <CarouselDots />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Spacing"
                description="gap is the space between slides: None, Small (8px), Medium (16px, default) and Large (24px)."
            >
                <div class="space-y-8 px-12">
                    {[
                        (CarouselGap::None, "None"),
                        (CarouselGap::Small, "Small"),
                        (CarouselGap::Medium, "Medium"),
                        (CarouselGap::Large, "Large"),
                    ]
                        .into_iter()
                        .map(|(gap, name)| {
                            view! {
                                <Carousel class="mx-auto max-w-md" gap=gap label=format!("{name} gap carousel")>
                                    <CarouselContent>
                                        {(1..=4)
                                            .map(|n| {
                                                view! {
                                                    <CarouselItem class="basis-1/2">
                                                        <Slide n=n class="aspect-[3/2]" />
                                                    </CarouselItem>
                                                }
                                            })
                                            .collect_view()}
                                    </CarouselContent>
                                    <CarouselPrevious />
                                    <CarouselNext />
                                </Carousel>
                            }
                        })
                        .collect_view()}
                </div>
            </Section>

            <Section
                title="Vertical"
                description="orientation=Vertical: Up and Down arrow keys, buttons above and below. Give CarouselContent a height."
            >
                <div class="py-12">
                    <Carousel class="mx-auto max-w-xs" orientation=Orientation::Vertical label="Vertical carousel">
                        <CarouselContent class="h-[200px]">
                            {(1..=5)
                                .map(|n| {
                                    view! {
                                        <CarouselItem class="basis-1/2">
                                            <WideSlide n=n />
                                        </CarouselItem>
                                    }
                                })
                                .collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Loop"
                description="looping=true: stepping past either end wraps to the other, so Previous and Next never disable. The wrap is a smooth scroll back across the slides, not an endless strip."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-xs" looping=true label="Looping carousel">
                        <CarouselContent>
                            {(1..=4).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                        <CarouselDots />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Autoplay"
                description="autoplay=2500 advances every 2.5 seconds and wraps at the end. It pauses while the pointer is over the carousel, while keyboard focus is inside it and during a drag, and it does not run when the user prefers reduced motion."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-xs" autoplay=2500u32 label="Autoplay carousel">
                        <CarouselContent>
                            {(1..=4).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                        <CarouselDots />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Dots"
                description="CarouselDots: one button per snap position, the current one marked, each a way to jump there."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-xs" label="Dots carousel">
                        <CarouselContent>
                            {(1..=5).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselDots />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Controlled index, on_select and on_api"
                description="index is a signal: set it from outside and the carousel scrolls there; on_select reports changes made in the carousel; on_api hands over the CarouselApi (count, scroll_to, ...)."
            >
                <div class="space-y-4 px-12">
                    <Carousel
                        class="mx-auto max-w-xs"
                        label="Controlled carousel"
                        index=index
                        on_select=move |i| {
                            index.set(i);
                            selected.set(i);
                        }
                        on_api=move |a| api.set(Some(a))
                    >
                        <CarouselContent>
                            {(1..=5).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                    <div class="flex flex-wrap items-center justify-center gap-2">
                        <span class="text-sm text-muted-foreground" data-testid="controlled-readout">
                            {move || {
                                format!(
                                    "index {} of {} (last on_select: {})",
                                    index.get() + 1,
                                    api.get().map(|a| a.count()).unwrap_or(0),
                                    selected.get() + 1,
                                )
                            }}
                        </span>
                        {(0..5usize)
                            .map(|i| {
                                view! {
                                    <Button
                                        variant=ButtonVariant::Outline
                                        size=Size::Small
                                        on_click=move |_| index.set(i)
                                    >
                                        {format!("Go to {}", i + 1)}
                                    </Button>
                                }
                            })
                            .collect_view()}
                        <Button
                            variant=ButtonVariant::Secondary
                            size=Size::Small
                            on_click=move |_| {
                                if let Some(a) = api.get_untracked() {
                                    a.scroll_to(a.count().saturating_sub(1));
                                }
                            }
                        >
                            "api.scroll_to(last)"
                        </Button>
                    </div>
                </div>
            </Section>

            <Section
                title="Initial index"
                description="default_index=2 starts on the third slide (applied once the page is interactive)."
            >
                <div class="px-12">
                    <Carousel class="mx-auto max-w-xs" default_index=2usize label="Default index carousel">
                        <CarouselContent>
                            {(1..=5).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </div>
            </Section>

            <Section
                title="Mouse dragging and button variants"
                description="draggable (default true) lets the mouse drag the slides; touch always scrolls natively. The switch below turns it off for the carousel under it. The buttons take a variant."
            >
                <div class="space-y-4 px-12">
                    <div class="flex items-center justify-center gap-2 text-sm">
                        <Switch
                            default_checked=true
                            aria_label="Draggable"
                            on_checked_change=move |v| drag.set(v)
                        />
                        "Draggable"
                    </div>
                    <Carousel class="mx-auto max-w-xs" draggable=drag label="Drag carousel">
                        <CarouselContent>
                            {(1..=4).map(|n| view! { <CarouselItem><Slide n=n /></CarouselItem> }).collect_view()}
                        </CarouselContent>
                        <CarouselPrevious variant=ButtonVariant::Secondary />
                        <CarouselNext variant=ButtonVariant::Secondary />
                    </Carousel>
                </div>
            </Section>
        </Page>
    }
}
