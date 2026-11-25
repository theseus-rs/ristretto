//! Rooted projections cover strong edges without reading the locked payload.
#![expect(
    unsafe_code,
    reason = "fixtures expose only independently owning strong handles"
)]
#![expect(clippy::panic_in_result_fn, clippy::expect_used)]
#[path = "support/ownership.rs"]
mod support;
use ristretto_gc::cell::{MappedRwLockWriteGuard, RwLock, RwLockWriteGuard};
use ristretto_gc::{GarbageCollector, Gc, GcWeak, ReferenceKind, Result, Trace, TraceProgress};

fn project(owner: &RwLock<Vec<Gc<u64>>>) -> MappedRwLockWriteGuard<'_, [Gc<u64>]> {
    // SAFETY: preparation roots every current handle. Safe Gc construction,
    // cloning and extraction preserve ownership; the slice cannot access the
    // parent's demotion hook or add conditional/custom tracing actions.
    unsafe { RwLockWriteGuard::try_map_trace_rooted(owner.write(), |v| Some(v.as_mut_slice())) }
        .expect("slice projection")
}

fn finish(gc: &GarbageCollector) -> Result<()> {
    for _ in 0..20_000 {
        if gc.step() {
            return Ok(());
        }
    }
    Err(ristretto_gc::Error::CollectionPhaseError(
        "rooted projection blocked collection".into(),
    ))
}

#[test]
fn rooted_projection_keeps_current_children_and_reclaims_unrelated_objects() -> Result<()> {
    for gc in support::collectors() {
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let owner = Gc::try_new(&gc, RwLock::new(vec![child]))?;
        let guard = project(&owner);
        for _ in 0..3 {
            drop(Gc::try_new(&gc, 7_u64)?);
            gc.collect();
            finish(&gc)?;
            // Liveness is checked before reading through the borrowed handle.
            assert!(
                weak.upgrade().is_some(),
                "certified parent lost its rooted child"
            );
            assert_eq!(gc.statistics()?.live_objects, 2);
            assert_eq!(**guard.first().expect("child"), 42);
        }
        (**owner).trace(&gc);
        drop(guard);
        let ordinary = owner.write();
        assert!(matches!(
            (**owner).trace_step(&gc, 0, 8),
            TraceProgress::Busy
        ));
        drop(ordinary);
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        drop(owner);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
    }
    Ok(())
}

#[test]
fn nested_projection_replacements_and_escapes_keep_only_current_roots() -> Result<()> {
    for gc in support::collectors() {
        let old = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let old_weak = GcWeak::new(&old, ReferenceKind::Weak, 0);
        let owner = Gc::try_new(&gc, RwLock::new(vec![old]))?;
        let guard = project(&owner);
        let guard = MappedRwLockWriteGuard::try_map(guard, |_| None::<&mut Gc<u64>>)
            .expect_err("failed nested projection retains parent");
        let mut guard =
            MappedRwLockWriteGuard::map(guard, |values| values.first_mut().expect("child"));
        let replacement = Gc::try_new(&gc, 17_u64)?.into_rooted();
        let replacement_weak = GcWeak::new(&replacement, ReferenceKind::Weak, 0);
        let escaped = std::mem::replace(&mut *guard, replacement);
        gc.collect();
        finish(&gc)?;
        assert!(old_weak.upgrade().is_some());
        assert!(replacement_weak.upgrade().is_some());
        assert_eq!(*escaped, 42);
        drop(escaped);
        for _ in 0..3 {
            gc.collect();
            finish(&gc)?;
        }
        assert!(old_weak.upgrade().is_none(), "replaced child was retained");
        assert!(replacement_weak.upgrade().is_some());
        assert_eq!(**guard, 17);
        assert_eq!(gc.statistics()?.live_objects, 2);
        drop(guard);
        drop(owner);
        gc.stop()?;
        assert!(replacement_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn failed_rooted_projection_preserves_ownership_without_certification() -> Result<()> {
    for gc in support::collectors() {
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let owner = Gc::try_new(&gc, RwLock::new(vec![child]))?;
        let mut escaped = None;
        // SAFETY: this callback never succeeds; its mutation receives owning fields.
        let guard = unsafe {
            RwLockWriteGuard::try_map_trace_rooted(owner.write(), |values| {
                escaped = values.pop();
                None::<&mut Gc<u64>>
            })
        }
        .expect_err("failed mapping");
        assert!(matches!(
            (**owner).trace_step(&gc, 0, 8),
            TraceProgress::Busy
        ));
        drop(guard);
        drop(owner);
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(escaped);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn rooted_projection_unwind_revokes_before_demotion() -> Result<()> {
    for gc in support::collectors() {
        for inside_callback in [false, true] {
            let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
            let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
            let owner = Gc::try_new(&gc, RwLock::new(vec![child]))?;
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // SAFETY: a successful projection exposes only owning Gc fields.
                let _guard = unsafe {
                    RwLockWriteGuard::try_map_trace_rooted(owner.write(), |values| {
                        assert!(!inside_callback, "cancel callback");
                        Some(values.as_mut_slice())
                    })
                }
                .expect("slice projection");
                panic!("cancel guard");
            }));
            assert!(result.is_err());
            let ordinary = owner.write();
            assert!(matches!(
                (**owner).trace_step(&gc, 0, 8),
                TraceProgress::Busy
            ));
            drop(ordinary);
            gc.collect_full()?;
            assert!(weak.upgrade().is_some());
            drop(owner);
            gc.stop()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn stopped_rooted_writer_allows_unrelated_native_reclamation() -> Result<()> {
    for gc in support::collectors() {
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let owner = Gc::try_new(&gc, RwLock::new(vec![child]))?;
        std::thread::scope(|scope| -> Result<()> {
            let (ready_tx, ready_rx) = std::sync::mpsc::channel();
            let (resume_tx, resume_rx) = std::sync::mpsc::channel();
            let borrowed = &owner;
            let worker = scope.spawn(move || {
                let guard = project(borrowed);
                ready_tx.send(()).expect("collector waiting");
                let _ = resume_rx.recv();
                assert_eq!(**guard.first().expect("child"), 42);
            });
            ready_rx.recv().expect("roots and certificate published");
            for _ in 0..3 {
                drop(Gc::try_new(&gc, 17_u64)?);
                gc.collect();
                finish(&gc)?;
                assert!(weak.upgrade().is_some());
                assert_eq!(gc.statistics()?.live_objects, 2);
            }
            resume_tx.send(()).expect("release writer");
            worker.join().expect("writer finished");
            Ok(())
        })?;
        drop(owner);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.diagnostics().active_payload_guards, 0);
    }
    Ok(())
}
