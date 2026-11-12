use crate::calendar_state::use_synced;
use crate::primitives::calendar::{
    Bounds, CalendarDate, CalendarNames, DateRange, GridKey, RangePart, Weekday, days_in_month,
    find_enabled, month_weeks, navigate, next_range, toggle_date,
};
use crate::{Button, ButtonVariant, CalendarMode, CaptionLayout, Size, icons};
use leptos::wasm_bindgen::JsCast;
use leptos::{html, prelude::*};

/// Everything a day or caption needs from the calendar around it. All of it is `Copy`, so the many
/// small closures that make up the grid can each take their own copy.
#[derive(Clone, Copy)]
struct Ctx {
    mode: CalendarMode,
    week_starts_on: Weekday,
    fixed_weeks: bool,
    show_outside_days: bool,
    show_week_numbers: bool,
    caption_layout: CaptionLayout,
    start_year: MaybeProp<i32>,
    end_year: MaybeProp<i32>,
    names: StoredValue<CalendarNames>,
    today: Signal<CalendarDate>,
    bounds: Signal<Bounds>,
    single: RwSignal<Option<CalendarDate>>,
    many: RwSignal<Option<Vec<CalendarDate>>>,
    range: RwSignal<Option<DateRange>>,
    tab_date: Signal<Option<CalendarDate>>,
    focus_day: RwSignal<Option<CalendarDate>>,
    is_disabled: Callback<CalendarDate, bool>,
    select: Callback<CalendarDate>,
    /// Show the month containing this date as the first month.
    go_to: Callback<CalendarDate>,
}

impl Ctx {
    fn is_selected(&self, date: CalendarDate) -> bool {
        match self.mode {
            CalendarMode::Single => self.single.get() == Some(date),
            CalendarMode::Multiple => self
                .many
                .with(|m| m.as_ref().is_some_and(|v| v.contains(&date))),
            CalendarMode::Range => self.range.get().is_some_and(|r| r.part(date).is_some()),
        }
    }

    fn range_part(&self, date: CalendarDate) -> Option<RangePart> {
        if self.mode == CalendarMode::Range {
            self.range.get().and_then(|r| r.part(date))
        } else {
            None
        }
    }
}

fn range_part_str(part: RangePart) -> &'static str {
    match part {
        RangePart::Only => "only",
        RangePart::Start => "start",
        RangePart::Middle => "middle",
        RangePart::End => "end",
    }
}

/// The date of the day button a keyboard event came from.
fn event_date(ev: &leptos::web_sys::KeyboardEvent) -> Option<CalendarDate> {
    let target = ev.target()?.dyn_into::<leptos::web_sys::Element>().ok()?;
    let button = target.closest("[data-date]").ok()??;
    CalendarDate::parse_iso(&button.get_attribute("data-date")?)
}

fn day_view(ctx: Ctx, date: CalendarDate, month: CalendarDate) -> AnyView {
    let outside = !date.same_month(month);
    if outside && !ctx.show_outside_days {
        return view! { <td class="vdl-calendar__day" role="gridcell" data-hidden="" aria-hidden="true"></td> }
            .into_any();
    }

    view! {
        <td
            class="vdl-calendar__day"
            role="gridcell"
            aria-selected=move || ctx.is_selected(date).to_string()
            data-selected=move || ctx.is_selected(date).then_some("")
            data-today=move || (ctx.today.get() == date).then_some("")
            data-outside=outside.then_some("")
            data-disabled=move || ctx.is_disabled.run(date).then_some("")
            data-range=move || ctx.range_part(date).map(range_part_str)
        >
            <button
                type="button"
                class="vdl-calendar__day-button"
                data-date=date.to_iso()
                data-outside=outside.then_some("")
                tabindex=move || if !outside && ctx.tab_date.get() == Some(date) { "0" } else { "-1" }
                disabled=move || ctx.is_disabled.run(date)
                aria-current=move || (ctx.today.get() == date).then_some("date")
                aria-label=move || {
                    let mut label = ctx.names.with_value(|n| n.day_label(date));
                    if ctx.today.get() == date {
                        label.push_str(", ");
                        label.push_str(&ctx.names.with_value(|n| n.today.clone()));
                    }
                    label
                }
                on:click=move |_| ctx.select.run(date)
                on:focus=move |_| ctx.focus_day.set(Some(date))
            >
                {date.day().to_string()}
            </button>
        </td>
    }
    .into_any()
}

/// The month and year of one month, as text or as menus.
fn caption_view(ctx: Ctx, month: CalendarDate, index: usize) -> AnyView {
    let layout = ctx.caption_layout;
    if layout == CaptionLayout::Label {
        return view! {
            <div class="vdl-calendar__caption" aria-live="polite">
                {ctx.names.with_value(|n| n.month_caption(month))}
            </div>
        }
        .into_any();
    }

    let go = move |target: Option<CalendarDate>| {
        if let Some(target) = target {
            ctx.go_to.run(target.add_months(-(index as i64)));
        }
    };

    let month_part = if layout.has_month_menu() {
        view! {
            <label class="vdl-calendar__dropdown">
                <select
                    class="vdl-calendar__select"
                    aria-label=ctx.names.with_value(|n| n.month.clone())
                    on:change=move |ev| {
                        if let Ok(m) = event_target_value(&ev).parse::<u8>() {
                            go(CalendarDate::new(month.year(), m, 1));
                        }
                    }
                >
                    {move || {
                        let bounds = ctx.bounds.get();
                        (1u8..=12)
                            .map(|m| {
                                let first = CalendarDate::new(month.year(), m, 1).unwrap_or(month);
                                let last = first.end_of_month();
                                let out = bounds.min.is_some_and(|min| last < min)
                                    || bounds.max.is_some_and(|max| first > max);
                                let name = ctx.names.with_value(|n| n.month_name(m).to_string());
                                view! {
                                    <option value=m.to_string() selected=m == month.month() disabled=out>
                                        {name}
                                    </option>
                                }
                            })
                            .collect_view()
                    }}
                </select>
                <span class="vdl-calendar__dropdown-label" aria-hidden="true">
                    {ctx.names.with_value(|n| n.month_short(month.month()).to_string())}
                    {icons::chevron_down()}
                </span>
            </label>
        }
        .into_any()
    } else {
        view! { <span>{ctx.names.with_value(|n| n.month_name(month.month()).to_string())}</span> }
            .into_any()
    };

    let year_part = if layout.has_year_menu() {
        view! {
            <label class="vdl-calendar__dropdown">
                <select
                    class="vdl-calendar__select"
                    aria-label=ctx.names.with_value(|n| n.year.clone())
                    on:change=move |ev| {
                        if let Ok(y) = event_target_value(&ev).parse::<i32>() {
                            go(CalendarDate::new(y, month.month(), 1));
                        }
                    }
                >
                    {move || {
                        let bounds = ctx.bounds.get();
                        let today = ctx.today.get();
                        let low = ctx
                            .start_year
                            .get()
                            .or(bounds.min.map(|d| d.year()))
                            .unwrap_or(today.year() - 100)
                            .min(month.year());
                        let high = ctx
                            .end_year
                            .get()
                            .or(bounds.max.map(|d| d.year()))
                            .unwrap_or(today.year() + 10)
                            .max(month.year());
                        (low.max(CalendarDate::MIN_YEAR)..=high.min(CalendarDate::MAX_YEAR))
                            .map(|y| {
                                view! {
                                    <option value=y.to_string() selected=y == month.year()>
                                        {y.to_string()}
                                    </option>
                                }
                            })
                            .collect_view()
                    }}
                </select>
                <span class="vdl-calendar__dropdown-label" aria-hidden="true">
                    {month.year().to_string()}
                    {icons::chevron_down()}
                </span>
            </label>
        }
        .into_any()
    } else {
        view! { <span>{month.year().to_string()}</span> }.into_any()
    };

    view! {
        <div class="vdl-calendar__dropdowns" aria-live="polite">
            {month_part}
            {year_part}
        </div>
    }
    .into_any()
}

fn month_view(ctx: Ctx, month: CalendarDate, index: usize) -> AnyView {
    let weeks = month_weeks(
        month.year(),
        month.month(),
        ctx.week_starts_on,
        ctx.fixed_weeks,
    );
    let headers = (0..7)
        .map(|i| {
            let weekday = Weekday::from_index(ctx.week_starts_on.index() as i64 + i);
            ctx.names.with_value(|n| {
                (
                    n.weekday_name(weekday).to_string(),
                    n.weekday_short(weekday).to_string(),
                )
            })
        })
        .collect::<Vec<_>>();
    let caption_text = ctx.names.with_value(|n| n.month_caption(month));
    let week_label = ctx.names.with_value(|n| n.week_number.clone());
    let show_week_numbers = ctx.show_week_numbers;

    view! {
        <div class="vdl-calendar__month">
            {caption_view(ctx, month, index)}
            <table
                class="vdl-calendar__grid"
                role="grid"
                aria-label=caption_text
                aria-multiselectable=(ctx.mode != CalendarMode::Single).then_some("true")
            >
                <thead>
                    <tr class="vdl-calendar__weekdays">
                        {show_week_numbers
                            .then(|| {
                                view! {
                                    <th class="vdl-calendar__week-number-header" scope="col">
                                        <span class="vdl-sr-only">{week_label.clone()}</span>
                                    </th>
                                }
                            })}
                        {headers
                            .into_iter()
                            .map(|(full, short)| {
                                view! {
                                    <th class="vdl-calendar__weekday" scope="col" abbr=full.clone() aria-label=full>
                                        {short}
                                    </th>
                                }
                            })
                            .collect_view()}
                    </tr>
                </thead>
                <tbody>
                    {weeks
                        .into_iter()
                        .map(|week| {
                            // The week number is the ISO week of the row's Thursday, which is in
                            // every row whatever day the week starts on.
                            let number = week
                                .iter()
                                .find(|d| d.weekday() == Weekday::Thursday)
                                .map(|d| d.iso_week().1)
                                .unwrap_or_default();
                            view! {
                                <tr class="vdl-calendar__week">
                                    {show_week_numbers
                                        .then(|| {
                                            view! {
                                                <th class="vdl-calendar__week-number" scope="row">
                                                    {number.to_string()}
                                                </th>
                                            }
                                        })}
                                    {week.into_iter().map(|d| day_view(ctx, d, month)).collect_view()}
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
        </div>
    }
    .into_any()
}

/// A month grid for choosing days, styled as shadcn/ui's Calendar (react-day-picker). The model
/// (dates, month grids, keyboard movement, ranges) is the headless
/// `crate::primitives::calendar`; this is the markup, state and look.
///
/// `mode` is `Single` (default), `Multiple` or `Range`. Each has its own value: `value` /
/// `default_value` / `on_value_change` (which gets `None` when the day is cleared), `values` /
/// `default_values` / `on_values_change`, and `range` / `default_range` / `on_range_change`. A value
/// is controlled by passing a signal: the calendar follows it when it changes, including to empty.
/// The shown month works the same way (`month`, `default_month`, `on_month_change`, which gets the
/// first of the first shown month). It starts at the chosen day, else today.
///
/// Days come from `verdelle_ui::CalendarDate`; `min`, `max` and `is_date_disabled` stop days being
/// chosen or focused (the keyboard skips them). `number_of_months` shows several months side by side.
/// `caption_layout` swaps the `November 2026` label for month and year menus. `week_starts_on`,
/// `show_outside_days` (default on), `fixed_weeks` and `show_week_numbers` shape the grid, and `names`
/// replaces the English words (month and weekday names and the button labels). Give it a `name` to
/// submit the chosen day as `YYYY-MM-DD` with a form (a range uses `name` and `name_end`).
///
/// Keys, on a day: the arrows move by a day or a week, Home and End go to the start and end of the
/// week, PageUp and PageDown to the previous and next month, with Shift to the previous and next year;
/// Enter or Space chooses. Only one day is a tab stop, the chosen one, else today's, else the first
/// of the month.
///
/// The calendar reads "today" from the clock; pass `today` to fix it (screenshots, tests). Until the
/// browser takes over it uses the date in UTC, which is also what the server renders, so the two agree.
#[component]
pub fn Calendar(
    #[prop(optional)] mode: CalendarMode,
    #[prop(into, optional)] value: MaybeProp<CalendarDate>,
    #[prop(into, optional)] default_value: MaybeProp<CalendarDate>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<CalendarDate>>>,
    #[prop(into, optional)] values: MaybeProp<Vec<CalendarDate>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<CalendarDate>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<CalendarDate>>>,
    #[prop(into, optional)] range: MaybeProp<DateRange>,
    #[prop(into, optional)] default_range: MaybeProp<DateRange>,
    #[prop(into, optional)] on_range_change: Option<Callback<DateRange>>,
    /// The first shown month, as any date in it.
    #[prop(into, optional)]
    month: MaybeProp<CalendarDate>,
    #[prop(into, optional)] default_month: MaybeProp<CalendarDate>,
    #[prop(into, optional)] on_month_change: Option<Callback<CalendarDate>>,
    #[prop(default = 1)] number_of_months: usize,
    #[prop(optional)] caption_layout: CaptionLayout,
    #[prop(optional)] week_starts_on: Weekday,
    #[prop(default = true)] show_outside_days: bool,
    #[prop(optional)] fixed_weeks: bool,
    #[prop(optional)] show_week_numbers: bool,
    #[prop(into, optional)] min: MaybeProp<CalendarDate>,
    #[prop(into, optional)] max: MaybeProp<CalendarDate>,
    /// Days for which this returns true cannot be chosen.
    #[prop(into, optional)]
    is_date_disabled: Option<Callback<CalendarDate, bool>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// `Single` mode: choosing the chosen day again clears it. On by default.
    #[prop(default = true)]
    deselectable: bool,
    /// `Multiple` mode: the most days that can be chosen.
    #[prop(optional)]
    max_selected: Option<usize>,
    /// First and last year offered by the year menu (default: the min and max, else a hundred years back
    /// and ten ahead of today).
    #[prop(into, optional)]
    start_year: MaybeProp<i32>,
    #[prop(into, optional)] end_year: MaybeProp<i32>,
    /// Fix what "today" is.
    #[prop(into, optional)]
    today: MaybeProp<CalendarDate>,
    #[prop(optional)] names: Option<CalendarNames>,
    /// Form field name: the chosen day (or the start of a range), as `YYYY-MM-DD`.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Form field name for the end of a range. Defaults to `<name>-end`.
    #[prop(into, optional)]
    name_end: MaybeProp<String>,
    /// Accessible name of the whole calendar.
    #[prop(optional, into)]
    label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let months_shown = number_of_months.max(1);
    let names = StoredValue::new(names.unwrap_or_default());
    let root = NodeRef::<html::Div>::new();

    // "Today": the server's UTC date until the browser says what the visitor's date is.
    let local_today = RwSignal::new(CalendarDate::today_utc());
    Effect::new(move |_| local_today.set(CalendarDate::today()));
    let today = Signal::derive(move || today.get().unwrap_or_else(|| local_today.get()));

    let bounds = Signal::derive(move || Bounds {
        min: min.get(),
        max: max.get(),
    });
    let is_disabled = Callback::new(move |d: CalendarDate| {
        disabled.get() || !bounds.get().contains(d) || is_date_disabled.is_some_and(|cb| cb.run(d))
    });

    // The shown month.
    let initial_month = month
        .get_untracked()
        .or_else(|| default_month.get_untracked())
        .or_else(|| match mode {
            CalendarMode::Single => value
                .get_untracked()
                .or_else(|| default_value.get_untracked()),
            CalendarMode::Multiple => values
                .get_untracked()
                .or_else(|| default_values.get_untracked())
                .and_then(|v| v.first().copied()),
            CalendarMode::Range => range
                .get_untracked()
                .or_else(|| default_range.get_untracked())
                .and_then(|r| r.start),
        })
        .unwrap_or_else(|| today.get_untracked())
        .start_of_month();
    // Never open on a month wholly outside the limits.
    let initial_month = {
        let latest = max
            .get_untracked()
            .map(|m| m.start_of_month().add_months(-(months_shown as i64 - 1)));
        let earliest = min.get_untracked().map(|m| m.start_of_month());
        let capped = latest.map_or(initial_month, |l| initial_month.min(l));
        earliest.map_or(capped, |e| capped.max(e))
    };
    let first_month = RwSignal::new(initial_month);
    let go_to = Callback::new(move |d: CalendarDate| {
        let mut first = d.start_of_month();
        let b = bounds.get_untracked();
        if let Some(max) = b.max {
            let latest = max.start_of_month().add_months(-(months_shown as i64 - 1));
            if first > latest {
                first = latest;
            }
        }
        if let Some(min) = b.min
            && first < min.start_of_month()
        {
            first = min.start_of_month();
        }
        if first != first_month.get_untracked() {
            first_month.set(first);
            if let Some(cb) = on_month_change {
                cb.run(first);
            }
        }
    });
    {
        let first = StoredValue::new(true);
        Effect::new(move |_| {
            let next = month.get();
            if first.get_value() {
                first.set_value(false);
                return;
            }
            if let Some(next) = next {
                let next = next.start_of_month();
                if next != first_month.get_untracked() {
                    first_month.set(next);
                }
            }
        });
    }
    // After the value changes from outside, make sure the chosen day is on show.
    let reveal = move |d: CalendarDate| {
        let first = first_month.get_untracked();
        let last = first.add_months(months_shown as i64 - 1);
        if d.start_of_month() < first || d.start_of_month() > last {
            go_to.run(d);
        }
    };

    let single = use_synced(value, default_value, move |v: &Option<CalendarDate>| {
        if let Some(d) = v {
            reveal(*d);
        }
    });
    let many = use_synced(values, default_values, |_: &Option<Vec<CalendarDate>>| {});
    let range_state = use_synced(range, default_range, move |v: &Option<DateRange>| {
        if let Some(d) = v.and_then(|r| r.start) {
            reveal(d);
        }
    });

    let focus_day = RwSignal::new(None::<CalendarDate>);

    let in_view = move |d: CalendarDate| {
        let first = first_month.get();
        let m = d.start_of_month();
        m >= first && m <= first.add_months(months_shown as i64 - 1)
    };
    let chosen_dates = move || -> Vec<CalendarDate> {
        match mode {
            CalendarMode::Single => single.get().into_iter().collect(),
            CalendarMode::Multiple => many.get().unwrap_or_default(),
            CalendarMode::Range => range_state
                .get()
                .map(|r| r.start.into_iter().chain(r.end).collect())
                .unwrap_or_default(),
        }
    };
    // The one day that is a tab stop: the focused one, else a chosen one, else today, else the first
    // selectable day of the first month.
    let tab_date = Signal::derive(move || {
        let usable = |d: CalendarDate| in_view(d) && !is_disabled.run(d);
        if let Some(d) = focus_day.get()
            && usable(d)
        {
            return Some(d);
        }
        if let Some(d) = chosen_dates().into_iter().find(|d| usable(*d)) {
            return Some(d);
        }
        let t = today.get();
        if usable(t) {
            return Some(t);
        }
        let first = first_month.get();
        (0..days_in_month(first.year(), first.month()) as i64)
            .map(|i| first.add_days(i))
            .find(|d| !is_disabled.run(*d))
    });

    let select = Callback::new(move |date: CalendarDate| {
        if is_disabled.run(date) {
            return;
        }
        match mode {
            CalendarMode::Single => {
                let current = single.get_untracked();
                let next = if current == Some(date) && deselectable {
                    None
                } else {
                    Some(date)
                };
                if next != current {
                    single.set(next);
                    if let Some(cb) = on_value_change {
                        cb.run(next);
                    }
                }
            }
            CalendarMode::Multiple => {
                let current = many.get_untracked().unwrap_or_default();
                let next = toggle_date(&current, date, max_selected);
                if next != current {
                    many.set(Some(next.clone()));
                    if let Some(cb) = on_values_change {
                        cb.run(next);
                    }
                }
            }
            CalendarMode::Range => {
                let next = next_range(range_state.get_untracked().unwrap_or_default(), date);
                range_state.set(Some(next));
                if let Some(cb) = on_range_change {
                    cb.run(next);
                }
            }
        }
        focus_day.set(Some(date));
        // A day of the next or previous month: show that month (with several months it is on show).
        if months_shown == 1 && !date.same_month(first_month.get_untracked()) {
            go_to.run(date);
        }
    });

    let ctx = Ctx {
        mode,
        week_starts_on,
        fixed_weeks,
        show_outside_days,
        show_week_numbers,
        caption_layout,
        start_year,
        end_year,
        names,
        today,
        bounds,
        single,
        many,
        range: range_state,
        tab_date,
        focus_day,
        is_disabled,
        select,
        go_to,
    };

    let focus_date = move |target: CalendarDate| {
        focus_day.set(Some(target));
        let first = first_month.get_untracked();
        let last = first.add_months(months_shown as i64 - 1);
        if target.start_of_month() < first {
            go_to.run(target);
        } else if target.start_of_month() > last {
            go_to.run(target.add_months(-(months_shown as i64 - 1)));
        }
        // Focus it now if its button is already on show (so a key pressed straight after, Enter
        // for one, acts on this day); after a change of month the button only exists once the
        // months have re-rendered.
        let try_focus = move || -> bool {
            let Some(root) = root.try_get_untracked().flatten() else {
                return false;
            };
            let selector = format!(
                "button[data-date=\"{}\"]:not([data-outside])",
                target.to_iso()
            );
            if let Ok(Some(el)) = root.query_selector(&selector)
                && let Ok(el) = el.dyn_into::<leptos::web_sys::HtmlElement>()
            {
                let _ = el.focus();
                return true;
            }
            false
        };
        if !try_focus() {
            request_animation_frame(move || {
                try_focus();
            });
        }
    };

    let on_keydown = move |ev: leptos::web_sys::KeyboardEvent| {
        if ev.ctrl_key() || ev.meta_key() || ev.alt_key() {
            return;
        }
        let Some(key) = GridKey::from_key(&ev.key(), ev.shift_key()) else {
            return;
        };
        let Some(current) = event_date(&ev) else {
            return;
        };
        ev.prevent_default();
        let candidate = navigate(current, key, week_starts_on);
        let b = bounds.get_untracked();
        if let Some(target) = find_enabled(candidate, key.direction(), &b, &|d| is_disabled.run(d))
        {
            focus_date(target);
        }
    };

    let prev_disabled = Signal::derive(move || {
        disabled.get()
            || bounds
                .get()
                .min
                .is_some_and(|m| first_month.get() <= m.start_of_month())
    });
    let next_disabled = Signal::derive(move || {
        disabled.get()
            || bounds.get().max.is_some_and(|m| {
                first_month.get().add_months(months_shown as i64 - 1) >= m.start_of_month()
            })
    });

    view! {
        <div
            node_ref=root
            class=format!("vdl-calendar {class}")
            role="group"
            aria-label=label
            data-mode=mode.as_str()
            data-months=months_shown.to_string()
            data-disabled=move || disabled.get().then_some("")
            on:keydown=on_keydown
        >
            <div class="vdl-calendar__months">
                <div class="vdl-calendar__nav">
                    <Button
                        variant=ButtonVariant::Ghost
                        size=Size::Small
                        icon_only=true
                        class="vdl-calendar__nav-button"
                        aria_label=names.with_value(|n| n.previous_month.clone())
                        disabled=prev_disabled
                        on_click=Callback::new(move |_| go_to.run(first_month.get_untracked().add_months(-1)))
                    >
                        {icons::chevron_left()}
                    </Button>
                    <Button
                        variant=ButtonVariant::Ghost
                        size=Size::Small
                        icon_only=true
                        class="vdl-calendar__nav-button"
                        aria_label=names.with_value(|n| n.next_month.clone())
                        disabled=next_disabled
                        on_click=Callback::new(move |_| go_to.run(first_month.get_untracked().add_months(1)))
                    >
                        {icons::chevron_right()}
                    </Button>
                </div>
                {move || {
                    let first = first_month.get();
                    (0..months_shown)
                        .map(|i| month_view(ctx, first.add_months(i as i64), i))
                        .collect_view()
                }}
            </div>
            {move || {
                name.get()
                    .map(|name| match mode {
                        CalendarMode::Single => {
                            let iso = single.get().map(|d| d.to_iso()).unwrap_or_default();
                            view! { <input type="hidden" name=name value=iso /> }.into_any()
                        }
                        CalendarMode::Multiple => {
                            many.get()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|d| view! { <input type="hidden" name=name.clone() value=d.to_iso() /> })
                                .collect_view()
                                .into_any()
                        }
                        CalendarMode::Range => {
                            let r = range_state.get().unwrap_or_default();
                            let start = r.start.map(|d| d.to_iso()).unwrap_or_default();
                            let end = r.end.map(|d| d.to_iso()).unwrap_or_default();
                            let end_name = name_end.get().unwrap_or_else(|| format!("{name}-end"));
                            view! {
                                <input type="hidden" name=name value=start />
                                <input type="hidden" name=end_name value=end />
                            }
                            .into_any()
                        }
                    })
            }}
        </div>
    }
}
