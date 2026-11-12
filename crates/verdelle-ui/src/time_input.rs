use crate::Size;
use crate::calendar_state::use_synced;
use crate::date_picker::date_field;
use crate::primitives::time_field::{
    Fields, HourCycle, Segment, in_range, segment_range, step, type_digit, type_period,
};
use leptos::{ev, html, prelude::*};

/// The text a segment shows: two digits, or empty while it has no value (the placeholder shows).
fn segment_text(segment: Segment, fields: &Fields) -> String {
    match segment {
        Segment::Hour => fields.hour.map(|h| format!("{h:02}")).unwrap_or_default(),
        Segment::Minute => fields.minute.map(|m| format!("{m:02}")).unwrap_or_default(),
        Segment::Period => fields
            .pm
            .map(|pm| if pm { "PM" } else { "AM" }.to_string())
            .unwrap_or_default(),
    }
}

/// A time of day entered in segments (hour, minute and, on a 12-hour clock, AM/PM), in the
/// Input look. The value is `HH:MM` on a 24-hour clock (what `<input type="time">` and a form
/// submit), whatever the clock shown; it is `None` until every segment is filled. Verdelle-original
/// (shadcn has no time field; shadcn's examples use a native `type="time"` input), built like
/// [`crate::DatePicker`] and [`crate::NumberField`]: each segment is a real text input with
/// `role="spinbutton"`, so it works with a touch keyboard, and the pure logic is tested.
///
/// Keyboard: type digits (a first digit that cannot start a longer number moves on by itself),
/// Up and Down step the focused segment (wrapping; minutes step by `minute_step`), Left and Right
/// move between segments, Home and End jump to the lowest and highest value, Backspace and Delete
/// clear the segment, `a` and `p` set AM and PM, Tab moves on as usual.
///
/// `value` / `default_value` / `on_value_change` work as on [`crate::DatePicker`]: pass a signal
/// to control it, and it follows when the signal changes, including to empty; `on_value_change`
/// gets `Some("14:30")` once complete and `None` when a segment is cleared. `hour_cycle` is
/// `HourCycle::H24` (default) or `H12`. `min` and `max` (`HH:MM`) do not block typing, as the
/// native input does not: a time outside them marks the field invalid (`aria-invalid`, the
/// destructive border); a `min` later than `max` is a range across midnight. `name` submits the
/// `HH:MM` value with a form and `required` makes the browser refuse a form with no time.
///
/// Not provided: seconds, a time zone, a popup list of times (compose a [`crate::Select`] for
/// that) and translated segment names (the accessible names are English).
#[component]
pub fn TimeInput(
    /// The value as `HH:MM` (24-hour). Pass a signal to control it.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Initial value when `value` is not given.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called with `Some("HH:MM")` when every segment is filled and `None` when one is cleared.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<String>>>,
    /// The clock shown: 24-hour (default) or 12-hour with an AM/PM segment.
    #[prop(optional, into)]
    hour_cycle: Signal<HourCycle>,
    /// The earliest time (`HH:MM`). A time before it is marked invalid.
    #[prop(into, optional)]
    min: MaybeProp<String>,
    /// The latest time (`HH:MM`). A time after it is marked invalid.
    #[prop(into, optional)]
    max: MaybeProp<String>,
    /// What Up and Down change the minute by (default 1); it moves to the next multiple.
    #[prop(default = 1)]
    minute_step: u8,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Focusable and copyable, but rejects edits. Keeps the normal appearance.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Marks it invalid: `aria-invalid` and the destructive border and ring.
    #[prop(optional, into)]
    invalid: Signal<bool>,
    /// Marks it as required: the browser refuses a form with no time.
    #[prop(optional)]
    required: bool,
    /// Form field name; the time is submitted as `HH:MM`.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    /// The `id` of the hour segment, so a [`crate::Label`] can point at it.
    #[prop(optional, into)]
    id: Option<String>,
    /// Accessible name of the whole field.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// The `id` of the element that describes the field (a hint or an error message).
    #[prop(optional, into)]
    aria_describedby: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let minute_step = minute_step.max(1);
    let aria_describedby = StoredValue::new(aria_describedby);
    let fields = RwSignal::new(Fields::default());
    let state = use_synced(value, default_value, move |next: &Option<String>| {
        fields.set(
            next.as_deref()
                .map(|v| Fields::from_value(v, hour_cycle.get_untracked()))
                .unwrap_or_default(),
        );
    });
    fields.set(
        state
            .get_untracked()
            .as_deref()
            .map(|v| Fields::from_value(v, hour_cycle.get_untracked()))
            .unwrap_or_default(),
    );

    // Switching the clock re-reads the value into the segments.
    let first_cycle = StoredValue::new(true);
    Effect::new(move |_| {
        let cycle = hour_cycle.get();
        if first_cycle.get_value() {
            first_cycle.set_value(false);
            return;
        }
        let current = state.get_untracked();
        fields.set(
            current
                .as_deref()
                .map(|v| Fields::from_value(v, cycle))
                .unwrap_or_default(),
        );
    });

    // Digits typed so far into the focused segment that a second digit could extend.
    let pending = StoredValue::new(None::<u8>);

    let hour_ref = NodeRef::<html::Input>::new();
    let minute_ref = NodeRef::<html::Input>::new();
    let period_ref = NodeRef::<html::Input>::new();

    let editable = move || !disabled.get_untracked() && !readonly.get_untracked();
    let node = move |segment: Segment| match segment {
        Segment::Hour => hour_ref,
        Segment::Minute => minute_ref,
        Segment::Period => period_ref,
    };
    // The segments on screen, in order.
    let order = move || {
        let mut order = vec![Segment::Hour, Segment::Minute];
        if hour_cycle.get_untracked() == HourCycle::H12 {
            order.push(Segment::Period);
        }
        order
    };
    let focus = move |segment: Segment| {
        if let Some(el) = node(segment).get_untracked() {
            let _ = el.focus();
        }
    };
    let focus_neighbour = move |from: Segment, delta: i32| {
        let order = order();
        if let Some(i) = order.iter().position(|s| *s == from) {
            let next = i as i32 + delta;
            if next >= 0 && (next as usize) < order.len() {
                focus(order[next as usize]);
            }
        }
    };

    // Publish the value when it changed.
    let commit = move || {
        let next = fields.get_untracked().to_value(hour_cycle.get_untracked());
        if next != state.get_untracked() {
            state.set(next.clone());
            if let Some(cb) = on_value_change {
                cb.run(next);
            }
        }
    };
    let set_segment = move |segment: Segment, value: Option<u8>| {
        fields.update(|f| match segment {
            Segment::Hour => f.hour = value,
            Segment::Minute => f.minute = value,
            Segment::Period => f.pm = value.map(|v| v == 1),
        });
        commit();
    };
    let current_of = move |segment: Segment| {
        let f = fields.get_untracked();
        match segment {
            Segment::Hour => f.hour,
            Segment::Minute => f.minute,
            Segment::Period => f.pm.map(u8::from),
        }
    };

    let on_keydown = move |segment: Segment, ev: ev::KeyboardEvent| {
        if ev.ctrl_key() || ev.meta_key() || ev.alt_key() {
            return;
        }
        let cycle = hour_cycle.get_untracked();
        match ev.key().as_str() {
            "ArrowLeft" => {
                ev.prevent_default();
                focus_neighbour(segment, -1);
            }
            "ArrowRight" => {
                ev.prevent_default();
                focus_neighbour(segment, 1);
            }
            key @ ("ArrowUp" | "ArrowDown") => {
                ev.prevent_default();
                if editable() {
                    pending.set_value(None);
                    let delta = if key == "ArrowUp" { 1 } else { -1 };
                    let next = step(segment, cycle, current_of(segment), delta, minute_step);
                    set_segment(segment, Some(next));
                }
            }
            "Home" | "End" => {
                ev.prevent_default();
                if editable() {
                    pending.set_value(None);
                    let (lo, hi) = segment_range(segment, cycle);
                    set_segment(segment, Some(if ev.key() == "Home" { lo } else { hi }));
                }
            }
            "Backspace" | "Delete" => {
                ev.prevent_default();
                if editable() {
                    pending.set_value(None);
                    set_segment(segment, None);
                }
            }
            _ => {}
        }
    };

    // Typing goes through `beforeinput`, which a touch keyboard also fires, and is always
    // cancelled: the segment shows what the model says, not what the browser typed.
    let on_beforeinput = move |segment: Segment, ev: web_sys::InputEvent| {
        ev.prevent_default();
        if !editable() {
            return;
        }
        let cycle = hour_cycle.get_untracked();
        match ev.input_type().as_str() {
            "deleteContentBackward" | "deleteContentForward" => {
                pending.set_value(None);
                set_segment(segment, None);
            }
            "insertText" | "insertCompositionText" => {
                let Some(ch) = ev.data().and_then(|d| d.chars().last()) else {
                    return;
                };
                if segment == Segment::Period {
                    if let Some(pm) = type_period(ch) {
                        set_segment(Segment::Period, Some(u8::from(pm)));
                        focus_neighbour(segment, 1);
                    }
                    return;
                }
                if let Some(digit) = ch.to_digit(10) {
                    let typed = type_digit(segment, cycle, pending.get_value(), digit as u8);
                    pending.set_value(typed.pending);
                    if let Some(value) = typed.value {
                        set_segment(segment, Some(value));
                    }
                    if typed.advance {
                        focus_neighbour(segment, 1);
                    }
                } else if ch == ':' || ch == ' ' {
                    focus_neighbour(segment, 1);
                } else if cycle == HourCycle::H12
                    && let Some(pm) = type_period(ch)
                {
                    // "a" or "p" typed in the hour or minute sets the period.
                    set_segment(Segment::Period, Some(u8::from(pm)));
                }
            }
            _ => {}
        }
    };

    let out_of_range = move || {
        state
            .get()
            .is_some_and(|v| !in_range(&v, min.get().as_deref(), max.get().as_deref()))
    };
    let is_invalid = move || invalid.get() || out_of_range();
    let flag = move |s: Signal<bool>| s.get().then_some("");

    let segment_input = move |segment: Segment,
                              label: &'static str,
                              placeholder: &'static str,
                              input_id: Option<String>| {
        let (lo, hi) = segment_range(segment, hour_cycle.get_untracked());
        view! {
            <input
                node_ref=node(segment)
                id=input_id
                type="text"
                role="spinbutton"
                class="vdl-time-input__segment"
                data-segment=match segment {
                    Segment::Hour => "hour",
                    Segment::Minute => "minute",
                    Segment::Period => "period",
                }
                inputmode=if segment == Segment::Period { "text" } else { "numeric" }
                autocomplete="off"
                autocapitalize="off"
                spellcheck="false"
                enterkeyhint="next"
                placeholder=placeholder
                aria-label=label
                aria-valuemin=if segment == Segment::Period { None } else { Some(lo.to_string()) }
                aria-valuemax=if segment == Segment::Period { None } else { Some(hi.to_string()) }
                aria-valuenow=move || {
                    let f = fields.get();
                    match segment {
                        Segment::Hour => f.hour,
                        Segment::Minute => f.minute,
                        Segment::Period => None,
                    }
                    .map(|v| v.to_string())
                }
                aria-valuetext=move || {
                    let text = segment_text(segment, &fields.get());
                    if text.is_empty() { "Empty".to_string() } else { text }
                }
                aria-invalid=move || is_invalid().then_some("true")
                aria-describedby=aria_describedby.get_value()
                disabled=move || disabled.get()
                readonly=move || readonly.get()
                prop:value=move || segment_text(segment, &fields.get())
                on:keydown=move |ev| on_keydown(segment, ev)
                on:beforeinput=move |ev: web_sys::InputEvent| on_beforeinput(segment, ev)
                on:focus=move |ev| {
                    pending.set_value(None);
                    // Select the text so the segment reads as highlighted.
                    let el = event_target::<web_sys::HtmlInputElement>(&ev);
                    el.select();
                }
                on:blur=move |_| pending.set_value(None)
                // A tap or click on a focused segment would put the caret in the text.
                on:mouseup=move |ev| {
                    ev.prevent_default();
                    let el = event_target::<web_sys::HtmlInputElement>(&ev);
                    el.select();
                }
            />
        }
    };

    view! {
        <div
            class=format!("vdl-input vdl-time-input {class}")
            role="group"
            aria-label=aria_label.unwrap_or_else(|| "Time".to_string())
            data-size=move || size.get().as_str()
            data-disabled=move || flag(disabled)
            data-invalid=move || is_invalid().then_some("")
            data-hour-cycle=move || match hour_cycle.get() {
                HourCycle::H24 => "24",
                HourCycle::H12 => "12",
            }
            // A click on the frame (not a segment) starts at the hour.
            on:click=move |ev: ev::MouseEvent| {
                if ev.target() == ev.current_target() && editable() {
                    focus(Segment::Hour);
                }
            }
        >
            {segment_input(Segment::Hour, "Hour", "--", id)}
            <span class="vdl-time-input__separator" aria-hidden="true">":"</span>
            {segment_input(Segment::Minute, "Minutes", "--", None)}
            {move || {
                (hour_cycle.get() == HourCycle::H12)
                    .then(|| segment_input(Segment::Period, "AM or PM", "--", None))
            }}
            {move || date_field(name.get(), state.get().unwrap_or_default(), required)}
        </div>
    }
}
