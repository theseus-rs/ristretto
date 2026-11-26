//! Isolated phase/catalog design experiment; not selected by the collector.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

mod tree;
mod state;
pub use state::*;

struct Link(arc_swap::ArcSwap<state::Head>);
impl Link {
    fn new(head: Arc<state::Head>) -> Self { Self(arc_swap::ArcSwap::new(head)) }
    fn load(&self) -> Arc<state::Head> { self.0.load_full() }
    fn compare(&self, expected: &Arc<state::Head>, proposed: Arc<state::Head>) -> bool {
        Arc::ptr_eq(&self.0.compare_and_swap(expected, proposed), expected)
    }
}

#[cfg(test)]
mod tests;
