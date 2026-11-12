use crate::{Breakpoint, TextAlign};
use leptos::prelude::*;

/// A data cell (`<td>`).
///
/// Cells do not wrap by default; set `wrap` for long text. `hide_below` hides the cell (and its
/// column's [`crate::TableHead`], which needs the same value) under a breakpoint. `label` is the
/// column name shown beside the value when the table is stacked (see [`crate::Table`]'s `stack_below`).
#[component]
pub fn TableCell(
    #[prop(optional)] align: TextAlign,
    #[prop(default = 1)] col_span: usize,
    /// Let the content wrap onto several lines (long words and emails break too).
    #[prop(optional, into)]
    wrap: Signal<bool>,
    /// Hide the cell on viewports narrower than this.
    #[prop(optional)]
    hide_below: Option<Breakpoint>,
    /// The column's name, shown before the value when the table is stacked.
    #[prop(optional, into)]
    label: Option<String>,
    /// Extra classes, for example `p-0` for a cell that holds a nested panel.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <td
            class=format!("vdl-table__cell {class}")
            colspan=(col_span > 1).then_some(col_span)
            data-align=align.attr()
            data-wrap=move || wrap.get().then_some("")
            data-hide-below=hide_below.map(Breakpoint::attr)
            data-label=label.filter(|l| !l.is_empty())
        >
            {children()}
        </td>
    }
}
