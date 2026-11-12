use crate::TooltipProvider;
use crate::sidebar_context::SidebarContext;
use leptos::prelude::*;

/// The sidebar breakpoint: below this width (px) the sidebar is a sheet. shadcn's `md`.
const MOBILE_BREAKPOINT: f64 = 768.0;
/// The cookie that remembers whether the desktop sidebar is expanded, as shadcn's.
const COOKIE: &str = "sidebar_state";

/// The root of a sidebar layout, as shadcn/ui's `SidebarProvider`: a flex row holding a
/// [`crate::Sidebar`] and a [`crate::SidebarInset`], and the state they share ([`crate::use_sidebar`]).
///
/// State: `default_open` (expanded, by default) or a two-way `open` signal. Unless `open` is
/// given, the choice is remembered in the `sidebar_state` cookie (`persist`, on by default) and
/// read back after hydration, so the sidebar may animate from its default to the saved state on
/// load; pass `open` and store it yourself (for example from the request's cookie) to avoid that.
/// The cookie is only touched in the browser.
///
/// Ctrl/Cmd + `keyboard_shortcut` (default `b`; empty turns it off) toggles the sidebar. Below
/// 768px the sidebar becomes a [`crate::Sheet`] and the shortcut and the trigger toggle that.
/// `style` takes CSS custom properties: `--sidebar-width` (16rem), `--sidebar-width-icon` (3rem).
///
/// `contained` positions the sidebar inside the provider's box instead of fixed to the window,
/// for a sidebar in a part of the page (a demo, a panel): give the provider a height, since
/// it no longer grows to the window. The provider also wraps its children in a
/// [`crate::TooltipProvider`] with no delay, for the collapsed menu buttons' tooltips.
#[component]
pub fn SidebarProvider(
    /// Initial open state when `open` is not given.
    #[prop(default = true)]
    default_open: bool,
    /// Two-way open state. Without it the provider keeps its own and persists it in a cookie.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,
    /// Remember the state in a cookie (only when `open` is not given). On by default.
    #[prop(default = true)]
    persist: bool,
    #[prop(default = "b".to_string(), into)] keyboard_shortcut: String,
    #[prop(optional)] contained: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    #[prop(optional, into)] style: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let controlled = open.is_some();
    let open = open.unwrap_or_else(|| RwSignal::new(default_open));
    let ctx = SidebarContext {
        open,
        open_mobile: RwSignal::new(false),
        is_mobile: RwSignal::new(false),
    };
    provide_context(ctx);
    // Transitions are off until the saved state has been applied, so the sidebar does not
    // animate from the default to it on load.
    let instant = RwSignal::new(true);

    // Everything below is in effects, which only run in the browser, so none of it needs a
    // `hydrate` cfg (the feature is not forwarded by every consumer).
    {
        use leptos::{ev, leptos_dom::helpers::set_timeout, wasm_bindgen::JsCast};
        use std::time::Duration;

        fn html_document() -> Option<web_sys::HtmlDocument> {
            document().dyn_into::<web_sys::HtmlDocument>().ok()
        }

        // Runs first: adopt the saved state.
        Effect::new(move |_| {
            if persist
                && !controlled
                && let Some(doc) = html_document()
                && let Ok(cookies) = doc.cookie()
                && let Some(saved) = cookies
                    .split(';')
                    .filter_map(|c| c.trim().split_once('='))
                    .find_map(|(k, v)| (k == COOKIE).then_some(v))
            {
                open.set(saved == "true");
            }
            set_timeout(move || instant.set(false), Duration::from_millis(50));
        });

        // Write the state on every change after the first run.
        Effect::new(move |prev: Option<()>| {
            let value = open.get();
            if prev.is_some()
                && persist
                && !controlled
                && let Some(doc) = html_document()
            {
                let _ = doc.set_cookie(&format!(
                    "{COOKIE}={value}; path=/; max-age=604800; samesite=lax"
                ));
            }
        });

        // Which of the two layouts to draw.
        Effect::new(move |_| {
            let check = move || {
                let wide = window()
                    .inner_width()
                    .ok()
                    .and_then(|w| w.as_f64())
                    .unwrap_or(MOBILE_BREAKPOINT);
                let mobile = wide < MOBILE_BREAKPOINT;
                if ctx.is_mobile.get_untracked() != mobile {
                    ctx.is_mobile.set(mobile);
                }
                if !mobile {
                    ctx.open_mobile.set(false);
                }
            };
            check();
            let handle = window_event_listener(ev::resize, move |_| check());
            on_cleanup(move || handle.remove());
        });

        // Ctrl/Cmd + key.
        let shortcut = keyboard_shortcut.to_lowercase();
        Effect::new(move |_| {
            let shortcut = shortcut.clone();
            if shortcut.is_empty() {
                return;
            }
            let handle = window_event_listener(ev::keydown, move |event| {
                if event.key().to_lowercase() == shortcut && (event.meta_key() || event.ctrl_key())
                {
                    event.prevent_default();
                    ctx.toggle();
                }
            });
            on_cleanup(move || handle.remove());
        });
    }
    view! {
        <div
            class=format!("vdl-sidebar-wrapper {class}")
            data-contained=contained.then_some("")
            data-instant=move || instant.get().then_some("")
            style=style
        >
            <TooltipProvider delay_duration=0>{children()}</TooltipProvider>
        </div>
    }
}
