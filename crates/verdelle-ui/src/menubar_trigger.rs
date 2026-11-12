use crate::primitives::menubar as primitive;
use leptos::prelude::*;

/// The label of a [`crate::MenubarMenu`] in the bar, a `<button>`. Click, Enter, Space or
/// Down opens the menu; Left and Right move to the neighbouring menu (opening it when one
/// is open); once a menu is open, hovering another trigger switches to it.
#[component]
pub fn MenubarTrigger(
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::MenubarTrigger
            disabled=disabled
            attr:class=format!("vdl-menubar__trigger {class}")
        >
            {children.with_value(|children| children())}
        </primitive::MenubarTrigger>
    }
}
