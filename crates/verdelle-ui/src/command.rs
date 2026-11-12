use crate::primitives::command as primitive;
use crate::primitives::core::listbox::{FilterQuery, contains_filter};
use leptos::prelude::*;

/// A search box over a list of actions, styled as shadcn/ui's Command and built on the
/// headless Command (the cmdk idea: focus stays in the input, the list is driven with
/// `aria-activedescendant`).
///
/// Compose with a [`crate::CommandInput`] and a [`crate::CommandList`] holding
/// [`crate::CommandGroup`]s of [`crate::CommandItem`]s, a [`crate::CommandEmpty`] and
/// [`crate::CommandSeparator`]s. Put it in a [`crate::CommandDialog`] for a palette opened with a
/// key, or in a [`crate::PopoverContent`] for a menu.
///
/// The root owns the search text (`search` controls it) and every item filters itself against
/// it with `filter` (default: each typed word occurs in the item's label or keywords). Set
/// `should_filter=false` to filter the data yourself. `on_value_change` is called with the
/// highlighted item's value as the highlight moves.
///
/// Keys, from the input: ArrowDown and ArrowUp move the highlight (Ctrl or Cmd with an arrow
/// jumps to the first or last), Home and End jump to the first and last, PageDown and PageUp
/// move ten, Enter runs the highlighted item. `loop_around` wraps the ends.
#[component]
pub fn Command(
    #[prop(into, optional)] search: MaybeProp<String>,
    #[prop(into, optional)] default_search: MaybeProp<String>,
    #[prop(into, optional)] on_search_change: Option<Callback<String>>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<String>>>,
    #[prop(into, optional)] filter: Option<Callback<FilterQuery, bool>>,
    #[prop(into, optional)] should_filter: MaybeProp<bool>,
    #[prop(into, optional)] loop_around: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::Command
            search=search
            default_search=default_search
            on_search_change=on_search_change.unwrap_or_else(|| Callback::new(|_: String| {}))
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Option<String>| {}))
            filter=filter.unwrap_or_else(|| Callback::new(|q: FilterQuery| contains_filter(&q)))
            should_filter=should_filter
            loop_around=loop_around
            attr:class=format!("vdl-command {class}")
        >
            {children.with_value(|children| children())}
        </primitive::Command>
    }
}
