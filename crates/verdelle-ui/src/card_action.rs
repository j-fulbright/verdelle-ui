use leptos::prelude::*;

/// Something placed at the end of a [`crate::CardHeader`], beside the title (a badge,
/// a button, a menu, a corner icon).
///
/// `stack` makes the action drop under the title and description when the header is narrower
/// than 22rem (a phone, a small tile), instead of squeezing them. It is off by default, which
/// is shadcn's layout: the action stays in the corner whatever the width. Use it for a text
/// button; leave it off for a corner icon.
#[component]
pub fn CardAction(
    #[prop(optional, into)] class: String,
    /// Drop under the title in a header narrower than 22rem.
    #[prop(optional)]
    stack: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-card__action {class}") data-stack=stack.then_some("")>
            {children()}
        </div>
    }
}
