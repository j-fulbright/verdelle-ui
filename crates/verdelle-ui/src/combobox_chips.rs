use crate::primitives::combobox as primitive;
use leptos::prelude::*;

/// The frame of a `multiple` [`crate::Combobox`]: a bordered, wrapping row holding
/// [`crate::ComboboxChip`]s (or a [`crate::ComboboxChipList`]) and a
/// [`crate::ComboboxChipsInput`]. It is the anchor the popup is placed under, and pressing it
/// focuses the input. `invalid` draws the destructive border.
#[component]
pub fn ComboboxChips(
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::ComboboxChips
            attr:class=format!("vdl-combobox__chips {class}")
            attr:data-invalid=move || invalid.get().then_some("")
        >
            {children.with_value(|children| children())}
        </primitive::ComboboxChips>
    }
}
