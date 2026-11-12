use crate::primitives::select as primitive;
use leptos::prelude::*;

/// The popup list, shown under the trigger while the select is open. It is as wide as the
/// trigger (at least 144px), flips above when there is no room, and scrolls when long.
#[component]
pub fn SelectContent(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::SelectPortal>
            <primitive::SelectContent
                side_offset=4.0
                collision_padding=8.0
                class=format!("vdl-select__content {class}")
            >
                <primitive::SelectViewport attr:class="vdl-select__viewport">
                    {children.with_value(|children| children())}
                </primitive::SelectViewport>
            </primitive::SelectContent>
        </primitive::SelectPortal>
    }
}
