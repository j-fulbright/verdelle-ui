use crate::Orientation;
use crate::primitives::slider as primitive;
use leptos::prelude::*;

/// A draggable value (or range) control, styled as shadcn/ui's Slider and built on the
/// headless Slider (each thumb is a `role="slider"` with `aria-valuenow`).
///
/// Values are a list, one per thumb: `default_value=vec![50.0]` is a single slider and
/// `vec![25.0, 75.0]` is a range with two thumbs (the thumb count is read once, from
/// `value` or `default_value`; without either there is one thumb at `min`). Uncontrolled
/// by default; pass `value` and `on_value_change` to control it. `on_value_commit` fires
/// when a drag ends or a key is pressed. Keyboard on a focused thumb: arrows move by
/// `step`, PageUp/PageDown by ten steps, Home/End to the ends. Thumbs cannot pass each
/// other. `orientation` vertical needs a height (the slider fills its parent's, at least
/// 160px). `thumb_labels` names the thumbs (for example "Minimum", "Maximum"); one label
/// is reused for all thumbs when only one is given.
#[component]
pub fn Slider(
    #[prop(into, optional)] value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<f64>>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] on_value_commit: Option<Callback<Vec<f64>>>,
    #[prop(into, optional, default = 0.0.into())] min: MaybeProp<f64>,
    #[prop(into, optional, default = 100.0.into())] max: MaybeProp<f64>,
    #[prop(into, optional, default = 1.0.into())] step: MaybeProp<f64>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(optional)] orientation: Orientation,
    /// Fill from the end instead of the start (and flip the keys to match).
    #[prop(into, optional)]
    inverted: MaybeProp<bool>,
    #[prop(optional, into)] thumb_labels: Vec<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
) -> impl IntoView {
    let thumbs = value
        .get_untracked()
        .or_else(|| default_value.get_untracked())
        .map(|v| v.len())
        .unwrap_or(1)
        .max(1);

    view! {
        <primitive::Slider
            value=value
            default_value=default_value
            on_value_change=on_value_change.unwrap_or_else(|| Callback::new(|_: Vec<f64>| {}))
            on_value_commit=on_value_commit.unwrap_or_else(|| Callback::new(|_: Vec<f64>| {}))
            min=min
            max=max
            step=step
            disabled=disabled
            orientation=orientation.to_string()
            inverted=inverted
            thumb_alignment="contain"
            attr:id=id
            attr:class=format!("vdl-slider {class}")
        >
            <primitive::SliderTrack attr:class="vdl-slider__track">
                <primitive::SliderRange attr:class="vdl-slider__range" />
            </primitive::SliderTrack>
            {(0..thumbs)
                .map(|index| {
                    let label = thumb_labels
                        .get(index)
                        .or(thumb_labels.first())
                        .cloned();
                    view! {
                        <primitive::SliderThumb
                            index=index
                            aria_label=label
                            class="vdl-slider__thumb"
                        />
                    }
                })
                .collect_view()}
        </primitive::Slider>
    }
}
