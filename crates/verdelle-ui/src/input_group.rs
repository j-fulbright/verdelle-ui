use leptos::prelude::*;

/// What an [`InputGroup`] tells the controls inside it.
#[derive(Clone, Copy)]
pub(crate) struct InputGroupContext {
    pub disabled: Signal<bool>,
    pub invalid: Signal<bool>,
}

/// One frame around an input or textarea and its addons, styled as shadcn/ui's Input Group:
/// one border, one focus ring. Put an [`crate::InputGroupInput`] (or
/// [`crate::InputGroupTextarea`]) and any number of [`crate::InputGroupAddon`]s inside.
///
/// `disabled` dims the group and disables its control; `invalid` draws the destructive
/// border and ring and sets `aria-invalid` on the control. The group is 32px tall, taller
/// when it holds a textarea or a block addon.
#[component]
pub fn InputGroup(
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The content.
    children: Children,
) -> impl IntoView {
    provide_context(InputGroupContext { disabled, invalid });
    view! {
        <div
            class=format!("vdl-input-group {class}")
            role="group"
            aria-label=aria_label
            data-disabled=move || disabled.get().then_some("")
            data-invalid=move || invalid.get().then_some("")
        >
            {children()}
        </div>
    }
}
