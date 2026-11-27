//! Logical bytes follow descriptor ownership, including publication unwind.
#![expect(clippy::panic_in_result_fn)]
use super::*;

struct Probe(Arc<AtomicUsize>);
// SAFETY: the fixture owns no outgoing managed references.
unsafe impl Trace for Probe {
    fn trace(&self, _: &GarbageCollector) {}
}
impl Finalize for Probe {
    fn finalize(&self) {}
}
impl Drop for Probe {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn private_metadata<T: Send + Sync + Trace + 'static>(
    gc: &GarbageCollector,
    value: T,
) -> Result<ObjectMetadata> {
    let layout = crate::gc::Allocation::<T>::layout();
    let reservation = gc.reserve_exact_bytes(layout.size())?;
    let storage = crate::allocation::Storage::global(layout)?;
    // SAFETY: storage has the complete allocation layout and is uninitialized.
    let allocation =
        unsafe { crate::allocation::Initialized::new(storage, crate::gc::Allocation::new(value)) };
    let mut metadata =
        ObjectMetadata::new_for_gc(allocation, layout.size(), crate::ObjectId::allocate()?);
    reservation.transfer(&mut metadata);
    Ok(metadata)
}

#[test]
fn dropping_private_metadata_refunds_its_charge() -> Result<()> {
    let gc = GarbageCollector::new();
    let drops = Arc::new(AtomicUsize::new(0));
    let metadata = private_metadata(&gc, Probe(drops.clone()))?;
    assert!(gc.managed_bytes() > 0);
    drop(metadata);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn diagnostics_unwind_does_not_strand_destroyed_payload_charge() -> Result<()> {
    struct PanicOnEvent;
    impl tracing::Subscriber for PanicOnEvent {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
        fn event(&self, _: &tracing::Event<'_>) {
            panic!("injected diagnostic unwind");
        }
        fn enter(&self, _: &tracing::span::Id) {}
        fn exit(&self, _: &tracing::span::Id) {}
    }
    struct Panicking(Probe);
    // SAFETY: this fixture owns no outgoing managed references.
    unsafe impl Trace for Panicking {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Panicking {
        fn drop(&mut self) {
            assert_eq!(self.0.0.load(Ordering::Relaxed), 0);
            panic!("injected payload unwind");
        }
    }
    for borrowed in [false, true] {
        let gc = GarbageCollector::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let metadata = private_metadata(&gc, Panicking(Probe(drops.clone())))?;
        let result = tracing::subscriber::with_default(PanicOnEvent, || {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if borrowed {
                    metadata.retire();
                    metadata.run_drop_fn_in(&gc);
                } else {
                    metadata.drop_object();
                }
            }))
        });
        assert!(result.is_err());
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(
            gc.managed_bytes(),
            0,
            "destroyed payload charge survived diagnostic unwind"
        );
        drop(metadata);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn published_descriptor_keeps_its_charge_when_constructor_unwinds() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for local in [false, true] {
                for constructor in 0..5 {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .root_storage(storage)
                            .local_allocation_buffers(local)
                            .allocation_threshold(usize::MAX)
                            .max_heap_bytes(8192)
                            .build(),
                    );
                    let context = gc.register_context();
                    let drops = Arc::new(AtomicUsize::new(0));
                    let charged = Arc::new(AtomicUsize::new(0));
                    *lock(&gc.metadata_publish_failure) = Some(charged.clone());
                    let result = context.with(|| {
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let value = Probe(drops.clone());
                            match constructor {
                                0 => drop(Gc::try_new(&gc, value)),
                                1 => drop(Gc::new(&gc, value)),
                                2 => drop(Gc::new_with_finalizer(&gc, value)),
                                // SAFETY: collection is disabled during construction;
                                // the injected hook unwinds before an unrooted handle escapes.
                                3 => drop(unsafe { Gc::with_collector_unrooted(&gc, value) }),
                                _ => drop(unsafe {
                                    Gc::with_collector_and_finalizer_unrooted(&gc, value)
                                }),
                            }
                        }))
                    });
                    assert!(result.is_err());
                    assert_eq!(drops.load(Ordering::Relaxed), 0);
                    assert_eq!(gc.statistics()?.live_objects, 1);
                    assert!(charged.load(Ordering::Acquire) > 0);
                    assert_eq!(
                        gc.managed_bytes(),
                        charged.load(Ordering::Acquire),
                        "published storage was refunded by its constructor"
                    );
                    drop(context);
                    gc.stop()?;
                    assert_eq!(drops.load(Ordering::Relaxed), 1);
                    assert_eq!(gc.managed_bytes(), 0);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn private_metadata_owns_and_refunds_transferred_credit() -> Result<()> {
    let gc = GarbageCollector::new();
    let drops = Arc::new(AtomicUsize::new(0));
    let layout = crate::gc::Allocation::<Probe>::layout();
    let reservation = gc.reserve_exact_bytes(layout.size())?;
    let storage = crate::allocation::Storage::global(layout)?;
    // SAFETY: storage has the complete allocation layout and is uninitialized.
    let allocation = unsafe {
        crate::allocation::Initialized::new(
            storage,
            crate::gc::Allocation::new(Probe(drops.clone())),
        )
    };
    let mut metadata =
        ObjectMetadata::new_for_gc(allocation, layout.size(), crate::ObjectId::allocate()?);
    let strong = Arc::strong_count(&gc);
    reservation.transfer(&mut metadata);
    assert_eq!(
        Arc::strong_count(&gc),
        strong,
        "charge created a collector cycle"
    );
    assert_eq!(gc.statistics()?.live_objects, 0);
    assert_eq!(gc.managed_bytes(), layout.size());
    let metadata = MetadataRef::new(metadata);
    let stale_owner = metadata.clone();
    metadata.drop_object();
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(metadata);
    stale_owner.drop_object();
    drop(stale_owner);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn retired_storage_lease_keeps_credit_but_old_metadata_does_not() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let drops = Arc::new(AtomicUsize::new(0));
        let root = Gc::try_new(&gc, Probe(drops.clone()))?;
        let metadata = gc
            .metadata_for_address(root.as_ptr() as usize)
            .expect("metadata");
        let lease = ObjectMetadata::lease(&metadata).expect("live storage lease");
        let charged = gc.managed_bytes();
        drop(root);
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.managed_bytes(), charged);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        metadata.run_drop_fn();
        assert_eq!(gc.managed_bytes(), charged);
        drop(lease);
        gc.stop()?;
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        metadata.run_drop_fn();
        drop(metadata);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn transferred_charge_does_not_keep_a_destroyed_heap_alive() -> Result<()> {
    let gc = GarbageCollector::new();
    let weak = gc.weak();
    let drops = Arc::new(AtomicUsize::new(0));
    let root = Gc::try_new(&gc, Probe(drops.clone()))?;
    let metadata = gc
        .metadata_for_address(root.as_ptr() as usize)
        .expect("metadata");
    let lease = ObjectMetadata::lease(&metadata).expect("storage lease");
    drop(root);
    drop(gc);
    assert!(weak.upgrade().is_none());
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(metadata);
    drop(lease);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn panicking_payload_refunds_only_after_its_drop_glue_runs() -> Result<()> {
    struct Panicking {
        collector: Weak<GarbageCollector>,
        charged_in_drop: Arc<AtomicUsize>,
        child: Probe,
    }
    // SAFETY: this fixture owns no outgoing managed references.
    unsafe impl Trace for Panicking {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Panicking {
        fn drop(&mut self) {
            let gc = self.collector.upgrade().expect("collector still lives");
            self.charged_in_drop
                .store(gc.managed_bytes(), Ordering::Release);
            assert_eq!(self.child.0.load(Ordering::Relaxed), 0);
            panic!("injected payload destructor unwind");
        }
    }
    let gc = GarbageCollector::new();
    let drops = Arc::new(AtomicUsize::new(0));
    let charged_in_drop = Arc::new(AtomicUsize::new(0));
    let root = Gc::try_new(
        &gc,
        Panicking {
            collector: gc.weak(),
            charged_in_drop: charged_in_drop.clone(),
            child: Probe(drops.clone()),
        },
    )?;
    let charged = gc.managed_bytes();
    drop(root);
    gc.stop()?;
    assert_eq!(charged_in_drop.load(Ordering::Acquire), charged);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_foreign_refund_collector_cannot_claim_or_refund_storage() -> Result<()> {
    let owner = GarbageCollector::new();
    let foreign = GarbageCollector::new();
    let drops = Arc::new(AtomicUsize::new(0));
    let metadata = private_metadata(&owner, Probe(drops.clone()))?;
    let charged = owner.managed_bytes();
    metadata.retire();
    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        metadata.run_drop_fn_in(&foreign);
    }));
    assert!(rejected.is_err());
    assert!(metadata.storage_ready(), "foreign heap claimed storage");
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(owner.managed_bytes(), charged);
    assert_eq!(foreign.managed_bytes(), 0);
    metadata.run_drop_fn_in(&owner);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(owner.managed_bytes(), 0);
    metadata.run_drop_fn_in(&owner);
    drop(metadata);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(owner.managed_bytes(), 0);
    assert_eq!(foreign.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn finalizer_diagnostics_unwind_does_not_strand_private_charge() -> Result<()> {
    struct PanicOnEvent;
    impl tracing::Subscriber for PanicOnEvent {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }
        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }
        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
        fn event(&self, _: &tracing::Event<'_>) {
            panic!("injected diagnostic unwind");
        }
        fn enter(&self, _: &tracing::span::Id) {}
        fn exit(&self, _: &tracing::span::Id) {}
    }

    struct Finalized(Probe);
    // SAFETY: this fixture owns no outgoing managed references.
    unsafe impl Trace for Finalized {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Finalize for Finalized {
        fn finalize(&self) {
            assert_eq!(self.0.0.load(Ordering::Relaxed), 0);
            panic!("injected finalizer unwind");
        }
    }
    let gc = GarbageCollector::new();
    let drops = Arc::new(AtomicUsize::new(0));
    let layout = crate::gc::Allocation::<Finalized>::layout();
    let reservation = gc.reserve_exact_bytes(layout.size())?;
    let storage = crate::allocation::Storage::global(layout)?;
    // SAFETY: storage has the complete allocation layout and is uninitialized.
    let allocation = unsafe {
        crate::allocation::Initialized::new(
            storage,
            crate::gc::Allocation::new(Finalized(Probe(drops.clone()))),
        )
    };
    let mut metadata = ObjectMetadata::new_for_gc_with_finalizer(
        allocation,
        layout.size(),
        crate::ObjectId::allocate()?,
    );
    reservation.transfer(&mut metadata);
    let result = tracing::subscriber::with_default(PanicOnEvent, || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(metadata)))
    });
    assert!(result.is_err());
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(
        gc.managed_bytes(),
        0,
        "private finalizer unwind bypassed refund"
    );
    Ok(())
}

#[test]
fn partial_context_credit_combines_with_an_independent_refund() -> Result<()> {
    let batch = crate::allocation::CREDIT;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(batch + 4)
            .local_allocation_buffers(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let four = gc.reserve_exact_bytes(4)?;
    let scope = gc.register_context();
    let pinned = scope.with(|| {
        [1024, 1024, 1024, 1020]
            .into_iter()
            .map(|size| gc.reserve_bytes(size))
            .collect::<Result<Vec<_>>>()
    })?;
    assert_eq!(gc.managed_bytes(), batch + 4);
    let wait = scope.with(|| gc.observe_space(8))?;
    assert!(!wait.was_offered());
    assert_eq!(gc.allocation_headroom(), 4);
    drop(four);
    assert!(wait.was_offered());
    let root = scope.with(|| Gc::try_new(&gc, 42_u64))?;
    assert_eq!(**root, 42);
    assert_eq!(gc.managed_bytes(), batch + 4);
    drop((wait, root, pinned, scope));
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
