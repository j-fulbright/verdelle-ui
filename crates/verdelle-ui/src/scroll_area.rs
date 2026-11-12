use crate::ScrollAreaOrientation;
use crate::primitives::scroll_area as primitive;
use leptos::prelude::*;

pub use primitive::ScrollAreaType;

/// A scrollable region with themed scrollbars, styled as shadcn/ui's Scroll Area and built
/// on the headless Scroll Area (native scrolling with the browser's bars hidden and
/// our own bar and thumb drawn from the measured sizes; the viewport is focusable so the
/// keyboard scrolls it).
///
/// Give it a size with `class` (for example Tailwind's `h-72 w-48`): the area fills the box
/// it is given and scrolls its content inside. `orientation` picks the directions (shadcn
/// has a vertical bar and adds a second `ScrollBar` for horizontal; here it is one prop).
/// `r#type` is when the bars show: `Hover` (default), `Scroll`, `Auto` (whenever the
/// content overflows) or `Always`.
#[component]
pub fn ScrollArea(
    #[prop(optional)] orientation: ScrollAreaOrientation,
    #[prop(optional)] r#type: ScrollAreaType,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <div
            class=format!("vdl-scroll-area {class}")
            data-orientation=orientation.as_str()
        >
            <primitive::ScrollArea r#type=r#type>
                <primitive::ScrollAreaViewport attr:class="vdl-scroll-area__viewport">
                    {children.with_value(|children| children())}
                </primitive::ScrollAreaViewport>
                {orientation
                    .has_vertical()
                    .then(|| {
                        view! {
                            <primitive::ScrollAreaScrollbar
                                orientation="vertical".to_string()
                                attr:class="vdl-scroll-area__scrollbar"
                            >
                                <primitive::ScrollAreaThumb attr:class="vdl-scroll-area__thumb" />
                            </primitive::ScrollAreaScrollbar>
                        }
                    })}
                {orientation
                    .has_horizontal()
                    .then(|| {
                        view! {
                            <primitive::ScrollAreaScrollbar
                                orientation="horizontal".to_string()
                                attr:class="vdl-scroll-area__scrollbar"
                            >
                                <primitive::ScrollAreaThumb attr:class="vdl-scroll-area__thumb" />
                            </primitive::ScrollAreaScrollbar>
                        }
                    })}
                {(orientation == ScrollAreaOrientation::Both)
                    .then(|| view! { <primitive::ScrollAreaCorner attr:class="vdl-scroll-area__corner" /> })}
            </primitive::ScrollArea>
        </div>
    }
}
