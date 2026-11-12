use crate::Size;
use crate::primitives::number_field as primitive;
use leptos::{context::Provider, prelude::*};

/// What a [`NumberField`] hands its parts.
#[derive(Clone, Copy)]
pub(crate) struct NumberFieldStyle {
    pub invalid: Signal<bool>,
}

/// A numeric input with increment and decrement buttons, in the Input Group look, built on the
/// headless Number Field (the input is a `role="spinbutton"`). shadcn has no number
/// field; this follows Base UI's.
///
/// Compose it from a [`crate::NumberFieldGroup`] holding a [`crate::NumberFieldDecrement`], a
/// [`crate::NumberFieldInput`] and a [`crate::NumberFieldIncrement`]; an optional
/// [`crate::NumberFieldScrubArea`] (for example around the label) changes the value by dragging.
///
/// The value is a number or empty (`None`). Pass `value` (a signal that is read and written) to
/// control it, or `default_value` to start somewhere; `on_value_change` fires on every change,
/// `on_value_commit` when a value is settled (blur, Enter, a step). `step` (1) is the arrow and
/// button step; Alt uses `small_step` (0.1), and Shift, PageUp and PageDown `large_step` (10).
/// `min` and `max` clamp stepping and committed values; `decimals` shows and rounds to a fixed
/// number of places; `prefix` and `suffix` are shown around the number. With `snap_on_step`, a
/// step lands on a multiple of `step` counted from `min`. `name` submits the numeric value with
/// a form. `invalid` draws the destructive border and ring.
#[component]
pub fn NumberField(
    #[prop(optional)] value: Option<RwSignal<Option<f64>>>,
    #[prop(into, optional)] default_value: MaybeProp<f64>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<f64>>>,
    #[prop(into, optional)] on_value_commit: Option<Callback<Option<f64>>>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional, default = 1.0.into())] step: MaybeProp<f64>,
    #[prop(into, optional, default = 0.1.into())] small_step: MaybeProp<f64>,
    #[prop(into, optional, default = 10.0.into())] large_step: MaybeProp<f64>,
    #[prop(into, optional)] snap_on_step: MaybeProp<bool>,
    #[prop(into, optional)] decimals: MaybeProp<usize>,
    #[prop(into, optional)] prefix: MaybeProp<String>,
    #[prop(into, optional)] suffix: MaybeProp<String>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    /// Form field name; the value is submitted under it.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Provider value=NumberFieldStyle { invalid }>
            <primitive::NumberField
                value=value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked()))
                default_value=default_value
                on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Option<f64>| {}))
                on_value_commit=on_value_commit.unwrap_or_else(|| Callback::new(|_: Option<f64>| {}))
                min=min
                max=max
                step=step
                small_step=small_step
                large_step=large_step
                snap_on_step=snap_on_step
                decimals=decimals
                prefix=prefix
                suffix=suffix
                disabled=disabled
                read_only=read_only
                name=name
                attr:class=format!("vdl-number-field {class}")
                attr:data-size=size.as_str()
                attr:data-invalid=move || invalid.get().then_some("")
            >
                {children.with_value(|children| children())}
            </primitive::NumberField>
        </Provider>
    }
}
