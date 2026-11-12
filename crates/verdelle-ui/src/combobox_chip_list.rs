use crate::ComboboxChip;
use crate::primitives::combobox::use_combobox_values;
use leptos::prelude::*;

/// A [`crate::ComboboxChip`] for every value the enclosing `multiple` combobox holds, labelled
/// with the item's label. Use it inside [`crate::ComboboxChips`] before the input. (shadcn's
/// examples map over the value array by hand; this does it for uncontrolled comboboxes too. To
/// render something else per value, read the values with [`crate::use_combobox_values`].)
#[component]
pub fn ComboboxChipList(
    #[prop(default = true)] show_remove: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let values = use_combobox_values();
    view! {
        <For
            each=move || values.get()
            key=|(value, _)| value.clone()
            children=move |(value, _)| {
                let label_value = value.clone();
                let label = Memo::new(move |_| {
                    values
                        .get()
                        .into_iter()
                        .find(|(v, _)| *v == label_value)
                        .map(|(_, l)| l)
                        .unwrap_or_default()
                });
                view! {
                    <ComboboxChip value=value show_remove=show_remove class=class.clone()>
                        {move || label.get()}
                    </ComboboxChip>
                }
            }
        />
    }
}
