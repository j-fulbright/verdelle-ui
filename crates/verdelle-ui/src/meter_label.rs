use crate::meter::MeterContext;
use leptos::prelude::*;

/// The meter's title. It names the meter (`aria-labelledby`), so a meter with a label needs no
/// `aria_label`.
#[component]
pub fn MeterLabel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<MeterContext>();
    // Post-mount: the meter's attributes are written before its children exist on the server.
    Effect::new(move |_| ctx.has_label.set(true));
    view! { <span id=ctx.label_id.clone() class=format!("vdl-meter__label {class}")>{children()}</span> }
}
