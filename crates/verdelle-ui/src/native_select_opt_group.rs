use leptos::prelude::*;

/// An `<optgroup>` of a [`crate::NativeSelect`]: a labelled set of options.
#[component]
pub fn NativeSelectOptGroup(
    #[prop(into)] label: String,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <optgroup class=format!("vdl-native-select__option {class}") label=label disabled=disabled>
            {children()}
        </optgroup>
    }
}
