use crate::calendar_state::use_synced;
use crate::date_picker::{date_field, focus_tab_day};
use crate::{
    Align, ButtonVariant, Calendar, CalendarDate, CalendarMode, CalendarNames, CaptionLayout,
    DateFormat, DateRange, Popover, PopoverContent, PopoverTrigger, Size, Weekday, icons,
};
use leptos::prelude::*;

/// A button that opens a two-month [`crate::Calendar`] in a popover for choosing a range of dates,
/// styled as shadcn/ui's Date Picker with a range. The first click picks the start, the second the end
/// (a click before the start swaps them) and a click after a complete range starts a new one. The
/// popover closes when the range is complete, unless `close_on_complete` is off.
///
/// `range` / `default_range` / `on_range_change` work as on [`crate::Calendar`]; the button shows
/// `Nov 9, 2026 – Nov 16, 2026` (in `format`, Medium by default) or the `placeholder`. `name` and `name_end` submit the
/// two dates as `YYYY-MM-DD` with a form (`name_end` defaults to `<name>-end`); `required` makes
/// the browser refuse a form with an incomplete range. Everything else, `min`, `max`,
/// `is_date_disabled`, `number_of_months`, `caption_layout`, `names` and the grid options, is passed to
/// the calendar. There is no typed entry for a range.
#[component]
pub fn DateRangePicker(
    #[prop(into, optional)] range: MaybeProp<DateRange>,
    #[prop(into, optional)] default_range: MaybeProp<DateRange>,
    #[prop(into, optional)] on_range_change: Option<Callback<DateRange>>,
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Shown while nothing is chosen. Defaults to `Pick a date range`.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// How the dates read on the button. Medium (`Nov 9, 2026`) by default, so two fit.
    #[prop(default = DateFormat::Medium)]
    format: DateFormat,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    #[prop(into, optional)] min: MaybeProp<CalendarDate>,
    #[prop(into, optional)] max: MaybeProp<CalendarDate>,
    #[prop(into, optional)] is_date_disabled: Option<Callback<CalendarDate, bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Marks it as required (`required` / `aria-required`).
    #[prop(optional)]
    required: bool,
    /// Form field name for the start; the date is submitted as `YYYY-MM-DD`.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Form field name for the end. Defaults to `<name>-end`.
    #[prop(into, optional)]
    name_end: MaybeProp<String>,
    /// Add a button that clears the range.
    #[prop(optional)]
    clearable: bool,
    #[prop(default = 2)] number_of_months: usize,
    /// Close the popover once the end is chosen. On by default.
    #[prop(default = true)]
    close_on_complete: bool,
    #[prop(optional)] caption_layout: CaptionLayout,
    #[prop(optional)] week_starts_on: Weekday,
    #[prop(default = true)] show_outside_days: bool,
    #[prop(optional)] fixed_weeks: bool,
    #[prop(optional)] show_week_numbers: bool,
    #[prop(into, optional)] start_year: MaybeProp<i32>,
    #[prop(into, optional)] end_year: MaybeProp<i32>,
    #[prop(into, optional)] today: MaybeProp<CalendarDate>,
    /// The month the calendar opens on when nothing is chosen (default: today's).
    #[prop(into, optional)]
    default_month: MaybeProp<CalendarDate>,
    #[prop(optional)] names: Option<CalendarNames>,
    /// The element's `id`.
    #[prop(optional, into)]
    id: Option<String>,
    /// Accessible name of the button.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let names = names.unwrap_or_default();
    let words = StoredValue::new(names.clone());
    let placeholder =
        StoredValue::new(placeholder.unwrap_or_else(|| "Pick a date range".to_string()));
    let id = StoredValue::new(id);
    let aria_label = StoredValue::new(aria_label);

    let state = use_synced(range, default_range, |_: &Option<DateRange>| {});
    let open_state = use_synced(open, default_open, |_: &Option<bool>| {});
    let is_open = Signal::derive(move || open_state.get().unwrap_or(false));
    let set_open = Callback::new(move |next: bool| {
        if next && disabled.get_untracked() {
            return;
        }
        if is_open.get_untracked() != next {
            open_state.set(Some(next));
            if let Some(cb) = on_open_change {
                cb.run(next);
            }
        }
    });
    let commit = Callback::new(move |next: DateRange| {
        let next = if next.is_empty() { None } else { Some(next) };
        if state.get_untracked() != next {
            state.set(next);
            if let Some(cb) = on_range_change {
                cb.run(next.unwrap_or_default());
            }
        }
    });

    let label = Signal::derive(move || {
        let r = state.get().unwrap_or_default();
        match (r.start, r.end) {
            (Some(start), Some(end)) => words
                .with_value(|n| format!("{} – {}", n.format(start, format), n.format(end, format))),
            (Some(start), None) => words.with_value(|n| format!("{} –", n.format(start, format))),
            _ => placeholder.get_value(),
        }
    });

    let content = move || {
        let names = names.clone();
        view! {
            <PopoverContent
                class="vdl-date-picker__content"
                align=Align::Start
                label="Choose a date range"
                on_open_auto_focus=Callback::new(move |ev: leptos::web_sys::Event| {
                    ev.prevent_default();
                    focus_tab_day();
                })
            >
                <Calendar
                    mode=CalendarMode::Range
                    range=state
                    number_of_months=number_of_months
                    min=min
                    max=max
                    is_date_disabled=is_date_disabled
                        .unwrap_or_else(|| Callback::new(|_: CalendarDate| false))
                    caption_layout=caption_layout
                    week_starts_on=week_starts_on
                    show_outside_days=show_outside_days
                    fixed_weeks=fixed_weeks
                    show_week_numbers=show_week_numbers
                    start_year=start_year
                    end_year=end_year
                    today=today
                    default_month=default_month
                    names=names.clone()
                    on_range_change=Callback::new(move |r: DateRange| {
                        commit.run(r);
                        if close_on_complete && r.is_complete() {
                            set_open.run(false);
                        }
                    })
                />
            </PopoverContent>
        }
    };

    let hidden = move || {
        let start_name = name.get();
        (start_name.is_some() || required).then(|| {
            let r = state.get().unwrap_or_default();
            let end_name = start_name
                .as_ref()
                .map(|n| name_end.get().unwrap_or_else(|| format!("{n}-end")));
            view! {
                {date_field(start_name, r.start.map(|d| d.to_iso()).unwrap_or_default(), required)}
                {date_field(end_name, r.end.map(|d| d.to_iso()).unwrap_or_default(), required)}
            }
        })
    };

    view! {
        <div
            class=format!("vdl-date-picker {class}")
            data-range=""
            data-disabled=move || disabled.get().then_some("")
        >
            <Popover open=Signal::derive(move || Some(is_open.get())) on_open_change=set_open>
                <PopoverTrigger
                    variant=ButtonVariant::Outline
                    size=size
                    class="vdl-date-picker__trigger"
                    attr:id=id.get_value()
                    attr:aria-label=aria_label.get_value()
                    attr:aria-invalid=move || invalid.get().then_some("true")
                    attr:disabled=move || disabled.get()
                    attr:data-empty=move || state.get().is_none_or(|r| r.is_empty()).then_some("")
                >
                    {icons::calendar()}
                    <span class="vdl-date-picker__value">{label}</span>
                </PopoverTrigger>
                {content()}
            </Popover>
            {move || {
                (clearable && state.get().is_some_and(|r| !r.is_empty()) && !disabled.get())
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="vdl-date-picker__clear"
                                aria-label="Clear range"
                                on:click=move |_| commit.run(DateRange::default())
                            >
                                {icons::x()}
                            </button>
                        }
                    })
            }}
            {hidden}
        </div>
    }
}
