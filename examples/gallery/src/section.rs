use leptos::prelude::*;

/// shadcn-exported CSS, the example in `docs/examples/`, used by the app and the Themes page.
pub const OCEAN_CSS: &str = include_str!("../../../docs/examples/ocean-theme.css");

/// A page of the gallery: a heading, a line of context, and its sections.
#[component]
pub fn Page(
    #[prop(into)] title: String,
    #[prop(into)] summary: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="mx-auto max-w-4xl space-y-10">
            <header class="space-y-2">
                <h1 class="text-2xl font-semibold">{title}</h1>
                <p class="text-muted-foreground">{summary}</p>
            </header>
            {children()}
        </div>
    }
}

/// A titled group of examples on a page.
#[component]
pub fn Section(
    #[prop(into)] title: String,
    #[prop(optional, into)] description: String,
    children: Children,
) -> impl IntoView {
    let has_description = !description.is_empty();
    view! {
        <section class="space-y-4">
            <div class="space-y-1">
                <h2 class="text-lg font-medium">{title}</h2>
                {has_description.then(|| view! { <p class="text-sm text-muted-foreground">{description}</p> })}
            </div>
            {children()}
        </section>
    }
}

/// A row of examples with a small label at its start.
#[component]
pub fn Row(#[prop(optional, into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex min-w-0 flex-wrap items-center gap-3">
            {(!label.is_empty()).then(|| view! { <span class="w-28 shrink-0 text-xs text-muted-foreground">{label}</span> })}
            {children()}
        </div>
    }
}
