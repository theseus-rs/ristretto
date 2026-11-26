use super::*;

#[test]
fn a_stopped_preferred_request_cannot_hold_a_vacancy_or_any_other_slot() {
    let p = crate::vacancy::Preference::default();
    assert!(p.nominate(17));
    p.offer(0);
    assert_eq!(p.slot_for(17), Some(0));
    assert!(!p.defer(0, 17));
    for _ in 1..crate::vacancy::DEFERRALS {
        assert!(!p.defer(1, 23));
        assert!(p.defer(0, 23));
    }
    assert!(!p.defer(0, 23));
    assert!(p.slot_for(17).is_none());
    assert!(p.nominate(42));
    p.offer(1);
    p.finish(17);
    assert_eq!(p.slot_for(42), Some(1));
    p.finish(42);
    assert!(!p.defer(1, 23));
}

#[test]
fn a_parked_or_cancelled_nominee_leaves_the_only_slot_reusable() {
    for cancel in [false, true] {
        let catalog = Catalog::with_backing_capacity(1, 128, 128);
        let first = catalog.prepare_cell(0, 8).unwrap();
        let mut waiting = catalog.prepare_private_creation(0, 16, (), false).unwrap();
        let mut peer = catalog.prepare_private_creation(0, 24, (), false).unwrap();
        for _ in 0..crate::vacancy::PROBE_INTERVAL + 2 {
            assert!(waiting.poll_rotating(&catalog).unwrap().is_none());
        }
        for _ in 0..2 {
            assert!(peer.poll_rotating(&catalog).unwrap().is_none());
        }
        let abandon = catalog.abandon(snapshot(&catalog), first.clone()).unwrap();
        assert_eq!(catalog.commit(&abandon), Outcome::Committed);
        assert!(catalog.release_retired(&first));
        // The released slot is vacant. A competing prepared cell yields once
        // to the active nominee, whose own thread is deliberately parked.
        assert!(peer.poll_rotating(&catalog).unwrap().is_none());
        let mut waiting = Some(waiting);
        let limit = if cancel {
            drop(waiting.take());
            1
        } else {
            crate::vacancy::PROBE_INTERVAL
        };
        let installed = (0..limit)
            .find_map(|_| peer.poll_rotating(&catalog).unwrap())
            .expect("peer must publish without the nominee running");
        assert_eq!(installed.slot(), 0);
        assert!(Pooled::ptr_eq(&discover(&catalog, 0).unwrap(), &installed));
        drop(waiting);
        assert!(Pooled::ptr_eq(&discover(&catalog, 0).unwrap(), &installed));
    }
}

#[test]
fn rotating_announcement_uses_released_slots_and_recovers_from_stale_hints() {
    for reuse_before_probe in [false, true] {
        let catalog = Catalog::with_backing_capacity(5, 128, 128);
        let mut pending = catalog.begin_create_with(0, 8, ()).unwrap();
        let _first = catalog.prepare_cell(0, 16).unwrap();
        let _second = catalog.prepare_cell(1, 24).unwrap();
        let old = catalog.prepare_cell(4, 32).unwrap();
        let abandon = catalog.abandon(snapshot(&catalog), old.clone()).unwrap();
        assert_eq!(catalog.commit(&abandon), Outcome::Committed);
        assert!(catalog.release_retired(&old));
        let replacement = reuse_before_probe.then(|| catalog.prepare_cell(4, 40).unwrap());

        let cell = (0..10)
            .find_map(|_| pending.poll_rotating(&catalog).unwrap())
            .expect("vacancy hint or sequential probe must reach a free slot");
        assert_eq!(cell.slot(), if reuse_before_probe { 2 } else { 4 });
        if let Some(replacement) = replacement {
            assert!(Pooled::ptr_eq(
                &discover(&catalog, 4).unwrap(),
                &replacement
            ));
        }
    }
}

#[test]
fn sparse_free_control_slots_do_not_livelock_private_preparation() {
    let catalog = Catalog::with_backing_capacity(1, 32, 8);
    // Six free slots can hold the three-record proposal, but each pair is
    // followed by more occupied slots than one bounded search can cross.
    let free = [1, 2, 11, 12, 21, 22];
    let held = catalog.hold_control_slots(
        &(1..32)
            .filter(|index| !free.contains(index))
            .collect::<Vec<_>>(),
    );
    assert_eq!(catalog.pool_occupancy().0, 26);
    let cell = fixture_cell(&catalog, 8);
    let mut preparation = catalog
        .begin_insert(snapshot(&catalog), cell.clone())
        .unwrap();
    let mut prepared = None;
    for _ in 0..128 {
        if let Some(attempt) = preparation.poll(&catalog).unwrap() {
            prepared = Some(attempt);
            break;
        }
    }
    let attempt = prepared.expect("private preparation repeatedly discarded usable slots");
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert!(matches!(membership(&cell), Membership::Live(_)));
    drop(held);
}

#[test]
fn preparation_reaches_two_isolated_slots_one_probe_per_poll() {
    let catalog = Catalog::with_backing_capacity(1, 32, 8);
    let free = [1, 31];
    let held = catalog.hold_control_slots(
        &(1..32)
            .filter(|index| !free.contains(index))
            .collect::<Vec<_>>(),
    );
    let cell = fixture_cell(&catalog, 8);
    let mut preparation = catalog
        .begin_insert(snapshot(&catalog), cell.clone())
        .unwrap();
    let mut ready = None;
    let mut previous = 30;
    for poll in 1..=32 {
        ready = preparation.poll(&catalog).unwrap();
        let occupied = catalog.pool_occupancy().0;
        assert!(occupied >= previous && occupied <= previous + 1);
        previous = occupied;
        if ready.is_some() {
            assert_eq!(poll, 31);
            break;
        }
    }
    let attempt = ready.expect("two free slots were not reached in one pass");
    assert_eq!(snapshot(&catalog).phase().domains[0].floor, 0);
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    drop(held);
}

#[test]
fn partial_preparation_cancellation_returns_every_private_slot() {
    for polls in 0..2 {
        let catalog = Catalog::with_backing_capacity(1, 32, 8);
        let cell = fixture_cell(&catalog, 8);
        let mut preparation = catalog
            .begin_insert(snapshot(&catalog), cell.clone())
            .unwrap();
        for _ in 0..polls {
            assert!(preparation.poll(&catalog).unwrap().is_none());
        }
        assert_eq!(catalog.pool_occupancy(), (1 + polls, 2));
        drop(preparation);
        assert_eq!(catalog.pool_occupancy(), (1, 1));
        assert_eq!(membership(&cell), Membership::Prepared);
        assert_eq!(snapshot(&catalog).phase().revision, 1);
    }
}

#[test]
fn failed_final_record_allocation_retains_both_prepared_outcomes() {
    let catalog = Catalog::with_backing_capacity(1, 32, 8);
    let held = catalog.hold_control_slots(&(2..32).collect::<Vec<_>>());
    let cell = fixture_cell(&catalog, 8);
    let mut preparation = catalog
        .begin_insert(snapshot(&catalog), cell.clone())
        .unwrap();
    assert!(preparation.poll(&catalog).unwrap().is_none());
    assert!(preparation.poll(&catalog).unwrap().is_none());
    assert_eq!(catalog.pool_occupancy(), (32, 2));
    for _ in 0..64 {
        assert!(preparation.poll(&catalog).unwrap().is_none());
        assert_eq!(catalog.pool_occupancy(), (32, 2));
    }
    drop(preparation);
    assert_eq!(catalog.pool_occupancy(), (31, 1));
    assert_eq!(membership(&cell), Membership::Prepared);
    drop(held);
    assert_eq!(catalog.pool_occupancy(), (1, 1));
}

#[test]
fn foreign_or_finished_preparation_polls_do_not_consume_another_result() {
    let catalog = Catalog::new();
    let foreign = Catalog::new();
    let mut preparation = catalog.begin_advance(snapshot(&catalog), 0).unwrap();
    assert!(matches!(
        preparation.poll(&foreign),
        Err(PreparationError::Foreign)
    ));
    assert_eq!(catalog.pool_occupancy(), (1, 0));
    let attempt = preparation.poll(&catalog).unwrap().unwrap();
    assert!(matches!(
        preparation.poll(&catalog),
        Err(PreparationError::Finished)
    ));
    drop(preparation);
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert_eq!(snapshot(&catalog).phase().domains[0].floor, 1);
    assert_eq!(snapshot(&foreign).phase().domains[0].floor, 0);
}

#[test]
fn a_partially_prepared_operation_cannot_refresh_its_expected_phase() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let mut preparation = catalog
        .begin_insert(snapshot(&catalog), cell.clone())
        .unwrap();
    assert!(preparation.poll(&catalog).unwrap().is_none());
    for _ in 0..64 {
        assert_eq!(
            catalog.commit(&advance_cycle(&catalog, 0).unwrap()),
            Outcome::Committed
        );
    }
    let phase = snapshot(&catalog).phase();
    let attempt = (0..768)
        .find_map(|_| preparation.poll(&catalog).unwrap())
        .expect("private preparation did not resume after phase changes");
    assert_eq!(catalog.commit(&attempt), Outcome::Retry);
    assert_eq!(snapshot(&catalog).phase(), phase);
    drop(attempt);
    drop(preparation);
    assert_eq!(catalog.pool_occupancy(), (1, 1));
    assert_eq!(membership(&cell), Membership::Prepared);
}

#[test]
fn blocked_final_record_finishes_after_backing_is_released() {
    let catalog = Catalog::with_backing_capacity(1, 32, 8);
    let held = catalog.hold_control_slots(&(2..32).collect::<Vec<_>>());
    let cell = fixture_cell(&catalog, 8);
    let mut preparation = catalog
        .begin_insert(snapshot(&catalog), cell.clone())
        .unwrap();
    for _ in 0..3 {
        assert!(preparation.poll(&catalog).unwrap().is_none());
    }
    assert_eq!(catalog.pool_occupancy(), (32, 2));
    drop(held);
    let attempt = preparation.poll(&catalog).unwrap().unwrap();
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [1, 1]
    );
    drop(attempt);
    assert_eq!(catalog.pool_occupancy(), (1, 1));
}

#[test]
fn discovery_acquisition_failure_is_not_an_empty_slot() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
    assert!(matches!(catalog.discover(cell.slot()), Err(Contended)));
    assert!(Pooled::ptr_eq(
        &discover(&catalog, cell.slot()).unwrap(),
        &cell
    ));
    assert_eq!(catalog.cell_pool_occupancy(), 1);
}

#[test]
fn acquisition_failure_before_install_preserves_the_same_request() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    FAIL_SHARE.with(|remaining| remaining.set(Some(0)));
    assert_eq!(catalog.commit(&attempt), Outcome::Pending);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 0]
    );
    assert_eq!(membership(&cell), Membership::Prepared);
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
}

#[test]
fn certified_effect_survives_a_failed_controller_release_acquisition() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    // Sharing Busy and the effect succeeds; sharing the final controller fails.
    FAIL_SHARE.with(|remaining| remaining.set(Some(2)));
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert!(catalog.snapshot().is_none());
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [1, 1]
    );
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
}

#[test]
fn installed_request_cannot_be_discarded_as_a_retry_after_acquisition_failure() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    // Share the Busy proposal, then fail sharing the prepared effect. The
    // request is discoverable and may commit after this call returns.
    FAIL_SHARE.with(|remaining| remaining.set(Some(1)));
    assert_eq!(
        catalog.commit(&attempt),
        Outcome::Pending,
        "retry discarded an unresolved installed request"
    );
    assert!(catalog.snapshot().is_none());
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [1, 1]
    );
}

trait FixtureClone: Sized {
    fn clone(&self) -> Self;
}

#[test]
fn an_applied_effect_is_certified_without_another_owning_observation() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    catalog.effect_and_stop(&attempt);
    // The Busy observation succeeds. An extra effect-state observation would
    // fail, but losing the exact CAS already proves that effect occurred.
    FAIL_LOAD.with(|remaining| remaining.set(Some(1)));
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert_eq!(FAIL_LOAD.with(|remaining| remaining.replace(None)), Some(0));
    assert!(!attempt.cancel());
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [1, 1]
    );
}

impl<T> FixtureClone for Pooled<T> {
    fn clone(&self) -> Self {
        for _ in 0..1000 {
            if let Some(owner) = share(self) {
                return owner;
            }
            #[cfg(not(target_family = "wasm"))]
            std::thread::yield_now();
        }
        panic!("fixture clone did not settle");
    }
}
fn membership(cell: &Pooled<Cell>) -> Membership {
    for _ in 0..1000 {
        if let Some(state) = cell.membership() {
            return state;
        }
    }
    panic!("fixture membership did not settle");
}
fn discover(catalog: &Catalog, slot: usize) -> Option<Pooled<Cell>> {
    for _ in 0..1000 {
        if let Ok(cell) = catalog.discover(slot) {
            return cell;
        }
    }
    panic!("fixture observation did not settle");
}

fn snapshot(catalog: &Catalog) -> Snapshot {
    for _ in 0..10 {
        if let Some(snapshot) = catalog.snapshot() {
            return snapshot;
        }
    }
    panic!("controller did not finish bounded work");
}

#[test]
fn retained_controller_validates_without_a_new_owner_and_helps_a_stopped_install() {
    let catalog = Catalog::new();
    let other = Catalog::new();
    let before = snapshot(&catalog);
    assert!(!other.validates_snapshot_in(&before, &()));
    FAIL_LOAD.with(|fault| fault.set(Some(0)));
    assert!(catalog.validates_snapshot_in(&before, &()));
    assert_eq!(FAIL_LOAD.with(|fault| fault.take()), Some(0));

    let cell = fixture_cell(&catalog, 8);
    let insert = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    assert!(catalog.install_and_stop(&insert));
    assert!(!catalog.validates_snapshot_in(&before, &()));
    // The mismatching observation itself helped the installed publication.
    assert!(matches!(membership(&cell), Membership::Live(_)));
    assert!(!catalog.validates_snapshot_in(&before, &()));
    assert!(catalog.validates_snapshot_in(&snapshot(&catalog), &()));
}
fn drain(catalog: &Catalog, cell: &Pooled<Cell>) {
    for domain in 0..2 {
        let Membership::Live(work) = membership(cell) else {
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
                catalog.commit(&advance_cycle(&catalog, domain).unwrap()),
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
        assert_eq!(membership(&cell), Membership::Retired);
        assert!(matches!(membership(&replacement), Membership::Live(_)));
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
    assert_eq!(membership(&cell), Membership::Prepared);
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
        catalog.commit(&advance_cycle(&catalog, 1).unwrap()),
        Outcome::Committed
    );
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    assert_eq!(membership(&cell), Membership::Prepared);
    assert_eq!(
        catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
        Outcome::Committed
    );
    let Membership::Live(work) = membership(&cell) else {
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
                catalog.commit(&advance_cycle(&catalog, domain).unwrap()),
                Outcome::Committed
            );
        }
    }
    assert_eq!(
        snapshot(&catalog).phase().domains,
        [Domain {
            mode: DomainMode::Full,
            epoch: 3,
            floor: 0,
            pending: 0
        }; 2]
    );
}

#[test]
fn version_and_epoch_exhaustion_refuse_to_wrap() {
    for field in 0..3 {
        let mut phase = Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 4,
            domains: [Domain {
                mode: DomainMode::Full,
                epoch: 7,
                floor: RETIREMENT_FLOOR,
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
            let lifetime = next.observe();
            let slot = next.slot();
            assert_eq!(
                catalog.commit(&catalog.insert(snapshot(&catalog), next.clone()).unwrap()),
                Outcome::Committed
            );
            drain(&catalog, &next);
            assert_eq!(
                catalog.commit(&catalog.retire(snapshot(&catalog), next).unwrap()),
                Outcome::Committed
            );
            assert!(catalog.release_retired(&discover(&catalog, slot).unwrap()));
            assert!(
                lifetime.try_load().is_none(),
                "stopped helper retained later cells"
            );
        }
        let before = snapshot(&catalog).phase();
        // The stopped operation retains three old control versions and two old
        // membership values. It does not retain later completed operations.
        assert_eq!(catalog.pool_occupancy(), (4, 3));
        assert_eq!(catalog.cell_pool_occupancy(), 1);
        resume.send(()).unwrap();
        helper.join().unwrap();
        assert_eq!(membership(&cell), Membership::Retired);
        assert_eq!(snapshot(&catalog).phase(), before);
        assert_eq!(before.domains.map(|domain| domain.pending), [0, 0]);
        assert_eq!(catalog.commit(&attempt), Outcome::Committed);
        drop(attempt);
        assert_eq!(catalog.pool_occupancy(), (1, 1));
        drop(cell);
        assert_eq!(catalog.pool_occupancy(), (1, 0));
        assert_eq!(catalog.cell_pool_occupancy(), 0);
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
                    loop {
                        assert!(
                            Instant::now() < deadline,
                            "pending request did not converge"
                        );
                        match catalog.commit(&attempt) {
                            Outcome::Committed => return,
                            Outcome::Retry => break,
                            Outcome::Pending => {}
                            Outcome::Cancelled => panic!("unexpected cancellation"),
                        }
                    }
                }
            };
            for index in 1..=200 {
                let identity = worker * 200 + index;
                let cell = fixture_cell(&catalog, identity as usize * 8);
                run(&|snapshot| catalog.insert(snapshot, cell.clone()));
                for domain in 0..2 {
                    let Membership::Live(work) = membership(&cell) else {
                        panic!("publication lost")
                    };
                    run(&|snapshot| {
                        catalog.acknowledge(snapshot, cell.clone(), domain, work[domain])
                    });
                }
                run(&|snapshot| catalog.retire(snapshot, cell.clone()));
                assert_eq!(membership(&cell), Membership::Retired);
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

fn fixture_cell(catalog: &Catalog, address: usize) -> Pooled<Cell> {
    for slot in 0..catalog.capacity() {
        if let Some(old) = discover(catalog, slot) {
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
        let cell = discover(&catalog, slot).unwrap();
        assert_eq!(membership(&cell), Membership::Prepared);
        assert_eq!(
            catalog.commit(&catalog.insert(snapshot(&catalog), cell).unwrap()),
            Outcome::Committed
        );
    }
    for domain in 0..2 {
        for slot in 0..4 {
            let cell = discover(&catalog, slot).unwrap();
            let Membership::Live(work) = membership(&cell) else {
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
            catalog.commit(&advance_cycle(&catalog, domain).unwrap()),
            Outcome::Committed
        );
    }
    for slot in 0..4 {
        let cell = discover(&catalog, slot).unwrap();
        assert_eq!(
            catalog.commit(&catalog.retire(snapshot(&catalog), cell.clone()).unwrap()),
            Outcome::Committed
        );
        assert!(catalog.release_retired(&cell));
        assert!(discover(&catalog, slot).is_none());
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
    assert_eq!(membership(&old), Membership::Retired);
    assert!(Pooled::ptr_eq(&discover(&catalog, 0).unwrap(), &current));
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
    assert!(discover(&catalog, 0).is_none());
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
            let cell = discover(&catalog, 0).unwrap();
            assert_eq!(
                catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
                Outcome::Committed
            );
            drain(&catalog, &cell);
        } else {
            assert!(discover(&catalog, 0).is_none());
        }
        for _ in 0..8 {
            for domain in 0..2 {
                assert_eq!(
                    catalog.commit(&advance_cycle(&catalog, domain).unwrap()),
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
        assert!(Pooled::ptr_eq(&discover(&catalog, 0).unwrap(), &original));
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
        assert_eq!(membership(&original), Membership::Live([0, 0]));
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
        assert_eq!(membership(&cell), Membership::Prepared);
    }
}

#[test]
fn cancelled_private_attempts_return_all_prepared_control_and_state_slots() {
    let catalog = Catalog::with_backing_capacity(1, 8, 8);
    let cell = catalog.prepare_cell(0, 8).unwrap();
    for _ in 0..50 {
        let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
        assert_eq!(catalog.pool_occupancy(), (3, 2));
        assert!(attempt.cancel());
        drop(attempt);
        assert_eq!(catalog.pool_occupancy(), (1, 1));
        assert_eq!(membership(&cell), Membership::Prepared);
    }
    assert_eq!(
        catalog.commit(&catalog.insert(snapshot(&catalog), cell.clone()).unwrap()),
        Outcome::Committed
    );
    assert_eq!(catalog.pool_occupancy(), (1, 1));
}

fn trace_prepared(catalog: &Catalog, mut preparation: Preparation) -> Attempt {
    for _ in 0..1024 {
        if let Some(attempt) = preparation.poll(catalog).unwrap() {
            return attempt;
        }
    }
    panic!("tracing preparation did not settle");
}
fn trace_commit(catalog: &Catalog, preparation: Preparation) {
    assert_eq!(
        catalog.commit(&trace_prepared(catalog, preparation)),
        Outcome::Committed
    );
}
fn traced_cell(catalog: &Catalog) -> Pooled<Cell> {
    let cell = fixture_cell(catalog, 8);
    trace_commit(
        catalog,
        catalog
            .begin_trace_insert(snapshot(catalog), cell.clone())
            .unwrap(),
    );
    cell
}
fn trace_marks(cell: &Pooled<Cell>) -> [TraceMark; 2] {
    let Membership::Tracing(marks) = membership(cell) else {
        panic!("not tracing")
    };
    marks.marks
}
fn trace_ack(catalog: &Catalog, cell: &Pooled<Cell>, domain: usize) {
    let ticket = trace_marks(cell)[domain].ticket;
    trace_commit(
        catalog,
        catalog
            .begin_trace_acknowledge(snapshot(catalog), cell.clone(), domain, ticket)
            .unwrap(),
    );
}

#[test]
fn canonical_trace_floors_couple_retention_with_counted_work() {
    for floor in 0..=RETIREMENT_FLOOR {
        let catalog = Catalog::with_phase(Phase {
            root_cursors: [0; 2],
            reference_cursors: [0; 2],
            revision: 1,
            domains: [Domain {
                mode: DomainMode::Full,
                epoch: 9,
                floor,
                pending: 0,
            }; 2],
        });
        let cell = traced_cell(&catalog);
        // Generic numeric removal cannot silently serve as GC retirement.
        assert!(
            catalog
                .begin_retire(snapshot(&catalog), cell.clone())
                .is_none()
        );
        let shade =
            catalog.begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong);
        if floor == RETIREMENT_FLOOR {
            assert!(shade.is_none());
            trace_commit(
                &catalog,
                catalog
                    .begin_trace_retire(snapshot(&catalog), cell.clone(), 0)
                    .unwrap(),
            );
            assert!(catalog.release_retired(&cell));
        } else {
            trace_commit(&catalog, shade.unwrap());
            let marks = trace_marks(&cell);
            assert_eq!(marks[0].retention, TraceRetention::at_floor(floor));
            assert_ne!(
                marks[0].ticket, 0,
                "retention published without traversal work"
            );
            assert!(marks[0].marked_before(9, false) == (floor == 0));
            assert!(marks[0].marked_before(9, true) == (floor <= 2));
            assert_eq!(
                snapshot(&catalog).phase().domains.map(|d| d.pending),
                [1, 0]
            );
            assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
            trace_ack(&catalog, &cell, 0);
            assert_eq!(trace_marks(&cell)[0].retention, marks[0].retention);
            assert_eq!(
                snapshot(&catalog).phase().domains.map(|d| d.pending),
                [0, 0]
            );
        }
    }
}

#[test]
fn stronger_trace_discovery_replaces_ticket_without_double_counting() {
    let catalog = Catalog::new();
    let cell = traced_cell(&catalog);
    trace_commit(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Phantom)
            .unwrap(),
    );
    let old_ticket = trace_marks(&cell)[0].ticket;
    let old_ack = trace_prepared(
        &catalog,
        catalog
            .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), 0, old_ticket)
            .unwrap(),
    );
    trace_commit(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
            .unwrap(),
    );
    assert_eq!(catalog.commit(&old_ack), Outcome::Retry);
    let mark = trace_marks(&cell)[0];
    assert_ne!(mark.ticket, old_ticket);
    assert_eq!(mark.retention, Some(TraceRetention::Strong));
    assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
    assert!(
        catalog
            .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), 0, old_ticket)
            .is_none(),
        "stale trace ticket accepted"
    );
    assert!(
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Phantom)
            .is_none()
    );
    assert_eq!(trace_marks(&cell)[0], mark);
    trace_ack(&catalog, &cell, 0);
}

#[test]
fn completed_weaker_trace_requires_new_work_on_stronger_discovery() {
    let catalog = Catalog::new();
    let cell = traced_cell(&catalog);
    trace_commit(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Phantom)
            .unwrap(),
    );
    let first = trace_marks(&cell)[0].ticket;
    trace_ack(&catalog, &cell, 0);
    assert_eq!(snapshot(&catalog).phase().domains[0].pending, 0);
    trace_commit(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Final)
            .unwrap(),
    );
    let stronger = trace_marks(&cell)[0];
    assert_eq!(stronger.retention, Some(TraceRetention::Final));
    assert!(stronger.ticket > first);
    assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
    assert!(!stronger.marked_before(1, false));
    assert!(stronger.marked_before(1, true));
    trace_ack(&catalog, &cell, 0);
}

#[test]
fn old_epoch_trace_help_cannot_replace_current_reachability() {
    let catalog = Catalog::new();
    let cell = traced_cell(&catalog);
    let old = trace_prepared(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
            .unwrap(),
    );
    catalog.effect_and_stop(&old);
    assert_eq!(catalog.commit(&old), Outcome::Committed);
    trace_ack(&catalog, &cell, 0);
    for _ in 0..=RETIREMENT_FLOOR {
        assert_eq!(
            catalog.commit(&advance_cycle(&catalog, 0).unwrap()),
            Outcome::Committed
        );
    }
    assert_eq!(snapshot(&catalog).phase().domains[0].epoch, 2);
    assert_eq!(trace_marks(&cell)[0].retention_at(2), None);
    trace_commit(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Final)
            .unwrap(),
    );
    let expected = membership(&cell);
    catalog.late_help(&old);
    assert_eq!(membership(&cell), expected);
    assert_eq!(snapshot(&catalog).phase().domains[0].pending, 1);
    trace_ack(&catalog, &cell, 0);
}

#[test]
fn tracing_retirement_excludes_preclosure_discovery_and_reused_records() {
    let catalog = Catalog::with_phase(Phase {
        root_cursors: [0; 2],
        reference_cursors: [0; 2],
        revision: 1,
        domains: [Domain {
            mode: DomainMode::Full,
            epoch: 7,
            floor: 4,
            pending: 0,
        }; 2],
    });
    let cell = traced_cell(&catalog);
    let stale = trace_prepared(
        &catalog,
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 0, TraceRetention::Strong)
            .unwrap(),
    );
    assert!(
        catalog
            .begin_trace_retire(snapshot(&catalog), cell.clone(), 0)
            .is_none()
    );
    assert_eq!(
        catalog.commit(&advance_cycle(&catalog, 0).unwrap()),
        Outcome::Committed
    );
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    trace_commit(
        &catalog,
        catalog
            .begin_trace_retire(snapshot(&catalog), cell.clone(), 0)
            .unwrap(),
    );
    assert!(catalog.release_retired(&cell));
    let replacement = traced_cell(&catalog);
    assert_ne!(cell.identity, replacement.identity);
    let expected = membership(&replacement);
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    assert_eq!(membership(&replacement), expected);
    assert_eq!(membership(&cell), Membership::Retired);
}

#[test]
fn failed_effect_preparation_leaves_the_installed_request_cancellable() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let attempt = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    assert!(catalog.install_and_stop(&attempt));
    // Loading Busy succeeds. The first share in help is the effect owner.
    FAIL_SHARE.with(|remaining| remaining.set(Some(0)));
    assert!(catalog.snapshot().is_none());
    assert!(
        attempt.cancel(),
        "effect preparation made an irreversible commit decision"
    );
    assert_eq!(catalog.commit(&attempt), Outcome::Cancelled);
    assert_eq!(membership(&cell), Membership::Prepared);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 0]
    );
}

// Existing lifecycle fixtures explicitly end and restart a terminal cycle.
fn advance_cycle(catalog: &Catalog, domain: usize) -> Option<Attempt> {
    let before = snapshot(catalog);
    let entry = before.phase().domains[domain];
    if entry.floor == RETIREMENT_FLOOR {
        let finish = trace_prepared(catalog, catalog.begin_finish(before, domain)?);
        assert_eq!(catalog.commit(&finish), Outcome::Committed);
        Some(trace_prepared(
            catalog,
            catalog.begin_start(snapshot(catalog), domain, entry.mode)?,
        ))
    } else {
        catalog.advance(before, domain)
    }
}

fn domain_phase(modes: [DomainMode; 2], floors: [u8; 2]) -> Phase {
    Phase {
        root_cursors: [0; 2],
        reference_cursors: [0; 2],
        revision: 11,
        domains: std::array::from_fn(|i| Domain {
            mode: modes[i],
            epoch: 7 + i as u64,
            floor: floors[i],
            pending: 0,
        }),
    }
}

#[test]
fn publication_floor_table_distinguishes_births_from_reachability_and_work() {
    for young_mode in [DomainMode::Inactive, DomainMode::Young, DomainMode::Full] {
        for old_mode in [DomainMode::Inactive, DomainMode::Full, DomainMode::Old] {
            for young_floor in 0..=RETIREMENT_FLOOR {
                for old_floor in 0..=RETIREMENT_FLOOR {
                    let modes = [young_mode, old_mode];
                    let floors = [young_floor, old_floor];
                    if (0..2).any(|i| modes[i] == DomainMode::Inactive && floors[i] != 0) {
                        continue;
                    }
                    let before = domain_phase(modes, floors);
                    let (resident, after) = TraceState::publication(before).unwrap();
                    assert_eq!(resident.generation, Generation::Young);
                    assert!(!resident.logically_retired(after));
                    for i in 0..2 {
                        let domain = after.domains[i];
                        let inactive = modes[i] == DomainMode::Inactive;
                        let birth = !inactive
                            && (modes[i] == DomainMode::Old
                                || (i == 1 && floors[i] == RETIREMENT_FLOOR));
                        let needs_trace = !inactive && !birth && floors[i] == 0;
                        assert_eq!(domain.pending, usize::from(needs_trace), "{before:?}");
                        assert_eq!(resident[i].ticket, if needs_trace { 12 } else { 0 });
                        assert_eq!(resident[i].born, birth);
                        assert_eq!(resident[i].marked(domain.epoch), !inactive);
                        assert_eq!(
                            resident[i].marked_before(domain.epoch, true),
                            !inactive && !birth
                        );
                        assert!(!resident[i].marked_before(domain.epoch + 1, true));
                    }
                    assert_eq!(
                        resident.old_birth,
                        if old_mode == DomainMode::Old { 8 } else { 0 }
                    );
                }
            }
        }
    }
}

#[test]
fn publication_cannot_commit_using_a_closed_or_restarted_domain_snapshot() {
    for restart in [false, true] {
        let catalog = Catalog::new();
        let cell = fixture_cell(&catalog, 8);
        let stale = trace_prepared(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        for _ in 0..if restart { 6 } else { 1 } {
            assert_eq!(
                catalog.commit(&advance_cycle(&catalog, 0).unwrap()),
                Outcome::Committed
            );
        }
        assert_eq!(catalog.commit(&stale), Outcome::Retry);
        assert_eq!(membership(&cell), Membership::Prepared);
        trace_commit(
            &catalog,
            catalog
                .begin_publish(snapshot(&catalog), cell.clone())
                .unwrap(),
        );
        let Membership::Tracing(state) = membership(&cell) else {
            panic!()
        };
        let phase = snapshot(&catalog).phase();
        assert_eq!(state[0].epoch, if restart { 2 } else { 1 });
        assert_eq!(state[0].ticket != 0, restart);
        assert_eq!(phase.domains[0].pending, usize::from(restart));
        assert_eq!(phase.domains[1].pending, 1);
    }
}

#[test]
fn installed_publication_is_helped_before_phase_closure() {
    let catalog = Catalog::with_phase(domain_phase([DomainMode::Young, DomainMode::Full], [0, 0]));
    let cell = fixture_cell(&catalog, 8);
    let attempt = trace_prepared(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    assert!(catalog.install_and_stop(&attempt));
    let phase = snapshot(&catalog).phase();
    assert_eq!(phase.domains.map(|d| d.pending), [1, 1]);
    assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    let Membership::Tracing(state) = membership(&cell) else {
        panic!()
    };
    assert_eq!(state.marks.map(|m| m.ticket), [phase.revision; 2]);
    trace_commit(
        &catalog,
        catalog
            .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), 0, state[0].ticket)
            .unwrap(),
    );
    assert_eq!(
        catalog.commit(&catalog.advance(snapshot(&catalog), 0).unwrap()),
        Outcome::Committed
    );
    assert!(catalog.begin_advance(snapshot(&catalog), 1).is_none());
    assert_eq!(catalog.commit(&attempt), Outcome::Committed);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 1]
    );
}

#[test]
fn idle_publication_retries_after_full_domain_start() {
    let catalog = Catalog::with_phase(domain_phase([DomainMode::Inactive; 2], [0; 2]));
    let cell = fixture_cell(&catalog, 8);
    let stale = trace_prepared(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    trace_commit(
        &catalog,
        catalog
            .begin_start(snapshot(&catalog), 1, DomainMode::Full)
            .unwrap(),
    );
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    trace_commit(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    let Membership::Tracing(state) = membership(&cell) else {
        panic!()
    };
    assert!(!state[0].marked(7));
    assert_eq!(state[1].epoch, 9);
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 1]
    );
}

#[test]
fn old_only_birth_promotion_couples_generation_and_handoff_without_dropping_young_work() {
    let catalog = Catalog::with_phase(domain_phase([DomainMode::Young, DomainMode::Old], [0, 0]));
    let cell = fixture_cell(&catalog, 8);
    trace_commit(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    let Membership::Tracing(before) = membership(&cell) else {
        panic!()
    };
    assert_eq!(before[1].ticket, 0);
    assert!(before[1].born);
    assert!(
        catalog
            .begin_shade(snapshot(&catalog), cell.clone(), 1, TraceRetention::Strong)
            .is_none()
    );
    let promotion = trace_prepared(
        &catalog,
        catalog
            .begin_promote(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    assert!(catalog.install_and_stop(&promotion));
    let phase = snapshot(&catalog).phase();
    let Membership::Tracing(after) = membership(&cell) else {
        panic!()
    };
    assert_eq!(after.generation, Generation::Old);
    assert_eq!(after.old_birth, before.old_birth);
    assert_eq!(after[0], before[0]);
    assert!(!after[1].born);
    assert_eq!(after[1].retention, Some(TraceRetention::Strong));
    assert_eq!(phase.domains.map(|d| d.pending), [1, 1]);
    assert!(catalog.begin_advance(snapshot(&catalog), 1).is_none());
    assert!(
        catalog
            .begin_promote(snapshot(&catalog), cell.clone())
            .is_none()
    );
    for i in 0..2 {
        trace_commit(
            &catalog,
            catalog
                .begin_trace_acknowledge(snapshot(&catalog), cell.clone(), i, after[i].ticket)
                .unwrap(),
        );
    }
    assert_eq!(
        snapshot(&catalog).phase().domains.map(|d| d.pending),
        [0, 0]
    );
}

#[test]
fn promotion_reprepared_after_old_closure_is_black_without_reopening_work() {
    let catalog = Catalog::with_phase(domain_phase(
        [DomainMode::Inactive, DomainMode::Old],
        [0, 0],
    ));
    let cell = fixture_cell(&catalog, 8);
    trace_commit(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    let stale = trace_prepared(
        &catalog,
        catalog
            .begin_promote(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    for _ in 0..RETIREMENT_FLOOR {
        assert_eq!(
            catalog.commit(&catalog.advance(snapshot(&catalog), 1).unwrap()),
            Outcome::Committed
        );
    }
    assert_eq!(catalog.commit(&stale), Outcome::Retry);
    trace_commit(
        &catalog,
        catalog
            .begin_promote(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    let Membership::Tracing(after) = membership(&cell) else {
        panic!()
    };
    assert_eq!(after.old_birth, 8);
    assert_eq!(after.generation, Generation::Old);
    assert!(after[1].marked_before(8, false));
    assert_eq!(after[1].ticket, 0);
    assert_eq!(snapshot(&catalog).phase().domains[1].pending, 0);
    assert!(
        catalog
            .begin_trace_retire(snapshot(&catalog), cell.clone(), 1)
            .is_none()
    );
}

#[test]
fn generation_exclusions_and_other_domain_retirement_prevent_resurrection() {
    let phase = domain_phase([DomainMode::Young, DomainMode::Old], [RETIREMENT_FLOOR; 2]);
    let mut state = TraceState::unmarked(phase);
    assert_eq!(state.can_retire(phase, 1), Err(TraceError::Conflict));
    assert_eq!(state.can_retire(phase, 0), Ok(()));
    assert!(state.reference_live(phase.domains[1], 1, false));
    state.generation = Generation::Old;
    assert_eq!(state.can_retire(phase, 0), Err(TraceError::Conflict));
    assert_eq!(state.can_retire(phase, 1), Ok(()));
    assert!(state.reference_live(phase.domains[0], 0, false));
    for retiring in 0..2 {
        let mut phase = domain_phase([DomainMode::Full; 2], [0; 2]);
        phase.domains[retiring].floor = RETIREMENT_FLOOR;
        let state = TraceState::unmarked(phase);
        assert_eq!(
            state.shade(phase, 1 - retiring, TraceRetention::Strong),
            Err(TraceError::Conflict)
        );
        assert_eq!(state.promote(phase), Err(TraceError::Conflict));
    }
}

#[test]
fn idle_and_birth_states_cannot_authorize_ordinary_retirement_or_reference_liveness() {
    let idle = domain_phase([DomainMode::Inactive; 2], [0; 2]);
    let (state, _) = TraceState::publication(idle).unwrap();
    for i in 0..2 {
        assert_eq!(
            state.shade(idle, i, TraceRetention::Strong),
            Err(TraceError::Conflict)
        );
        assert_eq!(state.can_retire(idle, i), Err(TraceError::Conflict));
        assert!(!state.reference_live(idle.domains[i], i, true));
    }
    let retiring = domain_phase(
        [DomainMode::Inactive, DomainMode::Full],
        [0, RETIREMENT_FLOOR],
    );
    let (state, phase) = TraceState::publication(retiring).unwrap();
    assert!(state[1].marked(8));
    assert!(!state.reference_live(phase.domains[1], 1, true));
    assert_eq!(state.can_retire(phase, 1), Err(TraceError::Conflict));
}

#[test]
fn explicit_domain_lifecycle_refuses_pending_work_and_checked_epoch_overflow() {
    let catalog = Catalog::with_phase(domain_phase([DomainMode::Full; 2], [RETIREMENT_FLOOR; 2]));
    assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    assert!(
        catalog
            .begin_start(snapshot(&catalog), 0, DomainMode::Young)
            .is_none()
    );
    trace_commit(
        &catalog,
        catalog.begin_finish(snapshot(&catalog), 0).unwrap(),
    );
    assert_eq!(snapshot(&catalog).phase().domains[0].epoch, 7);
    assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    trace_commit(
        &catalog,
        catalog
            .begin_start(snapshot(&catalog), 0, DomainMode::Young)
            .unwrap(),
    );
    assert_eq!(
        snapshot(&catalog).phase().domains[0],
        Domain {
            mode: DomainMode::Young,
            epoch: 8,
            floor: 0,
            pending: 0
        }
    );
    let cell = fixture_cell(&catalog, 8);
    trace_commit(
        &catalog,
        catalog
            .begin_publish(snapshot(&catalog), cell.clone())
            .unwrap(),
    );
    assert!(catalog.begin_advance(snapshot(&catalog), 0).is_none());
    assert!(catalog.begin_finish(snapshot(&catalog), 0).is_none());
    let mut phase = domain_phase([DomainMode::Inactive; 2], [0; 2]);
    phase.domains[0].epoch = u64::MAX;
    let catalog = Catalog::with_phase(phase);
    assert!(
        catalog
            .begin_start(snapshot(&catalog), 0, DomainMode::Young)
            .is_none()
    );
    assert_eq!(snapshot(&catalog).phase(), phase);
}

#[test]
fn initial_young_handoff_is_retained_without_old_queue_and_full_promotion_preserves_strength() {
    let phase = domain_phase([DomainMode::Young, DomainMode::Old], [0; 2]);
    let state = TraceState::unmarked(phase);
    let (retained, after) = state.shade(phase, 1, TraceRetention::Strong).unwrap();
    assert!(retained[1].marked_before(8, false));
    assert_eq!(retained[1].ticket, 0);
    assert_eq!(after.domains.map(|d| d.pending), [0, 0]);
    let phase = domain_phase([DomainMode::Young, DomainMode::Full], [0; 2]);
    let (state, phase) = TraceState::publication(phase).unwrap();
    let (promoted, after) = state.promote(phase).unwrap();
    assert_eq!(promoted.marks, state.marks);
    assert_eq!(after, phase);
    assert_eq!(promoted.generation, Generation::Old);
}

#[test]
fn publication_and_promotion_refuse_count_or_ticket_wrap() {
    let mut phase = domain_phase([DomainMode::Young, DomainMode::Old], [0; 2]);
    phase.domains[0].pending = usize::MAX;
    assert_eq!(TraceState::publication(phase), Err(TraceError::Exhausted));
    phase.domains[0].pending = 0;
    let (state, mut phase) = TraceState::publication(phase).unwrap();
    phase.domains[1].pending = usize::MAX;
    assert_eq!(state.promote(phase), Err(TraceError::Exhausted));
    phase.domains[1].pending = 0;
    phase.revision = u64::MAX;
    assert_eq!(state.promote(phase), Err(TraceError::Exhausted));
    assert_eq!(TraceState::publication(phase), Err(TraceError::Exhausted));
}

#[test]
fn a_released_cancelled_successor_already_has_its_completion_certificate() {
    let catalog = Catalog::new();
    let cell = fixture_cell(&catalog, 8);
    let before = snapshot(&catalog).phase();
    let proposal = catalog.insert(snapshot(&catalog), cell.clone()).unwrap();
    assert!(catalog.install_and_stop(&proposal));
    assert!(proposal.cancel());
    // The releasing helper is parked after installing the successor. A
    // second helper may read or help this head before the first resumes.
    catalog.help_at(3, || {
        let chosen = snapshot(&catalog).phase();
        assert_eq!(chosen.revision, before.revision + 1);
        assert_eq!(
            chosen.domains.map(|domain| domain.pending),
            before.domains.map(|domain| domain.pending),
            "released cancellation successor exposed committed work"
        );
        assert_eq!(membership(&cell), Membership::Prepared);
    });
    assert_eq!(catalog.commit(&proposal), Outcome::Cancelled);
}
