use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Calendar, CalendarDate, CalendarMode, CalendarNames, CaptionLayout,
    DateFormat, DatePicker, DateRange, DateRangePicker, Label, Popover, PopoverContent,
    PopoverTrigger, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Size, Weekday,
};

fn d(year: i32, month: u8, day: u8) -> CalendarDate {
    CalendarDate::new(year, month, day).expect("a valid date")
}

/// A month that is always the same, so the examples with fixed dates look the same every day.
fn november() -> CalendarDate {
    d(2026, 11, 1)
}

fn text(date: Option<CalendarDate>) -> String {
    date.map_or_else(|| "none".to_string(), |d| d.to_iso())
}

fn range_text(range: Option<DateRange>) -> String {
    let r = range.unwrap_or_default();
    format!("{} to {}", text(r.start), text(r.end))
}

const FRAME: &str = "rounded-lg border border-border";

#[component]
pub fn CalendarsPage() -> impl IntoView {
    view! {
        <Page
            title="Calendar and Date Picker"
            summary="Calendar is a month grid (react-day-picker's look) on a headless date model; DatePicker and DateRangePicker put it in a popover behind a button. Dates are CalendarDate values with no time zone. Keys on a day: arrows move by a day or a week, Home and End to the start and end of the week, PageUp and PageDown by a month (with Shift, a year), Enter or Space chooses."
        >
            <SingleSection />
            <MultipleAndRange />
            <CaptionSection />
            <GridSection />
            <ConstraintSection />
            <MonthsSection />
            <NamesAndFormSection />
            <PickersSection />
            <TypedAndFormSection />
            <RangePickerSection />
            <PresetsSection />
        </Page>
    }
}

#[component]
fn SingleSection() -> impl IntoView {
    let chosen = RwSignal::new(Some(d(2026, 11, 12)));
    let changes = RwSignal::new(0_u32);

    view! {
        <Section
            title="Single day"
            description="Controlled with a signal: the readout and the buttons below use the same one, so setting or clearing it from outside moves the calendar. Today is the muted tile; choosing the chosen day again clears it."
        >
            <div class="flex flex-wrap items-start gap-6">
                <Calendar
                    value=chosen
                    default_month=november()
                    on_value_change=Callback::new(move |v: Option<CalendarDate>| {
                        chosen.set(v);
                        changes.update(|n| *n += 1);
                    })
                    class=FRAME
                    label="Single day"
                />
                <div class="space-y-3 text-sm">
                    <p>
                        "Chosen: " <strong id="single-readout">{move || text(chosen.get())}</strong>
                    </p>
                    <p>
                        "Changes reported: " <span id="single-changes">{move || changes.get()}</span>
                    </p>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| chosen.set(Some(d(2027, 2, 3)))
                        >
                            "Set to 3 Feb 2027"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| chosen.set(None)
                        >
                            "Clear"
                        </Button>
                    </Row>
                </div>
            </div>
        </Section>
        <Section
            title="Today, and not deselectable"
            description="The first calendar opens on the current month and shows today. The second has deselectable off: choosing the chosen day again keeps it."
        >
            <div class="flex flex-wrap items-start gap-6">
                <Calendar class=FRAME label="Current month" />
                <Calendar
                    default_value=d(2026, 11, 20)
                    default_month=november()
                    deselectable=false
                    class=FRAME
                    label="Not deselectable"
                />
            </div>
        </Section>
    }
}

#[component]
fn MultipleAndRange() -> impl IntoView {
    let many = RwSignal::new(Some(vec![d(2026, 11, 3), d(2026, 11, 17)]));
    let capped = RwSignal::new(None::<Vec<CalendarDate>>);
    let range = RwSignal::new(Some(DateRange::new(d(2026, 11, 10), d(2026, 11, 14))));

    view! {
        <Section
            title="Multiple days"
            description="mode=Multiple with values and on_values_change; each click toggles a day. The second calendar allows at most three (max_selected)."
        >
            <div class="flex flex-wrap items-start gap-6">
                <div class="space-y-2">
                    <Calendar
                        mode=CalendarMode::Multiple
                        values=many
                        on_values_change=Callback::new(move |v: Vec<CalendarDate>| many.set(Some(v)))
                        default_month=november()
                        class=FRAME
                        label="Multiple days"
                    />
                    <p class="text-sm">
                        "Chosen: "
                        <span id="multi-readout">
                            {move || {
                                many.get()
                                    .unwrap_or_default()
                                    .iter()
                                    .map(|d| d.to_iso())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }}
                        </span>
                    </p>
                </div>
                <div class="space-y-2">
                    <Calendar
                        mode=CalendarMode::Multiple
                        values=capped
                        on_values_change=Callback::new(move |v: Vec<CalendarDate>| capped.set(Some(v)))
                        max_selected=3
                        default_month=november()
                        class=FRAME
                        label="At most three days"
                    />
                    <p class="text-sm">
                        "Chosen: " <span id="capped-count">{move || capped.get().unwrap_or_default().len()}</span>
                        " of 3"
                    </p>
                </div>
            </div>
        </Section>
        <Section
            title="Range"
            description="mode=Range: the first click starts a range, the second ends it (before the start swaps them), a click after a complete range starts again. The band between the ends is muted."
        >
            <div class="flex flex-wrap items-start gap-6">
                <Calendar
                    mode=CalendarMode::Range
                    range=range
                    on_range_change=Callback::new(move |r: DateRange| range.set(Some(r)))
                    default_month=november()
                    class=FRAME
                    label="Range"
                />
                <div class="space-y-3 text-sm">
                    <p>"Range: " <strong id="range-readout">{move || range_text(range.get())}</strong></p>
                    <Button
                        variant=ButtonVariant::Outline
                        size=Size::Small
                        on_click=move |_| range.set(None)
                    >
                        "Clear"
                    </Button>
                </div>
            </div>
        </Section>
    }
}

#[component]
fn CaptionSection() -> impl IntoView {
    view! {
        <Section
            title="Caption layouts"
            description="Label shows the month as text and moves with the arrows. The three menu layouts use real selects laid invisibly over the label, so the browser's own picker and keyboard handling work. The year menu runs from start_year to end_year (default: a hundred years back to ten ahead, or the min and max)."
        >
            <div class="flex flex-wrap items-start gap-6">
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Label (default)"</p>
                    <Calendar default_month=november() class=FRAME label="Label caption" />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Dropdown"</p>
                    <Calendar
                        caption_layout=CaptionLayout::Dropdown
                        default_month=november()
                        class=FRAME
                        label="Dropdown caption"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"DropdownMonths"</p>
                    <Calendar
                        caption_layout=CaptionLayout::DropdownMonths
                        default_month=november()
                        class=FRAME
                        label="Month dropdown caption"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"DropdownYears, 2020 to 2030"</p>
                    <Calendar
                        caption_layout=CaptionLayout::DropdownYears
                        start_year=2020
                        end_year=2030
                        default_month=november()
                        class=FRAME
                        label="Year dropdown caption"
                    />
                </div>
            </div>
        </Section>
    }
}

#[component]
fn GridSection() -> impl IntoView {
    view! {
        <Section
            title="Grid options"
            description="week_starts_on, show_week_numbers (ISO weeks), fixed_weeks (always six rows, so the height never changes) and show_outside_days off."
        >
            <div class="flex flex-wrap items-start gap-6">
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Week starts on Monday"</p>
                    <Calendar
                        week_starts_on=Weekday::Monday
                        default_month=november()
                        class=FRAME
                        label="Monday start"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Week numbers"</p>
                    <Calendar
                        show_week_numbers=true
                        week_starts_on=Weekday::Monday
                        default_month=november()
                        class=FRAME
                        label="Week numbers"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Fixed weeks (February 2026 has four)"</p>
                    <Calendar
                        fixed_weeks=true
                        default_month=d(2026, 2, 1)
                        class=FRAME
                        label="Fixed weeks"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"No outside days"</p>
                    <Calendar
                        show_outside_days=false
                        default_month=november()
                        class=FRAME
                        label="No outside days"
                    />
                </div>
            </div>
        </Section>
    }
}

#[component]
fn ConstraintSection() -> impl IntoView {
    let weekend =
        Callback::new(|d: CalendarDate| matches!(d.weekday(), Weekday::Saturday | Weekday::Sunday));

    view! {
        <Section
            title="Limits and disabled days"
            description="min and max (the arrows stop at the limits and the keyboard skips what is outside them), is_date_disabled (here weekends: arrows jump over them), and a calendar that is disabled as a whole."
        >
            <div class="flex flex-wrap items-start gap-6">
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"min 5 Nov, max 25 Nov 2026"</p>
                    <Calendar
                        min=d(2026, 11, 5)
                        max=d(2026, 11, 25)
                        default_month=november()
                        class=FRAME
                        label="Limited to November 5 to 25"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Weekends disabled"</p>
                    <Calendar
                        is_date_disabled=weekend
                        default_month=november()
                        class=FRAME
                        label="Weekdays only"
                    />
                </div>
                <div class="space-y-1">
                    <p class="text-xs text-muted-foreground">"Disabled"</p>
                    <Calendar
                        disabled=true
                        default_value=d(2026, 11, 12)
                        default_month=november()
                        class=FRAME
                        label="Disabled"
                    />
                </div>
            </div>
        </Section>
    }
}

#[component]
fn MonthsSection() -> impl IntoView {
    let month = RwSignal::new(Some(d(2026, 11, 1)));
    let reported = RwSignal::new(String::from("none yet"));
    let range = RwSignal::new(None::<DateRange>);

    view! {
        <Section
            title="Several months"
            description="number_of_months shows months side by side (from 768px wide; narrower they stack). Where the page column is narrower than the months, as here beside the gallery sidebar at tablet width, wrap the calendars in an overflow-x-auto box. The arrows are over the first and last caption. Keys carry the focus across months."
        >
            <div class="space-y-3 overflow-x-auto p-1">
                <Calendar
                    mode=CalendarMode::Range
                    range=range
                    on_range_change=Callback::new(move |r: DateRange| range.set(Some(r)))
                    number_of_months=2
                    default_month=november()
                    class=FRAME
                    label="Two months"
                />
                <p class="text-sm">"Range: " <span id="two-months-readout">{move || range_text(range.get())}</span></p>
                <Calendar
                    number_of_months=3
                    default_month=november()
                    caption_layout=CaptionLayout::DropdownMonths
                    class=FRAME
                    label="Three months"
                />
            </div>
        </Section>
        <Section
            title="Controlled month"
            description="month, default_month and on_month_change. The buttons set the month from outside; the readout is what on_month_change last reported (the first of the first shown month)."
        >
            <div class="flex flex-wrap items-start gap-6">
                <Calendar
                    month=month
                    on_month_change=Callback::new(move |m: CalendarDate| {
                        month.set(Some(m));
                        reported.set(m.to_iso());
                    })
                    class=FRAME
                    label="Controlled month"
                />
                <div class="space-y-3 text-sm">
                    <p>"Reported: " <span id="month-readout">{move || reported.get()}</span></p>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| month.set(Some(d(2024, 2, 1)))
                        >
                            "Go to Feb 2024"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| month.set(Some(d(2030, 12, 1)))
                        >
                            "Go to Dec 2030"
                        </Button>
                    </Row>
                </div>
            </div>
        </Section>
    }
}

fn spanish() -> CalendarNames {
    let list = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    CalendarNames {
        months: list(&[
            "enero",
            "febrero",
            "marzo",
            "abril",
            "mayo",
            "junio",
            "julio",
            "agosto",
            "septiembre",
            "octubre",
            "noviembre",
            "diciembre",
        ]),
        months_short: list(&[
            "ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic",
        ]),
        weekdays: list(&[
            "domingo",
            "lunes",
            "martes",
            "miércoles",
            "jueves",
            "viernes",
            "sábado",
        ]),
        weekdays_short: list(&["do", "lu", "ma", "mi", "ju", "vi", "sá"]),
        previous_month: "Ir al mes anterior".into(),
        next_month: "Ir al mes siguiente".into(),
        month: "Mes".into(),
        year: "Año".into(),
        week_number: "Semana".into(),
        today: "hoy".into(),
    }
}

#[component]
fn NamesAndFormSection() -> impl IntoView {
    let submitted = RwSignal::new(String::from("not submitted"));

    view! {
        <Section
            title="Names"
            description="names replaces every word the calendar shows and reads aloud: months, weekdays and the button and menu labels. Here Spanish, with the week starting on Monday."
        >
            <Calendar
                names=spanish()
                week_starts_on=Weekday::Monday
                caption_layout=CaptionLayout::Dropdown
                default_value=d(2026, 11, 12)
                default_month=november()
                class=FRAME
                label="Calendario"
            />
        </Section>
        <Section
            title="In a form"
            description="name submits the chosen day as YYYY-MM-DD in a hidden input; a range uses name and name_end; a multiple selection one input per day."
        >
            <form
                class="space-y-3"
                on:submit=move |ev| {
                    ev.prevent_default();
                    use leptos::wasm_bindgen::JsCast;
                    if let Some(form) = ev
                        .target()
                        .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                        && let Ok(data) = leptos::web_sys::FormData::new_with_form(&form)
                    {
                        let get = |k: &str| data.get(k).as_string().unwrap_or_else(|| "(absent)".into());
                        submitted
                            .set(format!("day={}, from={}, until={}", get("day"), get("from"), get("until")));
                    }
                }
            >
                <div class="flex flex-wrap items-start gap-6">
                    <Calendar
                        name="day"
                        default_value=d(2026, 11, 9)
                        default_month=november()
                        class=FRAME
                        label="Form day"
                    />
                    <Calendar
                        mode=CalendarMode::Range
                        name="from"
                        name_end="until"
                        default_range=DateRange::new(d(2026, 11, 16), d(2026, 11, 20))
                        default_month=november()
                        class=FRAME
                        label="Form range"
                    />
                </div>
                <Row>
                    <button type="submit" class="vdl-btn" data-variant="outline" data-size="sm">"Submit"</button>
                    <span class="text-sm" id="form-readout">{move || submitted.get()}</span>
                </Row>
            </form>
        </Section>
    }
}

#[component]
fn PickersSection() -> impl IntoView {
    let chosen = RwSignal::new(None::<CalendarDate>);
    let open = RwSignal::new(false);

    view! {
        <Section
            title="Date picker"
            description="A popover with a calendar behind an outline button, as in shadcn. The button shows the date, or the placeholder in muted text. Choosing a day closes the popover and focus goes back to the button. When it opens, focus lands on the chosen day (or today)."
        >
            <Row label="Default">
                <DatePicker aria_label="Default date picker" />
            </Row>
            <Row label="With a value">
                <DatePicker default_value=d(2026, 11, 9) aria_label="Date picker with a value" />
            </Row>
            <Row label="Controlled">
                <DatePicker
                    value=chosen
                    on_value_change=Callback::new(move |v: Option<CalendarDate>| chosen.set(v))
                    open=open
                    on_open_change=Callback::new(move |o: bool| open.set(o))
                    aria_label="Controlled date picker"
                />
                <Button
                    variant=ButtonVariant::Outline
                    size=Size::Small
                    on_click=move |_| chosen.set(Some(d(2027, 1, 15)))
                >
                    "Set 15 Jan 2027"
                </Button>
                <Button variant=ButtonVariant::Outline size=Size::Small on_click=move |_| chosen.set(None)>
                    "Clear"
                </Button>
                <span class="text-sm">
                    "Value: " <span id="picker-readout">{move || text(chosen.get())}</span> ", open: "
                    <span id="picker-open">{move || open.get().to_string()}</span>
                </span>
            </Row>
        </Section>
        <Section
            title="Date picker options"
            description="format (Long, Medium, Iso), size, clearable, placeholder, min and max, disabled and invalid, and a date of birth picker with year and month menus."
        >
            <Row label="Long">
                <DatePicker default_value=d(2026, 11, 9) format=DateFormat::Long aria_label="Long format" />
            </Row>
            <Row label="Medium">
                <DatePicker default_value=d(2026, 11, 9) format=DateFormat::Medium aria_label="Medium format" />
            </Row>
            <Row label="Iso">
                <DatePicker default_value=d(2026, 11, 9) format=DateFormat::Iso aria_label="ISO format" />
            </Row>
            <Row label="Small">
                <DatePicker size=Size::Small aria_label="Small date picker" />
            </Row>
            <Row label="Large">
                <DatePicker size=Size::Large aria_label="Large date picker" />
            </Row>
            <Row label="Clearable">
                <DatePicker
                    clearable=true
                    default_value=d(2026, 11, 9)
                    placeholder="Choose a day"
                    aria_label="Clearable date picker"
                />
            </Row>
            <Row label="Limited">
                <DatePicker
                    min=d(2026, 11, 5)
                    max=d(2026, 11, 25)
                    default_month=november()
                    aria_label="Limited date picker"
                />
            </Row>
            <Row label="Disabled">
                <DatePicker disabled=true default_value=d(2026, 11, 9) aria_label="Disabled date picker" />
            </Row>
            <Row label="Invalid">
                <DatePicker invalid=true aria_label="Invalid date picker" />
            </Row>
            <Row label="Date of birth">
                <div class="flex flex-col gap-2">
                    <Label html_for="dob">"Date of birth"</Label>
                    <DatePicker
                        id="dob"
                        caption_layout=CaptionLayout::Dropdown
                        end_year=2026
                        placeholder="Select date"
                    />
                </div>
            </Row>
        </Section>
    }
}

#[component]
fn TypedAndFormSection() -> impl IntoView {
    let typed = RwSignal::new(Some(d(2026, 11, 9)));
    let submitted = RwSignal::new(String::from("not submitted"));

    view! {
        <Section
            title="Typed entry"
            description="typed turns the button into a text field with a calendar button. Type YYYY-MM-DD: a valid date is chosen as you type and the calendar follows; a complete but invalid or disallowed date (here, outside 2026) marks the field invalid; leaving the field with text that is not a date puts the chosen date back. clearable adds the clear button."
        >
            <Row label="Typed">
                <DatePicker
                    typed=true
                    clearable=true
                    value=typed
                    on_value_change=Callback::new(move |v: Option<CalendarDate>| typed.set(v))
                    min=d(2026, 1, 1)
                    max=d(2026, 12, 31)
                    aria_label="Typed date"
                />
                <span class="text-sm">"Value: " <span id="typed-readout">{move || text(typed.get())}</span></span>
            </Row>
        </Section>
        <Section
            title="Date picker in a form"
            description="name submits YYYY-MM-DD; required makes the browser block a form with no date (it shows its own message). Pick a date, or leave it empty, and submit."
        >
            <form
                class="flex flex-wrap items-center gap-3"
                on:submit=move |ev| {
                    ev.prevent_default();
                    use leptos::wasm_bindgen::JsCast;
                    if let Some(form) = ev
                        .target()
                        .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                        && let Ok(data) = leptos::web_sys::FormData::new_with_form(&form)
                    {
                        submitted
                            .set(
                                format!(
                                    "arrival={}",
                                    data.get("arrival").as_string().unwrap_or_else(|| "(absent)".into()),
                                ),
                            );
                    }
                }
            >
                <DatePicker name="arrival" required=true placeholder="Arrival date" aria_label="Arrival date" />
                <button type="submit" class="vdl-btn" data-variant="outline" data-size="md">"Submit"</button>
                <span class="text-sm" id="picker-form-readout">{move || submitted.get()}</span>
            </form>
        </Section>
    }
}

#[component]
fn RangePickerSection() -> impl IntoView {
    let range = RwSignal::new(None::<DateRange>);
    let submitted = RwSignal::new(String::from("not submitted"));

    view! {
        <Section
            title="Date range picker"
            description="Two months in a popover. It closes when the end is chosen (close_on_complete, on by default). The button reads 'start – end'; with only a start chosen it reads 'start –'."
        >
            <Row label="Default">
                <DateRangePicker aria_label="Default range picker" />
            </Row>
            <Row label="Controlled">
                <DateRangePicker
                    range=range
                    on_range_change=Callback::new(move |r: DateRange| range.set(Some(r)))
                    clearable=true
                    aria_label="Controlled range picker"
                />
                <span class="text-sm">"Range: " <span id="range-picker-readout">{move || range_text(range.get())}</span></span>
            </Row>
            <Row label="Stays open">
                <DateRangePicker
                    close_on_complete=false
                    default_range=DateRange::new(d(2026, 11, 10), d(2026, 11, 14))
                    aria_label="Range picker that stays open"
                />
            </Row>
            <Row label="One month">
                <DateRangePicker number_of_months=1 aria_label="One month range picker" />
            </Row>
            <Row label="Iso format">
                <DateRangePicker
                    format=DateFormat::Iso
                    default_range=DateRange::new(d(2026, 11, 10), d(2026, 11, 14))
                    aria_label="ISO range picker"
                />
            </Row>
            <Row label="In a form">
                <form
                    class="flex flex-wrap items-center gap-3"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        use leptos::wasm_bindgen::JsCast;
                        if let Some(form) = ev
                            .target()
                            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                            && let Ok(data) = leptos::web_sys::FormData::new_with_form(&form)
                        {
                            let get = |k: &str| data.get(k).as_string().unwrap_or_else(|| "(absent)".into());
                            submitted.set(format!("stay={} to {}", get("stay"), get("stay-end")));
                        }
                    }
                >
                    <DateRangePicker
                        name="stay"
                        default_range=DateRange::new(d(2026, 12, 20), d(2026, 12, 27))
                        aria_label="Stay"
                    />
                    <button type="submit" class="vdl-btn" data-variant="outline" data-size="md">"Submit"</button>
                    <span class="text-sm" id="range-form-readout">{move || submitted.get()}</span>
                </form>
            </Row>
        </Section>
    }
}

#[component]
fn PresetsSection() -> impl IntoView {
    let chosen = RwSignal::new(None::<CalendarDate>);
    let month = RwSignal::new(None::<CalendarDate>);
    let open = RwSignal::new(false);
    let today = CalendarDate::today_utc();

    let preset = Callback::new(move |days: String| {
        if let Ok(days) = days.parse::<i64>() {
            let date = today.add_days(days);
            chosen.set(Some(date));
            month.set(Some(date));
        }
    });

    view! {
        <Section
            title="With presets"
            description="shadcn's presets example: a Select of shortcuts above a calendar, both driving one value. Picking a preset moves the calendar to that month; picking a day closes the popover."
        >
            <Row>
                <Popover open=open on_open_change=move |o| open.set(o)>
                    <PopoverTrigger variant=ButtonVariant::Outline class="vdl-date-picker__trigger">
                        {move || {
                            chosen
                                .get()
                                .map_or_else(|| "Pick a date".to_string(), |d| {
                                    verdelle_ui::CalendarNames::default()
                                        .format(d, DateFormat::Long)
                                })
                        }}
                    </PopoverTrigger>
                    <PopoverContent
                        class="vdl-date-picker__content"
                        label="Choose a date with presets"
                        width=300u32
                    >
                        <div class="flex flex-col gap-2 p-3 pb-0">
                            <Select on_value_change=preset>
                                <SelectTrigger aria_label="Preset">
                                    <SelectValue placeholder="Select a preset" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="0">"Today"</SelectItem>
                                    <SelectItem value="1">"Tomorrow"</SelectItem>
                                    <SelectItem value="3">"In 3 days"</SelectItem>
                                    <SelectItem value="7">"In a week"</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                        <Calendar
                            value=chosen
                            month=month
                            on_month_change=Callback::new(move |m: CalendarDate| month.set(Some(m)))
                            on_value_change=Callback::new(move |v: Option<CalendarDate>| {
                                chosen.set(v);
                                if v.is_some() {
                                    open.set(false);
                                }
                            })
                            deselectable=false
                        />
                    </PopoverContent>
                </Popover>
                <span class="text-sm">"Value: " <span id="presets-readout">{move || text(chosen.get())}</span></span>
            </Row>
        </Section>
    }
}
