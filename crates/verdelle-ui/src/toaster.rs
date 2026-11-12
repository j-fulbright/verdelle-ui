use crate::primitives::toast::{ToastController, use_toast_controller};
use crate::{
    Toast, ToastAction, ToastDescription, ToastPosition, ToastProvider, ToastTitle, ToastVariant,
    ToastViewport,
};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use std::time::Duration;

/// How long a closed toast stays in the list so its exit animation can play.
const EXIT_GRACE: Duration = Duration::from_millis(400);

/// A button shown on a toast made with [`ToastHandle::show`].
#[derive(Clone)]
pub struct ToastActionData {
    /// The visible label.
    pub label: String,
    /// The accessible name; say what the action does.
    pub alt_text: String,
    pub on_click: Callback<()>,
}

/// What to show for a toast made with [`ToastHandle::show`]. Build it with
/// [`ToastData::new`] and the chained setters.
#[derive(Clone)]
pub struct ToastData {
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    /// Milliseconds before it closes itself. `None` uses the [`Toaster`]'s default; 0 never closes.
    pub duration: Option<i32>,
    pub action: Option<ToastActionData>,
    /// Runs once when the toast closes, whichever way it was closed.
    pub on_close: Option<Callback<()>>,
}

impl ToastData {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            duration: None,
            action: None,
            on_close: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn duration(mut self, ms: i32) -> Self {
        self.duration = Some(ms);
        self
    }

    pub fn action(
        mut self,
        label: impl Into<String>,
        alt_text: impl Into<String>,
        on_click: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        self.action = Some(ToastActionData {
            label: label.into(),
            alt_text: alt_text.into(),
            on_click: Callback::new(move |()| on_click()),
        });
        self
    }

    pub fn on_close(mut self, on_close: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_close = Some(Callback::new(move |()| on_close()));
        self
    }
}

#[derive(Clone)]
struct ToastItem {
    id: u32,
    data: ToastData,
}

/// Shows toasts from anywhere under a [`Toaster`]. Get one with [`use_toaster`]; it is `Copy`.
#[derive(Clone, Copy)]
pub struct ToastHandle {
    items: RwSignal<Vec<ToastItem>>,
    next_id: RwSignal<u32>,
    controller: Option<ToastController>,
}

impl ToastHandle {
    /// Shows a toast.
    pub fn show(&self, data: ToastData) {
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        self.items
            .update(|items| items.push(ToastItem { id, data }));
    }

    /// Shows a toast with only a title.
    pub fn message(&self, title: impl Into<String>) {
        self.show(ToastData::new(title));
    }

    /// Closes every toast; each one's `on_close` runs.
    pub fn dismiss_all(&self) {
        if let Some(controller) = self.controller {
            controller.dismiss_all();
        }
    }
}

/// The [`ToastHandle`] of the nearest [`Toaster`].
///
/// Panics when there is no [`Toaster`] above the caller, as that is a wiring mistake.
pub fn use_toaster() -> ToastHandle {
    use_context::<ToastHandle>().expect("use_toaster() needs a <Toaster> above the component")
}

/// Wrap the app (or the part of it that shows toasts) in this, then call [`use_toaster`] in
/// any component below to show toasts without managing a list yourself:
///
/// ```ignore
/// let toaster = use_toaster();
/// toaster.show(ToastData::new("Saved").description("Your changes are live.").variant(ToastVariant::Success));
/// ```
///
/// It renders the provider, the viewport and a [`crate::Toast`] per message. For full control
/// of the markup, use [`crate::ToastProvider`], [`crate::ToastViewport`] and [`crate::Toast`]
/// yourself. `position` is where toasts appear (bottom right by default; it can change at
/// any time) and `duration` the default lifetime in milliseconds (5000).
#[component]
pub fn Toaster(
    #[prop(optional, into)] position: Signal<ToastPosition>,
    #[prop(into, optional)] duration: MaybeProp<i32>,
    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <ToastProvider duration=duration position=position>
            <ToasterInner>{children.with_value(|children| children())}</ToasterInner>
        </ToastProvider>
    }
}

/// Sits inside the provider so it can reach the controller, publishes the handle and renders
/// the list.
#[component]
fn ToasterInner(children: Children) -> impl IntoView {
    let items = RwSignal::new(Vec::<ToastItem>::new());
    let handle = ToastHandle {
        items,
        next_id: RwSignal::new(1),
        controller: use_toast_controller(),
    };
    provide_context(handle);

    view! {
        {children()}
        <ToastViewport>
            <For each=move || items.get() key=|item| item.id let:item>
                {
                    let id = item.id;
                    let data = item.data;
                    let on_close = data.on_close;
                    let title = StoredValue::new(data.title.clone());
                    let description = StoredValue::new(data.description.clone());
                    let action = StoredValue::new(data.action.clone());
                    view! {
                        <Toast
                            variant=data.variant
                            duration=data.duration
                            on_open_change=Callback::new(move |open: bool| {
                                if open {
                                    return;
                                }
                                if let Some(on_close) = on_close {
                                    on_close.run(());
                                }
                                // Drop it from the list once its exit animation has played.
                                set_timeout(
                                    move || items.update(|items| items.retain(|i| i.id != id)),
                                    EXIT_GRACE,
                                );
                            })
                        >
                            <ToastTitle>{move || title.get_value()}</ToastTitle>
                            {move || description.get_value().map(|d| view! { <ToastDescription>{d.clone()}</ToastDescription> })}
                            {move || action.get_value().map(|a| {
                                let label = a.label.clone();
                                view! {
                                    <ToastAction alt_text=a.alt_text.clone() on_click=a.on_click>
                                        {label.clone()}
                                    </ToastAction>
                                }
                            })}
                        </Toast>
                    }
                }
            </For>
        </ToastViewport>
    }
}
