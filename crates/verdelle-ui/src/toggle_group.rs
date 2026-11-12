use crate::primitives::toggle_group as primitive;
use crate::{Orientation, Size, ToggleVariant};
use leptos::{context::Provider, prelude::*};

/// What a [`crate::ToggleGroup`] hands its items.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ToggleGroupStyle {
    pub variant: ToggleVariant,
    pub size: Size,
    pub spacing: u8,
}

/// A row (or column) of [`crate::ToggleGroupItem`]s, styled as shadcn/ui's ToggleGroup and
/// built on the headless ToggleGroup (a `role="group"` of `aria-pressed` buttons
/// with a roving tab stop; arrow keys, Home and End move focus).
///
/// One item can be pressed at a time unless `multiple` is set (Base UI's name for Radix's
/// `type="multiple"`). `value` is the list of pressed items' values; uncontrolled by
/// default (`default_value`), or pass `value` and `on_value_change`. `variant` and `size`
/// apply to every item. `spacing` is the gap in 4px steps (shadcn's default is 2); at 0
/// the items join into one bar and share borders. Name the group with `aria_label`.
#[component]
pub fn ToggleGroup(
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<Vec<String>>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Vec<String>>>,
    #[prop(optional)] multiple: bool,
    /// Single mode only: whether pressing the pressed item clears the group. On by default
    /// (as in Radix, Base UI and shadcn), so the value may be empty; set it to
    /// `false` for a segmented control that always has one choice. It does not choose the initial
    /// value: start from `default_value` / `value` so something is pressed. Ignored with `multiple`.
    #[prop(default = true)]
    deselectable: bool,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] variant: ToggleVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    #[prop(default = 2)] spacing: u8,
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
    let group_type = if multiple {
        primitive::ToggleGroupType::Multiple
    } else {
        primitive::ToggleGroupType::Single
    };

    view! {
        <Provider value=ToggleGroupStyle {
            variant,
            size,
            spacing,
        }>
            <primitive::ToggleGroup
                r#type=group_type
                value=value
                default_value=default_value
                on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Vec<String>| {}))
                disabled=disabled
                deselectable=deselectable
                orientation=orientation.to_string()
                attr:aria-label=aria_label
                attr:data-variant=variant.as_str()
                attr:data-size=size.as_str()
                attr:data-spacing=spacing.to_string()
                attr:style=format!("--vdl-toggle-gap:{spacing}")
                attr:class=format!("vdl-toggle-group {class}")
            >
                {children.with_value(|children| children())}
            </primitive::ToggleGroup>
        </Provider>
    }
}
