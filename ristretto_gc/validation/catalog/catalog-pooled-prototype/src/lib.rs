//! Isolated bounded-effect experiment. No collector admission is replaced here.
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
mod state;
pub use state::*;

use pool::{Owned as Pooled, Pool};
struct PooledLink<T>(arc_swap::ArcSwapAny<Pooled<T>>);
impl<T> PooledLink<T> {
    fn new(value: Pooled<T>) -> Self {
        Self(arc_swap::ArcSwapAny::new(value))
    }
    fn load(&self) -> Pooled<T> {
        self.0.load_full()
    }
    fn compare(&self, expected: &Pooled<T>, proposed: Pooled<T>) -> Pooled<T> {
        arc_swap::Guard::into_inner(self.0.compare_and_swap(expected, proposed))
    }
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
mod tests;

struct OptionalLink<T>(arc_swap::ArcSwapOption<T>);
impl<T> OptionalLink<T> {
    fn empty() -> Self {
        Self(arc_swap::ArcSwapOption::empty())
    }
    fn load(&self) -> Option<Arc<T>> {
        self.0.load_full()
    }
    fn compare(&self, expected: Option<&Arc<T>>, proposed: Option<Arc<T>>) -> bool {
        let previous = self
            .0
            .compare_and_swap(expected.map_or(std::ptr::null(), Arc::as_ptr), proposed);
        match (previous.as_ref(), expected) {
            (None, None) => true,
            (Some(previous), Some(expected)) => Arc::ptr_eq(previous, expected),
            _ => false,
        }
    }
}
