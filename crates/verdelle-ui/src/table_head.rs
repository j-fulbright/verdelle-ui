use crate::{Breakpoint, TextAlign};
use leptos::prelude::*;

/// A column header cell (`<th scope="col">`).
///
/// Named as shadcn does; it is the header *cell*, while
/// [`crate::TableHeader`] is the header *group*.
#[component]
pub fn TableHead(
    #[prop(optional)] align: TextAlign,
    #[prop(default = 1)] col_span: usize,
    /// Let the text wrap onto several lines.
    #[prop(optional, into)]
    wrap: Signal<bool>,
    /// Hide the header on viewports narrower than this (give the column's cells the same value).
    #[prop(optional)]
    hide_below: Option<Breakpoint>,
    /// Extra classes.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <th
            class=format!("vdl-table__head {class}")
            scope="col"
            colspan=(col_span > 1).then_some(col_span)
            data-align=align.attr()
            data-wrap=move || wrap.get().then_some("")
            data-hide-below=hide_below.map(Breakpoint::attr)
        >
            {children()}
        </th>
    }
}
