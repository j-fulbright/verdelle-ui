use crate::primitives::tooltip as primitive;
use crate::{Align, Side, TooltipContent};
use leptos::prelude::*;

/// A tooltip with a text label on any element, in one line: the lightweight way to add a hint.
///
/// ```ignore
/// <TooltipText text="Delete this row">
///     <Button variant=ButtonVariant::Ghost icon_only=true aria_label="Delete">...</Button>
/// </TooltipText>
/// ```
///
/// The single child element (a [`crate::Button`], an icon, a `<span>`, a link) becomes the
/// trigger itself: no extra wrapper element and no `<button>` of its own, so it works on
/// anything that can receive hover and focus. It expands to a [`crate::Tooltip`] with a
/// trigger (using the child, `as_child`) and a [`crate::TooltipContent`] holding `text`.
/// Pass exactly one element as the child; text or several siblings would not receive the
/// trigger's attributes.
///
/// Behaviour is [`crate::Tooltip`]'s: it opens on hover or keyboard focus after the provider's
/// delay (or `delay_duration`), closes on pointer leave, blur, a press or Escape, and stays open
/// while the pointer is over it (`hoverable`, on by default). `text` may be a signal. The text also
/// describes the trigger for screen readers through `aria-describedby`; it is not its name, so an
/// icon-only trigger still needs its own `aria_label`. Not shown on touch screens: nothing
/// essential belongs in it. For rich content or a custom layout compose `Tooltip`,
/// `TooltipTrigger` and `TooltipContent` yourself.
#[component]
pub fn TooltipText(
    /// The label.
    #[prop(into)]
    text: Signal<String>,
    /// Preferred side of the trigger; it flips to the opposite side when there is no room. Top by default.
    #[prop(into, optional)]
    side: MaybeProp<Side>,
    /// How the label lines up with the trigger along that side: `Start`, `Center` or `End`.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// Milliseconds the pointer must rest before it opens, overriding the provider.
    #[prop(into, optional)]
    delay_duration: MaybeProp<i32>,
    /// Keep it open while the pointer is over the label. On by default.
    #[prop(into, optional)]
    hoverable: MaybeProp<bool>,
    /// Show the arrow. On by default, as in shadcn.
    #[prop(default = true)]
    arrow: bool,
    /// Extra classes for the label, for layout (for example Tailwind utilities).
    #[prop(optional, into)]
    class: String,
    /// The one element the tooltip describes.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let class = StoredValue::new(class);

    view! {
        <primitive::Tooltip delay_duration=delay_duration hoverable=hoverable>
            <primitive::TooltipTrigger as_child=true>
                {children.with_value(|children| children())}
            </primitive::TooltipTrigger>
            <TooltipContent side=side align=align arrow=arrow class=class.get_value()>
                {move || text.get()}
            </TooltipContent>
        </primitive::Tooltip>
    }
}
