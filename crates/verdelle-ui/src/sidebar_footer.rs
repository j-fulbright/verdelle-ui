use leptos::prelude::*;

/// The bottom of a sidebar: a column with 8px padding and gap. Put the user menu or a version
/// label in it.
#[component]
pub fn SidebarFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("vdl-sidebar__footer {class}") data-sidebar="footer">
            {children()}
        </div>
    }
}
