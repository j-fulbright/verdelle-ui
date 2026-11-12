use crate::primitives::hover_card as primitive;
use leptos::prelude::*;

/// A hover card, styled as shadcn/ui's Hover Card (Base UI's Preview Card): a small panel
/// of preview content that appears when the pointer rests on a link, or the link gets
/// keyboard focus, and goes away when the pointer leaves both the link and the panel.
/// Built on the headless Hover Card.
///
/// Compose with [`crate::HoverCardTrigger`] and [`crate::HoverCardContent`].
/// `open_delay` (default 600ms) is how long the pointer must rest before it opens and
/// `close_delay` (default 300ms) how long it may stay away before it closes; the pointer
/// can cross the gap onto the panel. Uncontrolled by default; pass `open` (with
/// `on_open_change`) to control it.
///
/// Hover cards are for sighted pointer users only: they do not open on touch, and
/// nothing essential may live in one.
#[component]
pub fn HoverCard(
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever the card opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Milliseconds the pointer must rest on the trigger before it opens. Default 600.
    #[prop(into, optional, default = 600.into())]
    open_delay: MaybeProp<i32>,
    /// Milliseconds before it closes after the pointer leaves. Default 300.
    #[prop(into, optional, default = 300.into())]
    close_delay: MaybeProp<i32>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::HoverCard
            open=open
            default_open=default_open
            open_delay=open_delay
            close_delay=close_delay
            on_open_change=Callback::new(move |o: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(o);
                }
            })
        >
            {children.with_value(|children| children())}
        </primitive::HoverCard>
    }
}
