use crate::primitives::toast as primitive;
use leptos::prelude::*;

/// A button on a [`crate::Toast`] for an action the user can take now (Undo, View, ...).
/// Clicking it runs `on_click` and then closes the toast.
///
/// `alt_text` is the accessible name: say what the action does, since the visible label
/// ("Undo") may be too short on its own.
#[component]
pub fn ToastAction(
    #[prop(into)] alt_text: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::ToastAction
            alt_text=alt_text
            attr:class=format!("vdl-toast__action {class}")
            on:click=move |_| {
                if let Some(on_click) = on_click {
                    on_click.run(());
                }
            }
        >
            {children.with_value(|children| children())}
        </primitive::ToastAction>
    }
}
