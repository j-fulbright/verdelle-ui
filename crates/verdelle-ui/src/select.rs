use crate::primitives::select as primitive;
use leptos::prelude::*;

/// A select, styled as shadcn/ui's Select and built on the headless Select, which
/// provide the roles, keyboard handling (arrows, Home/End, PageUp/PageDown, typeahead,
/// Escape), positioning and the hidden form input.
///
/// Compose with [`crate::SelectTrigger`], [`crate::SelectValue`] and [`crate::SelectContent`]
/// holding [`crate::SelectItem`]s (optionally in a [`crate::SelectGroup`] with a
/// [`crate::SelectLabel`], split by a [`crate::SelectSeparator`]). Uncontrolled by default
/// (`default_value`); pass `value` and `on_value_change` to control it. Give it a `name` to
/// submit the value with a form.
///
/// Items only exist in the page while the list is open. To show the selected item's text on
/// the closed trigger before the first open, pass `items` as (value, label) pairs; without
/// it the trigger shows the raw value until then (a debug build logs a console hint when
/// that happens). Passing `items` is the way to avoid that flash.
///
/// An item may have the empty value (`<SelectItem value="">All</SelectItem>`), which is how
/// an "All" or "Any" choice is made without a sentinel string. When the value is `""` and an
/// item (or an `items` pair) with the empty value gives it a label, the trigger shows that label;
/// when no item gives one, the trigger shows the `placeholder`, as it does while nothing is chosen
/// (no value at all). So an empty-valued item and the placeholder never both appear: put the label
/// in the `""` item and the trigger reads "All".
///
/// `clearable` lets the user reset the value: Backspace or Delete on the focused trigger, and a
/// small X (the trigger draws a [`crate::SelectClear`]) that appears while there is a value. Clearing
/// leaves nothing chosen (the placeholder shows) and calls `on_value_change` with `""`.
#[component]
pub fn Select(
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Called with the new state whenever it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Form field name; the value is submitted under it.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Marks it as required (`required` / `aria-required`).
    #[prop(into, optional)]
    required: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    /// (value, label) pairs, so the closed trigger can show the label of the chosen value before
    /// the list has ever been opened.
    #[prop(into, optional)]
    items: MaybeProp<Vec<(String, String)>>,
    /// Let the value be cleared: Backspace or Delete on the trigger, and a click on the X the
    /// trigger shows while there is a value. Clearing calls `on_value_change` with `""`.
    #[prop(into, optional)]
    clearable: MaybeProp<bool>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Select
            value=value
            default_value=default_value
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: String| {}))
            open=open
            on_open_change=on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
            disabled=disabled
            name=name
            required=required
            form=form
            items=items
            clearable=clearable
        >
            {children.with_value(|children| children())}
        </primitive::Select>
    }
}
