use crate::icons;
use leptos::prelude::*;

/// The mark between two [`crate::BreadcrumbItem`]s. It is an `<li>` of its own, hidden from
/// assistive technology. Without children it draws a chevron; pass children (a `/`, say)
/// to replace it.
#[component]
pub fn BreadcrumbSeparator(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li class=format!("vdl-breadcrumb__separator {class}") role="presentation" aria-hidden="true">
            {match children {
                Some(children) => children().into_any(),
                None => icons::chevron_right().into_any(),
            }}
        </li>
    }
}
