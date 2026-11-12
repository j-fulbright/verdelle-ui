use crate::InputGroupAlign;
use leptos::{prelude::*, web_sys};

/// An icon, text, key hint or button attached to an [`crate::InputGroup`]. Clicking it (but
/// not a button inside it) focuses the group's input, as shadcn's does.
#[component]
pub fn InputGroupAddon(
    #[prop(optional, into)] align: Signal<InputGroupAlign>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("vdl-input-group__addon {class}")
            role="group"
            data-align=move || align.get().as_str()
            on:click=move |ev| focus_control(&ev)
        >
            {children()}
        </div>
    }
}

fn focus_control(ev: &web_sys::MouseEvent) {
    use leptos::wasm_bindgen::JsCast;
    let in_button = ev
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| el.closest("button").ok().flatten())
        .is_some();
    if in_button {
        return;
    }
    let control = ev
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| el.parent_element())
        .and_then(|parent| parent.query_selector("input, textarea").ok().flatten())
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
    if let Some(control) = control {
        let _ = control.focus();
    }
}
