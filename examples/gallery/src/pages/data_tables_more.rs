use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Badge, BadgeVariant, Breakpoint, Button, ButtonVariant, Card, CardContent, CardDescription,
    CardHeader, CardTitle, DataTable, DataTableColumn, DataTableFooterInfo, DataTableState, Icon,
    IconName, RowState, Size, TextAlign,
};

#[derive(Clone, Debug, PartialEq)]
struct Account {
    id: u32,
    name: &'static str,
    email: &'static str,
    plan: &'static str,
    renews: &'static str,
    seats: u32,
    /// Days until the renewal; negative is past due.
    days: i32,
}

fn accounts() -> Vec<Account> {
    vec![
        Account {
            id: 1,
            name: "Northwind Traders",
            email: "billing.department.northwind@very-long-company-domain-name.example.com",
            plan: "Team",
            renews: "2026-12-01",
            seats: 24,
            days: 21,
        },
        Account {
            id: 2,
            name: "Contoso",
            email: "ap@contoso.example",
            plan: "Business",
            renews: "2026-11-02",
            seats: 120,
            days: -8,
        },
        Account {
            id: 3,
            name: "Fabrikam",
            email: "finance@fabrikam.example",
            plan: "Team",
            renews: "2027-01-15",
            seats: 12,
            days: 66,
        },
        Account {
            id: 4,
            name: "Adventure Works",
            email: "accounts-receivable-and-invoicing@adventure-works.example",
            plan: "Free",
            renews: "2026-05-30",
            seats: 3,
            days: -164,
        },
        Account {
            id: 5,
            name: "Tailspin",
            email: "pay@tailspin.example",
            plan: "Business",
            renews: "2026-11-14",
            seats: 75,
            days: 4,
        },
    ]
}

fn account_key(a: &Account) -> String {
    a.id.to_string()
}

fn plan_badge(plan: &'static str) -> impl IntoView {
    let variant = match plan {
        "Business" => BadgeVariant::Default,
        "Team" => BadgeVariant::Secondary,
        _ => BadgeVariant::Outline,
    };
    view! { <Badge variant=variant>{plan}</Badge> }
}

fn edit_button(a: &Account) -> AnyView {
    let label = format!("Edit {}", a.name);
    view! {
        <Button variant=ButtonVariant::Outline size=Size::Small aria_label=label>
            "Edit"
        </Button>
    }
    .into_any()
}

/// name, email, plan, renews, seats, edit: the shared column set, with sorting on the data columns.
fn base_columns() -> Vec<DataTableColumn<Account>> {
    vec![
        DataTableColumn::text("name", "Account", |a: &Account| a.name.to_string()).hideable(false),
        DataTableColumn::text("email", "Billing email", |a: &Account| a.email.to_string()),
        DataTableColumn::new("plan", "Plan", |a: &Account| plan_badge(a.plan))
            .filter_text(|a| a.plan.to_string())
            .sort_by_key(|a| a.plan),
        DataTableColumn::text("renews", "Renews", |a: &Account| a.renews.to_string()),
        DataTableColumn::new("seats", "Seats", |a: &Account| a.seats)
            .sort_by_key(|a| a.seats)
            .align(TextAlign::Right),
        DataTableColumn::new("edit", "Actions", |a: &Account| edit_button(a))
            .header_hidden()
            .hideable(false)
            .align(TextAlign::Right),
    ]
}

#[component]
pub fn DataTablesMorePage() -> impl IntoView {
    view! {
        <Page
            title="Data Table: layout and rows"
            summary="Column wrapping and widths, hiding and pinning columns on narrow screens, the stacked card layout, row emphasis, expandable detail rows, footer options and a table inside a card. Narrow the window (or open this at 390px) to see the responsive options."
        >
            <Wrapping />
            <HideBelow />
            <Pinned />
            <Stacked />
            <RowStates />
            <Expandable />
            <FooterOptions />
            <InCard />
        </Page>
    }
}

#[component]
fn Wrapping() -> impl IntoView {
    let columns = vec![
        DataTableColumn::text("name", "Account", |a: &Account| a.name.to_string()),
        DataTableColumn::text("email", "Billing email", |a: &Account| a.email.to_string())
            .wrap()
            .min_width("10rem")
            .max_width("18rem"),
        DataTableColumn::new("plan", "Plan", |a: &Account| plan_badge(a.plan)),
    ];
    view! {
        <Section
            title="Wrapping and widths"
            description="Cells stay on one line by default, so a long value makes the table scroll sideways. A column marked wrap() breaks long values (emails included); min_width and max_width bound it. Here the email column wraps between 10rem and 18rem."
        >
            <DataTable
                data=accounts()
                columns=columns
                row_key=account_key
                paginated=false
                show_status=false
                aria_label="Accounts with a wrapping column"
            />
        </Section>
    }
}

#[component]
fn HideBelow() -> impl IntoView {
    let mut columns = base_columns();
    columns[1] = DataTableColumn::text("email", "Billing email", |a: &Account| a.email.to_string())
        .wrap()
        .max_width("16rem")
        .hide_below(Breakpoint::Lg);
    columns[3] = DataTableColumn::text("renews", "Renews", |a: &Account| a.renews.to_string())
        .hide_below(Breakpoint::Md);
    columns[4] = DataTableColumn::new("seats", "Seats", |a: &Account| a.seats)
        .sort_by_key(|a| a.seats)
        .align(TextAlign::Right)
        .hide_below(Breakpoint::Sm);
    view! {
        <Section
            title="Hide columns below a breakpoint"
            description="hide_below hides a column's header, cells and footer on viewports narrower than the breakpoint: Billing email below lg (1024px), Renews below md (768px), Seats below sm (640px). The actions column always stays."
        >
            <DataTable
                data=accounts()
                columns=columns
                row_key=account_key
                paginated=false
                show_status=false
                aria_label="Accounts, columns hidden by width"
            />
        </Section>
    }
}

#[component]
fn Pinned() -> impl IntoView {
    let mut columns = base_columns();
    columns[0] = DataTableColumn::text("name", "Account", |a: &Account| a.name.to_string())
        .hideable(false)
        .sticky();
    columns[5] = DataTableColumn::new("edit", "Actions", |a: &Account| edit_button(a))
        .header_hidden()
        .hideable(false)
        .align(TextAlign::Right)
        .sticky_end();
    view! {
        <Section
            title="Pinned first and last column"
            description="sticky pins the key column to the left edge and sticky_end pins the actions column to the right while the rest scrolls sideways. The box here is 22rem wide so it scrolls at any size; with selectable, the checkbox column is pinned too."
        >
            <div class="max-w-[22rem]">
                <DataTable
                    data=accounts()
                    columns=columns
                    row_key=account_key
                    paginated=false
                    show_status=false
                    selectable=true
                    aria_label="Accounts with pinned columns"
                />
            </div>
        </Section>
    }
}

#[component]
fn Stacked() -> impl IntoView {
    view! {
        <Section
            title="Stacked cards below a breakpoint"
            description="stack_below turns each row into a card with the column names beside the values, under the breakpoint (md here: below 768px). The header row is replaced by a bar with Select all and a Sort menu. Wide enough, it is the normal table."
        >
            <DataTable
                data=accounts()
                columns=base_columns()
                row_key=account_key
                paginated=false
                selectable=true
                stack_below=Breakpoint::Md
                aria_label="Accounts, stacked on small screens"
            />
        </Section>
    }
}

#[component]
fn RowStates() -> impl IntoView {
    view! {
        <Section
            title="Row class and state"
            description="row_state maps a row to Normal, Dimmed, Highlighted or Destructive (past-due rows here are Destructive, expired free accounts Dimmed, the next to renew Highlighted). row_class adds your own classes per row; here a line-through on the dimmed ones."
        >
            <DataTable
                data=accounts()
                columns=base_columns()
                row_key=account_key
                paginated=false
                show_status=false
                row_state=|a: Account| {
                    if a.days < -100 {
                        RowState::Dimmed
                    } else if a.days < 0 {
                        RowState::Destructive
                    } else if a.days < 7 {
                        RowState::Highlighted
                    } else {
                        RowState::Normal
                    }
                }
                row_class=|a: Account| {
                    if a.days < -100 { "line-through".to_string() } else { String::new() }
                }
                aria_label="Accounts by renewal"
            />
        </Section>
    }
}

#[component]
fn Expandable() -> impl IntoView {
    let state = DataTableState::new();
    let ids = [1_u32, 2, 3, 4, 5];
    view! {
        <Section
            title="Expandable detail rows"
            description="detail adds a toggle column (a button with aria-expanded, reachable with Tab and Space or Enter) and renders a panel under every open row. The open rows are DataTableState::expanded, so the buttons below can drive it."
        >
            <Row>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    on_click=move |_| {
                        state.expanded.set(ids.iter().map(|i| i.to_string()).collect())
                    }
                >
                    "Expand all"
                </Button>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    on_click=move |_| state.expanded.set(Default::default())
                >
                    "Collapse all"
                </Button>
                <span class="text-sm text-muted-foreground" role="status">
                    {move || format!("{} open", state.expanded.with(|e| e.len()))}
                </span>
            </Row>
            <DataTable
                data=accounts()
                columns=base_columns()
                row_key=account_key
                state=state
                paginated=false
                show_status=false
                selectable=true
                detail=Callback::new(|a: Account| {
                    view! {
                        <dl class="grid grid-cols-[auto_1fr] gap-x-6 gap-y-1 text-sm">
                            <dt class="text-muted-foreground">"Billing email"</dt>
                            <dd>{a.email}</dd>
                            <dt class="text-muted-foreground">"Renews"</dt>
                            <dd>{a.renews}</dd>
                            <dt class="text-muted-foreground">"Seats"</dt>
                            <dd>{a.seats}</dd>
                        </dl>
                    }
                    .into_any()
                })
                aria_label="Accounts with details"
            />
        </Section>
    }
}

#[component]
fn FooterOptions() -> impl IntoView {
    view! {
        <Section
            title="Footer options"
            description="With paginated off and show_status off no footer is drawn (first table). footer_text replaces the sentence from a DataTableFooterInfo (second), footer_end adds content after the page controls, and toolbar_end adds a primary action to the toolbar."
        >
            <DataTable
                data=accounts()
                columns=base_columns()
                row_key=account_key
                paginated=false
                show_status=false
                aria_label="Accounts, no footer"
            />
            <DataTable
                data=accounts()
                columns=base_columns()
                row_key=account_key
                paginated=false
                footer_text=Callback::new(|i: DataTableFooterInfo| {
                    format!("{} accounts, {} shown", i.total, i.shown)
                })
                footer_end=ViewFn::from(|| {
                    view! { <Badge variant=BadgeVariant::Secondary>"Updated just now"</Badge> }
                })
                toolbar_end=ViewFn::from(|| {
                    view! {
                        <Button size=Size::Small>
                            <Icon name=IconName::Plus />
                            "Add account"
                        </Button>
                    }
                })
                searchable=true
                aria_label="Accounts, custom footer"
            />
        </Section>
    }
}

#[component]
fn InCard() -> impl IntoView {
    view! {
        <Section
            title="Inside a card"
            description="bordered=false drops the table's own border so the card is the frame; the card's content has no padding. The toolbar keeps its own padding row."
        >
            <Card class="gap-0 py-0">
                <CardHeader class="py-4">
                    <CardTitle>"Accounts"</CardTitle>
                    <CardDescription>"Renewals this quarter."</CardDescription>
                </CardHeader>
                <CardContent class="p-0">
                    <DataTable
                        data=accounts()
                        columns=base_columns()
                        row_key=account_key
                        bordered=false
                        wrap=true
                        paginated=false
                        aria_label="Accounts in a card"
                    />
                </CardContent>
            </Card>
        </Section>
    }
}
