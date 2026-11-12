use crate::primitives::tooltip as primitive;
use leptos::prelude::*;

/// A tooltip, styled as shadcn/ui's Tooltip: a short label that appears when the pointer
/// rests on its trigger or the trigger gets keyboard focus, and goes away on pointer leave,
/// blur, a press or Escape. Built on the headless Tooltip.
///
/// Compose with [`crate::TooltipTrigger`] and [`crate::TooltipContent`], inside a
/// [`crate::TooltipProvider`] (which sets the delay). `delay_duration` here overrides the
/// provider's for this tooltip; without a provider the delay is 700ms. Uncontrolled by
/// default; pass `open` (with `on_open_change`) to control it.
///
/// For a one-line hint on any element, see [`crate::TooltipText`].
///
/// Tooltips are for supplementary text only. They are not shown on touch screens, so do
/// not put anything essential in one.
#[component]
pub fn Tooltip(
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever the tooltip opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Milliseconds the pointer must rest before it opens, overriding the provider.
    #[prop(into, optional)]
    delay_duration: MaybeProp<i32>,
    /// Keep the tooltip open while the pointer moves onto its content, so the text can be
    /// selected or a link in it clicked. On by default (as Radix's `disableHoverableContent`
    /// being false); turn it off for a tooltip that must go as soon as the pointer leaves the
    /// trigger. The close after leaving the trigger is delayed by 120ms so the pointer can cross
    /// the gap; moving onto the content cancels it.
    #[prop(into, optional)]
    hoverable: MaybeProp<bool>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Tooltip
            open=open
            default_open=default_open
            delay_duration=delay_duration
            hoverable=hoverable
            on_open_change=Callback::new(move |o: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(o);
                }
            })
        >
            {children.with_value(|children| children())}
        </primitive::Tooltip>
    }
}
