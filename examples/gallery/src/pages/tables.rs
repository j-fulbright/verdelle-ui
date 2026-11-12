use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Badge, BadgeVariant, Breakpoint, Button, ButtonVariant, Card, RowState, Size, Table, TableBody,
    TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow, TextAlign,
};

/// Sample rows: branch, owner, status, status variant, open pull requests.
const BRANCHES: [(&str, &str, &str, BadgeVariant, u32); 5] = [
    ("main", "platform", "Deployed", BadgeVariant::Default, 0),
    (
        "feat/table",
        "dana",
        "In review",
        BadgeVariant::Secondary,
        2,
    ),
    ("fix/cookie", "sam", "Blocked", BadgeVariant::Destructive, 1),
    ("chore/deps", "bot", "Queued", BadgeVariant::Outline, 4),
    (
        "revert/cache",
        "ops",
        "Failed",
        BadgeVariant::Destructive,
        0,
    ),
];

#[component]
pub fn TablesPage() -> impl IntoView {
    let striped = RwSignal::new(false);
    let selected_row = RwSignal::new(1_usize);

    view! {
        <Page title="Tables" summary="Real table elements, with a hairline between rows, hover tint, and optional stripes, selection and sticky header.">
            <Section title="Table" description="With a caption, a footer, right-aligned numbers and status badges.">
                <Row>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        selected=striped
                        on_click=move |_| striped.update(|v| *v = !*v)
                    >
                        "Striped"
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        on_click=move |_| selected_row.update(|r| *r = (*r + 1) % BRANCHES.len())
                    >
                        "Next row"
                    </Button>
                </Row>
                <Table striped=striped bordered=true>
                    <TableCaption>"Open branches (sample data)"</TableCaption>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Branch"</TableHead>
                            <TableHead>"Owner"</TableHead>
                            <TableHead>"Status"</TableHead>
                            <TableHead align=TextAlign::Right>"Open PRs"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {BRANCHES
                            .into_iter()
                            .enumerate()
                            .map(|(i, (branch, owner, status, variant, prs))| {
                                view! {
                                    <TableRow selected=Signal::derive(move || selected_row.get() == i)>
                                        <TableCell>{branch}</TableCell>
                                        <TableCell>{owner}</TableCell>
                                        <TableCell>
                                            <Badge variant=variant>{status}</Badge>
                                        </TableCell>
                                        <TableCell align=TextAlign::Right>{prs}</TableCell>
                                    </TableRow>
                                }
                            })
                            .collect_view()}
                    </TableBody>
                    <TableFooter>
                        <TableRow>
                            <TableCell col_span=3>"Total open pull requests"</TableCell>
                            <TableCell align=TextAlign::Right>{BRANCHES.iter().map(|b| b.4).sum::<u32>()}</TableCell>
                        </TableRow>
                    </TableFooter>
                </Table>
            </Section>

            <Section title="Sticky header" description="Inside a card, with a height set through class. Scroll the table.">
                <Card class="gap-0 py-0">
                    <Table sticky_header=true class="max-h-64">
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Row"</TableHead>
                                <TableHead>"Name"</TableHead>
                                <TableHead align=TextAlign::Right>"Value"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {(1..=30)
                                .map(|i| {
                                    view! {
                                        <TableRow>
                                            <TableCell>{i}</TableCell>
                                            <TableCell>{format!("item-{i}")}</TableCell>
                                            <TableCell align=TextAlign::Right>{i * 37 % 101}</TableCell>
                                        </TableRow>
                                    }
                                })
                                .collect_view()}
                        </TableBody>
                    </Table>
                </Card>
            </Section>

            <Section title="Classes, wrap and row state" description="Every table part takes a class. Here a thick divider after a row (TableRow class), a zero-padding cell holding a nested panel (TableCell class p-0), a wrapping cell (wrap), and rows dimmed or tinted through state: Dimmed, Highlighted, Destructive.">
                <Table bordered=true>
                    <TableHeader>
                        <TableRow>
                            <TableHead class="w-40">"Item"</TableHead>
                            <TableHead wrap=true>"Notes (this column wraps)"</TableHead>
                            <TableHead align=TextAlign::Right>"State"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow state=RowState::Normal>
                            <TableCell>"Normal"</TableCell>
                            <TableCell wrap=true>"A long note that would normally push the table wider than its box: cells stay on one line unless wrap is set on them."</TableCell>
                            <TableCell align=TextAlign::Right>"Normal"</TableCell>
                        </TableRow>
                        <TableRow state=RowState::Dimmed>
                            <TableCell>"Expired"</TableCell>
                            <TableCell wrap=true>"Dimmed rows fade back."</TableCell>
                            <TableCell align=TextAlign::Right>"Dimmed"</TableCell>
                        </TableRow>
                        <TableRow state=RowState::Highlighted>
                            <TableCell>"Next up"</TableCell>
                            <TableCell wrap=true>"Highlighted rows get a faint primary wash."</TableCell>
                            <TableCell align=TextAlign::Right>"Highlighted"</TableCell>
                        </TableRow>
                        <TableRow state=RowState::Destructive class="border-b-4 border-border">
                            <TableCell>"Past due"</TableCell>
                            <TableCell wrap=true>"Destructive rows are tinted red. This row also has a thick divider below it (class)."</TableCell>
                            <TableCell align=TextAlign::Right>"Destructive"</TableCell>
                        </TableRow>
                        <TableRow selected=true>
                            <TableCell>"Selected"</TableCell>
                            <TableCell wrap=true>"selected uses the theme's active row colour (data-selected)."</TableCell>
                            <TableCell align=TextAlign::Right>"Selected"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell col_span=3 class="p-0">
                                <div class="bg-muted/40 px-4 py-3 text-sm text-muted-foreground">
                                    "A detail cell with p-0 and its own padding inside."
                                </div>
                            </TableCell>
                        </TableRow>
                    </TableBody>
                    <TableFooter>
                        <TableRow>
                            <TableCell col_span=2>"Footer row: muted band, medium weight, no hover tint"</TableCell>
                            <TableCell align=TextAlign::Right>"6"</TableCell>
                        </TableRow>
                    </TableFooter>
                </Table>
            </Section>

            <Section title="Inside a card" description="A table in a card needs no border of its own: leave bordered off (the default) and give the card py-0 gap-0. The card supplies the frame and the footer keeps its band.">
                <Card class="gap-0 py-0">
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Branch"</TableHead>
                                <TableHead>"Owner"</TableHead>
                                <TableHead align=TextAlign::Right>"Open PRs"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {BRANCHES
                                .into_iter()
                                .take(3)
                                .map(|(branch, owner, _, _, prs)| {
                                    view! {
                                        <TableRow>
                                            <TableCell>{branch}</TableCell>
                                            <TableCell>{owner}</TableCell>
                                            <TableCell align=TextAlign::Right>{prs}</TableCell>
                                        </TableRow>
                                    }
                                })
                                .collect_view()}
                        </TableBody>
                        <TableFooter>
                            <TableRow>
                                <TableCell col_span=2>"Total"</TableCell>
                                <TableCell align=TextAlign::Right>"3"</TableCell>
                            </TableRow>
                        </TableFooter>
                    </Table>
                </Card>
            </Section>

            <Section title="Responsive tables" description="Three patterns for narrow screens. Scroll: the default; the wrapper scrolls sideways and nothing is lost, best for wide numeric tables. Hide columns: hide_below on TableHead and TableCell drops the minor columns under a breakpoint, best when a few columns carry the meaning. Stack: stack_below on Table turns each row into a card with the column names (label on TableCell), best for few rows with many fields. DataTable has the same three (see Data Table: layout), plus pinned columns for a scrolling table.">
                <h3 class="text-sm font-medium">"1. Scroll (a narrow box shows it)"</h3>
                <div class="max-w-xs">
                    <Table bordered=true>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Branch"</TableHead>
                                <TableHead>"Owner"</TableHead>
                                <TableHead>"Status"</TableHead>
                                <TableHead align=TextAlign::Right>"Open PRs"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {BRANCHES
                                .into_iter()
                                .take(3)
                                .map(|(branch, owner, status, _, prs)| {
                                    view! {
                                        <TableRow>
                                            <TableCell>{branch}</TableCell>
                                            <TableCell>{owner}</TableCell>
                                            <TableCell>{status}</TableCell>
                                            <TableCell align=TextAlign::Right>{prs}</TableCell>
                                        </TableRow>
                                    }
                                })
                                .collect_view()}
                        </TableBody>
                    </Table>
                </div>
                <h3 class="text-sm font-medium">"2. Hide columns (Owner below md, Status below lg)"</h3>
                <Table bordered=true>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Branch"</TableHead>
                            <TableHead hide_below=Breakpoint::Md>"Owner"</TableHead>
                            <TableHead hide_below=Breakpoint::Lg>"Status"</TableHead>
                            <TableHead align=TextAlign::Right>"Open PRs"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {BRANCHES
                            .into_iter()
                            .take(3)
                            .map(|(branch, owner, status, _, prs)| {
                                view! {
                                    <TableRow>
                                        <TableCell>{branch}</TableCell>
                                        <TableCell hide_below=Breakpoint::Md>{owner}</TableCell>
                                        <TableCell hide_below=Breakpoint::Lg>{status}</TableCell>
                                        <TableCell align=TextAlign::Right>{prs}</TableCell>
                                    </TableRow>
                                }
                            })
                            .collect_view()}
                    </TableBody>
                </Table>
                <h3 class="text-sm font-medium">"3. Stack into cards (below md)"</h3>
                <Table bordered=true stack_below=Breakpoint::Md>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Branch"</TableHead>
                            <TableHead>"Owner"</TableHead>
                            <TableHead>"Status"</TableHead>
                            <TableHead align=TextAlign::Right>"Open PRs"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {BRANCHES
                            .into_iter()
                            .take(3)
                            .map(|(branch, owner, status, _, prs)| {
                                view! {
                                    <TableRow>
                                        <TableCell label="Branch">{branch}</TableCell>
                                        <TableCell label="Owner">{owner}</TableCell>
                                        <TableCell label="Status">{status}</TableCell>
                                        <TableCell label="Open PRs" align=TextAlign::Right>{prs}</TableCell>
                                    </TableRow>
                                }
                            })
                            .collect_view()}
                    </TableBody>
                </Table>
            </Section>
        </Page>
    }
}
