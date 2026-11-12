use crate::FieldOrientation;
use leptos::prelude::*;

/// One form field: a label, a control, and the text that goes with them. Styled as
/// shadcn/ui's Field (`role="group"`).
///
/// Compose with [`crate::FieldLabel`], [`crate::FieldContent`], [`crate::FieldDescription`]
/// and [`crate::FieldError`] around any control. `orientation` is vertical (default),
/// horizontal or responsive (see [`FieldOrientation`]). `invalid` turns the text destructive
/// (mark the control itself invalid too: `Input`, `Textarea` and `Checkbox` take `invalid`),
/// and `disabled` dims the label (disable the control yourself). Field does not wire ids:
/// point the label's `html_for` at the control's `id`, and the control's `aria_describedby`
/// at the description's or error's `id`.
#[component]
pub fn Field(
    #[prop(optional)] orientation: FieldOrientation,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
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
        <div
            role="group"
            class=format!("vdl-field {class}")
            data-orientation=orientation.as_str()
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=move || disabled.get().then_some("true")
        >
            {children()}
        </div>
    }
}
