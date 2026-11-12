use crate::Breakpoint;
use leptos::prelude::*;

/// A data table built on a real `<table>`, styled as shadcn/ui's Table. Compose it
/// with [`crate::TableHeader`], [`crate::TableBody`], [`crate::TableFooter`],
/// [`crate::TableRow`], [`crate::TableHead`], [`crate::TableCell`] and
/// [`crate::TableCaption`].
///
/// Table elements give assistive technology row and column semantics.
///
/// Extras over shadcn's table: `bordered` (a rounded outer border, for a table that
/// is not inside a card) and `striped` (tinted even rows). For sorting, search, paging,
/// selection and the rest, use [`crate::DataTable`], which is built on this.
///
/// **Inside a card**, leave `bordered` off (the default): the table has no outer border and the card
/// supplies the frame. **On narrow screens** there are three patterns: scroll sideways (the default;
/// the wrapper scrolls), hide minor columns (`hide_below` on [`crate::TableHead`] and
/// [`crate::TableCell`]), or turn each row into a labelled card (`stack_below`, with a `label` on
/// every cell).
#[component]
pub fn Table(
    #[prop(optional, into)] bordered: Signal<bool>,
    /// Keep the header row visible while the table scrolls. Give the table a height through
    /// `class` (for example `max-h-96`), which also makes it scroll vertically.
    #[prop(optional)]
    sticky_header: bool,
    /// Extra classes for the scroll container, for layout and sizing.
    #[prop(optional, into)]
    class: String,
    #[prop(optional, into)] striped: Signal<bool>,
    /// Tint the row under the pointer. On by default, as shadcn.
    #[prop(default = true.into(), into)]
    hoverable: Signal<bool>,
    /// Accessible name for the table. Prefer a [`crate::TableCaption`] when a visible title fits.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Let every cell wrap its content (cells do not wrap by default).
    #[prop(optional, into)]
    wrap: Signal<bool>,
    /// Below this width, draw each row as a card: the header row is hidden and each cell shows its
    /// `label` (set on [`crate::TableCell`]) before the value. The table loses its table layout there.
    #[prop(optional, into)]
    stack_below: MaybeProp<Breakpoint>,
    /// The content.
    children: Children,
) -> impl IntoView {
    let flag = |s: Signal<bool>| move || s.get().then_some("");

    view! {
        <div class=format!("vdl-table-wrap {class}") data-bordered=flag(bordered)
            data-stack-below=move || stack_below.get().map(Breakpoint::attr)
        >
            <table
                class="vdl-table"
                data-sticky-header=sticky_header.then_some("")
                data-striped=flag(striped)
                data-hoverable=flag(hoverable)
                data-wrap=flag(wrap)
                aria-label=aria_label.filter(|label| !label.is_empty())
            >
                {children()}
            </table>
        </div>
    }
}
