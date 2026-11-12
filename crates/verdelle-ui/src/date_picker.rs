use crate::calendar_state::use_synced;
use crate::{
    Align, ButtonVariant, Calendar, CalendarDate, CalendarNames, CaptionLayout, DateFormat,
    Popover, PopoverContent, PopoverTrigger, Size, Weekday, icons,
};
use leptos::wasm_bindgen::JsCast;
use leptos::{html, prelude::*};

/// Move focus to the day the calendar in the open date picker would tab to (the chosen one, else
/// today's). A calendar opening in a popover calls this instead of focusing its first button, the
/// previous-month arrow. The popover is portalled, so the search is scoped to its content class: a
/// plain query would find a calendar elsewhere in the page and move focus out, which closes it.
pub(crate) fn focus_tab_day() {
    request_animation_frame(|| {
        if let Ok(Some(el)) = document()
            .query_selector(
                ".vdl-date-picker__content .vdl-calendar__day-button[tabindex=\"0\"]:not([data-outside])",
            )
            && let Ok(el) = el.dyn_into::<leptos::web_sys::HtmlElement>()
        {
            let _ = el.focus();
        }
    });
}

/// A hidden form field for a picked date. With `required`, a visually hidden text field takes its
/// place, because browsers skip validation on `type=hidden`.
pub(crate) fn date_field(name: Option<String>, value: String, required: bool) -> AnyView {
    if required {
        view! {
            <input
                type="text"
                name=name
                value=value
                required=true
                tabindex="-1"
                aria-hidden="true"
                style="position:absolute;width:1px;height:1px;opacity:0;pointer-events:none;"
            />
        }
        .into_any()
    } else {
        view! { <input type="hidden" name=name value=value /> }.into_any()
    }
}

/// A button that opens a [`crate::Calendar`] in a popover and shows the chosen date, styled as
/// shadcn/ui's Date Picker (a Popover, an outline Button and a Calendar). Choosing a day closes it.
///
/// `value` / `default_value` / `on_value_change` work as on [`crate::Calendar`]: pass a signal to
/// control it, and it follows when the signal changes, including to empty. The button shows the date
/// in `format` (default `November 9, 2026`) or the `placeholder`. `clearable` adds a button that clears
/// it. `min`, `max` and `is_date_disabled` limit what can be chosen, and `names`, `caption_layout`,
/// `week_starts_on`, `show_outside_days`, `fixed_weeks` and `show_week_numbers` are passed to the
/// calendar.
///
/// With `typed`, the button becomes a text field with a calendar button at its end. Type
/// `YYYY-MM-DD`: a valid, allowed date is chosen as you type (and the calendar follows), an
/// invalid one marks the field invalid, and leaving the field with text that is not a date restores the
/// chosen date.
///
/// Give it a `name` to submit the date as `YYYY-MM-DD` with a form; `required` makes the browser
/// refuse a form with no date. The popover is `open` / `default_open` / `on_open_change`.
#[component]
pub fn DatePicker(
    #[prop(into, optional)] value: MaybeProp<CalendarDate>,
    #[prop(into, optional)] default_value: MaybeProp<CalendarDate>,
    /// Called with the new value whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<CalendarDate>>>,
    /// Controlled open state. Leave unset to let it manage itself (see `default_open`).
    #[prop(into, optional)]
    open: MaybeProp<bool>,
    /// Initial open state when `open` is not given.
    #[prop(into, optional)]
    default_open: MaybeProp<bool>,
    /// Called with the new state whenever it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Shown while nothing is chosen. Defaults to `Pick a date`.
    #[prop(optional, into)]
    placeholder: Option<String>,
    #[prop(optional)] format: DateFormat,
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
    /// Form field name; the date is submitted as `YYYY-MM-DD`.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// Add a button that clears the date.
    #[prop(optional)]
    clearable: bool,
    /// A text field with a calendar button, instead of a button.
    #[prop(optional)]
    typed: bool,
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
    /// Accessible name of the button or text field.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let names = names.unwrap_or_default();
    let words = StoredValue::new(names.clone());
    let placeholder = StoredValue::new(placeholder.unwrap_or_else(|| "Pick a date".to_string()));
    let id = StoredValue::new(id);
    let aria_label = StoredValue::new(aria_label);

    let state = use_synced(value, default_value, |_: &Option<CalendarDate>| {});
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
    let commit = Callback::new(move |next: Option<CalendarDate>| {
        if state.get_untracked() != next {
            state.set(next);
            if let Some(cb) = on_value_change {
                cb.run(next);
            }
        }
    });
    let allowed = move |d: CalendarDate| {
        min.get_untracked().is_none_or(|m| d >= m)
            && max.get_untracked().is_none_or(|m| d <= m)
            && !is_date_disabled.is_some_and(|cb| cb.run(d))
    };

    let content = move || {
        let names = names.clone();
        view! {
            <PopoverContent
                class="vdl-date-picker__content"
                align=Align::Start
                label="Choose a date"
                on_open_auto_focus=Callback::new(move |ev: leptos::web_sys::Event| {
                    ev.prevent_default();
                    focus_tab_day();
                })
            >
                <Calendar
                    value=state
                    min=min
                    max=max
                    is_date_disabled=is_date_disabled
                        .unwrap_or_else(|| Callback::new(|_: CalendarDate| false))
                    deselectable=false
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
                    on_value_change=Callback::new(move |v: Option<CalendarDate>| {
                        commit.run(v);
                        if v.is_some() {
                            set_open.run(false);
                        }
                    })
                />
            </PopoverContent>
        }
    };

    let hidden = move || {
        let name = name.get();
        (name.is_some() || required).then(|| {
            date_field(
                name,
                state.get().map(|d| d.to_iso()).unwrap_or_default(),
                required,
            )
        })
    };

    if typed {
        let text = RwSignal::new(
            state
                .get_untracked()
                .map(|d| d.to_iso())
                .unwrap_or_default(),
        );
        let text_invalid = RwSignal::new(false);
        let wrapper = NodeRef::<html::Div>::new();
        let field = NodeRef::<html::Input>::new();
        // The text follows the chosen date, so a click in the calendar fills the field.
        Effect::new(move |_| {
            let iso = state.get().map(|d| d.to_iso()).unwrap_or_default();
            if text.get_untracked() != iso {
                text.set(iso);
            }
        });
        // The chosen date follows the text, once it is a date that can be chosen.
        Effect::new(move |_| {
            let t = text.get();
            if t.is_empty() {
                text_invalid.set(false);
                commit.run(None);
                return;
            }
            match CalendarDate::parse_iso(&t) {
                Some(d) if allowed(d) => {
                    text_invalid.set(false);
                    commit.run(Some(d));
                }
                Some(_) => text_invalid.set(true),
                None => text_invalid.set(t.len() >= 10),
            }
        });
        // Leaving the field (and the picker) with something that is not a date puts the date back.
        let on_focusout = move |ev: leptos::web_sys::FocusEvent| {
            let inside = ev
                .related_target()
                .and_then(|t| t.dyn_into::<leptos::web_sys::Node>().ok())
                .is_some_and(|node| {
                    wrapper
                        .get_untracked()
                        .is_some_and(|w| w.contains(Some(&node)))
                });
            if !inside {
                let iso = state
                    .get_untracked()
                    .map(|d| d.to_iso())
                    .unwrap_or_default();
                if text.get_untracked() != iso {
                    text.set(iso);
                }
                text_invalid.set(false);
            }
        };

        return view! {
            <div
                node_ref=wrapper
                class=format!("vdl-date-picker {class}")
                data-typed=""
                on:focusout=on_focusout
            >
                <Popover open=Signal::derive(move || Some(is_open.get())) on_open_change=set_open>
                    <div
                        class="vdl-input vdl-date-picker__field"
                        data-size=size.as_str()
                        data-disabled=move || disabled.get().then_some("")
                    >
                        <input
                            node_ref=field
                            class="vdl-input__field"
                            type="text"
                            inputmode="numeric"
                            autocomplete="off"
                            id=id.get_value()
                            placeholder="YYYY-MM-DD"
                            aria-label=aria_label.get_value().unwrap_or_else(|| "Date".to_string())
                            aria-invalid=move || {
                                (invalid.get() || text_invalid.get()).then_some("true")
                            }
                            disabled=move || disabled.get()
                            prop:value=move || text.get()
                            on:input=move |ev| text.set(event_target_value(&ev))
                        />
                        {move || {
                            (clearable && !text.get().is_empty() && !disabled.get())
                                .then(|| {
                                    view! {
                                        <button
                                            type="button"
                                            class="vdl-input__clear"
                                            aria-label="Clear date"
                                            tabindex="-1"
                                            on:click=move |_| {
                                                text.set(String::new());
                                                if let Some(el) = field.get_untracked() {
                                                    let _ = el.focus();
                                                }
                                            }
                                        >
                                            "×"
                                        </button>
                                    }
                                })
                        }}
                        <span class="vdl-input__affix">
                            <PopoverTrigger
                                variant=ButtonVariant::Ghost
                                size=Size::XSmall
                                icon_only=true
                                class="vdl-date-picker__button"
                                attr:aria-label="Choose a date"
                                attr:disabled=move || disabled.get()
                            >
                                {icons::calendar()}
                            </PopoverTrigger>
                        </span>
                    </div>
                    {content()}
                </Popover>
                {hidden}
            </div>
        }
        .into_any();
    }

    view! {
        <div
            class=format!("vdl-date-picker {class}")
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
                    attr:data-empty=move || state.get().is_none().then_some("")
                >
                    {icons::calendar()}
                    <span class="vdl-date-picker__value">
                        {move || match state.get() {
                            Some(d) => words.with_value(|n| n.format(d, format)),
                            None => placeholder.get_value(),
                        }}
                    </span>
                </PopoverTrigger>
                {content()}
            </Popover>
            {move || {
                (clearable && state.get().is_some() && !disabled.get())
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="vdl-date-picker__clear"
                                aria-label="Clear date"
                                on:click=move |_| commit.run(None)
                            >
                                {icons::x()}
                            </button>
                        }
                    })
            }}
            {hidden}
        </div>
    }
    .into_any()
}
