use super::model::{Parsed, clamp_to, format_number, parse_number, round_to, step_value};
use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::primitive::{Primitive, VoidPrimitive};
use leptos::{
    context::Provider,
    ev::{KeyboardEvent, PointerEvent},
    html,
    prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/// How long a pressed increment or decrement button waits before it starts repeating, and the
/// gap between repeats.
const REPEAT_DELAY_MS: i32 = 400;
const REPEAT_INTERVAL_MS: i32 = 60;
/// Pixels of pointer travel per step in a scrub area.
const SCRUB_PIXELS_PER_STEP: f64 = 4.0;

/// Run `f` once after `ms` milliseconds; returns the timer id. The closure is built when the
/// timer is set, in the browser: building one while rendering would panic on the server.
fn set_timeout(f: impl FnOnce() + 'static, ms: i32) -> i32 {
    let callback = Closure::once_into_js(f);
    web_sys::window()
        .expect("window")
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), ms)
        .expect("set_timeout should succeed")
}

fn clear_timeout(id: i32) {
    if let Some(window) = web_sys::window() {
        window.clear_timeout_with_handle(id);
    }
}

/// How big a step is: the plain step, the small one (Alt) or the large one (Shift, PageUp,
/// PageDown).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Amount {
    Normal,
    Small,
    Large,
}

#[derive(Clone, Copy)]
struct NumberFieldContext {
    value: RwSignal<Option<f64>>,
    text: RwSignal<String>,
    /// True while the user has typed text that has not been committed (blur, Enter, a step).
    dirty: RwSignal<bool>,
    min: Signal<Option<f64>>,
    max: Signal<Option<f64>>,
    step: Signal<f64>,
    small_step: Signal<f64>,
    large_step: Signal<f64>,
    snap_on_step: Signal<bool>,
    /// Fixed number of decimals, when set.
    decimals: Signal<Option<usize>>,
    prefix: Signal<String>,
    suffix: Signal<String>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    on_value_change: Option<Callback<Option<f64>>>,
    on_value_commit: Option<Callback<Option<f64>>>,
    input_ref: AnyNodeRef,
    /// The timer of a button being held down.
    repeat_timer: RwSignal<Option<i32>>,
}

impl NumberFieldContext {
    fn amount(&self, amount: Amount) -> f64 {
        match amount {
            Amount::Normal => self.step.get_untracked(),
            Amount::Small => self.small_step.get_untracked(),
            Amount::Large => self.large_step.get_untracked(),
        }
    }

    fn format(&self, v: f64) -> String {
        format_number(
            v,
            self.decimals.get_untracked(),
            &self.prefix.get_untracked(),
            &self.suffix.get_untracked(),
        )
    }

    /// Set the value (not the text) and tell the owner.
    fn set_value(&self, v: Option<f64>) {
        if self.value.get_untracked() != v {
            self.value.set(v);
            if let Some(cb) = self.on_value_change {
                cb.run(v);
            }
        }
    }

    /// Settle the value: keep it within the bounds, round it to the fixed decimals, show it
    /// formatted and tell the owner it is final.
    fn commit(&self, v: Option<f64>) {
        let v = v.map(|v| {
            let v = clamp_to(v, self.min.get_untracked(), self.max.get_untracked());
            match self.decimals.get_untracked() {
                Some(d) => round_to(v, d),
                None => v,
            }
        });
        self.set_value(v);
        self.dirty.set(false);
        self.text.set(v.map(|v| self.format(v)).unwrap_or_default());
        if let Some(cb) = self.on_value_commit {
            cb.run(v);
        }
    }

    /// Take what is typed as the new value; text that is not a number is turned back to the
    /// last good text. Bounds are not applied until the field is committed.
    fn read_text(&self, typed: &str) -> bool {
        match parse_number(
            typed,
            &self.prefix.get_untracked(),
            &self.suffix.get_untracked(),
        ) {
            Parsed::Invalid => false,
            Parsed::Empty => {
                self.set_value(None);
                true
            }
            Parsed::Partial => true,
            Parsed::Number(v) => {
                self.set_value(Some(v));
                true
            }
        }
    }

    /// Commit whatever is typed: an unfinished or empty entry empties the field, anything
    /// else is clamped.
    fn commit_typed(&self) {
        if !self.dirty.get_untracked() {
            return;
        }
        let typed = self.text.get_untracked();
        match parse_number(
            &typed,
            &self.prefix.get_untracked(),
            &self.suffix.get_untracked(),
        ) {
            Parsed::Number(v) => self.commit(Some(v)),
            _ => self.commit(None),
        }
    }

    fn can_edit(&self) -> bool {
        !self.disabled.get_untracked() && !self.read_only.get_untracked()
    }

    /// One step up (`direction` 1) or down (-1).
    fn step(&self, direction: i8, amount: Amount) {
        if !self.can_edit() {
            return;
        }
        // A pending typed value counts: step from what the field shows.
        self.commit_typed();
        let snap = self.snap_on_step.get_untracked() && amount == Amount::Normal;
        let next = step_value(
            self.value.get_untracked(),
            direction,
            self.amount(amount),
            self.min.get_untracked(),
            self.max.get_untracked(),
            snap,
        );
        self.commit(Some(next));
    }

    fn at_limit(&self, direction: i8) -> bool {
        let Some(v) = self.value.get() else {
            return false;
        };
        if direction > 0 {
            self.max.get().is_some_and(|m| v >= m)
        } else {
            self.min.get().is_some_and(|m| v <= m)
        }
    }

    fn focus_input(&self) {
        if let Some(input) = self
            .input_ref
            .get_untracked()
            .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = input.focus();
        }
    }

    fn stop_repeat(&self) {
        if let Some(id) = self.repeat_timer.try_get_untracked().flatten() {
            clear_timeout(id);
            self.repeat_timer.try_set(None);
        }
    }

    fn schedule_repeat(self, direction: i8, delay: i32) {
        let id = set_timeout(
            move || {
                if self.repeat_timer.try_get_untracked().flatten().is_none() {
                    return;
                }
                self.step(direction, Amount::Normal);
                self.schedule_repeat(direction, REPEAT_INTERVAL_MS);
            },
            delay,
        );
        self.repeat_timer.try_set(Some(id));
    }
}

/// The root: holds the value and the settings every part reads.
///
/// The value is a number or empty (`None`). Pass `value` (a signal that is read and written)
/// to control it, or `default_value` to start somewhere. `on_value_change` fires as the value
/// changes (also while typing); `on_value_commit` fires when a value is settled (blur, Enter,
/// a step or the end of a scrub).
///
/// `step` (1) is the arrow-key and button step, `small_step` (0.1, with Alt) and `large_step`
/// (10, with Shift, PageUp and PageDown) the other two. `min` and `max` clamp stepping and
/// committed values. `decimals` shows (and rounds to) a fixed number of places. `prefix` and
/// `suffix` are shown around the number and ignored when typed. With `snap_on_step`, a step
/// lands on a multiple of `step` counted from `min` (or 0). `name` submits the numeric value
/// with a form, through a hidden input.
#[component]
pub fn NumberField(
    #[prop(optional)] value: Option<RwSignal<Option<f64>>>,
    #[prop(into, optional)] default_value: MaybeProp<f64>,
    #[prop(into, optional)] on_value_change: Option<Callback<Option<f64>>>,
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
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked()));
    let positive = |v: MaybeProp<f64>, fallback: f64| {
        Signal::derive(move || v.get().filter(|s| *s > 0.0).unwrap_or(fallback))
    };
    let ctx = NumberFieldContext {
        value,
        text: RwSignal::new(String::new()),
        dirty: RwSignal::new(false),
        min: Signal::derive(move || min.get()),
        max: Signal::derive(move || max.get()),
        step: positive(step, 1.0),
        small_step: positive(small_step, 0.1),
        large_step: positive(large_step, 10.0),
        snap_on_step: Signal::derive(move || snap_on_step.get().unwrap_or(false)),
        decimals: Signal::derive(move || decimals.get()),
        prefix: Signal::derive(move || prefix.get().unwrap_or_default()),
        suffix: Signal::derive(move || suffix.get().unwrap_or_default()),
        disabled: Signal::derive(move || disabled.get().unwrap_or(false)),
        read_only: Signal::derive(move || read_only.get().unwrap_or(false)),
        on_value_change,
        on_value_commit,
        input_ref: AnyNodeRef::new(),
        repeat_timer: RwSignal::new(None),
    };
    // The text starts as the formatted value (so the server renders it) and follows later
    // changes to the value unless the user is in the middle of typing.
    ctx.text.set(
        value
            .get_untracked()
            .map(|v| ctx.format(v))
            .unwrap_or_default(),
    );
    Effect::new(move |_| {
        let v = value.get();
        let (_, _, _) = (ctx.decimals.get(), ctx.prefix.get(), ctx.suffix.get());
        if !ctx.dirty.get_untracked() {
            ctx.text.set(v.map(|v| ctx.format(v)).unwrap_or_default());
        }
    });
    on_cleanup(move || ctx.stop_repeat());

    let hidden_value = move || {
        value
            .get()
            .map(|v| format_number(v, decimals.get(), "", ""))
            .unwrap_or_default()
    };

    view! {
        <Provider value=ctx>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-disabled=move || ctx.disabled.get().then_some("")
                attr:data-readonly=move || ctx.read_only.get().then_some("")
            >
                {children.with_value(|children| children())}
                {move || {
                    name.get()
                        .map(|name| {
                            view! { <input type="hidden" name=name prop:value=hidden_value /> }
                        })
                }}
            </Primitive>
        </Provider>
    }
}

/// The text field: `role="spinbutton"` with the value, limits and a text form of the value.
///
/// Typing accepts digits, one `.` and a leading sign; other characters are refused. Blur and
/// Enter settle the value (clamped to `min` and `max`, rounded to `decimals`); Escape-free:
/// an empty or unfinished entry empties the field. Keys: Up and Down step, with Shift or
/// PageUp/PageDown the large step, with Alt the small step; Home and End go to `min` and
/// `max` when they are set.
#[component]
pub fn NumberFieldInput(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<NumberFieldContext>();
    let refs = use_composed_refs(vec![node_ref, ctx.input_ref]);

    view! {
        <VoidPrimitive
            element=html::input
            as_child=as_child
            node_ref=refs
            attr:r#type="text"
            attr:role="spinbutton"
            attr:inputmode="decimal"
            attr:autocomplete="off"
            attr:spellcheck="false"
            attr:aria-valuenow=move || ctx.value.get().map(|v| v.to_string())
            attr:aria-valuemin=move || ctx.min.get().map(|v| v.to_string())
            attr:aria-valuemax=move || ctx.max.get().map(|v| v.to_string())
            attr:aria-valuetext=move || ctx.value.get().map(|v| {
                format_number(v, ctx.decimals.get(), &ctx.prefix.get(), &ctx.suffix.get())
            })
            attr:data-disabled=move || ctx.disabled.get().then_some("")
            attr:disabled=move || ctx.disabled.get()
            attr:readonly=move || ctx.read_only.get()
            prop:value=move || ctx.text.get()
            on:input=move |ev: leptos::ev::Event| {
                let Some(input) = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                else {
                    return;
                };
                let typed = input.value();
                if ctx.read_text(&typed) {
                    ctx.dirty.set(true);
                    ctx.text.set(typed);
                } else {
                    // Not a number: put the last good text back.
                    input.set_value(&ctx.text.get_untracked());
                }
            }
            on:blur=move |_| ctx.commit_typed()
            on:keydown=move |event: KeyboardEvent| {
                let large = event.shift_key();
                let small = event.alt_key();
                let amount = if small {
                    Amount::Small
                } else if large {
                    Amount::Large
                } else {
                    Amount::Normal
                };
                match event.key().as_str() {
                    "ArrowUp" => {
                        event.prevent_default();
                        ctx.step(1, amount);
                    }
                    "ArrowDown" => {
                        event.prevent_default();
                        ctx.step(-1, amount);
                    }
                    "PageUp" => {
                        event.prevent_default();
                        ctx.step(1, Amount::Large);
                    }
                    "PageDown" => {
                        event.prevent_default();
                        ctx.step(-1, Amount::Large);
                    }
                    "Home" if ctx.min.get_untracked().is_some() && ctx.can_edit() => {
                        event.prevent_default();
                        ctx.commit(ctx.min.get_untracked());
                    }
                    "End" if ctx.max.get_untracked().is_some() && ctx.can_edit() => {
                        event.prevent_default();
                        ctx.commit(ctx.max.get_untracked());
                    }
                    "Enter" => ctx.commit_typed(),
                    _ => {}
                }
            }
        >
            {""}
        </VoidPrimitive>
    }
}

/// A button that steps the value. Press and hold repeats after a short wait. It does not take
/// focus from the input and is not a tab stop (the input has the keyboard); it is disabled
/// at the limit.
#[component]
fn NumberFieldStepper(
    direction: i8,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let label = if direction > 0 {
        "Increase"
    } else {
        "Decrease"
    };
    let ctx = expect_context::<NumberFieldContext>();
    let unavailable = Signal::derive(move || {
        ctx.disabled.get() || ctx.read_only.get() || ctx.at_limit(direction)
    });
    let start = move |event: PointerEvent| {
        if unavailable.get_untracked() || event.button() != 0 {
            return;
        }
        // Keep the focus where it is, and show the value being stepped.
        event.prevent_default();
        ctx.focus_input();
        ctx.step(direction, Amount::Normal);
        ctx.stop_repeat();
        // Holding the button repeats until the pointer leaves, lifts or the limit is reached.
        ctx.schedule_repeat(direction, REPEAT_DELAY_MS);
    };
    let stop = move |_: PointerEvent| ctx.stop_repeat();

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:tabindex="-1"
            attr:aria-label=label
            attr:data-disabled=move || unavailable.get().then_some("")
            attr:disabled=move || unavailable.get()
            on:pointerdown=start
            on:pointerup=stop
            on:pointerleave=stop
            on:pointercancel=stop
            on:contextmenu=move |event: leptos::ev::MouseEvent| event.prevent_default()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn NumberFieldIncrement(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <NumberFieldStepper direction=1 as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </NumberFieldStepper>
    }
}

#[component]
pub fn NumberFieldDecrement(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <NumberFieldStepper direction=-1 as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </NumberFieldStepper>
    }
}

/// An area (usually the field's label) that changes the value when dragged sideways: one
/// `step` for every few pixels, to the right up and to the left down. The pointer is captured,
/// so the drag keeps working outside the area.
#[component]
pub fn NumberFieldScrubArea(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<NumberFieldContext>();
    // Pointer x at the last step, while dragging.
    let anchor = RwSignal::new(None::<f64>);

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-scrubbing=move || anchor.get().is_some().then_some("")
            attr:style="touch-action:none;user-select:none;-webkit-user-select:none"
            on:pointerdown=move |event: PointerEvent| {
                if !ctx.can_edit() || event.button() != 0 {
                    return;
                }
                if let Some(target) = event
                    .current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let _ = target.set_pointer_capture(event.pointer_id());
                }
                ctx.commit_typed();
                anchor.set(Some(f64::from(event.client_x())));
            }
            on:pointermove=move |event: PointerEvent| {
                let Some(from) = anchor.get_untracked() else {
                    return;
                };
                let x = f64::from(event.client_x());
                let steps = ((x - from) / SCRUB_PIXELS_PER_STEP).trunc();
                if steps == 0.0 {
                    return;
                }
                let direction = if steps > 0.0 { 1 } else { -1 };
                let amount = if event.shift_key() {
                    Amount::Large
                } else if event.alt_key() {
                    Amount::Small
                } else {
                    Amount::Normal
                };
                for _ in 0..steps.abs() as usize {
                    ctx.step(direction, amount);
                }
                anchor.set(Some(from + steps * SCRUB_PIXELS_PER_STEP));
            }
            on:pointerup=move |_| anchor.set(None)
            on:pointercancel=move |_| anchor.set(None)
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
