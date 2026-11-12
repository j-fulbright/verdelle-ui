use leptos::prelude::*;

/// An `<option>` of a [`crate::NativeSelect`], drawn in the system's menu colours.
/// `selected` sets the initial choice when the select has no `value` signal.
#[component]
pub fn NativeSelectOption(
    #[prop(into)] value: String,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: bool,
    #[prop(optional)] selected: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <option
            class=format!("vdl-native-select__option {class}")
            value=value
            disabled=disabled
            selected=selected
        >
            {children()}
        </option>
    }
}
