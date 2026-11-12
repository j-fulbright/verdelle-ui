use crate::data_table_column::Sticky;
use crate::primitives::data_table::{
    ColumnFilter, ColumnModel, DataTableQuery, PageWindow, SelectionState, SortDirection, SortKey,
    cycle_sort, filter_indices, page_window, selection_state, set_selected, sort_direction_of,
    sort_indices, sort_rank_of, status_text, toggle_key,
};
use crate::{
    Align, Breakpoint, Button, ButtonVariant, DataTableColumn, DataTableState, DropdownMenu,
    DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuLabel, DropdownMenuSeparator,
    DropdownMenuTrigger, Empty, EmptyDescription, EmptyHeader, EmptyTitle, Input, NativeSelect,
    NativeSelectOption, RowState, Size, Skeleton, Table, TableBody, TableFooter, icons,
};
use leptos::prelude::*;
use std::sync::Arc;

/// The rows a table is showing right now, worked out from the data and the state.
struct Derived<T> {
    /// Rows that pass the search and filters (all pages), or the server's total.
    total: usize,
    window: PageWindow,
    /// The rows of the current page, in display order.
    rows: Vec<T>,
    keys: Vec<String>,
    /// Every row that passes the search and filters, only kept when a column has a footer.
    filtered: Vec<T>,
}

/// What the footer sentence is built from, passed to a data table's `footer_text`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableFooterInfo {
    /// Rows that pass the search and filters (all pages), or the server's total.
    pub total: usize,
    /// Index of the first row on this page, from 0.
    pub start: usize,
    /// How many rows are on this page.
    pub shown: usize,
    /// How many rows are selected (all pages).
    pub selected: usize,
    /// Whether the table has a selection column.
    pub selectable: bool,
    /// The page shown, from 0.
    pub page_index: usize,
    /// How many pages there are.
    pub page_count: usize,
}

/// The inline `style` of a column's header or cell: width (header only), min and max width.
fn column_style<T>(column: &DataTableColumn<T>, with_width: bool) -> Option<String> {
    let mut style = String::new();
    if let (true, Some(w)) = (with_width, &column.width) {
        style.push_str(&format!("width: {w};"));
    }
    if let Some(w) = &column.min_width {
        style.push_str(&format!("min-width: {w};"));
    }
    if let Some(w) = &column.max_width {
        style.push_str(&format!("max-width: {w};"));
    }
    (!style.is_empty()).then_some(style)
}

fn sticky_attr(sticky: Option<Sticky>) -> Option<&'static str> {
    sticky.map(|s| match s {
        Sticky::Start => "start",
        Sticky::End => "end",
    })
}

/// True when a click started on something interactive inside a row (a button, link, checkbox, menu
/// item), so the row's own click action should not run.
fn from_interactive(ev: &leptos::ev::MouseEvent) -> bool {
    event_target::<leptos::web_sys::Element>(ev)
        .closest(
            "button, a, input, select, textarea, label, [role='menuitem'], [data-no-row-click]",
        )
        .ok()
        .flatten()
        .is_some()
}

/// A data table: sorting, search, column filters, paging, row selection, column visibility, loading
/// and empty states, and a footer, on top of [`crate::Table`]. The headless logic is
/// `crate::primitives::data_table`; this draws it in shadcn's Data Table look.
///
/// ```ignore
/// <DataTable
///     data=rows
///     columns=vec![
///         DataTableColumn::text("name", "Name", |r: &Server| r.name.clone()),
///         DataTableColumn::new("cpu", "CPU", |r: &Server| format!("{}%", r.cpu))
///             .sort_by_key(|r| r.cpu)
///             .align(TextAlign::Right),
///     ]
///     row_key=|r: &Server| r.id.to_string()
///     searchable=true
///     selectable=true
/// />
/// ```
///
/// **Data.** `data` is a signal of the rows; the table never changes it. `row_key` gives each row a
/// stable string identity (selection and row clicks use it), so ids, not positions.
///
/// **Client-side (default).** The table filters, sorts and pages `data` itself. Every column that has a
/// comparator ([`DataTableColumn::sort_by`], [`DataTableColumn::text`]) gets a sort button in its header
/// (click: ascending, descending, off; with `multi_sort`, shift-click adds a column). The search box
/// (`searchable`) needs every word to appear in some column that has text. Only the current page's rows
/// are rendered, so a thousand rows is as cheap as ten.
///
/// **Server-side.** Pass `total_rows` and the table stops filtering, sorting and paging: it shows the
/// rows you give it, as one page, and takes the page count from `total_rows`. Watch
/// `on_query_change` (or the signals of a [`DataTableState`] you pass as `state`) to fetch the matching
/// page, and set `loading` while you wait. `on_query_change` also runs once on load, so the first fetch
/// uses the same path; it runs in the browser only. Debounce it yourself if each call costs a request.
///
/// **State.** Pass your own [`DataTableState`] to read or change the sort, search, filters, page,
/// selection or hidden columns from outside; otherwise the table keeps its own.
///
/// **Selection** (`selectable`) adds a checkbox column. The header box selects or clears the rows on the
/// current page and shows the mixed state when only some are selected. The selection is a set of row
/// keys that survives sorting, paging and filtering; `on_selection_change` reports it.
///
/// Rows are rebuilt when the data, sort, filter or page changes, not patched in place.
///
/// **Narrow screens.** The table box scrolls sideways by default. Options: hide minor columns below a
/// breakpoint ([`DataTableColumn::hide_below`]), pin the key column or the actions column while it
/// scrolls ([`DataTableColumn::sticky`], [`DataTableColumn::sticky_end`]), or set `stack_below` to
/// draw each row as a labelled card under a breakpoint (the header row is replaced by a small bar
/// with "select all" and a sort menu).
///
/// **Row state and details.** `row_class` and `row_state` style single rows (expired, past due).
/// `detail` adds a toggle column and a panel under each row that is open; the open rows are
/// `DataTableState::expanded`.
///
/// **Footer.** The sentence ("Showing 1-10 of 40 rows.") can be replaced (`footer_text`), hidden
/// (`show_status=false`) and the page controls dropped (`paginated=false`); with neither left the
/// footer is not drawn.
///
/// Not provided: column resizing, drag reordering, grouping, virtualized scrolling (paging keeps
/// the DOM small instead).
#[component]
pub fn DataTable<T, K>(
    #[prop(into)] data: Signal<Vec<T>>,
    columns: Vec<DataTableColumn<T>>,
    row_key: K,
    #[prop(optional)] state: Option<DataTableState>,
    /// Total number of rows on the server. Setting it switches the table to server-side mode.
    #[prop(optional, into)]
    total_rows: MaybeProp<usize>,
    /// Add a checkbox column.
    #[prop(optional)]
    selectable: bool,
    /// Show a search box above the table.
    #[prop(optional)]
    searchable: bool,
    #[prop(default = "Search...".into(), into)] search_placeholder: String,
    /// Show a "Columns" menu to hide and show columns.
    #[prop(optional)]
    column_menu: bool,
    /// Show the page controls and only render one page of rows (default true).
    #[prop(default = true)]
    paginated: bool,
    /// The choices of the "Rows per page" select.
    #[prop(default = vec![10, 20, 50, 100])]
    page_sizes: Vec<usize>,
    /// Show placeholder rows instead of the data (and set `aria-busy`).
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// How many placeholder rows to show while loading.
    #[prop(default = 5)]
    skeleton_rows: usize,
    /// Draw the table's own rounded border (default true). Turn it off inside a card, which frames it.
    #[prop(default = true)]
    bordered: bool,
    /// Let every cell wrap its content. Per column, use [`DataTableColumn::wrap`].
    #[prop(optional)]
    wrap: bool,
    /// Below this width draw each row as a card with the column labels beside the values.
    #[prop(optional)]
    stack_below: Option<Breakpoint>,
    /// Keep the header in view while the rows scroll. Give the table a height with `table_class`.
    #[prop(optional)]
    sticky_header: bool,
    #[prop(optional)] striped: bool,
    /// Tint the row under the pointer (default true).
    #[prop(default = true)]
    hoverable: bool,
    /// Let shift-click on a header add it to the sort instead of replacing it.
    #[prop(optional)]
    multi_sort: bool,
    /// Classes for a row, from the row's data (for example a dimmed or struck-through look).
    #[prop(optional, into)]
    row_class: Option<Callback<T, String>>,
    /// Emphasis for a row (dimmed, highlighted, destructive), from the row's data.
    #[prop(optional, into)]
    row_state: Option<Callback<T, RowState>>,
    /// Renders a panel under a row that is expanded, and adds a toggle column (a button with
    /// `aria-expanded`). Open rows are in [`DataTableState::expanded`]; pass your own state to open
    /// some from outside. Build the panel with `.into_any()`.
    #[prop(optional, into)]
    detail: Option<Callback<T, AnyView>>,
    /// Called with the row when it is clicked or, focused, activated with Enter or Space. Clicks on
    /// buttons, links, checkboxes and menus inside the row do not count. Makes rows focusable.
    #[prop(optional, into)]
    on_row_click: Option<Callback<T>>,
    /// Called with the whole query (sort, search, filters, page) whenever any part changes, and once
    /// on load. For fetching server-side data.
    #[prop(optional, into)]
    on_query_change: Option<Callback<DataTableQuery>>,
    /// Called with the sorted keys of the selected rows whenever the selection changes.
    #[prop(optional, into)]
    on_selection_change: Option<Callback<Vec<String>>>,
    /// What to show when no row matches. The default is an [`crate::Empty`] saying "No results".
    #[prop(optional, into)]
    empty: Option<ViewFn>,
    /// Extra controls placed after the search box, such as [`crate::DataTableColumnFilter`]s.
    #[prop(optional, into)]
    toolbar: Option<ViewFn>,
    /// Extra controls at the right end of the toolbar, before the "Columns" menu (a primary action).
    #[prop(optional, into)]
    toolbar_end: Option<ViewFn>,
    /// Show the footer sentence ("Showing 1-10 of 40 rows.", or the selection count). Default true.
    /// With `paginated=false` too, no footer is drawn.
    #[prop(default = true)]
    show_status: bool,
    /// Replaces the footer sentence.
    #[prop(optional, into)]
    footer_text: Option<Callback<DataTableFooterInfo, String>>,
    /// Extra content at the end of the footer, after the page controls.
    #[prop(optional, into)]
    footer_end: Option<ViewFn>,
    /// Accessible name for the table.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes for the whole component.
    #[prop(optional, into)]
    class: String,
    /// Extra classes for the scrolling table box, for a height (`max-h-96`) with `sticky_header`.
    #[prop(optional, into)]
    table_class: String,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    K: Fn(&T) -> String + Send + Sync + 'static,
{
    let own_state = state.is_none();
    let state = state.unwrap_or_default();
    let columns = Arc::new(columns);
    if own_state {
        for column in columns.iter().filter(|c| c.hidden_by_default) {
            state.set_column_visible(column.id(), false);
        }
    }
    let row_key = Arc::new(row_key);
    let models: Arc<Vec<ColumnModel<T>>> =
        Arc::new(columns.iter().map(|c| c.model.clone()).collect());
    let manual = total_rows.get_untracked().is_some();
    let has_footer = columns.iter().any(|c| c.footer.is_some());

    let derived = Memo::new_owning({
        let models = models.clone();
        let row_key = row_key.clone();
        move |_: Option<Derived<T>>| {
            let size = state.page_size.get();
            let page = state.page_index.get();
            let out = data.with(|rows| {
                if manual {
                    let total = total_rows.get().unwrap_or(rows.len());
                    Derived {
                        total,
                        window: page_window(total, page, size),
                        keys: rows.iter().map(|r| row_key(r)).collect(),
                        rows: rows.clone(),
                        filtered: if has_footer { rows.clone() } else { Vec::new() },
                    }
                } else {
                    let search = state.search.get();
                    let filters = state.column_filters.get();
                    let sort = state.sort.get();
                    let mut idx = filter_indices(rows, &models, &search, &filters);
                    sort_indices(rows, &mut idx, &models, &sort);
                    let total = idx.len();
                    let window = if paginated {
                        page_window(total, page, size)
                    } else {
                        page_window(total, 0, total.max(1))
                    };
                    let shown = &idx[window.range.clone()];
                    let page_rows: Vec<T> = shown.iter().map(|&i| rows[i].clone()).collect();
                    Derived {
                        total,
                        window,
                        keys: page_rows.iter().map(|r| row_key(r)).collect(),
                        rows: page_rows,
                        filtered: if has_footer {
                            idx.iter().map(|&i| rows[i].clone()).collect()
                        } else {
                            Vec::new()
                        },
                    }
                }
            });
            (out, true)
        }
    });

    // A new search, filter or sort starts again from the first page (as TanStack Table does).
    Effect::new(move |previous: Option<()>| {
        state.search.track();
        state.column_filters.track();
        state.sort.track();
        if previous.is_some() {
            state.page_index.set(0);
        }
    });
    if let Some(callback) = on_query_change {
        // Report the query, but not the half-updated one: when the search, filters or sort changed while
        // on a later page, wait for the reset to page 0 above so the app is called once, with page 0.
        Effect::new(
            move |previous: Option<(String, Vec<ColumnFilter>, Vec<SortKey>)>| {
                let query = state.query();
                let shape = (
                    query.search.clone(),
                    query.column_filters.clone(),
                    query.sort.clone(),
                );
                if previous.as_ref().is_some_and(|p| *p != shape) && query.page_index != 0 {
                    state.page_index.set(0);
                    return shape;
                }
                callback.run(query);
                shape
            },
        );
    }
    if let Some(callback) = on_selection_change {
        Effect::new(move |previous: Option<()>| {
            let keys = state.selected_keys();
            if previous.is_some() {
                callback.run(keys);
            }
        });
    }

    // The page-size select works on strings; keep it and the numeric state in step.
    let size_text = RwSignal::new(state.page_size.get_untracked().to_string());
    Effect::new(move |_| {
        if let Ok(n) = size_text.get().parse::<usize>()
            && n != state.page_size.get_untracked()
        {
            state.page_size.set(n);
            state.page_index.set(0);
        }
    });
    Effect::new(move |_| {
        let text = state.page_size.get().to_string();
        if size_text.get_untracked() != text {
            size_text.set(text);
        }
    });

    let visible = Memo::new({
        let columns = columns.clone();
        move |_| {
            state.hidden_columns.with(|hidden| {
                columns
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| !hidden.contains(c.id()))
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>()
            })
        }
    });
    let has_detail = detail.is_some();
    let lead_count = usize::from(selectable) + usize::from(has_detail);
    let column_count = move || visible.with(|v| v.len()) + lead_count;
    let sticky_start = columns.iter().any(|c| c.sticky == Some(Sticky::Start));
    // Left offset of the select column (after the toggle column), and of the first pinned data column.
    let select_left = if has_detail { "40px" } else { "0px" };
    let lead_px = format!("--vdl-lead: {}px;", lead_count * 40);
    let lead_cell = move |left: &str| sticky_start.then(|| format!("left: {left};"));

    let page_selection =
        Memo::new(move |_| derived.with(|d| state.selected.with(|s| selection_state(s, &d.keys))));

    // ---- header
    let header_cells = {
        let columns = columns.clone();
        move || {
            visible
                .get()
                .into_iter()
                .map(|i| {
                    let column = &columns[i];
                    let id = column.id().to_string();
                    let sortable = column.model.compare.is_some();
                    let label = if column.header_visible {
                        column.header.clone().into_any()
                    } else {
                        view! { <span class="vdl-sr-only">{column.header.clone()}</span> }
                            .into_any()
                    };
                    let aria_sort = {
                        let id = id.clone();
                        move || {
                            sortable.then(|| {
                                state
                                    .sort
                                    .with(|s| sort_direction_of(s, &id))
                                    .map_or("none", SortDirection::aria)
                            })
                        }
                    };
                    let content = if sortable {
                        let on_click_id = id.clone();
                        let icon_id = id.clone();
                        let rank_id = id.clone();
                        let active_id = id.clone();
                        view! {
                            <button
                                type="button"
                                class="vdl-data-table__sort"
                                data-align=column.align.attr()
                                on:click=move |ev| {
                                    let multi = multi_sort && ev.shift_key();
                                    state.sort.update(|s| *s = cycle_sort(s, &on_click_id, multi));
                                }
                            >
                                <span>{label}</span>
                                <span
                                    class="vdl-data-table__sort-icon"
                                    data-active=move || {
                                        state
                                            .sort
                                            .with(|s| sort_direction_of(s, &active_id).is_some())
                                            .then_some("")
                                    }
                                >
                                    {move || match state.sort.with(|s| sort_direction_of(s, &icon_id)) {
                                        Some(SortDirection::Asc) => icons::arrow_up().into_any(),
                                        Some(SortDirection::Desc) => icons::arrow_down().into_any(),
                                        None => icons::chevrons_up_down().into_any(),
                                    }}
                                </span>
                                <span class="vdl-data-table__sort-rank" aria-hidden="true">
                                    {move || {
                                        state.sort.with(|s| {
                                            if s.len() > 1 {
                                                sort_rank_of(s, &rank_id)
                                                    .map(|n| n.to_string())
                                                    .unwrap_or_default()
                                            } else {
                                                String::new()
                                            }
                                        })
                                    }}
                                </span>
                            </button>
                        }
                        .into_any()
                    } else {
                        label
                    };
                    view! {
                        <th
                            class=format!("vdl-table__head vdl-data-table__head {}", column.class)
                            scope="col"
                            data-align=column.align.attr()
                            data-wrap=column.wrap.then_some("")
                            data-hide-below=column.hide_below.map(Breakpoint::attr)
                            data-sticky=sticky_attr(column.sticky)
                            aria-sort=aria_sort
                            style=column_style(column, true)
                        >
                            {content}
                        </th>
                    }
                })
                .collect_view()
        }
    };

    // ---- body
    let select_all = move |ev: leptos::ev::Event| {
        let on = event_target_checked(&ev);
        let keys = derived.with(|d| d.keys.clone());
        state.selected.update(|s| set_selected(s, &keys, on));
        // The browser clears the mixed state on click; the reactive props set it again if it still holds.
        event_target::<leptos::web_sys::HtmlInputElement>(&ev)
            .set_indeterminate(page_selection.get_untracked() == SelectionState::Some);
    };

    let body_rows = {
        let columns = columns.clone();
        let empty = empty.clone();
        move || {
            let vis = visible.get();
            let span = column_count();
            if loading.get() {
                return (0..skeleton_rows)
                    .map(|_| {
                        view! {
                            <tr class="vdl-table__row" aria-hidden="true">
                                {has_detail
                                    .then(|| {
                                        view! {
                                            <td class="vdl-table__cell vdl-data-table__expand"></td>
                                        }
                                    })}
                                {selectable
                                    .then(|| {
                                        view! {
                                            <td class="vdl-table__cell vdl-data-table__select">
                                                <Skeleton class="vdl-data-table__skeleton vdl-data-table__skeleton--box" />
                                            </td>
                                        }
                                    })}
                                {vis
                                    .iter()
                                    .map(|&i| {
                                        view! {
                                            <td
                                                class="vdl-table__cell"
                                                data-hide-below=columns[i].hide_below.map(Breakpoint::attr)
                                            >
                                                <Skeleton class="vdl-data-table__skeleton" />
                                            </td>
                                        }
                                    })
                                    .collect_view()}
                            </tr>
                        }
                    })
                    .collect_view()
                    .into_any();
            }
            let (rows, keys) = derived.with(|d| (d.rows.clone(), d.keys.clone()));
            if rows.is_empty() {
                let content = match empty.clone() {
                    Some(custom) => custom.run(),
                    None => view! {
                        <Empty>
                            <EmptyHeader>
                                <EmptyTitle>"No results"</EmptyTitle>
                                <EmptyDescription>
                                    "Nothing matches. Try changing or clearing the search and filters."
                                </EmptyDescription>
                            </EmptyHeader>
                        </Empty>
                    }
                    .into_any(),
                };
                return view! {
                    <tr class="vdl-table__row vdl-data-table__empty-row">
                        <td class="vdl-table__cell" colspan=span>
                            {content}
                        </td>
                    </tr>
                }
                .into_any();
            }
            rows.into_iter()
                .zip(keys)
                .map(|(row, key)| {
                    let checked_key = key.clone();
                    let change_key = key.clone();
                    let selected_key = key.clone();
                    let toggle_key_ = key.clone();
                    let expanded_key = key.clone();
                    let aria_key = key.clone();
                    let detail_key = key;
                    let clickable = on_row_click.is_some();
                    let class_extra = row_class.map(|c| c.run(row.clone())).unwrap_or_default();
                    let emphasis = row_state.map(|c| c.run(row.clone())).unwrap_or_default();
                    let detail_row = row.clone();
                    let click_row = row.clone();
                    let key_row = row.clone();
                    let cells = vis
                        .iter()
                        .map(|&i| {
                            let column = &columns[i];
                            view! {
                                <td
                                    class=format!("vdl-table__cell {}", column.class)
                                    data-align=column.align.attr()
                                    data-wrap=column.wrap.then_some("")
                                    data-hide-below=column.hide_below.map(Breakpoint::attr)
                                    data-sticky=sticky_attr(column.sticky)
                                    data-label=column.header_visible.then(|| column.header.clone())
                                    style=column_style(column, false)
                                >
                                    {(column.cell)(&row)}
                                </td>
                            }
                        })
                        .collect_view();
                    let toggle_cell = has_detail
                        .then(|| {
                            let open = Signal::derive(move || {
                                state.expanded.with(|e| e.contains(&aria_key))
                            });
                            view! {
                                <td
                                    class="vdl-table__cell vdl-data-table__expand"
                                    data-sticky=sticky_start.then_some("start")
                                    style=lead_cell("0px")
                                >
                                    <button
                                        type="button"
                                        class="vdl-data-table__toggle"
                                        aria-label=move || {
                                            if open.get() { "Collapse row" } else { "Expand row" }
                                        }
                                        aria-expanded=move || open.get().to_string()
                                        on:click=move |_| {
                                            state.expanded.update(|e| {
                                                toggle_key(e, &toggle_key_);
                                            });
                                        }
                                    >
                                        {icons::chevron_right()}
                                    </button>
                                </td>
                            }
                        });
                    let detail_panel = detail
                        .map(|render| {
                            move || {
                                state
                                    .expanded
                                    .with(|e| e.contains(&detail_key))
                                    .then(|| {
                                        view! {
                                            <tr class="vdl-table__row vdl-data-table__detail-row">
                                                <td
                                                    class="vdl-table__cell vdl-data-table__detail"
                                                    colspan=span
                                                >
                                                    {render.run(detail_row.clone())}
                                                </td>
                                            </tr>
                                        }
                                    })
                            }
                        });
                    view! {
                        <tr
                            class=format!("vdl-table__row {class_extra}")
                            data-row-state=emphasis.attr()
                            data-expanded=move || {
                                state.expanded.with(|e| e.contains(&expanded_key)).then_some("")
                            }
                            data-selected=move || {
                                state.selected.with(|s| s.contains(&selected_key)).then_some("")
                            }
                            data-clickable=clickable.then_some("")
                            tabindex=clickable.then_some("0")
                            on:click=move |ev| {
                                if let Some(callback) = on_row_click
                                    && !from_interactive(&ev)
                                {
                                    callback.run(click_row.clone());
                                }
                            }
                            on:keydown=move |ev| {
                                if let Some(callback) = on_row_click {
                                    let own = ev.target() == ev.current_target();
                                    if own && (ev.key() == "Enter" || ev.key() == " ") {
                                        ev.prevent_default();
                                        callback.run(key_row.clone());
                                    }
                                }
                            }
                        >
                            {toggle_cell}
                            {selectable
                                .then(|| {
                                    view! {
                                        <td
                                            class="vdl-table__cell vdl-data-table__select"
                                            data-sticky=sticky_start.then_some("start")
                                            style=lead_cell(select_left)
                                        >
                                            <input
                                                type="checkbox"
                                                class="vdl-checkbox"
                                                aria-label="Select row"
                                                prop:checked=move || {
                                                    state.selected.with(|s| s.contains(&checked_key))
                                                }
                                                on:change=move |ev| {
                                                    let on = event_target_checked(&ev);
                                                    let key = change_key.clone();
                                                    state.selected.update(|s| {
                                                        if on {
                                                            s.insert(key);
                                                        } else {
                                                            s.remove(&key);
                                                        }
                                                    });
                                                }
                                            />
                                        </td>
                                    }
                                })}
                            {cells}
                        </tr>
                        {detail_panel}
                    }
                })
                .collect_view()
                .into_any()
        }
    };

    // ---- footer row of totals
    let footer_row = has_footer.then(|| {
        let columns = columns.clone();
        view! {
            <TableFooter>
                <tr class="vdl-table__row">
                    {(lead_count > 0)
                        .then(|| {
                            view! {
                                <td class="vdl-table__cell" colspan=lead_count.max(1)></td>
                            }
                        })}
                    {move || {
                        let rows = derived.with(|d| d.filtered.clone());
                        visible
                            .get()
                            .into_iter()
                            .map(|i| {
                                let column = &columns[i];
                                view! {
                                    <td
                                        class=format!("vdl-table__cell {}", column.class)
                                        data-align=column.align.attr()
                                        data-wrap=column.wrap.then_some("")
                                        data-hide-below=column.hide_below.map(Breakpoint::attr)
                                        data-sticky=sticky_attr(column.sticky)
                                        data-label=column.header_visible.then(|| column.header.clone())
                                        style=column_style(column, false)
                                    >
                                        {column.footer.as_ref().map(|f| f(&rows))}
                                    </td>
                                }
                            })
                            .collect_view()
                    }}
                </tr>
            </TableFooter>
        }
    });

    // ---- toolbar
    let columns_menu = column_menu.then(|| {
        // (position, id, header) of the columns the menu may hide. In a StoredValue so the nested
        // children closures can each read it without moving it.
        let items = StoredValue::new(
            columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c.hideable)
                .map(|(i, c)| (i, c.id().to_string(), c.header.clone()))
                .collect::<Vec<_>>(),
        );
        view! {
            <DropdownMenu>
                <DropdownMenuTrigger variant=ButtonVariant::Outline size=Size::Small>
                    "Columns"
                    {icons::chevron_down()}
                </DropdownMenuTrigger>
                <DropdownMenuContent align=Align::End>
                    <DropdownMenuLabel>"Toggle columns"</DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    {move || {
                        items
                            .get_value()
                            .into_iter()
                            .map(|(i, id, header)| {
                                let checked_id = id.clone();
                                view! {
                                    <DropdownMenuCheckboxItem
                                        checked=Signal::derive(move || {
                                            state.hidden_columns.with(|h| !h.contains(&checked_id))
                                        })
                                        disabled=Signal::derive(move || {
                                            visible.with(|v| v.len() == 1 && v.contains(&i))
                                        })
                                        on_checked_change=Callback::new(move |on: bool| {
                                            state.set_column_visible(&id, on)
                                        })
                                    >
                                        {header.clone()}
                                    </DropdownMenuCheckboxItem>
                                }
                            })
                            .collect_view()
                    }}
                </DropdownMenuContent>
            </DropdownMenu>
        }
    });
    let has_toolbar = searchable || column_menu || toolbar.is_some() || toolbar_end.is_some();

    // ---- footer
    let status = move || {
        derived.with(|d| {
            let shown = if manual {
                d.rows.len()
            } else {
                d.window.range.len()
            };
            let selected = state.selected.with(|s| s.len());
            match footer_text {
                Some(custom) => custom.run(DataTableFooterInfo {
                    total: d.total,
                    start: d.window.range.start,
                    shown,
                    selected,
                    selectable,
                    page_index: d.window.page_index,
                    page_count: d.window.page_count,
                }),
                None => status_text(selectable, selected, d.total, d.window.range.start, shown),
            }
        })
    };
    let mut sizes = page_sizes;
    let current = state.page_size.get_untracked();
    if !sizes.contains(&current) {
        sizes.push(current);
    }
    sizes.sort_unstable();
    let size_options = sizes
        .into_iter()
        .map(|n| {
            view! {
                <NativeSelectOption value=n.to_string() selected=n == current>
                    {n.to_string()}
                </NativeSelectOption>
            }
        })
        .collect_view();
    let at_first = Signal::derive(move || !derived.with(|d| d.window.has_previous()));
    let at_last = Signal::derive(move || !derived.with(|d| d.window.has_next()));
    let pager = paginated.then(|| {
        view! {
            <div class="vdl-data-table__pager">
                <div class="vdl-data-table__page-size">
                    <span aria-hidden="true">"Rows per page"</span>
                    <NativeSelect value=size_text size=Size::Small aria_label="Rows per page">
                        {size_options}
                    </NativeSelect>
                </div>
                <span class="vdl-data-table__page-count">
                    {move || {
                        derived
                            .with(|d| {
                                format!("Page {} of {}", d.window.page_index + 1, d.window.page_count)
                            })
                    }}
                </span>
                <div class="vdl-data-table__pager-buttons">
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        icon_only=true
                        class="vdl-data-table__edge"
                        aria_label="Go to first page"
                        disabled=at_first
                        on_click=move |_| state.page_index.set(0)
                    >
                        {icons::chevrons_left()}
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        icon_only=true
                        aria_label="Go to previous page"
                        disabled=at_first
                        on_click=move |_| {
                            let page = derived.with_untracked(|d| d.window.page_index);
                            state.page_index.set(page.saturating_sub(1));
                        }
                    >
                        {icons::chevron_left()}
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        icon_only=true
                        aria_label="Go to next page"
                        disabled=at_last
                        on_click=move |_| {
                            let page = derived.with_untracked(|d| d.window.page_index);
                            state.page_index.set(page + 1);
                        }
                    >
                        {icons::chevron_right()}
                    </Button>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        icon_only=true
                        class="vdl-data-table__edge"
                        aria_label="Go to last page"
                        disabled=at_last
                        on_click=move |_| {
                            let last = derived.with_untracked(|d| d.window.page_count - 1);
                            state.page_index.set(last);
                        }
                    >
                        {icons::chevrons_right()}
                    </Button>
                </div>
            </div>
        }
    });

    // ---- stacked layout: the header row is hidden, so select-all and sorting move to a small bar.
    let sort_text = RwSignal::new(String::new());
    let sortable: Vec<(String, String)> = columns
        .iter()
        .filter(|c| c.model.compare.is_some())
        .map(|c| (c.id().to_string(), c.header.clone()))
        .collect();
    Effect::new(move |_| {
        let text = state.sort.with(|s| {
            s.first()
                .map(|k| format!("{}:{}", k.column, k.direction.aria()))
                .unwrap_or_default()
        });
        if sort_text.get_untracked() != text {
            sort_text.set(text);
        }
    });
    Effect::new(move |_| {
        let text = sort_text.get();
        let key = text.split_once(':').map(|(column, dir)| SortKey {
            column: column.to_string(),
            direction: if dir == "descending" {
                SortDirection::Desc
            } else {
                SortDirection::Asc
            },
        });
        let current = state.sort.with_untracked(|s| s.first().cloned());
        if key != current {
            state.sort.set(key.into_iter().collect());
        }
    });
    let stack_bar = stack_below.map(|_| {
        let sort_options = sortable
            .iter()
            .flat_map(|(id, header)| {
                [
                    (format!("{id}:ascending"), format!("{header} (ascending)")),
                    (format!("{id}:descending"), format!("{header} (descending)")),
                ]
            })
            .map(|(value, text)| view! { <NativeSelectOption value=value>{text}</NativeSelectOption> })
            .collect_view();
        let has_sort = !sortable.is_empty();
        view! {
            <div class="vdl-data-table__stack-bar">
                {selectable
                    .then(|| {
                        view! {
                            <label class="vdl-data-table__stack-select">
                                <input
                                    type="checkbox"
                                    class="vdl-checkbox"
                                    disabled=move || derived.with(|d| d.keys.is_empty())
                                    prop:checked=move || page_selection.get() == SelectionState::All
                                    prop:indeterminate=move || {
                                        page_selection.get() == SelectionState::Some
                                    }
                                    on:change=select_all
                                />
                                "Select all"
                            </label>
                        }
                    })}
                {has_sort
                    .then(|| {
                        view! {
                            <NativeSelect
                                value=sort_text
                                size=Size::Small
                                aria_label="Sort by"
                                class="vdl-data-table__stack-sort"
                            >
                                <NativeSelectOption value="">"Sort: default"</NativeSelectOption>
                                {sort_options}
                            </NativeSelect>
                        }
                    })}
            </div>
        }
    });
    let has_footer_bar = show_status || paginated || footer_end.is_some();

    view! {
        <div
            class=format!("vdl-data-table {class}")
            aria-busy=move || loading.get().then_some("true")
            data-stack-below=stack_below.map(Breakpoint::attr)
            style=lead_px
        >
            {has_toolbar
                .then(|| {
                    view! {
                        <div class="vdl-data-table__toolbar">
                            {searchable
                                .then(|| {
                                    view! {
                                        <div class="vdl-data-table__search">
                                            <Input
                                                value=state.search
                                                cleanable=true
                                                placeholder=search_placeholder.clone()
                                                aria_label="Search the table"
                                            />
                                        </div>
                                    }
                                })}
                            {toolbar.map(|t| t.run())}
                            <div class="vdl-data-table__toolbar-end">
                                {toolbar_end.map(|t| t.run())} {columns_menu}
                            </div>
                        </div>
                    }
                })}
            {stack_bar}
            <Table
                sticky_header=sticky_header
                bordered=bordered
                striped=striped
                hoverable=hoverable
                wrap=wrap
                stack_below=MaybeProp::from(stack_below)
                class=table_class
                aria_label=aria_label.unwrap_or_default()
            >
                <thead class="vdl-table__header">
                    <tr class="vdl-table__row">
                        {has_detail
                            .then(|| {
                                view! {
                                    <th
                                        class="vdl-table__head vdl-data-table__expand"
                                        scope="col"
                                        data-sticky=sticky_start.then_some("start")
                                        style=lead_cell("0px")
                                    >
                                        <span class="vdl-sr-only">"Expand"</span>
                                    </th>
                                }
                            })}
                        {selectable
                            .then(|| {
                                view! {
                                    <th
                                        class="vdl-table__head vdl-data-table__select"
                                        scope="col"
                                        data-sticky=sticky_start.then_some("start")
                                        style=lead_cell(select_left)
                                    >
                                        <input
                                            type="checkbox"
                                            class="vdl-checkbox"
                                            aria-label="Select all rows on this page"
                                            disabled=move || derived.with(|d| d.keys.is_empty())
                                            prop:checked=move || {
                                                page_selection.get() == SelectionState::All
                                            }
                                            prop:indeterminate=move || {
                                                page_selection.get() == SelectionState::Some
                                            }
                                            on:change=select_all
                                        />
                                    </th>
                                }
                            })} {header_cells}
                    </tr>
                </thead>
                <TableBody>{body_rows}</TableBody>
                {footer_row}
            </Table>
            {has_footer_bar
                .then(|| {
                    view! {
                        <div class="vdl-data-table__footer">
                            {show_status
                                .then(|| {
                                    view! {
                                        <div class="vdl-data-table__status" role="status">
                                            {status}
                                        </div>
                                    }
                                })} {pager} {footer_end.map(|f| f.run())}
                        </div>
                    }
                })}
        </div>
    }
}
