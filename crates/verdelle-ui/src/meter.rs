use leptos::{context::Provider, prelude::*};

/// What the parts of a meter share.
#[derive(Clone)]
pub(crate) struct MeterContext {
    pub value: Signal<f64>,
    /// Set by a [`crate::MeterLabel`] so the meter can point `aria-labelledby` at it.
    pub has_label: RwSignal<bool>,
    pub label_id: String,
}

/// How good the value is, as the HTML `<meter>` element works it out from `low`, `high` and
/// `optimum`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterState {
    /// The value is in the same region as the optimum.
    Optimum,
    /// One region away from the optimum.
    Suboptimal,
    /// The far side of the middle from the optimum.
    EvenLessGood,
}

impl MeterState {
    pub fn as_str(self) -> &'static str {
        match self {
            MeterState::Optimum => "optimum",
            MeterState::Suboptimal => "suboptimal",
            MeterState::EvenLessGood => "even-less-good",
        }
    }
}

/// The fill of the meter as a percentage (0 to 100). A range with no width counts as empty.
pub(crate) fn percent(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
}

#[derive(PartialEq)]
enum Region {
    Low,
    Middle,
    High,
}

/// The state of `value` in `min..=max` with the thresholds `low`, `high` and `optimum`
/// (defaults: `low` = `min`, `high` = `max`, `optimum` = the midpoint), following the HTML
/// meter rules: with no thresholds every value is optimum.
pub fn meter_state(
    value: f64,
    min: f64,
    max: f64,
    low: Option<f64>,
    high: Option<f64>,
    optimum: Option<f64>,
) -> MeterState {
    let max = max.max(min);
    let value = value.clamp(min, max);
    let low = low.unwrap_or(min).clamp(min, max);
    let high = high.unwrap_or(max).clamp(low, max);
    let optimum = optimum.unwrap_or((min + max) / 2.0).clamp(min, max);
    let region = |x: f64| {
        if x < low {
            Region::Low
        } else if x > high {
            Region::High
        } else {
            Region::Middle
        }
    };
    let (best, actual) = (region(optimum), region(value));
    if best == actual {
        MeterState::Optimum
    } else if best == Region::Middle || actual == Region::Middle {
        MeterState::Suboptimal
    } else {
        MeterState::EvenLessGood
    }
}

/// A gauge for a value within a known range (disk usage, a score), as Base UI's Meter and the
/// HTML `<meter>`: `role="meter"` with `aria-valuenow`, `aria-valuemin` and `aria-valuemax`.
/// Unlike [`crate::Progress`] it shows a measurement, not the advance of a task.
///
/// `value` runs from `min` (0) to `max` (100). `low`, `high` and `optimum` colour the fill:
/// in the optimum region it is the primary colour, one region away it is muted, and on the
/// far side it is destructive (`data-state` is `optimum`, `suboptimal` or `even-less-good`).
/// Without thresholds the fill is always the primary colour. `value_text` turns the value
/// into the text read out and shown by [`crate::MeterValue`] (default: the number). Put a
/// [`crate::MeterLabel`] and a [`crate::MeterValue`] in the children for a title and the
/// reading above the bar; without a label give it `aria_label`.
#[component]
pub fn Meter(
    #[prop(into)] value: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] min: Signal<f64>,
    #[prop(into, optional, default = 100.0.into())] max: Signal<f64>,
    #[prop(into, optional)] low: MaybeProp<f64>,
    #[prop(into, optional)] high: MaybeProp<f64>,
    #[prop(into, optional)] optimum: MaybeProp<f64>,
    #[prop(optional, into)] value_text: Option<Callback<f64, String>>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let ctx = MeterContext {
        value,
        has_label: RwSignal::new(false),
        label_id: format!(
            "vdl-meter-label-{}",
            crate::primitives::core::id::use_id(None).get_untracked()
        ),
    };
    let text = move || match value_text {
        Some(f) => f.run(value.get()),
        None => value.get().to_string(),
    };
    let state = move || {
        meter_state(
            value.get(),
            min.get(),
            max.get(),
            low.get(),
            high.get(),
            optimum.get(),
        )
    };
    let labelled_by = {
        let ctx = ctx.clone();
        move || ctx.has_label.get().then(|| ctx.label_id.clone())
    };
    let children = StoredValue::new(children);
    let fill = move || percent(value.get(), min.get(), max.get());

    view! {
        <Provider value=ctx.clone()>
            <div
                class=format!("vdl-meter {class}")
                role="meter"
                aria-valuenow=move || value.get().to_string()
                aria-valuemin=move || min.get().to_string()
                aria-valuemax=move || max.get().to_string()
                aria-valuetext=text
                aria-label=aria_label
                aria-labelledby=labelled_by
                data-state=move || state().as_str()
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
                <div class="vdl-meter__track">
                    <div class="vdl-meter__indicator" style=move || format!("width:{}%", fill())></div>
                </div>
            </div>
        </Provider>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_thresholds_every_value_is_optimum() {
        for v in [0.0, 40.0, 100.0] {
            assert_eq!(
                meter_state(v, 0.0, 100.0, None, None, None),
                MeterState::Optimum
            );
        }
    }

    #[test]
    fn a_high_optimum_makes_a_low_value_bad() {
        // optimum at the top, "low" below 30 and "high" above 70
        let s = |v| meter_state(v, 0.0, 100.0, Some(30.0), Some(70.0), Some(90.0));
        assert_eq!(s(95.0), MeterState::Optimum);
        assert_eq!(s(50.0), MeterState::Suboptimal);
        assert_eq!(s(10.0), MeterState::EvenLessGood);
    }

    #[test]
    fn a_low_optimum_makes_a_high_value_bad() {
        // disk usage: low is good
        let s = |v| meter_state(v, 0.0, 100.0, Some(60.0), Some(85.0), Some(10.0));
        assert_eq!(s(20.0), MeterState::Optimum);
        assert_eq!(s(70.0), MeterState::Suboptimal);
        assert_eq!(s(95.0), MeterState::EvenLessGood);
    }

    #[test]
    fn a_middle_optimum_makes_both_ends_suboptimal() {
        let s = |v| meter_state(v, 0.0, 100.0, Some(30.0), Some(70.0), Some(50.0));
        assert_eq!(s(50.0), MeterState::Optimum);
        assert_eq!(s(10.0), MeterState::Suboptimal);
        assert_eq!(s(90.0), MeterState::Suboptimal);
    }

    #[test]
    fn values_and_thresholds_outside_the_range_are_clamped() {
        assert_eq!(
            meter_state(500.0, 0.0, 100.0, Some(-5.0), Some(500.0), Some(10.0)),
            MeterState::Optimum
        );
    }

    #[test]
    fn percent_is_scaled_and_clamped() {
        assert_eq!(percent(25.0, 0.0, 50.0), 50.0);
        assert_eq!(percent(15.0, 10.0, 20.0), 50.0);
        assert_eq!(percent(150.0, 0.0, 100.0), 100.0);
        assert_eq!(percent(-5.0, 0.0, 100.0), 0.0);
        assert_eq!(percent(5.0, 10.0, 10.0), 0.0);
    }
}
