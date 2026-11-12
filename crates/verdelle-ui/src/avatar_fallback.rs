use crate::primitives::avatar as primitive;
use leptos::prelude::*;

/// What shows while the image loads, or when it fails or is missing: usually initials.
/// `delay_ms` waits before showing it, so a fast image does not flash the fallback first.
///
/// Give `name` instead of children to show the person's initials ([`crate::initials`]: up to two
/// letters, derived from the name and following it when it is a signal); the name is the `title` of
/// the initials. Children, when given, win.
#[component]
pub fn AvatarFallback(
    #[prop(into, optional)] delay_ms: MaybeProp<i32>,
    /// A person's name, to show its initials. Takes a string, an `Option` or a signal of an `Option`.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content. Optional when `name` is given.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <primitive::AvatarFallback
            delay_ms=delay_ms
            class=format!("vdl-avatar__fallback {class}")
        >
            {match children.as_ref() {
                Some(children) => children().into_any(),
                None => {
                    view! {
                        <span title=move || name.get()>
                            {move || name.get().map(|n| crate::initials(&n))}
                        </span>
                    }
                        .into_any()
                }
            }}
        </primitive::AvatarFallback>
    }
}
