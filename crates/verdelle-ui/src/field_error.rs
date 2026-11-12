use leptos::prelude::*;

/// An error message under a control, announced when it appears (`role="alert"`).
///
/// Pass `errors` (a list of messages) or children. Nothing is rendered while there are no
/// errors, so it can sit in the Field permanently. Duplicate messages are shown once; several
/// distinct ones become a bulleted list. Give it an `id` and point the control's
/// `aria_describedby` at it.
#[component]
pub fn FieldError(
    #[prop(optional, into)] errors: Signal<Vec<String>>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let unique = move || {
        let mut seen = Vec::new();
        for message in errors.get() {
            if !message.is_empty() && !seen.contains(&message) {
                seen.push(message);
            }
        }
        seen
    };

    move || {
        let has_children = children.with_value(|c| c.is_some());
        let messages = unique();
        if !has_children && messages.is_empty() {
            return None;
        }
        let body = if has_children {
            children.with_value(|c| c.as_ref().map(|c| c())).into_any()
        } else if messages.len() == 1 {
            messages[0].clone().into_any()
        } else {
            view! {
                <ul class="vdl-field-error__list">
                    {messages.into_iter().map(|m| view! { <li>{m}</li> }).collect_view()}
                </ul>
            }
            .into_any()
        };
        Some(view! {
            <div id=id.clone() role="alert" class=format!("vdl-field-error {class}")>
                {body}
            </div>
        })
    }
}
