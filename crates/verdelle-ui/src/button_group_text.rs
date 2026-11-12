use leptos::prelude::*;

/// Static text (or an icon and text) joined to the buttons of a [`crate::ButtonGroup`]: a
/// muted, bordered cell with the same height as a button.
#[component]
pub fn ButtonGroupText(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-button-group__text {class}")>{children()}</div> }
}
