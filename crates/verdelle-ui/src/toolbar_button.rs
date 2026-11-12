use crate::primitives::toolbar as primitive;
use crate::{ButtonVariant, Size};
use leptos::prelude::*;

/// A button in a [`crate::Toolbar`], with the look of [`crate::Button`] (default variant:
/// ghost, small). A disabled toolbar button is skipped by the arrow keys. An icon-only
/// button needs `aria_label`.
#[component]
pub fn ToolbarButton(
    /// Visual variant: see `ButtonVariant`.
    #[prop(default = ButtonVariant::Ghost)]
    variant: ButtonVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(default = Size::Small)]
    size: Size,
    #[prop(optional)] icon_only: bool,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ToolbarButton
            disabled=disabled
            attr:class=format!("vdl-btn {class}")
            attr:data-variant=variant.as_str()
            attr:data-size=size.as_str()
            attr:data-icon-only=icon_only.then_some("")
            attr:aria-label=aria_label
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            <span class="vdl-btn__content">{children.with_value(|children| children())}</span>
        </primitive::ToolbarButton>
    }
}
