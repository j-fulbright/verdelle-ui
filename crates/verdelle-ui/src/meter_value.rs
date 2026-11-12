use crate::meter::MeterContext;
use leptos::prelude::*;

/// The meter's reading as text, at the end of the row opposite the [`crate::MeterLabel`].
/// Pass `text` to format it (for example `"{}%"`); by default it shows the number. It is hidden
/// from assistive technology: the meter already announces its value.
#[component]
pub fn MeterValue(
    #[prop(optional, into)] text: Option<Callback<f64, String>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let ctx = expect_context::<MeterContext>();
    view! {
        <span class=format!("vdl-meter__value {class}") aria-hidden="true">
            {move || {
                let v = ctx.value.get();
                match text {
                    Some(f) => f.run(v),
                    None => v.to_string(),
                }
            }}
        </span>
    }
}
