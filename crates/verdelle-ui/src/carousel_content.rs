use crate::CarouselApi;
use leptos::{ev, prelude::*};

/// The scrolling area of a `Carousel` that holds the `CarouselItem`s, styled as shadcn/ui's
/// CarouselContent.
///
/// It is the scroll container (native scrolling with scroll-snap, scrollbars hidden); `class`
/// lands on it, so give a vertical carousel its height here (`h-[200px]`). Must be inside a
/// `Carousel`.
#[component]
pub fn CarouselContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let api = use_context::<CarouselApi>().expect("CarouselContent must be inside a Carousel");

    view! {
        <div
            class=format!("vdl-carousel__viewport {class}")
            node_ref=api.viewport
            tabindex="0"
            data-orientation=api.orientation().to_string()
            on:scroll=move |_| api.on_scroll()
            on:wheel=move |_| api.interrupt()
            on:touchstart=move |_| api.interrupt()
            on:pointerdown=move |ev: ev::PointerEvent| api.pointer_down(&ev)
            on:pointermove=move |ev: ev::PointerEvent| api.pointer_move(&ev)
            on:pointerup=move |ev: ev::PointerEvent| api.pointer_end(&ev)
            on:pointercancel=move |ev: ev::PointerEvent| api.pointer_end(&ev)
            on:dragstart=move |ev: ev::DragEvent| ev.prevent_default()
        >
            <div
                class="vdl-carousel__content"
                data-orientation=api.orientation().to_string()
                aria-live=move || api.live_region()>
                {children()}
            </div>
        </div>
    }
}
