use super::super::{Arc, Budget, Capacities, ConfigError, Progress};
use super::*;

fn setup() -> (CatalogHandle, Participant, Arc<Budget>, RecordKey) {
    let capacities = Capacities::derive(1, 1, 1).unwrap().with_roots(1).unwrap();
    let budget = Budget::new(usize::MAX);
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    let mut worker = catalog.register_mutator().unwrap();
    worker.submit_record(0, 8, ()).unwrap();
    let mut key = None;
    for _ in 0..8 {
        if let Progress::Created(created) = worker.poll() {
            key = Some(created);
            break;
        }
    }
    (catalog, worker, budget, key.unwrap())
}
fn root(worker: &mut Participant, target: RecordKey) -> RootHandle {
    worker.submit_root_creation(0, target).unwrap();
    for _ in 0..8 {
        if let Progress::RootCreated(_) = worker.poll() {
            return worker.take_root().unwrap();
        }
    }
    panic!("finite root creation did not finish");
}

#[test]
fn root_quota_retains_charge_after_participants_and_catalog_are_dropped() {
    let (catalog, mut worker, budget, key) = setup();
    let root = root(&mut worker, key);
    assert_eq!(catalog.reserved_root_tokens(), 1);
    assert_eq!(
        worker.submit_root_creation(0, key),
        Err(RequestError::RootLimit)
    );
    assert!(root.borrow_live_record().is_none());
    worker
        .submit_root(RootCommand::Publish(root.key()))
        .unwrap();
    for _ in 0..8 {
        if worker.poll() == Progress::Committed {
            break;
        }
    }
    assert!(root.borrow_live_record().is_some());
    assert_eq!(root.stably_live_in(&()), Some(true));
    drop((worker, catalog));
    assert!(budget.used() > 0);
    drop(root);
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancellation_and_drop_refund_root_quota_after_token_custody() {
    let (catalog, mut worker, budget, key) = setup();
    let root = root(&mut worker, key);
    let root_key = root.key();
    worker.submit_root(RootCommand::Cancel(root_key)).unwrap();
    for _ in 0..8 {
        if worker.poll() == Progress::Committed {
            break;
        }
    }
    assert!(!worker.release_retired_root(root_key).unwrap());
    drop(root);
    assert_eq!(catalog.reserved_root_tokens(), 0);
    assert!(worker.release_retired_root(root_key).unwrap());
    let replacement = self::root(&mut worker, key);
    assert_ne!(replacement.key(), root_key);
    drop((replacement, worker, catalog));
    assert_eq!(budget.used(), 0);
}

#[test]
fn unsupported_reference_or_edge_capacities_cannot_get_an_undercharged_catalog() {
    let base = Capacities::derive(1, 1, 1).unwrap();
    let budget = Budget::new(usize::MAX);
    assert!(matches!(
        BoundedCatalog::new_in(base.with_references(1).unwrap(), &budget),
        Err(ConfigError::UnsupportedReferences)
    ));
    assert!(matches!(
        BoundedCatalog::new_in(base.with_edges(1).unwrap(), &budget),
        Err(ConfigError::InvalidEdgeLayout)
    ));
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancelling_root_creation_at_each_boundary_returns_token_quota() {
    for polls in 0..=3 {
        let (catalog, mut worker, budget, key) = setup();
        worker.submit_root_creation(0, key).unwrap();
        assert_eq!(catalog.reserved_root_tokens(), 1);
        for _ in 0..polls {
            let _ = worker.poll();
        }
        drop(worker);
        assert_eq!(catalog.reserved_root_tokens(), 0);
        let replacement = catalog.register_mutator().unwrap();
        drop((replacement, catalog));
        assert_eq!(budget.used(), 0);
    }
}
