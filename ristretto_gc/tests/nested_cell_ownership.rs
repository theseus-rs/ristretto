//! Managed ownership follows nested cells through attachment, movement and unwind.
#![expect(clippy::panic_in_result_fn)]
#[path = "support/ownership.rs"]
mod ownership;
use ownership::collectors;
use ristretto_gc::cell::RwLock;
use ristretto_gc::{GarbageCollector, Gc, GcWeak, HeapBuffer, ReferenceKind, Result, Trace};
use std::collections::HashMap;
use std::sync::Arc;

type Cell = RwLock<Option<Gc<u64>>>;

#[expect(clippy::expect_used, reason = "assert the fixture contains its child")]
fn check_container<T: Trace + Send + Sync + 'static>(
    gc: &Arc<GarbageCollector>,
    build: impl FnOnce(Cell) -> T,
    cell: impl FnOnce(&T) -> &Cell,
) -> Result<()> {
    let child = Gc::try_new(gc, 42_u64)?.into_rooted();
    let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
    let parent = Gc::try_new(gc, build(RwLock::new(Some(child))))?;
    let moved = cell(&parent).write().take().expect("child");
    drop(parent);
    for _ in 0..3 {
        gc.collect_full()?;
    }
    assert!(
        weak.upgrade().is_some(),
        "nested cell lost the moved child's root"
    );
    assert_eq!(*moved, 42);
    drop(moved);
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn nested_container_cells_root_values_before_mutable_escape() -> Result<()> {
    for gc in collectors() {
        let context = gc.register_context();
        context.with(|| -> Result<()> {
            check_container(&gc, Some, |v| v.as_ref().expect("cell"))?;
            check_container(&gc, |c| vec![c], |v| v.first().expect("cell"))?;
            check_container(&gc, |c| (c, 7_u64), |v| &v.0)?;
            check_container(&gc, |c| (7_u64, c, 9_u64), |v| &v.1)?;
            check_container(&gc, |c| HashMap::from([(7_u64, c)]), |v| &v[&7])?;
            check_container(
                &gc,
                |c| HeapBuffer::from(vec![c]),
                |v| v.first().expect("cell"),
            )?;
            Ok(())
        })?;
        drop(context);
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn detached_cells_do_not_demote_host_values_after_the_parent_dies() -> Result<()> {
    for gc in collectors() {
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&gc, RwLock::new(RwLock::new(Some(child))))?;
        let moved = std::mem::replace(&mut *parent.write(), RwLock::new(None));
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        drop(moved.write());
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert!(
            weak.upgrade().is_some(),
            "detached cell demoted its host-owned child"
        );
        assert_eq!(**moved.read().as_ref().expect("child"), 42);
        drop(moved);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn rebound_cells_restore_mutation_hooks_for_the_new_parent() -> Result<()> {
    for gc in collectors() {
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let old = Gc::try_new(&gc, RwLock::new(RwLock::new(Some(child))))?;
        let moved = std::mem::replace(&mut *old.write(), RwLock::new(None));
        let new = Gc::try_new(&gc, RwLock::new(vec![moved]))?;
        drop(old);
        // Repeated whole-parent mutable borrows detach and rebind the inner cell.
        for _ in 0..3 {
            new.write().reserve(0);
            gc.collect_full()?;
        }
        let extracted = new
            .read()
            .first()
            .expect("cell")
            .write()
            .take()
            .expect("child");
        drop(new);
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(*extracted, 42);
        drop(extracted);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn nested_cell_cycles_are_reclaimed_after_their_roots_drop() -> Result<()> {
    struct Node {
        edge: Vec<RwLock<Option<Gc<Node>>>>,
    }
    // SAFETY: every strong field and nested cell receives paired ownership hooks.
    #[expect(
        unsafe_code,
        reason = "recursive fixture implements the Trace ownership contract"
    )]
    unsafe impl Trace for Node {
        fn trace(&self, gc: &GarbageCollector) {
            self.edge.trace(gc);
        }
        fn root(&mut self) {
            self.edge.root();
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            // SAFETY: edge is a field of the supplied protected owner.
            unsafe { self.edge.unroot(owner) };
        }
        unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            // SAFETY: construction protection covers every nested cell.
            unsafe { self.edge.attach(owner) };
        }
    }
    for gc in collectors() {
        let a = Gc::try_new(
            &gc,
            Node {
                edge: vec![RwLock::new(None)],
            },
        )?;
        let b = Gc::try_new(
            &gc,
            Node {
                edge: vec![RwLock::new(None)],
            },
        )?;
        let a_weak = GcWeak::new(&a, ReferenceKind::Weak, 0);
        let b_weak = GcWeak::new(&b, ReferenceKind::Weak, 0);
        *a.edge.first().expect("cell").write() = Some(b.clone_gc());
        *b.edge.first().expect("cell").write() = Some(a.clone_gc());
        drop(b);
        gc.collect_full()?;
        assert!(b_weak.upgrade().is_some());
        drop(a);
        gc.stop()?;
        assert!(a_weak.upgrade().is_none());
        assert!(b_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn partial_root_unwind_rebinds_nested_cells_before_releasing_the_writer() -> Result<()> {
    struct Panics {
        cell: Cell,
        once: bool,
    }
    // SAFETY: the cell is traced and always rebound on guard cleanup, including unwind.
    #[expect(
        unsafe_code,
        reason = "fixture tests partial rooting under the Trace contract"
    )]
    unsafe impl Trace for Panics {
        fn trace(&self, gc: &GarbageCollector) {
            self.cell.trace(gc);
        }
        fn root(&mut self) {
            self.cell.root();
            assert!(
                !std::mem::take(&mut self.once),
                "injected partial root failure"
            );
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            // SAFETY: the guard retains exclusive access and the owning graph.
            unsafe { self.cell.unroot(owner) };
        }
    }
    for gc in collectors() {
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::try_new(
            &gc,
            RwLock::new(Panics {
                cell: RwLock::new(Some(child)),
                once: true,
            }),
        )?;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = &mut *parent.write();
        }));
        assert!(result.is_err());
        assert!(!ristretto_gc::cell::has_guards());
        let moved = parent.read().cell.write().take().expect("child");
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(*moved, 42);
        drop(moved);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
