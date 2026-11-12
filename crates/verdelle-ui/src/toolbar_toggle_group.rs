use crate::primitives::toolbar as primitive;
use crate::toggle_group::ToggleGroupStyle;
use crate::{Size, ToggleVariant};
use leptos::{context::Provider, prelude::*};

/// A group of toggle buttons inside a [`crate::Toolbar`]. One item can be pressed at a time
/// unless `multiple` is set. `value` is the list of pressed items' values; uncontrolled by
/// default (`default_value`), or pass `value` and `on_value_change`. `variant` and `size`
/// apply to every item. Add [`crate::ToolbarToggleItem`]s.
#[component]
pub fn ToolbarToggleGroup(
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<Vec<String>>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Vec<String>>>,
    #[prop(optional)] multiple: bool,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(optional)] variant: ToggleVariant,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(default = Size::Small)]
    size: Size,
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
        <Provider value=ToggleGroupStyle {
            variant,
            size,
            spacing: 0,
        }>
            <primitive::ToolbarToggleGroup
                r#type=if multiple { "multiple" } else { "single" }
                value=value
                default_value=default_value
                on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Vec<String>| {}))
                disabled=disabled
                attr:aria-label=aria_label
                attr:data-variant=variant.as_str()
                attr:data-size=size.as_str()
                attr:data-spacing="0"
                attr:class=format!("vdl-toggle-group {class}")
            >
                {children.with_value(|children| children())}
            </primitive::ToolbarToggleGroup>
        </Provider>
    }
}
