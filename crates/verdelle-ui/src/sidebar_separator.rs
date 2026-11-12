use crate::Separator;
use leptos::prelude::*;

/// A horizontal rule between sidebar sections, in the sidebar-border colour with 8px margins.
#[component]
pub fn SidebarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <Separator decorative=true class=format!("vdl-sidebar__separator {class}") /> }
}
