//! The glyphs the components draw themselves, as the public icon set's shapes
//! ([`crate::IconName`]) without `width`/`height`: the CSS of the component that uses them
//! sets the size, and colour is `currentColor`. Apps use [`crate::Icon`] instead.

use crate::icon::svg;
use crate::icon_name::IconName;
use leptos::prelude::*;

fn bare(name: IconName) -> impl IntoView {
    svg(Signal::stored(name), None, 2.0, String::new(), None)
}

pub(crate) fn chevron_right() -> impl IntoView {
    bare(IconName::ChevronRight)
}

pub(crate) fn chevron_left() -> impl IntoView {
    bare(IconName::ChevronLeft)
}

pub(crate) fn chevron_down() -> impl IntoView {
    bare(IconName::ChevronDown)
}

pub(crate) fn x() -> impl IntoView {
    bare(IconName::X)
}

pub(crate) fn panel_left() -> impl IntoView {
    bare(IconName::PanelLeft)
}

pub(crate) fn ellipsis() -> impl IntoView {
    bare(IconName::Ellipsis)
}

pub(crate) fn arrow_up() -> impl IntoView {
    bare(IconName::ArrowUp)
}

pub(crate) fn arrow_down() -> impl IntoView {
    bare(IconName::ArrowDown)
}

pub(crate) fn chevrons_up_down() -> impl IntoView {
    bare(IconName::ChevronsUpDown)
}

pub(crate) fn chevrons_left() -> impl IntoView {
    bare(IconName::ChevronsLeft)
}

pub(crate) fn chevrons_right() -> impl IntoView {
    bare(IconName::ChevronsRight)
}

pub(crate) fn check() -> impl IntoView {
    bare(IconName::Check)
}

pub(crate) fn search() -> impl IntoView {
    bare(IconName::Search)
}

pub(crate) fn eye() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M2.06 12.35a1 1 0 0 1 0-.7 10.75 10.75 0 0 1 19.88 0 1 1 0 0 1 0 .7 10.75 10.75 0 0 1-19.88 0" />
            <circle cx="12" cy="12" r="3" />
        </svg>
    }
}

pub(crate) fn eye_off() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c4.5 0 8.34 2.75 9.94 6.65a1 1 0 0 1 0 .7 10.8 10.8 0 0 1-1.67 2.68" />
            <path d="M6.61 6.61A13.5 13.5 0 0 0 2.06 11.65a1 1 0 0 0 0 .7C3.66 16.25 7.5 19 12 19a9.7 9.7 0 0 0 5.39-1.61" />
            <path d="m2 2 20 20" />
            <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
        </svg>
    }
}

pub(crate) fn plus() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M5 12h14M12 5v14" />
        </svg>
    }
}

pub(crate) fn minus() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M5 12h14" />
        </svg>
    }
}

pub(crate) fn calendar() -> impl IntoView {
    bare(IconName::Calendar)
}
