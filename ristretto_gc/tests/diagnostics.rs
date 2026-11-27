//! Diagnostics distinguish recovery from explicit collection and remain optional.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, GarbageCollector, Gc, Result};

#[test]
fn guard_hold_timing_follows_mapped_ownership_and_excludes_failed_acquisition() -> Result<()> {
    use ristretto_gc::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
    for (diagnostics, policy) in [(false, true), (true, false), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .cost_aware_relocation(policy)
                .diagnostics(diagnostics)
                .build(),
        );
        let value = Gc::try_new(&gc, RwLock::new(vec![1_u64, 2]))?;
        let enabled = diagnostics && policy;
        let before = gc.diagnostics().payload_guard_holds.count;
        let read = value.read();
        assert_eq!(gc.diagnostics().active_payload_guards, usize::from(enabled));
        assert!(value.try_write().is_none());
        let read = RwLockReadGuard::map(read, |values| values.as_slice());
        let read = ristretto_gc::sync::MappedRwLockReadGuard::try_map(read, <[u64]>::first)
            .map_err(|_| ristretto_gc::Error::InvalidPointer("missing mapped element".into()))?;
        assert_eq!(*read, 1);
        assert_eq!(gc.diagnostics().payload_guard_holds.count, before);
        drop(read);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
        assert_eq!(
            gc.diagnostics().payload_guard_holds.count,
            before + u64::from(enabled)
        );
        let write = value.try_write().expect("write after mapped read unlocks");
        assert!(value.try_read().is_none());
        let mut write = RwLockWriteGuard::map(write, |values| values.as_mut_slice());
        write.fill(3);
        drop(write);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
        assert_eq!(
            gc.diagnostics().payload_guard_holds.count,
            before + 2 * u64::from(enabled)
        );
        let first = value.read();
        let second = value.try_read().expect("shared read");
        assert_eq!(
            gc.diagnostics().active_payload_guards,
            2 * usize::from(enabled)
        );
        drop(first);
        assert_eq!(gc.diagnostics().active_payload_guards, usize::from(enabled));
        drop(second);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
        assert_eq!(
            gc.diagnostics().payload_guard_holds.count,
            before + 4 * u64::from(enabled)
        );
        if !gc.diagnostics().clock_available {
            assert_eq!(gc.diagnostics().payload_guard_holds.total_ns, 0);
        }
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn a_panicking_projection_releases_its_pin_sample_once() -> Result<()> {
    use ristretto_gc::sync::{RwLock, RwLockWriteGuard};
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .cost_aware_relocation(true)
            .diagnostics(true)
            .build(),
    );
    let value = Gc::try_new(&gc, RwLock::new(1_u64))?;
    let before = gc.diagnostics().payload_guard_holds.count;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = RwLockWriteGuard::map(value.write(), |_: &mut u64| -> &mut u64 {
            panic!("projection panics while owning a managed guard");
        });
    }));
    assert!(result.is_err());
    assert_eq!(gc.diagnostics().active_payload_guards, 0);
    assert_eq!(gc.diagnostics().payload_guard_holds.count, before + 1);
    assert!(value.try_write().is_some());
    Ok(())
}

#[test]
fn recovery_and_explicit_collection_have_separate_counters() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .diagnostics(true)
            .max_heap_bytes(128)
            .build(),
    );
    let live = Gc::new(&gc, 42_u64);
    gc.collect_full()?;
    assert_eq!(gc.diagnostics().full_collection_calls, 1);
    assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
    assert!(gc.make_room(129).is_err());
    assert_eq!(gc.diagnostics().allocation_recovery.count, 0);
    assert!(gc.make_room(128).is_err());
    let metrics = gc.diagnostics();
    assert_eq!(metrics.full_collection_calls, 1);
    assert_eq!(gc.statistics()?.full_collections, 3);
    assert_eq!(metrics.allocation_recovery.count, 2);
    assert!(metrics.caller_steps.count > 0);
    assert_eq!(metrics.worker_steps.count, 0);
    assert_eq!(**live, 42);
    Ok(())
}

#[test]
fn disabled_diagnostics_leave_counters_empty() -> Result<()> {
    let gc = GarbageCollector::new();
    drop(Gc::new(&gc, 42_u64));
    gc.collect_full()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    let metrics = gc.diagnostics();
    assert!(!metrics.enabled);
    assert_eq!(metrics.full_collection_calls, 0);
    assert_eq!(metrics.caller_steps.count, 0);
    assert_eq!(metrics.trace_tasks, 0);
    assert_eq!(metrics.root_acquire.count, 0);
    assert_eq!(metrics.root_release.count, 0);
    assert_eq!(metrics.barrier_record.count, 0);
    Ok(())
}

#[test]
fn operation_attribution_counts_publication_and_drained_work() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .diagnostics(true)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .barrier_buffer_size(4)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .build(),
    );
    let context = gc.register_context();
    let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
    assert_eq!(gc.diagnostics().descriptor_publication.count, 1);
    assert_eq!(gc.diagnostics().root_acquire.count, 1);
    gc.collect();
    assert!(!gc.step());
    context.with(|| drop(root));
    let before = gc.diagnostics();
    assert_eq!(before.root_release.count, 1);
    assert!(before.barrier_record.count >= 1);
    assert_eq!(before.barrier_drain.count, 0);
    context.flush();
    assert_eq!(
        gc.diagnostics().barrier_drain.count,
        before.barrier_buffered
    );
    gc.stop()?;
    Ok(())
}

#[test]
fn converting_cloned_guards_transfers_the_existing_registration() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration::builder().diagnostics(true).build());
    let root = Gc::new(&gc, 42_u64);
    let second = root.clone();
    let handle = root.into_rooted();
    let other = second.into_rooted();
    assert_eq!(gc.diagnostics().root_registrations, 1);
    drop(handle);
    gc.collect_full()?;
    assert_eq!(*other, 42);
    drop(other);
    gc.collect_full()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn a_replaced_guard_handle_gets_its_own_registration() -> Result<()> {
    let gc = GarbageCollector::new();
    let mut original = Gc::new(&gc, 1_u64);
    let replacement = Gc::new(&gc, 2_u64);
    *original = replacement.clone_gc();
    let handle = original.into_rooted();
    drop(replacement);
    gc.collect_full()?;
    assert_eq!(*handle, 2);
    assert_eq!(gc.statistics()?.live_objects, 1);
    Ok(())
}
