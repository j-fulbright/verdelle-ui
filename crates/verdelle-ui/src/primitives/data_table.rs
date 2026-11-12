//! The headless model behind a data table: sorting, filtering, paging and row selection as plain
//! functions over a slice of rows. It does no rendering and touches no DOM, so it is testable natively and
//! usable with any markup. `verdelle_ui::DataTable` is the styled component built on it.
//!
//! Rows are never copied or reordered in place: every step works on a list of row *indices* into the
//! caller's slice (`filter_indices` -> `sort_indices` -> `page_window`), which keeps a table of a few
//! thousand rows cheap to re-sort on each keystroke.
//!
//! This is original Verdelle code (not a Leptix port): TanStack Table, the engine behind shadcn's Data
//! Table guide, is React-only.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;

/// Which way a column is sorted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    /// The value of the `aria-sort` attribute.
    pub fn aria(self) -> &'static str {
        match self {
            SortDirection::Asc => "ascending",
            SortDirection::Desc => "descending",
        }
    }
}

/// One sorted column. A table's sort is a list of these, most significant first.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SortKey {
    pub column: String,
    pub direction: SortDirection,
}

/// A column filter: the row's text for `column` must equal `value` (ignoring case). An empty `value`
/// matches every row, so a "All" choice needs no special handling.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColumnFilter {
    pub column: String,
    pub value: String,
}

/// Everything that decides which rows a table shows. Handed to the app on each change so it can fetch
/// the matching page from a server (see `DataTable`'s `on_query_change`).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DataTableQuery {
    pub sort: Vec<SortKey>,
    pub search: String,
    pub column_filters: Vec<ColumnFilter>,
    pub page_index: usize,
    pub page_size: usize,
}

type TextFn<T> = Arc<dyn Fn(&T) -> String + Send + Sync>;
type CompareFn<T> = Arc<dyn Fn(&T, &T) -> Ordering + Send + Sync>;

/// What the model needs to know about a column: its id, the text a search and column filters look at, and
/// how two rows compare when sorting. A column with no `text` is skipped by searching and filtering; one
/// with no `compare` cannot be sorted.
pub struct ColumnModel<T> {
    pub id: String,
    pub text: Option<TextFn<T>>,
    pub compare: Option<CompareFn<T>>,
}

impl<T> Clone for ColumnModel<T> {
    fn clone(&self) -> Self {
        ColumnModel {
            id: self.id.clone(),
            text: self.text.clone(),
            compare: self.compare.clone(),
        }
    }
}

impl<T> ColumnModel<T> {
    pub fn new(id: impl Into<String>) -> Self {
        ColumnModel {
            id: id.into(),
            text: None,
            compare: None,
        }
    }
}

/// The direction `column` is sorted in, if it is.
pub fn sort_direction_of(sort: &[SortKey], column: &str) -> Option<SortDirection> {
    sort.iter()
        .find(|k| k.column == column)
        .map(|k| k.direction)
}

/// The position (1-based) of `column` in a multi-column sort, if it is sorted.
pub fn sort_rank_of(sort: &[SortKey], column: &str) -> Option<usize> {
    sort.iter().position(|k| k.column == column).map(|i| i + 1)
}

/// The sort after activating `column`'s header: unsorted -> ascending -> descending -> unsorted.
///
/// Without `multi` the other columns' sorts are dropped (the usual click). With `multi` (shift-click)
/// they are kept, and the clicked column cycles in place, or is added as the least significant key.
pub fn cycle_sort(current: &[SortKey], column: &str, multi: bool) -> Vec<SortKey> {
    let next = match sort_direction_of(current, column) {
        None => Some(SortDirection::Asc),
        Some(SortDirection::Asc) => Some(SortDirection::Desc),
        Some(SortDirection::Desc) => None,
    };
    let mut out: Vec<SortKey> = if multi {
        current
            .iter()
            .filter(|k| k.column != column)
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    if let Some(direction) = next {
        let key = SortKey {
            column: column.to_string(),
            direction,
        };
        if multi {
            // Keep the column where it was in the order, so cycling does not reshuffle priorities.
            let at = current
                .iter()
                .position(|k| k.column == column)
                .unwrap_or(out.len())
                .min(out.len());
            out.insert(at, key);
        } else {
            out.push(key);
        }
    }
    out
}

fn column<'a, T>(columns: &'a [ColumnModel<T>], id: &str) -> Option<&'a ColumnModel<T>> {
    columns.iter().find(|c| c.id == id)
}

/// The indices of the rows that pass the search and the column filters, in their original order.
///
/// The search is split on whitespace; every word must appear (ignoring case) in the text of at least one
/// column. A blank search matches everything. Column filters must all hold. A filter on an unknown column
/// or one with no `text` is ignored rather than hiding every row.
pub fn filter_indices<T>(
    rows: &[T],
    columns: &[ColumnModel<T>],
    search: &str,
    column_filters: &[ColumnFilter],
) -> Vec<usize> {
    let words: Vec<String> = search.split_whitespace().map(str::to_lowercase).collect();
    let filters: Vec<(&TextFn<T>, String)> = column_filters
        .iter()
        .filter(|f| !f.value.is_empty())
        .filter_map(|f| {
            let text = column(columns, &f.column)?.text.as_ref()?;
            Some((text, f.value.to_lowercase()))
        })
        .collect();
    let searchable: Vec<&TextFn<T>> = columns.iter().filter_map(|c| c.text.as_ref()).collect();

    rows.iter()
        .enumerate()
        .filter(|(_, row)| {
            filters
                .iter()
                .all(|(text, want)| text(row).to_lowercase() == *want)
                && (words.is_empty() || {
                    let haystack: Vec<String> =
                        searchable.iter().map(|t| t(row).to_lowercase()).collect();
                    words.iter().all(|w| haystack.iter().any(|h| h.contains(w)))
                })
        })
        .map(|(i, _)| i)
        .collect()
}

/// Sort `indices` (indices into `rows`) by the sort keys. Stable, so rows that compare equal keep their
/// order, and later keys break ties of earlier ones. A key for an unknown or non-sortable column is
/// skipped.
pub fn sort_indices<T>(
    rows: &[T],
    indices: &mut [usize],
    columns: &[ColumnModel<T>],
    sort: &[SortKey],
) {
    let keys: Vec<(&CompareFn<T>, SortDirection)> = sort
        .iter()
        .filter_map(|k| Some((column(columns, &k.column)?.compare.as_ref()?, k.direction)))
        .collect();
    if keys.is_empty() {
        return;
    }
    indices.sort_by(|&a, &b| {
        for (compare, direction) in &keys {
            let ord = compare(&rows[a], &rows[b]);
            let ord = match direction {
                SortDirection::Asc => ord,
                SortDirection::Desc => ord.reverse(),
            };
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    });
}

/// Where a page sits in a list of `total` rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageWindow {
    /// The page shown, clamped into range (so a page past the end after filtering shows the last page).
    pub page_index: usize,
    pub page_count: usize,
    /// The positions (in the sorted, filtered list) on this page.
    pub range: Range<usize>,
}

impl PageWindow {
    pub fn has_previous(&self) -> bool {
        self.page_index > 0
    }
    pub fn has_next(&self) -> bool {
        self.page_index + 1 < self.page_count
    }
}

/// The window of page `page_index` (0-based) of `total` rows at `page_size` rows a page. An empty list has
/// one empty page, so "page 1 of 1" never reads "page 1 of 0". A `page_size` of 0 is treated as 1.
pub fn page_window(total: usize, page_index: usize, page_size: usize) -> PageWindow {
    let size = page_size.max(1);
    let page_count = total.div_ceil(size).max(1);
    let page_index = page_index.min(page_count - 1);
    let start = (page_index * size).min(total);
    let end = (start + size).min(total);
    PageWindow {
        page_index,
        page_count,
        range: start..end,
    }
}

/// How much of a set of rows is selected, for a header checkbox.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionState {
    None,
    Some,
    All,
}

/// Whether none, some or all of `keys` are in `selected`. An empty `keys` is `None` (nothing to select).
pub fn selection_state(selected: &HashSet<String>, keys: &[String]) -> SelectionState {
    let count = keys.iter().filter(|k| selected.contains(*k)).count();
    if count == 0 {
        SelectionState::None
    } else if count == keys.len() {
        SelectionState::All
    } else {
        SelectionState::Some
    }
}

/// Select or deselect every one of `keys`, leaving the rest of the selection alone.
pub fn set_selected(selected: &mut HashSet<String>, keys: &[String], on: bool) {
    for key in keys {
        if on {
            selected.insert(key.clone());
        } else {
            selected.remove(key);
        }
    }
}

/// The distinct non-empty texts of a column, sorted case-insensitively: the choices for a column filter.
pub fn distinct_values<T>(rows: &[T], column: &ColumnModel<T>) -> Vec<String> {
    let Some(text) = column.text.as_ref() else {
        return Vec::new();
    };
    let mut values: Vec<String> = rows
        .iter()
        .map(|r| text(r))
        .filter(|v| !v.is_empty())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    values.sort_by_key(|v| v.to_lowercase());
    values
}

/// Flip `key` in a set of keys (expanded rows, say). Returns true when the key is now in the set.
pub fn toggle_key(keys: &mut HashSet<String>, key: &str) -> bool {
    if keys.remove(key) {
        false
    } else {
        keys.insert(key.to_string());
        true
    }
}

/// The default footer sentence: "2 of 40 row(s) selected." when rows are selectable, otherwise
/// "Showing 1-10 of 40 rows." (`start` is the 0-based index of the first shown row, `shown` how many
/// rows are on this page) or "No rows.".
pub fn status_text(
    selectable: bool,
    selected: usize,
    total: usize,
    start: usize,
    shown: usize,
) -> String {
    if selectable {
        format!("{selected} of {total} row(s) selected.")
    } else if total == 0 {
        "No rows.".to_string()
    } else {
        format!("Showing {}-{} of {} rows.", start + 1, start + shown, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Row {
        name: &'static str,
        team: &'static str,
        age: u32,
    }

    fn rows() -> Vec<Row> {
        vec![
            Row {
                name: "Ada",
                team: "Core",
                age: 36,
            },
            Row {
                name: "Bo",
                team: "Web",
                age: 29,
            },
            Row {
                name: "Cy",
                team: "Core",
                age: 29,
            },
            Row {
                name: "Di",
                team: "Web",
                age: 41,
            },
            Row {
                name: "Ed",
                team: "Ops",
                age: 29,
            },
        ]
    }

    fn columns() -> Vec<ColumnModel<Row>> {
        vec![
            ColumnModel {
                id: "name".into(),
                text: Some(Arc::new(|r: &Row| r.name.to_string())),
                compare: Some(Arc::new(|a: &Row, b: &Row| a.name.cmp(b.name))),
            },
            ColumnModel {
                id: "team".into(),
                text: Some(Arc::new(|r: &Row| r.team.to_string())),
                compare: Some(Arc::new(|a: &Row, b: &Row| a.team.cmp(b.team))),
            },
            ColumnModel {
                id: "age".into(),
                text: None,
                compare: Some(Arc::new(|a: &Row, b: &Row| a.age.cmp(&b.age))),
            },
        ]
    }

    fn key(column: &str, direction: SortDirection) -> SortKey {
        SortKey {
            column: column.into(),
            direction,
        }
    }

    fn names(rows: &[Row], idx: &[usize]) -> Vec<&'static str> {
        idx.iter().map(|&i| rows[i].name).collect()
    }

    #[test]
    fn cycle_goes_asc_desc_none() {
        let a = cycle_sort(&[], "name", false);
        assert_eq!(a, vec![key("name", SortDirection::Asc)]);
        let d = cycle_sort(&a, "name", false);
        assert_eq!(d, vec![key("name", SortDirection::Desc)]);
        assert!(cycle_sort(&d, "name", false).is_empty());
    }

    #[test]
    fn plain_click_replaces_other_columns_multi_keeps_them() {
        let start = vec![key("team", SortDirection::Asc)];
        assert_eq!(
            cycle_sort(&start, "age", false),
            vec![key("age", SortDirection::Asc)]
        );
        assert_eq!(
            cycle_sort(&start, "age", true),
            vec![
                key("team", SortDirection::Asc),
                key("age", SortDirection::Asc)
            ]
        );
    }

    #[test]
    fn multi_cycle_keeps_priority_position_and_can_remove() {
        let s = vec![
            key("team", SortDirection::Asc),
            key("age", SortDirection::Asc),
            key("name", SortDirection::Asc),
        ];
        let s = cycle_sort(&s, "age", true);
        assert_eq!(s[1], key("age", SortDirection::Desc));
        assert_eq!(s.len(), 3);
        let s = cycle_sort(&s, "age", true);
        assert_eq!(
            s,
            vec![
                key("team", SortDirection::Asc),
                key("name", SortDirection::Asc)
            ]
        );
        assert_eq!(sort_rank_of(&s, "name"), Some(2));
        assert_eq!(sort_direction_of(&s, "age"), None);
    }

    #[test]
    fn sorts_ascending_and_descending() {
        let rows = rows();
        let cols = columns();
        let mut idx: Vec<usize> = (0..rows.len()).collect();
        sort_indices(&rows, &mut idx, &cols, &[key("name", SortDirection::Desc)]);
        assert_eq!(names(&rows, &idx), ["Ed", "Di", "Cy", "Bo", "Ada"]);
    }

    #[test]
    fn sort_is_stable_and_later_keys_break_ties() {
        let rows = rows();
        let cols = columns();
        // Age ascending: Bo, Cy and Ed are all 29 and keep their original order.
        let mut idx: Vec<usize> = (0..rows.len()).collect();
        sort_indices(&rows, &mut idx, &cols, &[key("age", SortDirection::Asc)]);
        assert_eq!(names(&rows, &idx), ["Bo", "Cy", "Ed", "Ada", "Di"]);
        // Then by name descending within an age.
        let mut idx: Vec<usize> = (0..rows.len()).collect();
        sort_indices(
            &rows,
            &mut idx,
            &cols,
            &[
                key("age", SortDirection::Asc),
                key("name", SortDirection::Desc),
            ],
        );
        assert_eq!(names(&rows, &idx), ["Ed", "Cy", "Bo", "Ada", "Di"]);
    }

    #[test]
    fn unknown_sort_column_leaves_order_alone() {
        let rows = rows();
        let cols = columns();
        let mut idx: Vec<usize> = (0..rows.len()).collect();
        sort_indices(&rows, &mut idx, &cols, &[key("nope", SortDirection::Asc)]);
        assert_eq!(idx, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn search_matches_any_text_column_ignoring_case() {
        let rows = rows();
        let cols = columns();
        assert_eq!(filter_indices(&rows, &cols, "web", &[]), vec![1, 3]);
        assert_eq!(filter_indices(&rows, &cols, "  ", &[]), vec![0, 1, 2, 3, 4]);
        assert!(filter_indices(&rows, &cols, "zzz", &[]).is_empty());
    }

    #[test]
    fn search_words_must_all_match_but_may_hit_different_columns() {
        let rows = rows();
        let cols = columns();
        // "cy" is a name, "core" a team.
        assert_eq!(filter_indices(&rows, &cols, "core cy", &[]), vec![2]);
        assert!(filter_indices(&rows, &cols, "ada web", &[]).is_empty());
    }

    #[test]
    fn age_has_no_text_so_search_ignores_it() {
        let rows = rows();
        let cols = columns();
        assert!(filter_indices(&rows, &cols, "29", &[]).is_empty());
    }

    #[test]
    fn column_filters_are_exact_and_combine_with_search() {
        let rows = rows();
        let cols = columns();
        let team = |v: &str| ColumnFilter {
            column: "team".into(),
            value: v.into(),
        };
        assert_eq!(
            filter_indices(&rows, &cols, "", &[team("CORE")]),
            vec![0, 2]
        );
        assert_eq!(
            filter_indices(&rows, &cols, "c", &[team("core")]),
            vec![0, 2]
        );
        assert_eq!(
            filter_indices(&rows, &cols, "ada", &[team("core")]),
            vec![0]
        );
        // Exact, not substring.
        assert!(filter_indices(&rows, &cols, "", &[team("cor")]).is_empty());
        // Empty value and unknown column do nothing.
        assert_eq!(filter_indices(&rows, &cols, "", &[team("")]).len(), 5);
        let unknown = ColumnFilter {
            column: "zzz".into(),
            value: "x".into(),
        };
        assert_eq!(filter_indices(&rows, &cols, "", &[unknown]).len(), 5);
    }

    #[test]
    fn filter_then_sort_gives_the_displayed_order() {
        let rows = rows();
        let cols = columns();
        let mut idx = filter_indices(
            &rows,
            &cols,
            "",
            &[ColumnFilter {
                column: "team".into(),
                value: "web".into(),
            }],
        );
        sort_indices(&rows, &mut idx, &cols, &[key("age", SortDirection::Desc)]);
        assert_eq!(names(&rows, &idx), ["Di", "Bo"]);
    }

    #[test]
    fn page_window_slices_and_clamps() {
        let w = page_window(25, 0, 10);
        assert_eq!((w.page_index, w.page_count, w.range.clone()), (0, 3, 0..10));
        assert!(!w.has_previous() && w.has_next());
        let w = page_window(25, 2, 10);
        assert_eq!(w.range, 20..25);
        assert!(w.has_previous() && !w.has_next());
        // Past the end (the filter shrank the list): the last page.
        let w = page_window(25, 9, 10);
        assert_eq!((w.page_index, w.range), (2, 20..25));
    }

    #[test]
    fn page_window_edge_cases() {
        let w = page_window(0, 0, 10);
        assert_eq!((w.page_count, w.range), (1, 0..0));
        assert_eq!(page_window(20, 1, 10).range, 10..20);
        assert_eq!(page_window(20, 5, 10).page_count, 2);
        assert_eq!(page_window(3, 0, 0).page_count, 3, "size 0 acts as 1");
    }

    #[test]
    fn selection_state_for_a_header_checkbox() {
        let keys: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        let mut selected = HashSet::new();
        assert_eq!(selection_state(&selected, &keys), SelectionState::None);
        selected.insert("b".to_string());
        assert_eq!(selection_state(&selected, &keys), SelectionState::Some);
        set_selected(&mut selected, &keys, true);
        assert_eq!(selection_state(&selected, &keys), SelectionState::All);
        assert_eq!(selection_state(&selected, &[]), SelectionState::None);
    }

    #[test]
    fn selecting_a_page_keeps_other_pages_selected() {
        let page1: Vec<String> = ["a", "b"].iter().map(|s| s.to_string()).collect();
        let page2: Vec<String> = ["c", "d"].iter().map(|s| s.to_string()).collect();
        let mut selected = HashSet::new();
        set_selected(&mut selected, &page1, true);
        set_selected(&mut selected, &page2, true);
        set_selected(&mut selected, &page1, false);
        assert_eq!(selected.len(), 2);
        assert!(selected.contains("c") && selected.contains("d"));
    }

    #[test]
    fn distinct_values_are_unique_and_sorted() {
        let rows = rows();
        let cols = columns();
        assert_eq!(distinct_values(&rows, &cols[1]), vec!["Core", "Ops", "Web"]);
        assert!(distinct_values(&rows, &cols[2]).is_empty());
    }

    #[test]
    fn toggle_key_flips_membership() {
        let mut keys = HashSet::new();
        assert!(toggle_key(&mut keys, "a"));
        assert!(toggle_key(&mut keys, "b"));
        assert!(!toggle_key(&mut keys, "a"));
        assert_eq!(keys.len(), 1);
        assert!(keys.contains("b"));
    }

    #[test]
    fn status_text_covers_each_case() {
        assert_eq!(status_text(false, 0, 0, 0, 0), "No rows.");
        assert_eq!(
            status_text(false, 0, 40, 10, 10),
            "Showing 11-20 of 40 rows."
        );
        assert_eq!(status_text(false, 0, 3, 0, 3), "Showing 1-3 of 3 rows.");
        assert_eq!(status_text(true, 2, 40, 0, 10), "2 of 40 row(s) selected.");
    }
}
