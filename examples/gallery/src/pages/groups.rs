use super::glyphs;
use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonGroup, ButtonGroupSeparator, ButtonGroupText, ButtonVariant, Input, InputGroup,
    InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupButtonSize, InputGroupInput,
    InputGroupText, InputGroupTextarea, Kbd, NativeSelect, NativeSelectOptGroup,
    NativeSelectOption, Orientation, Size, Spinner,
};

const BUTTON_VARIANTS: [ButtonVariant; 3] = [
    ButtonVariant::Outline,
    ButtonVariant::Secondary,
    ButtonVariant::Default,
];

#[component]
pub fn GroupsPage() -> impl IntoView {
    view! {
        <Page title="Button Group, Input Group, Native Select" summary="Controls joined into one: buttons side by side, an input with addons inside one frame, and the browser's own select.">
            <ButtonGroupSection />
            <InputGroupSection />
            <NativeSelectSection />
        </Page>
    }
}

#[component]
fn ButtonGroupSection() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let liked = RwSignal::new(false);
    let choice = RwSignal::new("month".to_string());

    view! {
        <Section title="Button Group" description="Direct children are joined: inner corners square off and inner borders merge. Tab through to see the focus ring sit above its neighbours.">
            <Row label="Variants">
                {BUTTON_VARIANTS
                    .into_iter()
                    .map(|v| view! {
                        <ButtonGroup aria_label=v.as_str()>
                            <Button variant=v>"Archive"</Button>
                            <Button variant=v>"Report"</Button>
                            <Button variant=v>"Snooze"</Button>
                        </ButtonGroup>
                    })
                    .collect_view()}
            </Row>
            <Row label="Sizes">
                {[Size::XSmall, Size::Small, Size::Medium, Size::Large]
                    .into_iter()
                    .map(|s| view! {
                        <ButtonGroup>
                            <Button variant=ButtonVariant::Outline size=s>{s.as_str()}</Button>
                            <Button variant=ButtonVariant::Outline size=s>"Two"</Button>
                            <Button variant=ButtonVariant::Outline size=s icon_only=true aria_label="Add">{glyphs::plus()}</Button>
                        </ButtonGroup>
                    })
                    .collect_view()}
            </Row>
            <Row label="Separator">
                <ButtonGroup>
                    <Button variant=ButtonVariant::Secondary>"Copy"</Button>
                    <ButtonGroupSeparator />
                    <Button variant=ButtonVariant::Secondary>"Paste"</Button>
                    <ButtonGroupSeparator />
                    <Button variant=ButtonVariant::Secondary icon_only=true aria_label="More">{glyphs::more_vertical()}</Button>
                </ButtonGroup>
            </Row>
            <Row label="Vertical">
                <ButtonGroup orientation=Orientation::Vertical aria_label="Zoom">
                    <Button variant=ButtonVariant::Outline icon_only=true aria_label="Zoom in">{glyphs::plus()}</Button>
                    <Button variant=ButtonVariant::Outline icon_only=true aria_label="Reset">{glyphs::search()}</Button>
                    <Button variant=ButtonVariant::Outline icon_only=true aria_label="Favourite">{glyphs::star()}</Button>
                </ButtonGroup>
                <ButtonGroup orientation=Orientation::Vertical>
                    <Button variant=ButtonVariant::Secondary>"Top"</Button>
                    <ButtonGroupSeparator orientation=Orientation::Horizontal />
                    <Button variant=ButtonVariant::Secondary>"Middle"</Button>
                    <ButtonGroupSeparator orientation=Orientation::Horizontal />
                    <Button variant=ButtonVariant::Secondary>"Bottom"</Button>
                </ButtonGroup>
            </Row>
            <Row label="Nested">
                <ButtonGroup>
                    <ButtonGroup>
                        <Button variant=ButtonVariant::Outline icon_only=true aria_label="Add">{glyphs::plus()}</Button>
                    </ButtonGroup>
                    <ButtonGroup>
                        <Button variant=ButtonVariant::Outline>"Previous"</Button>
                        <Button variant=ButtonVariant::Outline>"Next"</Button>
                    </ButtonGroup>
                    <ButtonGroup>
                        <Button variant=ButtonVariant::Outline selected=liked on_click=move |_| liked.update(|l| *l = !*l)>
                            {glyphs::star()} "Like"
                        </Button>
                    </ButtonGroup>
                </ButtonGroup>
            </Row>
            <Row label="With text">
                <ButtonGroup>
                    <ButtonGroupText>"https://"</ButtonGroupText>
                    <ButtonGroupText>{glyphs::info()} "Notice"</ButtonGroupText>
                    <Button variant=ButtonVariant::Outline>"Go"</Button>
                </ButtonGroup>
            </Row>
            <Row label="With an input">
                <ButtonGroup class="w-80 max-w-full">
                    <Input value=query placeholder="Search..." aria_label="Search" />
                    <Button variant=ButtonVariant::Outline icon_only=true aria_label="Search">{glyphs::search()}</Button>
                </ButtonGroup>
            </Row>
            <Row label="With a select">
                <ButtonGroup>
                    <NativeSelect value=choice aria_label="Period">
                        <NativeSelectOption value="week">"Weekly"</NativeSelectOption>
                        <NativeSelectOption value="month">"Monthly"</NativeSelectOption>
                        <NativeSelectOption value="year">"Yearly"</NativeSelectOption>
                    </NativeSelect>
                    <Button variant=ButtonVariant::Outline>"Apply"</Button>
                </ButtonGroup>
                <span class="text-xs text-muted-foreground">"value: " {move || choice.get()}</span>
            </Row>
        </Section>
    }
}

#[component]
fn InputGroupSection() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let url = RwSignal::new("example.com".to_string());
    let amount = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let secret = RwSignal::new("A toggle button, pressed state".to_string());
    let show = RwSignal::new(false);
    let message = RwSignal::new(String::new());
    let title = RwSignal::new(String::new());
    let copied = RwSignal::new(false);
    let disabled_value = RwSignal::new("Can't edit".to_string());
    let invalid_value = RwSignal::new("not an email".to_string());
    let loading_value = RwSignal::new("Searching...".to_string());
    let sizes_value = RwSignal::new(String::new());

    view! {
        <Section title="Input Group" description="One border and one focus ring around a bare input and its addons. Click an addon to focus the input.">
            <div class="grid max-w-xl gap-4">
                <InputGroup>
                    <InputGroupInput value=search placeholder="Search..." aria_label="Search" />
                    <InputGroupAddon>{glyphs::search()}</InputGroupAddon>
                    <InputGroupAddon align=InputGroupAlign::InlineEnd>
                        <InputGroupText>{move || if search.get().is_empty() { "12 results".to_string() } else { format!("{} chars", search.get().len()) }}</InputGroupText>
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupAddon><InputGroupText>"https://"</InputGroupText></InputGroupAddon>
                    <InputGroupInput value=url aria_label="Domain" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd><InputGroupText>".com"</InputGroupText></InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupAddon><InputGroupText>"$"</InputGroupText></InputGroupAddon>
                    <InputGroupInput value=amount placeholder="0.00" aria_label="Amount" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd><InputGroupText>"USD"</InputGroupText></InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput value=email kind="email" placeholder="you@example.com" aria_label="Email" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd>
                        <InputGroupButton
                            size=InputGroupButtonSize::IconXs
                            aria_label="Copy"
                            on_click=move |_| copied.update(|c| *c = !*c)
                        >
                            {move || if copied.get() { glyphs::check().into_any() } else { glyphs::copy().into_any() }}
                        </InputGroupButton>
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput value=secret aria_label="Label" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd>
                        <InputGroupButton
                            size=InputGroupButtonSize::IconXs
                            aria_label="Star"
                            selected=show
                            on_click=move |_| show.update(|s| *s = !*s)
                        >
                            {glyphs::star()}
                        </InputGroupButton>
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupAddon><InputGroupButton variant=ButtonVariant::Secondary>"Filter"</InputGroupButton></InputGroupAddon>
                    <InputGroupInput value=sizes_value placeholder="Text button, extra small" aria_label="Filter text" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd>
                        <InputGroupButton variant=ButtonVariant::Outline size=InputGroupButtonSize::Sm>"Go"</InputGroupButton>
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupAddon>{glyphs::command()}</InputGroupAddon>
                    <InputGroupInput value=RwSignal::new(String::new()) placeholder="Type a command" aria_label="Command" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd><Kbd>"⌘K"</Kbd></InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupInput value=loading_value aria_label="Searching" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd><Spinner /></InputGroupAddon>
                </InputGroup>
            </div>

            <h3 class="pt-2 text-sm font-medium">"Textarea and block addons"</h3>
            <div class="grid max-w-xl gap-4">
                <InputGroup>
                    <InputGroupTextarea value=message placeholder="Write a message..." aria_label="Message" />
                    <InputGroupAddon align=InputGroupAlign::BlockEnd>
                        <InputGroupText>{move || format!("{} characters", message.get().len())}</InputGroupText>
                        <InputGroupButton variant=ButtonVariant::Default size=InputGroupButtonSize::IconSm class="ml-auto rounded-full" aria_label="Send">
                            {glyphs::arrow_up()}
                        </InputGroupButton>
                    </InputGroupAddon>
                </InputGroup>

                <InputGroup>
                    <InputGroupAddon align=InputGroupAlign::BlockStart class="border-b">
                        <InputGroupText>"script.js"</InputGroupText>
                        <InputGroupButton size=InputGroupButtonSize::IconXs class="ml-auto" aria_label="Copy">{glyphs::copy()}</InputGroupButton>
                    </InputGroupAddon>
                    <InputGroupTextarea value=RwSignal::new("console.log('hello');".to_string()) rows=2 class="font-mono text-sm" aria_label="Code" />
                </InputGroup>

                <InputGroup>
                    <InputGroupAddon align=InputGroupAlign::BlockStart>
                        <InputGroupText>"Title"</InputGroupText>
                    </InputGroupAddon>
                    <InputGroupInput value=title placeholder="Block start with an input" aria_label="Title" />
                    <InputGroupAddon align=InputGroupAlign::BlockEnd>
                        <InputGroupText>"Shown on your profile."</InputGroupText>
                    </InputGroupAddon>
                </InputGroup>
            </div>

            <GroupEvents />

            <h3 class="pt-2 text-sm font-medium">"States"</h3>
            <div class="grid max-w-xl gap-4">
                <InputGroup disabled=true>
                    <InputGroupAddon>{glyphs::user()}</InputGroupAddon>
                    <InputGroupInput value=disabled_value aria_label="Disabled" />
                </InputGroup>
                <InputGroup invalid=true>
                    <InputGroupAddon>{glyphs::mail()}</InputGroupAddon>
                    <InputGroupInput value=invalid_value aria_label="Invalid" />
                    <InputGroupAddon align=InputGroupAlign::InlineEnd>{glyphs::info()}</InputGroupAddon>
                </InputGroup>
            </div>
        </Section>
    }
}

#[component]
fn NativeSelectSection() -> impl IntoView {
    let fruit = RwSignal::new("banana".to_string());
    let region = RwSignal::new(String::new());

    view! {
        <Section title="Native Select" description="The browser's own select in a styled frame. Open it with the keyboard (Space or Alt+Down) to check the platform picker.">
            <Row label="Default">
                <NativeSelect aria_label="Fruit" value=fruit>
                    <NativeSelectOption value="apple">"Apple"</NativeSelectOption>
                    <NativeSelectOption value="banana">"Banana"</NativeSelectOption>
                    <NativeSelectOption value="cherry">"Cherry"</NativeSelectOption>
                    <NativeSelectOption value="durian" disabled=true>"Durian (sold out)"</NativeSelectOption>
                </NativeSelect>
                <span class="text-xs text-muted-foreground">"value: " {move || fruit.get()}</span>
            </Row>
            <Row label="Small">
                <NativeSelect aria_label="Small" size=Size::Small>
                    <NativeSelectOption value="a" selected=true>"Small select"</NativeSelectOption>
                    <NativeSelectOption value="b">"Another"</NativeSelectOption>
                </NativeSelect>
            </Row>
            <Row label="Uncontrolled">
                <NativeSelect aria_label="Uncontrolled">
                    <NativeSelectOption value="a">"First"</NativeSelectOption>
                    <NativeSelectOption value="b" selected=true>"Second (selected attribute)"</NativeSelectOption>
                </NativeSelect>
            </Row>
            <Row label="Option groups">
                <NativeSelect aria_label="Region" value=region>
                    <NativeSelectOption value="">"Choose a region"</NativeSelectOption>
                    <NativeSelectOptGroup label="Europe">
                        <NativeSelectOption value="uk">"United Kingdom"</NativeSelectOption>
                        <NativeSelectOption value="fr">"France"</NativeSelectOption>
                    </NativeSelectOptGroup>
                    <NativeSelectOptGroup label="Asia">
                        <NativeSelectOption value="jp">"Japan"</NativeSelectOption>
                        <NativeSelectOption value="in">"India"</NativeSelectOption>
                    </NativeSelectOptGroup>
                    <NativeSelectOptGroup label="Closed" disabled=true>
                        <NativeSelectOption value="xx">"Nowhere"</NativeSelectOption>
                    </NativeSelectOptGroup>
                </NativeSelect>
            </Row>
            <Row label="Disabled">
                <NativeSelect aria_label="Disabled" disabled=true>
                    <NativeSelectOption value="a">"Disabled select"</NativeSelectOption>
                </NativeSelect>
            </Row>
            <Row label="Invalid">
                <NativeSelect aria_label="Invalid" invalid=true>
                    <NativeSelectOption value="a">"Invalid select"</NativeSelectOption>
                    <NativeSelectOption value="b">"Another"</NativeSelectOption>
                </NativeSelect>
            </Row>
            <Row label="Full width">
                <NativeSelect aria_label="Wide" class="w-full max-w-md">
                    <NativeSelectOption value="a">"Takes the width of its container"</NativeSelectOption>
                </NativeSelect>
            </Row>
        </Section>
    }
}

/// InputGroupTextarea grow and max_rows, and the event callbacks on the group controls.
#[component]
fn GroupEvents() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let searched = RwSignal::new(String::from("(none)"));
    let note = RwSignal::new(String::from("one\ntwo\nthree\nfour\nfive\nsix"));
    let sent = RwSignal::new(String::from("(none)"));

    view! {
        <h3 class="pt-2 text-sm font-medium">"Events, grow and max_rows"</h3>
        <div class="grid max-w-xl gap-4">
            <InputGroup>
                <InputGroupInput
                    value=search
                    placeholder="Press Enter to search"
                    aria_label="Search with Enter"
                    on_enter=move |_| searched.set(search.get_untracked())
                />
                <InputGroupAddon align=InputGroupAlign::InlineEnd>
                    <InputGroupText>
                        "searched: " <span id="group-searched">{move || searched.get()}</span>
                    </InputGroupText>
                </InputGroupAddon>
            </InputGroup>
            <InputGroup>
                <InputGroupTextarea
                    value=note
                    rows=2
                    grow=true
                    max_rows=4
                    aria_label="Grow with a cap"
                    on_keydown=move |ev: leptos::ev::KeyboardEvent| {
                        if ev.key() == "Enter" && !ev.shift_key() {
                            ev.prevent_default();
                        }
                    }
                    on_enter=move |_| sent.set(note.get_untracked().replace('\n', "\u{23ce}"))
                />
                <InputGroupAddon align=InputGroupAlign::BlockEnd>
                    <InputGroupText>"grow, max_rows=4, Enter sends: "<span id="group-sent">{move || sent.get()}</span></InputGroupText>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
