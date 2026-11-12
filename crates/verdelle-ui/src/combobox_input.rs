use crate::primitives::combobox as primitive;
use crate::{InputGroupAddon, InputGroupAlign, icons};
use leptos::prelude::*;

/// The text field of a [`crate::Combobox`], framed like an [`crate::InputGroup`] with a chevron
/// button at the end that opens and closes the list. The frame is the anchor the list is placed
/// under and as wide as.
///
/// `show_trigger` (default true) shows the chevron; `show_clear` shows a clear button once there
/// is a value or text (it takes the chevron's place, as shadcn's does). `invalid` draws the
/// destructive border and sets `aria-invalid`.
#[component]
pub fn ComboboxInput(
    /// Text shown while there is no value.
    #[prop(into, optional)]
    placeholder: Option<String>,
    #[prop(default = true)] show_trigger: bool,
    #[prop(optional)] show_clear: bool,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let (placeholder, aria_label, id) = (
        StoredValue::new(placeholder),
        StoredValue::new(aria_label),
        StoredValue::new(id),
    );
    view! {
        <primitive::ComboboxAnchor
            attr:class=format!("vdl-input-group vdl-combobox__input-group {class}")
            attr:role="group"
            attr:data-invalid=move || invalid.get().then_some("")
        >
            <primitive::ComboboxInput
                attr:class="vdl-input-group__control"
                attr:placeholder=placeholder.get_value()
                attr:aria-label=aria_label.get_value()
                attr:id=id.get_value()
                attr:aria-invalid=move || invalid.get().then_some("true")
            />
            <InputGroupAddon align=InputGroupAlign::InlineEnd>
                {show_trigger
                    .then(|| {
                        view! {
                            <primitive::ComboboxTrigger
                                attr:class="vdl-btn vdl-input-group__button vdl-combobox__trigger"
                                attr:data-variant="ghost"
                                attr:data-size="xs"
                                attr:data-icon-only=""
                                attr:data-ig-size="icon-xs"
                            >
                                <span class="vdl-btn__content">{icons::chevron_down()}</span>
                            </primitive::ComboboxTrigger>
                        }
                    })}
                {show_clear
                    .then(|| {
                        view! {
                            <primitive::ComboboxClear
                                attr:class="vdl-btn vdl-input-group__button vdl-combobox__clear"
                                attr:data-variant="ghost"
                                attr:data-size="xs"
                                attr:data-icon-only=""
                                attr:data-ig-size="icon-xs"
                            >
                                <span class="vdl-btn__content">{icons::x()}</span>
                            </primitive::ComboboxClear>
                        }
                    })}
            </InputGroupAddon>
        </primitive::ComboboxAnchor>
    }
}
