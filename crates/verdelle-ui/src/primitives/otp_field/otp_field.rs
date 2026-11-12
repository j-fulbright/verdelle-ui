use crate::primitives::core::id::use_id;
use crate::primitives::core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputValidationType {
    #[default]
    Numeric,
    Alphanumeric,
    Text,
}

impl InputValidationType {
    fn pattern(&self) -> &'static str {
        match self {
            InputValidationType::Numeric => r"[0-9]",
            InputValidationType::Alphanumeric => r"[a-zA-Z0-9]",
            InputValidationType::Text => r".",
        }
    }

    fn is_valid(&self, ch: char) -> bool {
        match self {
            InputValidationType::Numeric => ch.is_ascii_digit(),
            InputValidationType::Alphanumeric => ch.is_ascii_alphanumeric(),
            InputValidationType::Text => true,
        }
    }
}

#[derive(Clone, Debug)]
struct OtpFieldContextValue {
    value: RwSignal<Vec<String>>,
    max_length: usize,
    validation_type: InputValidationType,
    disabled: Signal<bool>,
    base_id: String,
}

/// Split `text` into one entry per slot (empty for the slots it does not reach).
fn to_slots(text: &str, max_length: usize) -> Vec<String> {
    let mut slots: Vec<String> = text.chars().take(max_length).map(String::from).collect();
    slots.resize(max_length, String::new());
    slots
}

/// Write `chars` into the slots from `start`, as many as fit. Returns the index to focus next.
fn fill_slots(slots: &mut [String], start: usize, chars: &[char]) -> usize {
    let mut last = start;
    for (k, c) in chars.iter().enumerate() {
        if let Some(slot) = slots.get_mut(start + k) {
            *slot = c.to_string();
            last = start + k;
        }
    }
    (last + 1).min(slots.len().saturating_sub(1))
}

fn focus_slot(base_id: &str, idx: usize) {
    use web_sys::wasm_bindgen::JsCast;
    if let Some(doc) = web_sys::window().and_then(|w| w.document())
        && let Some(el) = doc.get_element_by_id(&format!("{base_id}-input-{idx}"))
        && let Ok(el) = el.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = el.focus();
    }
}

#[component]
pub fn OneTimePasswordField(
    /// Number of input segments.
    #[prop(into, optional, default = 6.into())]
    max_length: MaybeProp<usize>,
    /// The code when initially rendered (uncontrolled).
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// The code (controlled). Shorter than `max_length` leaves the last slots empty.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// Fires with the whole code (empty slots left out) whenever it changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Fires when every slot is filled.
    #[prop(into, optional)]
    on_complete: Option<Callback<String>>,
    #[prop(into, optional)] validation_type: Option<InputValidationType>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let max_length = max_length.get_untracked().unwrap_or(6);
    let validation_type = validation_type.unwrap_or_default();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let base_id = use_id(None).get_untracked();

    let initial = value
        .get_untracked()
        .or_else(|| default_value.get_untracked())
        .unwrap_or_default();
    let value_signal = RwSignal::new(to_slots(&initial, max_length));

    // A controlled value overwrites the slots whenever it differs from them.
    Effect::new(move |_| {
        if let Some(v) = value.get() {
            let slots = to_slots(&v, max_length);
            if value_signal.get_untracked() != slots {
                value_signal.set(slots);
            }
        }
    });

    // Report changes (not the initial value).
    Effect::new(move |previous: Option<String>| {
        let combined: String = value_signal.get().iter().map(|s| s.as_str()).collect();
        if let Some(previous) = &previous
            && *previous != combined
        {
            if let Some(cb) = on_value_change {
                cb.run(combined.clone());
            }
            if combined.chars().count() == max_length
                && let Some(cb) = on_complete
            {
                cb.run(combined.clone());
            }
        }
        combined
    });

    let ctx = OtpFieldContextValue {
        value: value_signal,
        max_length,
        validation_type,
        disabled,
        base_id,
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="group"
                attr:data-disabled=move || disabled.get().then_some("")
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

/// One slot of the code; `index` is its position, from 0.
///
/// Typing replaces the slot and moves on, Backspace clears it (and steps back when it was
/// already empty), the arrow keys, Home and End move between slots, and pasting spreads the
/// text over the slots. Characters the validation type rejects are ignored.
#[component]
pub fn OneTimePasswordFieldInput(
    #[prop(into, optional)] index: Option<usize>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<OtpFieldContextValue>();
    let idx = index.unwrap_or(0);

    let input_value = Signal::derive(move || ctx.value.get().get(idx).cloned().unwrap_or_default());
    let base_id = ctx.base_id.clone();
    let id_input = ctx.base_id.clone();
    let id_keydown = ctx.base_id.clone();
    let id_paste = ctx.base_id.clone();
    let last = ctx.max_length.saturating_sub(1);

    view! {
        <VoidPrimitive element=html::input as_child=as_child node_ref=node_ref
            attr:id=format!("{}-input-{}", base_id, idx)
            attr:r#type="text"
            attr:inputmode=match ctx.validation_type {
                InputValidationType::Numeric => "numeric",
                _ => "text",
            }
            attr:pattern=ctx.validation_type.pattern()
            attr:autocomplete="one-time-code"
            prop:value=move || input_value.get()
            attr:disabled=move || ctx.disabled.get().then_some("")
            attr:data-vdl-otp-input=""
            attr:data-filled=move || (!input_value.get().is_empty()).then_some("")
            attr:aria-label=format!("Digit {} of {}", idx + 1, ctx.max_length)
            on:focus=move |event: leptos::ev::FocusEvent| {
                use web_sys::wasm_bindgen::JsCast;
                if let Some(input) = event.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                    input.select();
                }
            }
            on:input=move |event: leptos::ev::Event| {
                use web_sys::wasm_bindgen::JsCast;
                let Some(input) = event.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) else {
                    return;
                };
                let raw = input.value();
                let old = input_value.get_untracked().chars().next();
                let mut typed: Vec<char> = raw.chars().filter(|c| ctx.validation_type.is_valid(*c)).collect();
                // Typing into a filled slot leaves old and new characters side by side.
                if typed.len() == 2
                    && let Some(old) = old
                {
                    if typed[0] == old {
                        typed.remove(0);
                    } else if typed[1] == old {
                        typed.remove(1);
                    }
                }
                if typed.is_empty() {
                    if raw.is_empty() {
                        ctx.value.update(|v| v[idx].clear());
                    } else {
                        // Nothing acceptable was typed: put back what the slot holds.
                        input.set_value(&input_value.get_untracked());
                    }
                    return;
                }
                let mut next = idx;
                ctx.value.update(|v| next = fill_slots(v, idx, &typed));
                input.set_value(&input_value.get_untracked());
                if next != idx || idx < last {
                    focus_slot(&id_input, next);
                }
            }
            on:paste=move |event: web_sys::ClipboardEvent| {
                let Some(text) = event.clipboard_data().and_then(|d| d.get_data("text").ok()) else {
                    return;
                };
                event.prevent_default();
                let typed: Vec<char> = text.chars().filter(|c| ctx.validation_type.is_valid(*c)).collect();
                if typed.is_empty() {
                    return;
                }
                // A whole code replaces everything; shorter text goes in from this slot.
                let start = if typed.len() >= ctx.max_length { 0 } else { idx };
                let mut next = idx;
                ctx.value.update(|v| next = fill_slots(v, start, &typed));
                focus_slot(&id_paste, next);
            }
            on:keydown=move |event: KeyboardEvent| {
                match event.key().as_str() {
                    "Backspace" => {
                        event.prevent_default();
                        if !input_value.get_untracked().is_empty() {
                            ctx.value.update(|v| v[idx].clear());
                        } else if idx > 0 {
                            ctx.value.update(|v| v[idx - 1].clear());
                            focus_slot(&id_keydown, idx - 1);
                        }
                    }
                    "Delete" => {
                        event.prevent_default();
                        ctx.value.update(|v| v[idx].clear());
                    }
                    "ArrowLeft" if idx > 0 => {
                        event.prevent_default();
                        focus_slot(&id_keydown, idx - 1);
                    }
                    "ArrowRight" if idx < last => {
                        event.prevent_default();
                        focus_slot(&id_keydown, idx + 1);
                    }
                    "Home" => {
                        event.prevent_default();
                        focus_slot(&id_keydown, 0);
                    }
                    "End" => {
                        event.prevent_default();
                        focus_slot(&id_keydown, last);
                    }
                    _ => {}
                }
            }
        >
            {""}
        </VoidPrimitive>
    }
}

#[component]
pub fn OneTimePasswordFieldHiddenInput(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<OtpFieldContextValue>();
    let combined = Signal::derive(move || {
        ctx.value
            .get()
            .iter()
            .map(|s| s.as_str())
            .collect::<String>()
    });

    view! {
        <input
            node_ref=node_ref
            type="hidden"
            name=move || name.get().unwrap_or_default()
            value=move || combined.get()
            aria-hidden="true"
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_pad_and_truncate() {
        assert_eq!(to_slots("12", 4), ["1", "2", "", ""]);
        assert_eq!(to_slots("123456", 4), ["1", "2", "3", "4"]);
        assert_eq!(to_slots("", 2), ["", ""]);
    }

    #[test]
    fn fill_slots_spreads_and_stops_at_the_end() {
        let mut slots = to_slots("", 4);
        assert_eq!(fill_slots(&mut slots, 1, &['7', '8']), 3);
        assert_eq!(slots, ["", "7", "8", ""]);
        // Overflowing text is cut off, and focus stays on the last slot.
        assert_eq!(fill_slots(&mut slots, 2, &['1', '2', '3']), 3);
        assert_eq!(slots, ["", "7", "1", "2"]);
    }

    #[test]
    fn numeric_validation() {
        let v = InputValidationType::Numeric;
        assert!(v.is_valid('5'));
        assert!(v.is_valid('0'));
        assert!(!v.is_valid('a'));
        assert!(!v.is_valid(' '));
    }

    #[test]
    fn alphanumeric_validation() {
        let v = InputValidationType::Alphanumeric;
        assert!(v.is_valid('a'));
        assert!(v.is_valid('Z'));
        assert!(v.is_valid('5'));
        assert!(!v.is_valid(' '));
        assert!(!v.is_valid('-'));
    }

    #[test]
    fn text_validation() {
        let v = InputValidationType::Text;
        assert!(v.is_valid('a'));
        assert!(v.is_valid(' '));
        assert!(v.is_valid('!'));
    }

    #[test]
    fn validation_patterns() {
        assert_eq!(InputValidationType::Numeric.pattern(), r"[0-9]");
        assert_eq!(InputValidationType::Alphanumeric.pattern(), r"[a-zA-Z0-9]");
        assert_eq!(InputValidationType::Text.pattern(), r".");
    }
}
