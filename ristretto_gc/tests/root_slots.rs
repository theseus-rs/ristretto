//! Owning roots retain barriers and lifecycle guarantees when their slots are reused.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, RootStorage,
};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Waker};

fn collector() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(16)
            .work_stealing(true)
            .independent_marking(true)
            .mark_batch_size(8)
            .root_storage(RootStorage::Slots)
            .allocation_threshold(usize::MAX)
            .max_pause_time_us(0)
            .incremental_step_size(8)
            .build(),
    )
}

#[test]
fn slots_are_selectable_and_numeric_root_ids_remain_compatible() -> Result<()> {
    assert_eq!(Configuration::default().root_storage, RootStorage::Tree);
    let gc = collector();
    let root = Gc::try_new(&gc, 42_u64)?;
    let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
    let id = gc.add_root(&root);
    assert_eq!(gc.root_statistics().slot_roots, 1);
    assert_eq!(gc.root_statistics().compatibility_roots, 1);
    drop(root);
    gc.collect_full()?;
    assert!(weak.upgrade().is_some());
    gc.remove_root_by_id(id);
    gc.stop()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.root_statistics().pages, 0);
    assert_eq!(gc.root_statistics().compatibility_roots, 0);
    Ok(())
}

#[test]
fn cloned_guards_transfer_one_slot_and_release_it_once() -> Result<()> {
    let gc = collector();
    let root = Gc::try_new(&gc, 42_u64)?;
    let second = root.clone();
    let handle = root.into_rooted();
    let other = second.into_rooted();
    assert_eq!(gc.root_statistics().slot_roots, 1);
    drop(handle);
    gc.collect_full()?;
    assert_eq!(*other, 42);
    drop(other);
    gc.stop()?;
    assert_eq!(gc.root_statistics().slot_roots, 0);
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn owning_and_edge_conversions_preserve_collector_and_reference_lifetimes() -> Result<()> {
    for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
        for concurrent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .root_storage(storage)
                    .concurrent_publication(concurrent)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let original = context.with(|| Gc::try_new(&gc, 42_u64))?.into_rooted();
            let weak = GcWeak::new(&original, ReferenceKind::Weak, 0);
            let mut edge = original.clone();
            // SAFETY: The original owning handle protects the edge until it is re-rooted.
            #[expect(unsafe_code, reason = "explicit non-owning edge fixture")]
            unsafe {
                edge.unroot();
            };
            let rerooted = context.with(|| edge.rooted_clone());
            drop((original, context));
            gc.collect_full()?;
            assert_eq!(weak.upgrade().as_deref(), Some(&42));
            assert_eq!(*rerooted, 42);
            drop(edge);
            gc.collect_full()?;
            let resolved = Gc::<u64>::from_handle_rooted(&gc, rerooted.as_handle_i64())?;
            let collector = Arc::downgrade(&gc);
            drop(gc);
            drop(rerooted);
            assert!(collector.upgrade().is_some());
            assert_eq!(*resolved, 42);
            drop(resolved);
            assert!(collector.upgrade().is_none());
            assert!(weak.upgrade().is_none());
        }
    }
    Ok(())
}

#[test]
fn replaced_guard_handle_registers_the_replacement() -> Result<()> {
    let gc = collector();
    let mut original = Gc::try_new(&gc, 1_u64)?;
    let replacement = Gc::try_new(&gc, 2_u64)?;
    *original = replacement.clone_gc();
    let handle = original.into_rooted();
    drop(replacement);
    gc.collect_full()?;
    assert_eq!(*handle, 2);
    assert_eq!(gc.root_statistics().slot_roots, 1);
    assert_eq!(gc.statistics()?.live_objects, 1);
    Ok(())
}

#[test]
#[expect(
    unsafe_code,
    reason = "explicit compatibility revocation lifetime fixture"
)]
fn compatibility_removal_cannot_make_a_late_drop_remove_a_reused_slot() -> Result<()> {
    let gc = collector();
    let old = Gc::try_new(&gc, 7_u64)?;
    // Compatibility removal intentionally revokes roots. No collector runs
    // between removal and registering this still-valid allocation again.
    // SAFETY: no collection runs until a replacement registration protects the
    // still-live allocation; old is subsequently dropped without payload access.
    unsafe { gc.remove_root(&**old) };
    let current = gc.create_root_guard(old.clone_gc());
    assert_eq!(gc.root_statistics().slot_reuses, 1);
    drop(old);
    gc.collect_full()?;
    assert_eq!(**current, 7);
    assert_eq!(gc.root_statistics().slot_roots, 1);
    drop(current);
    gc.stop()?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn sparse_pages_are_released_and_stop_trims_only_empty_pages() -> Result<()> {
    let gc = collector();
    let root = Gc::try_new(&gc, 7_u64)?;
    let guards: Vec<_> = (0..1024)
        .map(|_| gc.create_root_guard(root.clone_gc()))
        .collect();
    assert_eq!(gc.root_statistics().slot_roots, 1025);
    assert!(gc.root_statistics().pages >= 17);
    drop(guards);
    assert_eq!(gc.root_statistics().pages, 1);
    assert_eq!(gc.root_statistics().cached_empty_pages, 0);
    gc.stop()?;
    assert_eq!(**root, 7);
    assert_eq!(gc.root_statistics().pages, 1);
    drop(root);
    assert_eq!(gc.root_statistics().cached_empty_pages, 1);
    gc.stop()?;
    let stats = gc.root_statistics();
    assert_eq!(stats.pages, 0);
    assert_eq!(stats.page_bytes, 0);
    assert_eq!(stats.resident_pages, 0);
    assert_eq!(stats.pages_created, stats.pages_released);
    Ok(())
}

#[test]
fn suspended_future_keeps_its_slot_until_cancellation() -> Result<()> {
    let gc = collector();
    let root = Gc::try_new(&gc, 7_u64)?.into_rooted();
    let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
    let mut future = Box::pin(async move {
        std::future::pending::<()>().await;
        assert_eq!(*root, 7);
    });
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    gc.collect_full()?;
    assert!(weak.upgrade().is_some());
    assert_eq!(gc.root_statistics().slot_roots, 1);
    drop(future);
    gc.stop()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.root_statistics().slot_roots, 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn suspended_roots_can_migrate_and_be_cancelled_on_another_thread() -> Result<()> {
    let gc = collector();
    let root = Gc::try_new(&gc, 7_u64)?.into_rooted();
    let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
    let mut future = Box::pin(async move {
        std::future::pending::<()>().await;
        assert_eq!(*root, 7);
    });
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    let other_gc = gc.clone();
    std::thread::spawn(move || -> Result<()> {
        other_gc.collect_full()?;
        assert_eq!(other_gc.root_statistics().slot_roots, 1);
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        drop(future);
        other_gc.collect_full()?;
        Ok(())
    })
    .join()
    .unwrap_or_else(|error| std::panic::resume_unwind(error))?;
    assert!(weak.upgrade().is_none());
    gc.stop()?;
    assert_eq!(gc.root_statistics().pages, 0);
    Ok(())
}
