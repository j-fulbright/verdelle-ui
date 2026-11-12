use crate::{Button, ButtonVariant, CarouselApi, Icon, IconName, Size};
use leptos::prelude::*;

/// The previous-slide button of a `Carousel`, styled as shadcn/ui's CarouselPrevious: an outline
/// icon button beside the slides (left, or above when vertical). It is disabled at the start
/// unless the carousel loops.
#[component]
pub fn CarouselPrevious(
    /// Visual variant: see `ButtonVariant`.
    #[prop(default = ButtonVariant::Outline)]
    variant: ButtonVariant,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let api = use_context::<CarouselApi>().expect("CarouselPrevious must be inside a Carousel");

    view! {
        <Button
            variant=variant
            size=Size::Small
            icon_only=true
            class=format!("vdl-carousel__previous {class}")
            aria_label="Previous slide"
            disabled=Signal::derive(move || !api.can_scroll_prev())
            on_click=move |_| api.scroll_prev()
        >
            <Icon name=IconName::ChevronLeft />
        </Button>
    }
}
