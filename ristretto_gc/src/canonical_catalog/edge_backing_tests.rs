use super::*;
fn catalog() -> Catalog {
    Catalog::with_record_capacities_and_limit(2, 32, 32, 32, 32)
        .with_edge_capacities(4, 16, 16, 32)
        .with_source_edge_degree(2)
}
fn edge(catalog: &Catalog, slot: usize) -> EdgeToken {
    let mut creation = catalog.begin_create_edge(slot).unwrap();
    for _ in 0..4 {
        if let Some(edge) = creation.poll(catalog).unwrap() {
            return edge;
        }
    }
    panic!("finite edge creation did not finish")
}
#[test]
fn edge_identity_has_fixed_source_geometry_and_posts_only_removal_intent() {
    let catalog = catalog();
    assert_eq!(catalog.source_edge_range(0), Some(0..2));
    assert_eq!(catalog.source_edge_range(1), Some(2..4));
    assert_eq!(catalog.source_edge_range(2), None);
    assert!(!catalog.edge_slot_matches_source(3, 0));
    let token = edge(&catalog, 1);
    let held = token.try_share().unwrap();
    assert_eq!(held.membership(), Some(EdgeMembership::Prepared));
    assert!(!held.removal_requested());
    drop(token);
    assert!(held.removal_requested());
    assert_eq!(held.membership(), Some(EdgeMembership::Prepared));
    assert_eq!(
        catalog.discover_edge(1).unwrap().unwrap().identity,
        held.identity
    );
}
#[test]
fn private_edge_creation_cancellation_refunds_backing_and_rejects_foreign_poll() {
    for prefix in 0..3 {
        let catalog = catalog();
        let foreign = self::catalog();
        let mut creation = catalog.begin_create_edge(0).unwrap();
        assert!(matches!(
            creation.poll(&foreign),
            Err(CreationError::Foreign)
        ));
        for _ in 0..prefix {
            assert!(creation.poll(&catalog).unwrap().is_none());
        }
        drop(creation);
        let pools = catalog.edges.as_ref().unwrap();
        assert_eq!(
            (pools.slots_pool.occupied(), pools.states.occupied()),
            (0, 0)
        );
        assert!(catalog.discover_edge(0).unwrap().is_none());
    }
}
#[test]
fn failed_edge_discovery_is_contended_and_competing_creators_preserve_winner() {
    let catalog = catalog();
    crate::canonical_catalog::FAIL_LOAD.with(|fault| fault.set(Some(0)));
    assert!(matches!(
        catalog.begin_create_edge(0),
        Err(EdgePrepareError::Contended)
    ));
    let mut first = catalog.begin_create_edge(0).unwrap();
    let mut second = catalog.begin_create_edge(0).unwrap();
    for _ in 0..2 {
        assert!(first.poll(&catalog).unwrap().is_none());
        assert!(second.poll(&catalog).unwrap().is_none());
    }
    let winner = first.poll(&catalog).unwrap().unwrap();
    assert!(matches!(
        second.poll(&catalog),
        Err(CreationError::Occupied)
    ));
    assert_eq!(
        catalog.discover_edge(0).unwrap().unwrap().identity,
        winner.identity
    );
}
