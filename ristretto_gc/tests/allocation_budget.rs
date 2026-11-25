#![expect(
    clippy::panic_in_result_fn,
    reason = "tests propagate setup errors and assert invariants"
)]

use ristretto_gc::{Configuration, Error, GarbageCollector, Gc, Trace};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

struct Payload {
    collector: Weak<GarbageCollector>,
    drops: Arc<AtomicUsize>,
    charged_drops: Arc<AtomicUsize>,
}

// SAFETY: the weak collector link and atomic counters contain no managed edges.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Payload {
    fn trace(&self, _: &GarbageCollector) {}
}

impl Drop for Payload {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
        if let Some(collector) = self.collector.upgrade()
            && collector.managed_bytes() >= size_of::<Self>()
        {
            self.charged_drops.fetch_add(1, Ordering::SeqCst);
        }
    }
}

#[test]
fn limit_failure_and_success_refund_once_after_destruction() -> ristretto_gc::Result<()> {
    let collector = GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .max_heap_bytes(size_of::<Payload>())
            .build(),
    );
    collector.start();
    let drops = Arc::new(AtomicUsize::new(0));
    let charged = Arc::new(AtomicUsize::new(0));
    let make = || Payload {
        collector: Arc::downgrade(&collector),
        drops: drops.clone(),
        charged_drops: charged.clone(),
    };
    let root = Gc::try_new(&collector, make())?;
    assert_eq!(collector.managed_bytes(), size_of::<Payload>());
    assert!(matches!(
        Gc::try_new(&collector, make()),
        Err(Error::OutOfMemory)
    ));
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(collector.managed_bytes(), size_of::<Payload>());
    drop(root);
    collector.stop()?;
    assert_eq!(drops.load(Ordering::SeqCst), 2);
    assert_eq!(charged.load(Ordering::SeqCst), 2);
    assert_eq!(collector.managed_bytes(), 0);
    collector.stop()?;
    assert_eq!(drops.load(Ordering::SeqCst), 2);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test]
fn existing_constructor_uses_the_same_budget() -> ristretto_gc::Result<()> {
    let collector = GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .max_heap_bytes(size_of::<u64>())
            .build(),
    );
    collector.start();
    let root = Gc::new(&collector, 42_u64);
    assert_eq!(**root, 42);
    assert!(matches!(
        Gc::try_new(&collector, 7_u64),
        Err(Error::OutOfMemory)
    ));
    drop(root);
    collector.stop()?;
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}
