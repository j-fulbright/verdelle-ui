use crate::progress::ProgressContext;
use leptos::prelude::*;

/// The progress bar's title. It names the bar (`aria-labelledby`), so a bar with a label
/// needs no `aria_label`.
#[component]
pub fn ProgressLabel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<ProgressContext>();
    // Post-mount: the bar's attributes are written before its children exist on the server.
    Effect::new(move |_| ctx.has_label.set(true));
    view! { <span id=ctx.label_id.clone() class=format!("vdl-progress__label {class}")>{children()}</span> }
}
