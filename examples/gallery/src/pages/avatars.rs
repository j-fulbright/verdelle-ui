use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage,
    ImageLoadingStatus, Input, Size, initials,
};

/// Not an image: the browser raises an error event without logging a failed request.
const BROKEN: &str = "data:image/png;base64,AAAA";

/// A picture that needs no network: a gradient disc as an SVG data URI.
const PICTURE: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'%3E%3Cdefs%3E%3ClinearGradient id='g' x1='0' y1='0' x2='1' y2='1'%3E%3Cstop offset='0' stop-color='%23f97316'/%3E%3Cstop offset='1' stop-color='%236366f1'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='64' height='64' fill='url(%23g)'/%3E%3Ccircle cx='32' cy='26' r='11' fill='white' fill-opacity='.85'/%3E%3Cpath d='M10 64c2-16 14-24 22-24s20 8 22 24z' fill='white' fill-opacity='.85'/%3E%3C/svg%3E";

#[component]
fn Person(#[prop(optional)] size: Size, initials: &'static str) -> impl IntoView {
    view! {
        <Avatar size=size>
            <AvatarImage src=PICTURE alt=initials />
            <AvatarFallback>{initials}</AvatarFallback>
        </Avatar>
    }
}

#[component]
pub fn AvatarsPage() -> impl IntoView {
    let status = RwSignal::new(String::from("(waiting)"));
    let broken_status = RwSignal::new(String::from("(waiting)"));
    let person = RwSignal::new(String::from("Ada Lovelace"));

    view! {
        <Page
            title="Avatar"
            summary="A round picture with a fallback. The image is only placed in the page once it has loaded, so a broken source falls back to initials instead of a broken-image icon."
        >
            <Section title="Image and fallback" description="Loaded image; an image that fails to load; no source at all; a fallback with a delay.">
                <Row label="Image">
                    <Avatar>
                        <AvatarImage
                            src=PICTURE
                            alt="Ada"
                            on_loading_status_change=move |s: ImageLoadingStatus| status.set(format!("{s:?}"))
                        />
                        <AvatarFallback>"AD"</AvatarFallback>
                    </Avatar>
                    <span id="avatar-status" class="text-sm text-muted-foreground">"status: " {move || status.get()}</span>
                </Row>
                <Row label="Fails to load">
                    <Avatar>
                        <AvatarImage
                            src=BROKEN
                            alt="Missing"
                            on_loading_status_change=move |s: ImageLoadingStatus| broken_status.set(format!("{s:?}"))
                        />
                        <AvatarFallback>"CN"</AvatarFallback>
                    </Avatar>
                    <span id="avatar-broken-status" class="text-sm text-muted-foreground">
                        "status: " {move || broken_status.get()}
                    </span>
                </Row>
                <Row label="No source">
                    <Avatar>
                        <AvatarImage />
                        <AvatarFallback>"?"</AvatarFallback>
                    </Avatar>
                </Row>
                <Row label="Fallback delay">
                    <Avatar>
                        <AvatarImage src=BROKEN alt="" />
                        <AvatarFallback delay_ms=600>"LT"</AvatarFallback>
                    </Avatar>
                    <span class="text-sm text-muted-foreground">"shows after 600 ms"</span>
                </Row>
            </Section>

            <Section title="Sizes" description="Small 24px, default 32px, large 40px. The same sizes with initials only below.">
                <Row label="Image">
                    <Person size=Size::Small initials="SM" />
                    <Person initials="MD" />
                    <Person size=Size::Large initials="LG" />
                </Row>
                <Row label="Initials">
                    <Avatar size=Size::Small>
                        <AvatarFallback>"SM"</AvatarFallback>
                    </Avatar>
                    <Avatar>
                        <AvatarFallback>"MD"</AvatarFallback>
                    </Avatar>
                    <Avatar size=Size::Large>
                        <AvatarFallback>"LG"</AvatarFallback>
                    </Avatar>
                </Row>
            </Section>

            <Section title="Badge" description="A status dot at the corner, scaled by size. The icon shows from the default size up.">
                <Row>
                    <Avatar size=Size::Small>
                        <AvatarImage src=PICTURE alt="" />
                        <AvatarFallback>"SM"</AvatarFallback>
                        <AvatarBadge aria_label="Online" />
                    </Avatar>
                    <Avatar>
                        <AvatarImage src=PICTURE alt="" />
                        <AvatarFallback>"MD"</AvatarFallback>
                        <AvatarBadge aria_label="Online" />
                    </Avatar>
                    <Avatar size=Size::Large>
                        <AvatarFallback>"LG"</AvatarFallback>
                        <AvatarBadge aria_label="Online">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="4" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M5 12l5 5L20 7"></path>
                            </svg>
                        </AvatarBadge>
                    </Avatar>
                </Row>
            </Section>

            <Section title="Group" description="Overlapping avatars ringed with the page background; AvatarGroupCount takes the last place. It follows the avatars' size.">
                <Row label="Default">
                    <AvatarGroup>
                        <Person initials="AD" />
                        <Person initials="BK" />
                        <Avatar>
                            <AvatarFallback>"CN"</AvatarFallback>
                        </Avatar>
                        <AvatarGroupCount>"+3"</AvatarGroupCount>
                    </AvatarGroup>
                </Row>
                <Row label="Large">
                    <AvatarGroup>
                        <Person size=Size::Large initials="AD" />
                        <Person size=Size::Large initials="BK" />
                        <AvatarGroupCount>"+12"</AvatarGroupCount>
                    </AvatarGroup>
                </Row>
                <Row label="Small">
                    <AvatarGroup>
                        <Person size=Size::Small initials="AD" />
                        <Person size=Size::Small initials="BK" />
                        <AvatarGroupCount>"+9"</AvatarGroupCount>
                    </AvatarGroup>
                </Row>
            </Section>
            <Section
                title="Initials from a name"
                description="AvatarFallback takes name instead of children and shows up to two initials (first letter of the first two words, upper-cased; punctuation skipped). The initials() function does the same for any string, and ShellUser uses it."
            >
                <Row label="name prop">
                    {["Ada Lovelace", "grace hopper", "Plato", "Ada King Lovelace", "(Dr.) Alan Turing", "élodie dupont"]
                        .into_iter()
                        .map(|n| {
                            view! {
                                <Avatar>
                                    <AvatarFallback name=n />
                                </Avatar>
                            }
                        })
                        .collect_view()}
                </Row>
                <Row label="Empty name">
                    <Avatar>
                        <AvatarFallback name="" />
                    </Avatar>
                    <span class="text-xs text-muted-foreground">"No letters: the circle is empty."</span>
                </Row>
                <Row label="From a signal">
                    <div class="w-56">
                        <Input value=person aria_label="Name" placeholder="Type a name" />
                    </div>
                    <Avatar size=Size::Large>
                        <AvatarFallback name=Signal::derive(move || Some(person.get())) />
                    </Avatar>
                    <span class="text-sm text-muted-foreground">
                        "initials(): " <code id="initials-out">{move || initials(&person.get())}</code>
                    </span>
                </Row>
            </Section>
        </Page>
    }
}
