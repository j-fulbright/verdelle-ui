//! The list behind Combobox and Command: items that filter themselves against a search string,
//! one highlighted ("active") item, keyboard movement through whatever is currently shown, and
//! the bits that depend on how many items are showing (an empty message, groups that hide
//! when none of their items match).
//!
//! Verdelle-original: Leptix has no Combobox or Command. The design follows cmdk (Command) and
//! Base UI's combobox: focus stays in the text input and the list is driven with
//! `aria-activedescendant`, so options are never focused.
//!
//! The DOM is the source of truth for order. An item that does not match the search renders
//! nothing, so "the next item" is the next `[role=option]` in the list element. That keeps groups,
//! headings and dynamically loaded items working without a registry that has to be kept in sync
//! with the markup.

use crate::primitives::core::compose_refs::use_composed_refs;
use crate::primitives::core::primitive::Primitive;
use leptos::{context::Provider, ev::PointerEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use std::cell::Cell;
use web_sys::wasm_bindgen::JsCast;

thread_local! {
    static NEXT_DOM_ID: Cell<u64> = const { Cell::new(0) };
}

/// A fresh element id. Ids here are made in the browser only, after hydration, never while
/// rendering on the server: `use_id`'s counter is order-dependent, and the server and the client
/// do not create components in the same order (the sidebar's switcher is built at a different
/// moment), so ids made during render do not match between the two and the highlight could not
/// find its element. The server's markup simply has no ids until the page is interactive.
fn next_dom_id(kind: &str) -> String {
    NEXT_DOM_ID.with(|c| {
        c.set(c.get() + 1);
        format!("vdl-lb-{kind}-{}", c.get())
    })
}

/// An id signal that is empty until the browser fills it in.
fn client_id(kind: &'static str) -> RwSignal<String> {
    let id = RwSignal::new(String::new());
    Effect::new(move |_| id.set(next_dom_id(kind)));
    id
}

/// `Some(id)` once the browser has made it.
fn present(id: RwSignal<String>) -> Option<String> {
    let id = id.get();
    (!id.is_empty()).then_some(id)
}

/// What a filter is asked about one item.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FilterQuery {
    /// The item's identity.
    pub value: String,
    /// The text the item shows (its `label`, or its `value` when it has none).
    pub label: String,
    /// Extra words the item should match that it does not show.
    pub keywords: Vec<String>,
    /// What the user typed.
    pub search: String,
}

impl FilterQuery {
    /// The label and keywords as one lower-case string, the text a search is matched against.
    pub fn haystack(&self) -> String {
        let mut text = self.label.to_lowercase();
        for keyword in &self.keywords {
            text.push(' ');
            text.push_str(&keyword.to_lowercase());
        }
        text
    }
}

/// The default filter: every word of the search must appear somewhere in the label or the
/// keywords, in any order, ignoring case. An empty search matches everything.
pub fn contains_filter(query: &FilterQuery) -> bool {
    let haystack = query.haystack();
    query
        .search
        .to_lowercase()
        .split_whitespace()
        .all(|term| haystack.contains(term))
}

/// Matches when the label (or a keyword) starts with the search.
pub fn starts_with_filter(query: &FilterQuery) -> bool {
    let search = query.search.trim().to_lowercase();
    search.is_empty()
        || query.label.to_lowercase().starts_with(&search)
        || query
            .keywords
            .iter()
            .any(|k| k.to_lowercase().starts_with(&search))
}

/// Matches when the search's letters appear in the label or keywords in order, not necessarily
/// next to each other (`"gh"` matches `"GitHub"`). Spaces in the search are ignored.
pub fn fuzzy_filter(query: &FilterQuery) -> bool {
    let haystack = query.haystack();
    let mut letters = haystack.chars();
    query
        .search
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .all(|wanted| letters.any(|c| c == wanted))
}

/// Which index a keyboard move lands on. `current` is the highlighted index, if any; `delta`
/// is how far to move (negative goes back). With `wrap` the ends join up; without it the move
/// stops at the ends. With nothing highlighted, a forward move lands on the first item and a
/// backward one on the last. `None` when the list is empty.
pub fn move_index(len: usize, current: Option<usize>, delta: isize, wrap: bool) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let last = len as isize - 1;
    let Some(current) = current else {
        return Some(if delta >= 0 { 0 } else { last as usize });
    };
    let target = current as isize + delta;
    let target = if wrap {
        target.rem_euclid(len as isize)
    } else {
        target.clamp(0, last)
    };
    Some(target as usize)
}

/// Everything the list parts share. Provided by the root of a Combobox or Command.
#[derive(Clone, Copy)]
pub struct ListboxState {
    /// What the user has typed. Items are matched against it.
    pub search: Signal<String>,
    /// The matcher. [`contains_filter`] unless the consumer gave one.
    pub filter: Callback<FilterQuery, bool>,
    /// False when the consumer filters the data itself (async results): items always show.
    pub should_filter: Signal<bool>,
    /// The id of the highlighted item.
    pub active: RwSignal<Option<String>>,
    /// The element holding the options; arrow keys move through the options inside it.
    pub list_ref: AnyNodeRef,
    /// The id the list element carries, for `aria-controls`. Empty until the browser has made it;
    /// read it with [`ListboxState::list_id_attr`].
    pub list_id: RwSignal<String>,
    /// How many items currently match.
    pub visible_count: RwSignal<usize>,
    /// True once the page is interactive. Counts are filled in by effects, which do not run
    /// during server rendering, so "nothing matches" is only believed after this.
    pub ready: RwSignal<bool>,
    /// Highlight the first match whenever the search changes (and when the list appears).
    pub auto_highlight: Signal<bool>,
    /// When the list appears, highlight the selected item instead of the first.
    pub prefer_selected: bool,
    /// Arrow keys wrap from the last item to the first and back.
    pub wrap: Signal<bool>,
    validate_pending: StoredValue<bool>,
    force_first: StoredValue<bool>,
    scroll_next: StoredValue<bool>,
}

pub struct ListboxParams {
    pub search: Signal<String>,
    pub filter: Option<Callback<FilterQuery, bool>>,
    pub should_filter: Signal<bool>,
    pub auto_highlight: Signal<bool>,
    pub prefer_selected: bool,
    pub wrap: Signal<bool>,
}

impl ListboxState {
    /// Create the state and the effects that keep the highlight valid. Call once, in the root
    /// component, and provide the result as context.
    pub fn new(params: ListboxParams) -> Self {
        let state = Self {
            search: params.search,
            filter: params
                .filter
                .unwrap_or_else(|| Callback::new(|q: FilterQuery| contains_filter(&q))),
            should_filter: params.should_filter,
            active: RwSignal::new(None),
            list_ref: AnyNodeRef::new(),
            list_id: client_id("list"),
            visible_count: RwSignal::new(0),
            ready: RwSignal::new(false),
            auto_highlight: params.auto_highlight,
            prefer_selected: params.prefer_selected,
            wrap: params.wrap,
            validate_pending: StoredValue::new(false),
            force_first: StoredValue::new(false),
            scroll_next: StoredValue::new(false),
        };

        // Browser only: effects never run while rendering on the server.
        Effect::new(move |_| state.ready.set(true));

        // A new search starts a new result list: highlight its first item.
        Effect::new(move |previous: Option<()>| {
            state.search.track();
            if previous.is_some() {
                state.force_first.set_value(true);
                state.schedule_validate();
            }
        });

        // The list element appeared (the popup opened): highlight something in it.
        Effect::new(move |_| {
            if state.list_ref.get().is_some() {
                state.force_first.set_value(true);
                state.schedule_validate();
            }
        });

        // Keep the highlighted item in view after keyboard movement.
        Effect::new(move |_| {
            if state.active.get().is_some() && state.scroll_next.get_value() {
                state.scroll_next.set_value(false);
                // A popup is sized after it mounts (the available height arrives from the
                // positioning step), so the first scroll can be against a list that is not yet
                // scrollable. Repeat it as the layout settles; it is idempotent.
                request_animation_frame(move || {
                    state.scroll_active_into_view();
                    request_animation_frame(move || state.scroll_active_into_view());
                });
                set_timeout(
                    move || state.scroll_active_into_view(),
                    std::time::Duration::from_millis(80),
                );
            }
        });

        state
    }

    /// The list's id for an attribute: `None` until the browser has made it.
    pub fn list_id_attr(&self) -> Option<String> {
        present(self.list_id)
    }

    /// True while a search is narrowing the list.
    pub fn is_filtering(&self) -> bool {
        self.should_filter.get() && !self.search.get().trim().is_empty()
    }

    /// Whether an item should show for the current search. Reactive.
    pub fn matches(&self, value: &str, label: &str, keywords: &[String]) -> bool {
        if !self.is_filtering() {
            return true;
        }
        self.filter.run(FilterQuery {
            value: value.to_string(),
            label: label.to_string(),
            keywords: keywords.to_vec(),
            search: self.search.get(),
        })
    }

    /// False once the owning component has been disposed. Frame and timer callbacks scheduled
    /// earlier can still fire after that (a palette closing in a dialog).
    fn alive(&self) -> bool {
        self.active.try_get_untracked().is_some()
    }

    /// The enabled options in the list, in document order.
    fn options(&self) -> Vec<web_sys::HtmlElement> {
        if !self.alive() {
            return Vec::new();
        }
        let Some(list) = self.list_ref.get_untracked() else {
            return Vec::new();
        };
        let Ok(nodes) = list.query_selector_all("[role='option']:not([aria-disabled='true'])")
        else {
            return Vec::new();
        };
        (0..nodes.length())
            .filter_map(|i| nodes.item(i))
            .filter_map(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
            .collect()
    }

    /// Highlight an item by element id. `scroll` brings it into view (keyboard movement does,
    /// the pointer does not: the item is already under it).
    pub fn set_active(&self, id: Option<String>, scroll: bool) {
        if self.active.get_untracked() == id {
            return;
        }
        self.scroll_next.set_value(scroll);
        self.active.set(id);
    }

    /// Move the highlight by `delta` items among the enabled, matching ones.
    pub fn move_by(&self, delta: isize) {
        let options = self.options();
        let current = self
            .active
            .get_untracked()
            .and_then(|id| options.iter().position(|el| el.id() == id));
        let wrap = self.wrap.get_untracked() && delta.abs() == 1;
        if let Some(i) = move_index(options.len(), current, delta, wrap) {
            self.set_active(Some(options[i].id()), true);
        }
    }

    pub fn go_first(&self) {
        let options = self.options();
        if let Some(first) = options.first() {
            self.set_active(Some(first.id()), true);
        }
    }

    pub fn go_last(&self) {
        let options = self.options();
        if let Some(last) = options.last() {
            self.set_active(Some(last.id()), true);
        }
    }

    /// Activate the highlighted item as if it had been clicked. Returns whether there was one.
    pub fn click_active(&self) -> bool {
        let Some(id) = self.active.get_untracked() else {
            return false;
        };
        let element = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&id))
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
        match element {
            Some(el) if el.get_attribute("aria-disabled").as_deref() != Some("true") => {
                el.click();
                true
            }
            _ => false,
        }
    }

    /// Ask for the highlight to be checked after the DOM has settled: if the highlighted item
    /// is gone (filtered out, unmounted) pick another. Many callers in one frame cost one check.
    pub fn schedule_validate(&self) {
        if self.validate_pending.try_get_value().unwrap_or(true) {
            return;
        }
        self.validate_pending.try_set_value(true);
        let state = *self;
        request_animation_frame(move || {
            if state.validate_pending.try_set_value(false).is_none() {
                state.validate();
            }
        });
    }

    fn validate(&self) {
        if !self.alive() {
            return;
        }
        let options = self.options();
        let force = self.force_first.get_value();
        self.force_first.set_value(false);
        let auto = self.auto_highlight.get_untracked();
        let current = self
            .active
            .get_untracked()
            .filter(|id| options.iter().any(|el| &el.id() == id));

        let first_choice = || {
            let chosen = if self.prefer_selected && self.search.get_untracked().trim().is_empty() {
                options
                    .iter()
                    .find(|el| el.get_attribute("data-state").as_deref() == Some("checked"))
            } else {
                None
            };
            chosen.or(options.first()).map(|el| el.id())
        };

        let target = if force {
            if auto { first_choice() } else { None }
        } else if current.is_some() {
            current
        } else if auto {
            first_choice()
        } else {
            None
        };
        if target.is_some() || force {
            self.set_active(target, true);
        }
    }

    fn scroll_active_into_view(&self) {
        // Called again after a delay, when the combobox may be gone.
        let Some(Some(id)) = self.active.try_get_untracked() else {
            return;
        };
        let Some(list) = self.list_ref.get_untracked() else {
            return;
        };
        let Some(item) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&id))
        else {
            return;
        };
        let Ok(list) = list.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        // The first option scrolls to the very top so a group heading above it shows too.
        let is_first = self.options().first().is_some_and(|el| el.id() == id);
        if is_first {
            list.set_scroll_top(0);
            return;
        }
        let (lr, ir) = (
            list.get_bounding_client_rect(),
            item.get_bounding_client_rect(),
        );
        if ir.top() < lr.top() {
            list.set_scroll_top(list.scroll_top() - (lr.top() - ir.top()).ceil() as i32 - 4);
        } else if ir.bottom() > lr.bottom() {
            list.set_scroll_top(list.scroll_top() + (ir.bottom() - lr.bottom()).ceil() as i32 + 4);
        }
    }
}

#[derive(Clone, Copy)]
struct GroupState {
    count: RwSignal<usize>,
    heading_id: RwSignal<String>,
    has_heading: RwSignal<bool>,
}

fn present_untracked(id: RwSignal<String>) -> Option<String> {
    let id = id.get_untracked();
    (!id.is_empty()).then_some(id)
}

fn adjust(counter: RwSignal<usize>, up: bool) {
    counter.try_update(|c| *c = if up { *c + 1 } else { c.saturating_sub(1) });
}

/// The element that holds the options: `role="listbox"`, the target of the input's
/// `aria-controls`, and the element arrow keys search for options in.
#[component]
pub fn ListboxList(
    #[prop(into, optional)] multiselectable: MaybeProp<bool>,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let state = expect_context::<ListboxState>();
    let refs = use_composed_refs(vec![node_ref, state.list_ref]);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=refs
            attr:id=move || state.list_id_attr()
            attr:role="listbox"
            attr:aria-label=move || label.get()
            attr:aria-multiselectable=move || multiselectable.get().unwrap_or(false).then_some("true")
            attr:data-empty=move || (state.ready.get() && state.visible_count.get() == 0).then_some("")
            // A press on the scrollbar or a gap must not take focus from the input.
            on:mousedown=move |event: web_sys::MouseEvent| event.prevent_default()
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// One option. `value` is its identity; `label` (default: the value) is what it is matched
/// on; `keywords` are extra words it matches. It renders nothing while it does not match the
/// search. `selected` marks it as chosen (a Combobox's value); `command_style` makes
/// `aria-selected` follow the highlight instead, as cmdk does.
#[component]
pub fn ListboxItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] keywords: MaybeProp<Vec<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] selected: MaybeProp<bool>,
    /// Keep rendering while the search does not match (the consumer filters it).
    #[prop(into, optional)]
    force_match: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(optional)] command_style: bool,
    /// Class for the option. `attr:class` is lost on a component that renders nothing while
    /// hidden, so this is a prop.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let state = expect_context::<ListboxState>();
    let group = use_context::<GroupState>();
    let id = client_id("item");
    let value = StoredValue::new(value);

    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let selected = Signal::derive(move || selected.get().unwrap_or(false));
    let visible = Signal::derive(move || {
        force_match.get().unwrap_or(false)
            || value.with_value(|v| {
                let label = label.get().unwrap_or_else(|| v.clone());
                state.matches(v, &label, &keywords.get().unwrap_or_default())
            })
    });
    let active = Signal::derive(move || {
        present(id).is_some_and(|id| state.active.get().as_deref() == Some(&id))
    });

    // Count this item while it shows, so the list knows when it is empty and a group knows
    // when to hide. Each change also asks for the highlight to be re-checked.
    let counted = StoredValue::new(false);
    Effect::new(move |_| {
        let shown = visible.get();
        if shown != counted.get_value() {
            counted.set_value(shown);
            adjust(state.visible_count, shown);
            if let Some(group) = group {
                adjust(group.count, shown);
            }
            state.schedule_validate();
        }
    });
    on_cleanup(move || {
        if counted.try_get_value().unwrap_or(false) {
            adjust(state.visible_count, false);
            if let Some(group) = group {
                adjust(group.count, false);
            }
            state.schedule_validate();
        }
    });

    let select = move || {
        if !disabled.get_untracked()
            && let Some(cb) = on_select
        {
            cb.run(());
        }
    };
    let refs = use_composed_refs(vec![node_ref]);

    view! {
        <Show when=move || visible.get()>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:id=move || present(id)
                attr:class=move || class.get()
                attr:role="option"
                attr:aria-selected=move || {
                    if command_style { active.get() } else { selected.get() }.to_string()
                }
                attr:aria-disabled=move || disabled.get().then_some("true")
                attr:data-disabled=move || disabled.get().then_some("")
                attr:data-highlighted=move || active.get().then_some("")
                attr:data-state=move || if selected.get() { "checked" } else { "unchecked" }
                attr:data-value=value.get_value()
                // Keep focus in the input when an option is pressed.
                on:mousedown=move |event: web_sys::MouseEvent| event.prevent_default()
                on:pointermove=move |_: PointerEvent| {
                    if !disabled.get_untracked() {
                        state.set_active(present_untracked(id), false);
                    }
                }
                on:click=move |_| select()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

/// A set of options under one heading. It hides itself when none of its options match.
#[component]
pub fn ListboxGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let state = expect_context::<ListboxState>();
    let group = GroupState {
        count: RwSignal::new(0),
        heading_id: client_id("heading"),
        has_heading: RwSignal::new(false),
    };

    view! {
        <Provider value=group>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="group"
                attr:aria-labelledby=move || group.has_heading.get().then(|| present(group.heading_id)).flatten()
                attr:hidden=move || (state.ready.get() && group.count.get() == 0).then_some("")
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

/// The visible title of a [`ListboxGroup`]; the group is named by it.
#[component]
pub fn ListboxGroupHeading(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let group = use_context::<GroupState>();
    if let Some(group) = group {
        Effect::new(move |_| group.has_heading.set(true));
    }

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:id=move || group.and_then(|g| present(g.heading_id))
            // The group is named by it; screen readers should not also read it as an option.
            attr:aria-hidden="true"
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

/// A divider between groups. Hidden while a search is narrowing the list (a divider between
/// two sets of results that may not both show is noise), unless `always_render`.
#[component]
pub fn ListboxSeparator(
    #[prop(into, optional)] always_render: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let state = expect_context::<ListboxState>();
    let hidden = move || !always_render.get().unwrap_or(false) && state.is_filtering();
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="separator"
            attr:hidden=move || hidden().then_some("")
        >
            {""}
        </Primitive>
    }
}

/// Shown when no item matches.
#[component]
pub fn ListboxEmpty(
    /// Class for the message (`attr:class` is lost while it is not shown, so this is a prop).
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let state = expect_context::<ListboxState>();

    view! {
        <Show when=move || state.ready.get() && state.visible_count.get() == 0>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:class=move || class.get()
                attr:role="presentation"
                attr:aria-live="polite"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(label: &str, keywords: &[&str], search: &str) -> FilterQuery {
        FilterQuery {
            value: label.to_lowercase(),
            label: label.to_string(),
            keywords: keywords.iter().map(|k| k.to_string()).collect(),
            search: search.to_string(),
        }
    }

    #[test]
    fn contains_needs_every_word_in_any_order() {
        assert!(contains_filter(&query("Next.js Framework", &[], "")));
        assert!(contains_filter(&query(
            "Next.js Framework",
            &[],
            "fram next"
        )));
        assert!(contains_filter(&query("Next.js Framework", &[], "XT.J")));
        assert!(!contains_filter(&query(
            "Next.js Framework",
            &[],
            "next vue"
        )));
    }

    #[test]
    fn contains_also_searches_keywords() {
        assert!(contains_filter(&query(
            "Calendar",
            &["date", "schedule"],
            "sched"
        )));
        assert!(!contains_filter(&query("Calendar", &["date"], "sched")));
    }

    #[test]
    fn starts_with_ignores_the_middle_of_a_word() {
        assert!(starts_with_filter(&query("Calendar", &[], "cal")));
        assert!(!starts_with_filter(&query("Calendar", &[], "end")));
        assert!(starts_with_filter(&query("Calendar", &["agenda"], "age")));
    }

    #[test]
    fn fuzzy_matches_letters_in_order() {
        assert!(fuzzy_filter(&query("GitHub", &[], "gh")));
        assert!(fuzzy_filter(&query("GitHub", &[], "g h b")));
        assert!(!fuzzy_filter(&query("GitHub", &[], "hg")));
    }

    #[test]
    fn move_index_wraps_or_stops_at_the_ends() {
        assert_eq!(move_index(0, None, 1, true), None);
        assert_eq!(move_index(3, None, 1, true), Some(0));
        assert_eq!(move_index(3, None, -1, true), Some(2));
        assert_eq!(move_index(3, Some(2), 1, true), Some(0));
        assert_eq!(move_index(3, Some(0), -1, true), Some(2));
        assert_eq!(move_index(3, Some(2), 1, false), Some(2));
        assert_eq!(move_index(3, Some(0), -1, false), Some(0));
        assert_eq!(move_index(30, Some(5), 10, false), Some(15));
        assert_eq!(move_index(30, Some(25), 10, false), Some(29));
    }
}
