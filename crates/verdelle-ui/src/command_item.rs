use crate::primitives::command as primitive;
use leptos::prelude::*;

/// One action. `value` is its identity (what `on_select` receives); `label` (default: the
/// value) and `keywords` are what a search matches; the children are what it shows (an icon, the
/// name, a [`crate::CommandShortcut`]). The highlighted item is filled; pressing it or Enter
/// runs `on_select`. It is hidden while it does not match the search; `force_match` keeps it.
#[component]
pub fn CommandItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] keywords: MaybeProp<Vec<String>>,
    /// Disables it: no focus or input, and it is styled as disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    #[prop(into, optional)] force_match: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<String>>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::CommandItem
            value=value
            label=label
            keywords=keywords
            disabled=disabled
            force_match=force_match
            on_select=on_select.unwrap_or_else(|| Callback::new(|_: String| {}))
            class=format!("vdl-command__item {class}")
        >
            {children.with_value(|children| children())}
        </primitive::CommandItem>
    }
}
