use crate::primitives::toolbar as primitive;
use crate::toggle_group::ToggleGroupStyle;
use crate::{Size, ToggleVariant};
use leptos::prelude::*;

/// One toggle of a [`crate::ToolbarToggleGroup`]. `value` is what the group's value contains
/// while it is pressed. An icon-only item needs `aria_label`.
#[component]
pub fn ToolbarToggleItem(
    #[prop(into)] value: String,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(optional)] variant: Option<ToggleVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Option<Size>,
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
    let group = use_context::<ToggleGroupStyle>();
    let variant = variant.or(group.map(|g| g.variant)).unwrap_or_default();
    let size = size.or(group.map(|g| g.size)).unwrap_or_default();

    view! {
        <primitive::ToolbarToggleItem
            value=value
            disabled=disabled
            attr:aria-label=aria_label
            attr:data-variant=variant.as_str()
            attr:data-size=size.as_str()
            attr:data-spacing="0"
            attr:class=format!("vdl-toggle vdl-toggle-group__item {class}")
        >
            {children.with_value(|children| children())}
        </primitive::ToolbarToggleItem>
    }
}
