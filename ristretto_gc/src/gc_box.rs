use bitflags::bitflags;
use std::sync::atomic::{AtomicU8, Ordering as AtomicOrdering};

use crate::GarbageCollector;
use crate::collector::Trace;
use crate::pointers::SafePtr;

bitflags! {
    /// Represents the state of a garbage-collected object in a tri-color marking scheme.
    ///
    /// - `White`: The object is potentially garbage, not reachable.
    /// - `Gray`: The object is reachable but not yet processed.
    /// - `Black`: The object is reachable and has been processed.
    /// - `Marked`: The object has been marked for collection.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct GcState: u8 {
        const WHITE  = 0b0000_0001;
        const GRAY   = 0b0000_0010;
        const BLACK  = 0b0000_0100;
        const MARKED = 0b0000_1000;
    }
}

/// Internal data structure for garbage-collected objects
pub(crate) struct GcBox<T> {
    pub(crate) state: AtomicU8,
    pub(crate) data: T,
}

impl<T> GcBox<T> {
    pub(crate) fn new(data: T) -> Self {
        Self {
            state: AtomicU8::new(GcState::WHITE.bits()),
            data,
        }
    }

    /// Gets the current color of this object for tri-color marking
    ///
    /// Note: This is only safe to call from the GC thread or when holding appropriate locks
    pub(crate) fn get_color(&self) -> GcState {
        GcState::from_bits_truncate(self.state.load(AtomicOrdering::Acquire))
    }

    /// Sets the color of this object for tri-color marking
    ///
    /// Note: This is only safe to call from the GC thread or when holding appropriate locks
    pub(crate) fn set_color(&self, color: GcState) {
        self.state
            .fetch_update(AtomicOrdering::AcqRel, AtomicOrdering::Acquire, |old| {
                let mut flags = GcState::from_bits_truncate(old);
                flags.remove(GcState::WHITE | GcState::GRAY | GcState::BLACK);
                flags |= color;
                Some(flags.bits())
            })
            .ok();
    }

    /// Marks this object if not already marked.
    /// Returns true if this call actually marked the object
    pub(crate) fn mark(&self) -> bool {
        let previous = self
            .state
            .fetch_or(GcState::MARKED.bits(), AtomicOrdering::AcqRel);
        (previous & GcState::MARKED.bits()) == 0
    }

    /// Checks if this object is marked
    pub(crate) fn is_marked(&self) -> bool {
        let flags = GcState::from_bits_truncate(self.state.load(AtomicOrdering::Acquire));
        flags.contains(GcState::MARKED)
    }

    /// Clears the mark bit (used during sweep phase)
    pub(crate) fn unmark(&self) {
        self.state
            .fetch_and(!GcState::MARKED.bits(), AtomicOrdering::AcqRel);
    }

    /// Resets the object to white color (used at the start of a new GC cycle)
    pub(crate) fn reset_to_white(&self) {
        self.state
            .store(GcState::WHITE.bits(), AtomicOrdering::Release);
    }
}

impl<T: Trace> Trace for GcBox<T> {
    fn trace(&self, collector: &GarbageCollector) {
        // Mark this GcBox as reachable in the object registry
        let ptr = SafePtr::from_ptr(std::ptr::from_ref::<GcBox<T>>(self).cast::<u8>());
        collector.mark_object(ptr);
        // Trace the contents of this GcBox to mark referenced objects
        self.data.trace(collector);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marking() {
        let gc_box = GcBox::new(42);

        assert!(!gc_box.is_marked());
        gc_box.mark();
        assert!(gc_box.is_marked());

        gc_box.unmark();
        assert!(!gc_box.is_marked());

        gc_box.reset_to_white();
        assert_eq!(gc_box.get_color(), GcState::WHITE);
    }

    #[test]
    fn test_color_management() {
        let gc_box = GcBox::new(42);

        // Initially should be white
        assert_eq!(gc_box.get_color(), GcState::WHITE);

        // Set to gray
        gc_box.set_color(GcState::GRAY);
        assert_eq!(gc_box.get_color(), GcState::GRAY);

        // Set to black
        gc_box.set_color(GcState::BLACK);
        assert_eq!(gc_box.get_color(), GcState::BLACK);

        // Reset to white
        gc_box.reset_to_white();
        assert_eq!(gc_box.get_color(), GcState::WHITE);
    }
}
