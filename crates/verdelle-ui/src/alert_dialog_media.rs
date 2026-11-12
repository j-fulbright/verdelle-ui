use leptos::prelude::*;

/// A 40px tile for an icon, shown beside (default size) or above (small) the title.
#[component]
pub fn AlertDialogMedia(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-alert-dialog__media {class}") aria-hidden="true">
            {children()}
        </div>
    }
}
