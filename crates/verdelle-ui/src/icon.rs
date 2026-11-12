use crate::icon_name::IconName;
use leptos::prelude::*;

/// The shared drawing: a 24x24 stroke SVG whose children are the icon's shapes. `size` of
/// `None` leaves out `width`/`height`, for icons whose size comes from the CSS around them.
pub(crate) fn svg(
    name: Signal<IconName>,
    size: Option<u32>,
    stroke_width: f32,
    class: String,
    label: Option<String>,
) -> impl IntoView {
    let class = (!class.is_empty()).then_some(class);
    let hidden = label.is_none().then_some("true");
    let role = label.is_some().then_some("img");
    view! {
        <svg
            class=class
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden=hidden
            role=role
            aria-label=label
            inner_html=move || name.get().body()
        ></svg>
    }
}

/// A [Lucide](https://lucide.dev) icon (ISC licence), drawn as inline SVG.
///
/// ```ignore
/// <Icon name=IconName::Check />
/// <Icon name=IconName::Trash2 size=20 class="text-destructive" />
/// <Icon name=IconName::TriangleAlert label="Warning" />
/// ```
///
/// The icon is 24x24 stroke artwork drawn in `currentColor`, so it takes the text colour of
/// whatever it sits in. It is 16px by default (shadcn's `size-4`); `size` is in pixels and
/// `stroke_width` is the line width in the 24-unit grid (2 by default). A CSS rule that sets
/// the svg's `width` and `height` wins over `size`, which is how components size their own
/// icons. The icon is hidden from assistive technology unless it has a `label`, which makes
/// it an image with that accessible name: give one when the icon stands alone and means
/// something, and none when text next to it says the same thing.
///
/// `name` can be a signal, so one `Icon` can switch between two icons (eye and eye-off).
/// [`IconName::ALL`] lists every icon.
#[component]
pub fn Icon(
    #[prop(into)] name: Signal<IconName>,
    #[prop(default = 16)] size: u32,
    #[prop(default = 2.0)] stroke_width: f32,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// An accessible name. Without one the icon is `aria-hidden`.
    #[prop(optional, into)]
    label: Option<String>,
) -> impl IntoView {
    svg(name, Some(size), stroke_width, class, label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;
    use std::collections::HashSet;

    fn html(label: Option<&str>, size: Option<u32>) -> String {
        let owner = Owner::new();
        owner.with(|| {
            svg(
                Signal::stored(IconName::Check),
                size,
                2.0,
                "x".into(),
                label.map(String::from),
            )
            .to_html()
        })
    }

    #[test]
    fn names_are_unique_kebab_case_and_have_shapes() {
        let mut seen = HashSet::new();
        for icon in IconName::ALL {
            let name = icon.as_str();
            assert!(seen.insert(name), "{name} is listed twice");
            assert!(
                name.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{name} is not kebab-case"
            );
            let body = icon.body();
            assert!(
                body.starts_with('<') && body.ends_with("/>"),
                "{name} has no shapes"
            );
        }
        assert!(IconName::ALL.len() > 150);
    }

    #[test]
    fn an_icon_without_a_label_is_hidden_and_with_one_is_an_image() {
        let hidden = html(None, Some(16));
        assert!(hidden.contains("aria-hidden=\"true\""), "{hidden}");
        assert!(!hidden.contains("role="), "{hidden}");
        let labelled = html(Some("Done"), Some(16));
        assert!(
            labelled.contains("role=\"img\"") && labelled.contains("aria-label=\"Done\""),
            "{labelled}"
        );
        assert!(!labelled.contains("aria-hidden"), "{labelled}");
    }

    #[test]
    fn size_is_written_only_when_given_and_the_shape_is_inside() {
        let sized = html(None, Some(20));
        assert!(
            sized.contains("width=\"20\"") && sized.contains("height=\"20\""),
            "{sized}"
        );
        assert!(sized.contains("M20 6 9 17l-5-5"), "{sized}");
        let bare = html(None, None);
        assert!(!bare.contains(" width="), "{bare}");
    }
}
