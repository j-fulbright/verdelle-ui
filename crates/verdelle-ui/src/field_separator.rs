use crate::Separator;
use leptos::prelude::*;

/// A line between [`crate::Field`]s in a group, optionally with text in the middle
/// ("or continue with").
#[component]
pub fn FieldSeparator(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_content = children.is_some();
    view! {
        <div
            class=format!("vdl-field-separator {class}")
            data-content=has_content.then_some("true")
        >
            <Separator decorative=true class="vdl-field-separator__line" />
            {children
                .map(|c| view! { <span class="vdl-field-separator__content">{c()}</span> })}
        </div>
    }
}
