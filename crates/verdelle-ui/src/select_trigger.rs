use crate::Size;
use crate::primitives::select as primitive;
use crate::{Icon, IconName};
use leptos::prelude::*;

/// The button that opens the list. Put a [`crate::SelectValue`] inside; the chevron is added,
/// and, on a `clearable` select, an X that clears the value.
///
/// `size` is `data-size` (xs 24px, sm 28px, md 32px default, lg 36px). `invalid` sets
/// `aria-invalid` for the error ring.
#[component]
pub fn SelectTrigger(
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(into, optional)]
    invalid: MaybeProp<bool>,
    /// The accessible name, when no visible label points at the trigger.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    /// The element's `id`.
    #[prop(into, optional)]
    id: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::SelectTrigger
            attr:class=format!("vdl-select__trigger {class}")
            attr:data-size=move || size.get().as_str()
            attr:aria-label=move || aria_label.get()
            attr:id=move || id.get()
            attr:aria-invalid=move || invalid.get().unwrap_or(false).then_some("true")
        >
            {children.with_value(|children| children())}
            <primitive::SelectClear class="vdl-select__clear">
                <Icon name=IconName::X />
            </primitive::SelectClear>
            <Icon name=IconName::ChevronDown class="vdl-select__icon" />
        </primitive::SelectTrigger>
    }
}
