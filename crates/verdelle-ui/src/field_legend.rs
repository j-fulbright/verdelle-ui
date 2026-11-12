use crate::FieldLegendVariant;
use leptos::prelude::*;

/// The `<legend>` of a [`crate::FieldSet`]: 16px (`legend`) or label-sized (`label`).
#[component]
pub fn FieldLegend(
    #[prop(optional)] variant: FieldLegendVariant,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <legend class=format!("vdl-field-legend {class}") data-variant=variant.as_str()>
            {children()}
        </legend>
    }
}
