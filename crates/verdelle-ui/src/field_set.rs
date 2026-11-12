use leptos::prelude::*;

/// A `<fieldset>` of related [`crate::Field`]s, named by a [`crate::FieldLegend`]. A disabled
/// fieldset disables every control inside it, as native HTML does.
#[component]
pub fn FieldSet(
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <fieldset class=format!("vdl-field-set {class}") disabled=move || disabled.get()>
            {children()}
        </fieldset>
    }
}
