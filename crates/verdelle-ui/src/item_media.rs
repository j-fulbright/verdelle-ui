use crate::ItemMediaVariant;
use leptos::prelude::*;

/// The leading icon or image of an [`crate::Item`]. When the item has a description, the
/// media aligns to the top, nudged 2px down to line up with the title.
#[component]
pub fn ItemMedia(
    #[prop(optional, into)] variant: Signal<ItemMediaVariant>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("vdl-item__media {class}") data-variant=move || variant.get().as_str()>
            {children()}
        </div>
    }
}
