use leptos::prelude::*;

/// One slide of a `Carousel`, styled as shadcn/ui's CarouselItem.
///
/// A slide fills the viewport by default. Size it with a basis class through `class` to show
/// several at once (Tailwind's `basis-1/2`, `md:basis-1/3`), or set `slides_per_view` on the
/// `Carousel`. Once the page is interactive each slide is labelled "n of m" for assistive
/// technology (unless it already has an `aria-label` of its own).
#[component]
pub fn CarouselItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    // `data-vdl-carousel-item` is `crate::primitives::carousel::ITEM_ATTRIBUTE`, which the model
    // uses to find the slides.
    view! {
        <div
            class=format!("vdl-carousel__item {class}")
            role="group"
            aria-roledescription="slide"
            data-vdl-carousel-item=""
        >
            {children()}
        </div>
    }
}
