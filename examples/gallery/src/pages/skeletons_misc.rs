use super::glyphs;
use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    AspectRatio, Button, ButtonVariant, Card, CardContent, CardHeader, Kbd, KbdGroup, Size,
    Skeleton,
};

/// An inline SVG as a data URI, so the aspect-ratio examples need no image file.
const PICTURE: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 160 90' preserveAspectRatio='xMidYMid slice'><defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'><stop offset='0' stop-color='%2334d399'/><stop offset='1' stop-color='%236366f1'/></linearGradient></defs><rect width='160' height='90' fill='url(%23g)'/><circle cx='120' cy='28' r='12' fill='white' fill-opacity='.7'/><path d='M0 90 50 40l30 30 20-18 60 38z' fill='black' fill-opacity='.25'/></svg>";

#[component]
pub fn SkeletonsMiscPage() -> impl IntoView {
    view! {
        <Page title="Skeleton, Kbd, Aspect Ratio" summary="Three small pure-CSS components: a loading placeholder, a keyboard key, and a box that holds a ratio.">
            <Section title="Skeleton" description="No size of its own: width and height come from class. It pulses; the pulse stops under prefers-reduced-motion.">
                <Row label="Lines">
                    <div class="flex w-64 flex-col gap-2" aria-busy="true">
                        <Skeleton class="h-4 w-full" />
                        <Skeleton class="h-4 w-5/6" />
                        <Skeleton class="h-4 w-2/3" />
                    </div>
                </Row>
                <Row label="Avatar and text">
                    <div class="flex items-center gap-4" aria-busy="true">
                        <Skeleton class="size-12 rounded-full" />
                        <div class="flex flex-col gap-2">
                            <Skeleton class="h-4 w-[250px]" />
                            <Skeleton class="h-4 w-[200px]" />
                        </div>
                    </div>
                </Row>
                <Row label="In a card">
                    <div class="w-80 max-w-full">
                        <Card>
                            <CardHeader>
                                <div class="flex flex-col gap-2" aria-busy="true">
                                    <Skeleton class="h-4 w-2/3" />
                                    <Skeleton class="h-4 w-1/2" />
                                </div>
                            </CardHeader>
                            <CardContent>
                                <Skeleton class="aspect-video w-full" />
                            </CardContent>
                        </Card>
                    </div>
                </Row>
                <Row label="Shapes">
                    <Skeleton class="h-9 w-24" />
                    <Skeleton class="h-9 w-9 rounded-full" />
                    <Skeleton class="h-6 w-40 rounded-4xl" />
                    <Skeleton class="size-16 rounded-xl" />
                </Row>
            </Section>

            <Section title="Kbd" description="Renders a kbd element. KbdGroup keeps several keys together.">
                <Row label="Keys">
                    <Kbd>"Ctrl"</Kbd>
                    <Kbd>"⌘"</Kbd>
                    <Kbd>"⇧"</Kbd>
                    <Kbd>"Esc"</Kbd>
                    <Kbd>"Enter"</Kbd>
                    <Kbd>"Space"</Kbd>
                </Row>
                <Row label="Group">
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <Kbd>"Shift"</Kbd>
                        <Kbd>"P"</Kbd>
                    </KbdGroup>
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"K"</Kbd>
                    </KbdGroup>
                    <span class="text-sm text-muted-foreground">
                        "Press " <KbdGroup><Kbd>"Ctrl"</Kbd> "+" <Kbd>"B"</Kbd></KbdGroup> " to toggle the sidebar"
                    </span>
                </Row>
                <Row label="With an icon">
                    <Kbd>{glyphs::command()}</Kbd>
                    <Kbd>{glyphs::arrow_up()}</Kbd>
                    <Kbd>{glyphs::command()} "K"</Kbd>
                </Row>
                <Row label="In a button">
                    <Button variant=ButtonVariant::Outline size=Size::Small>"Accept " <Kbd>"⏎"</Kbd></Button>
                    <Button variant=ButtonVariant::Outline>"Search " <KbdGroup><Kbd>"⌘"</Kbd><Kbd>"K"</Kbd></KbdGroup></Button>
                </Row>
            </Section>

            <Section title="Aspect Ratio" description="Width comes from the parent; the height follows from the ratio. Each box below is 16/9, 1/1, 4/3 and 21/9.">
                <div class="grid grid-cols-2 gap-6 sm:grid-cols-4">
                    <AspectRatio ratio=16.0 / 9.0 class="overflow-hidden rounded-lg bg-muted">
                        <img src=PICTURE alt="16 by 9" class="size-full object-cover" />
                    </AspectRatio>
                    <AspectRatio ratio=1.0 class="overflow-hidden rounded-lg bg-muted">
                        <img src=PICTURE alt="1 by 1" class="size-full object-cover" />
                    </AspectRatio>
                    <AspectRatio ratio=4.0 / 3.0 class="overflow-hidden rounded-lg bg-muted">
                        <img src=PICTURE alt="4 by 3" class="size-full object-cover" />
                    </AspectRatio>
                    <AspectRatio ratio=21.0 / 9.0 class="overflow-hidden rounded-lg bg-muted">
                        <img src=PICTURE alt="21 by 9" class="size-full object-cover" />
                    </AspectRatio>
                </div>
                <Row label="Non-image content">
                    <div class="w-64">
                        <AspectRatio ratio=2.0 class="flex items-center justify-center rounded-lg border text-sm text-muted-foreground">
                            "2 / 1"
                        </AspectRatio>
                    </div>
                </Row>
            </Section>
        </Page>
    }
}
