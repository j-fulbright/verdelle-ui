use crate::primitives::core::primitive::Primitive;
use crate::primitives::core::use_controllable_state::{
    UseControllableStateParams, use_controllable_state,
};
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
struct CollapsibleContextValue {
    content_id: String,
    open: Signal<bool>,
    disabled: Signal<bool>,
    on_open_toggle: Callback<()>,
}

/// A handle to the open state of a collapsible, for a trigger and content that do not share one
/// [`Collapsible`] wrapper (for example in different table rows). Make one with
/// [`use_collapsible`] and pass it as `state` to the parts that should share it. It is `Copy`.
#[derive(Clone, Copy, Debug)]
pub struct CollapsibleState {
    /// Whether the content is shown. Read it, or set it to open and close from code.
    pub open: RwSignal<bool>,
    /// Whether the trigger is disabled.
    pub disabled: RwSignal<bool>,
    content_id: StoredValue<String>,
}

impl CollapsibleState {
    /// The `data-state` value, `"open"` or `"closed"`. Reactive: read it inside a closure or an
    /// attribute to style anything (a rotating chevron elsewhere in a row) from the same state.
    pub fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }

    /// Flip the open state.
    pub fn toggle(&self) {
        self.open.update(|open| *open = !*open);
    }

    fn context(&self) -> CollapsibleContextValue {
        let open = self.open;
        CollapsibleContextValue {
            content_id: self.content_id.get_value(),
            open: open.into(),
            disabled: self.disabled.into(),
            on_open_toggle: Callback::new(move |()| open.update(|o| *o = !*o)),
        }
    }
}

/// Create a [`CollapsibleState`] starting at `default_open`. Call it where the state should live
/// (a component's body), then pass it as `state` to a collapsible's trigger and content.
pub fn use_collapsible(default_open: bool) -> CollapsibleState {
    CollapsibleState {
        open: RwSignal::new(default_open),
        disabled: RwSignal::new(false),
        content_id: StoredValue::new(format!(
            "vdl-collapsible-content-{}",
            crate::primitives::core::id::use_id(None).get_untracked()
        )),
    }
}

#[component]
pub fn Collapsible(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    /// Share this state with a trigger and content elsewhere (see [`use_collapsible`]). When
    /// given, it replaces `open`, `default_open` and `on_open_change`.
    #[prop(optional, into)]
    state: MaybeProp<CollapsibleState>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let state = state.get_untracked();
    let disabled = Signal::derive(move || {
        disabled.get().unwrap_or(false) || state.is_some_and(|s| s.disabled.get())
    });

    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        on_change: on_open_change.map(|cb| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_open,
    });
    let open = Signal::derive(move || open.get().unwrap_or(false));

    let mut context_value = CollapsibleContextValue {
        content_id: format!(
            "vdl-collapsible-content-{}",
            crate::primitives::core::id::use_id(None).get_untracked()
        ),
        open,
        disabled,
        on_open_toggle: Callback::new(move |()| {
            set_open.run(Some(!open.get_untracked()));
        }),
    };
    if let Some(state) = state {
        context_value = state.context();
        context_value.disabled = disabled;
    }
    let open = context_value.open;

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || if open.get() { "open" } else { "closed" }
                attr:data-disabled=move || disabled.get().then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn CollapsibleTrigger(
    /// Bind to a shared state instead of the enclosing [`Collapsible`].
    #[prop(optional, into)]
    state: MaybeProp<CollapsibleState>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = state
        .get_untracked()
        .map(|s| s.context())
        .unwrap_or_else(expect_context::<CollapsibleContextValue>);

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:aria-controls=context.content_id.clone()
            attr:aria-expanded=move || context.open.get().to_string()
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || context.disabled.get().then_some("")
            attr:disabled=move || context.disabled.get().then_some("")
            on:click=move |_| context.on_open_toggle.run(())
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn CollapsibleContent(
    /// Used to force mounting when more control is needed.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    /// Bind to a shared state instead of the enclosing [`Collapsible`].
    #[prop(optional, into)]
    state: MaybeProp<CollapsibleState>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = state
        .get_untracked()
        .map(|s| s.context())
        .unwrap_or_else(expect_context::<CollapsibleContextValue>);
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    // Ref to the content element for measuring height
    let content_ref =
        crate::primitives::core::compose_refs::use_composed_refs(vec![node_ref, AnyNodeRef::new()]);

    // Track whether we've ever measured (first open)
    let measured_height = RwSignal::new(0.0f64);
    let measured_width = RwSignal::new(0.0f64);
    // Track if content is "present" (stays true during close animation)
    let is_present = RwSignal::new(context.open.get_untracked());

    // When opening: immediately set present=true so content renders.
    // When closing: keep present=true until animation ends; if no animation, unmount immediately.
    Effect::new(move |_| {
        if context.open.get() {
            is_present.set(true);
        } else if is_present.get_untracked() {
            // Check if the element has a CSS animation. If not, unmount immediately
            // instead of waiting for an animationend event that will never fire.
            let has_animation = content_ref
                .get()
                .and_then(|node| {
                    web_sys::window()?
                        .get_computed_style(node.as_ref())
                        .ok()
                        .flatten()
                })
                .and_then(|style| style.get_property_value("animation-name").ok())
                .is_some_and(|name| name != "none" && !name.is_empty());

            if !has_animation {
                is_present.set(false);
            }
            // Otherwise, animationend handler will clear is_present.
        }
    });

    // Measure the content's natural height whenever it becomes present
    Effect::new(move |_| {
        let _present = is_present.get();
        let _open = context.open.get();

        // Use request_animation_frame to measure after the DOM has rendered
        if let Some(node) = content_ref.get() {
            let node: web_sys::HtmlElement = node.unchecked_ref::<web_sys::HtmlElement>().clone();
            // Defer measurement to next frame so content has rendered
            let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                let style = node.style();
                // Temporarily show content at natural height to measure
                let _ = style.set_property("height", "auto");
                let _ = style.set_property("animation", "none");
                let rect = node.get_bounding_client_rect();
                let h = rect.height();
                let w = rect.width();
                measured_height.set(h);
                measured_width.set(w);
                // Set CSS variables for animations
                let _ = style.set_property("--vdl-collapsible-content-height", &format!("{h}px"));
                let _ = style.set_property("--vdl-collapsible-content-width", &format!("{w}px"));
                let _ = style.set_property("--radix-collapsible-content-height", &format!("{h}px"));
                let _ = style.set_property("--radix-collapsible-content-width", &format!("{w}px"));
                let _ = style.set_property("--radix-accordion-content-height", &format!("{h}px"));
                let _ = style.set_property("--vdl-accordion-content-height", &format!("{h}px"));
                // Remove overrides so CSS animations can take over
                let _ = style.remove_property("height");
                let _ = style.remove_property("animation");
            });
            if let Some(w) = web_sys::window() {
                let _ = w.request_animation_frame(cb.as_ref().unchecked_ref());
            }
        }
    });

    // Handle close: listen for animationend to remove content
    let on_animation_end = move |_: web_sys::AnimationEvent| {
        if !context.open.get_untracked() {
            is_present.set(false);
        }
    };

    let should_render =
        Signal::derive(move || context.open.get() || is_present.get() || force_mount.get());

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=content_ref
            attr:id=context.content_id.clone()
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || context.disabled.get().then_some("")
            // A table row (as_child on a `<tr>`) keeps its own role.
            attr:role=move || (!as_child.get().unwrap_or(false)).then_some("region")
            attr:hidden=move || (!should_render.get()).then_some("")
            on:animationend=on_animation_end
        >
            {move || {
                if should_render.get() {
                    Some(children.with_value(|children| children()))
                } else {
                    None
                }
            }}
        </Primitive>
    }
}
