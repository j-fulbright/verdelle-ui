use crate::{Size, icons};
use leptos::{html, prelude::*};

/// The browser's own `<select>`, styled as shadcn/ui's Native Select: the same frame as
/// [`crate::Input`], a chevron, and the platform's picker (so it works with a screen reader,
/// on mobile and in forms with no script). Fill it with [`crate::NativeSelectOption`] and
/// [`crate::NativeSelectOptGroup`].
///
/// `size` `Small` is 28px; every other size is the 32px default (shadcn has two sizes).
/// Pass `value` for a two-way binding, or leave it out and mark an option `selected`.
/// `invalid` sets `aria-invalid` and the destructive border. The width is that of the widest
/// option; use `class` (for example `w-full`) to change it.
#[component]
pub fn NativeSelect(
    #[prop(optional)] value: Option<RwSignal<String>>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Form field name; the value is submitted under it.
    #[prop(optional, into)]
    name: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: Children,
) -> impl IntoView {
    let data_size = move || {
        if size.get() == Size::Small {
            "sm"
        } else {
            "default"
        }
    };
    let field = NodeRef::<html::Select>::new();
    // Push the signal into the element once it exists (runs in the browser only). Setting the
    // `value` property in the view would clear the choice when there is no signal.
    Effect::new(move |_| {
        if let (Some(v), Some(el)) = (value, field.get()) {
            el.set_value(&v.get());
        }
    });
    view! {
        <div class=format!("vdl-native-select {class}") data-size=data_size>
            <select
                class="vdl-native-select__field"
                data-size=data_size
                id=id
                name=name
                aria-label=aria_label
                aria-invalid=move || invalid.get().then_some("true")
                disabled=move || disabled.get()
                node_ref=field
                on:change=move |ev| {
                    if let Some(v) = value {
                        v.set(event_target_value(&ev));
                    }
                }
            >
                {children()}
            </select>
            <span class="vdl-native-select__icon" aria-hidden="true">
                {icons::chevron_down()}
            </span>
        </div>
    }
}
