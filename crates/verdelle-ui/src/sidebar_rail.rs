use crate::sidebar_context::use_sidebar;
use leptos::prelude::*;

/// A thin strip along the sidebar's edge that toggles it when clicked (it is not in the tab
/// order; the trigger is the accessible control). Put it inside the [`crate::Sidebar`], last.
#[component]
pub fn SidebarRail(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = use_sidebar();
    view! {
        <button
            type="button"
            class=format!("vdl-sidebar__rail {class}")
            data-sidebar="rail"
            aria-label="Toggle Sidebar"
            title="Toggle Sidebar"
            tabindex="-1"
            on:click=move |_| ctx.toggle()
        ></button>
    }
}
