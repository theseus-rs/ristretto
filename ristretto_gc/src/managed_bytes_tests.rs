#![expect(
    clippy::panic_in_result_fn,
    reason = "finite heap accounting assertions"
)]
use crate::{GarbageCollector, Gc};

#[test]
fn managed_bytes_matches_accounting_through_real_reclamation() -> crate::Result<()> {
    let gc = GarbageCollector::new();
    assert_eq!(gc.managed_bytes(), 0);
    let first = Gc::new(&gc, 11_u64);
    let second = Gc::new(&gc, 22_u64);
    let both = gc.managed_bytes();
    assert!(both > 0);
    assert_eq!(both, gc.statistics()?.bytes_managed);
    drop(first);
    gc.collect_full()?;
    assert!(gc.managed_bytes() > 0 && gc.managed_bytes() < both);
    assert_eq!(gc.managed_bytes(), gc.statistics()?.bytes_managed);
    drop(second);
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn published_logical_bytes_debt_and_private_charge_are_distinct() -> crate::Result<()> {
    let gc = GarbageCollector::new();
    let private = gc.reserve_exact_bytes(31)?;
    let snapshot = gc.statistics()?;
    assert_eq!(snapshot.bytes_managed, 31);
    assert_eq!(snapshot.bytes_live, 0);
    assert_eq!(snapshot.bytes_allocated, 0);
    assert_eq!(snapshot.allocation_debt, 0);
    drop(private);
    assert_eq!(gc.managed_bytes(), 0);

    let rooted = Gc::try_new(&gc, 7_u64)?;
    let physical = Gc::allocation_size(&7_u64)?;
    let snapshot = gc.statistics()?;
    assert_eq!(snapshot.bytes_live, size_of::<u64>());
    assert_eq!(snapshot.bytes_allocated, size_of::<u64>());
    assert_eq!(snapshot.bytes_managed, physical);
    assert_eq!(snapshot.allocation_debt, physical);

    gc.collect_full()?;
    let snapshot = gc.statistics()?;
    assert_eq!(snapshot.bytes_live, size_of::<u64>());
    assert_eq!(snapshot.bytes_allocated, 0);
    assert_eq!(snapshot.bytes_freed, 0);
    assert_eq!(snapshot.bytes_managed, physical);
    drop(rooted);
    gc.collect_full()?;
    let snapshot = gc.statistics()?;
    assert_eq!(snapshot.bytes_live, 0);
    assert_eq!(snapshot.bytes_managed, 0);
    assert_eq!(snapshot.bytes_freed, size_of::<u64>());
    assert_eq!(snapshot.managed_bytes_freed, physical);
    Ok(())
}
