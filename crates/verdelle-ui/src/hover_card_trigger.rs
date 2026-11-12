use crate::primitives::hover_card as primitive;
use leptos::prelude::*;

/// The element a [`crate::HoverCard`] previews. It is an `<a>`, as in Base UI's Preview
/// Card: give it an `href` for a real link. Without one it is still focusable (so the
/// keyboard can open the card) but does not navigate.
///
/// Style it with `class`; it has no look of its own.
#[component]
pub fn HoverCardTrigger(
    /// Link target.
    #[prop(optional, into)]
    href: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let focusable = href.is_none().then_some("0");

    view! {
        <primitive::HoverCardTrigger
            attr:class=format!("vdl-hover-card-trigger {class}")
            attr:href=href
            attr:tabindex=focusable
        >
            {children.with_value(|children| children())}
        </primitive::HoverCardTrigger>
    }
}
