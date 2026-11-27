//! Escaping clones own their graphs; guard clones share one registration.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::cell::RwLock;
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, RootStorage,
};
use std::sync::Arc;

fn collectors() -> impl Iterator<Item = Arc<GarbageCollector>> {
    [false, true].into_iter().flat_map(|concurrent| {
        [false, true].into_iter().flat_map(move |independent| {
            [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts]
                .into_iter()
                .map(move |storage| {
                    GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .independent_marking(independent)
                            .root_storage(storage)
                            .diagnostics(true)
                            .allocation_threshold(usize::MAX)
                            .build(),
                    )
                })
        })
    })
}

#[test]
fn cloned_read_edge_owns_its_graph_after_source_reclamation() -> Result<()> {
    for gc in collectors() {
        let context = gc.register_context();
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let source = Gc::try_new(&gc, RwLock::new(Some(child)))?;
        let before = gc.diagnostics().root_registrations;
        let cloned = context.with(|| source.read().as_ref().expect("child").clone());
        assert_eq!(gc.diagnostics().root_registrations, before + 1);
        drop((source, context));
        gc.collect_young();
        while !gc.step() {}
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert!(
            weak.upgrade().is_some(),
            "cloning a borrowed heap edge lost its graph"
        );
        assert_eq!(*cloned, 42);
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(cloned);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn cloned_guard_and_handles_share_one_registration_and_keep_the_heap_alive() -> Result<()> {
    for gc in collectors() {
        let context = gc.register_context();
        let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
        let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
        let guard = root.clone();
        let handle = root.clone_gc();
        let other = handle.clone();
        assert_eq!(gc.diagnostics().root_registrations, 1);
        drop((root, guard, handle, context));
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert!(weak.upgrade().is_some());
        let heap = Arc::downgrade(&gc);
        drop(gc);
        assert!(heap.upgrade().is_some());
        assert_eq!(*other, 42);
        drop(other);
        assert!(heap.upgrade().is_none());
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

#[test]
fn clones_of_replaced_guards_protect_the_replacement_without_registering_it_again() -> Result<()> {
    for gc in collectors() {
        let mut original = Gc::try_new(&gc, 1_u64)?;
        let replacement = Gc::try_new(&gc, 42_u64)?;
        *original = replacement.clone_gc();
        let guard = original.clone();
        let cloned = original.clone_gc();
        assert_eq!(gc.diagnostics().root_registrations, 2);
        drop((original, replacement));
        gc.collect_full()?;
        assert_eq!(**guard, 42);
        assert_eq!(*cloned, 42);
        drop(guard);
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(cloned);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn moving_a_guards_inner_handle_preserves_its_registration() -> Result<()> {
    for gc in collectors() {
        let mut original = Gc::try_new(&gc, 42_u64)?;
        let replacement = Gc::try_new(&gc, 7_u64)?;
        let weak = GcWeak::new(&original, ReferenceKind::Weak, 0);
        let moved = std::mem::replace(&mut *original, replacement.clone_gc());
        assert_eq!(gc.diagnostics().root_registrations, 2);
        drop((original, replacement));
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert!(
            weak.upgrade().is_some(),
            "moving a guard's inner handle lost its graph"
        );
        assert_eq!(*moved, 42);
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(moved);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn a_suspended_clone_keeps_descendants_until_cancellation() -> Result<()> {
    use std::future::Future;
    use std::task::{Context, Waker};

    for gc in collectors() {
        let leaf = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&leaf, ReferenceKind::Weak, 0);
        let child = Gc::try_new(&gc, vec![leaf])?.into_rooted();
        let parent = Gc::try_new(&gc, RwLock::new(Some(child)))?;
        let cloned = parent.read().as_ref().expect("child").clone();
        let mut suspended = Box::pin(async move {
            std::future::pending::<()>().await;
            assert_eq!(cloned.len(), 1);
        });
        assert!(
            suspended
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        drop(parent);
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert!(weak.upgrade().is_some());
        assert_eq!(gc.statistics()?.live_objects, 2);
        drop(suspended);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn borrowed_edge_clones_survive_concurrent_collection_and_cross_thread_drop() -> Result<()> {
    for gc in collectors() {
        gc.start();
        let mut workers = Vec::new();
        for _ in 0..2 {
            let gc = gc.clone();
            workers.push(std::thread::spawn(move || -> Result<()> {
                let context = gc.register_context();
                for _ in 0..32 {
                    let leaf = Gc::try_new(&gc, 42_u64)?.into_rooted();
                    let weak = GcWeak::new(&leaf, ReferenceKind::Weak, 0);
                    let parent = Gc::try_new(&gc, RwLock::new(Some(leaf)))?;
                    let cloned = context.with(|| parent.read().as_ref().expect("leaf").clone());
                    drop(parent);
                    gc.collect_full()?;
                    assert!(weak.upgrade().is_some());
                    assert_eq!(*cloned, 42);
                    std::thread::spawn(move || drop(cloned))
                        .join()
                        .unwrap_or_else(|error| std::panic::resume_unwind(error));
                }
                Ok(())
            }));
        }
        for worker in workers {
            worker
                .join()
                .unwrap_or_else(|error| std::panic::resume_unwind(error))?;
        }
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn cloning_an_expired_edge_rejects_it_without_leaking_a_reserved_root() -> Result<()> {
    for gc in collectors() {
        let context = gc.register_context();
        let root = Gc::try_new(&gc, 42_u64)?;
        let mut stale = root.clone_gc();
        // An identity-only stale-edge fixture. No payload access after demotion.
        // SAFETY: The root still protects this identity; after retirement only checked cloning is attempted.
        #[expect(unsafe_code, reason = "explicit non-owning edge fixture")]
        unsafe {
            stale.unroot();
        };
        drop(root);
        gc.collect_full()?;
        let before = gc.diagnostics().root_registrations;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            context.with(|| stale.clone())
        }));
        assert!(result.is_err());
        assert!(matches!(
            context.with(|| stale.as_root(&gc)),
            Err(ristretto_gc::Error::InvalidPointer(_))
        ));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gc.add_root(&stale))).is_err()
        );
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gc.create_root_guard(stale)))
                .is_err()
        );
        assert_eq!(gc.diagnostics().root_registrations, before);
        assert_eq!(gc.root_statistics().reserved_slots, 0);
        assert_eq!(gc.root_statistics().slot_roots, 0);
        assert_eq!(gc.root_statistics().compatibility_roots, 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
