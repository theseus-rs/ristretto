use super::super::{Arc, Budget, Capacities, Footprint};
use super::*;
fn fixture() -> (CatalogHandle, Participant, Arc<Budget>) {
    let budget = Budget::new(usize::MAX);
    let capacities = Capacities::derive(2, 1, 1).unwrap().with_edges(2).unwrap();
    let catalog =
        BoundedCatalog::with_indexed_records_and_storage_in(capacities, &budget, 0, 1).unwrap();
    let worker = catalog.register_mutator().unwrap();
    (catalog, worker, budget)
}
fn finish(worker: &mut Participant) -> EdgeHandle {
    for _ in 0..8 {
        if let Progress::EdgeCreated(_) = worker.poll() {
            return worker.take_edge().unwrap();
        }
    }
    panic!("finite creation");
}
#[test]
fn edge_creation_owner_retains_quota_and_credit_through_final_drop() {
    let (catalog, mut worker, budget) = fixture();
    worker.submit_edge_creation(0).unwrap();
    let first = finish(&mut worker);
    worker.submit_edge_creation(1).unwrap();
    let second = finish(&mut worker);
    assert_eq!(catalog.reserved_edge_tokens(), 2);
    assert_eq!(worker.submit_edge_creation(0), Err(RequestError::EdgeLimit));
    assert!(worker.take_edge().is_none());
    drop((worker, catalog, first));
    assert!(budget.used() > 0);
    drop(second);
    assert_eq!(budget.used(), 0);
}
#[test]
fn cancelling_each_edge_creation_prefix_refunds_quota_and_posts_cleanup() {
    for prefix in 0..=4 {
        let (catalog, mut worker, budget) = fixture();
        worker.submit_edge_creation(0).unwrap();
        for _ in 0..prefix {
            let _ = worker.poll();
        }
        assert!(worker.cancel());
        assert_eq!(catalog.reserved_edge_tokens(), 0);
        if let Some(observed) = worker.discover_edge(0).unwrap() {
            assert!(observed.removal_requested);
        }
        drop((catalog, worker));
        assert_eq!(budget.used(), 0);
    }
}
#[test]
fn edge_backing_and_token_storage_are_charged_before_setup() {
    let capacities = Capacities::derive(2, 1, 1).unwrap().with_edges(2).unwrap();
    let footprint = Footprint::for_capacities(capacities).unwrap();
    assert_eq!(footprint.edge_tokens(), 2 * size_of::<EdgeHandle>());
    let too_small = Budget::new(footprint.total() - 1);
    assert!(BoundedCatalog::new_in(capacities, &too_small).is_err());
    assert_eq!(too_small.used(), 0);
    let exact = Budget::new(footprint.total());
    let catalog = BoundedCatalog::new_in(capacities, &exact).unwrap();
    assert_eq!(exact.used(), footprint.total());
    drop(catalog);
    assert_eq!(exact.used(), 0);
}
#[test]
fn invalid_edge_geometry_cannot_reserve_credit() {
    let caps = Capacities::derive(2, 1, 1).unwrap().with_edges(3).unwrap();
    let budget = Budget::new(usize::MAX);
    assert!(
        BoundedCatalog::<()>::with_indexed_records_and_storage_in(caps, &budget, 0, 1).is_err()
    );
    assert!(
        BoundedCatalog::<()>::with_indexed_records_and_storage_in(caps, &budget, 0, 0).is_err()
    );
    assert_eq!(budget.used(), 0);
}
