use super::*;

fn snapshot(catalog: &Catalog) -> Snapshot {
    (0..8)
        .find_map(|_| catalog.snapshot())
        .expect("phase fixture did not settle")
}
fn prepare(catalog: &Catalog, mut preparation: Preparation) -> Attempt {
    (0..catalog.capacity().max(256))
        .find_map(|_| preparation.poll(catalog).unwrap())
        .expect("phase preparation did not reach free backing")
}
fn next(catalog: &Catalog) -> Attempt {
    let snapshot = snapshot(catalog);
    let domain = snapshot.phase().domains[0];
    let preparation = if domain.mode == DomainMode::Inactive {
        catalog.begin_start(snapshot, 0, DomainMode::Full)
    } else if domain.floor == RETIREMENT_FLOOR {
        catalog.begin_finish_epoch(snapshot, 0, domain.epoch)
    } else {
        catalog.begin_advance(snapshot, 0)
    };
    prepare(catalog, preparation.unwrap())
}

#[test]
fn phase_only_requests_use_one_slot_without_retaining_a_predecessor_chain() {
    let catalog = Catalog::with_backing_capacity(1, 8, 8);
    for _ in 0..128 {
        let before = snapshot(&catalog).phase();
        assert_eq!(catalog.pool_occupancy(), (1, 0));
        let attempt = next(&catalog);
        assert_eq!(
            catalog.pool_occupancy(),
            (2, 0),
            "phase request reserved extra controllers"
        );
        assert_eq!(snapshot(&catalog).phase(), before);
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        assert_eq!(snapshot(&catalog).phase().revision, before.revision + 1);
        drop(attempt);
        assert_eq!(
            catalog.pool_occupancy(),
            (1, 0),
            "completed phase retained predecessor backing"
        );
    }
}

#[test]
fn stopped_phase_installation_is_helped_and_its_exact_result_survives_later_updates() {
    let catalog = Catalog::new();
    let attempt = next(&catalog);
    assert!(catalog.install_and_stop(&attempt));
    assert!(
        catalog.snapshot().is_none(),
        "undecided phase escaped as a snapshot"
    );
    let retained = snapshot(&catalog);
    let committed = retained.phase();
    assert_eq!(committed.domains[0].floor, 1);
    for _ in 0..16 {
        assert_eq!(catalog.commit(&next(&catalog)), Outcome::Committed);
    }
    let latest = snapshot(&catalog).phase();
    assert_eq!(retained.phase(), committed);
    catalog.late_help(&attempt);
    assert_eq!(
        catalog.commit(&attempt),
        Outcome::Committed,
        "lost completed phase result"
    );
    assert!(!attempt.cancel());
    assert_eq!(
        snapshot(&catalog).phase(),
        latest,
        "late phase helper rewound the head"
    );
    drop((attempt, retained));
    assert_eq!(catalog.pool_occupancy(), (1, 0));
}

#[test]
fn phase_cancellation_before_or_after_install_preserves_the_cancelled_outcome() {
    for installed in [false, true] {
        let catalog = Catalog::new();
        let before = snapshot(&catalog).phase();
        let attempt = next(&catalog);
        if installed {
            assert!(catalog.install_and_stop(&attempt));
        }
        assert!(attempt.cancel());
        assert!(attempt.cancel());
        assert_eq!(catalog.commit(&attempt), Outcome::Cancelled);
        let mut cancelled = before;
        cancelled.revision += 1;
        assert_eq!(
            snapshot(&catalog).phase(),
            cancelled,
            "cancelled phase applied proposed data"
        );
        assert_eq!(catalog.commit(&next(&catalog)), Outcome::Committed);
        let later = snapshot(&catalog).phase();
        catalog.late_help(&attempt);
        assert_eq!(catalog.commit(&attempt), Outcome::Cancelled);
        assert_eq!(snapshot(&catalog).phase(), later);
        drop(attempt);
        assert_eq!(catalog.pool_occupancy(), (1, 0));
    }
}

#[test]
fn phase_failed_owning_acquisitions_never_imply_installation_or_completion() {
    let catalog = Catalog::new();
    let before = snapshot(&catalog).phase();
    let attempt = next(&catalog);
    FAIL_SHARE.with(|counter| counter.set(Some(0)));
    assert_eq!(catalog.commit(&attempt), Outcome::Pending);
    assert_eq!(snapshot(&catalog).phase(), before);
    assert!(catalog.install_and_stop(&attempt));
    FAIL_LOAD.with(|counter| counter.set(Some(0)));
    assert!(catalog.snapshot().is_none());
    assert!(
        catalog.snapshot().is_none(),
        "failed observation certified the phase"
    );
    assert_eq!(snapshot(&catalog).phase().domains[0].floor, 1);
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
}

#[test]
fn phase_preparation_never_refreshes_its_expected_head_after_competition() {
    let catalog = Catalog::new();
    let mut older = catalog.begin_advance(snapshot(&catalog), 0).unwrap();
    assert_eq!(catalog.commit(&next(&catalog)), Outcome::Committed);
    let current = snapshot(&catalog).phase();
    let attempt = older.poll(&catalog).unwrap().unwrap();
    assert_eq!(catalog.commit(&attempt), Outcome::Retry);
    assert_eq!(
        snapshot(&catalog).phase(),
        current,
        "stale phase proposal replaced a newer head"
    );
    assert!(attempt.cancel());
    assert_eq!(catalog.commit(&attempt), Outcome::Cancelled);
    assert_eq!(snapshot(&catalog).phase(), current);
}

#[test]
fn phase_preparation_retries_full_backing_one_probe_at_a_time() {
    let catalog = Catalog::with_backing_capacity(1, 8, 8);
    let held = catalog.hold_control_slots(&(1..8).collect::<Vec<_>>());
    let mut preparation = catalog.begin_advance(snapshot(&catalog), 0).unwrap();
    for _ in 0..16 {
        assert!(preparation.poll(&catalog).unwrap().is_none());
        assert_eq!(catalog.pool_occupancy(), (8, 0));
        assert_eq!(snapshot(&catalog).phase().domains[0].floor, 0);
    }
    drop(held);
    let attempt = prepare(&catalog, preparation);
    assert_eq!(catalog.pool_occupancy(), (2, 0));
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    drop(attempt);
    assert_eq!(catalog.pool_occupancy(), (1, 0));
}
