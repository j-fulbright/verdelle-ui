//! Short names for the icons the gallery examples use most, drawn with Verdelle's public icon set
//! ([`verdelle_ui::Icon`]). The CSS of the component an icon sits in sets its size.

use leptos::prelude::*;
use verdelle_ui::{Icon, IconName};

fn glyph(name: IconName) -> impl IntoView {
    view! { <Icon name=name /> }
}

pub fn search() -> impl IntoView {
    glyph(IconName::Search)
}

pub fn inbox() -> impl IntoView {
    glyph(IconName::Inbox)
}

pub fn folder() -> impl IntoView {
    glyph(IconName::Folder)
}

pub fn info() -> impl IntoView {
    glyph(IconName::Info)
}

pub fn plus() -> impl IntoView {
    glyph(IconName::Plus)
}

pub fn mail() -> impl IntoView {
    glyph(IconName::Mail)
}

pub fn copy() -> impl IntoView {
    glyph(IconName::Copy)
}

pub fn check() -> impl IntoView {
    glyph(IconName::Check)
}

pub fn arrow_up() -> impl IntoView {
    glyph(IconName::ArrowUp)
}

pub fn user() -> impl IntoView {
    glyph(IconName::User)
}

pub fn shield() -> impl IntoView {
    glyph(IconName::Shield)
}

pub fn star() -> impl IntoView {
    glyph(IconName::Star)
}

pub fn more_vertical() -> impl IntoView {
    glyph(IconName::EllipsisVertical)
}

pub fn command() -> impl IntoView {
    glyph(IconName::Command)
}

pub fn bell() -> impl IntoView {
    glyph(IconName::Bell)
}
