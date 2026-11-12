use crate::primitives::popover as primitive;
use leptos::prelude::*;

/// A popover, styled as shadcn/ui's Popover: a non-modal panel that opens next to its
/// trigger and closes on Escape or a press outside it. Built on the headless
/// Popover, which positions it with floating-ui and handles dismissal and focus.
///
/// Compose with [`crate::PopoverTrigger`] and [`crate::PopoverContent`] (plus
/// [`crate::PopoverHeader`], [`crate::PopoverTitle`], [`crate::PopoverDescription`] inside
/// the content). Uncontrolled by default; pass `open` (and `on_open_change`) to control it,
/// for example to close it from a button inside.
#[component]
pub fn Popover(
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever the popover opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Popover open=open default_open=default_open on_open_change=Callback::new(move |o: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(o);
                }
            })
        >
            {children.with_value(|children| children())}
        </primitive::Popover>
    }
}
