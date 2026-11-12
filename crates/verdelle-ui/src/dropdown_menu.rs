use crate::primitives::dropdown_menu as primitive;
use leptos::prelude::*;

/// A dropdown menu, styled as shadcn/ui's Dropdown Menu: a list of actions that opens
/// from a button. Built on the headless DropdownMenu, which provides the roles,
/// arrow-key / Home / End navigation, typeahead, sub menus, dismissal and focus handling.
///
/// Compose with [`crate::DropdownMenuTrigger`] and [`crate::DropdownMenuContent`], and in
/// the content: [`crate::DropdownMenuItem`], [`crate::DropdownMenuCheckboxItem`],
/// [`crate::DropdownMenuRadioGroup`] with [`crate::DropdownMenuRadioItem`],
/// [`crate::DropdownMenuLabel`], [`crate::DropdownMenuSeparator`],
/// [`crate::DropdownMenuGroup`], [`crate::DropdownMenuShortcut`] and
/// [`crate::DropdownMenuSub`] with [`crate::DropdownMenuSubTrigger`] and
/// [`crate::DropdownMenuSubContent`].
///
/// Uncontrolled by default; pass `open` (with `on_open_change`) to control it.
#[component]
pub fn DropdownMenu(
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever the menu opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::DropdownMenu
            open=open
            default_open=default_open
            on_open_change=Callback::new(move |o: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(o);
                }
            })
        >
            {children.with_value(|children| children())}
        </primitive::DropdownMenu>
    }
}
