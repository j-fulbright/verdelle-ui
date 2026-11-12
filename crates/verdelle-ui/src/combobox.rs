use crate::primitives::combobox as primitive;
use crate::primitives::core::listbox::{FilterQuery, contains_filter};
use leptos::prelude::*;

/// A text input with a filtered list of choices, styled as shadcn/ui's Combobox and built on
/// the headless Combobox (roles, `aria-activedescendant`, keyboard, filtering,
/// positioning, the hidden form input).
///
/// Compose with a [`crate::ComboboxInput`] (or [`crate::ComboboxChips`] for several values)
/// and a [`crate::ComboboxContent`] holding a [`crate::ComboboxList`] of
/// [`crate::ComboboxItem`]s (optionally in a [`crate::ComboboxGroup`] with a
/// [`crate::ComboboxLabel`]) and a [`crate::ComboboxEmpty`].
///
/// One value by default (`value`, `default_value`, `on_value_change`, which gets `None` when it
/// is cleared); with `multiple`, a list (`values`, `default_values`, `on_values_change`) and
/// the input only searches. Each is controlled when given and uncontrolled otherwise. Likewise
/// `input_value` (the text) and `open`. Give it a `name` to submit the value with a form.
///
/// Items are matched against the typed text with `filter` (default: every typed word occurs in
/// the item's label or keywords; see [`crate::contains_filter`], [`crate::starts_with_filter`],
/// [`crate::fuzzy_filter`]). When results come from a server, set `should_filter=false` and
/// render what you got. Items only exist in the page while the popup is open: pass `items` as
/// (value, label) pairs so the closed input shows the label of the selected value.
///
/// Keys, from the input: ArrowDown and ArrowUp open the popup and move the highlight (with
/// Ctrl or Cmd they jump to the first or last), PageUp and PageDown move ten, Enter picks the
/// highlighted item, Escape closes, Backspace on an empty input removes the last pick of a
/// `multiple` combobox. Home and End move the text caret.
#[component]
pub fn Combobox(
    #[prop(optional)] multiple: bool,
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<String>>>,
    #[prop(into, optional)] values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] input_value: MaybeProp<String>,
    #[prop(into, optional)] default_input_value: MaybeProp<String>,
    #[prop(into, optional)] on_input_value_change: Option<Callback<String>>,
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    /// Marks it as required (`required` / `aria-required`).
    #[prop(into, optional)]
    required: MaybeProp<bool>,
    /// Form field name; the value is submitted under it.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] items: MaybeProp<Vec<(String, String)>>,
    #[prop(into, optional)] filter: Option<Callback<FilterQuery, bool>>,
    #[prop(into, optional)] should_filter: MaybeProp<bool>,
    #[prop(into, optional)] auto_highlight: MaybeProp<bool>,
    #[prop(into, optional)] loop_around: MaybeProp<bool>,
    #[prop(into, optional)] open_on_input_click: MaybeProp<bool>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Combobox
            multiple=multiple
            value=value
            default_value=default_value
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Option<String>| {}))
            values=values
            default_values=default_values
            on_values_change=on_values_change.unwrap_or_else(|| Callback::new(|_: Vec<String>| {}))
            input_value=input_value
            default_input_value=default_input_value
            on_input_value_change=on_input_value_change.unwrap_or_else(|| Callback::new(|_: String| {}))
            open=open
            default_open=default_open
            on_open_change=on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
            disabled=disabled
            read_only=read_only
            required=required
            name=name
            form=form
            items=items
            filter=filter.unwrap_or_else(|| Callback::new(|q: FilterQuery| contains_filter(&q)))
            should_filter=should_filter
            auto_highlight=auto_highlight
            loop_around=loop_around
            open_on_input_click=open_on_input_click
        >
            {children.with_value(|children| children())}
        </primitive::Combobox>
    }
}
