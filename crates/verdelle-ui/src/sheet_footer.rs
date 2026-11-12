use leptos::prelude::*;

/// The actions at the bottom of a sheet. It sticks to the bottom edge (`margin-top: auto`), with
/// 16px padding; put [`crate::Button`]s or [`crate::SheetClose`] in it.
#[component]
pub fn SheetFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-sheet__footer {class}")>{children()}</div> }
}
