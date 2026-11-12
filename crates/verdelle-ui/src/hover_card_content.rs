use crate::primitives::hover_card as primitive;
use crate::{Align, Side};
use leptos::prelude::*;

/// The panel of a [`crate::HoverCard`], shown in a portal at the end of `<body>`.
///
/// It opens on `side` of the trigger (default bottom), `side_offset` pixels away (default
/// 4), lined up by `align` (default centre) and shifted `align_offset` along that axis. It
/// flips to the other side and slides when it would run off the viewport
/// (`avoid_collisions`, on by default, keeping `collision_padding` pixels clear). It is
/// 256px wide unless `width` (pixels) is set. Escape or a press outside closes it.
#[component]
pub fn HoverCardContent(
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
    /// Width in pixels. Defaults to 256.
    #[prop(optional)]
    width: Option<u32>,
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
        <primitive::HoverCardPortal>
            <primitive::HoverCardContent
                side=side
                align=align
                side_offset=side_offset
                align_offset=align_offset
                avoid_collisions=avoid_collisions
                collision_padding=collision_padding
                class=format!("vdl-hover-card {class}")
                style=width.map(|w| format!("--vdl-hover-card-width:{w}px"))
            >
                {children.with_value(|children| children())}
            </primitive::HoverCardContent>
        </primitive::HoverCardPortal>
    }
}
