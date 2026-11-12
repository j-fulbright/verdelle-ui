use crate::primitives::progress as primitive;
use leptos::{context::Provider, prelude::*};

/// What the parts of a progress bar share.
#[derive(Clone)]
pub(crate) struct ProgressContext {
    pub value: Signal<Option<f64>>,
    pub max: Signal<f64>,
    /// Set by a [`crate::ProgressLabel`] so the bar can point `aria-labelledby` at it.
    pub has_label: RwSignal<bool>,
    pub label_id: String,
}

/// The fill of a bar as a percentage (0 to 100), or `None` when there is no value.
/// A `max` that is not positive counts as 100, as in the primitive.
pub(crate) fn percent(value: Option<f64>, max: f64) -> Option<f64> {
    let max = if max > 0.0 { max } else { 100.0 };
    value.map(|v| (v / max * 100.0).clamp(0.0, 100.0))
}

/// A progress bar, styled as shadcn/ui's Progress and built on the headless
/// Progress (`role="progressbar"` with `aria-valuenow`, `aria-valuetext` and a
/// `data-state` of `loading`, `complete` or `indeterminate`).
///
/// `value` runs from 0 to `max` (100 by default); leave it unset for an indeterminate bar,
/// which shows a sliding fill. The track and fill are drawn by the component; put a
/// [`crate::ProgressLabel`] and a [`crate::ProgressValue`] in the children to show a title
/// and the percentage above the bar. Without a label, give it `aria_label`.
#[component]
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let max_signal = Signal::derive(move || max.get().filter(|m| *m > 0.0).unwrap_or(100.0));
    let ctx = ProgressContext {
        value: Signal::derive(move || value.get()),
        max: max_signal,
        has_label: RwSignal::new(false),
        label_id: format!(
            "vdl-progress-label-{}",
            crate::primitives::core::id::use_id(None).get_untracked()
        ),
    };
    let labelled_by = {
        let ctx = ctx.clone();
        move || ctx.has_label.get().then(|| ctx.label_id.clone())
    };
    let children = StoredValue::new(children);

    view! {
        <Provider value=ctx.clone()>
        <primitive::Progress
            value=value
            max=max
            attr:class=format!("vdl-progress {class}")
            attr:aria-label=aria_label
            attr:aria-labelledby=labelled_by
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
            <div class="vdl-progress__track">
                <primitive::ProgressIndicator
                    attr:class="vdl-progress__indicator"
                    attr:style=move || {
                        percent(ctx.value.get(), ctx.max.get()).map(|p| format!("width:{p}%"))
                    }
                />
            </div>
        </primitive::Progress>
        </Provider>
    }
}

#[cfg(test)]
mod tests {
    use super::percent;

    #[test]
    fn percent_scales_to_max() {
        assert_eq!(percent(Some(25.0), 50.0), Some(50.0));
        assert_eq!(percent(Some(0.0), 100.0), Some(0.0));
    }

    #[test]
    fn percent_clamps_out_of_range_values() {
        assert_eq!(percent(Some(150.0), 100.0), Some(100.0));
        assert_eq!(percent(Some(-5.0), 100.0), Some(0.0));
    }

    #[test]
    fn percent_is_none_without_a_value_and_survives_a_zero_max() {
        assert_eq!(percent(None, 100.0), None);
        assert_eq!(percent(Some(40.0), 0.0), Some(40.0));
    }
}
