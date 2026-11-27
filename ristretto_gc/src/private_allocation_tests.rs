#![expect(
    clippy::panic_in_result_fn,
    reason = "tests propagate setup errors and assert ownership"
)]
use super::*;
use crate::{GcRootGuard, ObjectId};

fn storage<T>(value: T) -> Result<crate::allocation::Initialized<crate::gc::Allocation<T>>> {
    let storage = crate::allocation::Storage::global(crate::gc::Allocation::<T>::layout())?;
    // SAFETY: the new global allocation has the exact layout of Allocation<T>.
    Ok(unsafe { crate::allocation::Initialized::new(storage, crate::gc::Allocation::new(value)) })
}

struct Protected {
    child: GcRootGuard<u64>,
    drops: Arc<AtomicUsize>,
}
// SAFETY: child has its own owning root for the entire lifetime of the payload.
unsafe impl Trace for Protected {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Protected {
    fn drop(&mut self) {
        assert_eq!(**self.child, 73);
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn cancellation_releases_payload_then_child_root_and_credit() -> Result<()> {
    let collector = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    let child = Gc::try_new(&collector, 73_u64)?;
    let child_bytes = collector.managed_bytes();
    let drops = Arc::new(AtomicUsize::new(0));
    let value = Protected {
        child,
        drops: drops.clone(),
    };
    let bytes = Gc::allocation_size(&value)?;
    let metadata = collector.prepare_allocated_object(
        storage(value)?,
        collector.reserve_exact_bytes(bytes)?,
        ObjectId::allocate()?,
    );
    assert_eq!(collector.root_statistics().compatibility_roots, 1);
    assert_eq!(collector.managed_bytes(), child_bytes + bytes);
    // Constructing a descriptor does not publish it. Dropping that sole owner is
    // the same cleanup path used when a prepared request is cancelled.
    assert_eq!(collector.statistics()?.live_objects, 1);
    drop(metadata);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(collector.root_statistics().compatibility_roots, 0);
    assert_eq!(collector.managed_bytes(), child_bytes);
    collector.cleanup_remaining_objects();
    assert_eq!(collector.managed_bytes(), 0);
    assert_eq!(collector.statistics()?.bytes_live, 0);
    Ok(())
}

#[test]
fn metadata_cleanup_transfers_payload_ownership_exactly_once() -> Result<()> {
    struct Probe(Arc<AtomicUsize>);
    impl Drop for Probe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    // SAFETY: Probe contains only a host atomic counter.
    unsafe impl Trace for Probe {
        fn trace(&self, _: &GarbageCollector) {}
    }
    let collector = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    let drops = Arc::new(AtomicUsize::new(0));
    let value = Probe(drops.clone());
    let bytes = Gc::allocation_size(&value)?;
    let metadata = collector.prepare_allocated_object(
        storage(value)?,
        collector.reserve_exact_bytes(bytes)?,
        ObjectId::allocate()?,
    );
    metadata.run_drop_fn();
    metadata.run_drop_fn();
    drop(metadata);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[test]
fn finalizer_precedes_once_only_storage_release() -> Result<()> {
    struct Payload(Arc<AtomicUsize>);
    impl Finalize for Payload {
        fn finalize(&self) {
            assert_eq!(self.0.fetch_add(1, Ordering::SeqCst), 0);
        }
    }
    impl Drop for Payload {
        fn drop(&mut self) {
            assert_eq!(self.0.fetch_add(1, Ordering::SeqCst), 1);
        }
    }
    // SAFETY: Payload contains only a host atomic counter.
    unsafe impl Trace for Payload {
        fn trace(&self, _: &GarbageCollector) {}
    }
    let collector = GarbageCollector::new();
    let events = Arc::new(AtomicUsize::new(0));
    let value = Payload(events.clone());
    let bytes = Gc::allocation_size(&value)?;
    let metadata = collector.prepare_allocated_object_with_finalizer(
        storage(value)?,
        collector.reserve_exact_bytes(bytes)?,
        ObjectId::allocate()?,
    );
    metadata.drop_object();
    metadata.drop_object();
    drop(metadata);
    assert_eq!(events.load(Ordering::SeqCst), 2);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}
