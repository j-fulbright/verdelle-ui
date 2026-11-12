use crate::{ButtonType, ButtonVariant, Size};
use leptos::{ev, prelude::*};

/// What `rel` a link button gets: the caller's, or `noopener noreferrer` for a new-tab link.
fn link_rel(rel: Option<String>, target: Option<&str>) -> Option<String> {
    rel.filter(|r| !r.is_empty())
        .or_else(|| (target == Some("_blank")).then(|| "noopener noreferrer".to_string()))
}

/// A button, styled as shadcn/ui's Button (default "nova" style).
///
/// States are carried by `data-*` attributes and styled in `components.css`.
/// `loading` keeps the variant's colours and the label, dims the button, shows a spinner and
/// ignores clicks, including a form submit (it stays focusable); shadcn has no loading state, so
/// this is ours. `disabled` also sets the native attribute. `selected` gives the hover look and
/// sets `aria-pressed`, for toggle buttons.
///
/// Sizes are xs 24px, sm 28px, md 32px (default) and lg 36px. Pass `icon_only`
/// for a square button of that size.
///
/// With `href` the button is an `<a>` with the same classes, attributes and states (shadcn's
/// `render={<a/>}` / `asChild`). Whether it is a link is decided when the component is built; the
/// `href` itself may change afterwards. A disabled or loading link has no `href`, gets
/// `aria-disabled="true"` and `tabindex="-1"`, and a click does not navigate. A link with
/// `target="_blank"` gets `rel="noopener noreferrer"` unless `rel` is given. `kind` applies to
/// the `<button>` form only.
///
/// `href`, `target`, `rel`, `title` and `aria_label` take a plain value, an `Option`, or a signal
/// of an `Option`, so a caller can pass `Some`/`None` without two view branches.
#[component]
pub fn Button(
    /// Visual variant: see `ButtonVariant`.
    #[prop(optional, into)]
    variant: Signal<ButtonVariant>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    #[prop(optional, into)] icon_only: Signal<bool>,
    #[prop(optional, into)] selected: Signal<bool>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Shows a spinner beside the label and ignores clicks until it is false again.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// The HTML `type` of a `<button>`: `Button` (default), `Submit` or `Reset`.
    #[prop(optional, into)]
    kind: Signal<ButtonType>,
    /// Renders an `<a>` that goes here. See the component docs.
    #[prop(optional, into)]
    href: MaybeProp<String>,
    /// Where a link opens: `_blank` for a new tab, and so on. Links only.
    #[prop(optional, into)]
    target: MaybeProp<String>,
    /// A link's `rel`. Default for `target="_blank"`: `noopener noreferrer`. Links only.
    #[prop(optional, into)]
    rel: MaybeProp<String>,
    /// Tooltip text (the native `title`); say what the button does when its label cannot.
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// Extra classes, for layout (for example Tailwind's `w-full` or `flex-1`). They
    /// override the button's own styles.
    #[prop(optional, into)]
    class: String,
    /// Accessible name for buttons whose visible content is not text.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Called on click.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// The content.
    children: Children,
) -> impl IntoView {
    let flag = |s: Signal<bool>| move || s.get().then_some("");
    let inert = move || disabled.get() || loading.get();
    let class = format!("vdl-btn {class}");
    let is_link = href.get_untracked().is_some();
    let click = move |ev: ev::MouseEvent| {
        if inert() {
            // A submit button would submit and a link would navigate.
            ev.prevent_default();
            return;
        }
        if let Some(cb) = on_click {
            cb.run(ev);
        }
    };

    if is_link {
        view! {
            <a
                class=class
                data-variant=move || variant.get().as_str()
                data-size=move || size.get().as_str()
                data-icon-only=flag(icon_only)
                data-selected=flag(selected)
                data-disabled=flag(disabled)
                data-loading=flag(loading)
                href=move || if inert() { None } else { href.get() }
                target=move || target.get()
                rel=move || link_rel(rel.get(), target.get().as_deref())
                title=move || title.get()
                aria-disabled=move || inert().then_some("true")
                aria-busy=move || loading.get().then_some("true")
                aria-current=move || selected.get().then_some("true")
                aria-label=move || aria_label.get()
                tabindex=move || inert().then_some("-1")
                on:click=click
            >
                <span class="vdl-btn__content">{children()}</span>
            </a>
        }
        .into_any()
    } else {
        view! {
            <button
                type=move || kind.get().as_str()
                class=class
                data-variant=move || variant.get().as_str()
                data-size=move || size.get().as_str()
                data-icon-only=flag(icon_only)
                data-selected=flag(selected)
                data-disabled=flag(disabled)
                data-loading=flag(loading)
                disabled=move || disabled.get()
                aria-busy=move || loading.get().then_some("true")
                aria-pressed=move || selected.get().then_some("true")
                aria-label=move || aria_label.get()
                title=move || title.get()
                on:click=click
            >
                <span class="vdl-btn__content">{children()}</span>
            </button>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_tab_link_gets_noopener_unless_rel_is_given() {
        assert_eq!(
            link_rel(None, Some("_blank")).as_deref(),
            Some("noopener noreferrer")
        );
        assert_eq!(
            link_rel(Some("nofollow".into()), Some("_blank")).as_deref(),
            Some("nofollow")
        );
        assert_eq!(
            link_rel(Some(String::new()), Some("_blank")).as_deref(),
            Some("noopener noreferrer")
        );
        assert_eq!(link_rel(None, None), None);
        assert_eq!(link_rel(None, Some("_self")), None);
    }
}
