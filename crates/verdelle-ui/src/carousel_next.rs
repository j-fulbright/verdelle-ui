use crate::{Button, ButtonVariant, CarouselApi, Icon, IconName, Size};
use leptos::prelude::*;

/// The next-slide button of a `Carousel`, styled as shadcn/ui's CarouselNext: an outline icon
/// button beside the slides (right, or below when vertical). It is disabled at the end unless
/// the carousel loops.
#[component]
pub fn CarouselNext(
    /// Visual variant: see `ButtonVariant`.
    #[prop(default = ButtonVariant::Outline)]
    variant: ButtonVariant,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let api = use_context::<CarouselApi>().expect("CarouselNext must be inside a Carousel");

    view! {
        <Button
            variant=variant
            size=Size::Small
            icon_only=true
            class=format!("vdl-carousel__next {class}")
            aria_label="Next slide"
            disabled=Signal::derive(move || !api.can_scroll_next())
            on_click=move |_| api.scroll_next()
        >
            <Icon name=IconName::ChevronRight />
        </Button>
    }
}
