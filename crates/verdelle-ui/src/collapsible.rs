use crate::primitives::collapsible as primitive;
use leptos::prelude::*;

/// A region that shows or hides its content, styled as shadcn/ui's Collapsible (which has
/// no styling of its own) and built on the headless Collapsible. One
/// [`crate::CollapsibleTrigger`] toggles one [`crate::CollapsibleContent`]; the trigger
/// carries `aria-expanded` and `aria-controls`.
///
/// Uncontrolled by default (`default_open` sets the start); pass `open` and
/// `on_open_change` to control it. For several related sections use [`crate::Accordion`].
///
/// A trigger and its content normally sit inside one `Collapsible`. When they cannot (a
/// trigger in one table row and its content in the next), make a [`crate::CollapsibleState`] with
/// [`crate::use_collapsible`] and pass it as `state` to the trigger and the content: they then
/// need no wrapper at all, and anything else can read the same state (`state.open`,
/// `state.data_state()`). `as_child` lets the wrapper (and the trigger and content) be the one
/// element you give them instead of a `div` / `button`, for a `<tbody>` or a `<tr>`; the
/// component's own classes are then not applied.
#[component]
pub fn Collapsible(
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
    /// Share one open state with parts outside this wrapper (see [`crate::use_collapsible`]).
    /// Replaces `open`, `default_open` and `on_open_change`.
    #[prop(optional, into)]
    state: MaybeProp<crate::CollapsibleState>,
    /// Use the single child element instead of a `div`.
    #[prop(optional)]
    as_child: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Collapsible
            open=open
            default_open=default_open
            on_open_change=on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
            disabled=disabled
            state=state
            as_child=as_child
            attr:class=(!as_child).then(|| format!("vdl-collapsible {class}"))
        >
            {children.with_value(|children| children())}
        </primitive::Collapsible>
    }
}
