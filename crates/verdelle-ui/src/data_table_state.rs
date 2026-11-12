use crate::primitives::data_table::{ColumnFilter, DataTableQuery, SortKey};
use leptos::prelude::*;
use std::collections::HashSet;

/// The state of a [`crate::DataTable`]: sort, search, column filters, page, selection and hidden
/// columns, each a signal. The table makes one for itself; create your own and pass it as `state` to
/// read or change any of it from outside (a "clear filters" button, a selection toolbar, restoring a
/// saved view) or to keep it when the table is re-created.
///
/// It is `Copy`, so it moves into closures freely. Row identity is the string your `row_key`
/// returns; the selection is a set of those keys, so it survives paging, sorting and filtering.
#[derive(Clone, Copy)]
pub struct DataTableState {
    /// Sorted columns, most significant first.
    pub sort: RwSignal<Vec<SortKey>>,
    /// The search box text. Rows must match every word in some column's text.
    pub search: RwSignal<String>,
    pub column_filters: RwSignal<Vec<ColumnFilter>>,
    /// The page shown, from 0.
    pub page_index: RwSignal<usize>,
    pub page_size: RwSignal<usize>,
    /// Keys of the selected rows.
    pub selected: RwSignal<HashSet<String>>,
    /// Ids of the columns hidden through the column menu.
    pub hidden_columns: RwSignal<HashSet<String>>,
    /// Keys of the rows whose detail panel is open (see `DataTable`'s `detail`).
    pub expanded: RwSignal<HashSet<String>>,
}

impl Default for DataTableState {
    fn default() -> Self {
        Self::new()
    }
}

impl DataTableState {
    /// An unsorted, unfiltered state on the first page of 10 rows.
    pub fn new() -> Self {
        Self::with_page_size(10)
    }

    pub fn with_page_size(page_size: usize) -> Self {
        DataTableState {
            sort: RwSignal::new(Vec::new()),
            search: RwSignal::new(String::new()),
            column_filters: RwSignal::new(Vec::new()),
            page_index: RwSignal::new(0),
            page_size: RwSignal::new(page_size.max(1)),
            selected: RwSignal::new(HashSet::new()),
            hidden_columns: RwSignal::new(HashSet::new()),
            expanded: RwSignal::new(HashSet::new()),
        }
    }

    /// A snapshot of what decides the rows shown. Reading it inside an effect or closure tracks every
    /// part of it.
    pub fn query(&self) -> DataTableQuery {
        DataTableQuery {
            sort: self.sort.get(),
            search: self.search.get(),
            column_filters: self.column_filters.get(),
            page_index: self.page_index.get(),
            page_size: self.page_size.get(),
        }
    }

    /// The current filter value for `column` (empty when there is none).
    pub fn column_filter(&self, column: &str) -> String {
        self.column_filters.with(|filters| {
            filters
                .iter()
                .find(|f| f.column == column)
                .map(|f| f.value.clone())
                .unwrap_or_default()
        })
    }

    /// Set the filter for `column`; an empty `value` removes it.
    pub fn set_column_filter(&self, column: &str, value: &str) {
        self.column_filters.update(|filters| {
            filters.retain(|f| f.column != column);
            if !value.is_empty() {
                filters.push(ColumnFilter {
                    column: column.to_string(),
                    value: value.to_string(),
                });
            }
        });
    }

    /// Clear the search and every column filter.
    pub fn clear_filters(&self) {
        self.search.set(String::new());
        self.column_filters.set(Vec::new());
    }

    /// The selected row keys, sorted.
    pub fn selected_keys(&self) -> Vec<String> {
        self.selected.with(|s| {
            let mut keys: Vec<String> = s.iter().cloned().collect();
            keys.sort();
            keys
        })
    }

    pub fn clear_selection(&self) {
        self.selected.set(HashSet::new());
    }

    /// Show or hide a column (as the column menu does).
    pub fn set_column_visible(&self, column: &str, visible: bool) {
        self.hidden_columns.update(|hidden| {
            if visible {
                hidden.remove(column);
            } else {
                hidden.insert(column.to_string());
            }
        });
    }
}
