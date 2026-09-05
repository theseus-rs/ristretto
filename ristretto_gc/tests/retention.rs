#![cfg(not(target_family = "wasm"))]

use ristretto_gc::{Finalize, GarbageCollector, Gc, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

struct Allocation {
    finalized: Arc<AtomicBool>,
    dropped: Arc<AtomicBool>,
}

impl Trace for Allocation {
    fn trace(&self, _collector: &GarbageCollector) {}
}

impl Finalize for Allocation {
    fn finalize(&self) {
        self.finalized.store(true, Ordering::Release);
    }
}

impl Drop for Allocation {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
    }
}

#[expect(clippy::expect_used, reason = "collection assertions in a test helper")]
fn collect(collector: &GarbageCollector) {
    let previous = collector
        .statistics()
        .expect("statistics")
        .collections_completed;
    collector.collect();
    let deadline = Instant::now() + Duration::from_secs(5);
    while collector
        .statistics()
        .expect("statistics")
        .collections_completed
        == previous
    {
        assert!(Instant::now() < deadline, "collection did not complete");
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn retained_allocation_survives_root_removal_and_is_finalized_at_shutdown() {
    let collector = GarbageCollector::new();
    collector.start();
    let retained_finalized = Arc::new(AtomicBool::new(false));
    let retained_dropped = Arc::new(AtomicBool::new(false));
    let retained = Gc::new_with_finalizer(
        &collector,
        Allocation {
            finalized: retained_finalized.clone(),
            dropped: retained_dropped.clone(),
        },
    );
    let normal_finalized = Arc::new(AtomicBool::new(false));
    let normal_dropped = Arc::new(AtomicBool::new(false));
    let normal = Gc::new_with_finalizer(
        &collector,
        Allocation {
            finalized: normal_finalized.clone(),
            dropped: normal_dropped.clone(),
        },
    );
    // Both allocations must have been marked before releasing their temporary roots.
    collect(&collector);
    let retained = retained.into_retained();
    drop(normal);
    collect(&collector);
    assert!(normal_finalized.load(Ordering::Acquire));
    assert!(normal_dropped.load(Ordering::Acquire));
    assert!(!retained_finalized.load(Ordering::Acquire));
    assert!(!retained_dropped.load(Ordering::Acquire));
    assert_eq!(1, collector.statistics().expect("statistics").objects_swept);
    drop(retained);
    collector.stop().expect("stop collector");
    assert!(retained_finalized.load(Ordering::Acquire));
    assert!(retained_dropped.load(Ordering::Acquire));
}
