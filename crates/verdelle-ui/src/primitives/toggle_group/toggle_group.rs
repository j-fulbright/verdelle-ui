use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::direction::{Direction, use_direction};
use crate::primitives::core::primitive::Primitive;
use crate::primitives::core::use_controllable_state::{
    UseControllableStateParams, use_controllable_state,
};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleGroupType {
    Single,
    Multiple,
}

#[derive(Clone, Debug)]
struct ToggleGroupContextValue {
    value: Signal<Vec<String>>,
    on_item_activate: Callback<String>,
    disabled: Signal<bool>,
    orientation: Signal<String>,
    /// The item that last had focus: the roving tab stop once the user has moved.
    focused: RwSignal<Option<String>>,
}

#[component]
pub fn ToggleGroup(
    #[prop(into)] r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    /// In single mode: whether pressing the pressed item clears the value (default true).
    #[prop(into, optional)]
    deselectable: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let deselectable = Signal::derive(move || deselectable.get().unwrap_or(true));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change.map(|cb| {
            Callback::new(move |value: Option<Vec<String>>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_value,
    });
    let value = Signal::derive(move || value.get().unwrap_or_default());

    let group_type = r#type;
    let context_value = ToggleGroupContextValue {
        value,
        orientation,
        on_item_activate: Callback::new(move |item_value: String| {
            let current = value.get();
            let next = match group_type {
                ToggleGroupType::Single => {
                    if current.contains(&item_value) {
                        if !deselectable.get_untracked() {
                            return;
                        }
                        vec![]
                    } else {
                        vec![item_value]
                    }
                }
                ToggleGroupType::Multiple => {
                    if current.contains(&item_value) {
                        current.into_iter().filter(|v| *v != item_value).collect()
                    } else {
                        let mut next = current;
                        next.push(item_value);
                        next
                    }
                }
            };
            set_value.run(Some(next));
        }),
        disabled,
        focused: RwSignal::new(None),
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="group"
                attr:data-orientation=move || orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
                attr:dir=move || direction.get().to_string()
                on:keydown=move |event: KeyboardEvent| {
                    let is_vertical = orientation.get() != "horizontal";
                    let is_horizontal = orientation.get() != "vertical";
                    let is_rtl = direction.get() == Direction::Rtl;

                    let next = match event.key().as_str() {
                        "ArrowUp" if is_vertical => Some(false),
                        "ArrowDown" if is_vertical => Some(true),
                        "ArrowLeft" if is_horizontal => Some(is_rtl),
                        "ArrowRight" if is_horizontal => Some(!is_rtl),
                        "Home" => {
                            event.prevent_default();
                            roving_focus_items(&event, false, true, true);
                            None
                        }
                        "End" => {
                            event.prevent_default();
                            roving_focus_items(&event, true, true, true);
                            None
                        }
                        _ => None,
                    };

                    if let Some(forward) = next {
                        event.prevent_default();
                        roving_focus_items(&event, forward, do_loop.get(), false);
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ToggleGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ToggleGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();

    let disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));
    let is_pressed = Signal::derive(move || context.value.get().contains(&item_value));
    let item_value_focus = value.clone();
    let item_value_stop = value.clone();
    let item_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, item_ref]);

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=composed_refs
            attr:r#type="button"
            attr:aria-pressed=move || is_pressed.get().to_string()
            attr:data-state=move || if is_pressed.get() { "on" } else { "off" }
            attr:data-orientation=move || context.orientation.get()
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            attr:tabindex=move || {
                // Roving tab index: the item that last had focus is the tab stop; before
                // that, the first pressed item, or the first enabled one when none is.
                let value_now = context.value.get();
                match context.focused.get() {
                    Some(focused) => {
                        if focused == item_value_stop { "0" } else { "-1" }
                    }
                    None => {
                        if is_tab_stop(item_ref, !value_now.is_empty()) { "0" } else { "-1" }
                    }
                }
            }
            on:focus=move |_| context.focused.set(Some(item_value_focus.clone()))
            on:click=move |_| {
                if !disabled.get() {
                    context.on_item_activate.run(item_value_click.clone());
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/// Helper: move focus between sibling toggle-group items using DOM queries.
/// With `to_edge`, jump to the last item (`forward`) or the first one instead of stepping.
fn roving_focus_items(event: &KeyboardEvent, forward: bool, do_loop: bool, to_edge: bool) {
    let target = event.current_target();
    let Some(group) = target.and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) = group.query_selector_all("button:not([disabled])") else {
        return;
    };

    let mut nodes = vec![];
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            nodes.push(node);
        }
    }

    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());

    let current_index = active
        .as_ref()
        .and_then(|a| {
            use web_sys::wasm_bindgen::JsCast;
            let a_node: &web_sys::Node = a.unchecked_ref();
            nodes.iter().position(|n| n == a_node)
        })
        .unwrap_or(0);

    let next_index = if to_edge {
        if forward {
            nodes.len().checked_sub(1)
        } else {
            Some(0)
        }
    } else if forward {
        if current_index + 1 < nodes.len() {
            Some(current_index + 1)
        } else if do_loop {
            Some(0)
        } else {
            None
        }
    } else if current_index > 0 {
        Some(current_index - 1)
    } else if do_loop {
        Some(nodes.len().saturating_sub(1))
    } else {
        None
    };

    if let Some(idx) = next_index
        && let Some(node) = nodes.get(idx)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = node.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

/// Whether `item` is the tab stop of its group before any item has had focus: the first
/// enabled pressed item, or the first enabled item when none is pressed. Before the item
/// is mounted (server rendering) this is true for every item, which leaves the group
/// tabbable; the client corrects it on hydration.
fn is_tab_stop(item: AnyNodeRef, any_pressed: bool) -> bool {
    let Some(item) = item.get() else {
        return true;
    };
    let Ok(Some(group)) = item.closest("[role='group']") else {
        return true;
    };
    let selector = if any_pressed {
        "button[aria-pressed='true']:not([disabled])"
    } else {
        "button:not([disabled])"
    };
    match group.query_selector(selector) {
        Ok(Some(first)) => first.is_same_node(Some(&item)),
        _ => true,
    }
}
