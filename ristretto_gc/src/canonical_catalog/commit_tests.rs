use super::*;

fn fixture() -> (Catalog<()>, Owned<Cell<()>>, Attempt<()>) {
    let catalog = Catalog::with_record_capacities_and_limit(2, 32, 32, 32, 32);
    let mut creation = catalog.begin_create_with(0, 8, ()).unwrap();
    for _ in 0..2 {
        assert!(creation.poll(&catalog).unwrap().is_none());
    }
    let cell = creation.poll(&catalog).unwrap().unwrap();
    let snapshot = catalog.snapshot_in(&()).unwrap();
    let mut preparation = catalog
        .begin_publish(snapshot, share(&cell).unwrap())
        .unwrap();
    assert!(preparation.poll(&catalog).unwrap().is_none());
    let attempt = preparation.poll(&catalog).unwrap().unwrap();
    (catalog, cell, attempt)
}

fn completed_phase(catalog: &Catalog<()>) -> Phase {
    for _ in 0..4 {
        if let Some(snapshot) = catalog.snapshot_in(&()) {
            return snapshot.phase();
        }
    }
    panic!("quiescent helper did not finish");
}

#[test]
fn another_helper_finishes_a_stopped_installer_once() {
    let (catalog, cell, attempt) = fixture();
    assert!(catalog.install_and_stop(&attempt));
    assert!(catalog.snapshot_in(&()).is_none());
    let phase = completed_phase(&catalog);
    assert_eq!(phase.domains.map(|domain| domain.pending), [1, 1]);
    assert!(matches!(cell.membership(), Some(Membership::Tracing(_))));
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
    assert_eq!(completed_phase(&catalog), phase);
}

#[test]
fn applied_effect_is_certified_without_fallible_membership_reread() {
    let (catalog, cell, attempt) = fixture();
    catalog.effect_and_stop(&attempt);
    assert!(matches!(cell.membership(), Some(Membership::Tracing(_))));
    let phase = completed_phase(&catalog);
    assert_eq!(phase.domains.map(|domain| domain.pending), [1, 1]);
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
}

#[test]
fn failed_controller_acquisition_keeps_the_installed_attempt_pending() {
    let (catalog, _, attempt) = fixture();
    assert!(catalog.install_and_stop(&attempt));
    super::super::FAIL_SHARE.with(|remaining| remaining.set(Some(0)));
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Pending);
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
    assert_eq!(completed_phase(&catalog).domains[0].pending, 1);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_helpers_publish_one_cell_effect_and_phase_certificate() {
    let (catalog, cell, attempt) = fixture();
    assert!(catalog.install_and_stop(&attempt));
    let catalog = Arc::new(catalog);
    let attempt = Arc::new(attempt);
    let other_catalog = catalog.clone();
    let other_attempt = attempt.clone();
    let worker = std::thread::spawn(move || other_catalog.commit_in(&other_attempt, &()));
    let _ = catalog.commit_in(&attempt, &());
    let _ = worker.join().unwrap();
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
    assert_eq!(
        completed_phase(&catalog)
            .domains
            .map(|domain| domain.pending),
        [1, 1]
    );
    assert!(matches!(cell.membership(), Some(Membership::Tracing(_))));
}
