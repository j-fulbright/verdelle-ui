use crate::primitives::menubar as primitive;
use leptos::prelude::*;

/// The nested panel of a [`crate::MenubarSub`]. It opens to the right of its trigger
/// (to the left when there is no room), top-aligned with it, and takes the same items as
/// [`crate::MenubarContent`]. The pointer can cross to it diagonally without closing it.
#[component]
pub fn MenubarSubContent(
    /// Pixels between the trigger and the panel. Defaults to 0.
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// Pixels along the trigger's edge. Defaults to -3, which lines the first item up with the trigger.
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let side_offset = MaybeProp::derive(move || Some(side_offset.get().unwrap_or(0.0)));
    let align_offset = MaybeProp::derive(move || Some(align_offset.get().unwrap_or(-3.0)));

    view! {
        <primitive::MenubarSubContent
            side_offset=side_offset
            align_offset=align_offset
            class=format!("vdl-menu vdl-menu--sub {class}")
        >
            {children.with_value(|children| children())}
        </primitive::MenubarSubContent>
    }
}
