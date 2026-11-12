use crate::progress::{ProgressContext, percent};
use leptos::prelude::*;

/// The bar's value as text, for example "60%" (empty while indeterminate). Sits at the end
/// of the row, opposite the [`crate::ProgressLabel`]. It is hidden from assistive
/// technology: the bar already announces its value.
#[component]
pub fn ProgressValue(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<ProgressContext>();
    view! {
        <span class=format!("vdl-progress__value {class}") aria-hidden="true">
            {move || percent(ctx.value.get(), ctx.max.get()).map(|p| format!("{}%", p.round()))}
        </span>
    }
}
