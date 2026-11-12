use crate::primitives::context_menu as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// An item with a check, for an on/off option. `checked` controls it (pair with
/// `on_checked_change`); without `checked` it keeps its own state starting from
/// `default_checked`.
///
/// Toggling it does not close the menu unless `close_on_select` is set, so several options
/// can be changed in one visit. `disabled`, `inset` and `on_select` work as on
/// [`crate::ContextMenuItem`].
#[component]
pub fn ContextMenuCheckboxItem(
    /// Controlled checked state. Leave unset to let it manage itself (see `default_checked`).
    #[prop(into, optional)]
    checked: MaybeProp<bool>,
    /// Initial checked state when `checked` is not given.
    #[prop(into, optional)]
    default_checked: bool,
    /// Called with the new state whenever it changes.
    #[prop(into, optional)]
    on_checked_change: Option<Callback<bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Called when the item is chosen, by click or keyboard.
    #[prop(into, optional)]
    on_select: Option<Callback<()>>,
    /// Close the menu when this item is chosen. Off by default for checkbox and radio items.
    #[prop(into, optional)]
    close_on_select: MaybeProp<bool>,
    /// Indents it to line up with sibling items that have an icon or check indicator.
    #[prop(optional)]
    inset: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let own = RwSignal::new(default_checked);
    let is_checked = Signal::derive(move || checked.get().unwrap_or_else(|| own.get()));

    view! {
        <primitive::ContextMenuCheckboxItem
            checked=is_checked
            on_checked_change=Callback::new(move |next: bool| {
                own.set(next);
                if let Some(cb) = on_checked_change {
                    cb.run(next);
                }
            })
            disabled=disabled
            on_select=Callback::new(move |()| {
                if let Some(cb) = on_select {
                    cb.run(());
                }
            })
            close_on_select=close_on_select
            attr:class=format!("vdl-menu__item vdl-menu__item--check {class}")
            attr:data-inset=inset.then_some("")
        >
            <span class="vdl-menu__indicator">
                <primitive::ContextMenuItemIndicator>
                    <Icon name=IconName::Check />
                </primitive::ContextMenuItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </primitive::ContextMenuCheckboxItem>
    }
}
