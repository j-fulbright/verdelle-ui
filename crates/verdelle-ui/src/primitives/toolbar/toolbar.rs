use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::direction::{Direction, use_direction};
use crate::primitives::core::primitive::{Primitive, VoidPrimitive};
use leptos::{
    context::Provider,
    ev::{FocusEvent, KeyboardEvent},
    html,
    prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ToolbarContextValue {
    orientation: Signal<String>,
    direction: Signal<Direction>,
}

#[component]
pub fn Toolbar(
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    let context_value = ToolbarContextValue {
        orientation,
        direction,
    };

    // One tab stop: after hydration every item but the first enabled one leaves the tab order.
    // Focus moving into the toolbar keeps this current (see `on:focusin` below).
    let toolbar_ref = AnyNodeRef::new();
    let own_ref = use_composed_refs(vec![node_ref, toolbar_ref]);
    Effect::new(move |_| {
        if let Some(toolbar) = toolbar_ref
            .get()
            .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
        {
            sync_tab_stops(&toolbar, None);
        }
    });

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=own_ref
                attr:role="toolbar"
                attr:aria-orientation=move || orientation.get()
                attr:data-orientation=move || orientation.get()
                attr:dir=move || direction.get().to_string()
                on:focusin=move |event: FocusEvent| {
                    if let Some(toolbar) = event_current_element(&event)
                        && let Some(target) = event_target_element(&event)
                    {
                        sync_tab_stops(&toolbar, Some(&target));
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    let is_horizontal = orientation.get() == "horizontal";
                    let is_rtl = direction.get() == Direction::Rtl;

                    // Arrow keys inside a text field move its caret, and only hand over focus at
                    // the field's edge (Base UI's behaviour).
                    let at_edge = |forward: bool| text_field_at_edge(&event, forward);
                    let next = match event.key().as_str() {
                        "ArrowLeft" if is_horizontal => {
                            let forward = is_rtl;
                            at_edge(forward).then_some(forward)
                        }
                        "ArrowRight" if is_horizontal => {
                            let forward = !is_rtl;
                            at_edge(forward).then_some(forward)
                        }
                        "ArrowUp" if !is_horizontal => Some(false),
                        "ArrowDown" if !is_horizontal => Some(true),
                        "Home" if !is_text_field(&event) => {
                            event.prevent_default();
                            focus_toolbar_item(&event, true, true, true);
                            None
                        }
                        "End" if !is_text_field(&event) => {
                            event.prevent_default();
                            focus_toolbar_item(&event, false, true, true);
                            None
                        }
                        _ => None,
                    };

                    if let Some(forward) = next {
                        event.prevent_default();
                        focus_toolbar_item(&event, forward, do_loop.get(), false);
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ToolbarButton(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:data-toolbar-item=""
            attr:data-disabled=move || disabled.get().unwrap_or(false).then_some("")
            attr:disabled=move || disabled.get().unwrap_or(false)
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/// A text input inside the toolbar: it is one stop of the roving focus, and Left/Right only
/// leave it at the edge of the text.
#[component]
pub fn ToolbarInput(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <VoidPrimitive
            element=html::input
            as_child=as_child
            node_ref=node_ref
            attr:data-toolbar-item=""
            attr:data-disabled=move || disabled.get().unwrap_or(false).then_some("")
            attr:disabled=move || disabled.get().unwrap_or(false)
        >
            {""}
        </VoidPrimitive>
    }
}

#[component]
pub fn ToolbarLink(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive
            element=html::a
            as_child=as_child
            node_ref=node_ref
            attr:data-toolbar-item=""
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn ToolbarSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ToolbarContextValue>();

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:role="separator"
            attr:aria-orientation=move || {
                match context.orientation.get().as_str() {
                    "horizontal" => "vertical",
                    _ => "horizontal",
                }
            }
            attr:data-orientation=move || {
                match context.orientation.get().as_str() {
                    "horizontal" => "vertical",
                    _ => "horizontal",
                }
            }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ToolbarToggleGroupContextValue {
    value: Signal<Vec<String>>,
    on_item_toggle: Callback<String>,
    r#type: Signal<String>,
    disabled: Signal<bool>,
}

#[component]
pub fn ToolbarToggleGroup(
    #[prop(into)] r#type: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ToolbarContextValue>();
    let toggle_type = Signal::derive(move || r#type.get().unwrap_or("single".into()));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let (value_state, set_value) =
        crate::primitives::core::use_controllable_state::use_controllable_state(
            crate::primitives::core::use_controllable_state::UseControllableStateParams {
                prop: value,
                on_change: on_value_change.map(|cb| {
                    Callback::new(move |value: Option<Vec<String>>| {
                        if let Some(value) = value {
                            cb.run(value);
                        }
                    })
                }),
                default_prop: default_value,
            },
        );
    let value = Signal::derive(move || value_state.get().unwrap_or_default());

    let toggle_ctx = ToolbarToggleGroupContextValue {
        value,
        on_item_toggle: Callback::new(move |item_value: String| {
            let current = value.get();
            let is_single = toggle_type.get() == "single";
            let next = if is_single {
                if current.contains(&item_value) {
                    vec![]
                } else {
                    vec![item_value]
                }
            } else if current.contains(&item_value) {
                current.into_iter().filter(|v| *v != item_value).collect()
            } else {
                let mut next = current;
                next.push(item_value);
                next
            };
            set_value.run(Some(next));
        }),
        r#type: toggle_type,
        disabled,
    };

    view! {
        <Provider value=toggle_ctx>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="group"
                attr:data-orientation=move || context.orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ToolbarToggleItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let toggle_ctx = expect_context::<ToolbarToggleGroupContextValue>();
    let item_value = value.clone();
    let item_value2 = value.clone();
    let is_pressed = Signal::derive(move || toggle_ctx.value.get().contains(&item_value));
    let disabled =
        Signal::derive(move || toggle_ctx.disabled.get() || disabled.get().unwrap_or(false));

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:data-toolbar-item=""
            attr:aria-pressed=move || is_pressed.get().to_string()
            attr:data-state=move || if is_pressed.get() { "on" } else { "off" }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get() {
                    toggle_ctx.on_item_toggle.run(item_value2.clone());
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

const ITEM: &str = "[data-toolbar-item]";

fn event_current_element(event: &FocusEvent) -> Option<web_sys::Element> {
    event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
}

fn event_target_element(event: &FocusEvent) -> Option<web_sys::Element> {
    event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
}

fn is_enabled(el: &web_sys::Element) -> bool {
    !el.has_attribute("disabled") && !el.has_attribute("data-disabled")
}

/// The toolbar's enabled items in document order.
fn enabled_items(toolbar: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let Ok(all) = toolbar.query_selector_all(ITEM) else {
        return vec![];
    };
    (0..all.length())
        .filter_map(|i| all.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        .filter(|el| is_enabled(el))
        .collect()
}

/// Roving tab stop: `active` (or the first enabled item) gets `tabindex=0`, every other item
/// `-1`, so Tab enters and leaves the toolbar in one step.
fn sync_tab_stops(toolbar: &web_sys::Element, active: Option<&web_sys::Element>) {
    let Ok(all) = toolbar.query_selector_all(ITEM) else {
        return;
    };
    let items: Vec<web_sys::Element> = (0..all.length())
        .filter_map(|i| all.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::Element>().ok())
        .collect();
    let stop = active
        .and_then(|a| {
            items
                .iter()
                .find(|i| a.is_same_node(Some(i)) && is_enabled(i))
        })
        .or_else(|| items.iter().find(|i| is_enabled(i)));
    for item in &items {
        let on = stop.is_some_and(|s| s.is_same_node(Some(item)));
        let _ = item.set_attribute("tabindex", if on { "0" } else { "-1" });
    }
}

fn target_input(event: &KeyboardEvent) -> Option<web_sys::HtmlInputElement> {
    event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
}

fn is_text_field(event: &KeyboardEvent) -> bool {
    target_input(event).is_some()
}

/// For a text input, whether the caret is at the edge in the direction of travel (a
/// non-input target counts as at the edge).
fn text_field_at_edge(event: &KeyboardEvent, forward: bool) -> bool {
    let Some(input) = target_input(event) else {
        return true;
    };
    let (Ok(Some(start)), Ok(Some(end))) = (input.selection_start(), input.selection_end()) else {
        return true;
    };
    if start != end {
        return false;
    }
    if forward {
        end as usize >= input.value().encode_utf16().count()
    } else {
        start == 0
    }
}

/// Move focus to the next or previous enabled item; `edge` jumps to the first (forward) or
/// last (backward) one instead, for Home and End.
fn focus_toolbar_item(event: &KeyboardEvent, forward: bool, do_loop: bool, edge: bool) {
    let Some(toolbar) = event
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    else {
        return;
    };
    let items = enabled_items(&toolbar);
    if items.is_empty() {
        return;
    }
    let current = event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|t| items.iter().position(|i| t.is_same_node(Some(i))));

    let next = if edge {
        Some(if forward { 0 } else { items.len() - 1 })
    } else {
        match (current, forward) {
            (None, _) => Some(0),
            (Some(i), true) if i + 1 < items.len() => Some(i + 1),
            (Some(_), true) => do_loop.then_some(0),
            (Some(i), false) if i > 0 => Some(i - 1),
            (Some(_), false) => do_loop.then_some(items.len() - 1),
        }
    };
    if let Some(item) = next.and_then(|i| items.get(i)) {
        sync_tab_stops(&toolbar, Some(item));
        let _ = item.focus();
    }
}
