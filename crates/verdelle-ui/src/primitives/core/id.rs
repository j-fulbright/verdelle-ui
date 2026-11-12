//! Ids for ARIA wiring (`aria-controls`, `aria-labelledby`, ...).
//!
//! The server and the browser must hand out the same ids, or server-rendered
//! attributes and client-side signals disagree after hydration. Upstream used one
//! process-wide counter, which on the server keeps climbing across requests, so the
//! two sides could never agree. Here the counter lives in a reactive context that
//! the app provides once at its root ([`provide_id_generator`]): a server render
//! gets a fresh counter per request, and the browser starts from zero when it
//! hydrates, so both count the same components in the same order.
//!
//! The ids agree as long as components that exist on both sides are created in the
//! same order, which holds for ordinary top-to-bottom rendering. Content created
//! only on the client (after a client-side load, for example) keeps counting from
//! where hydration stopped, so its ids are still unique. Pass an explicit id wherever
//! the order might differ.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use leptos::prelude::*;

/// Used only when no [`IdGenerator`] was provided (tests, or an app that forgot to).
static FALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// A counter shared by every component under the place it was provided.
#[derive(Clone, Default)]
pub struct IdGenerator(Arc<AtomicUsize>);

impl IdGenerator {
    fn next(&self) -> usize {
        self.0.fetch_add(1, Ordering::Relaxed)
    }
}

/// Provide a fresh id counter to everything below the current reactive owner.
/// Call it once, near the root of the app, so each server request gets its own.
pub fn provide_id_generator() {
    provide_context(IdGenerator::default());
}

pub fn use_id(deterministic_id: Option<String>) -> ReadSignal<String> {
    let id = deterministic_id.unwrap_or_else(|| {
        let n = match use_context::<IdGenerator>() {
            Some(generator) => generator.next(),
            None => FALLBACK_COUNT.fetch_add(1, Ordering::Relaxed),
        };
        format!("vdl-{n}")
    });
    let (id, _) = signal(id);
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_provided_generator_counts_from_zero_each_time() {
        let owner = Owner::new();
        owner.with(|| {
            provide_id_generator();
            assert_eq!(use_id(None).get_untracked(), "vdl-0");
            assert_eq!(use_id(None).get_untracked(), "vdl-1");
        });
        // A second owner models a second server request: it starts over.
        let owner = Owner::new();
        owner.with(|| {
            provide_id_generator();
            assert_eq!(use_id(None).get_untracked(), "vdl-0");
        });
    }

    #[test]
    fn an_explicit_id_is_used_as_given_and_does_not_consume_the_counter() {
        let owner = Owner::new();
        owner.with(|| {
            provide_id_generator();
            assert_eq!(use_id(Some("custom".into())).get_untracked(), "custom");
            assert_eq!(use_id(None).get_untracked(), "vdl-0");
        });
    }
}
