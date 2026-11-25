//! A certified primitive projection preserves empty tracing across its complete lifetime.
#![expect(unsafe_code, reason = "fixtures certify only the entire Empty variant")]
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::cell::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, RootStorage, Trace,
    TraceProgress,
};

#[derive(Debug)]
enum Payload {
    Empty([u64; 2]),
    Edges(Vec<Gc<u64>>),
}
// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Payload {
    fn root(&mut self) {
        if let Self::Edges(edges) = self {
            edges.root();
        }
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        if let Self::Edges(edges) = self {
            unsafe { Trace::unroot(edges, owner) };
        }
    }
    fn trace(&self, gc: &GarbageCollector) {
        if let Self::Edges(edges) = self {
            edges.trace(gc);
        }
    }
}
fn project(value: &mut Payload) -> Option<&mut [u64]> {
    match value {
        Payload::Empty(values) => Some(values),
        Payload::Edges(_) => None,
    }
}
fn finish(gc: &GarbageCollector) -> Result<()> {
    for _ in 0..20_000 {
        if gc.step() {
            return Ok(());
        }
    }
    Err(ristretto_gc::Error::CollectionPhaseError(
        "projection blocked collection".into(),
    ))
}

#[test]
fn typed_projection_allows_cycles_but_later_unrestricted_writers_still_trace() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .root_storage(storage)
                        .independent_phases(independent)
                        .independent_marking(independent)
                        .independent_relocation(independent)
                        .compact_heap(true)
                        .overlapping_generations(true)
                        .old_collection_interval(usize::MAX)
                        .allocation_threshold(usize::MAX)
                        .full_collection_interval(usize::MAX)
                        .incremental_step_size(8)
                        .max_pause_time_us(0)
                        .diagnostics(true)
                        .build(),
                );
                let owner = Gc::try_new(&gc, RwLock::new(Payload::Empty([7, 9])))?;
                // SAFETY: project selects the entire trace-empty variant and exposes only u64s.
                let guard =
                    unsafe { RwLockWriteGuard::try_map_trace_empty(owner.write(), project) }
                        .map_err(|_| {
                            ristretto_gc::Error::CollectionPhaseError(
                                "missing empty projection".into(),
                            )
                        })?;
                let failed = MappedRwLockWriteGuard::try_map(guard, |_| None::<&mut u64>);
                let guard = failed.err().ok_or_else(|| {
                    ristretto_gc::Error::CollectionPhaseError(
                        "nested projection unexpectedly succeeded".into(),
                    )
                })?;
                let mut guard = MappedRwLockWriteGuard::map(guard, |values| {
                    values.first_mut().expect("nonempty payload")
                });
                let pointer = std::ptr::from_ref(&*guard);
                gc.collect_old();
                finish(&gc)?;
                gc.collect_young();
                finish(&gc)?;
                for _ in 0..3 {
                    drop(Gc::try_new(&gc, 17_u64)?);
                    gc.collect();
                    finish(&gc)?;
                    assert_eq!(gc.statistics()?.live_objects, 1);
                    assert_eq!(std::ptr::from_ref(&*guard), pointer);
                    assert_eq!(*guard, 7);
                }
                *guard = 11;
                drop(guard);
                let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
                let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
                *owner.write() = Payload::Edges(vec![child]);
                let full = gc.statistics()?.full_collections;
                let guard = owner.write();
                assert!(matches!(
                    (**owner).trace_step(&gc, 0, 8),
                    TraceProgress::Busy
                ));
                gc.collect();
                for _ in 0..1000 {
                    gc.step();
                }
                assert_eq!(gc.statistics()?.full_collections, full);
                drop(guard);
                finish(&gc)?;
                assert!(weak.upgrade().is_some());
                drop(owner);
                gc.stop()?;
                assert!(weak.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
                assert_eq!(gc.diagnostics().active_payload_guards, 0);
            }
        }
    }
    Ok(())
}

#[test]
fn failed_projection_cannot_certify_a_reference_bearing_parent() -> Result<()> {
    let gc = GarbageCollector::new();
    let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
    let owner = Gc::try_new(&gc, RwLock::new(Payload::Edges(vec![child])))?;
    // SAFETY: project returns None for Edges and can certify only Empty.
    let guard = unsafe { RwLockWriteGuard::try_map_trace_empty(owner.write(), project) }
        .err()
        .ok_or_else(|| {
            ristretto_gc::Error::CollectionPhaseError("ineligible projection succeeded".into())
        })?;
    assert!(matches!(
        (**owner).trace_step(&gc, 0, 8),
        TraceProgress::Busy
    ));
    drop(guard);
    gc.collect_full()?;
    assert_eq!(gc.statistics()?.live_objects, 2);
    drop(owner);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn unwind_revokes_before_another_writer_can_introduce_edges() -> Result<()> {
    for inside_callback in [false, true] {
        let gc = GarbageCollector::new();
        let owner = Gc::try_new(&gc, RwLock::new(Payload::Empty([7, 9])))?;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // SAFETY: a successful callback exposes only the entire Empty variant.
            let _guard = unsafe {
                RwLockWriteGuard::try_map_trace_empty(owner.write(), |value| {
                    assert!(!inside_callback, "callback cancellation");
                    project(value)
                })
            }
            .expect("empty projection");
            panic!("guard cancellation");
        }));
        assert!(result.is_err());
        let guard = owner.write();
        assert!(matches!(
            (**owner).trace_step(&gc, 0, 8),
            TraceProgress::Busy
        ));
        drop(guard);
        drop(owner);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_stopped_native_projection_does_not_hold_up_unrelated_reclamation() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .compact_heap(true)
                .allocation_threshold(usize::MAX)
                .diagnostics(true)
                .build(),
        );
        let owner = Gc::try_new(&gc, RwLock::new(Payload::Empty([7, 9])))?;
        std::thread::scope(|scope| -> Result<()> {
            let (ready_tx, ready_rx) = std::sync::mpsc::channel();
            let (resume_tx, resume_rx) = std::sync::mpsc::channel();
            let borrowed = &owner;
            let worker = scope.spawn(move || {
                // SAFETY: the projection exposes only the entire Empty variant.
                let guard =
                    unsafe { RwLockWriteGuard::try_map_trace_empty(borrowed.write(), project) }
                        .expect("projection");
                ready_tx.send(()).expect("collector waiting");
                let _ = resume_rx.recv();
                assert_eq!(guard.first(), Some(&7));
            });
            ready_rx.recv().expect("projection published");
            for _ in 0..3 {
                drop(Gc::try_new(&gc, 42_u64)?);
                gc.collect();
                finish(&gc)?;
                assert_eq!(gc.statistics()?.live_objects, 1);
            }
            resume_tx.send(()).expect("release stopped writer");
            worker.join().expect("writer finished");
            Ok(())
        })?;
        drop(owner);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
    }
    Ok(())
}
