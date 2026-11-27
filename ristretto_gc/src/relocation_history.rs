//! Optional per-payload observations. Region IDs are advisory and never dereferenced.
use crate::{Trace, TraceProgress, diagnostics, region::RegionHeap};
use portable_atomic::AtomicU64;
use std::sync::{Mutex, atomic::Ordering};
use std::time::Instant;

struct Pending {
    started: Option<Instant>,
}

pub(crate) struct History {
    region: AtomicU64,
    lookup: fn(*const (), &RegionHeap) -> Option<u64>,
    // Pending records a failed attempt even on hosts with no clock.
    busy: Mutex<Option<Pending>>,
}

impl History {
    pub(crate) fn new<T: Trace>(value: &T, heap: &RegionHeap) -> Option<Self> {
        Some(Self {
            region: AtomicU64::new(value.relocation_region(heap)?),
            lookup: Self::lookup::<T>,
            busy: Mutex::new(None),
        })
    }

    fn lookup<T: Trace>(ptr: *const (), heap: &RegionHeap) -> Option<u64> {
        // SAFETY: the constructor pairs this callback with T; the collector holds
        // a descriptor storage lease across the copy callback and this lookup.
        unsafe { (&*ptr.cast::<T>()).relocation_region(heap) }
    }

    pub(crate) fn observe(
        &self,
        progress: TraceProgress,
        ptr: *const (),
        heap: &RegionHeap,
        timing: Option<&diagnostics::Timing>,
    ) {
        if progress == TraceProgress::Busy {
            heap.record_pin(self.region.load(Ordering::Relaxed));
            if timing.is_some() {
                self.busy
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .get_or_insert_with(|| Pending {
                        started: diagnostics::now(),
                    });
            }
        } else {
            if let Some(region) = (self.lookup)(ptr, heap) {
                self.region.store(region, Ordering::Relaxed);
            }
            if let Some(timing) = timing
                && let Some(started) = self
                    .busy
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .take()
            {
                timing.record(started.started);
            }
        }
    }
}
