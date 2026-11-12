use crate::primitives::combobox as primitive;
use crate::{Align, Side};
use leptos::prelude::*;

/// The popup of a [`crate::Combobox`], shown in a portal under the input. It is as wide as the
/// input's frame, flips above when there is no room, and holds a [`crate::ComboboxList`] and a
/// [`crate::ComboboxEmpty`]. It opens on the bottom side, aligned to the start, 6px away.
#[component]
pub fn ComboboxContent(
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
    /// The popup of a chips combobox: no wider than its anchor (the chips row), where the
    /// input one is 28px wider than its frame, as shadcn's.
    #[prop(optional)]
    chips: bool,
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
        MaybeProp::derive(move || Some(align.get().map_or("start", |a| a.as_str()).to_string()));
    let side_offset = MaybeProp::derive(move || Some(side_offset.get().unwrap_or(6.0)));
    let collision_padding = MaybeProp::derive(move || Some(collision_padding.get().unwrap_or(8.0)));

    view! {
        <primitive::ComboboxPortal>
            <primitive::ComboboxContent
                side=side
                align=align
                side_offset=side_offset
                align_offset=align_offset
                avoid_collisions=avoid_collisions
                collision_padding=collision_padding
                class=format!("vdl-combobox__content {class}")
                style=chips.then_some("--vdl-combobox-extra:0px".to_string())
            >
                {children.with_value(|children| children())}
            </primitive::ComboboxContent>
        </primitive::ComboboxPortal>
    }
}
