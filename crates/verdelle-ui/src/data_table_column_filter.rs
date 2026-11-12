use crate::{DataTableState, NativeSelect, NativeSelectOption, Size};
use leptos::prelude::*;

/// A select that filters a [`crate::DataTable`] to the rows whose text for `column` equals the chosen
/// option, with an "All" choice that clears the filter. Put it in the table's `toolbar`, passing the same
/// [`DataTableState`] you gave the table.
///
/// The match is on the text the column's `filter_text` (or `text` column) returns, ignoring case. Fill
/// `options` with the values to offer; `crate::primitives::data_table::distinct_values` lists a column's
/// values from the rows. The select follows the state, so `state.clear_filters()` resets it.
#[component]
pub fn DataTableColumnFilter(
    state: DataTableState,
    /// The id of the column to filter.
    #[prop(into)]
    column: String,
    #[prop(into)] options: Signal<Vec<String>>,
    /// The text of the choice that removes the filter.
    #[prop(default = "All".into(), into)]
    all_label: String,
    /// Accessible name for the select; defaults to the column id.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let label = aria_label.unwrap_or_else(|| format!("Filter by {column}"));
    let in_state = move |column: &str, track: bool| {
        let read = |filters: &Vec<crate::ColumnFilter>| {
            filters
                .iter()
                .find(|f| f.column == column)
                .map(|f| f.value.clone())
                .unwrap_or_default()
        };
        if track {
            state.column_filters.with(read)
        } else {
            state.column_filters.with_untracked(read)
        }
    };
    let value = RwSignal::new(in_state(&column, false));
    // The select's choice -> the state. It must only react to the choice: if it also tracked the state it
    // would re-run on a clear before the select has caught up, and put the old choice back.
    {
        let column = column.clone();
        Effect::new(move |_| {
            let chosen = value.get();
            if chosen != in_state(&column, false) {
                state.set_column_filter(&column, &chosen);
            }
        });
    }
    // The state -> the select, so clearing the filters elsewhere resets it.
    {
        let column = column.clone();
        Effect::new(move |_| {
            let current = in_state(&column, true);
            if value.get_untracked() != current {
                value.set(current);
            }
        });
    }
    let initial = value.get_untracked();

    view! {
        <NativeSelect value=value size=Size::Small aria_label=label class="vdl-data-table__filter">
            <NativeSelectOption value="" selected=initial.is_empty()>
                {all_label}
            </NativeSelectOption>
            {move || {
                options
                    .get()
                    .into_iter()
                    .map(|option| {
                        let selected = option == initial;
                        view! {
                            <NativeSelectOption value=option.clone() selected=selected>
                                {option}
                            </NativeSelectOption>
                        }
                    })
                    .collect_view()
            }}
        </NativeSelect>
    }
}
