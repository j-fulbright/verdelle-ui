//! A pretend server for the gallery: the page's `ThemeProvider` keeps its values in a
//! `SyncedStorage` whose write-through hooks log every call and also write them to the
//! browser's `localStorage` (so a reload still finds them, and the head's no-flash script,
//! which reads `localStorage`, still works). The Themes page shows the log and lets you
//! simulate values arriving from the "server".

use leptos::prelude::*;
use verdelle_ui::SyncedStorage;
use verdelle_ui::custom_theme::{LocalStorage, ThemeStorage, keys};

/// The gallery's pretend server, found with `use_context::<ServerDemo>()`.
#[derive(Clone)]
pub struct ServerDemo {
    /// The storage the page's `ThemeProvider` uses.
    pub storage: SyncedStorage,
    /// Every write-through call so far, newest last.
    pub log: RwSignal<Vec<String>>,
}

const KEYS: [&str; 4] = [keys::THEME, keys::MODE, keys::CUSTOM, keys::PREVIEW];

/// Shortens a stored value for the log.
pub fn short(value: &str) -> String {
    let one_line: String = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() > 60 {
        format!("{}...", one_line.chars().take(60).collect::<String>())
    } else {
        one_line
    }
}

impl ServerDemo {
    pub fn new() -> Self {
        let log = RwSignal::new(Vec::<String>::new());
        let push = move |line: String| {
            log.update(|l| {
                l.push(line);
                if l.len() > 40 {
                    l.remove(0);
                }
            })
        };
        let storage = SyncedStorage::new([])
            .on_set(move |key, value| {
                LocalStorage.set(key, value);
                push(format!("on_set({key}, {})", short(value)));
            })
            .on_remove(move |key| {
                LocalStorage.remove(key);
                push(format!("on_remove({key})"));
            });
        ServerDemo { storage, log }
    }

    /// What the pretend server "has stored": the browser's `localStorage`. Called after
    /// hydration, like a fetch on login.
    pub fn load_stored(&self) {
        let values: Vec<(String, String)> = KEYS
            .iter()
            .filter_map(|k| LocalStorage.get(k).map(|v| (k.to_string(), v)))
            .collect();
        let n = values.len();
        self.storage.load(values);
        self.log
            .update(|l| l.push(format!("load({n} values) from the pretend server")));
    }
}

impl Default for ServerDemo {
    fn default() -> Self {
        Self::new()
    }
}
