use leptos::prelude::*;

/// The actions at the bottom of a drawer. It sticks to the bottom edge (`margin-top: auto`), with
/// 16px padding; put [`crate::Button`]s or [`crate::DrawerClose`] in it.
#[component]
pub fn DrawerFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-drawer__footer {class}")>{children()}</div> }
}
