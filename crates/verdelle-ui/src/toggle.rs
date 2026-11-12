use crate::primitives::toggle as primitive;
use crate::{Size, ToggleVariant};
use leptos::prelude::*;

/// A two-state button, styled as shadcn/ui's Toggle and built on the headless
/// Toggle (a `<button aria-pressed>`).
///
/// Uncontrolled by default (`default_pressed` sets the start); pass `pressed` and
/// `on_pressed_change` to control it. `variant` is default or outline. `size` is
/// `Size::Small` (28px), the default (32px) or `Size::Large` (36px); an icon-only toggle
/// needs `aria_label`.
#[component]
pub fn Toggle(
    #[prop(into, optional)] pressed: MaybeProp<bool>,
    #[prop(into, optional)] default_pressed: MaybeProp<bool>,
    #[prop(into, optional)] on_pressed_change: Option<Callback<bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(optional)] variant: ToggleVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Toggle
            pressed=pressed
            default_pressed=default_pressed
            on_pressed_change=on_pressed_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
            disabled=disabled
            attr:id=id
            attr:aria-label=aria_label
            attr:data-variant=variant.as_str()
            attr:data-size=size.as_str()
            attr:class=format!("vdl-toggle {class}")
        >
            {children.with_value(|children| children())}
        </primitive::Toggle>
    }
}
