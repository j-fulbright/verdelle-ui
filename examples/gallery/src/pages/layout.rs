use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant, Badge, Button, ButtonVariant,
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, IconName,
    Separator, Size, Spinner, Tabs, TabsContent, TabsList, TabsTrigger,
};

const PICTURE: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 160 90' preserveAspectRatio='xMidYMid slice'><defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'><stop offset='0' stop-color='%2334d399'/><stop offset='1' stop-color='%236366f1'/></linearGradient></defs><rect width='160' height='90' fill='url(%23g)'/><circle cx='120' cy='28' r='12' fill='white' fill-opacity='.7'/></svg>";

#[component]
pub fn LayoutPage() -> impl IntoView {
    let icon = RwSignal::new(Some(IconName::Info));
    let dismissals = RwSignal::new(0_u32);
    let again = RwSignal::new(0_u32);

    view! {
        <Page title="Cards, tabs, alerts" summary="Surfaces and structure: cards, tabs, alerts (icons, tones, dismiss), separators and spinners.">
            <Section title="Card">
                <div class="grid gap-6 md:grid-cols-2">
                    <Card>
                        <CardHeader>
                            <CardTitle>"QA1"</CardTitle>
                            <CardDescription>"A card with a header action and a footer."</CardDescription>
                            <CardAction>
                                <Badge>"running"</Badge>
                            </CardAction>
                        </CardHeader>
                        <CardContent class="flex flex-col gap-3">
                            <p class="text-sm">"Card content."</p>
                            <Separator decorative=true />
                            <div class="flex items-center gap-2 text-sm text-muted-foreground">
                                <Spinner />
                                "Loading..."
                            </div>
                        </CardContent>
                        <CardFooter bordered=true class="gap-2">
                            <Button>"Save"</Button>
                            <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                        </CardFooter>
                    </Card>
                    <Card small=true>
                        <CardHeader>
                            <CardTitle>"Small card"</CardTitle>
                            <CardDescription>"12px padding instead of 16px."</CardDescription>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm">"Content."</p>
                        </CardContent>
                    </Card>
                    <Card>
                        <img src=PICTURE alt="" class="aspect-video w-full object-cover" />
                        <CardHeader>
                            <CardTitle>"Card with an image"</CardTitle>
                            <CardDescription>"A first-child image is flush with the top edge."</CardDescription>
                        </CardHeader>
                    </Card>
                </div>
            </Section>

            <Section title="Tabs" description="Arrow keys, Home and End move between tabs; Tab leaves the list. Billing is disabled.">
                <Tabs default_value="account".to_string()>
                    <TabsList>
                        <TabsTrigger value="account".to_string()>"Account"</TabsTrigger>
                        <TabsTrigger value="password".to_string()>"Password"</TabsTrigger>
                        <TabsTrigger value="billing".to_string() disabled=true>"Billing"</TabsTrigger>
                    </TabsList>
                    <TabsContent value="account".to_string()>
                        <p class="text-sm">"Account settings."</p>
                    </TabsContent>
                    <TabsContent value="password".to_string()>
                        <p class="text-sm">"Change your password."</p>
                    </TabsContent>
                </Tabs>
            </Section>

            <Section title="Alert" description="A default alert is announced politely (role=status); a destructive one at once (role=alert).">
                <div class="grid max-w-2xl gap-3">
                    <Alert>
                        <AlertTitle>"Heads up"</AlertTitle>
                        <AlertDescription>"Something you might want to know."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Destructive>
                        <AlertTitle>"Something failed"</AlertTitle>
                        <AlertDescription>"The request could not be completed."</AlertDescription>
                    </Alert>
                </div>
            </Section>

            <Section
                title="Alert: icon and tones"
                description="icon draws a leading icon and the title and description line up beside it. Success and warning are Verdelle extras (shadcn has default and destructive); all four read in light and dark."
            >
                <div class="grid max-w-2xl gap-3">
                    <Alert icon=IconName::Info>
                        <AlertTitle>"Heads up"</AlertTitle>
                        <AlertDescription>"A default alert with an icon. The description lines up under the title."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Destructive icon=IconName::CircleAlert>
                        <AlertTitle>"Something failed"</AlertTitle>
                        <AlertDescription>"The request could not be completed. Check the details and try again."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Success icon=IconName::CircleCheck>
                        <AlertTitle>"Saved"</AlertTitle>
                        <AlertDescription>"Your changes are live."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Warning icon=IconName::TriangleAlert>
                        <AlertTitle>"Storage is almost full"</AlertTitle>
                        <AlertDescription>"You have used 92% of your plan. Older files will be archived first."</AlertDescription>
                    </Alert>
                    <Alert variant=AlertVariant::Success>
                        <AlertTitle>"A success tone without an icon"</AlertTitle>
                    </Alert>
                    <Alert variant=AlertVariant::Warning>
                        <AlertTitle>"A warning with a title only"</AlertTitle>
                        <AlertDescription>"And a description under it."</AlertDescription>
                    </Alert>
                    <Alert icon=IconName::Info>
                        <AlertTitle>"A title only, with an icon"</AlertTitle>
                    </Alert>
                </div>
            </Section>

            <Section
                title="Alert: icon from a signal"
                description="icon takes an Option or a signal of one, so an icon can come and go without a second view."
            >
                <div class="grid max-w-2xl gap-3">
                    <Alert icon=icon>
                        <AlertTitle>"Optional icon"</AlertTitle>
                        <AlertDescription>"Toggle the icon with the button."</AlertDescription>
                    </Alert>
                    <div>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| icon.update(|i| *i = if i.is_some() { None } else { Some(IconName::Info) })
                        >
                            "Toggle icon"
                        </Button>
                    </div>
                </div>
            </Section>

            <Section
                title="Alert: dismiss and action"
                description="dismissible adds a close button that hides the alert and calls on_dismiss (the parent can keep it hidden). AlertAction puts a button at the top right and the alert makes room for it."
            >
                <div class="grid max-w-2xl gap-3">
                    {move || {
                        let _ = again.get();
                        view! {
                            <Alert
                                variant=AlertVariant::Warning
                                icon=IconName::TriangleAlert
                                dismissible=true
                                dismiss_label="Dismiss warning"
                                on_dismiss=move |_| dismissals.update(|n| *n += 1)
                            >
                                <AlertTitle>"Maintenance tonight"</AlertTitle>
                                <AlertDescription>"The service will be read-only from 22:00 to 23:00."</AlertDescription>
                            </Alert>
                        }
                    }}
                    <Alert icon=IconName::Info>
                        <AlertTitle>"A new version is available"</AlertTitle>
                        <AlertDescription>"Reload to get the latest changes."</AlertDescription>
                        <AlertAction>
                            <Button variant=ButtonVariant::Outline size=Size::XSmall>"Reload"</Button>
                        </AlertAction>
                    </Alert>
                    <div class="flex items-center gap-3 text-sm text-muted-foreground">
                        <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| again.update(|n| *n += 1)>
                            "Show the warning again"
                        </Button>
                        "Dismissals: " <span id="alert-dismissals">{move || dismissals.get()}</span>
                    </div>
                </div>
            </Section>

            <Section
                title="Card header action on a narrow screen"
                description="A header with an action is a container: when its content is narrower than 22rem (a phone) the action drops under the title and description instead of squeezing them. The left card is 300px wide, the right one is as wide as the column."
            >
                <div class="grid items-start gap-6 md:grid-cols-2">
                    <div class="w-[300px] max-w-full">
                        <Card>
                            <CardHeader>
                                <CardTitle>"Production database"</CardTitle>
                                <CardDescription>"A longer description that would be squeezed into a thin column beside the action."</CardDescription>
                                <CardAction stack=true>
                                    <Button size=Size::Small variant=ButtonVariant::Outline>"Manage"</Button>
                                </CardAction>
                            </CardHeader>
                            <CardContent>
                                <p class="text-sm">"Narrow: the action stacks."</p>
                            </CardContent>
                        </Card>
                    </div>
                    <Card>
                        <CardHeader>
                            <CardTitle>"Production database"</CardTitle>
                            <CardDescription>"A longer description that has room beside the action."</CardDescription>
                            <CardAction stack=true>
                                <Button size=Size::Small variant=ButtonVariant::Outline>"Manage"</Button>
                            </CardAction>
                        </CardHeader>
                        <CardContent>
                            <p class="text-sm">"Wide: the action sits at the end."</p>
                        </CardContent>
                    </Card>
                </div>
            </Section>

            <Section title="Separator and spinner">
                <Row>
                    <span class="text-sm">"left"</span>
                    <div class="h-5"><Separator orientation=verdelle_ui::Orientation::Vertical decorative=true /></div>
                    <span class="text-sm">"right"</span>
                    <Spinner />
                </Row>
            </Section>
        </Page>
    }
}
