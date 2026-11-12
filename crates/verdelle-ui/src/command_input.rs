use crate::primitives::command as primitive;
use crate::{InputGroupAddon, InputGroupAlign, icons};
use leptos::prelude::*;

/// The search box of a [`crate::Command`]: a compact framed input with a magnifier at the
/// start, as shadcn's.
#[component]
pub fn CommandInput(
    /// Text shown while there is no value.
    #[prop(into, optional)]
    placeholder: MaybeProp<String>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <div class="vdl-command__input-wrapper">
            <div class="vdl-input-group vdl-command__input-group" role="group">
                <InputGroupAddon align=InputGroupAlign::InlineStart>
                    <span class="vdl-command__input-icon">{icons::search()}</span>
                </InputGroupAddon>
                <primitive::CommandInput
                    placeholder=placeholder
                    disabled=disabled
                    attr:class=format!("vdl-input-group__control vdl-command__input {class}")
                    attr:aria-label=aria_label
                />
            </div>
        </div>
    }
}
