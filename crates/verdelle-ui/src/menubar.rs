use crate::primitives::menubar as primitive;
use leptos::prelude::*;

/// A menubar, styled as shadcn/ui's Menubar: a horizontal bar of menus, like the menu bar
/// of a desktop application. Built on the headless Menubar, which shares the
/// dropdown menu's behaviour and adds the bar's: one tab stop, Left and Right arrows moving
/// between menus (and between open menus), and hover switching once a menu is open.
///
/// Compose with [`crate::MenubarMenu`], each holding a [`crate::MenubarTrigger`] and a
/// [`crate::MenubarContent`]; the content takes [`crate::MenubarItem`],
/// [`crate::MenubarCheckboxItem`], [`crate::MenubarRadioGroup`] with
/// [`crate::MenubarRadioItem`], [`crate::MenubarLabel`], [`crate::MenubarSeparator`],
/// [`crate::MenubarGroup`], [`crate::MenubarShortcut`] and [`crate::MenubarSub`] with
/// [`crate::MenubarSubTrigger`] and [`crate::MenubarSubContent`].
///
/// Uncontrolled by default; pass `value` (the `value` of the open menu, `None` for all
/// closed) with `on_value_change` to control which menu is open.
#[component]
pub fn Menubar(
    /// The open menu's `value`, or `None` for none. Give it to control the bar.
    #[prop(into, optional)]
    value: Option<Signal<Option<String>>>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with the `value` of the menu that opened, or `None` when the open one closed.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<String>>>,
    /// Whether Left/Right wrap around at the ends. On by default.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let own = RwSignal::new(default_value.get_untracked());
    let current = Signal::derive(move || match value {
        Some(controlled) => controlled.get(),
        None => own.get(),
    });

    view! {
        <primitive::Menubar
            value=current
            on_value_change=Callback::new(move |v: Option<String>| {
                if value.is_none() {
                    own.set(v.clone());
                }
                if let Some(cb) = on_value_change {
                    cb.run(v);
                }
            })
            r#loop=r#loop
            attr:class=format!("vdl-menubar {class}")
        >
            {children.with_value(|children| children())}
        </primitive::Menubar>
    }
}
