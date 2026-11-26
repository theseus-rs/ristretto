use super::*;

fn fixture() -> (Catalog<()>, Owned<Cell<()>>) {
    let catalog = Catalog::with_record_capacities_and_limit(1, 8, 8, 8, 8);
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    for _ in 0..2 {
        assert!(creation.poll(&catalog).unwrap().is_none());
    }
    let cell = creation.poll(&catalog).unwrap().unwrap();
    (catalog, cell)
}

fn private_snapshot(catalog: &Catalog<()>) -> Snapshot<()> {
    Snapshot {
        owner: catalog.owner.clone(),
        head: catalog.head.load().unwrap(),
    }
}

#[test]
fn resumable_proposal_keeps_backing_without_publishing_effects() {
    let (catalog, cell) = fixture();
    let initial = catalog.controls.occupied();
    let mut preparation = catalog
        .begin_publish(private_snapshot(&catalog), share(&cell).unwrap())
        .unwrap();
    assert!(preparation.poll(&catalog).unwrap().is_none());
    assert_eq!(catalog.controls.occupied(), initial + 1);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
    let attempt = preparation.poll(&catalog).unwrap().unwrap();
    assert_eq!(catalog.controls.occupied(), initial + 2);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
    assert_eq!(catalog.head.load().unwrap().phase().domains[0].pending, 0);
    assert!(attempt.cancel());
    drop(attempt);
    drop(preparation);
    assert_eq!(catalog.controls.occupied(), initial);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
}

#[test]
fn partial_preparation_drop_and_foreign_poll_preserve_owners() {
    let (catalog, cell) = fixture();
    let (foreign, _) = fixture();
    let mut preparation = catalog
        .begin_publish(private_snapshot(&catalog), share(&cell).unwrap())
        .unwrap();
    assert!(matches!(
        preparation.poll(&foreign),
        Err(PreparationError::Foreign)
    ));
    assert!(preparation.poll(&catalog).unwrap().is_none());
    drop(preparation);
    assert_eq!(catalog.controls.occupied(), 1);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
}
