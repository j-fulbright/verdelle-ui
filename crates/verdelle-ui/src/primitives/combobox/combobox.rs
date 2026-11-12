use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::dismissable_layer::use_dismissable_layer_with;
use crate::primitives::core::listbox::{
    FilterQuery, ListboxEmpty, ListboxGroup, ListboxGroupHeading, ListboxItem, ListboxList,
    ListboxParams, ListboxSeparator, ListboxState,
};
use crate::primitives::core::popper::{
    Popper, PopperAnchor, PopperContent, parse_align_opt, parse_side_or, use_popper_anchor_ref,
};
use crate::primitives::core::portal::Portal;
use crate::primitives::core::presence::{PortalKeep, keep_portal_mounted, use_presence};
use crate::primitives::core::primitive::{Primitive, VoidPrimitive};
use crate::primitives::core::use_controllable_state::{
    UseControllableStateParams, use_controllable_state,
};
use floating_ui_leptos::{Padding, Side};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
struct ComboboxContext {
    list: ListboxState,
    search: RwSignal<String>,
    open: Signal<bool>,
    set_open: Callback<bool>,
    multiple: bool,
    selected: Signal<Vec<String>>,
    input_value: Signal<String>,
    set_input_value: Callback<String>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    open_on_input_click: Signal<bool>,
    input_ref: AnyNodeRef,
    has_anchor: RwSignal<bool>,
    /// (value, label) of the items that have mounted. Items are only in the page while the
    /// popup is open.
    labels: RwSignal<Vec<(String, String)>>,
    /// (value, label) given up front, so closed inputs and chips can show text before the
    /// popup has ever opened.
    items: Signal<Vec<(String, String)>>,
    choose: Callback<(String, String)>,
    remove: Callback<String>,
    clear: Callback<()>,
}

impl ComboboxContext {
    fn label_for(&self, value: &str) -> String {
        let find = |list: &[(String, String)]| {
            list.iter()
                .find(|(v, _)| v == value)
                .map(|(_, label)| label.clone())
        };
        self.labels
            .with(|l| find(l))
            .or_else(|| self.items.with(|l| find(l)))
            .unwrap_or_else(|| value.to_string())
    }

    fn focus_input(&self) {
        let input = self.input_ref;
        request_animation_frame(move || {
            if let Some(el) = input.get_untracked()
                && let Ok(el) = el.dyn_into::<web_sys::HtmlElement>()
            {
                let _ = el.focus();
            }
        });
    }
}

/// The selected values of the enclosing [`Combobox`] with their labels, for rendering chips.
pub fn use_combobox_values() -> Signal<Vec<(String, String)>> {
    let ctx = expect_context::<ComboboxContext>();
    Signal::derive(move || {
        ctx.selected
            .get()
            .into_iter()
            .map(|v| {
                let label = ctx.label_for(&v);
                (v, label)
            })
            .collect()
    })
}

/// A text input with a filtered list of options under it.
///
/// Single selection (the default) holds one `value`; with `multiple` it holds `values` and
/// the input is for searching only (show the picks as [`ComboboxChip`]s). Each pair
/// (`value`, `on_value_change` / `values`, `on_values_change`, `input_value`, `open`) is
/// controlled when the prop is given and uncontrolled otherwise, with a `default_` form.
///
/// Items are matched against what is typed with `filter` ([`contains_filter`] unless given).
/// For results that arrive from a server, set `should_filter=false`, render the items you
/// got, and listen to `on_input_value_change`. `items` lists (value, label) pairs so the input
/// can show the label of a selected value before the popup has ever opened.
///
/// Keyboard, with focus in the input: ArrowDown/ArrowUp open the popup and move the highlight
/// (Ctrl or Cmd with an arrow jumps to the first or last item), PageDown/PageUp move ten,
/// Enter picks the highlighted item, Escape closes, Backspace on an empty input removes the
/// last pick when `multiple`. Home and End stay with the text caret.
#[component]
pub fn Combobox(
    /// Allow several values; the input then only searches.
    #[prop(optional)]
    multiple: bool,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    /// Called with the new value, or `None` when it was cleared.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<String>>>,
    #[prop(into, optional)] values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] input_value: MaybeProp<String>,
    #[prop(into, optional)] default_input_value: MaybeProp<String>,
    #[prop(into, optional)] on_input_value_change: Option<Callback<String>>,
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] form: MaybeProp<String>,
    /// (value, label) pairs known up front.
    #[prop(into, optional)]
    items: MaybeProp<Vec<(String, String)>>,
    /// How items are matched against the typed text.
    #[prop(into, optional)]
    filter: Option<Callback<FilterQuery, bool>>,
    /// Set to false when the items you render are already filtered (server search).
    #[prop(into, optional)]
    should_filter: MaybeProp<bool>,
    /// Highlight the first match as the user types (and the selected item on open). On by default.
    #[prop(into, optional)]
    auto_highlight: MaybeProp<bool>,
    /// Arrow keys wrap around the ends of the list. On by default.
    #[prop(into, optional)]
    loop_around: MaybeProp<bool>,
    /// Clicking the input opens the popup. On by default.
    #[prop(into, optional)]
    open_on_input_click: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let read_only = Signal::derive(move || read_only.get().unwrap_or(false));
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let name = Signal::derive(move || name.get());
    let form = Signal::derive(move || form.get());
    let items = Signal::derive(move || items.get().unwrap_or_default());
    let open_on_input_click = Signal::derive(move || open_on_input_click.get().unwrap_or(true));

    let (open_state, set_open_state) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.map(|cb| {
            Callback::new(move |v: Option<bool>| {
                if let Some(v) = v {
                    cb.run(v);
                }
            })
        }),
    });
    let open_signal = Signal::derive(move || open_state.get().unwrap_or(false) && !disabled.get());
    let set_open = Callback::new(move |v: bool| set_open_state.run(Some(v)));

    let (single, set_single) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change
            .map(|cb| Callback::new(move |v: Option<String>| cb.run(v.filter(|s| !s.is_empty())))),
    });
    let (many, set_many) = use_controllable_state(UseControllableStateParams {
        prop: values,
        default_prop: default_values,
        on_change: on_values_change
            .map(|cb| Callback::new(move |v: Option<Vec<String>>| cb.run(v.unwrap_or_default()))),
    });
    let selected = Signal::derive(move || {
        if multiple {
            many.get().unwrap_or_default()
        } else {
            single
                .get()
                .filter(|v| !v.is_empty())
                .into_iter()
                .collect::<Vec<_>>()
        }
    });
    let set_selected = Callback::new(move |next: Vec<String>| {
        if multiple {
            set_many.run(Some(next));
        } else {
            set_single.run(Some(next.into_iter().next().unwrap_or_default()));
        }
    });

    let labels: RwSignal<Vec<(String, String)>> = RwSignal::new(Vec::new());
    let label_of = move |value: &str| {
        labels
            .with_untracked(|l| l.iter().find(|(v, _)| v == value).map(|(_, l)| l.clone()))
            .or_else(|| {
                items.with_untracked(|l| l.iter().find(|(v, _)| v == value).map(|(_, l)| l.clone()))
            })
            .unwrap_or_else(|| value.to_string())
    };

    // The text in the input. In single mode it starts as the label of the selected value.
    let initial_text = input_value
        .get_untracked()
        .or_else(|| default_input_value.get_untracked())
        .unwrap_or_else(|| {
            if multiple {
                String::new()
            } else {
                selected
                    .get_untracked()
                    .first()
                    .map(|v| label_of(v))
                    .unwrap_or_default()
            }
        });
    let (text, set_text) = use_controllable_state(UseControllableStateParams {
        prop: input_value,
        default_prop: Some(initial_text).into(),
        on_change: on_input_value_change
            .map(|cb| Callback::new(move |v: Option<String>| cb.run(v.unwrap_or_default()))),
    });
    let input_signal = Signal::derive(move || text.get().unwrap_or_default());
    let set_input_value = Callback::new(move |v: String| set_text.run(Some(v)));

    // What the user has typed since the popup opened. It filters the list; the input's own
    // text may instead be a selected item's label, which must not filter.
    let search = RwSignal::new(String::new());
    let list = ListboxState::new(ListboxParams {
        search: search.into(),
        filter,
        should_filter: Signal::derive(move || should_filter.get().unwrap_or(true)),
        auto_highlight: Signal::derive(move || auto_highlight.get().unwrap_or(true)),
        prefer_selected: !multiple,
        wrap: Signal::derive(move || loop_around.get().unwrap_or(true)),
    });

    let input_ref = AnyNodeRef::new();

    let choose = Callback::new(move |(value, label): (String, String)| {
        labels.update(|l| {
            if !l.iter().any(|(v, _)| v == &value) {
                l.push((value.clone(), label.clone()));
            }
        });
        if multiple {
            let mut current = selected.get_untracked();
            if let Some(pos) = current.iter().position(|v| v == &value) {
                current.remove(pos);
            } else {
                current.push(value);
            }
            set_selected.run(current);
            set_input_value.run(String::new());
            search.set(String::new());
        } else {
            set_selected.run(vec![value]);
            set_input_value.run(label);
            search.set(String::new());
            set_open.run(false);
        }
    });
    let remove = Callback::new(move |value: String| {
        let mut current = selected.get_untracked();
        current.retain(|v| v != &value);
        set_selected.run(current);
    });
    let clear = Callback::new(move |()| {
        set_selected.run(Vec::new());
        set_input_value.run(String::new());
        search.set(String::new());
    });

    let has_anchor = RwSignal::new(false);
    let ctx = ComboboxContext {
        list,
        search,
        open: open_signal,
        set_open,
        multiple,
        selected,
        input_value: input_signal,
        set_input_value,
        disabled,
        read_only,
        open_on_input_click,
        input_ref,
        has_anchor,
        labels,
        items,
        choose,
        remove,
        clear,
    };

    // A selection changed from outside (controlled, or cleared): show it in the input.
    Effect::new(move |previous: Option<Vec<String>>| {
        let current = selected.get();
        if let Some(previous) = previous
            && previous != current
            && !multiple
            && !ctx.open.get_untracked()
        {
            let label = current
                .first()
                .map(|v| ctx.label_for(v))
                .unwrap_or_default();
            set_input_value.run(label);
        }
        current
    });

    // The popup closed: forget the search and the highlight, and put the input back to showing
    // the selection (single) or nothing (multiple).
    Effect::new(move |previous: Option<bool>| {
        let is_open = ctx.open.get();
        if previous == Some(true) && !is_open {
            search.set(String::new());
            let label = if multiple {
                String::new()
            } else {
                selected
                    .get_untracked()
                    .first()
                    .map(|v| ctx.label_for(v))
                    .unwrap_or_default()
            };
            if ctx.input_value.get_untracked() != label {
                set_input_value.run(label);
            }
        }
        if !is_open {
            list.active.set(None);
        }
        is_open
    });

    view! {
        <Popper>
            <Provider value=ctx>
                <Provider value=list>
                {children.with_value(|c| c())}
                {move || {
                    name.get().map(|name| {
                        let form = form.get();
                        if multiple {
                            selected
                                .get()
                                .into_iter()
                                .map(|v| view! { <input type="hidden" name=name.clone() value=v form=form.clone() /> })
                                .collect_view()
                                .into_any()
                        } else {
                            let v = selected.get().into_iter().next().unwrap_or_default();
                            if required.get() {
                                // Browsers skip validation on type=hidden, so a required value
                                // needs a (visually hidden, unfocusable) text field to speak for it.
                                view! {
                                    <input
                                        type="text"
                                        name=name
                                        value=v
                                        form=form
                                        required=true
                                        tabindex="-1"
                                        aria-hidden="true"
                                        style="position:absolute;width:1px;height:1px;opacity:0;pointer-events:none;"
                                    />
                                }
                                .into_any()
                            } else {
                                view! { <input type="hidden" name=name value=v form=form /> }
                                    .into_any()
                            }
                        }
                    })
                }}
                </Provider>
            </Provider>
        </Popper>
    }
}

/// The element the popup is placed against and as wide as: the input's frame, or the chips
/// container. Put the [`ComboboxInput`] inside it. Without one the input itself is the anchor.
#[component]
pub fn ComboboxAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    ctx.has_anchor.set(true);
    on_cleanup(move || {
        ctx.has_anchor.try_set(false);
    });

    view! {
        <PopperAnchor as_child=as_child node_ref=node_ref
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
        >
            {children.with_value(|c| c())}
        </PopperAnchor>
    }
}

/// The text input. It has `role="combobox"`; the list is reached with `aria-activedescendant`,
/// so focus never leaves it.
#[component]
pub fn ComboboxInput(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();
    let mut refs = vec![node_ref, ctx.input_ref];
    if !ctx.has_anchor.get_untracked() {
        refs.push(use_popper_anchor_ref());
    }
    let refs = use_composed_refs(refs);
    let list = ctx.list;

    let on_keydown = move |event: KeyboardEvent| {
        if ctx.disabled.get_untracked() || event.is_composing() {
            return;
        }
        let open = ctx.open.get_untracked();
        let jump = event.ctrl_key() || event.meta_key();
        match event.key().as_str() {
            "ArrowDown" | "ArrowUp" => {
                event.prevent_default();
                let down = event.key() == "ArrowDown";
                if ctx.read_only.get_untracked() {
                    return;
                }
                if !open {
                    ctx.set_open.run(true);
                } else if jump {
                    if down {
                        list.go_last()
                    } else {
                        list.go_first()
                    }
                } else {
                    list.move_by(if down { 1 } else { -1 });
                }
            }
            "PageDown" | "PageUp" if open => {
                event.prevent_default();
                list.move_by(if event.key() == "PageDown" { 10 } else { -10 });
            }
            "Enter" if open => {
                // Never submit a form while a list is up; pick what is highlighted.
                event.prevent_default();
                list.click_active();
            }
            "Escape" if open => {
                // Keep an enclosing dialog from closing as well.
                event.prevent_default();
                event.stop_propagation();
                ctx.set_open.run(false);
            }
            "Tab" if open => ctx.set_open.run(false),
            "Backspace" if ctx.multiple && ctx.input_value.get_untracked().is_empty() => {
                if let Some(last) = ctx.selected.get_untracked().last().cloned() {
                    ctx.remove.run(last);
                }
            }
            _ => {}
        }
    };

    view! {
        <VoidPrimitive element=html::input as_child=as_child node_ref=refs
            attr:r#type="text"
            attr:role="combobox"
            attr:autocomplete="off"
            attr:autocapitalize="none"
            attr:spellcheck="false"
            attr:aria-autocomplete="list"
            attr:aria-haspopup="listbox"
            attr:aria-expanded=move || ctx.open.get().to_string()
            attr:aria-controls=move || if ctx.open.get() { list.list_id_attr() } else { None }
            attr:aria-activedescendant=move || if ctx.open.get() { list.active.get() } else { None }
            attr:disabled=move || ctx.disabled.get().then_some("")
            attr:readonly=move || ctx.read_only.get().then_some("")
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            attr:data-slot="input-group-control"
            prop:value=move || ctx.input_value.get()
            on:input=move |event: leptos::ev::Event| {
                let text = event_target_value(&event);
                ctx.set_input_value.run(text.clone());
                ctx.search.set(text);
                if !ctx.open.get_untracked() && !ctx.read_only.get_untracked() {
                    ctx.set_open.run(true);
                }
            }
            on:click=move |_| {
                if ctx.open_on_input_click.get_untracked()
                    && !ctx.open.get_untracked()
                    && !ctx.read_only.get_untracked()
                    && !ctx.disabled.get_untracked()
                {
                    ctx.set_open.run(true);
                }
            }
            on:blur=move |_| ctx.set_open.run(false)
            on:keydown=on_keydown
        >
            {""}
        </VoidPrimitive>
    }
}

/// A button that opens and closes the popup. It never takes focus from the input.
#[component]
pub fn ComboboxTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:tabindex="-1"
            attr:aria-label="Toggle options"
            attr:aria-haspopup="listbox"
            attr:aria-expanded=move || ctx.open.get().to_string()
            attr:aria-controls=move || if ctx.open.get() { ctx.list.list_id_attr() } else { None }
            attr:disabled=move || ctx.disabled.get().then_some("")
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            on:mousedown=move |event: web_sys::MouseEvent| event.prevent_default()
            on:click=move |_| {
                if !ctx.disabled.get_untracked() && !ctx.read_only.get_untracked() {
                    ctx.set_open.run(!ctx.open.get_untracked());
                    ctx.focus_input();
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// A button that clears the selection and the text. It is `hidden` while both are empty.
#[component]
pub fn ComboboxClear(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    let has_value = move || !ctx.selected.get().is_empty() || !ctx.input_value.get().is_empty();
    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:tabindex="-1"
            attr:aria-label="Clear"
            attr:hidden=move || (!has_value()).then_some("")
            attr:disabled=move || ctx.disabled.get().then_some("")
            on:mousedown=move |event: web_sys::MouseEvent| event.prevent_default()
            on:click=move |_| {
                if !ctx.disabled.get_untracked() && !ctx.read_only.get_untracked() {
                    ctx.clear.run(());
                    ctx.focus_input();
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// The row that holds the chips and the input of a `multiple` combobox. Pressing it
/// (outside a chip's remove button) focuses the input.
#[component]
pub fn ComboboxChips(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    ctx.has_anchor.set(true);
    on_cleanup(move || {
        ctx.has_anchor.try_set(false);
    });
    let refs = use_composed_refs(vec![node_ref, use_popper_anchor_ref()]);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=refs
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || ctx.disabled.get().then_some("")
            on:click=move |event: web_sys::MouseEvent| {
                let on_button = event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .and_then(|el| el.closest("button").ok().flatten())
                    .is_some();
                if !on_button && !ctx.disabled.get_untracked() {
                    ctx.focus_input();
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[derive(Clone)]
struct ChipContext {
    value: String,
}

/// One pick, shown as a chip. Put the label in it, plus a [`ComboboxChipRemove`].
#[component]
pub fn ComboboxChip(
    #[prop(into)] value: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Provider value=ChipContext { value }>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-slot="combobox-chip"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

/// The button that removes its chip's value.
#[component]
pub fn ComboboxChipRemove(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    let chip = expect_context::<ChipContext>();
    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:tabindex="-1"
            attr:aria-label="Remove"
            attr:disabled=move || ctx.disabled.get().then_some("")
            on:mousedown=move |event: web_sys::MouseEvent| event.prevent_default()
            on:click={
                let value = chip.value.clone();
                move |_| {
                    if !ctx.disabled.get_untracked() && !ctx.read_only.get_untracked() {
                        ctx.remove.run(value.clone());
                        ctx.focus_input();
                    }
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ComboboxPortal(
    /// Target container element to portal into. Defaults to `document.body`.
    #[prop(into, optional)]
    container: MaybeProp<SendWrapper<web_sys::Element>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    let list = ctx.list;
    let keep = PortalKeep::new();
    view! {
        <Provider value=keep>
        <Show when=move || ctx.open.get() || keep.get()>
            <Portal container=container>
                <Provider value=ctx>
                    <Provider value=list>
                        {children.with_value(|c| c())}
                    </Provider>
                </Provider>
            </Portal>
        </Show>
        </Provider>
    }
}

/// The popup: placed under the anchor and as wide as it, flips above when there is no room.
/// `class` and `style` reach the popup itself; `attr:class` would land on the positioning
/// wrapper, whose transform places it.
#[component]
pub fn ComboboxContent(
    /// "top" | "right" | "bottom" | "left"
    #[prop(into, optional)]
    side: MaybeProp<String>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    /// "start" | "center" | "end"
    #[prop(into, optional)]
    align: MaybeProp<String>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    #[prop(into, optional)] collision_padding: MaybeProp<f64>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    let list = ctx.list;

    let popper_side = Signal::derive(move || parse_side_or(side.get().as_deref(), Side::Bottom));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align = Signal::derive(move || parse_align_opt(align.get().as_deref()));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding =
        Signal::derive(move || Padding::All(collision_padding.get().unwrap_or(0.0)));

    let present = ctx.open;
    let presence = use_presence(present);
    keep_portal_mounted(presence.is_present);

    // A press on the anchor (the input, its trigger) is not an outside press: the input
    // handles its own clicks, and dismissing here would reopen the popup straight after.
    let anchor = use_popper_anchor_ref();
    let on_pointer_down_outside = Callback::new(move |event: web_sys::PointerEvent| {
        let on_anchor = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .zip(anchor.get_untracked())
            .is_some_and(|(target, anchor)| anchor.contains(Some(&target)));
        if on_anchor {
            event.prevent_default();
        }
    });
    let dismiss_ref = use_dismissable_layer_with(
        None,
        Some(on_pointer_down_outside),
        None,
        None,
        Some(Callback::new(move |()| ctx.set_open.run(false))),
        Signal::derive(move || !ctx.open.get()),
        // Focus stays in the input, which is outside the popup; losing it is handled there.
        false,
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, dismiss_ref]);

    view! {
        <Show when=move || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                class=class
                style=style
                as_child=as_child
                node_ref=refs
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                attr:data-empty=move || (list.ready.get() && list.visible_count.get() == 0).then_some("")
            >
                {children.with_value(|c| c())}
            </PopperContent>
        </Show>
    }
}

/// The scrolling list of options inside the popup (`role="listbox"`).
#[component]
pub fn ComboboxList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    view! {
        <ListboxList multiselectable=ctx.multiple as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </ListboxList>
    }
}

#[derive(Clone, Copy)]
struct ItemContext {
    selected: Signal<bool>,
}

/// One option. `value` is what the combobox holds; `label` (default: the value) is what it is
/// matched on and what the input shows once it is picked.
#[component]
pub fn ComboboxItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] keywords: MaybeProp<Vec<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    /// Class for the option (see `ListboxItem`: `attr:class` does not reach it).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ComboboxContext>();
    let value = StoredValue::new(value);
    let text = Signal::derive(move || label.get().unwrap_or_else(|| value.get_value()));
    let selected = Signal::derive(move || {
        ctx.selected
            .with(|sel| value.with_value(|v| sel.iter().any(|s| s == v)))
    });

    // Tell the combobox what this value is called, for the input and the chips.
    Effect::new(move |_| {
        let (v, l) = (value.get_value(), text.get());
        ctx.labels.update(
            |labels| match labels.iter_mut().find(|(known, _)| *known == v) {
                Some(entry) => entry.1 = l,
                None => labels.push((v, l)),
            },
        );
    });

    view! {
        <Provider value=ItemContext { selected }>
            <ListboxItem
                value=value.get_value()
                label=label
                keywords=keywords
                disabled=disabled
                selected=selected
                class=class
                as_child=as_child
                node_ref=node_ref
                on_select=Callback::new(move |()| {
                    ctx.choose.run((value.get_value(), text.get_untracked()));
                    if let Some(cb) = on_select {
                        cb.run(());
                    }
                })
            >
                {children.with_value(|c| c())}
            </ListboxItem>
        </Provider>
    }
}

/// A check mark: `hidden` unless the item is selected.
#[component]
pub fn ComboboxItemIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let item = expect_context::<ItemContext>();
    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref
            attr:aria-hidden="true"
            attr:hidden=move || (!item.selected.get()).then_some("")
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// A set of options with a [`ComboboxLabel`]; it hides while none of its options match.
#[component]
pub fn ComboboxGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <ListboxGroup as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </ListboxGroup>
    }
}

/// The title of a [`ComboboxGroup`].
#[component]
pub fn ComboboxLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <ListboxGroupHeading as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </ListboxGroupHeading>
    }
}

/// A divider between groups; hidden while a search narrows the list.
#[component]
pub fn ComboboxSeparator(
    #[prop(into, optional)] always_render: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! { <ListboxSeparator always_render=always_render as_child=as_child node_ref=node_ref /> }
}

/// Shown when nothing matches.
#[component]
pub fn ComboboxEmpty(
    /// Class for the message (`attr:class` does not reach it while it is hidden).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <ListboxEmpty class=class as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </ListboxEmpty>
    }
}
