use super::*;

fn fixture() -> (Catalog<()>, Owned<Cell<()>>) {
    let catalog = Catalog::with_record_root_capacities_and_limit(1, 16, 16, 16, 1, 8, 16);
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    for _ in 0..2 {
        assert!(creation.poll(&catalog).unwrap().is_none());
    }
    let cell = creation.poll(&catalog).unwrap().unwrap();
    (catalog, cell)
}

#[test]
fn root_token_keeps_exact_target_and_posts_only_removal_intent() {
    let (catalog, cell) = fixture();
    let mut creation = catalog.begin_create_root(0, share(&cell).unwrap()).unwrap();
    assert!(creation.poll(&catalog).unwrap().is_none());
    let token = creation.poll(&catalog).unwrap().unwrap();
    assert_eq!(token.status(), RootStatus::Prepared);
    assert_eq!(token.target_identity(), cell.identity);
    assert!(!token.removal_requested());
    assert!(matches!(
        catalog.begin_create_root(0, share(&cell).unwrap()),
        Err(RootPrepareError::Occupied)
    ));
    drop(token);
    let observed = catalog.discover_root(0).unwrap().unwrap();
    assert!(observed.removal_requested());
    assert_eq!(observed.status(), RootStatus::Prepared);
    assert!(!catalog.release_retired_root(&observed));
    assert_eq!(observed.target_identity(), cell.identity);
}

#[test]
fn private_root_cancellation_returns_backing_and_foreign_targets_are_rejected() {
    let (catalog, cell) = fixture();
    let (foreign, other) = fixture();
    assert!(matches!(
        catalog.begin_create_root(0, other),
        Err(RootPrepareError::Foreign)
    ));
    let mut creation = catalog.begin_create_root(0, share(&cell).unwrap()).unwrap();
    assert!(matches!(
        creation.poll(&foreign),
        Err(CreationError::Foreign)
    ));
    assert!(creation.poll(&catalog).unwrap().is_none());
    assert_eq!(catalog.roots.as_ref().unwrap().pool.occupied(), 1);
    drop(creation);
    assert_eq!(catalog.roots.as_ref().unwrap().pool.occupied(), 0);
    assert!(catalog.discover_root(0).unwrap().is_none());
}
