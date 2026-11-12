use crate::primitives::select as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// One choice. `value` is what the select holds; the children are what it shows.
#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::SelectItem
            value=value
            disabled=disabled
            attr:class=format!("vdl-select__item {class}")
        >
            <primitive::SelectItemText attr:class="vdl-select__item-text">
                {children.with_value(|children| children())}
            </primitive::SelectItemText>
            <primitive::SelectItemIndicator attr:class="vdl-select__item-indicator">
                <Icon name=IconName::Check />
            </primitive::SelectItemIndicator>
        </primitive::SelectItem>
    }
}
