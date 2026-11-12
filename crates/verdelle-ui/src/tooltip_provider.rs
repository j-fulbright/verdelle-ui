use crate::primitives::tooltip as primitive;
use leptos::prelude::*;

/// Shares delay settings between the tooltips inside it, as shadcn/ui's TooltipProvider.
/// Put one near the root of the app or around a toolbar.
///
/// `delay_duration` (milliseconds, default 0 as in shadcn) is how long the pointer must
/// rest on a trigger before its tooltip opens. After one tooltip closes, any tooltip that
/// opens within `skip_delay_duration` (default 300ms) opens at once, so moving along a
/// row of buttons does not wait on each. Keyboard focus always opens at once.
#[component]
pub fn TooltipProvider(
    #[prop(into, optional)] delay_duration: MaybeProp<i32>,
    #[prop(into, optional)] skip_delay_duration: MaybeProp<i32>,
    /// The content.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let delay = MaybeProp::derive(move || Some(delay_duration.get().unwrap_or(0)));
    let skip = MaybeProp::derive(move || Some(skip_delay_duration.get().unwrap_or(300)));

    view! {
        <primitive::TooltipProvider delay_duration=delay skip_delay_duration=skip>
            {children.with_value(|children| children())}
        </primitive::TooltipProvider>
    }
}
