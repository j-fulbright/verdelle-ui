use crate::Size;
use crate::primitives::switch as primitive;
use leptos::prelude::*;

/// An on/off switch, styled as shadcn/ui's Switch and built on the headless
/// Switch (a `role="switch"` button with the thumb inside).
///
/// Uncontrolled by default (`default_checked` sets the start); pass `checked` and
/// `on_checked_change` to control it. `size` is `Size::Small` (24 x 14px) or the default
/// (32 x 18px); other sizes draw as the default. Name it with a [`crate::Label`]
/// pointing at its `id`, or with `aria_label`. Inside a `<form>` it submits `name`/`value`.
#[component]
pub fn Switch(
    /// Controlled checked state. Leave unset to let it manage itself (see `default_checked`).
    #[prop(into, optional)]
    checked: MaybeProp<bool>,
    /// Initial checked state when `checked` is not given.
    #[prop(into, optional)]
    default_checked: MaybeProp<bool>,
    /// Called with the new state whenever it changes.
    #[prop(into, optional)]
    on_checked_change: Option<Callback<bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Marks it as required (`required` / `aria-required`).
    #[prop(into, optional)]
    required: MaybeProp<bool>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Form field name; the value is submitted under it.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::Switch
            checked=checked
            default_checked=default_checked
            on_checked_change=on_checked_change.unwrap_or_else(|| Callback::new(|_: bool| {}))
            disabled=disabled
            required=required
            name=name
            value=value
            attr:id=id
            attr:aria-label=aria_label
            attr:data-size=size.as_str()
            attr:class=format!("vdl-switch {class}")
        >
            <primitive::SwitchThumb attr:class="vdl-switch__thumb" />
        </primitive::Switch>
    }
}
