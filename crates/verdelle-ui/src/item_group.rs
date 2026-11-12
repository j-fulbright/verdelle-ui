use leptos::prelude::*;

/// A vertical list of [`crate::Item`]s (`role="list"`), 16px apart, or closer when it holds
/// small or extra small items. Put an [`crate::ItemSeparator`] between rows for a divided list.
#[component]
pub fn ItemGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("vdl-item-group {class}") role="list">
            {children()}
        </div>
    }
}
