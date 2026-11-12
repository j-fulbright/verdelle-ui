use crate::theme_provider::{ThemeChoice, ThemeContext, ThemeSource};
use crate::themes::THEMES;
use leptos::prelude::*;

/// A theme chooser: one swatch card per theme. Not part of shadcn/ui, which picks
/// themes in its docs site and CLI; added for the web. [`crate::ThemeSwitcher`] is the
/// compact version and lists the same themes.
///
/// **With a [`crate::ThemeProvider`]** it needs no props: it lists the built-in themes, then
/// the ones declared with [`crate::ThemeStyle`], the provider's `preset`, the themes people
/// saved in the browser and the unsaved preview, shows the chosen one and changes it (the
/// list updates as themes are saved, renamed or deleted).
///
/// **Without one**, or with a `value`, it is controlled: `value` holds the selected theme slug
/// and is two-way, and the picker only edits that signal; applying it (`data-theme` on
/// `<html>`, persistence) is the caller's job. `themes` replaces the list with your own
/// `(slug, name)` pairs (the built-in ones when absent); a theme added that way must have CSS
/// scoped to `data-theme="<slug>"`.
///
/// Each card carries its own `data-theme`, so the swatch is drawn from that theme's
/// tokens (including the scoped CSS of registered and custom themes) in whatever mode the
/// page is in. The cards are native radio inputs, which gives arrow-key navigation, a single
/// tab stop and screen-reader semantics for free. `legend` is the group's accessible name and
/// is visually hidden.
#[component]
pub fn ThemePicker(
    /// The selected theme slug, read and written by the component. Default: the provider's.
    #[prop(optional, into)]
    value: Option<RwSignal<String>>,
    /// Replace the list of themes with these `(slug, name)` pairs.
    #[prop(optional, into)]
    themes: MaybeProp<Vec<(String, String)>>,
    #[prop(default = "Theme", into)] legend: &'static str,
) -> impl IntoView {
    let ctx = ThemeContext::try_use();
    // One radio group per picker, so several pickers on a page do not share a selection.
    let group = crate::primitives::core::id::use_id(None).get_untracked();
    let group = format!("vdl-theme-{group}");
    let own = RwSignal::new(crate::themes::DEFAULT_THEME.to_string());
    let selected: RwSignal<String> = match (value, ctx) {
        (Some(v), _) => v,
        (None, Some(c)) => c.theme,
        (None, None) => own,
    };

    let choices = Signal::derive(move || -> Vec<ThemeChoice> {
        if let Some(own) = themes.get() {
            return own
                .into_iter()
                .map(|(slug, name)| ThemeChoice {
                    slug,
                    name,
                    source: ThemeSource::BuiltIn,
                })
                .collect();
        }
        match ctx {
            Some(ctx) => ctx.choices(),
            None => THEMES
                .iter()
                .map(|t| ThemeChoice {
                    slug: t.slug.to_string(),
                    name: t.name.to_string(),
                    source: ThemeSource::BuiltIn,
                })
                .collect(),
        }
    });

    view! {
        <fieldset class="vdl-theme-picker">
            <legend class="vdl-sr-only">{legend}</legend>
            <div class="vdl-theme-picker__grid">
                <For
                    each=move || choices.get()
                    key=|c| (c.slug.clone(), c.name.clone())
                    children=move |c: ThemeChoice| {
                        let slug = StoredValue::new(c.slug.clone());
                        view! {
                            <label class="vdl-theme-card" data-theme=c.slug.clone()>
                                <input
                                    type="radio"
                                    class="vdl-theme-card__input"
                                    name=group.clone()
                                    value=c.slug.clone()
                                    prop:checked=move || selected.get() == slug.get_value()
                                    on:change=move |_| selected.set(slug.get_value())
                                />
                                <span class="vdl-theme-card__swatch" aria-hidden="true">
                                    <crate::theme_switcher::SwatchDots />
                                </span>
                                <span class="vdl-theme-card__name">{c.name.clone()}</span>
                            </label>
                        }
                    }
                />
            </div>
        </fieldset>
    }
}
