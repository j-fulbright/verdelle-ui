//! [`SyncedStorage`]: a [`ThemeStorage`] for apps that keep a person's theme on a server.
//!
//! It holds the values in memory, starting from what the app already has (loaded on login, or
//! passed in while rendering on the server), and calls back into the app on every change so
//! the app can write it to its database. Values that arrive later replace the contents with
//! [`SyncedStorage::load`], and the [`crate::ThemeProvider`] picks them up.

use crate::custom_theme::ThemeStorage;
use leptos::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// A change the app has not been told about yet (see [`PendingWrites`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Write {
    /// `key` now has this value.
    Set(String, String),
    /// `key` was removed.
    Remove(String),
}

/// Writes waiting for a debounce timer. Several changes to one key become the last one, in the
/// order the keys were first touched. Pure: the timer lives in [`SyncedStorage`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PendingWrites {
    order: Vec<String>,
    latest: BTreeMap<String, Option<String>>,
}

impl PendingWrites {
    /// Notes that `key` was set.
    pub fn set(&mut self, key: &str, value: &str) {
        self.note(key, Some(value.to_string()));
    }

    /// Notes that `key` was removed.
    pub fn remove(&mut self, key: &str) {
        self.note(key, None);
    }

    fn note(&mut self, key: &str, value: Option<String>) {
        if !self.latest.contains_key(key) {
            self.order.push(key.to_string());
        }
        self.latest.insert(key.to_string(), value);
    }

    /// Whether nothing is waiting.
    pub fn is_empty(&self) -> bool {
        self.latest.is_empty()
    }

    /// Takes everything that is waiting, one [`Write`] per key.
    pub fn take(&mut self) -> Vec<Write> {
        let mut latest = std::mem::take(&mut self.latest);
        std::mem::take(&mut self.order)
            .into_iter()
            .filter_map(|key| {
                latest.remove(&key).map(|v| match v {
                    Some(v) => Write::Set(key, v),
                    None => Write::Remove(key),
                })
            })
            .collect()
    }

    /// Lays what is waiting over `map`, so values the person changed after a server answer was
    /// requested are not lost when the answer arrives.
    pub fn overlay(&self, map: &mut BTreeMap<String, String>) {
        for (key, value) in &self.latest {
            match value {
                Some(v) => {
                    map.insert(key.clone(), v.clone());
                }
                None => {
                    map.remove(key);
                }
            }
        }
    }
}

type SetHook = Arc<dyn Fn(&str, &str) + Send + Sync>;
type RemoveHook = Arc<dyn Fn(&str) + Send + Sync>;

#[derive(Default)]
struct Hooks {
    on_set: Option<SetHook>,
    on_remove: Option<RemoveHook>,
    debounce_ms: u32,
}

struct Shared {
    map: Mutex<BTreeMap<String, String>>,
    pending: Mutex<PendingWrites>,
    armed: Mutex<bool>,
    hooks: Mutex<Hooks>,
    version: ArcRwSignal<u64>,
}

/// A [`ThemeStorage`] that keeps the values in memory and tells the app about every change, so
/// the app can persist a person's theme on its server instead of (or as well as) their
/// browser. Not part of shadcn/ui.
///
/// ```ignore
/// let storage = SyncedStorage::new(values_from_the_server)       // key -> value strings
///     .on_set(|key, value| save_theme_setting(key, value))       // write through
///     .on_remove(|key| delete_theme_setting(key))
///     .debounce_ms(500);                                         // optional: batch quick changes
/// view! { <ThemeProvider storage=storage.clone().into_arc()> ... </ThemeProvider> }
///
/// // Values that arrive after hydration (a fetch on login):
/// storage.load(values);
/// ```
///
/// - **Write through**: `on_set(key, value)` and `on_remove(key)` run when the provider (or
///   anything else) changes a stored value. Setting a value it already has does nothing, so
///   what you [`load`](Self::load) is not echoed back. The keys are the ones in
///   [`crate::custom_theme::keys`]; values are short strings (JSON for the lists).
/// - **Debounce**: with `debounce_ms(n)` changes wait `n` ms (in the browser) and each key is
///   written once, with its last value. [`flush`](Self::flush) writes at once.
/// - **Arriving values**: [`load`](Self::load) replaces the contents without calling the hooks
///   and tells the provider, which then updates the theme, mode and saved themes. Changes the
///   person made that are still waiting for the debounce timer are kept on top.
/// - **First paint**: it implements [`ThemeStorage::seed`], so a provider built from values you
///   have while rendering on the server starts with them (and prints a no-flash script that
///   uses them). With the script in `<head>`, use [`crate::theme_init_script_seeded`] with
///   [`snapshot`](Self::snapshot).
///
/// It is cheap to clone; clones share the contents. Hooks may be called from the browser only
/// (the provider never writes during server rendering).
#[derive(Clone)]
pub struct SyncedStorage {
    shared: Arc<Shared>,
}

impl std::fmt::Debug for SyncedStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncedStorage")
            .field("values", &self.snapshot().len())
            .finish()
    }
}

impl SyncedStorage {
    /// Starts with `initial` values (storage key to string), for instance a person's saved
    /// theme settings read from the database. Pass nothing, `[]`, when none are known yet and
    /// [`load`](Self::load) them later.
    pub fn new(initial: impl IntoIterator<Item = (String, String)>) -> Self {
        SyncedStorage {
            shared: Arc::new(Shared {
                map: Mutex::new(initial.into_iter().collect()),
                pending: Mutex::new(PendingWrites::default()),
                armed: Mutex::new(false),
                hooks: Mutex::new(Hooks::default()),
                version: ArcRwSignal::new(0),
            }),
        }
    }

    /// Called with `(key, value)` when a value is set (after the debounce, if any).
    pub fn on_set(self, hook: impl Fn(&str, &str) + Send + Sync + 'static) -> Self {
        if let Ok(mut h) = self.shared.hooks.lock() {
            h.on_set = Some(Arc::new(hook));
        }
        self
    }

    /// Called with `key` when a value is removed (after the debounce, if any).
    pub fn on_remove(self, hook: impl Fn(&str) + Send + Sync + 'static) -> Self {
        if let Ok(mut h) = self.shared.hooks.lock() {
            h.on_remove = Some(Arc::new(hook));
        }
        self
    }

    /// Waits `ms` milliseconds after a change before calling the hooks, and calls them once
    /// per key. `0` (the default) calls them at once. The wait applies in the browser; use
    /// [`flush`](Self::flush) before leaving the page if a change must not be lost.
    pub fn debounce_ms(self, ms: u32) -> Self {
        if let Ok(mut h) = self.shared.hooks.lock() {
            h.debounce_ms = ms;
        }
        self
    }

    /// This storage as the `Arc` that [`crate::ThemeProvider`]'s `storage` prop takes.
    pub fn into_arc(self) -> Arc<dyn ThemeStorage> {
        Arc::new(self)
    }

    /// Replaces all values (they came from the server, so the hooks are not called) and
    /// tells the provider. Reactive: call it from an effect or a resource's callback, in the
    /// browser. Changes still waiting for the debounce timer stay on top of the new values.
    pub fn load(&self, values: impl IntoIterator<Item = (String, String)>) {
        let mut next: BTreeMap<String, String> = values.into_iter().collect();
        if let Ok(pending) = self.shared.pending.lock() {
            pending.overlay(&mut next);
        }
        if let Ok(mut map) = self.shared.map.lock() {
            *map = next;
        }
        self.shared.version.update(|v| *v += 1);
    }

    /// A copy of all the values: what to give the server's no-flash script
    /// ([`crate::theme_init_script_seeded`]) or to save as a whole.
    pub fn snapshot(&self) -> BTreeMap<String, String> {
        self.shared
            .map
            .lock()
            .map(|m| m.clone())
            .unwrap_or_default()
    }

    /// Calls the hooks now for everything the debounce timer is holding.
    pub fn flush(&self) {
        let writes = match self.shared.pending.lock() {
            Ok(mut p) => p.take(),
            Err(_) => return,
        };
        if writes.is_empty() {
            return;
        }
        let (on_set, on_remove) = match self.shared.hooks.lock() {
            Ok(h) => (h.on_set.clone(), h.on_remove.clone()),
            Err(_) => return,
        };
        for w in writes {
            match w {
                Write::Set(k, v) => {
                    if let Some(f) = &on_set {
                        f(&k, &v);
                    }
                }
                Write::Remove(k) => {
                    if let Some(f) = &on_remove {
                        f(&k);
                    }
                }
            }
        }
    }

    fn changed(&self, write: Write) {
        let debounce = self.shared.hooks.lock().map(|h| h.debounce_ms).unwrap_or(0);
        if let Ok(mut p) = self.shared.pending.lock() {
            match &write {
                Write::Set(k, v) => p.set(k, v),
                Write::Remove(k) => p.remove(k),
            }
        }
        if debounce == 0 {
            self.flush();
        } else {
            self.arm(debounce);
        }
    }

    #[cfg(feature = "hydrate")]
    fn arm(&self, ms: u32) {
        let already = match self.shared.armed.lock() {
            Ok(mut a) => std::mem::replace(&mut *a, true),
            Err(_) => return,
        };
        if already {
            return;
        }
        let this = self.clone();
        set_timeout(
            move || {
                if let Ok(mut a) = this.shared.armed.lock() {
                    *a = false;
                }
                this.flush();
            },
            std::time::Duration::from_millis(ms as u64),
        );
    }

    #[cfg(not(feature = "hydrate"))]
    fn arm(&self, _ms: u32) {
        // No timers outside the browser, and nothing is written there anyway.
        let _ = &self.shared.armed;
        self.flush();
    }
}

impl ThemeStorage for SyncedStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.shared.map.lock().ok()?.get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) {
        let same = match self.shared.map.lock() {
            Ok(mut m) => {
                if m.get(key).map(String::as_str) == Some(value) {
                    true
                } else {
                    m.insert(key.to_string(), value.to_string());
                    false
                }
            }
            Err(_) => return,
        };
        if !same {
            self.changed(Write::Set(key.to_string(), value.to_string()));
        }
    }

    fn remove(&self, key: &str) {
        let had = self
            .shared
            .map
            .lock()
            .map(|mut m| m.remove(key).is_some())
            .unwrap_or(false);
        if had {
            self.changed(Write::Remove(key.to_string()));
        }
    }

    fn changes(&self) -> Option<ArcReadSignal<u64>> {
        Some(self.shared.version.read_only())
    }

    fn seed(&self) -> Option<BTreeMap<String, String>> {
        Some(self.snapshot())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Arc<Mutex<Vec<String>>>;

    fn logged(initial: &[(&str, &str)]) -> (SyncedStorage, Log) {
        let log: Log = Arc::default();
        let (a, b) = (log.clone(), log.clone());
        let s = SyncedStorage::new(initial.iter().map(|(k, v)| (k.to_string(), v.to_string())))
            .on_set(move |k, v| a.lock().unwrap().push(format!("set {k}={v}")))
            .on_remove(move |k| b.lock().unwrap().push(format!("remove {k}")));
        (s, log)
    }

    fn lines(log: &Log) -> Vec<String> {
        log.lock().unwrap().clone()
    }

    #[test]
    fn starts_from_the_initial_values_without_calling_the_hooks() {
        let (s, log) = logged(&[("a", "1")]);
        assert_eq!(s.get("a").as_deref(), Some("1"));
        assert_eq!(s.get("b"), None);
        assert!(lines(&log).is_empty());
        assert_eq!(s.seed().unwrap().len(), 1);
    }

    #[test]
    fn writes_through_and_skips_unchanged_values() {
        let (s, log) = logged(&[("a", "1")]);
        s.set("a", "1");
        s.set("a", "2");
        s.set("b", "3");
        s.remove("a");
        s.remove("missing");
        assert_eq!(lines(&log), ["set a=2", "set b=3", "remove a"]);
        assert_eq!(s.get("a"), None);
        assert_eq!(s.get("b").as_deref(), Some("3"));
    }

    #[test]
    fn load_replaces_the_contents_silently_and_bumps_the_counter() {
        let (s, log) = logged(&[("a", "1"), ("gone", "x")]);
        let before = s.changes().unwrap().get_untracked();
        s.load([("a".to_string(), "9".to_string())]);
        assert_eq!(s.get("a").as_deref(), Some("9"));
        assert_eq!(s.get("gone"), None);
        assert_eq!(s.changes().unwrap().get_untracked(), before + 1);
        assert!(lines(&log).is_empty());
        // The provider writing back what it just read is not an echo.
        s.set("a", "9");
        assert!(lines(&log).is_empty());
    }

    #[test]
    fn clones_share_their_contents() {
        let (s, _) = logged(&[]);
        let t = s.clone();
        t.set("k", "v");
        assert_eq!(s.get("k").as_deref(), Some("v"));
    }

    #[test]
    fn pending_writes_keep_the_last_value_of_each_key_in_order() {
        let mut p = PendingWrites::default();
        assert!(p.is_empty());
        p.set("b", "1");
        p.set("a", "1");
        p.set("b", "2");
        p.remove("a");
        p.set("c", "3");
        p.remove("c");
        p.set("c", "4");
        assert_eq!(
            p.take(),
            [
                Write::Set("b".into(), "2".into()),
                Write::Remove("a".into()),
                Write::Set("c".into(), "4".into()),
            ]
        );
        assert!(p.is_empty());
        assert!(p.take().is_empty());
    }

    #[test]
    fn pending_writes_stay_on_top_of_loaded_values() {
        let mut p = PendingWrites::default();
        p.set("mode", "dark");
        p.remove("theme");
        let mut incoming: BTreeMap<String, String> = [("mode", "light"), ("theme", "rose")]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        p.overlay(&mut incoming);
        assert_eq!(incoming.get("mode").map(String::as_str), Some("dark"));
        assert!(!incoming.contains_key("theme"));
    }

    #[test]
    fn debounce_holds_changes_until_flushed() {
        // Outside the browser the timer does not exist and the write is immediate, so the
        // held state is driven through the pending list the way the timer does.
        let (s, log) = logged(&[]);
        s.shared.pending.lock().unwrap().set("a", "1");
        s.shared.pending.lock().unwrap().set("a", "2");
        s.shared.pending.lock().unwrap().remove("b");
        assert!(lines(&log).is_empty());
        s.flush();
        assert_eq!(lines(&log), ["set a=2", "remove b"]);
        s.flush();
        assert_eq!(lines(&log).len(), 2);
    }
}
