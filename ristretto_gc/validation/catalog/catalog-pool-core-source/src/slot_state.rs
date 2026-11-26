//! Reuse stays excluded between the final decrement and completed destruction.
use super::{AtomicBool, AtomicUsize, Ordering, fence};

pub struct SlotState {
    occupied: AtomicBool,
    references: AtomicUsize,
}
impl SlotState {
    pub fn new() -> Self {
        Self {
            occupied: AtomicBool::new(false),
            references: AtomicUsize::new(0),
        }
    }
    pub fn claim(&self) -> bool {
        if self
            .occupied
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return false;
        }
        self.references.store(1, Ordering::Relaxed);
        true
    }
    /// Caller already owns a strong count and must abort on excessive count.
    pub fn share(&self) -> usize {
        self.references.fetch_add(1, Ordering::Relaxed)
    }
    /// True gives the caller destruction ownership, but does not allow reuse.
    pub fn release(&self) -> bool {
        if self.references.fetch_sub(1, Ordering::Release) != 1 {
            return false;
        }
        fence(Ordering::Acquire);
        true
    }
    /// Only the final releaser calls this, after the value's drop glue completes.
    pub fn publish_free(&self) {
        debug_assert_eq!(self.references.load(Ordering::Relaxed), 0);
        self.occupied.store(false, Ordering::Release);
    }
    pub fn is_occupied(&self) -> bool {
        self.occupied.load(Ordering::Acquire)
    }
}
