use crate::{Avatar, AvatarFallback, Size};
use leptos::prelude::*;

/// The signed-in person for a shell's `user_slot`: an avatar with initials, a name and an
/// email on the first row, and the actions (a mode toggle, a sign-out button, ...) on a row of
/// their own below, wrapping when there are many. The name and email get the whole width of the
/// row (a long one is cut with an ellipsis and its full text is the `title`). In the icon rail
/// only the avatar shows, centred.
///
/// `name`, `email` and `initials` take plain values or signals (`Signal::derive(...)`, a
/// `Memo`), so they can change after the user loads. `initials` default to the first letters
/// of the name and follow it; the email line is left out while the email is empty.
#[component]
pub fn ShellUser(
    #[prop(into)] name: Signal<String>,
    #[prop(optional, into)] email: Signal<String>,
    /// Default: the first letters of the name (up to two), upper-cased.
    #[prop(optional, into)]
    initials: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Put the actions at the end of the first row, beside the name, instead of on a row of
    /// their own. Meant for one or two icon-only buttons (a sign-out icon, say); the default
    /// row below suits text buttons. Hidden in the icon rail either way.
    #[prop(optional)]
    inline_actions: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let initials = Signal::derive(move || {
        initials
            .get()
            .unwrap_or_else(|| name.with(|n| crate::initials(n)))
    });
    let (children_inline, children_below) = match (children, inline_actions) {
        (Some(c), true) => (
            Some(view! { <div class="vdl-shell-user__actions" data-inline="">{c()}</div> }),
            None,
        ),
        (Some(c), false) => (
            None,
            Some(view! { <div class="vdl-shell-user__actions">{c()}</div> }),
        ),
        (None, _) => (None, None),
    };
    view! {
        <div class=format!("vdl-shell-user {class}")>
            <div class="vdl-shell-user__row">
                <Avatar size=Size::Medium>
                    <AvatarFallback>{move || initials.get()}</AvatarFallback>
                </Avatar>
                <div class="vdl-shell-user__text">
                    <p class="vdl-shell-user__name" title=move || name.get()>{move || name.get()}</p>
                    {move || {
                        let email = email.get();
                        let title = email.clone();
                        (!email.is_empty())
                            .then(|| view! { <p class="vdl-shell-user__email" title=title>{email}</p> })
                    }}
                </div>
                {children_inline}
            </div>
            {children_below}
        </div>
    }
}
