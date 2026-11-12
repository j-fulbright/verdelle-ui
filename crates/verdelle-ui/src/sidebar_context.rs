use leptos::prelude::*;

/// The state a [`crate::SidebarProvider`] shares with the sidebar parts, from [`use_sidebar`].
///
/// `open` is the desktop state (expanded or collapsed); `open_mobile` is whether the small-screen
/// [`crate::Sheet`] is showing; `is_mobile` is true below 768px (known only after the page has
/// hydrated, so it starts false). All three are plain signals you can read or set.
#[derive(Clone, Copy)]
pub struct SidebarContext {
    pub open: RwSignal<bool>,
    pub open_mobile: RwSignal<bool>,
    pub is_mobile: RwSignal<bool>,
}

impl SidebarContext {
    /// `true` while the desktop sidebar is expanded (`data-state="expanded"`). Reactive.
    pub fn expanded(&self) -> bool {
        self.open.get()
    }

    /// `"expanded"` or `"collapsed"`. Reactive.
    pub fn state(&self) -> &'static str {
        if self.open.get() {
            "expanded"
        } else {
            "collapsed"
        }
    }

    /// Opens or closes the sidebar for the current screen size: the sheet on a small screen,
    /// the panel otherwise. This is what the trigger, the rail and the shortcut do.
    pub fn toggle(&self) {
        if self.is_mobile.get_untracked() {
            self.open_mobile.update(|o| *o = !*o);
        } else {
            self.open.update(|o| *o = !*o);
        }
    }
}

/// The nearest [`crate::SidebarProvider`]'s state. Panics outside one.
pub fn use_sidebar() -> SidebarContext {
    expect_context::<SidebarContext>()
}
