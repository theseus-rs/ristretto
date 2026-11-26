//! Isolated bounded-effect experiment. No collector admission is replaced here.
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
mod state;
pub use state::*;

struct Link<T>(arc_swap::ArcSwap<T>);
impl<T> Link<T> {
    fn new(value: Arc<T>) -> Self {
        Self(arc_swap::ArcSwap::new(value))
    }
    fn load(&self) -> Arc<T> {
        self.0.load_full()
    }
    fn compare(&self, expected: &Arc<T>, proposed: Arc<T>) -> Arc<T> {
        arc_swap::Guard::into_inner(self.0.compare_and_swap(expected, proposed))
    }
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
