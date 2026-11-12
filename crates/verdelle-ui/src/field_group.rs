use leptos::prelude::*;

/// A stack of [`crate::Field`]s with the spacing shadcn uses. It is also the container that
/// responsive Fields measure, so a Field switches to a row when its group is wide enough.
/// Groups nest (a nested group is tighter).
#[component]
pub fn FieldGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-field-group {class}")>{children()}</div> }
}
