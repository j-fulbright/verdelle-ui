use crate::{Button, ButtonVariant, Size};
use crate::{Icon, IconName};
use leptos::prelude::*;

/// A light/dark toggle: an icon button that flips the `.dark` class on `<html>`. Not part of
/// shadcn/ui's registry (its docs site builds the same control from a dropdown).
///
/// Inside a [`crate::ThemeProvider`] it flips the provider's mode (light or dark, from what the
/// page shows now; the provider applies and stores it). Otherwise it persists nothing: pass
/// `value` to share the state with the app (which then stores it), or let it keep its own,
/// reading the class on `<html>` once after hydration. The icon follows the class on `<html>`
/// in CSS, so it is right on the first paint when the page has already set the class before
/// hydration. The class is only touched in the browser.
#[component]
pub fn ModeToggle(
    /// Whether dark mode is on. Two-way; optional.
    #[prop(optional)]
    value: Option<RwSignal<bool>>,
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional, into)]
    variant: Signal<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// Accessible name, for when the visible content is not text.
    #[prop(default = "Toggle dark mode".to_string(), into)]
    aria_label: String,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let theme = if value.is_none() {
        crate::ThemeContext::try_use()
    } else {
        None
    };
    let dark = value.unwrap_or_else(|| RwSignal::new(false));

    #[cfg(feature = "hydrate")]
    if theme.is_none() {
        let html = || document().document_element();
        if value.is_none() {
            // Runs first: adopt what the page already set.
            Effect::new(move |_| {
                if let Some(html) = html() {
                    dark.set(html.class_list().contains("dark"));
                }
            });
        }
        Effect::new(move |_| {
            let on = dark.get();
            if let Some(html) = html() {
                _ = html.class_list().toggle_with_force("dark", on);
            }
        });
    }

    view! {
        <Button
            variant=variant
            size=size
            icon_only=true
            class=format!("vdl-mode-toggle {class}")
            aria_label=aria_label
            on_click=move |_| match theme {
                Some(theme) => theme.toggle_mode(),
                None => dark.update(|d| *d = !*d),
            }
        >
            <Icon name=IconName::Sun class="vdl-mode-toggle__sun" />
            <Icon name=IconName::Moon class="vdl-mode-toggle__moon" />
        </Button>
    }
}
