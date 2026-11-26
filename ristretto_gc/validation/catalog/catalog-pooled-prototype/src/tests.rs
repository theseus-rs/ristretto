use super::*;

fn snapshot(catalog: &Catalog) -> Snapshot {
    for _ in 0..10 {
        if let Some(snapshot) = catalog.snapshot() {
            return snapshot;
        }
    }
    panic!("controller did not finish bounded work");
}
fn drain(catalog: &Catalog, cell: &Arc<Cell>) {
    for domain in 0..2 {
        let Membership::Live(work) = cell.membership() else {
            panic!("not live")
        };
        let attempt = catalog
            .acknowledge(snapshot(catalog), cell.clone(), domain, work[domain])
            .unwrap();
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    }
}

#[test]
fn stopped_install_and_effect_are_finished_before_phase_closure() {
    for after_effect in [false, true] {
        let catalog = Catalog::new();
        let cell = fixture_cell(&catalog, 8);
        let insert = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        if after_effect {
            catalog.effect_and_stop(&insert);
        } else {
            assert!(catalog.install_and_stop(&insert));
        }
        let phase = snapshot(&catalog).phase();
        assert_eq!(phase.domains.map(|domain| domain.pending), [1, 1]);
        assert!(catalog.advance(snapshot(&catalog), 0).is_none());
        drain(&catalog, &cell);
        for domain in 0..2 {
            assert_eq!(
                catalog.commit(&catalog.advance(snapshot(&catalog), domain).unwrap()),
                Outcome::Committed
            );
        }
        assert_eq!(
            catalog.commit(&catalog.retire(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        let replacement = fixture_cell(&catalog, 8);
        assert_eq!(
            catalog.commit(
                &catalog
                    .insert(snapshot(&catalog), replacement.clone())
                    .unwrap()
            ),
            Outcome::Committed
        );
        assert_eq!(catalog.commit(&insert), Outcome::Committed);
        catalog.late_help(&insert);
        assert_eq!(cell.membership(), Membership::Retired);
        assert!(matches!(replacement.membership(), Membership::Live(_)));
        assert_eq!(
            snapshot(&catalog)
                .phase()
                .domains
                .map(|domain| domain.pending),
            [1, 1]
        );
    }
}

#[test]
fn cancellation_is_shared_and_cannot_undo_a_committed_effect() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let insert = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    assert!(catalog.install_and_stop(&insert));
    assert!(insert.cancel());
    assert_eq!(catalog.commit(&insert), Outcome::Cancelled);
    assert_eq!(cell.membership(), Membership::Prepared);
    assert_eq!(
        snapshot(&catalog)
            .phase()
            .domains
            .map(|domain| domain.pending),
        [0, 0]
    );
    let retry = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    catalog.effect_and_stop(&retry);
    assert!(!retry.cancel());
    assert_eq!(catalog.commit(&retry), Outcome::Committed);
    assert_eq!(catalog.commit(&insert), Outcome::Cancelled);
    assert_eq!(
        snapshot(&catalog)
            .phase()
            .domains
            .map(|domain| domain.pending),
        [1, 1]
    );
}

#[test]
fn stale_preparation_never_changes_a_newer_floor_or_work_ticket() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let stale = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    assert_eq!(
        catalog.commit(&catalog.advance(snapshot(&catalog), 1).unwrap()),
        Outcome::Committed
    );
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    assert_eq!(cell.membership(), Membership::Prepared);
    assert_eq!(
        catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
        Outcome::Committed
    );
    let Membership::Live(work) = cell.membership() else {
        unreachable!()
    };
    let ack = catalog
        .acknowledge(snapshot(&catalog), cell.clone(), 0, work[0])
        .unwrap();
    assert_eq!(
        catalog.commit(
            &catalog
                .require(snapshot(&catalog), cell.clone(), 0)
                .unwrap()
        ),
        Outcome::Committed
    );
    assert_eq!(catalog.commit(&ack), Outcome::Retry);
    assert!(
        catalog
            .acknowledge(snapshot(&catalog), cell, 0, work[0])
            .is_none()
    );
}

#[test]
fn foreign_cells_and_snapshots_are_rejected_before_installation() {
    let catalog = Catalog::new();
    let foreign = Catalog::new();
    assert!(
        catalog
            .insert(snapshot(&catalog), fixture_cell(&foreign, 8))
            .is_none()
    );
    assert!(
        catalog
            .insert(snapshot(&foreign), fixture_cell(&catalog, 8))
            .is_none()
    );
    assert!(catalog.advance(snapshot(&foreign), 0).is_none());
    assert_eq!(snapshot(&catalog).phase().revision, 1);
}

#[test]
fn independent_cells_match_phase_accounting_through_repeated_retirement() {
    let catalog = Catalog::with_capacity(512);
    let mut cells = Vec::new();
    let count = if cfg!(miri) { 12 } else { 500 };
    for identity in 1..=count {
        let cell = fixture_cell(&catalog, identity as usize * 8);
        assert_eq!(
            catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        cells.push(cell);
    }
    for (index, cell) in cells.iter().enumerate() {
        drain(&catalog, cell);
        assert_eq!(
            catalog.commit(&catalog.retire(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        assert_eq!(
            snapshot(&catalog)
                .phase()
                .domains
                .map(|domain| domain.pending),
            [count as usize - 1 - index; 2]
        );
    }
    for _ in 0..12 {
        for domain in 0..2 {
            assert_eq!(
                catalog.commit(&catalog.advance(snapshot(&catalog), domain).unwrap()),
                Outcome::Committed
            );
        }
    }
    assert_eq!(
        snapshot(&catalog).phase().domains,
        [Domain {
            epoch: 4,
            floor: 0,
            pending: 0
        }; 2]
    );
}

#[test]
fn version_and_epoch_exhaustion_refuse_to_wrap() {
    for field in 0..3 {
        let mut phase = Phase {
            revision: 4,
            domains: [Domain {
                epoch: 7,
                floor: 3,
                pending: 0,
            }; 2],
        };
        if field == 0 {
            phase.revision = u64::MAX;
        } else {
            phase.domains[field - 1].epoch = u64::MAX;
        }
        let catalog = Catalog::with_phase(phase);
        let domain = if field == 0 { 0 } else { field - 1 };
        assert!(catalog.advance(snapshot(&catalog), domain).is_none());
        assert_eq!(snapshot(&catalog).phase(), phase);
        if field == 0 {
            assert!(
                catalog
                    .insert(snapshot(&catalog), fixture_cell(&catalog, 8))
                    .is_none()
            );
        }
    }
}

#[test]
#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn a_late_helper_cannot_replay_after_retirement_and_address_reuse() {
    use std::sync::mpsc;
    use std::time::Duration;
    for point in 0..4 {
        let catalog = Arc::new(Catalog::new());
        let cell = fixture_cell(&catalog, 8);
        let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        assert!(catalog.install_and_stop(&attempt));
        let (arrive, arrived) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let helper = {
            let catalog = catalog.clone();
            std::thread::spawn(move || {
                catalog.help_at(point, || {
                    arrive.send(()).unwrap();
                    resumed.recv_timeout(Duration::from_secs(10)).unwrap();
                });
            })
        };
        arrived.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        drain(&catalog, &cell);
        assert_eq!(
            catalog.commit(&catalog.retire(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        for _ in 2..=1000 {
            let next = fixture_cell(&catalog, 8);
            let lifetime = Arc::downgrade(&next);
            let slot = next.slot;
            assert_eq!(
                catalog.commit(&catalog.insert(snapshot(&catalog), next.clone()).unwrap()),
                Outcome::Committed
            );
            drain(&catalog, &next);
            assert_eq!(
                catalog.commit(&catalog.retire(snapshot(&catalog), next).unwrap()),
                Outcome::Committed
            );
            assert!(catalog.release_retired(&catalog.discover(slot).unwrap()));
            assert!(
                lifetime.upgrade().is_none(),
                "stopped helper retained later cells"
            );
        }
        let before = snapshot(&catalog).phase();
        // The stopped operation retains four old control versions and two old
        // membership values. It does not retain later completed operations.
        assert_eq!(catalog.pool_occupancy(), (5, 3));
        resume.send(()).unwrap();
        helper.join().unwrap();
        assert_eq!(cell.membership(), Membership::Retired);
        assert_eq!(snapshot(&catalog).phase(), before);
        assert_eq!(before.domains.map(|domain| domain.pending), [0, 0]);
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        drop(attempt);
        assert_eq!(catalog.pool_occupancy(), (1, 1));
        drop(cell);
        assert_eq!(catalog.pool_occupancy(), (1, 0));
    }
}

#[test]
#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn competing_publishers_preserve_exact_domain_counts() {
    use std::time::{Duration, Instant};
    let catalog = Arc::new(Catalog::new());
    let mut workers = Vec::new();
    for worker in 0..4 {
        let catalog = catalog.clone();
        workers.push(std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(20);
            let run = |prepare: &dyn Fn(Snapshot) -> Option<Attempt>| {
                loop {
                    assert!(Instant::now() < deadline, "contention did not converge");
                    let Some(snapshot) = catalog.snapshot() else {
                        continue;
                    };
                    let Some(attempt) = prepare(snapshot) else {
                        continue;
                    };
                    match catalog.commit(&attempt) {
                        Outcome::Committed => break,
                        Outcome::Retry => {}
                        Outcome::Cancelled => panic!("unexpected cancellation"),
                    }
                }
            };
            for index in 1..=200 {
                let identity = worker * 200 + index;
                let cell = fixture_cell(&catalog, identity as usize * 8);
                run(&|snapshot| catalog.insert(snapshot, cell.clone()));
                for domain in 0..2 {
                    let Membership::Live(work) = cell.membership() else {
                        panic!("publication lost")
                    };
                    run(&|snapshot| {
                        catalog.acknowledge(snapshot, cell.clone(), domain, work[domain])
                    });
                }
                run(&|snapshot| catalog.retire(snapshot, cell.clone()));
                assert_eq!(cell.membership(), Membership::Retired);
            }
        }));
    }
    for worker in workers {
        worker.join().unwrap();
    }
    let phase = snapshot(&catalog).phase();
    assert_eq!(phase.domains.map(|domain| domain.pending), [0, 0]);
    assert_eq!(phase.revision, 1 + 4 * 200 * 4);
}

fn fixture_cell(catalog: &Catalog, address: usize) -> Arc<Cell> {
    for slot in 0..catalog.capacity() {
        if let Some(old) = catalog.discover(slot) {
            catalog.release_retired(&old);
        }
        match catalog.prepare_cell(slot, address) {
            Ok(cell) => return cell,
            Err(
                PrepareError::Occupied | PrepareError::Contended | PrepareError::BackingUnavailable,
            ) => {}
            Err(error) => panic!("cell preparation failed: {error:?}"),
        }
    }
    panic!("fixture registry exhausted");
}

#[test]
fn registry_discovery_preserves_pending_work_without_the_publishers_stack() {
    let catalog = Catalog::with_capacity(4);
    for slot in 0..4 {
        drop(catalog.prepare_cell(slot, (slot + 1) * 8).unwrap());
    }
    for slot in 0..4 {
        let cell = catalog.discover(slot).unwrap();
        assert_eq!(cell.membership(), Membership::Prepared);
        assert_eq!(
            catalog.commit(&catalog.insert(snapshot(&catalog), cell).unwrap()),
            Outcome::Committed
        );
    }
    for domain in 0..2 {
        for slot in 0..4 {
            let cell = catalog.discover(slot).unwrap();
            let Membership::Live(work) = cell.membership() else {
                panic!("membership disappeared")
            };
            assert!(work[domain] > 0);
            assert_eq!(
                catalog.commit(
                    &catalog
                        .acknowledge(snapshot(&catalog), cell, domain, work[domain])
                        .unwrap()
                ),
                Outcome::Committed
            );
        }
        assert_eq!(
            catalog.commit(&catalog.advance(snapshot(&catalog), domain).unwrap()),
            Outcome::Committed
        );
    }
    for slot in 0..4 {
        let cell = catalog.discover(slot).unwrap();
        assert_eq!(
            catalog.commit(&catalog.retire(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        assert!(catalog.release_retired(&cell));
        assert!(catalog.discover(slot).is_none());
    }
}

#[test]
fn cancelled_membership_cannot_remove_or_publish_a_reused_slot() {
    let catalog = Catalog::with_capacity(1);
    let old = catalog.prepare_cell(0, 8).unwrap();
    let stale = catalog.insert(snapshot(&catalog), old.clone()).unwrap();
    assert!(matches!(
        catalog.prepare_cell(0, 16),
        Err(PrepareError::Occupied)
    ));
    assert!(!catalog.release_retired(&old));
    assert_eq!(
        catalog.commit(&catalog.abandon(snapshot(&catalog), old.clone()).unwrap()),
        Outcome::Committed
    );
    assert!(catalog.release_retired(&old));
    let current = catalog.prepare_cell(0, 8).unwrap();
    assert!(current.identity > old.identity);
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    assert!(!catalog.release_retired(&old));
    assert_eq!(old.membership(), Membership::Retired);
    assert!(Arc::ptr_eq(&catalog.discover(0).unwrap(), &current));
    assert_eq!(
        snapshot(&catalog)
            .phase()
            .domains
            .map(|domain| domain.pending),
        [0, 0]
    );
    assert_eq!(
        catalog.commit(
            &catalog
                .abandon(snapshot(&catalog), current.clone())
                .unwrap()
        ),
        Outcome::Committed
    );
    assert!(catalog.release_retired(&current));
    catalog.exhaust_identities();
    assert!(matches!(
        catalog.prepare_cell(0, 8),
        Err(PrepareError::IdentityExhausted)
    ));
    assert!(catalog.discover(0).is_none());
}

#[test]
#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn a_stopped_announcement_is_either_private_or_discoverable() {
    use std::sync::mpsc;
    use std::time::Duration;
    for after in [false, true] {
        let catalog = Arc::new(Catalog::with_capacity(2));
        let (arrive, arrived) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let publisher = {
            let catalog = catalog.clone();
            std::thread::spawn(move || {
                catalog
                    .prepare_and_stop(0, 8, after, || {
                        arrive.send(()).unwrap();
                        resumed.recv_timeout(Duration::from_secs(10)).unwrap();
                    })
                    .unwrap()
            })
        };
        arrived.recv_timeout(Duration::from_secs(10)).unwrap();
        if after {
            let cell = catalog.discover(0).unwrap();
            assert_eq!(
                catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
                Outcome::Committed
            );
            drain(&catalog, &cell);
        } else {
            assert!(catalog.discover(0).is_none());
        }
        for _ in 0..8 {
            for domain in 0..2 {
                assert_eq!(
                    catalog.commit(&catalog.advance(snapshot(&catalog), domain).unwrap()),
                    Outcome::Committed
                );
            }
        }
        let other = catalog.prepare_cell(1, 16).unwrap();
        assert_eq!(
            catalog.commit(&catalog.insert(snapshot(&catalog), other.clone()).unwrap()),
            Outcome::Committed
        );
        drain(&catalog, &other);
        assert_eq!(
            catalog.commit(&catalog.retire(snapshot(&catalog), other.clone()).unwrap()),
            Outcome::Committed
        );
        assert!(catalog.release_retired(&other));
        resume.send(()).unwrap();
        let original = publisher.join().unwrap();
        assert!(Arc::ptr_eq(&catalog.discover(0).unwrap(), &original));
        if !after {
            assert_eq!(
                catalog.commit(
                    &catalog
                        .insert(snapshot(&catalog), original.clone())
                        .unwrap()
                ),
                Outcome::Committed
            );
            drain(&catalog, &original);
        }
        assert_eq!(original.membership(), Membership::Live([0, 0]));
    }
}

#[test]
fn failed_private_preparation_returns_its_backing_without_publishing() {
    // Deliberately too small for one full operation plus the current head.
    // This checks failure ownership, not a valid progress-reserve configuration.
    let catalog = Catalog::with_backing_capacity(1, 2, 8);
    let cell = catalog.prepare_cell(0, 8).unwrap();
    for _ in 0..20 {
        assert!(catalog.insert(snapshot(&catalog), cell.clone()).is_none());
        assert_eq!(catalog.pool_occupancy(), (1, 1));
        assert_eq!(snapshot(&catalog).phase().revision, 1);
        assert_eq!(cell.membership(), Membership::Prepared);
    }
}

#[test]
fn cancelled_private_attempts_return_all_prepared_control_and_state_slots() {
    let catalog = Catalog::with_backing_capacity(1, 8, 8);
    let cell = catalog.prepare_cell(0, 8).unwrap();
    for _ in 0..50 {
        let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        assert_eq!(catalog.pool_occupancy(), (4, 2));
        assert!(attempt.cancel());
        drop(attempt);
        assert_eq!(catalog.pool_occupancy(), (1, 1));
        assert_eq!(cell.membership(), Membership::Prepared);
    }
    assert_eq!(
        catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
        Outcome::Committed
    );
    assert_eq!(catalog.pool_occupancy(), (1, 1));
}
