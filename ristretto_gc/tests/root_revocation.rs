//! Safe numeric root removal cannot revoke owning registrations.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, RootStorage,
};

#[test]
fn numeric_removal_cannot_invalidate_owning_handles() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let root = context.with(|| Gc::try_new(&gc, 42_u64))?;
            let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
            // A safe caller can submit arbitrary IDs. These include the fresh
            // heap's owning tree registration and are not removal capabilities.
            for id in 0..128 {
                gc.remove_root_by_id(id);
            }
            gc.collect_full()?;
            assert!(
                weak.upgrade().is_some(),
                "numeric removal revoked an owning root"
            );
            assert_eq!(**root, 42);
            drop((root, context));
            gc.stop()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn numeric_removal_still_releases_manual_roots() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = Gc::try_new(&gc, 42_u64)?;
        let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
        let manual = gc.add_root(&root);
        drop(root);
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        gc.remove_root_by_id(manual);
        gc.remove_root_by_id(manual);
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.root_statistics().compatibility_roots, 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
