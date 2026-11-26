//! Private, bundled catalog for the optional canonical collector.
//! Numeric trace states alone grant no ownership or payload access.
use crate::canonical_pool::{Compare, Contended, Link, Owned, Pool};
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
    pub root_cursors: [usize; DOMAIN_COUNT],
    /// Per-domain cursor for the current conditional-reference stage.
    pub reference_cursors: [usize; DOMAIN_COUNT],
    pub revision: u64,
    pub domains: [Domain; DOMAIN_COUNT],
}

#[cfg(test)]
mod trace_tests;

struct PooledLink<T>(Link<T>);
impl<T> PooledLink<T> {
    fn new(pool: Arc<Pool<T>>, value: Owned<T>) -> Self {
        let link = Link::empty(pool);
        assert!(matches!(link.compare(None, Some(value)), Compare::Changed));
        Self(link)
    }

    fn load(&self) -> Option<Owned<T>> {
        #[cfg(test)]
        if fail_load() {
            return None;
        }
        self.0.try_load().ok().flatten()
    }

    fn points_to(&self, value: &Owned<T>) -> bool {
        self.0.points_to(value)
    }

    #[expect(
        clippy::panic,
        reason = "private catalog links only accept owners from their reserved pool"
    )]
    fn compare(&self, expected: &Owned<T>, proposed: Owned<T>) -> bool {
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

    fn load(&self) -> Result<Option<Owned<T>>, Contended> {
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
    fn compare(&self, expected: Option<&Owned<T>>, proposed: Option<Owned<T>>) -> bool {
        match self.0.compare(expected, proposed) {
            Compare::Changed => true,
            Compare::Unchanged(_) => false,
            Compare::Foreign(_) => panic!("foreign optional link"),
        }
    }
}

fn share<T>(value: &Owned<T>) -> Option<Owned<T>> {
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
const ALLOCATION_ATTEMPTS: usize = 8;

fn allocate<T>(pool: &Arc<Pool<T>>, mut value: T) -> Option<Owned<T>> {
    for _ in 0..ALLOCATION_ATTEMPTS {
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

#[path = "canonical_catalog/reference.rs"]
pub mod reference;

/// Optional allocator-order diagnostics, excluded from timing builds.
#[cfg(feature = "experimental-canonical-audit")]
#[path = "canonical_catalog/audit.rs"]
pub mod audit;
