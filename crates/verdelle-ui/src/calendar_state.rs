use leptos::prelude::*;

/// State that can be given from outside and still changes on its own.
///
/// It starts as `prop`, else `default`. After that the component changes it itself (and tells its
/// owner through a callback), and whenever `prop` changes later, including to `None`, the state follows
/// it. That makes a signal passed as `prop` a controlled value in both directions: setting it clears or
/// replaces what was chosen, and choosing something sets it through the callback. A plain value that
/// never changes only seeds the state. (The shared `use_controllable_state` cannot tell "controlled and
/// empty" from "not controlled", so a parent could not clear a value with it.)
///
/// `on_external` runs after the state follows a change of `prop`.
pub(crate) fn use_synced<T>(
    prop: MaybeProp<T>,
    default: MaybeProp<T>,
    on_external: impl Fn(&Option<T>) + Send + Sync + 'static,
) -> RwSignal<Option<T>>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let state = RwSignal::new(prop.get_untracked().or_else(|| default.get_untracked()));
    let first = StoredValue::new(true);
    Effect::new(move |_| {
        let next = prop.get();
        if first.get_value() {
            first.set_value(false);
            return;
        }
        if state.get_untracked() != next {
            state.set(next.clone());
            on_external(&next);
        }
    });
    state
}
