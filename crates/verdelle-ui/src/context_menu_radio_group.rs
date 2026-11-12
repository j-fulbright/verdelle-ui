use crate::primitives::context_menu as primitive;
use leptos::prelude::*;

/// Groups [`crate::ContextMenuRadioItem`]s so that exactly one is chosen. `value` controls
/// the choice (pair with `on_value_change`); without it the group keeps its own state
/// starting from `default_value`.
#[component]
pub fn ContextMenuRadioGroup(
    /// Controlled value. Leave unset to let it manage itself (see `default_value`).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: Option<String>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let own = RwSignal::new(default_value);
    let current = Signal::derive(move || value.get().or_else(|| own.get()));

    view! {
        <primitive::ContextMenuRadioGroup
            value=current
            on_value_change=Callback::new(move |next: String| {
                own.set(Some(next.clone()));
                if let Some(cb) = on_value_change {
                    cb.run(next);
                }
            })
            attr:class=format!("vdl-menu__group {class}")
        >
            {children.with_value(|children| children())}
        </primitive::ContextMenuRadioGroup>
    }
}
