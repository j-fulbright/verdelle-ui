use leptos::prelude::*;

/// The action row of an alert dialog: stacked on narrow screens, right-aligned from 640px
/// (two equal columns in a `Size::Small` dialog).
#[component]
pub fn AlertDialogFooter(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("vdl-alert-dialog__footer {class}")>{children()}</div> }
}
