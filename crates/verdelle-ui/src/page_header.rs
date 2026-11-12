use crate::{Icon, IconName};
use leptos::prelude::*;

/// The top of a page: an optional breadcrumb or back link, the title, an optional description,
/// and optional actions (buttons) at the end. A Verdelle extra (shadcn has no page header; see
/// `NAMING.md`).
///
/// The title is an `<h1>` by default (`level` 2 to 4 for a header inside a page that has its own h1; 24px, semibold, tight tracking; 20px on phones). The actions sit
/// beside the title and wrap below it when the row is too narrow, so on a phone they take a row of
/// their own. `breadcrumb` is a slot for a [`crate::Breadcrumb`]; `back_href` is the simple
/// alternative, a "< Back" link above the title (`back_label` words it). When both are given the
/// breadcrumb wins. `title`, `description` and `back_href` take plain values or signals, so they can
/// follow loaded data; `description` and `back_href` also take an `Option`.
#[component]
pub fn PageHeader(
    /// The page title.
    #[prop(into)]
    title: Signal<String>,
    /// A line under the title, in muted text. Left out while empty or `None`.
    #[prop(optional, into)]
    description: MaybeProp<String>,
    /// Buttons or other controls at the end of the title row.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// A breadcrumb trail above the title.
    #[prop(optional, into)]
    breadcrumb: Option<ViewFn>,
    /// Shows a back link above the title that goes here (ignored when `breadcrumb` is given).
    #[prop(optional, into)]
    back_href: MaybeProp<String>,
    /// The back link's text.
    #[prop(default = "Back".to_string(), into)]
    back_label: String,
    /// The heading level of the title: 1 (default) to 4. Same look at every level.
    #[prop(default = 1)]
    level: u8,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let has_breadcrumb = breadcrumb.is_some();
    let text = move || title.get();
    let heading = match level {
        2 => view! { <h2 class="vdl-page-header__title">{text}</h2> }.into_any(),
        3 => view! { <h3 class="vdl-page-header__title">{text}</h3> }.into_any(),
        4 => view! { <h4 class="vdl-page-header__title">{text}</h4> }.into_any(),
        _ => view! { <h1 class="vdl-page-header__title">{text}</h1> }.into_any(),
    };
    view! {
        <header class=format!("vdl-page-header {class}")>
            {breadcrumb.map(|b| view! { <div class="vdl-page-header__crumbs">{b.run()}</div> })}
            {move || {
                let label = back_label.clone();
                (!has_breadcrumb)
                    .then(|| back_href.get())
                    .flatten()
                    .map(|href| {
                        view! {
                            <a class="vdl-page-header__back" href=href>
                                <Icon name=IconName::ChevronLeft />
                                {label}
                            </a>
                        }
                    })
            }}
            <div class="vdl-page-header__row">
                <div class="vdl-page-header__text">
                    {heading}
                    {move || {
                        description
                            .get()
                            .filter(|d| !d.is_empty())
                            .map(|d| view! { <p class="vdl-page-header__description">{d}</p> })
                    }}
                </div>
                {actions.map(|a| view! { <div class="vdl-page-header__actions">{a.run()}</div> })}
            </div>
        </header>
    }
}
