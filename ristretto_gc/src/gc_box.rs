use crate::GarbageCollector;
use crate::collector::Trace;
use crate::pointers::SafePtr;

/// Internal data structure for garbage-collected objects
pub(crate) struct GcBox<T> {
    pub(crate) data: T,
}

impl<T> GcBox<T> {
    pub(crate) fn new(data: T) -> Self {
        Self { data }
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
