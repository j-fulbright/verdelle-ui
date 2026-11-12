use crate::icons;
use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// One pick of a `multiple` [`crate::Combobox`], drawn as a small pill. The children are its
/// label; `show_remove` (default true) adds the X button that removes it.
#[component]
pub fn ComboboxChip(
    #[prop(into)] value: String,
    #[prop(default = true)] show_remove: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxChip value=value attr:class=format!("vdl-combobox__chip {class}")>
            {children.with_value(|children| children())}
            {show_remove
                .then(|| {
                    view! {
                        <primitive::ComboboxChipRemove
                            attr:class="vdl-btn vdl-combobox__chip-remove"
                            attr:data-variant="ghost"
                            attr:data-size="xs"
                            attr:data-icon-only=""
                        >
                            <span class="vdl-btn__content">{icons::x()}</span>
                        </primitive::ComboboxChipRemove>
                    }
                })}
        </primitive::ComboboxChip>
    }
}
