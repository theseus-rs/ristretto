//! Foreign strong edges retain their source heap through attachment and rebinding.
#![expect(clippy::panic_in_result_fn)]
#[path = "support/ownership.rs"]
mod ownership;
use ownership::collectors;
use ristretto_gc::cell::RwLock;
use ristretto_gc::{Gc, GcWeak, ReferenceKind, Result};
use std::sync::Arc;

#[test]
fn a_foreign_child_remains_live_while_its_parent_is_rooted() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let context = source.register_context();
        let child = context.with(|| Gc::try_new(&source, 42_u64))?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&destination, RwLock::new(Some(child)))?;
        for _ in 0..3 {
            source.collect_full()?;
            destination.collect_full()?;
        }
        assert!(
            weak.upgrade().is_some(),
            "foreign edge lost the source heap root"
        );
        assert_eq!(**parent.read().as_ref().expect("child"), 42);
        let source_heap = Arc::downgrade(&source);
        drop((context, source));
        assert!(
            source_heap.upgrade().is_some(),
            "foreign edge lost its source collector"
        );
        assert_eq!(**parent.read().as_ref().expect("child"), 42);
        drop(parent);
        destination.stop()?;
        assert!(source_heap.upgrade().is_none());
        assert!(weak.upgrade().is_none());
        assert_eq!(destination.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn replacement_foreign_edges_survive_writes_and_release_with_the_owner() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let parent = Gc::try_new(&destination, RwLock::new(None))?;
        let child = Gc::try_new(&source, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        *parent.write() = Some(child);
        // An untouched writer must also preserve the foreign root.
        drop(parent.write());
        source.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(**parent.read().as_ref().expect("child"), 42);
        *parent.write() = None;
        source.collect_full()?;
        assert!(weak.upgrade().is_none());
        drop(parent);
        destination.stop()?;
        source.stop()?;
        assert_eq!(destination.managed_bytes(), 0);
        assert_eq!(source.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn a_moved_nested_cell_rebinds_to_its_destination_heap() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let child = Gc::try_new(&source, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let old = Gc::try_new(&source, RwLock::new(RwLock::new(Some(child))))?;
        let moved = std::mem::replace(&mut *old.write(), RwLock::new(None));
        let new = Gc::try_new(&destination, (moved, 7_u64))?;
        drop(old);
        drop(new.0.write());
        for _ in 0..3 {
            source.collect_full()?;
            destination.collect_full()?;
        }
        assert!(
            weak.upgrade().is_some(),
            "rebinding demoted a foreign strong edge"
        );
        let extracted = new.0.write().take().expect("child");
        drop(new);
        destination.stop()?;
        source.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(*extracted, 42);
        drop(extracted);
        source.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(source.managed_bytes(), 0);
        assert_eq!(destination.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn foreign_roots_survive_two_collectors_running_during_mutation() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let child = Gc::try_new(&source, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let parent = Gc::try_new(&destination, RwLock::new(Some(child)))?;
        let start = Arc::new(std::sync::Barrier::new(3));
        let cycles = if cfg!(miri) { 4 } else { 32 };
        let workers: Vec<_> = [source.clone(), destination.clone()]
            .into_iter()
            .map(|gc| {
                let start = start.clone();
                std::thread::spawn(move || -> Result<()> {
                    start.wait();
                    for _ in 0..cycles {
                        gc.collect_full()?;
                    }
                    Ok(())
                })
            })
            .collect();
        start.wait();
        for _ in 0..cycles {
            let child = parent.write().take().expect("child");
            assert!(weak.upgrade().is_some());
            assert_eq!(*child, 42);
            *parent.write() = Some(child);
        }
        for worker in workers {
            worker
                .join()
                .unwrap_or_else(|error| std::panic::resume_unwind(error))?;
        }
        drop(parent);
        destination.stop()?;
        source.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(source.managed_bytes(), 0);
        assert_eq!(destination.managed_bytes(), 0);
    }
    Ok(())
}
