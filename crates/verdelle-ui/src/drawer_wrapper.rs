use leptos::prelude::*;

/// Wrap the page (or the part of it that should recede) in this for a [`crate::Drawer`] with
/// `scale_background`: while that drawer is open the wrapper shrinks a little and gets rounded
/// corners, following the drawer as it is dragged. Without a wrapper the option does nothing.
#[component]
pub fn DrawerWrapper(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div data-vdl-drawer-wrapper class=class>
            {children()}
        </div>
    }
}
