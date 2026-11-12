use crate::primitives::popover as primitive;
use crate::{Align, Side};
use leptos::prelude::*;

/// Wrap an optional callback as the always-present callback the primitives take.
fn forward<E: 'static>(cb: Option<Callback<E>>) -> Callback<E> {
    Callback::new(move |event: E| {
        if let Some(cb) = cb {
            cb.run(event);
        }
    })
}

/// The panel of a [`crate::Popover`], shown in a portal at the end of `<body>`.
///
/// It opens on `side` of the trigger (default bottom), `side_offset` pixels away
/// (default 4), lined up by `align` (default centre) and shifted `align_offset` along
/// that axis. With `avoid_collisions` (on by default) it flips to the other side and
/// slides when it would run off the viewport, keeping `collision_padding` pixels clear.
///
/// `label` is the accessible name of the panel (it has `role="dialog"`). The panel is
/// 288px wide unless `width` (pixels) is set. Focus moves to the first focusable
/// element inside when it opens, and back to the trigger when it closes.
#[component]
pub fn PopoverContent(
    /// Preferred side of the trigger; it flips to the opposite side when there is no room. The default depends on the component.
    #[prop(into, optional)]
    side: MaybeProp<Side>,
    /// How the panel lines up with the trigger along that side: `Start`, `Center` or `End`.
    #[prop(into, optional)]
    align: MaybeProp<Align>,
    /// Distance in pixels between the trigger and the panel.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// Shifts the panel this many pixels along the alignment axis.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// Flip and slide to stay inside the viewport (on by default).
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// Pixels to keep clear of the viewport edge when avoiding collisions.
    #[prop(into, optional)]
    collision_padding: MaybeProp<f64>,
    /// Width in pixels. Defaults to 288.
    #[prop(optional)]
    width: Option<u32>,
    /// Accessible name.
    #[prop(optional, into)]
    label: Option<String>,
    /// Called when Escape is pressed. Call `prevent_default` on the event to keep it open.
    #[prop(into, optional)]
    on_escape_key_down: Option<Callback<leptos::web_sys::KeyboardEvent>>,
    /// Called on a press outside the panel. Call `prevent_default` to keep it open.
    #[prop(into, optional)]
    on_pointer_down_outside: Option<Callback<leptos::web_sys::PointerEvent>>,
    /// Called when focus moves into the panel after it opens; `prevent_default` stops that.
    #[prop(into, optional)]
    on_open_auto_focus: Option<Callback<leptos::web_sys::Event>>,
    /// Called when focus returns to the trigger after it closes; `prevent_default` stops that.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<leptos::web_sys::Event>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let side =
        MaybeProp::derive(move || Some(side.get().unwrap_or(Side::Bottom).as_str().to_string()));
    let align =
        MaybeProp::derive(move || Some(align.get().unwrap_or_default().as_str().to_string()));
    let side_offset = MaybeProp::derive(move || Some(side_offset.get().unwrap_or(4.0)));

    view! {
        <primitive::PopoverPortal>
            <primitive::PopoverContent
                side=side
                align=align
                side_offset=side_offset
                align_offset=align_offset
                avoid_collisions=avoid_collisions
                collision_padding=collision_padding
                on_escape_key_down=forward(on_escape_key_down)
                on_pointer_down_outside=forward(on_pointer_down_outside)
                on_open_auto_focus=forward(on_open_auto_focus)
                on_close_auto_focus=forward(on_close_auto_focus)
                class=format!("vdl-popover {class}")
                style=width.map(|w| format!("--vdl-popover-width:{w}px"))
                attr:aria-label=label.clone()
            >
                {children.with_value(|children| children())}
            </primitive::PopoverContent>
        </primitive::PopoverPortal>
    }
}
