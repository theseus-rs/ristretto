//! Private, bundled catalog for the optional canonical collector.
//! Numeric trace states alone grant no ownership or payload access.
use crate::canonical_pool::{Compare, Contended, Link, Owned as Pooled, Pool};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize, Ordering};
mod state;
pub use state::*;
pub mod bounded;
mod trace;
mod vacancy;
pub use trace::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Domain {
    pub mode: DomainMode,
    pub epoch: u64,
    pub floor: u8,
    pub pending: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase {
    /// Per-domain root scan cursor; only a canonical scan operation advances it.
    pub root_cursors: [usize; 2],
    /// Per-domain cursor for the current conditional-reference stage.
    pub reference_cursors: [usize; 2],
    pub revision: u64,
    pub domains: [Domain; 2],
}

#[cfg(test)]
mod trace_tests;

struct PooledLink<T>(Link<T>);
impl<T> PooledLink<T> {
    fn new(pool: Arc<Pool<T>>, value: Pooled<T>) -> Self {
        let link = Link::empty(pool);
        assert!(matches!(link.compare(None, Some(value)), Compare::Changed));
        Self(link)
    }
    fn load(&self) -> Option<Pooled<T>> {
        #[cfg(test)]
        if fail_load() {
            return None;
        }
        self.0.try_load().ok().flatten()
    }
    fn points_to(&self, value: &Pooled<T>) -> bool {
        self.0.points_to(value)
    }
    #[expect(
        clippy::panic,
        reason = "private catalog links only accept owners from their reserved pool"
    )]
    fn compare(&self, expected: &Pooled<T>, proposed: Pooled<T>) -> bool {
        match self.0.compare(Some(expected), Some(proposed)) {
            Compare::Changed => true,
            Compare::Unchanged(_) => false,
            Compare::Foreign(_) => panic!("foreign owning link"),
        }
    }
}
struct OptionalLink<T>(Link<T>);
impl<T> OptionalLink<T> {
    fn empty(pool: Arc<Pool<T>>) -> Self {
        Self(Link::empty(pool))
    }
    fn is_occupied(&self) -> bool {
        !self.0.is_empty()
    }
    fn load(&self) -> Result<Option<Pooled<T>>, Contended> {
        #[cfg(test)]
        if fail_load() {
            return Err(Contended);
        }
        self.0.try_load()
    }
    #[expect(
        clippy::panic,
        reason = "private catalog links only accept owners from their reserved pool"
    )]
    fn compare(&self, expected: Option<&Pooled<T>>, proposed: Option<Pooled<T>>) -> bool {
        match self.0.compare(expected, proposed) {
            Compare::Changed => true,
            Compare::Unchanged(_) => false,
            Compare::Foreign(_) => panic!("foreign optional link"),
        }
    }
}
fn share<T>(value: &Pooled<T>) -> Option<Pooled<T>> {
    #[cfg(test)]
    if FAIL_SHARE.with(|remaining| match remaining.get() {
        Some(0) => {
            remaining.set(None);
            true
        }
        Some(count) => {
            remaining.set(Some(count - 1));
            false
        }
        None => false,
    }) {
        return None;
    }
    value.try_clone()
}

#[cfg(test)]
thread_local! {
    static FAIL_SHARE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
    static FAIL_LOAD: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn fail_load() -> bool {
    FAIL_LOAD.with(|remaining| match remaining.get() {
        Some(0) => {
            remaining.set(None);
            true
        }
        Some(count) => {
            remaining.set(Some(count - 1));
            false
        }
        None => false,
    })
}
fn allocate<T>(pool: &Arc<Pool<T>>, mut value: T) -> Option<Pooled<T>> {
    for _ in 0..8 {
        match pool.try_allocate(value) {
            Ok(owned) => return Some(owned),
            Err(returned) => value = returned,
        }
    }
    None
}

#[cfg(test)]
mod link_tests;

#[cfg(test)]
#[path = "canonical_catalog/record_fixture.rs"]
mod record_fixture;
