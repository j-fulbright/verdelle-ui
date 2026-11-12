use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use std::time::Duration;
use verdelle_ui::{
    Badge, BadgeVariant, Button, ButtonVariant, DataTable, DataTableColumn, DataTableColumnFilter,
    DataTableQuery, DataTableState, DropdownMenu, DropdownMenuContent, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger, Empty, EmptyContent,
    EmptyDescription, EmptyHeader, EmptyTitle, MenuItemVariant, Size, SortDirection, TextAlign,
};

#[derive(Clone, Debug, PartialEq)]
struct Server {
    id: u32,
    name: String,
    region: &'static str,
    status: &'static str,
    cpu: u32,
    memory: u32,
}

const NAMES: [&str; 6] = ["atlas", "borealis", "cirrus", "drift", "ember", "fathom"];
const REGIONS: [&str; 4] = ["us-east", "us-west", "eu-central", "ap-south"];
const STATUSES: [&str; 4] = ["Running", "Stopped", "Degraded", "Provisioning"];

/// `count` servers with varied but repeatable values.
fn servers(count: u32) -> Vec<Server> {
    (0..count)
        .map(|i| Server {
            id: i + 1,
            name: format!("{}-{:03}", NAMES[i as usize % NAMES.len()], i + 1),
            region: REGIONS[((i * 7) % 4) as usize],
            status: STATUSES[((i * 5 + i / 3) % 4) as usize],
            cpu: (i * 37 + 11) % 100,
            memory: 4 * (i % 8 + 1),
        })
        .collect()
}

fn key(server: &Server) -> String {
    server.id.to_string()
}

fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Running" => BadgeVariant::Default,
        "Stopped" => BadgeVariant::Secondary,
        "Degraded" => BadgeVariant::Destructive,
        _ => BadgeVariant::Outline,
    }
}

fn status_badge(status: &'static str) -> impl IntoView {
    view! { <Badge variant=status_variant(status)>{status}</Badge> }
}

fn status_options() -> Vec<String> {
    STATUSES.iter().map(|s| s.to_string()).collect()
}

fn region_options() -> Vec<String> {
    REGIONS.iter().map(|r| r.to_string()).collect()
}

fn no_servers() -> Vec<Server> {
    Vec::new()
}

fn status_column() -> DataTableColumn<Server> {
    DataTableColumn::new("status", "Status", |s: &Server| status_badge(s.status))
        .filter_text(|s| s.status.to_string())
        .sort_by_key(|s| s.status)
}

fn cpu_column() -> DataTableColumn<Server> {
    DataTableColumn::new("cpu", "CPU", |s: &Server| format!("{}%", s.cpu))
        .sort_by_key(|s| s.cpu)
        .align(TextAlign::Right)
}

fn memory_column() -> DataTableColumn<Server> {
    DataTableColumn::new("memory", "Memory", |s: &Server| format!("{} GB", s.memory))
        .sort_by_key(|s| s.memory)
        .align(TextAlign::Right)
}

/// name, region, status, cpu, memory: every column sortable and searchable.
fn columns() -> Vec<DataTableColumn<Server>> {
    vec![
        DataTableColumn::text("name", "Name", |s: &Server| s.name.clone()).hideable(false),
        DataTableColumn::text("region", "Region", |s: &Server| s.region.to_string()),
        status_column(),
        cpu_column(),
        memory_column(),
    ]
}

/// The same data with no sorting or searching: header text only.
fn plain_columns() -> Vec<DataTableColumn<Server>> {
    vec![
        DataTableColumn::new("name", "Name", |s: &Server| s.name.clone()),
        DataTableColumn::new("region", "Region", |s: &Server| s.region),
        DataTableColumn::new("status", "Status", |s: &Server| status_badge(s.status)),
        DataTableColumn::new("cpu", "CPU", |s: &Server| format!("{}%", s.cpu))
            .align(TextAlign::Right),
    ]
}

#[component]
pub fn DataTablesPage() -> impl IntoView {
    view! {
        <Page
            title="Data Table"
            summary="Sorting, search, column filters, paging, row selection, column visibility, loading and empty states, row actions and totals, on top of Table. The logic is verdelle_ui::primitives::data_table; DataTable draws it."
        >
            <Basic />
            <Sortable />
            <Filterable />
            <Paginated />
            <Selectable />
            <ColumnVisibility />
            <Totals />
            <Empties />
            <Loading />
            <RowActions />
            <ServerSide />
            <Sticky />
            <Large />
        </Page>
    }
}

#[component]
fn Basic() -> impl IntoView {
    view! {
        <Section
            title="Basic"
            description="Columns of plain cells with paginated off: just the table and a row count. striped tints every other row."
        >
            <DataTable
                data=servers(6)
                columns=plain_columns()
                row_key=key
                paginated=false
                striped=true
                aria_label="Servers"
            />
        </Section>
    }
}

#[component]
fn Sortable() -> impl IntoView {
    let state = DataTableState::new();
    let multi = RwSignal::new(false);
    let describe = move || {
        state.sort.with(|sort| {
            if sort.is_empty() {
                "not sorted".to_string()
            } else {
                sort.iter()
                    .map(|k| {
                        let dir = match k.direction {
                            SortDirection::Asc => "ascending",
                            SortDirection::Desc => "descending",
                        };
                        format!("{} {}", k.column, dir)
                    })
                    .collect::<Vec<_>>()
                    .join(", then ")
            }
        })
    };
    view! {
        <Section
            title="Sortable"
            description="Click a header: ascending, descending, off. A header keeps aria-sort. With multi_sort, shift-click adds a column and a small number shows the priority. The sort lives in a DataTableState passed as state."
        >
            <Row>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    selected=multi
                    on_click=move |_| multi.update(|m| *m = !*m)
                >
                    "Multi-sort mode: "
                    {move || if multi.get() { "shift-click adds" } else { "off (click replaces)" }}
                </Button>
                <Button
                    variant=ButtonVariant::Ghost
                    size=Size::Small
                    on_click=move |_| state.sort.set(Vec::new())
                >
                    "Clear sort"
                </Button>
                <span class="text-sm text-muted-foreground">"Sort: " {describe}</span>
            </Row>
            {move || {
                // Multi-sort is a plain prop, so rebuild the table when it changes.
                view! {
                    <DataTable
                        data=servers(12)
                        columns=columns()
                        row_key=key
                        state=state
                        multi_sort=multi.get()
                        page_sizes=vec![5, 10, 25]
                        aria_label="Sortable servers"
                    />
                }
            }}
        </Section>
    }
}

#[component]
fn Filterable() -> impl IntoView {
    let state = DataTableState::new();
    view! {
        <Section
            title="Search and column filters"
            description="The search box needs every word to appear in some column (try 'east running', or 'cirrus'). The Status select filters by exact status. Both reset the page; Clear filters resets them and the select."
        >
            <DataTable
                data=servers(40)
                columns=columns()
                row_key=key
                state=state
                searchable=true
                search_placeholder="Search servers..."
                toolbar=move || {
                    view! {
                        <DataTableColumnFilter
                            state=state
                            column="status"
                            options=status_options()
                            all_label="All statuses"
                        />
                        <DataTableColumnFilter
                            state=state
                            column="region"
                            options=region_options()
                            all_label="All regions"
                        />
                        <Button
                            variant=ButtonVariant::Ghost
                            size=Size::Small
                            on_click=move |_| state.clear_filters()
                        >
                            "Clear filters"
                        </Button>
                    }
                }
                aria_label="Filterable servers"
            />
        </Section>
    }
}

#[component]
fn Paginated() -> impl IntoView {
    view! {
        <Section
            title="Pagination"
            description="Rows per page, first/previous/next/last, and 'Page x of y'. This one starts at 5 rows a page; a page past the end after filtering falls back to the last page."
        >
            <DataTable
                data=servers(57)
                columns=columns()
                row_key=key
                state=DataTableState::with_page_size(5)
                page_sizes=vec![5, 10, 20]
                aria_label="Paginated servers"
            />
        </Section>
    }
}

#[component]
fn Selectable() -> impl IntoView {
    let state = DataTableState::with_page_size(5);
    let reported = RwSignal::new(Vec::<String>::new());
    let on_selection = Callback::new(move |keys: Vec<String>| reported.set(keys));
    view! {
        <Section
            title="Row selection"
            description="The header checkbox selects the rows on this page and shows the mixed state when only some are selected. The selection is a set of row keys, so it survives paging and sorting. The footer counts it; on_selection_change reports it."
        >
            <Row>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    on_click=move |_| state.clear_selection()
                >
                    "Clear selection"
                </Button>
                <span class="text-sm text-muted-foreground">
                    "Selected ids: "
                    {move || {
                        let keys = state.selected_keys();
                        if keys.is_empty() { "none".to_string() } else { keys.join(", ") }
                    }}
                </span>
            </Row>
            <DataTable
                data=servers(23)
                columns=columns()
                row_key=key
                state=state
                selectable=true
                page_sizes=vec![5, 10]
                on_selection_change=on_selection
                aria_label="Selectable servers"
            />
            <p class="text-sm text-muted-foreground">
                "Last on_selection_change: " {move || reported.get().len().to_string()} " row(s)"
            </p>
        </Section>
    }
}

#[component]
fn ColumnVisibility() -> impl IntoView {
    view! {
        <Section
            title="Column visibility"
            description="The Columns menu toggles each column; Name is hideable(false) and Memory starts hidden (hidden_by_default). The last visible column cannot be hidden."
        >
            <DataTable
                data=servers(8)
                columns=vec![
                    DataTableColumn::text("name", "Name", |s: &Server| s.name.clone())
                        .hideable(false),
                    DataTableColumn::text("region", "Region", |s: &Server| s.region.to_string()),
                    status_column(),
                    cpu_column(),
                    memory_column().hidden_by_default(),
                ]
                row_key=key
                column_menu=true
                paginated=false
                aria_label="Servers with column menu"
            />
        </Section>
    }
}

#[component]
fn Totals() -> impl IntoView {
    view! {
        <Section
            title="Footer totals"
            description="A column's footer sees every row that passes the search and filters, across all pages. Search for 'atlas' and the totals follow."
        >
            <DataTable
                data=servers(30)
                columns=vec![
                    DataTableColumn::text("name", "Name", |s: &Server| s.name.clone())
                        .footer(|rows| format!("{} servers", rows.len())),
                    DataTableColumn::text("region", "Region", |s: &Server| s.region.to_string()),
                    cpu_column()
                        .footer(|rows| {
                            let total: u32 = rows.iter().map(|r| r.cpu).sum();
                            let average = if rows.is_empty() { 0 } else { total / rows.len() as u32 };
                            format!("{average}% avg")
                        }),
                    memory_column()
                        .footer(|rows| format!("{} GB", rows.iter().map(|r| r.memory).sum::<u32>())),
                ]
                row_key=key
                searchable=true
                page_sizes=vec![5, 10]
                state=DataTableState::with_page_size(5)
                aria_label="Servers with totals"
            />
        </Section>
    }
}

#[component]
fn Empties() -> impl IntoView {
    view! {
        <Section
            title="Empty states"
            description="No rows shows an Empty (the default says 'No results'); pass empty for your own. Search for something that does not exist to see the default on a table that has data."
        >
            <DataTable
                data=no_servers()
                columns=plain_columns()
                row_key=key
                paginated=false
                aria_label="No servers"
                empty=move || {
                    view! {
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"No servers yet"</EmptyTitle>
                                <EmptyDescription>"Create one to see it here."</EmptyDescription>
                            </EmptyHeader>
                            <EmptyContent>
                                <Button size=Size::Small>"Create server"</Button>
                            </EmptyContent>
                        </Empty>
                    }
                }
            />
            <DataTable
                data=servers(10)
                columns=columns()
                row_key=key
                searchable=true
                search_placeholder="Try 'zzz'..."
                paginated=false
                aria_label="Servers, search for nothing"
            />
        </Section>
    }
}

#[component]
fn Loading() -> impl IntoView {
    let loading = RwSignal::new(true);
    view! {
        <Section
            title="Loading"
            description="loading shows skeleton rows (skeleton_rows of them) in place of the data and sets aria-busy on the table."
        >
            <Row>
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    selected=loading
                    on_click=move |_| loading.update(|l| *l = !*l)
                >
                    "Loading"
                </Button>
            </Row>
            <DataTable
                data=servers(4)
                columns=columns()
                row_key=key
                selectable=true
                loading=loading
                skeleton_rows=4
                paginated=false
                aria_label="Servers loading"
            />
        </Section>
    }
}

#[component]
fn RowActions() -> impl IntoView {
    let last = RwSignal::new("Nothing yet".to_string());
    let mut actions = columns();
    actions.push(
        DataTableColumn::new("actions", "Actions", move |s: &Server| {
            let id = s.id;
            let name = s.name.clone();
            let label = StoredValue::new(format!("Open menu for {name}"));
            view! {
                <DropdownMenu>
                    <DropdownMenuTrigger variant=ButtonVariant::Ghost size=Size::Small icon_only=true>
                        <span class="vdl-sr-only">{move || label.get_value()}</span>
                        <svg
                            viewBox="0 0 24 24"
                            width="16"
                            height="16"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            aria-hidden="true"
                        >
                            <circle cx="12" cy="12" r="1" />
                            <circle cx="19" cy="12" r="1" />
                            <circle cx="5" cy="12" r="1" />
                        </svg>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align=verdelle_ui::Align::End>
                        <DropdownMenuLabel>"Actions"</DropdownMenuLabel>
                        <DropdownMenuItem on_select=Callback::new(move |()| {
                            last.set(format!("Copied id {id}"))
                        })>"Copy ID"</DropdownMenuItem>
                        <DropdownMenuItem on_select=Callback::new(move |()| {
                            last.set(format!("Opened server {id}"))
                        })>"View details"</DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem
                            variant=MenuItemVariant::Destructive
                            on_select=Callback::new(move |()| last.set(format!("Deleted server {id}")))
                        >
                            "Delete"
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            }
        })
        .header_hidden()
        .hideable(false)
        .align(TextAlign::Right)
        .width("56px"),
    );
    view! {
        <Section
            title="Row actions and row click"
            description="An actions column holds a Dropdown Menu (its header is hidden visually but read by screen readers). Clicking the row, or pressing Enter on a focused row, calls on_row_click; clicks on the menu button do not."
        >
            <p class="text-sm text-muted-foreground" role="status">
                "Last event: " {move || last.get()}
            </p>
            <DataTable
                data=servers(8)
                columns=actions
                row_key=key
                paginated=false
                on_row_click=move |s: Server| last.set(format!("Clicked row {} ({})", s.id, s.name))
                aria_label="Servers with actions"
            />
        </Section>
    }
}

/// What a server would do for `query`: search by name, region and status, sort by one column, slice a
/// page. Stands in for an HTTP call.
fn fake_server(query: &DataTableQuery) -> (Vec<Server>, usize) {
    let mut rows: Vec<Server> = servers(237);
    let words: Vec<String> = query
        .search
        .split_whitespace()
        .map(str::to_lowercase)
        .collect();
    rows.retain(|r| {
        let hay = format!("{} {} {}", r.name, r.region, r.status).to_lowercase();
        words.iter().all(|w| hay.contains(w))
    });
    if let Some(sort) = query.sort.first() {
        rows.sort_by(|a, b| {
            let order = match sort.column.as_str() {
                "name" => a.name.cmp(&b.name),
                "region" => a.region.cmp(b.region),
                "status" => a.status.cmp(b.status),
                "cpu" => a.cpu.cmp(&b.cpu),
                _ => a.memory.cmp(&b.memory),
            };
            if sort.direction == SortDirection::Desc {
                order.reverse()
            } else {
                order
            }
        });
    }
    let total = rows.len();
    let start = (query.page_index * query.page_size).min(total);
    let end = (start + query.page_size).min(total);
    (rows[start..end].to_vec(), total)
}

#[component]
fn ServerSide() -> impl IntoView {
    let state = DataTableState::with_page_size(10);
    let rows = RwSignal::new(Vec::<Server>::new());
    let total = RwSignal::new(0_usize);
    let loading = RwSignal::new(true);
    let requests = RwSignal::new(0_u32);
    let on_query = move |query: DataTableQuery| {
        loading.set(true);
        requests.update(|n| *n += 1);
        let ticket = requests.get_untracked();
        set_timeout(
            move || {
                // A newer request replaced this one: drop the stale answer.
                if requests.get_untracked() != ticket {
                    return;
                }
                let (page, count) = fake_server(&query);
                rows.set(page);
                total.set(count);
                loading.set(false);
            },
            Duration::from_millis(500),
        );
    };
    view! {
        <Section
            title="Server-side (controlled)"
            description="Passing total_rows turns off the table's own sorting, filtering and paging: it shows the rows it is given. on_query_change hands over the sort, search and page (also once on load) and this page answers after a 500 ms fake delay, dropping stale answers. Sorting, searching and paging happen on the fake server, over 237 rows. A new sort, search or filter goes back to page 1 and calls once."
        >
            <p class="text-sm text-muted-foreground">
                "Requests sent: " {move || requests.get().to_string()}
            </p>
            <DataTable
                data=rows
                columns=columns()
                row_key=key
                state=state
                total_rows=total
                loading=loading
                searchable=true
                search_placeholder="Search the server..."
                skeleton_rows=5
                on_query_change=on_query
                aria_label="Servers from a fake API"
            />
        </Section>
    }
}

#[component]
fn Sticky() -> impl IntoView {
    view! {
        <Section
            title="Sticky header and horizontal scroll"
            description="sticky_header keeps the header while the rows scroll inside table_class (here max-h-72). The box also scrolls sideways when the columns do not fit; narrow the window to see it."
        >
            <DataTable
                data=servers(40)
                columns=columns()
                row_key=key
                paginated=false
                sticky_header=true
                table_class="max-h-72"
                aria_label="Servers, scrolling"
            />
            <div class="max-w-xs">
                <DataTable
                    data=servers(4)
                    columns=columns()
                    row_key=key
                    paginated=false
                    aria_label="Servers in a narrow box"
                />
            </div>
        </Section>
    }
}

#[component]
fn Large() -> impl IntoView {
    view! {
        <Section
            title="1000 rows"
            description="Only the current page is rendered, so sorting, searching and paging a thousand rows stays fast. Select a page, sort, search; selection counts carry across pages."
        >
            <DataTable
                data=servers(1000)
                columns=columns()
                row_key=key
                state=DataTableState::with_page_size(25)
                searchable=true
                selectable=true
                column_menu=true
                page_sizes=vec![25, 50, 100]
                multi_sort=true
                aria_label="A thousand servers"
            />
        </Section>
    }
}
