//! Task ownership and buffered publication use the same protocol on native and wasm.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, RootStorage,
};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Waker};

fn collector(records: usize) -> Arc<GarbageCollector> {
    collector_with_publication(records, false)
}

fn collector_with_publication(records: usize, concurrent: bool) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(concurrent)
            .root_storage(RootStorage::Contexts)
            .barrier_buffer_size(records)
            .diagnostics(true)
            .metadata_shards(16)
            .independent_marking(true)
            .work_stealing(true)
            .mark_batch_size(8)
            .threads(4)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    )
}

#[test]
fn contexts_are_opt_in_and_buffers_are_bounded() {
    assert!(!GarbageCollector::new().execution_contexts_enabled());
    assert_eq!(Configuration::default().barrier_buffer_size, 0);
    assert_eq!(
        Configuration::builder()
            .barrier_buffer_size(usize::MAX)
            .build()
            .barrier_buffer_size,
        64
    );
    assert!(collector(usize::MAX).execution_contexts_enabled());
}

#[test]
fn nested_scopes_restore_the_context_and_unscoped_roots_stay_visible() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(4, concurrent);
        let outer = gc.register_context();
        let inner = gc.register_context();
        let (a, b, c) = outer.with(|| -> Result<_> {
            let a = Gc::try_new(&gc, 1_u64)?;
            let b = inner.with(|| Gc::try_new(&gc, 2_u64))?;
            let c = Gc::try_new(&gc, 3_u64)?;
            Ok((a, b, c))
        })?;
        let unscoped = Gc::try_new(&gc, 4_u64)?;
        assert_eq!(outer.root_statistics().slot_roots, 2);
        assert_eq!(inner.root_statistics().slot_roots, 1);
        assert_eq!(gc.root_statistics().slot_roots, 4);
        gc.collect_full()?;
        assert_eq!((**a, **b, **c, **unscoped), (1, 2, 3, 4));
        drop((a, b, c, unscoped, outer, inner));
        gc.stop()?;
        assert_eq!(gc.diagnostics().execution_contexts, 0);
        assert_eq!(gc.root_statistics().pages, 0);
    }
    Ok(())
}

#[test]
fn a_foreign_heap_cannot_use_the_current_context() -> Result<()> {
    for concurrent in [false, true] {
        let first = collector_with_publication(1, concurrent);
        let second = collector_with_publication(1, concurrent);
        let context = first.register_context();
        let root = context.with(|| Gc::try_new(&second, 7_u64))?;
        assert_eq!(context.root_statistics().slot_roots, 0);
        second.collect_full()?;
        assert_eq!(**root, 7);
        drop(root);
        second.stop()?;
    }
    Ok(())
}

#[test]
fn escaped_roots_keep_context_slots_after_detach_and_stop() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(4, concurrent);
        let context = gc.register_context();
        let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
        let clone = root.clone();
        let escaped = root.into_rooted();
        drop(context);
        assert_eq!(gc.diagnostics().execution_contexts, 1);
        assert_eq!(gc.root_statistics().slot_roots, 1);
        gc.stop()?;
        drop(clone);
        assert_eq!(*escaped, 42);
        drop(escaped);
        gc.stop()?;
        assert_eq!(gc.diagnostics().execution_contexts, 0);
        assert_eq!(gc.root_statistics().pages, 0);
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn detached_buffers_transfer_to_collection_and_preserve_the_snapshot() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(4, concurrent);
        let context = gc.register_context();
        let root = context.with(|| Gc::try_new(&gc, 7_u64))?;
        let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
        gc.collect();
        assert!(!gc.step()); // start the snapshot without enumerating this root
        context.with(|| drop(root));
        // Registration deletion publishes the snapshot record; the matching
        // inner guard edge no longer publishes a duplicate for coalescing.
        assert_eq!(gc.diagnostics().pending_barrier_records, 1);
        assert_eq!(gc.diagnostics().barrier_coalesced, 0);
        drop(context);
        assert_eq!(gc.diagnostics().execution_contexts, 1); // buffer owns the context
        while !gc.step() {}
        assert!(weak.upgrade().is_some());
        assert_eq!(gc.diagnostics().pending_barrier_records, 0);
        assert_eq!(gc.diagnostics().execution_contexts, 0);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

#[test]
fn overflow_is_lossless_and_collector_drains_one_record_per_work_unit() -> Result<()> {
    for concurrent in [false, true] {
        for limit in [1, 2, 64, usize::MAX] {
            let gc = collector_with_publication(limit, concurrent);
            let context = gc.register_context();
            let roots = context.with(|| {
                (0..130_u64)
                    .map(|value| Gc::try_new(&gc, value))
                    .collect::<Result<Vec<_>>>()
            })?;
            gc.collect();
            assert!(!gc.step());
            context.with(|| drop(roots));
            let before = gc.diagnostics();
            assert!(before.barrier_buffered >= 130);
            assert!(before.barrier_overflows > 0);
            assert!(before.pending_barrier_records <= limit.min(64));
            let recorded = before.barrier_buffered;
            loop {
                let before = gc.diagnostics().barrier_flushed;
                let done = gc.step();
                assert!(gc.diagnostics().barrier_flushed <= before + 1);
                if done {
                    break;
                }
            }
            assert_eq!(gc.statistics()?.live_objects, 130);
            let after = gc.diagnostics();
            assert_eq!(after.barrier_flushed, recorded);
            assert_eq!(after.pending_barrier_records, 0);
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert_eq!(context.root_statistics().pages, 0);
        }
    }
    Ok(())
}

#[test]
fn a_suspended_future_keeps_context_roots_without_leaking_tls() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(4, concurrent);
        let context = gc.register_context();
        let mut future = Box::pin(context.scope(async {
            let root = Gc::try_new(&gc, 9_u64)?;
            std::future::pending::<()>().await;
            assert_eq!(**root, 9);
            Ok::<_, ristretto_gc::Error>(())
        }));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        let unscoped = Gc::try_new(&gc, 10_u64)?;
        assert_eq!(context.root_statistics().slot_roots, 1);
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 2);
        drop((future, unscoped));
        gc.stop()?;
        assert_eq!(context.root_statistics().slot_roots, 0);
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_future_resumes_in_its_context_on_another_thread_and_cancels_there() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(2, concurrent);
        let context = gc.register_context();
        let task_context = context.clone();
        let task_gc = gc.clone();
        let mut future = Box::pin(async move {
            task_context
                .scope(async move {
                    let first = Gc::try_new(&task_gc, 1_u64)?;
                    let mut suspended = false;
                    std::future::poll_fn(|cx| {
                        if std::mem::replace(&mut suspended, true) {
                            std::task::Poll::Ready(())
                        } else {
                            cx.waker().wake_by_ref();
                            std::task::Poll::Pending
                        }
                    })
                    .await;
                    let second = Gc::try_new(&task_gc, 2_u64)?;
                    std::future::pending::<()>().await;
                    assert_eq!((**first, **second), (1, 2));
                    Ok::<_, ristretto_gc::Error>(())
                })
                .await
        });
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert_eq!(context.root_statistics().slot_roots, 1);
        let other_gc = gc.clone();
        let observer = context.clone();
        std::thread::spawn(move || -> Result<()> {
            assert!(
                future
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            assert_eq!(observer.root_statistics().slot_roots, 2);
            other_gc.collect_full()?;
            assert_eq!(other_gc.statistics()?.live_objects, 2);
            drop(future);
            other_gc.collect_full()?;
            Ok(())
        })
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error))?;
        gc.stop()?;
        assert_eq!(context.root_statistics().pages, 0);
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn unwinding_a_scope_restores_tls_and_flush_can_be_explicit() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(4, concurrent);
        let context = gc.register_context();
        let root = Gc::try_new(&gc, 1_u64)?;
        gc.collect();
        gc.step();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            context.with(|| {
                gc.write_barrier(&root);
                panic!("test unwind");
            })
        }));
        assert!(result.is_err());
        assert_eq!(gc.diagnostics().pending_barrier_records, 1);
        context.flush();
        assert_eq!(gc.diagnostics().pending_barrier_records, 0);
        let unscoped = Gc::try_new(&gc, 2_u64)?;
        assert_eq!(context.root_statistics().slot_roots, 0);
        drop((root, unscoped));
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn repeated_edges_coalesce_without_losing_snapshot_reachability() -> Result<()> {
    for concurrent in [false, true] {
        let gc = collector_with_publication(1, concurrent);
        let context = gc.register_context();
        let root = Gc::try_new(&gc, 42_u64)?;
        let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
        gc.collect();
        assert!(!gc.step());
        context.with(|| {
            for _ in 0..1000 {
                gc.write_barrier(&root);
            }
            drop(root);
        });
        let d = gc.diagnostics();
        assert_eq!(d.pending_barrier_records, 1);
        assert_eq!(d.barrier_buffered, 1);
        assert!(d.barrier_coalesced >= 999);
        assert_eq!(d.barrier_overflows, 0);
        while !gc.step() {}
        assert!(weak.upgrade().is_some());
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}
