#![cfg(not(target_family = "wasm"))]

use ristretto_gc::{Finalize, GarbageCollector, Gc, Trace};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

struct Allocation {
    finalized: Arc<AtomicBool>,
    dropped: mpsc::Sender<()>,
}

// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Allocation {
    fn trace(&self, _collector: &GarbageCollector) {}
}

impl Finalize for Allocation {
    fn finalize(&self) {
        self.finalized.store(true, Ordering::Release);
    }
}

impl Drop for Allocation {
    fn drop(&mut self) {
        let _ = self.dropped.send(());
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
    let (retained_drop, retained_dropped) = mpsc::channel();
    let retained = Gc::new_with_finalizer(
        &collector,
        Allocation {
            finalized: retained_finalized.clone(),
            dropped: retained_drop,
        },
    );
    let normal_finalized = Arc::new(AtomicBool::new(false));
    let (normal_drop, normal_dropped) = mpsc::channel();
    let normal = Gc::new_with_finalizer(
        &collector,
        Allocation {
            finalized: normal_finalized.clone(),
            dropped: normal_drop,
        },
    );
    // Both allocations must have been marked before releasing their temporary roots.
    collect(&collector);
    let retained = retained.into_retained();
    drop(normal);
    collect(&collector);
    // The logical cycle can finish while its native cleanup is still in flight.
    // Observe physical destruction from this async request, without helping it.
    normal_dropped
        .recv_timeout(Duration::from_secs(5))
        .expect("ordinary allocation destroyed by native cleanup");
    assert!(normal_finalized.load(Ordering::Acquire));
    assert!(!retained_finalized.load(Ordering::Acquire));
    assert_eq!(retained_dropped.try_recv(), Err(mpsc::TryRecvError::Empty));
    assert_eq!(1, collector.statistics().expect("statistics").objects_swept);
    drop(retained);
    collector.stop().expect("stop collector");
    assert!(retained_finalized.load(Ordering::Acquire));
    assert_eq!(retained_dropped.try_recv(), Ok(()));
}
