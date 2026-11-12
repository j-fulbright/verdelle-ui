use crate::primitives::context_menu as primitive;
use leptos::prelude::*;

/// A context menu, styled as shadcn/ui's Context Menu: a list of actions that opens at the
/// pointer when you right-click (or press and hold on a touch screen) inside its trigger
/// area. Built on the headless ContextMenu, which shares the dropdown menu's
/// behaviour: arrow-key / Home / End navigation, typeahead, sub menus, dismissal and focus.
///
/// Compose with [`crate::ContextMenuTrigger`] (the area) and [`crate::ContextMenuContent`],
/// and in the content: [`crate::ContextMenuItem`], [`crate::ContextMenuCheckboxItem`],
/// [`crate::ContextMenuRadioGroup`] with [`crate::ContextMenuRadioItem`],
/// [`crate::ContextMenuLabel`], [`crate::ContextMenuSeparator`], [`crate::ContextMenuGroup`],
/// [`crate::ContextMenuShortcut`] and [`crate::ContextMenuSub`] with
/// [`crate::ContextMenuSubTrigger`] and [`crate::ContextMenuSubContent`].
#[component]
pub fn ContextMenu(
    /// Called with the new state whenever the menu opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ContextMenu on_open_change=Callback::new(move |o: bool| {
            if let Some(cb) = on_open_change {
                cb.run(o);
            }
        })>{children.with_value(|children| children())}</primitive::ContextMenu>
    }
}
