use crate::primitives::context_menu as primitive;
use leptos::prelude::*;

/// A nested menu inside a [`crate::ContextMenuContent`]: holds a
/// [`crate::ContextMenuSubTrigger`] (the item that opens it) and a
/// [`crate::ContextMenuSubContent`] (the nested panel). It opens on hover, click, Enter,
/// Space or the right arrow, and the left arrow closes it.
#[component]
pub fn ContextMenuSub(
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ContextMenuSub
            open=open
            default_open=default_open
            on_open_change=Callback::new(move |o: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(o);
                }
            })
        >
            {children.with_value(|children| children())}
        </primitive::ContextMenuSub>
    }
}
