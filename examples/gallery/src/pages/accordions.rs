use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType, Button,
    ButtonVariant, Size,
};

#[component]
pub fn AccordionsPage() -> impl IntoView {
    let open = RwSignal::new(vec![String::from("shipping")]);

    view! {
        <Page
            title="Accordion"
            summary="Accordion, built on the headless primitives. Tab to a trigger; Enter or Space toggles; Up/Down, Home and End move between triggers. Panels animate their height (not under reduced motion)."
        >
            <Section title="Single, collapsible" description="One item open at a time; clicking the open one closes it. Starts with the first open.">
                <div class="max-w-xl">
                    <Accordion
                        r#type=AccordionType::Single
                        collapsible=true
                        default_value=vec![String::from("item-1")]
                    >
                        <AccordionItem value="item-1">
                            <AccordionTrigger>"Is it accessible?"</AccordionTrigger>
                            <AccordionContent>
                                "Yes. Triggers are buttons inside headings, they carry aria-expanded and aria-controls, and the panels are labelled by their triggers."
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="item-2">
                            <AccordionTrigger>"Is it styled?"</AccordionTrigger>
                            <AccordionContent>
                                "Yes. It follows shadcn's default style and reads your theme tokens."
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="item-3">
                            <AccordionTrigger>"Is it animated?"</AccordionTrigger>
                            <AccordionContent>
                                <p>"Yes: the height eases open and closed."</p>
                                <p>
                                    "A second paragraph makes the panel taller, so the motion is easy to see. "
                                    "With prefers-reduced-motion set, it changes instantly."
                                </p>
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </div>
            </Section>

            <Section title="Single, not collapsible" description="One item is always open: clicking the open one does nothing.">
                <div class="max-w-xl">
                    <Accordion r#type=AccordionType::Single default_value=vec![String::from("a")]>
                        <AccordionItem value="a">
                            <AccordionTrigger>"Always one open"</AccordionTrigger>
                            <AccordionContent>"Pick another item to move the open panel."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="b">
                            <AccordionTrigger>"Second"</AccordionTrigger>
                            <AccordionContent>"Second panel."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </div>
            </Section>

            <Section title="Multiple" description="Any number of items open at once; nothing open to start.">
                <div class="max-w-xl">
                    <Accordion r#type=AccordionType::Multiple>
                        <AccordionItem value="m1">
                            <AccordionTrigger>"Product information"</AccordionTrigger>
                            <AccordionContent>"Details about the product."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="m2">
                            <AccordionTrigger>"Shipping details"</AccordionTrigger>
                            <AccordionContent>"We ship worldwide."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="m3">
                            <AccordionTrigger>"Return policy"</AccordionTrigger>
                            <AccordionContent>"Thirty days, no questions."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </div>
            </Section>

            <Section title="Disabled item" description="The second item cannot be opened or focused, and keyboard navigation skips it.">
                <div class="max-w-xl">
                    <Accordion r#type=AccordionType::Single collapsible=true>
                        <AccordionItem value="d1">
                            <AccordionTrigger>"Enabled"</AccordionTrigger>
                            <AccordionContent>"This one works."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="d2" disabled=true>
                            <AccordionTrigger>"Disabled"</AccordionTrigger>
                            <AccordionContent>"You cannot see this."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="d3">
                            <AccordionTrigger>"Enabled too"</AccordionTrigger>
                            <AccordionContent>"This one works as well."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </div>
            </Section>

            <Section title="Disabled accordion">
                <div class="max-w-xl">
                    <Accordion r#type=AccordionType::Single disabled=true default_value=vec![String::from("x1")]>
                        <AccordionItem value="x1">
                            <AccordionTrigger>"Frozen open"</AccordionTrigger>
                            <AccordionContent>"The whole accordion is disabled."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="x2">
                            <AccordionTrigger>"Frozen closed"</AccordionTrigger>
                            <AccordionContent>"Hidden."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </div>
            </Section>

            <Section title="Controlled, with a readout" description="The open items live in a signal; the buttons change it from outside.">
                <div class="grid max-w-xl gap-3">
                    <Accordion
                        r#type=AccordionType::Multiple
                        value=open
                        on_value_change=move |v| open.set(v)
                    >
                        <AccordionItem value="shipping">
                            <AccordionTrigger>"Shipping"</AccordionTrigger>
                            <AccordionContent>"Ships in two days."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="returns">
                            <AccordionTrigger>"Returns"</AccordionTrigger>
                            <AccordionContent>"Free returns."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                    <div class="flex flex-wrap gap-2">
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| open.set(vec![String::from("shipping"), String::from("returns")])
                        >
                            "Open all"
                        </Button>
                        <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| open.set(vec![])>
                            "Close all"
                        </Button>
                    </div>
                    <span id="acc-readout" class="text-sm text-muted-foreground">
                        "open: [" {move || open.get().join(", ")} "]"
                    </span>
                </div>
            </Section>
        </Page>
    }
}
