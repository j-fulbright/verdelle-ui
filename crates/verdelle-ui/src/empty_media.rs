use crate::EmptyMediaVariant;
use leptos::prelude::*;

/// The illustration of an [`crate::Empty`]: an icon in a tile (`Icon`) or any content (`Default`).
#[component]
pub fn EmptyMedia(
    #[prop(optional, into)] variant: Signal<EmptyMediaVariant>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-empty__media {class}") data-variant=move || variant.get().as_str()>
            {children()}
        </div>
    }
}
