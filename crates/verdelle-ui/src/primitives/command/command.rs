use crate::primitives::core::listbox::{
    FilterQuery, ListboxEmpty, ListboxGroup, ListboxGroupHeading, ListboxItem, ListboxList,
    ListboxParams, ListboxSeparator, ListboxState,
};
use crate::primitives::core::primitive::{Primitive, VoidPrimitive};
use crate::primitives::core::use_controllable_state::{
    UseControllableStateParams, use_controllable_state,
};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy)]
struct CommandContext {
    list: ListboxState,
    search: Signal<String>,
    set_search: Callback<String>,
}

/// A search box over a list of actions, after cmdk. The list is always visible; the root owns
/// the search text and filters its items against it, and one item is highlighted at a time.
///
/// Focus stays in the [`CommandInput`]; the keys work from there: ArrowDown and ArrowUp move the
/// highlight (Ctrl or Cmd jumps to the first or last item), Home and End jump to the first and
/// last, PageDown and PageUp move ten, Enter runs the highlighted item.
///
/// `search` is controlled when given. Items are matched with `filter` ([`contains_filter`]
/// unless given); set `should_filter=false` to do the filtering yourself. `on_value_change` is
/// called with the highlighted item's value whenever the highlight moves.
///
/// [`contains_filter`]: crate::primitives::core::listbox::contains_filter
#[component]
pub fn Command(
    #[prop(into, optional)] search: MaybeProp<String>,
    #[prop(into, optional)] default_search: MaybeProp<String>,
    #[prop(into, optional)] on_search_change: Option<Callback<String>>,
    /// Called with the highlighted item's value (None when nothing is highlighted).
    #[prop(into, optional)]
    on_value_change: Option<Callback<Option<String>>>,
    #[prop(into, optional)] filter: Option<Callback<FilterQuery, bool>>,
    #[prop(into, optional)] should_filter: MaybeProp<bool>,
    /// Arrow keys wrap around the ends of the list. Off by default, as cmdk.
    #[prop(into, optional)]
    loop_around: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let (text, set_text) = use_controllable_state(UseControllableStateParams {
        prop: search,
        default_prop: default_search,
        on_change: on_search_change
            .map(|cb| Callback::new(move |v: Option<String>| cb.run(v.unwrap_or_default()))),
    });
    let search_signal = Signal::derive(move || text.get().unwrap_or_default());
    let set_search = Callback::new(move |v: String| set_text.run(Some(v)));

    let list = ListboxState::new(ListboxParams {
        search: search_signal,
        filter,
        should_filter: Signal::derive(move || should_filter.get().unwrap_or(true)),
        auto_highlight: Signal::derive(|| true),
        prefer_selected: false,
        wrap: Signal::derive(move || loop_around.get().unwrap_or(false)),
    });

    if let Some(cb) = on_value_change {
        Effect::new(move |_| {
            let value = list.active.get().and_then(|id| {
                web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(&id))
                    .and_then(|el| el.get_attribute("data-value"))
            });
            cb.run(value);
        });
    }

    let ctx = CommandContext {
        list,
        search: search_signal,
        set_search,
    };

    let on_keydown = move |event: KeyboardEvent| {
        if event.is_composing() {
            return;
        }
        let jump = event.ctrl_key() || event.meta_key();
        match event.key().as_str() {
            "ArrowDown" => {
                event.prevent_default();
                if jump {
                    list.go_last()
                } else {
                    list.move_by(1)
                }
            }
            "ArrowUp" => {
                event.prevent_default();
                if jump {
                    list.go_first()
                } else {
                    list.move_by(-1)
                }
            }
            "Home" => {
                event.prevent_default();
                list.go_first();
            }
            "End" => {
                event.prevent_default();
                list.go_last();
            }
            "PageDown" => {
                event.prevent_default();
                list.move_by(10);
            }
            "PageUp" => {
                event.prevent_default();
                list.move_by(-10);
            }
            // Running the highlighted item is the guard's side effect; with none, Enter is left alone.
            "Enter" if list.click_active() => event.prevent_default(),
            _ => {}
        }
    };

    view! {
        <Provider value=ctx>
            <Provider value=list>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-slot="command"
                on:keydown=on_keydown
            >
                {children.with_value(|c| c())}
            </Primitive>
            </Provider>
        </Provider>
    }
}

/// The search box. It has `role="combobox"` and drives the list with `aria-activedescendant`.
#[component]
pub fn CommandInput(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    let list = ctx.list;
    view! {
        <VoidPrimitive element=html::input as_child=as_child node_ref=node_ref
            attr:r#type="text"
            attr:role="combobox"
            attr:autocomplete="off"
            attr:autocapitalize="none"
            attr:spellcheck="false"
            attr:aria-autocomplete="list"
            attr:aria-expanded="true"
            attr:aria-controls=move || list.list_id_attr()
            attr:aria-activedescendant=move || list.active.get()
            attr:placeholder=move || placeholder.get()
            attr:disabled=move || disabled.get().unwrap_or(false).then_some("")
            attr:data-slot="input-group-control"
            prop:value=move || ctx.search.get()
            on:input=move |event: leptos::ev::Event| ctx.set_search.run(event_target_value(&event))
        >
            {""}
        </VoidPrimitive>
    }
}

/// The scrolling list of items (`role="listbox"`).
#[component]
pub fn CommandList(
    /// Accessible name of the list.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <ListboxList label=label as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </ListboxList>
    }
}

/// One action. `value` is its identity (what `on_select` receives); `label` (default: the
/// value) and `keywords` are what a search matches. It is hidden while it does not match.
#[component]
pub fn CommandItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] keywords: MaybeProp<Vec<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    /// Keep showing while the search does not match.
    #[prop(into, optional)]
    force_match: MaybeProp<bool>,
    /// Run when the item is picked (Enter or a click) with its value.
    #[prop(into, optional)]
    on_select: Option<Callback<String>>,
    /// Class for the item (see `ListboxItem`: `attr:class` does not reach it).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let picked = value.clone();
    view! {
        <ListboxItem
            value=value
            label=label
            keywords=keywords
            disabled=disabled
            force_match=force_match
            command_style=true
            class=class
            as_child=as_child
            node_ref=node_ref
            on_select=Callback::new(move |()| {
                if let Some(cb) = on_select {
                    cb.run(picked.clone());
                }
            })
        >
            {children.with_value(|c| c())}
        </ListboxItem>
    }
}

/// Items under one heading. Hidden while none of them match.
#[component]
pub fn CommandGroup(
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

/// The title of a [`CommandGroup`].
#[component]
pub fn CommandGroupHeading(
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

/// A divider; hidden while a search is narrowing the list unless `always_render`.
#[component]
pub fn CommandSeparator(
    #[prop(into, optional)] always_render: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! { <ListboxSeparator always_render=always_render as_child=as_child node_ref=node_ref /> }
}

/// Shown when nothing matches.
#[component]
pub fn CommandEmpty(
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
