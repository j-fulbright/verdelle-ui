use crate::primitives::collapsible as primitive;
use leptos::prelude::*;

/// The part of a [`crate::Collapsible`] that is shown and hidden (`role="region"`, with
/// `hidden` set while closed). `force_mount` keeps the content in the page while closed.
/// There is no height animation, as in shadcn's Collapsible; style `[data-state]` and the
/// `--vdl-collapsible-content-height` variable if you want one.
///
/// With `state` it binds to a [`crate::CollapsibleState`] instead of the enclosing
/// [`crate::Collapsible`]. With `as_child` it is the one element you give it, for example a
/// `<tr>` that is hidden while closed (it then has no `role="region"` and no component class).
#[component]
pub fn CollapsibleContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    /// Bind to a shared state (see [`crate::use_collapsible`]) instead of the enclosing
    /// [`crate::Collapsible`].
    #[prop(optional, into)]
    state: MaybeProp<crate::CollapsibleState>,
    /// Use the single child element instead of a `div`.
    #[prop(optional)]
    as_child: bool,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <primitive::CollapsibleContent
            force_mount=force_mount
            state=state
            as_child=as_child
            attr:class=(!as_child).then(|| format!("vdl-collapsible__content {class}"))
        >
            {children()}
        </primitive::CollapsibleContent>
    }
}
