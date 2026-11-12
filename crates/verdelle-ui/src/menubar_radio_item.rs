use crate::primitives::menubar as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// One choice in a [`crate::MenubarRadioGroup`], marked with a check when it is the
/// group's value. Choosing it does not close the menu unless `close_on_select` is set.
#[component]
pub fn MenubarRadioItem(
    #[prop(into)] value: String,
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

    view! {
        <primitive::MenubarRadioItem
            value=value
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
                <primitive::MenubarItemIndicator>
                    <Icon name=IconName::Check />
                </primitive::MenubarItemIndicator>
            </span>
            {children.with_value(|children| children())}
        </primitive::MenubarRadioItem>
    }
}
