use crate::{AlertVariant, Icon, IconName};
use leptos::prelude::*;

/// A callout for a message, styled as shadcn/ui's Alert. Compose with
/// [`crate::AlertTitle`], [`crate::AlertDescription`] and [`crate::AlertAction`].
///
/// A destructive or warning alert has `role="alert"` (announced at once); a default or success
/// one has `role="status"` (announced politely), which suits progress messages like
/// "Starting QA1...".
///
/// `icon` draws a leading icon; the title and description line up beside it (shadcn's
/// `has-[>svg]` grid, which also applies to an `<svg>` you place first inside the alert).
/// `dismissible` adds a close button that hides the alert and calls `on_dismiss`.
/// `Success` and `Warning` are Verdelle extras; shadcn has only default and destructive.
#[component]
pub fn Alert(
    #[prop(optional, into)] variant: Signal<AlertVariant>,
    /// A leading icon. Takes an `IconName`, an `Option` of one, or a signal of an `Option`.
    #[prop(optional, into)]
    icon: MaybeProp<IconName>,
    /// Adds a close button. The alert hides itself; `on_dismiss` tells the parent.
    #[prop(optional, into)]
    dismissible: Signal<bool>,
    /// The close button's accessible name.
    #[prop(default = "Dismiss".to_string(), into)]
    dismiss_label: String,
    /// Called after the close button hides the alert.
    #[prop(optional, into)]
    on_dismiss: Option<Callback<()>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let dismissed = RwSignal::new(false);
    view! {
        <div
            class=format!("vdl-alert {class}")
            role=move || variant.get().role()
            data-variant=move || variant.get().as_str()
            data-dismissible=move || dismissible.get().then_some("")
            hidden=move || dismissed.get()
        >
            {move || icon.get().map(|name| view! { <Icon name=name /> })}
            {children()}
            {move || {
                dismissible
                    .get()
                    .then(|| {
                        let label = dismiss_label.clone();
                        view! {
                            <button
                                type="button"
                                class="vdl-alert__dismiss"
                                aria-label=label
                                on:click=move |_| {
                                    dismissed.set(true);
                                    if let Some(cb) = on_dismiss {
                                        cb.run(());
                                    }
                                }
                            >
                                <Icon name=IconName::X />
                            </button>
                        }
                    })
            }}
        </div>
    }
}
