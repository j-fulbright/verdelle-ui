use leptos::prelude::*;

/// The top of a sidebar: a column with 8px padding and gap. Put the brand, a team switcher or a
/// [`crate::SidebarInput`] in it.
#[component]
pub fn SidebarHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__header {class}") data-sidebar="header">
            {children()}
        </div>
    }
}
