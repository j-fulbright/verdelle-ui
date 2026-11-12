use crate::icons;
use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// One choice. `value` is what the combobox holds; `label` (default: the value) is what a typed
/// search matches and what the input shows once it is picked; `keywords` are extra words it
/// matches. The children are what the list shows. A picked item gets a check mark at the end.
/// It is hidden while it does not match what has been typed.
#[component]
pub fn ComboboxItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] keywords: MaybeProp<Vec<String>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Called when the item is chosen, by click or keyboard.
    #[prop(into, optional)]
    on_select: Option<Callback<()>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxItem
            value=value
            label=label
            keywords=keywords
            disabled=disabled
            on_select=on_select.unwrap_or_else(|| Callback::new(|()| {}))
            class=format!("vdl-combobox__item {class}")
        >
            {children.with_value(|children| children())}
            <primitive::ComboboxItemIndicator attr:class="vdl-combobox__item-indicator">
                {icons::check()}
            </primitive::ComboboxItemIndicator>
        </primitive::ComboboxItem>
    }
}
