use crate::primitives::avatar as primitive;
use leptos::prelude::*;

pub use primitive::ImageLoadingStatus;

/// The picture. It is only put in the page once it has loaded, so a broken `src` never
/// shows a broken-image icon; [`crate::AvatarFallback`] shows instead.
/// `on_loading_status_change` reports `Loading`, then `Loaded` or `Error`.
#[component]
pub fn AvatarImage(
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] alt: MaybeProp<String>,
    #[prop(into, optional)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <primitive::AvatarImage
            src=src
            on_loading_status_change=on_loading_status_change
                .unwrap_or_else(|| Callback::new(|_: ImageLoadingStatus| {}))
            alt=Signal::derive(move || Some(alt.get().unwrap_or_default()))
            class=format!("vdl-avatar__image {class}")
        />
    }
}
