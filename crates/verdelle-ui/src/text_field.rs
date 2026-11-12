//! Shared by the text fields (`Input`, `Textarea`, the input group controls): the Enter-key rule
//! and the auto-grow for a `<textarea>`.

use leptos::{html, prelude::*};

/// Whether a key press should count as "Enter" for `on_enter`: the Enter key, outside an IME
/// composition (where Enter confirms a candidate rather than submitting).
pub(crate) fn is_enter(key: &str, composing: bool) -> bool {
    key == "Enter" && !composing
}

/// Runs the keydown callbacks: `on_keydown` always, then `on_enter` for a plain Enter (no Shift,
/// so a textarea can still take a new line).
pub(crate) fn handle_keydown(
    ev: leptos::ev::KeyboardEvent,
    on_keydown: Option<Callback<leptos::ev::KeyboardEvent>>,
    on_enter: Option<Callback<()>>,
    shift_newline: bool,
) {
    let enter = is_enter(&ev.key(), ev.is_composing()) && !(shift_newline && ev.shift_key());
    if let Some(cb) = on_keydown {
        cb.run(ev);
    }
    if enter && let Some(cb) = on_enter {
        cb.run(());
    }
}

/// The `style` for a grow-limited textarea: the row cap as a CSS variable the stylesheet reads.
pub(crate) fn max_rows_style(max_rows: Option<u32>) -> Option<String> {
    max_rows.map(|n| format!("--vdl-max-rows: {}", n.max(1)))
}

/// Keeps a `<textarea>` as tall as its text in browsers without CSS `field-sizing` (the
/// stylesheet handles the rest): on every value change the height is reset and set to the scroll
/// height; the CSS `max-height` (from `max_rows`) clamps it. Browser only.
pub(crate) fn use_grow_fallback(
    field: NodeRef<html::Textarea>,
    value: Signal<String>,
    enabled: bool,
) {
    #[cfg(feature = "hydrate")]
    if enabled {
        use leptos::wasm_bindgen::JsCast;
        Effect::new(move |_| {
            value.track();
            let Some(el) = field.get() else { return };
            let el: &web_sys::HtmlElement = el.unchecked_ref();
            let supported = web_sys::window()
                .and_then(|w| w.get_computed_style(el).ok().flatten())
                .and_then(|s| s.get_property_value("field-sizing").ok())
                .is_some_and(|v| v == "content");
            if supported {
                return;
            }
            let style = el.style();
            let _ = style.set_property("height", "auto");
            let _ = style.set_property("height", &format!("{}px", el.scroll_height() + 2));
        });
    }
    #[cfg(not(feature = "hydrate"))]
    let _ = (field, value, enabled);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_counts_outside_composition_only() {
        assert!(is_enter("Enter", false));
        assert!(!is_enter("Enter", true));
        assert!(!is_enter("a", false));
    }

    #[test]
    fn max_rows_becomes_a_css_variable_and_never_zero() {
        assert_eq!(max_rows_style(None), None);
        assert_eq!(
            max_rows_style(Some(6)).as_deref(),
            Some("--vdl-max-rows: 6")
        );
        assert_eq!(
            max_rows_style(Some(0)).as_deref(),
            Some("--vdl-max-rows: 1")
        );
    }
}
