use super::*;

#[test]
fn reserved_backing_starts_empty_and_releases_all_pool_owners() {
    let catalog = Catalog::<()>::with_record_capacities_and_limit(3, 8, 8, 8, 7);
    assert_eq!(catalog.capacity(), 3);
    assert_eq!(catalog.controls.occupied(), 1);
    assert_eq!(catalog.memberships.occupied(), 0);
    assert_eq!(catalog.cells.occupied(), 0);
    for slot in 0..3 {
        assert!(catalog.discover(slot).unwrap().is_none());
    }
    let controls = Arc::downgrade(&catalog.controls);
    let memberships = Arc::downgrade(&catalog.memberships);
    let cells = Arc::downgrade(&catalog.cells);
    drop(catalog);
    assert!(controls.upgrade().is_none());
    assert!(memberships.upgrade().is_none());
    assert!(cells.upgrade().is_none());
}

#[test]
fn backing_accounting_rejects_invalid_or_overflowing_capacities() {
    assert!(Catalog::<()>::record_backing_bytes(0, 8, 8, 8).is_none());
    assert!(Catalog::<()>::record_backing_bytes(3, 7, 8, 8).is_none());
    assert!(Catalog::<()>::record_backing_bytes(usize::MAX, 8, 8, 8).is_none());
    let bytes = Catalog::<()>::record_backing_bytes(3, 8, 8, 8).unwrap();
    assert!(bytes > 0);
    assert!(Catalog::<()>::record_backing_bytes(3, 8, 8, 16).unwrap() > bytes);
}
