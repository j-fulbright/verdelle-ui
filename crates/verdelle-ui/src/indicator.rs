use crate::Size;
use leptos::prelude::*;

/// A count or dot marker overlaid on another element, such as an unread count
/// on an icon. shadcn/ui has no equivalent; see `NAMING.md`.
///
/// With `dot` it always shows a small dot; otherwise it shows `count`, capped as
/// `max+` and hidden when the count is zero. The marker is `aria-hidden` and
/// ignores the pointer, so give the wrapped element its own accessible name
/// (for example "Notifications, 3 unread"). An icon variant is not provided yet.
#[component]
pub fn Indicator(
    #[prop(optional, into)] count: Signal<usize>,
    #[prop(default = 99)] max: usize,
    #[prop(optional, into)] dot: Signal<bool>,
    /// Any CSS color for the marker background. Defaults to the theme's destructive colour.
    #[prop(optional, into)]
    color: Option<String>,
    /// Any CSS color for the number. Defaults to white, which suits the default (destructive) marker; set it
    /// whenever `color` is a theme colour that flips between light and dark, for example `var(--primary-foreground)`.
    #[prop(optional, into)]
    foreground: Option<String>,
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional, into)]
    size: Signal<Size>,
    /// The content.
    children: Children,
) -> impl IntoView {
    let label = move || {
        let n = count.get();
        if n > max {
            format!("{max}+")
        } else {
            n.to_string()
        }
    };
    let mark = move || {
        let is_dot = dot.get();
        if !is_dot && count.get() == 0 {
            return None;
        }
        let len = if is_dot { 0 } else { label().len() };
        let mut style = format!("--_n:{len}");
        if let Some(c) = &color {
            style.push_str(&format!(";--_color:{c}"));
        }
        if let Some(f) = &foreground {
            style.push_str(&format!(";--_fg:{f}"));
        }
        Some(view! {
            <span
                class="vdl-indicator__mark"
                data-kind=if is_dot { "dot" } else { "number" }
                style=style
                aria-hidden="true"
            >
                {(!is_dot).then(label)}
            </span>
        })
    };

    view! {
        <span class="vdl-indicator" data-size=move || size.get().as_str()>
            {children()}
            {mark}
        </span>
    }
}
