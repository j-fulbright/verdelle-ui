use leptos::prelude::*;

/// A pulsing placeholder for content that is loading, styled as shadcn/ui's Skeleton.
///
/// It has no size of its own: give it one with `class` (for example Tailwind's
/// `h-4 w-48`, or `size-12 rounded-full` for an avatar). The shape is a muted
/// rounded box; the pulse stops under `prefers-reduced-motion`. It is decorative
/// and hidden from assistive technology, so mark the loading region itself
/// (for example with `aria-busy`).
#[component]
pub fn Skeleton(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div class=format!("vdl-skeleton {class}") aria-hidden="true"></div> }
}
