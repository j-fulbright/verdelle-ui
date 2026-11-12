use crate::{Icon, IconName, StatTone, StatTrend};
use leptos::prelude::*;

/// A tile that shows one number: a label, a large value, an optional change ("delta") with an
/// arrow, an optional icon and an optional footer line. It uses [`crate::Card`]'s surface and
/// tokens. A Verdelle extra (shadcn's dashboard blocks build this from Card each time; see
/// `NAMING.md`).
///
/// `label`, `value`, `delta`, `trend`, `tone` and `icon` take plain values or signals, so a tile can
/// follow loaded data; `value` is a string, so format it yourself ("1,284", "$12.4k").
/// The delta's arrow follows `trend` and its colour follows `tone` (default: up is green, down is
/// red); set `tone` when the sign and the news disagree, as for a falling error rate.
///
/// Pass `pressed` (a two-way `RwSignal<bool>`) to make the whole tile a toggle button with
/// `aria-pressed`, for filter tiles ("Open: 12", "Overdue: 3"); a click flips it and calls
/// `on_toggle` with the new state. Without `pressed` it is a plain, non-interactive block. Whether
/// it is a button is decided when the component is built.
#[component]
pub fn StatCard(
    /// What the number measures.
    #[prop(into)]
    label: Signal<String>,
    /// The number, already formatted.
    #[prop(into)]
    value: Signal<String>,
    /// The change since the last period, already formatted ("+12.5%"). Left out when `None`.
    #[prop(optional, into)]
    delta: MaybeProp<String>,
    /// Which arrow the delta shows.
    #[prop(optional, into)]
    trend: Signal<StatTrend>,
    /// Whether the delta is good or bad news; the default follows `trend`.
    #[prop(optional, into)]
    tone: Signal<StatTone>,
    /// An icon at the end of the label row. Takes an `IconName`, an `Option` or a signal of one.
    #[prop(optional, into)]
    icon: MaybeProp<IconName>,
    /// A line under the value and delta, in muted text ("vs. last month").
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Makes the tile a toggle button; see the component docs.
    #[prop(optional)]
    pressed: Option<RwSignal<bool>>,
    /// Called with the new state after a click on a toggle tile.
    #[prop(optional, into)]
    on_toggle: Option<Callback<bool>>,
    /// Uses 12px instead of 16px padding, as [`crate::Card`]'s `small`.
    #[prop(optional)]
    small: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let class = format!("vdl-card vdl-stat {class}");
    let size = small.then_some("sm");
    let body = move || {
        view! {
            <span class="vdl-stat__head">
                <span class="vdl-stat__label">{move || label.get()}</span>
                {move || {
                    icon.get().map(|name| view! { <Icon name=name class="vdl-stat__icon" /> })
                }}
            </span>
            <span class="vdl-stat__value">{move || value.get()}</span>
            {move || {
                delta
                    .get()
                    .filter(|d| !d.is_empty())
                    .map(|text| {
                        let direction = trend.get();
                        let icon = match direction {
                            StatTrend::Up => IconName::TrendingUp,
                            StatTrend::Down => IconName::TrendingDown,
                            StatTrend::Neutral => IconName::Minus,
                        };
                        let word = match direction {
                            StatTrend::Up => "Up ",
                            StatTrend::Down => "Down ",
                            StatTrend::Neutral => "",
                        };
                        view! {
                            <span
                                class="vdl-stat__delta"
                                data-trend=direction.as_str()
                                data-tone=tone.get().resolve(direction).as_str()
                            >
                                <Icon name=icon />
                                <span class="vdl-sr-only">{word}</span>
                                {text}
                            </span>
                        }
                    })
            }}
            {footer.as_ref().map(|f| view! { <span class="vdl-stat__footer">{f.run()}</span> })}
        }
    };

    match pressed {
        Some(pressed) => view! {
            <button
                type="button"
                class=class
                data-size=size
                data-pressed=move || pressed.get().then_some("")
                aria-pressed=move || if pressed.get() { "true" } else { "false" }
                on:click=move |_| {
                    let next = !pressed.get_untracked();
                    pressed.set(next);
                    if let Some(cb) = on_toggle {
                        cb.run(next);
                    }
                }
            >
                {body()}
            </button>
        }
        .into_any(),
        None => view! {
            <div class=class data-size=size>
                {body()}
            </div>
        }
        .into_any(),
    }
}
