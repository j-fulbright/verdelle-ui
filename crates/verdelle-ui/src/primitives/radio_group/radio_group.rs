use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::direction::{Direction, use_direction};
use crate::primitives::core::primitive::Primitive;
use crate::primitives::core::use_controllable_state::{
    UseControllableStateParams, use_controllable_state,
};
use crate::primitives::core::use_previous::use_previous;
use crate::primitives::core::use_size::use_size;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct RadioGroupContextValue {
    name: Signal<Option<String>>,
    value: Signal<Option<String>>,
    required: Signal<bool>,
    disabled: Signal<bool>,
    on_value_change: Callback<String>,
    orientation: Signal<Option<String>>,
    do_loop: Signal<bool>,
    direction: Signal<Direction>,
    /// The mounted items, as (id, element). Keyboard movement and the tab stop work from this
    /// list, in document order, so an item may sit anywhere under the group (for example in
    /// table cells) and not only in the group element's own children.
    items: RwSignal<Vec<(usize, AnyNodeRef)>>,
    next_item_id: StoredValue<usize>,
}

impl RadioGroupContextValue {
    /// The enabled items' elements, in document order.
    fn enabled_elements(&self) -> Vec<web_sys::HtmlElement> {
        use web_sys::wasm_bindgen::JsCast;
        let mut elements: Vec<web_sys::HtmlElement> = self
            .items
            .get_untracked()
            .into_iter()
            .filter_map(|(_, r)| r.get_untracked())
            .filter_map(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
            .filter(|el| !el.has_attribute("disabled"))
            .collect();
        elements.sort_by(|a, b| {
            // Bit 4 (FOLLOWING): `b` comes after `a`.
            if a.compare_document_position(b) & 4 != 0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        elements
    }
}

#[component]
pub fn RadioGroup(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get());
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change.map(|cb| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_value,
    });
    let value = Signal::derive(move || value.get());

    let context_value = RadioGroupContextValue {
        orientation,
        do_loop,
        direction,
        items: RwSignal::new(Vec::new()),
        next_item_id: StoredValue::new(0),
        name,
        value,
        required,
        disabled,
        on_value_change: Callback::new(move |val: String| {
            set_value.run(Some(val));
        }),
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="radiogroup"
                attr:aria-required=move || required.get().to_string()
                attr:aria-orientation=move || orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<RadioGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let item_value_form = StoredValue::new(value.clone());

    let disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));
    let is_checked =
        Signal::derive(move || context.value.get().as_deref() == Some(item_value.as_str()));

    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });

    let item_ctx = RadioItemContextValue { is_checked };

    // Register with the group so keys and the tab stop can find the items in document order.
    let item_id = context.next_item_id.get_value();
    context.next_item_id.set_value(item_id + 1);
    let registry = context.items;
    Effect::new(move |_| {
        if button_ref.get().is_some() {
            registry.update(|list| {
                if !list.iter().any(|(id, _)| *id == item_id) {
                    list.push((item_id, button_ref));
                }
            });
        }
    });
    on_cleanup(move || {
        registry.try_update(|list| list.retain(|(id, _)| *id != item_id));
    });
    let key_context = context.clone();
    let tab_context = context.clone();

    view! {
        <Provider value=item_ctx>
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=composed_refs
            attr:r#type="button"
            attr:role="radio"
            attr:aria-checked=move || is_checked.get().to_string()
            attr:data-state=move || if is_checked.get() { "checked" } else { "unchecked" }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            attr:tabindex=move || {
                // Roving tabindex: the checked item is the tab stop; with nothing checked,
                // the first enabled item is (as for native radios).
                if is_checked.get() {
                    "0"
                } else if tab_context.value.get().is_some() || !is_first_enabled(&tab_context, button_ref) {
                    "-1"
                } else {
                    "0"
                }
            }
            on:click=move |_| {
                if !disabled.get() {
                    context.on_value_change.run(item_value_click.clone());
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                let is_vertical = key_context.orientation.get_untracked().as_deref() != Some("horizontal");
                let is_horizontal = key_context.orientation.get_untracked().as_deref() != Some("vertical");
                let is_rtl = key_context.direction.get_untracked() == Direction::Rtl;
                let next = match event.key().as_str() {
                    "ArrowUp" if is_vertical => Some(false),
                    "ArrowDown" if is_vertical => Some(true),
                    "ArrowLeft" if is_horizontal => Some(is_rtl),
                    "ArrowRight" if is_horizontal => Some(!is_rtl),
                    _ => None,
                };
                if let Some(forward) = next {
                    event.prevent_default();
                    roving_focus_items(&key_context, forward);
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
        <Show when=move || is_form_control.get()>
            <BubbleRadioInput
                control_ref=button_ref
                name=context.name
                value=Signal::derive(move || item_value_form.get_value())
                checked=is_checked
                required=context.required
                disabled=disabled
            />
        </Show>
        </Provider>
    }
}

#[component]
pub fn RadioGroupIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<RadioGroupContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    // RadioGroupIndicator is rendered inside a RadioGroupItem.
    // We need to check the parent item's checked state via a local context.
    let item_context = expect_context::<RadioItemContextValue>();

    view! {
        <Show when=move || force_mount.get() || item_context.is_checked.get()>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || if item_context.is_checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || context.disabled.get().then_some("")
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

#[derive(Clone, Debug)]
struct RadioItemContextValue {
    is_checked: Signal<bool>,
}

// ---------------------------------------------------------------------------
// BubbleRadioInput — hidden native <input type="radio"> for form submission
// ---------------------------------------------------------------------------

#[component]
fn BubbleRadioInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] name: Signal<Option<String>>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(Signal::derive(move || checked.get()));
    let control_size = use_size(control_ref);

    // Dispatch synthetic click when checked state changes so form libraries are notified.
    Effect::new(move |_| {
        let current = checked.get();
        if let Some(input) = node_ref.get()
            && prev_checked.get() != current
        {
            let init = web_sys::EventInit::new();
            init.set_bubbles(true);

            let event = web_sys::Event::new_with_event_init_dict("click", &init)
                .expect("Click event should be instantiated.");

            input.set_checked(current);

            input
                .dispatch_event(&event)
                .expect("Click event should be dispatched.");
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="radio"
            aria-hidden="true"
            checked=move || checked.get().then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            name=move || name.get()
            value=move || value.get()
            tab-index="-1"
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
        />
    }
}

/// Move focus (and the choice) to the next or previous enabled item in document order. Works
/// from the group's item registry, so the items need not be children of one element.
fn roving_focus_items(context: &RadioGroupContextValue, forward: bool) {
    let nodes = context.enabled_elements();
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let current_index = active
        .as_ref()
        .and_then(|a| nodes.iter().position(|n| a.is_same_node(Some(n.as_ref()))))
        .unwrap_or(0);
    let do_loop = context.do_loop.get_untracked();

    let next_index = if forward {
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
        && let Some(el) = nodes.get(idx)
    {
        // Arrow keys move focus and choose the item, as native radios do.
        let _ = el.focus();
        el.click();
    }
}

/// Whether `item` is the first enabled radio of its group, in document order. Before the item is
/// mounted (server rendering, first client pass) this is true, so the group is never left with
/// no tab stop. Tracks the registry, so it settles as items mount.
fn is_first_enabled(context: &RadioGroupContextValue, item: AnyNodeRef) -> bool {
    context.items.track();
    let Some(item) = item.get() else {
        return true;
    };
    match context.enabled_elements().first() {
        Some(first) => first.is_same_node(Some(&item)),
        None => true,
    }
}
