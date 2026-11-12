use leptos::prelude::*;

/// How a table row is emphasised. Set through [`crate::TableRow`]'s `state` or a data table's `row_state`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RowState {
    /// No emphasis.
    #[default]
    Normal,
    /// Faded, for rows that are expired, archived or inactive.
    Dimmed,
    /// A tinted background, for the row that needs attention.
    Highlighted,
    /// A destructive-coloured tint, for rows that are past due or failed.
    Destructive,
}

impl RowState {
    /// The `data-row-state` value, `None` for [`RowState::Normal`].
    pub fn attr(self) -> Option<&'static str> {
        match self {
            RowState::Normal => None,
            RowState::Dimmed => Some("dimmed"),
            RowState::Highlighted => Some("highlighted"),
            RowState::Destructive => Some("destructive"),
        }
    }
}

/// A table row (`<tr>`).
///
/// `selected` styles the row with the theme's active colour (`data-selected`); `state` emphasises it
/// (`data-row-state`: dimmed, highlighted, destructive). If rows are interactive, the caller owns the
/// semantics and keyboard handling.
#[component]
pub fn TableRow(
    #[prop(optional, into)] selected: Signal<bool>,
    /// Emphasis for the row.
    #[prop(optional, into)]
    state: Signal<RowState>,
    /// Extra classes, for example a thicker divider.
    #[prop(optional, into)]
    class: String,
    /// The cells.
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            class=format!("vdl-table__row {class}")
            data-selected=move || selected.get().then_some("")
            data-row-state=move || state.get().attr()
        >
            {children()}
        </tr>
    }
}
