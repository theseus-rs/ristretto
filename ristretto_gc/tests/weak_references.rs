//! Conditional reachability and pending-root transfer, including cooperative execution.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{GarbageCollector, Gc, GcWeak, ReferenceKind, Result};

#[test]
fn an_unreachable_reference_owner_is_never_enqueued() -> Result<()> {
    for storage in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ] {
        for kind in [
            ReferenceKind::Soft,
            ReferenceKind::Weak,
            ReferenceKind::Phantom,
        ] {
            let gc = GarbageCollector::with_config(
                ristretto_gc::Configuration::builder()
                    .metadata_shards(16)
                    .work_stealing(true)
                    .independent_marking(true)
                    .mark_batch_size(8)
                    .root_storage(storage)
                    .build(),
            );
            let target = Gc::new(&gc, 42_u64);
            let owner = Gc::new(&gc, 7_u64);
            let edge = GcWeak::new(&target, kind, owner.as_ptr() as usize);
            drop((target, owner));
            gc.collect_full()?;
            assert!(edge.upgrade().is_none());
            assert!(!gc.has_pending_references());
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[test]
fn weak_upgrade_roots_the_referent_and_clearing_is_permanent() -> Result<()> {
    for storage in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .root_storage(storage)
                .build(),
        );
        let value = Gc::new(&gc, 42_u64).into_rooted();
        let weak = GcWeak::new(&value, ReferenceKind::Weak, 0);
        let upgraded = weak.upgrade();
        drop(value);
        gc.collect_full()?;
        assert_eq!(upgraded.as_deref(), Some(&42));
        assert!(weak.upgrade().is_some());
        drop(upgraded);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn weak_clearing_precedes_finalization_and_phantoms_follow_it() -> Result<()> {
    for storage in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .root_storage(storage)
                .build(),
        );
        let value = Gc::new(&gc, 42_u64).into_rooted();
        let final_reference = Gc::new(&gc, 1_u64).into_rooted();
        let phantom_reference = Gc::new(&gc, 2_u64).into_rooted();
        let weak = GcWeak::new(&value, ReferenceKind::Weak, 0);
        let final_edge = GcWeak::new(
            &value,
            ReferenceKind::Final,
            final_reference.as_ptr() as usize,
        );
        let phantom = GcWeak::new(
            &value,
            ReferenceKind::Phantom,
            phantom_reference.as_ptr() as usize,
        );
        drop(value);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert!(final_edge.upgrade().is_some());
        assert!(phantom.upgrade().is_some());
        let pending = gc.take_pending_reference::<u64>()?;
        assert_eq!(pending.as_deref(), Some(&1));
        assert!(!gc.has_pending_references());
        gc.collect_full()?;
        assert!(final_edge.upgrade().is_some());
        assert!(!gc.has_pending_references());
        final_edge.clear();
        gc.collect_full()?;
        assert!(phantom.upgrade().is_none());
        assert_eq!(gc.take_pending_reference::<u64>()?.as_deref(), Some(&2));
    }
    Ok(())
}

#[test]
fn a_pending_reference_keeps_its_owner_alive_until_transfer() -> Result<()> {
    for storage in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .root_storage(storage)
                .build(),
        );
        let target = Gc::new(&gc, 42_u64).into_rooted();
        let reference = Gc::new(&gc, 7_u64).into_rooted();
        let weak = GcWeak::new(&target, ReferenceKind::Weak, reference.as_ptr() as usize);
        drop(target);
        gc.collect_full()?;
        drop(reference);
        gc.collect_full()?;
        let reference = gc.take_pending_reference::<u64>()?;
        assert_eq!(reference.as_deref(), Some(&7));
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

#[test]
fn minor_collections_skip_promoted_weak_referents() -> Result<()> {
    for storage in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .root_storage(storage)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        let targets: Vec<_> = (0..1000).map(|i| Gc::new(&gc, i).into_rooted()).collect();
        let weak: Vec<_> = targets
            .iter()
            .map(|target| GcWeak::new(target, ReferenceKind::Weak, 0))
            .collect();
        gc.collect_full()?;
        gc.collect_full()?;
        gc.collect_young();
        while !gc.step() {}
        let examined = gc.statistics()?.references_examined;
        for _ in 0..5 {
            gc.collect_young();
            while !gc.step() {}
        }
        assert_eq!(gc.statistics()?.references_examined, examined);
        assert!(weak.iter().all(|edge| edge.upgrade().is_some()));
        drop(targets);
        gc.collect_full()?;
        assert!(weak.iter().all(|edge| edge.upgrade().is_none()));
    }
    Ok(())
}

#[test]
fn weak_clearing_includes_reference_objects_reachable_only_through_finalization() -> Result<()> {
    for storage in [
        ristretto_gc::RootStorage::Tree,
        ristretto_gc::RootStorage::Slots,
    ] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .metadata_shards(16)
                .work_stealing(true)
                .independent_marking(true)
                .mark_batch_size(8)
                .root_storage(storage)
                .build(),
        );
        let target = Gc::new(&gc, 42_u64).into_rooted();
        let owner = Gc::new(&gc, 7_u64).into_rooted();
        let weak = GcWeak::new(&target, ReferenceKind::Weak, owner.as_ptr() as usize);
        // Both the referent and the Reference subclass have pending finalization. The
        // Reference itself is not strongly reachable at the weak-clearing boundary.
        let final_target = GcWeak::new(&target, ReferenceKind::Final, 0);
        let final_owner = GcWeak::new(&owner, ReferenceKind::Final, 0);
        drop((target, owner));
        gc.collect_full()?;
        assert!(final_target.upgrade().is_some());
        assert!(final_owner.upgrade().is_some());
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.take_pending_reference::<u64>()?.as_deref(), Some(&7));
    }
    Ok(())
}
