use super::glyphs;
use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Badge, BadgeVariant, Button, ButtonVariant, Empty, EmptyContent, EmptyDescription, EmptyHeader,
    EmptyMedia, EmptyMediaVariant, EmptyTitle, Item, ItemActions, ItemContent, ItemDescription,
    ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemMediaVariant, ItemSeparator, ItemTitle,
    ItemVariant, Size,
};

const VARIANTS: [ItemVariant; 3] = [
    ItemVariant::Default,
    ItemVariant::Outline,
    ItemVariant::Muted,
];
const SIZES: [Size; 3] = [Size::Medium, Size::Small, Size::XSmall];

const PEOPLE: [(&str, &str); 3] = [
    ("Ada Lovelace", "ada@example.com"),
    ("Grace Hopper", "grace@example.com"),
    ("Alan Turing", "alan@example.com"),
];

#[component]
pub fn EmptyItemsPage() -> impl IntoView {
    view! {
        <Page title="Empty and Item" summary="Empty is the placeholder for a view with nothing in it. Item is a row of media, text and actions.">
            <Section title="Empty" description="Header (media, title, description) and content (actions). The icon media is a 32px muted tile.">
                <Empty bordered=true>
                    <EmptyHeader>
                        <EmptyMedia variant=EmptyMediaVariant::Icon>{glyphs::folder()}</EmptyMedia>
                        <EmptyTitle>"No projects yet"</EmptyTitle>
                        <EmptyDescription>
                            "You haven't created any projects yet. Get started by creating your first project."
                        </EmptyDescription>
                    </EmptyHeader>
                    <EmptyContent>
                        <div class="flex gap-2">
                            <Button>"Create project"</Button>
                            <Button variant=ButtonVariant::Outline>"Import project"</Button>
                        </div>
                    </EmptyContent>
                </Empty>
            </Section>

            <Section title="Without a border" description="The same component with bordered off, here on a muted surface and with a link in the description.">
                <div class="rounded-lg bg-muted/50">
                    <Empty>
                        <EmptyHeader>
                            <EmptyMedia variant=EmptyMediaVariant::Icon>{glyphs::bell()}</EmptyMedia>
                            <EmptyTitle>"No notifications"</EmptyTitle>
                            <EmptyDescription>
                                "You're all caught up. New notifications will appear here. "
                                <a href="#">"Notification settings"</a>
                            </EmptyDescription>
                        </EmptyHeader>
                    </Empty>
                </div>
            </Section>

            <Section title="Default media" description="The default media variant is bare: here a stack of initials, with one action.">
                <Empty bordered=true>
                    <EmptyHeader>
                        <EmptyMedia>
                            <div class="flex -space-x-2">
                                {["A", "G", "T"]
                                    .into_iter()
                                    .map(|c| view! {
                                        <span class="flex size-10 items-center justify-center rounded-full border-2 border-background bg-secondary text-sm font-medium">{c}</span>
                                    })
                                    .collect_view()}
                            </div>
                        </EmptyMedia>
                        <EmptyTitle>"No team members"</EmptyTitle>
                        <EmptyDescription>"Invite your team to collaborate on this project."</EmptyDescription>
                    </EmptyHeader>
                    <EmptyContent>
                        <Button size=Size::Small>{glyphs::plus()} "Invite members"</Button>
                    </EmptyContent>
                </Empty>
            </Section>

            <Section title="Compact" description="Padding and width are overridable with class.">
                <div class="grid gap-4 sm:grid-cols-2">
                    <Empty bordered=true class="p-6">
                        <EmptyHeader>
                            <EmptyMedia variant=EmptyMediaVariant::Icon>{glyphs::inbox()}</EmptyMedia>
                            <EmptyTitle>"Inbox zero"</EmptyTitle>
                            <EmptyDescription>"Nothing to read."</EmptyDescription>
                        </EmptyHeader>
                    </Empty>
                    <Empty bordered=true class="p-6">
                        <EmptyHeader>
                            <EmptyTitle>"404 - Not found"</EmptyTitle>
                            <EmptyDescription>"The page you're looking for doesn't exist."</EmptyDescription>
                        </EmptyHeader>
                        <EmptyContent>
                            <Button variant=ButtonVariant::Link>"Go home"</Button>
                        </EmptyContent>
                    </Empty>
                </div>
            </Section>

            <Section title="Item variants" description="Default, outline and muted, each with an icon, title, description and an action.">
                <ItemGroup>
                    {VARIANTS
                        .into_iter()
                        .map(|v| view! {
                            <Item variant=v>
                                <ItemMedia variant=ItemMediaVariant::Icon>{glyphs::shield()}</ItemMedia>
                                <ItemContent>
                                    <ItemTitle>{v.as_str()} " item"</ItemTitle>
                                    <ItemDescription>"A short description of what this row is about."</ItemDescription>
                                </ItemContent>
                                <ItemActions>
                                    <Button variant=ButtonVariant::Outline size=Size::Small>"Action"</Button>
                                </ItemActions>
                            </Item>
                        })
                        .collect_view()}
                </ItemGroup>
            </Section>

            <Section title="Item sizes" description="Medium (default), small and extra small. The group tightens its gap for the smaller sizes.">
                {SIZES
                    .into_iter()
                    .map(|s| view! {
                        <ItemGroup>
                            <Item variant=ItemVariant::Outline size=s>
                                <ItemMedia variant=ItemMediaVariant::Icon>{glyphs::info()}</ItemMedia>
                                <ItemContent>
                                    <ItemTitle>"Size " {s.as_str()}</ItemTitle>
                                    <ItemDescription>"Description text for this size."</ItemDescription>
                                </ItemContent>
                                <ItemActions>
                                    <Button variant=ButtonVariant::Ghost size=Size::XSmall>"Open"</Button>
                                </ItemActions>
                            </Item>
                            <Item variant=ItemVariant::Outline size=s>
                                <ItemMedia variant=ItemMediaVariant::Icon>{glyphs::info()}</ItemMedia>
                                <ItemContent>
                                    <ItemTitle>"Title only"</ItemTitle>
                                </ItemContent>
                            </Item>
                        </ItemGroup>
                    })
                    .collect_view()}
            </Section>

            <Section title="Media" description="Image media is a rounded square sized by the item (40, 32 and 24px). The default variant is bare.">
                <ItemGroup>
                    {SIZES
                        .into_iter()
                        .map(|s| view! {
                            <Item variant=ItemVariant::Outline size=s>
                                <ItemMedia variant=ItemMediaVariant::Image>
                                    <span class="flex size-full items-center justify-center bg-gradient-to-br from-emerald-400 to-indigo-500 text-xs font-semibold text-white">"BS"</span>
                                </ItemMedia>
                                <ItemContent>
                                    <ItemTitle>"Image media, " {s.as_str()}</ItemTitle>
                                    <ItemDescription>"The tile follows the item's size."</ItemDescription>
                                </ItemContent>
                            </Item>
                        })
                        .collect_view()}
                    <Item variant=ItemVariant::Muted>
                        <ItemMedia>
                            <span class="flex size-9 items-center justify-center rounded-full bg-primary text-sm text-primary-foreground">"AL"</span>
                        </ItemMedia>
                        <ItemContent>
                            <ItemTitle>"Bare media"</ItemTitle>
                            <ItemDescription>"Any content, at its own size."</ItemDescription>
                        </ItemContent>
                    </Item>
                </ItemGroup>
            </Section>

            <Section title="Header and footer" description="Full-width rows above and below the content.">
                <ItemGroup>
                    <Item variant=ItemVariant::Outline>
                        <ItemHeader>
                            <span class="text-xs text-muted-foreground">"Invoice 0042"</span>
                            <Badge variant=BadgeVariant::Secondary>"Paid"</Badge>
                        </ItemHeader>
                        <ItemMedia variant=ItemMediaVariant::Icon>{glyphs::mail()}</ItemMedia>
                        <ItemContent>
                            <ItemTitle>"Monthly plan"</ItemTitle>
                            <ItemDescription>"Billed on the first of each month."</ItemDescription>
                        </ItemContent>
                        <ItemContent>
                            <ItemTitle>"$29.00"</ItemTitle>
                        </ItemContent>
                        <ItemFooter>
                            <span class="text-xs text-muted-foreground">"Receipt sent to ada@example.com"</span>
                            <Button variant=ButtonVariant::Link size=Size::XSmall>"Download"</Button>
                        </ItemFooter>
                    </Item>
                </ItemGroup>
            </Section>

            <Section title="As a link" description="With href an item renders as an anchor: hover fill and a focus ring (Tab to it).">
                <ItemGroup>
                    <Item variant=ItemVariant::Outline href="#">
                        <ItemMedia variant=ItemMediaVariant::Icon>{glyphs::star()}</ItemMedia>
                        <ItemContent>
                            <ItemTitle>"A linked item"</ItemTitle>
                            <ItemDescription>"The whole row is the link."</ItemDescription>
                        </ItemContent>
                    </Item>
                    <Item variant=ItemVariant::Muted href="#">
                        <ItemContent>
                            <ItemTitle>"A muted linked item"</ItemTitle>
                        </ItemContent>
                    </Item>
                    <Item href="#">
                        <ItemContent>
                            <ItemTitle>"A default linked item"</ItemTitle>
                            <ItemDescription>"Without a border until hover."</ItemDescription>
                        </ItemContent>
                    </Item>
                </ItemGroup>
            </Section>

            <Section title="List with separators" description="ItemGroup is role=list. ItemSeparator draws a rule between rows.">
                <div class="max-w-md">
                    <ItemGroup>
                        {PEOPLE
                            .into_iter()
                            .enumerate()
                            .map(|(i, (name, mail))| view! {
                                {(i > 0).then(|| view! { <ItemSeparator /> })}
                                <Item>
                                    <ItemMedia variant=ItemMediaVariant::Image>
                                        <span class="flex size-full items-center justify-center bg-secondary text-sm font-medium">{name.chars().next().map(String::from).unwrap_or_default()}</span>
                                    </ItemMedia>
                                    <ItemContent>
                                        <ItemTitle>{name}</ItemTitle>
                                        <ItemDescription>{mail}</ItemDescription>
                                    </ItemContent>
                                    <ItemActions>
                                        <Button variant=ButtonVariant::Ghost icon_only=true size=Size::Small aria_label="More">{glyphs::more_vertical()}</Button>
                                    </ItemActions>
                                </Item>
                            })
                            .collect_view()}
                    </ItemGroup>
                </div>
            </Section>

            <Section title="Truncation" description="A title clamps to one line and a description to two, in a narrow container.">
                <div class="w-72">
                    <Item variant=ItemVariant::Outline>
                        <ItemMedia variant=ItemMediaVariant::Icon>{glyphs::folder()}</ItemMedia>
                        <ItemContent>
                            <ItemTitle>"An extremely long title that cannot possibly fit on one line"</ItemTitle>
                            <ItemDescription>"A very long description that goes on and on, and on, to show that it is cut off after the second line of text."</ItemDescription>
                        </ItemContent>
                        <ItemActions>
                            <Button variant=ButtonVariant::Ghost icon_only=true size=Size::Small aria_label="Copy">{glyphs::copy()}</Button>
                        </ItemActions>
                    </Item>
                </div>
            </Section>
            <Section
                title="Wrapping title"
                description="A title made of several children (a name and badges) is a single-line flex row by default and runs past a narrow card. wrap=true lets it wrap onto more lines. Both items sit in a narrow container."
            >
                <div class="flex flex-wrap items-start gap-4">
                    <div class="w-64 max-w-full">
                    <Item variant=ItemVariant::Outline>
                        <ItemContent>
                            <ItemTitle>
                                "Quarterly report"
                                <Badge variant=BadgeVariant::Secondary>"draft"</Badge>
                                <Badge variant=BadgeVariant::Outline>"internal"</Badge>
                                <Badge>"needs review"</Badge>
                            </ItemTitle>
                            <ItemDescription>"Default: a single flex row, so the badges run past the card."</ItemDescription>
                        </ItemContent>
                    </Item>
                    </div>
                    <div class="w-64 max-w-full">
                    <Item variant=ItemVariant::Outline>
                        <ItemContent>
                            <ItemTitle wrap=true>
                                "Quarterly report"
                                <Badge variant=BadgeVariant::Secondary>"draft"</Badge>
                                <Badge variant=BadgeVariant::Outline>"internal"</Badge>
                                <Badge>"needs review"</Badge>
                            </ItemTitle>
                            <ItemDescription>"wrap=true: the badges wrap under the name."</ItemDescription>
                        </ItemContent>
                    </Item>
                    </div>
                </div>
            </Section>
        </Page>
    }
}
