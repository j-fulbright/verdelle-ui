use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, Pagination, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationNext, PaginationPrevious, Size,
};

#[component]
pub fn NavigationPage() -> impl IntoView {
    let page = RwSignal::new(3_usize);

    view! {
        <Page title="Breadcrumb and Pagination" summary="Two navigation components. Both only lay out links; the app owns the routing and the page state.">
            <Section title="Breadcrumb" description="Links to ancestors, a separator between, and the current page as plain text.">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem><BreadcrumbLink href="#">"Home"</BreadcrumbLink></BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem><BreadcrumbLink href="#">"Components"</BreadcrumbLink></BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem><BreadcrumbPage>"Breadcrumb"</BreadcrumbPage></BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Section>

            <Section title="Custom separator" description="Children of BreadcrumbSeparator replace the chevron.">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem><BreadcrumbLink href="#">"Home"</BreadcrumbLink></BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                        <BreadcrumbItem><BreadcrumbLink href="#">"Settings"</BreadcrumbLink></BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                        <BreadcrumbItem><BreadcrumbPage>"Profile"</BreadcrumbPage></BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Section>

            <Section title="Collapsed" description="BreadcrumbEllipsis stands for hidden levels. Wrap it in a Dropdown Menu trigger once that component exists.">
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem><BreadcrumbLink href="#">"Home"</BreadcrumbLink></BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem><BreadcrumbEllipsis /></BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem><BreadcrumbLink href="#">"Components"</BreadcrumbLink></BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem><BreadcrumbPage>"Breadcrumb"</BreadcrumbPage></BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Section>

            <Section title="Wrapping" description="In a narrow container the list wraps onto more lines. Tab through the links to see focus.">
                <div class="w-60 rounded-md border border-border p-3">
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem><BreadcrumbLink href="#">"Organisation"</BreadcrumbLink></BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem><BreadcrumbLink href="#">"Engineering"</BreadcrumbLink></BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem><BreadcrumbLink href="#">"Platform"</BreadcrumbLink></BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem><BreadcrumbPage>"Deployment pipelines"</BreadcrumbPage></BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                </div>
            </Section>

            <Section title="Pagination" description="Page links are anchors drawn as buttons: ghost, and outline for the current page. Previous and Next hide their text below 640px.">
                <Pagination>
                    <PaginationContent>
                        <PaginationItem><PaginationPrevious /></PaginationItem>
                        <PaginationItem><PaginationLink href="#">"1"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationLink href="#" active=true>"2"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationLink href="#">"3"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationEllipsis /></PaginationItem>
                        <PaginationItem><PaginationNext /></PaginationItem>
                    </PaginationContent>
                </Pagination>
            </Section>

            <Section title="Driven by state" description="The app owns the page: the links here set a signal. The href is # and the click is intercepted.">
                <Pagination>
                    <PaginationContent>
                        <PaginationItem>
                            <PaginationPrevious on_click=move |ev: leptos::ev::MouseEvent| {
                                ev.prevent_default();
                                page.update(|p| *p = p.saturating_sub(1).max(1));
                            } />
                        </PaginationItem>
                        {(1..=5)
                            .map(|n| view! {
                                <PaginationItem>
                                    <PaginationLink
                                        href="#"
                                        active=Signal::derive(move || page.get() == n)
                                        on_click=move |ev: leptos::ev::MouseEvent| {
                                            ev.prevent_default();
                                            page.set(n);
                                        }
                                    >
                                        {n.to_string()}
                                    </PaginationLink>
                                </PaginationItem>
                            })
                            .collect_view()}
                        <PaginationItem>
                            <PaginationNext on_click=move |ev: leptos::ev::MouseEvent| {
                                ev.prevent_default();
                                page.update(|p| *p = (*p + 1).min(5));
                            } />
                        </PaginationItem>
                    </PaginationContent>
                </Pagination>
                <p class="text-center text-sm text-muted-foreground">"Page " {move || page.get()} " of 5"</p>
            </Section>

            <Section title="Custom text and sizes" description="Previous and Next take their text as a prop. Page links take a size; icon_only=false gives a text-width link.">
                <Pagination>
                    <PaginationContent>
                        <PaginationItem><PaginationPrevious text="Back" /></PaginationItem>
                        <PaginationItem><PaginationLink href="#" active=true>"1"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationLink href="#" icon_only=false>"Page two"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationNext text="Forward" /></PaginationItem>
                    </PaginationContent>
                </Pagination>
                {[Size::XSmall, Size::Small, Size::Medium, Size::Large]
                    .into_iter()
                    .map(|s| view! {
                        <Row label=s.as_str()>
                            <Pagination class="justify-start">
                                <PaginationContent>
                                    <PaginationItem><PaginationLink href="#" size=s>"1"</PaginationLink></PaginationItem>
                                    <PaginationItem><PaginationLink href="#" size=s active=true>"2"</PaginationLink></PaginationItem>
                                    <PaginationItem><PaginationLink href="#" size=s>"3"</PaginationLink></PaginationItem>
                                </PaginationContent>
                            </Pagination>
                        </Row>
                    })
                    .collect_view()}
            </Section>
        </Page>
    }
}
