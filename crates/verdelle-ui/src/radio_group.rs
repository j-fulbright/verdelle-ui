use crate::primitives::radio_group as primitive;
use leptos::prelude::*;

/// A set of radio buttons where one is chosen, styled as shadcn/ui's RadioGroup and
/// built on the headless RadioGroup (`role="radiogroup"`; arrow keys move
/// focus and select; Tab enters at the checked item).
///
/// Compose with [`crate::RadioGroupItem`]. Uncontrolled by default (`default_value`);
/// pass `value` and `on_value_change` to control it. By default the items stack (as in
/// shadcn) and all four arrow keys move. `orientation` "horizontal" puts them in a row
/// and limits movement to Left/Right; "vertical" keeps the stack and limits it to Up/Down.
///
/// The items do not have to be children of the group element: any [`crate::RadioGroupItem`]
/// rendered under the group takes part, and the arrow keys and the tab stop follow document
/// order. So a radio per table row works, with the group around the table (a plain `<div>`
/// breaks nothing) or, with `as_child`, as the element you give it: `<RadioGroup as_child=true>`
/// around one element (a `<div>`, a `<table>`, a `<tbody>`) puts the group's role, state and
/// attributes on that element instead of adding a `div`. With `as_child` the group's own
/// stacking style is not applied; style the element yourself.
#[component]
pub fn RadioGroup(
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Marks it as required (`required` / `aria-required`).
    #[prop(into, optional)]
    required: MaybeProp<bool>,
    /// Form field name; the value is submitted under it.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// "horizontal" or "vertical"; unset allows all four arrow keys.
    #[prop(into, optional)]
    orientation: MaybeProp<String>,
    /// Use the single child element as the group instead of rendering a `div` (the group's role
    /// and attributes go onto it; its stacking style is not applied).
    #[prop(optional)]
    as_child: bool,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::RadioGroup
            value=value
            default_value=default_value
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: String| {}))
            disabled=disabled
            required=required
            name=name
            orientation=orientation
            as_child=as_child
            attr:aria-label=aria_label
            attr:data-orientation=move || orientation.get()
            attr:class=(!as_child).then(|| format!("vdl-radio-group {class}"))
        >
            {children.with_value(|children| children())}
        </primitive::RadioGroup>
    }
}
