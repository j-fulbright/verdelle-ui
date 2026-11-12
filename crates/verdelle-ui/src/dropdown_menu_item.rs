use crate::MenuItemVariant;
use crate::primitives::dropdown_menu as primitive;
use leptos::prelude::*;

/// An action in a [`crate::DropdownMenuContent`]. Choosing it (click, Enter or Space) calls
/// `on_select` and closes the menu.
///
/// A `disabled` item is dimmed, skipped by the arrow keys and typeahead, and cannot be
/// chosen. `variant` picks the destructive look; `inset` indents the text to line up with
/// items that have a check or icon. Put a [`crate::DropdownMenuShortcut`] last for a key hint.
#[component]
pub fn DropdownMenuItem(
    #[prop(optional)] variant: MenuItemVariant,
    /// Indents it to line up with sibling items that have an icon or check indicator.
    #[prop(optional)]
    inset: bool,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Called when the item is chosen, by click or keyboard.
    #[prop(into, optional)]
    on_select: Option<Callback<()>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::DropdownMenuItem
            disabled=disabled
            on_select=Callback::new(move |()| {
                if let Some(cb) = on_select {
                    cb.run(());
                }
            })
            attr:class=format!("vdl-menu__item {class}")
            attr:data-variant=variant.as_str()
            attr:data-inset=inset.then_some("")
        >
            {children.with_value(|children| children())}
        </primitive::DropdownMenuItem>
    }
}
