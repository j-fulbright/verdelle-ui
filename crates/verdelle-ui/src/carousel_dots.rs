use crate::CarouselApi;
use leptos::prelude::*;

/// Dot indicators for a `Carousel`: one button per snap position, the current one marked
/// (`data-active`, `aria-current`). Clicking a dot scrolls there. shadcn has no dots; Embla's
/// docs show them as a recipe. They appear once the page is interactive, because the number of
/// snaps comes from measuring the slides.
#[component]
pub fn CarouselDots(#[prop(optional, into)] class: String) -> impl IntoView {
    let api = use_context::<CarouselApi>().expect("CarouselDots must be inside a Carousel");

    view! {
        <div class=format!("vdl-carousel__dots {class}")>
            {move || {
                (0..api.count())
                    .map(|i| {
                        view! {
                            <button
                                type="button"
                                class="vdl-carousel__dot"
                                aria-label=format!("Go to slide {}", i + 1)
                                data-active=move || (api.index() == i).then_some("")
                                aria-current=move || (api.index() == i).then_some("true")
                                on:click=move |_| api.scroll_to(i)
                            />
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
