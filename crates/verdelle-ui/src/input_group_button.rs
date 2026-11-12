use crate::{Button, ButtonVariant, InputGroupButtonSize, Size};
use leptos::{ev, prelude::*};

/// A [`crate::Button`] sized to sit inside an [`crate::InputGroupAddon`]: ghost by default,
/// no shadow, and small corners that nest inside the group's.
#[component]
pub fn InputGroupButton(
    /// Visual variant: see `ButtonVariant`.
    #[prop(default = ButtonVariant::Ghost.into(), into)]
    variant: Signal<ButtonVariant>,
    #[prop(optional, into)] size: Signal<InputGroupButtonSize>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional, into)] selected: Signal<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// The content.
    children: Children,
) -> impl IntoView {
    let button_size = Signal::derive(move || match size.get() {
        InputGroupButtonSize::Xs | InputGroupButtonSize::IconXs => Size::XSmall,
        InputGroupButtonSize::Sm | InputGroupButtonSize::IconSm => Size::Small,
    });
    let icon_only = Signal::derive(move || {
        matches!(
            size.get(),
            InputGroupButtonSize::IconXs | InputGroupButtonSize::IconSm
        )
    });
    view! {
        <Button
            variant=variant
            size=button_size
            icon_only=icon_only
            disabled=disabled
            selected=selected
            class=format!("vdl-input-group__button {class}")
            attr:aria-label=aria_label
            on_click=Callback::new(move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            })
            attr:data-ig-size=move || size.get().as_str()
        >
            {children()}
        </Button>
    }
}
