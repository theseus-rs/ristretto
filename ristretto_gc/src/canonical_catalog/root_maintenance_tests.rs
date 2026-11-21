use super::super::Outcome;
use super::*;

fn commit(catalog: &Catalog, mut preparation: Preparation) {
    for _ in 0..4 {
        if let Some(attempt) = preparation.poll(catalog).unwrap() {
            assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
            return;
        }
    }
    panic!("finite preparation did not finish");
}
fn fixture(roots: usize, slot: usize) -> (Catalog, RootToken) {
    let catalog = Catalog::with_record_root_capacities_and_limit(1, 32, 32, 32, roots, 32, 32);
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    for _ in 0..2 {
        assert!(creation.poll(&catalog).unwrap().is_none());
    }
    let cell = creation.poll(&catalog).unwrap().unwrap();
    let mut creation = catalog.begin_create_root(slot, cell).unwrap();
    assert!(creation.poll(&catalog).unwrap().is_none());
    let token = creation.poll(&catalog).unwrap().unwrap();
    (catalog, token)
}
#[test]
fn scan_batches_stop_at_eight_slots_and_serialize_requested_removal() {
    let (catalog, token) = fixture(10, 8);
    drop(token);
    let preparation = catalog
        .begin_scan_root_batch(catalog.snapshot_in(&()).unwrap(), 0)
        .unwrap();
    commit(&catalog, preparation);
    assert_eq!(catalog.phase_in(&()).unwrap().root_cursors[0], 8);
    assert!(
        catalog
            .begin_advance(catalog.snapshot_in(&()).unwrap(), 0)
            .is_none()
    );
    let preparation = catalog
        .begin_scan_root_batch(catalog.snapshot_in(&()).unwrap(), 0)
        .unwrap();
    commit(&catalog, preparation);
    assert_eq!(catalog.phase_in(&()).unwrap().root_cursors[0], 9);
    let retired = catalog.discover_root(8).unwrap().unwrap();
    assert_eq!(retired.status(), RootStatus::Retired);
    assert!(catalog.release_retired_root(&retired));
    let preparation = catalog
        .begin_scan_root_batch(catalog.snapshot_in(&()).unwrap(), 0)
        .unwrap();
    commit(&catalog, preparation);
    assert_eq!(catalog.phase_in(&()).unwrap().root_cursors[0], 10);
    commit(
        &catalog,
        catalog
            .begin_advance(catalog.snapshot_in(&()).unwrap(), 0)
            .unwrap(),
    );
}
#[test]
fn failed_root_observation_cannot_advance_the_scan_cursor() {
    let (catalog, _token) = fixture(2, 1);
    let snapshot = catalog.snapshot_in(&()).unwrap();
    crate::canonical_catalog::FAIL_LOAD.with(|fault| fault.set(Some(0)));
    assert!(catalog.begin_scan_root_batch(snapshot, 0).is_none());
    assert_eq!(catalog.phase_in(&()).unwrap().root_cursors[0], 0);
}
#[test]
fn root_removal_preserves_target_work_until_retirement_is_authorized() {
    let (catalog, token) = fixture(1, 0);
    commit(
        &catalog,
        catalog
            .begin_publish_root(
                catalog.snapshot_in(&()).unwrap(),
                token.try_share().unwrap(),
            )
            .unwrap(),
    );
    assert!(
        catalog
            .begin_requested_root_removal(
                catalog.snapshot_in(&()).unwrap(),
                token.try_share().unwrap()
            )
            .is_none()
    );
    let root = token.try_share().unwrap();
    drop(token);
    commit(
        &catalog,
        catalog
            .begin_requested_root_removal(catalog.snapshot_in(&()).unwrap(), share(&root).unwrap())
            .unwrap(),
    );
    assert_eq!(root.status(), RootStatus::Retired);
    assert!(matches!(
        root.target_membership(),
        Some(Membership::Tracing(_))
    ));
    assert_eq!(catalog.phase_in(&()).unwrap().domains[0].pending, 1);
    assert!(catalog.release_retired_root(&root));
}
#[test]
fn activation_cannot_publish_a_prepared_target_or_a_foreign_root() {
    let (catalog, token) = fixture(1, 0);
    assert!(
        catalog
            .begin_activate_root(
                catalog.snapshot_in(&()).unwrap(),
                token.try_share().unwrap()
            )
            .is_none()
    );
    let (foreign, _) = fixture(1, 0);
    assert!(
        foreign
            .begin_activate_root(
                foreign.snapshot_in(&()).unwrap(),
                token.try_share().unwrap()
            )
            .is_none()
    );
    assert_eq!(token.status(), RootStatus::Prepared);
}
