use crate::primitives::data_table::ColumnModel;
use crate::{Breakpoint, TextAlign};
use leptos::prelude::*;
use std::cmp::Ordering;
use std::sync::Arc;

type CellFn<T> = Arc<dyn Fn(&T) -> AnyView + Send + Sync>;
type FooterFn<T> = Arc<dyn Fn(&[T]) -> AnyView + Send + Sync>;

/// One column of a [`crate::DataTable`]: what its header says, how a row's cell draws, and (optionally)
/// how the column sorts, what the search and column filters read from it, and a footer total.
///
/// Build it with [`DataTableColumn::new`] (any cell view) or [`DataTableColumn::text`] (a plain string
/// cell that is also sortable and searchable), then chain options:
///
/// ```ignore
/// DataTableColumn::text("name", "Name", |r: &Server| r.name.clone()),
/// DataTableColumn::new("status", "Status", |r: &Server| view! { <Badge>{r.status.clone()}</Badge> })
///     .filter_text(|r| r.status.clone())      // searchable, and usable by a column filter
///     .sort_by_key(|r| r.status.clone()),     // sortable
/// DataTableColumn::new("cpu", "CPU", |r: &Server| format!("{}%", r.cpu))
///     .sort_by_key(|r| r.cpu)
///     .align(TextAlign::Right)
///     .footer(|rows| format!("{}% avg", average_cpu(rows))),
/// ```
///
/// `id` must be unique in the table; sort state, filters and hidden columns refer to it.
pub struct DataTableColumn<T> {
    pub(crate) header: String,
    pub(crate) cell: CellFn<T>,
    pub(crate) footer: Option<FooterFn<T>>,
    pub(crate) model: ColumnModel<T>,
    pub(crate) align: TextAlign,
    pub(crate) width: Option<String>,
    pub(crate) hideable: bool,
    pub(crate) hidden_by_default: bool,
    pub(crate) header_visible: bool,
    pub(crate) class: String,
    pub(crate) wrap: bool,
    pub(crate) min_width: Option<String>,
    pub(crate) max_width: Option<String>,
    pub(crate) hide_below: Option<Breakpoint>,
    pub(crate) sticky: Option<Sticky>,
}

/// Which edge a pinned column sticks to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sticky {
    Start,
    End,
}

impl<T: 'static> DataTableColumn<T> {
    /// A column drawing any view per row. It is not sortable or searchable until you add
    /// [`sort_by`](Self::sort_by) / [`sort_by_key`](Self::sort_by_key) / [`filter_text`](Self::filter_text).
    pub fn new<V>(
        id: impl Into<String>,
        header: impl Into<String>,
        cell: impl Fn(&T) -> V + Send + Sync + 'static,
    ) -> Self
    where
        V: IntoView + 'static,
    {
        DataTableColumn {
            header: header.into(),
            cell: Arc::new(move |row| cell(row).into_any()),
            footer: None,
            model: ColumnModel::new(id),
            align: TextAlign::Left,
            width: None,
            hideable: true,
            hidden_by_default: false,
            header_visible: true,
            class: String::new(),
            wrap: false,
            min_width: None,
            max_width: None,
            hide_below: None,
            sticky: None,
        }
    }

    /// A column showing `value(row)` as text. It is sortable (case-insensitively, then as written) and
    /// searchable by that same text.
    pub fn text(
        id: impl Into<String>,
        header: impl Into<String>,
        value: impl Fn(&T) -> String + Send + Sync + 'static,
    ) -> Self {
        let value = Arc::new(value);
        let shown = value.clone();
        let mut column = Self::new(id, header, move |row: &T| shown(row));
        let text = value.clone();
        column.model.text = Some(Arc::new(move |row: &T| text(row)));
        column.model.compare = Some(Arc::new(move |a: &T, b: &T| {
            let (a, b) = (value(a), value(b));
            a.to_lowercase()
                .cmp(&b.to_lowercase())
                .then_with(|| a.cmp(&b))
        }));
        column
    }

    /// The text the search box and column filters read from this column.
    pub fn filter_text(mut self, text: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        self.model.text = Some(Arc::new(text));
        self
    }

    /// Make the column sortable with a comparator (ascending order; descending is its reverse).
    pub fn sort_by(mut self, compare: impl Fn(&T, &T) -> Ordering + Send + Sync + 'static) -> Self {
        self.model.compare = Some(Arc::new(compare));
        self
    }

    /// Make the column sortable by a key, for example a number or a timestamp.
    pub fn sort_by_key<K: Ord>(self, key: impl Fn(&T) -> K + Send + Sync + 'static) -> Self {
        self.sort_by(move |a, b| key(a).cmp(&key(b)))
    }

    /// Align the header and cells. Numbers read best on the right.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// A CSS width for the column (`"120px"`, `"20%"`), set on its header cell.
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Whether the column menu may hide this column (default true).
    pub fn hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }

    /// Start with the column hidden (when the table makes its own state; with your own
    /// [`crate::DataTableState`], use `set_column_visible`).
    pub fn hidden_by_default(mut self) -> Self {
        self.hidden_by_default = true;
        self
    }

    /// Keep the header text for screen readers but do not draw it, for an actions column.
    pub fn header_hidden(mut self) -> Self {
        self.header_visible = false;
        self
    }

    /// Extra classes for this column's header and cells.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }

    /// Let this column's header and cells wrap (they do not by default). Long words and emails break
    /// too, so a key column no longer forces a sideways scroll. Combine with
    /// [`min_width`](Self::min_width) / [`max_width`](Self::max_width) to bound it.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// A CSS minimum width for the column (`"8rem"`), set on its header and cells.
    pub fn min_width(mut self, width: impl Into<String>) -> Self {
        self.min_width = Some(width.into());
        self
    }

    /// A CSS maximum width for the column (`"20rem"`), set on its header and cells. Pair with
    /// [`wrap`](Self::wrap) or the content is cut off.
    pub fn max_width(mut self, width: impl Into<String>) -> Self {
        self.max_width = Some(width.into());
        self
    }

    /// Hide the column (header, cells and footer) on viewports narrower than `breakpoint`. This is
    /// CSS only: the column is still in the column menu and in the stacked layout it is shown again.
    pub fn hide_below(mut self, breakpoint: Breakpoint) -> Self {
        self.hide_below = Some(breakpoint);
        self
    }

    /// Pin the column to the left edge while the table scrolls sideways (a key column).
    pub fn sticky(mut self) -> Self {
        self.sticky = Some(Sticky::Start);
        self
    }

    /// Pin the column to the right edge while the table scrolls sideways, so a row-actions column
    /// stays reachable on a phone.
    pub fn sticky_end(mut self) -> Self {
        self.sticky = Some(Sticky::End);
        self
    }

    /// A footer cell computed from every row that passes the search and filters (all pages), such as a
    /// total. With server-side data it sees only the rows you passed in.
    pub fn footer<V>(mut self, footer: impl Fn(&[T]) -> V + Send + Sync + 'static) -> Self
    where
        V: IntoView + 'static,
    {
        self.footer = Some(Arc::new(move |rows| footer(rows).into_any()));
        self
    }

    /// The column's id.
    pub fn id(&self) -> &str {
        &self.model.id
    }
}
