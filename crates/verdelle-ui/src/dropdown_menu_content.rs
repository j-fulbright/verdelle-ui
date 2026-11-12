use crate::primitives::dropdown_menu as primitive;
use crate::{Align, Side};
use leptos::prelude::*;

/// The menu panel of a [`crate::DropdownMenu`], shown in a portal at the end of `<body>`.
///
/// It opens on `side` of the trigger (default bottom), `side_offset` pixels away (default
/// 4), lined up by `align` (default start) and shifted `align_offset` along that axis.
/// With `avoid_collisions` (on by default) it flips and slides to stay inside the
/// viewport, keeping `collision_padding` pixels clear; a menu taller than the space
/// available scrolls.
///
/// It is as wide as its widest item, never narrower than the trigger or 128px. Set
/// `width` (pixels) for a fixed width. Arrow keys move through the items (wrapping), Home
/// and End jump to the ends, typing a letter jumps to the next item starting with it, Tab
/// is ignored and Escape closes the menu and returns focus to the trigger.
#[component]
pub fn DropdownMenuContent(
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
    /// Width in pixels. By default the menu fits its content.
    #[prop(optional)]
    width: Option<u32>,
    /// Called when Escape is pressed. Call `prevent_default` on the event to keep it open.
    #[prop(into, optional)]
    on_escape_key_down: Option<Callback<leptos::web_sys::KeyboardEvent>>,
    /// Called on a press outside the menu. Call `prevent_default` to keep it open.
    #[prop(into, optional)]
    on_pointer_down_outside: Option<Callback<leptos::web_sys::PointerEvent>>,
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
        MaybeProp::derive(move || Some(align.get().unwrap_or(Align::Start).as_str().to_string()));
    let side_offset = MaybeProp::derive(move || Some(side_offset.get().unwrap_or(4.0)));

    view! {
        <primitive::DropdownMenuPortal>
            <primitive::DropdownMenuContent
                side=side
                align=align
                side_offset=side_offset
                align_offset=align_offset
                avoid_collisions=avoid_collisions
                collision_padding=collision_padding
                on_escape_key_down=Callback::new(move |e| {
                    if let Some(cb) = on_escape_key_down {
                        cb.run(e);
                    }
                })
                on_pointer_down_outside=Callback::new(move |e| {
                    if let Some(cb) = on_pointer_down_outside {
                        cb.run(e);
                    }
                })
                class=format!("vdl-menu {class}")
                style=width.map(|w| format!("--vdl-menu-width:{w}px"))
            >
                {children.with_value(|children| children())}
            </primitive::DropdownMenuContent>
        </primitive::DropdownMenuPortal>
    }
}
