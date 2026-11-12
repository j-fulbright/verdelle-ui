//! shadcn/ui's Typography page as components. They are plain elements with a `vdl-` class, so
//! all of them live in this one file. For HTML you do not control (rendered Markdown, say),
//! wrap it in [`Prose`] instead, which styles the bare tags the same way.

use leptos::prelude::*;

/// A page title: extra-bold, centred, balanced. One per page.
#[component]
pub fn TypographyH1(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h1 class=format!("vdl-h1 {class}")>{children()}</h1> }
}

/// A section heading, with a bottom border.
#[component]
pub fn TypographyH2(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h2 class=format!("vdl-h2 {class}")>{children()}</h2> }
}

/// A subsection heading.
#[component]
pub fn TypographyH3(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h3 class=format!("vdl-h3 {class}")>{children()}</h3> }
}

/// A minor heading.
#[component]
pub fn TypographyH4(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h4 class=format!("vdl-h4 {class}")>{children()}</h4> }
}

/// A paragraph with generous line height; paragraphs after the first get space above.
#[component]
pub fn TypographyP(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <p class=format!("vdl-p {class}")>{children()}</p> }
}

/// An indented, italic quotation with a rule on its start side.
#[component]
pub fn TypographyBlockquote(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <blockquote class=format!("vdl-blockquote {class}")>{children()}</blockquote> }
}

/// A bulleted list. Put `<li>` elements inside; each after the first gets space above.
#[component]
pub fn TypographyList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <ul class=format!("vdl-list {class}")>{children()}</ul> }
}

/// Code inside a sentence: muted background, monospace, semibold.
#[component]
pub fn TypographyInlineCode(
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    /// The content.
    children: Children,
) -> impl IntoView {
    view! { <code class=format!("vdl-inline-code {class}")>{children()}</code> }
}

/// An introductory paragraph: larger and muted.
#[component]
pub fn TypographyLead(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <p class=format!("vdl-lead {class}")>{children()}</p> }
}

/// Larger, semibold text for a callout inside a block.
#[component]
pub fn TypographyLarge(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-large {class}")>{children()}</div> }
}

/// Small, medium-weight text for fine print or a caption.
#[component]
pub fn TypographySmall(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <small class=format!("vdl-small {class}")>{children()}</small> }
}

/// Small, muted text for secondary information.
#[component]
pub fn TypographyMuted(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <p class=format!("vdl-muted {class}")>{children()}</p> }
}

/// A wrapper that gives the bare tags inside it (h1 to h4, p, blockquote, ul and ol, code, pre,
/// hr, a, table) the typography styles, for HTML you do not write yourself.
#[component]
pub fn Prose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("vdl-prose {class}")>{children()}</div> }
}
