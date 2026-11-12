use crate::Orientation;
use crate::primitives::toolbar as primitive;
use leptos::prelude::*;

/// A row (or column) of controls with one tab stop, as in Base UI's Toolbar, built on the
/// headless Toolbar (`role="toolbar"`).
///
/// Tab enters the toolbar once and leaves it again; the arrow keys (Up and Down when
/// `orientation` is vertical) move between the enabled items, Home and End jump to the ends,
/// and focus wraps unless `loop_focus` is false. Put [`crate::ToolbarButton`],
/// [`crate::ToolbarToggleGroup`], [`crate::ToolbarLink`], [`crate::ToolbarInput`] and
/// [`crate::ToolbarSeparator`] inside; other controls are not part of the roving focus.
/// Name the toolbar with `aria_label`.
#[component]
pub fn Toolbar(
    #[prop(optional)] orientation: Orientation,
    /// Wrap from the last item to the first and back (default true).
    #[prop(default = true)]
    loop_focus: bool,
    /// Accessible name, for when the visible content is not text.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <primitive::Toolbar
            orientation=orientation.to_string()
            r#loop=loop_focus
            attr:aria-label=aria_label
            attr:class=format!("vdl-toolbar {class}")
        >
            {children.with_value(|children| children())}
        </primitive::Toolbar>
    }
}
