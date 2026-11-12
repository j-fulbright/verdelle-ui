use crate::Size;
use crate::primitives::avatar as primitive;
use leptos::prelude::*;

/// A round user picture with a fallback, styled as shadcn/ui's Avatar and built on the
/// headless Avatar (which loads the image off-screen and tells the fallback when
/// to show).
///
/// Compose with [`crate::AvatarImage`] and [`crate::AvatarFallback`]; the fallback shows
/// until the image has loaded, and stays if it fails. `size` is `Size::Small` (24px), the
/// default (32px) or `Size::Large` (40px); `Size::XSmall` draws as small. Add an
/// [`crate::AvatarBadge`] for a status dot, and stack several in an
/// [`crate::AvatarGroup`]. The avatar is decorative unless an image has `alt` text or the
/// fallback carries the person's initials.
#[component]
pub fn Avatar(
    /// Size: `Size::XSmall`, `Small`, `Medium` (default) or `Large`.
    #[prop(optional)]
    size: Size,
    /// Extra classes, for layout (for example Tailwind utilities). They override the component's own styles.
    #[prop(optional, into)]
    class: String,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <primitive::Avatar
            attr:class=format!("vdl-avatar {class}")
            attr:data-size=size.as_str()
        >
            {children.with_value(|children| children())}
        </primitive::Avatar>
    }
}
