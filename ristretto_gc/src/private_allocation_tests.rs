#![expect(
    clippy::panic_in_result_fn,
    reason = "tests propagate setup errors and assert ownership"
)]
use super::*;
use crate::private_allocation::Allocation;

struct Protected {
    child: GcRootGuard<u64>,
    drops: Arc<AtomicUsize>,
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
    let reservation = collector.reserve_exact_bytes(size_of::<Protected>())?;
    let (allocation, _) = Allocation::new(
        Protected {
            child,
            drops: drops.clone(),
        },
        reservation,
    )?;
    assert_eq!(collector.roots.len(), 1);
    assert_eq!(
        collector.managed_bytes(),
        child_bytes + size_of::<Protected>()
    );
    // Constructing a descriptor does not publish it. Dropping that sole owner is
    // the same cleanup path used when a prepared request is cancelled.
    let metadata = ObjectMetadata::new_for_gc(allocation);
    assert_eq!(collector.objects.len(), 1);
    drop(metadata);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(collector.roots.len(), 0);
    assert_eq!(collector.managed_bytes(), child_bytes);
    collector.cleanup_remaining_objects();
    assert_eq!(collector.managed_bytes(), 0);
    assert_eq!(collector.statistics()?.bytes_live, 0);
    Ok(())
}

#[test]
fn unpublished_allocation_can_move_without_publishing_a_pointer() -> Result<()> {
    let collector = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    let reservation = collector.reserve_exact_bytes(size_of::<u64>())?;
    let (allocation, _) = Allocation::new(91_u64, reservation)?;
    let owner = Some(allocation);
    assert_eq!(collector.objects.len(), 0);
    assert_eq!(collector.roots.len(), 0);
    assert_eq!(collector.statistics()?.bytes_live, 0);
    assert_eq!(collector.managed_bytes(), size_of::<u64>());
    drop(owner);
    assert_eq!(collector.managed_bytes(), 0);
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
    let collector = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    let drops = Arc::new(AtomicUsize::new(0));
    let reservation = collector.reserve_exact_bytes(size_of::<Probe>())?;
    let (allocation, _) = Allocation::new(Probe(drops.clone()), reservation)?;
    let metadata = ObjectMetadata::new_for_gc(allocation);
    metadata.run_drop_fn();
    metadata.run_drop_fn();
    drop(metadata);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn unwinding_payload_still_refunds_its_private_reservation() -> Result<()> {
    struct Panics;
    impl Drop for Panics {
        fn drop(&mut self) {
            std::panic::panic_any("private payload test unwind");
        }
    }
    let collector = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    // A ZST still has a destructor. A nonzero reservation proves refund on unwind.
    let reservation = collector.reserve_exact_bytes(17)?;
    let (allocation, _) = Allocation::new(Panics, reservation)?;
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(allocation)));
    assert!(outcome.is_err());
    assert_eq!(collector.managed_bytes(), 0);
    Ok(())
}
