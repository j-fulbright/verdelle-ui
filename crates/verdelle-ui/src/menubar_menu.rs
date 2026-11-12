use crate::primitives::menubar as primitive;
use leptos::prelude::*;

/// One menu of a [`crate::Menubar`]: holds a [`crate::MenubarTrigger`] and a
/// [`crate::MenubarContent`]. `value` names the menu in the bar's `value`; give one when
/// the bar is controlled, otherwise it is generated.
#[component]
pub fn MenubarMenu(
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::MenubarMenu value=value>
            {children.with_value(|children| children())}
        </primitive::MenubarMenu>
    }
}
