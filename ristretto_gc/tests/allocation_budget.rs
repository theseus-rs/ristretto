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
fn clearing_a_heap_buffer_without_element_destructors_runs_no_element_hooks() {
    struct Leaf;
    // SAFETY: this leaf contains no managed edges.
    #[expect(unsafe_code)]
    unsafe impl Trace for Leaf {
        fn may_contain_gc_references() -> bool {
            panic!("retirement invoked the element's tracing policy")
        }

        fn trace(&self, _: &GarbageCollector) {}
    }

    let mut buffer = ristretto_gc::HeapBuffer::from(vec![Leaf, Leaf]);
    assert!(ristretto_gc::HeapBuffer::<Leaf>::has_bounded_cleanup());
    assert_eq!(
        buffer.clear_step(0, 1),
        ristretto_gc::TraceProgress::Complete
    );
    assert!(buffer.is_empty());
}

#[test]
fn element_retirement_requires_bounded_destruction_before_clearing() {
    use ristretto_gc::HeapBuffer;

    assert!(Vec::<Option<String>>::has_bounded_cleanup());
    assert!(HeapBuffer::<Vec<u8>>::has_bounded_cleanup());
    assert!(!Vec::<Vec<String>>::has_bounded_cleanup());
    assert!(HeapBuffer::<u8>::has_bounded_cleanup());
    assert!(!Vec::<HeapBuffer<u8>>::has_bounded_cleanup());
    assert!(!HeapBuffer::<Payload>::has_bounded_cleanup());
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

#[test]
fn recovering_constructor_reclaims_an_unrooted_donor_at_the_logical_cap() -> ristretto_gc::Result<()>
{
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(32)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .build(),
    );
    drop(Gc::try_new(&gc, ((1u64, 2u64), (3u64, 4u64)))?);
    let before = gc.statistics()?.collections_completed;
    let Err((Error::OutOfMemory, value)) = Gc::try_new_once(&gc, 23u64) else {
        panic!("single attempt must return the unpublished value")
    };
    assert_eq!(gc.statistics()?.collections_completed, before);
    assert_eq!(gc.managed_bytes(), 32);
    let recovered = Gc::try_new(&gc, value)?;
    assert_eq!(**recovered, 23);
    assert_eq!(gc.managed_bytes(), 8);
    drop(recovered);
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn synchronous_recovery_retains_the_unpublished_parents_strong_edge() -> ristretto_gc::Result<()> {
    let size = Gc::<u64>::allocation_size(&7)?;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(size + 64)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let child = Gc::try_new(&gc, 7u64)?.into_rooted();
    let id = child.as_handle_i64();
    drop(Gc::try_new(
        &gc,
        ((0u64, 1u64, 2u64), (3u64, 4u64, 5u64), (6u64, 7u64)),
    )?);
    let parent = Gc::try_new(&gc, Some(child))?;
    let surviving = Gc::<u64>::from_handle_rooted(&gc, id)?;
    assert_eq!(*surviving, 7);
    assert_eq!((**parent).as_ref().map(Gc::as_handle_i64), Some(id));
    drop((parent, surviving));
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
