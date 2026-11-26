use super::*;

fn fixture(mutators: usize, registry: usize) -> CatalogHandle {
    BoundedCatalog::new(Capacities::derive(registry, mutators, 1).unwrap()).unwrap()
}
fn run(participant: &mut Participant, command: Command) -> Progress {
    participant.submit(command).unwrap();
    finish(participant)
}
fn finish(participant: &mut Participant) -> Progress {
    for _ in 0..4096 {
        let result = participant.poll();
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("quiescent participant did not finish within the pool traversal bound");
}
fn create(participant: &mut Participant, slot: usize) -> RecordKey {
    let Progress::Created(key) = run(
        participant,
        Command::Create {
            slot,
            address: 8 + slot * 8,
        },
    ) else {
        panic!("creation failed");
    };
    key
}
fn prepare_attempt(participant: &mut Participant, command: Command) {
    participant.submit(command).unwrap();
    for _ in 0..4096 {
        assert_eq!(participant.poll(), Progress::Pending);
        if matches!(participant.state, RequestState::Attempt(..)) {
            return;
        }
    }
    panic!("preparation did not finish");
}
fn install(participant: &Participant) {
    let RequestState::Attempt(_, attempt) = &participant.state else {
        panic!("not prepared")
    };
    assert!(participant.lease.owner.raw.install_and_stop(attempt));
}
fn assert_bounded(catalog: &BoundedCatalog) {
    let k = catalog.capacities.participants();
    let n = catalog.capacities.registry();
    let (controls, memberships) = catalog.raw.pool_occupancy();
    let cells = catalog.raw.cell_pool_occupancy();
    assert!(controls <= 7 * k + 3, "controls: {controls}");
    assert!(memberships <= n + 6 * k + 3, "memberships: {memberships}");
    assert!(cells <= n + 2 * k + 1, "cells: {cells}");
    let capacity = catalog.capacities.pools();
    assert!(controls < capacity.0 && memberships < capacity.1 && cells < capacity.2);
}
fn assert_empty(catalog: &BoundedCatalog) {
    assert_eq!(catalog.raw.pool_occupancy(), (1, 0));
    assert_eq!(catalog.raw.cell_pool_occupancy(), 0);
}

#[test]
fn checked_capacity_derivation_leaves_slack_and_rejects_overflow() {
    for k in [0, 1, 7, 31, 1023] {
        for n in [1, 3, 64, 1024] {
            let limits = Capacities::derive(n, k, 1).unwrap();
            let (controls, memberships, cells) = limits.pools();
            assert!(controls.is_power_of_two() && controls > 7 * (k + 1) + 3);
            assert!(memberships.is_power_of_two() && memberships > n + 6 * (k + 1) + 3);
            assert!(cells.is_power_of_two() && cells > n + 2 * (k + 1) + 1);
        }
    }
    assert_eq!(Capacities::derive(0, 1, 1), Err(ConfigError::EmptyRegistry));
    assert_eq!(Capacities::derive(1, 1, 0), Err(ConfigError::NoHelpers));
    for (n, m, h) in [
        (usize::MAX, 1, 1),
        (1, usize::MAX, 1),
        (1, 1, usize::MAX),
        (MAX_POOL, 0, 1),
        (1, MAX_POOL, 1),
    ] {
        assert_eq!(
            Capacities::derive(n, m, h),
            Err(ConfigError::CapacityOverflow)
        );
    }
}

#[test]
fn helpers_have_independent_registration_and_private_drop_releases_all_backing() {
    let catalog = fixture(2, 1);
    let a = catalog.register_mutator().unwrap();
    let mut b = catalog.register_mutator().unwrap();
    assert!(matches!(
        catalog.register_mutator(),
        Err(RegisterError::Limit)
    ));
    let helper = catalog.register_helper().unwrap();
    assert!(matches!(
        catalog.register_helper(),
        Err(RegisterError::Limit)
    ));
    prepare_attempt(&mut b, Command::Advance { domain: 0 });
    assert_eq!(catalog.registered(), (2, 1));
    assert_eq!(catalog.raw.pool_occupancy().0, 2);
    drop(b);
    assert_eq!(catalog.registered(), (1, 1));
    assert_empty(&catalog);
    let replacement = catalog.register_mutator().unwrap();
    drop((a, replacement, helper));
    assert_eq!(catalog.registered(), (0, 0));
}

#[test]
fn keys_do_not_retain_storage_or_alias_foreign_or_reused_slots() {
    let catalog = fixture(1, 1);
    let other = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    let mut q = other.register_mutator().unwrap();
    let old = create(&mut p, 0);
    let foreign = create(&mut q, 0);
    assert_eq!(old.identity(), foreign.identity());
    assert_eq!(p.observe(foreign), Err(RequestError::Foreign));
    assert_eq!(
        p.submit(Command::Abandon(foreign)),
        Err(RequestError::Foreign)
    );
    assert_eq!(run(&mut p, Command::Abandon(old)), Progress::Committed);
    assert!(p.release_retired(old).unwrap());
    assert_empty(&catalog);
    let new = create(&mut p, 0);
    assert_ne!(old.identity(), new.identity());
    assert_eq!(p.observe(old), Err(RequestError::Stale));
    assert_eq!(p.release_retired(old), Err(RequestError::Stale));
    assert_eq!(
        run(&mut p, Command::Insert(old)),
        Progress::Rejected(RequestError::Stale)
    );
    assert_eq!(p.discover(0).unwrap().unwrap().key, new);
}

#[test]
fn ordinary_lifecycle_checks_commands_and_phase_accounting() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    assert_eq!(
        run(
            &mut p,
            Command::Create {
                slot: 1,
                address: 8
            }
        ),
        Progress::Rejected(RequestError::InvalidSlot)
    );
    assert_eq!(
        run(
            &mut p,
            Command::Create {
                slot: 0,
                address: 0
            }
        ),
        Progress::Rejected(RequestError::InvalidAddress)
    );
    let record = create(&mut p, 0);
    p.submit(Command::Insert(record)).unwrap();
    assert_eq!(p.phase(), Err(RequestError::Busy));
    assert_eq!(p.observe(record), Err(RequestError::Busy));
    assert_eq!(p.discover(0), Err(RequestError::Busy));
    assert_eq!(p.release_retired(record), Err(RequestError::Busy));
    assert_eq!(p.submit(Command::Abandon(record)), Err(RequestError::Busy));
    assert_eq!(finish(&mut p), Progress::Committed);
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
    assert_eq!(
        run(&mut p, Command::Retire(record)),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        run(&mut p, Command::Advance { domain: 0 }),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        run(&mut p, Command::Advance { domain: 2 }),
        Progress::Rejected(RequestError::InvalidDomain)
    );
    assert_eq!(
        run(
            &mut p,
            Command::Require {
                key: record,
                domain: 2
            }
        ),
        Progress::Rejected(RequestError::InvalidDomain)
    );
    for domain in 0..2 {
        assert_eq!(
            run(
                &mut p,
                Command::Require {
                    key: record,
                    domain
                }
            ),
            Progress::Committed
        );
        let Membership::Live(tickets) = p.observe(record).unwrap().membership else {
            panic!()
        };
        assert_eq!(
            run(
                &mut p,
                Command::Acknowledge {
                    key: record,
                    domain,
                    ticket: tickets[domain]
                }
            ),
            Progress::Committed
        );
    }
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [0, 0]);
    assert_eq!(run(&mut p, Command::Retire(record)), Progress::Committed);
    assert!(p.release_retired(record).unwrap());
    assert_empty(&catalog);
}

#[test]
fn helper_advances_with_every_mutator_retaining_a_distinct_private_request() {
    let catalog = fixture(12, 12);
    let mut helper = catalog.register_helper().unwrap();
    let mut stopped = Vec::new();
    for slot in 0..12 {
        let mut participant = catalog.register_mutator().unwrap();
        let record = create(&mut participant, slot);
        prepare_attempt(&mut participant, Command::Insert(record));
        stopped.push(participant);
        // Supersede the stopped snapshot; each participant retains a distinct
        // stable controller plus Busy and both immutable outcomes.
        assert_eq!(advance_cycle(&mut helper, slot % 2), Progress::Committed);
        assert_bounded(&catalog);
    }
    assert!(matches!(
        catalog.register_mutator(),
        Err(RegisterError::Limit)
    ));
    for _ in 0..64 {
        assert_eq!(advance_cycle(&mut helper, 0), Progress::Committed);
        assert_bounded(&catalog);
    }
    for slot in 0..12 {
        let record = helper.discover(slot).unwrap().unwrap().key;
        assert_eq!(
            run(&mut helper, Command::Abandon(record)),
            Progress::Committed
        );
        assert!(helper.release_retired(record).unwrap());
        assert_bounded(&catalog);
    }
    // Released registry slots may be reused while old participant requests retain
    // old cells. Stale identities must never act on the replacement cells.
    for slot in 0..12 {
        let new = create(&mut helper, slot);
        assert_eq!(run(&mut helper, Command::Abandon(new)), Progress::Committed);
        assert!(helper.release_retired(new).unwrap());
    }
    for participant in &mut stopped {
        assert_eq!(finish(participant), Progress::Rejected(RequestError::Stale));
    }
    assert_empty(&catalog);
}

#[test]
fn dropped_installed_request_is_cancelled_and_helpable() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut p, 0);
    prepare_attempt(&mut p, Command::Insert(record));
    install(&p);
    drop(p);
    assert_eq!(catalog.registered(), (0, 1));
    assert_eq!(helper.phase(), Err(RequestError::Contended));
    assert_eq!(helper.phase().unwrap().domains.map(|d| d.pending), [0, 0]);
    assert_eq!(
        helper.observe(record).unwrap().membership,
        Membership::Prepared
    );
    assert_eq!(
        run(&mut helper, Command::Abandon(record)),
        Progress::Committed
    );
    assert!(helper.release_retired(record).unwrap());
    assert_empty(&catalog);
}

#[test]
fn committed_decision_survives_originator_drop_and_cannot_be_cancelled() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut p, 0);
    prepare_attempt(&mut p, Command::Insert(record));
    let RequestState::Attempt(_, attempt) = &p.state else {
        panic!()
    };
    catalog.raw.effect_and_stop(attempt);
    assert!(!p.cancel());
    assert!(matches!(p.state, RequestState::Attempt(..)));
    drop(p);
    assert_eq!(helper.phase(), Err(RequestError::Contended));
    assert_eq!(helper.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
    assert!(matches!(
        helper.observe(record).unwrap().membership,
        Membership::Live(_)
    ));
    assert_bounded(&catalog);
}

#[test]
fn cancelled_partial_creation_returns_every_private_slot() {
    for polls in 0..4 {
        let catalog = fixture(1, 1);
        let mut p = catalog.register_mutator().unwrap();
        p.submit(Command::Create {
            slot: 0,
            address: 8,
        })
        .unwrap();
        for _ in 0..polls {
            assert_eq!(p.poll(), Progress::Pending);
        }
        assert!(p.cancel());
        assert_eq!(p.poll(), Progress::Idle);
        assert_eq!(p.discover(0).unwrap(), None);
        assert_empty(&catalog);
    }
}

#[test]
fn discovery_contention_keeps_the_creation_command_pending() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    p.submit(Command::Create {
        slot: 0,
        address: 8,
    })
    .unwrap();
    crate::FAIL_LOAD.with(|remaining| remaining.set(Some(0)));
    assert_eq!(p.poll(), Progress::Pending);
    assert!(matches!(p.state, RequestState::Starting(_)));
    assert!(matches!(finish(&mut p), Progress::Created(_)));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn another_helper_finishes_while_every_mutator_is_stopped_in_help() {
    for point in 0..4 {
        let catalog = fixture(3, 1);
        let mut owner = catalog.register_mutator().unwrap();
        let mut helper = catalog.register_helper().unwrap();
        let record = create(&mut owner, 0);
        prepare_attempt(&mut owner, Command::Insert(record));
        install(&owner);
        let mut joins = Vec::new();
        let mut resumes = Vec::new();
        // Separate ordinary participants each hold a helper observation. The
        // fixture stops after acquisition inside the otherwise callback-free poll.
        // Acquire sequentially: acquisition may report contention, so this stop
        // fixture must not assume simultaneous owning loads always succeed.
        // Step each acquired observation to its stop sequentially: a concurrent
        // effect-owner clone can also fail and return before the stop hook.
        let mut starts = Vec::new();
        for _ in 0..2 {
            let participant = catalog.register_mutator().unwrap();
            let (send, receive) = std::sync::mpsc::sync_channel(0);
            let (observed_send, observed_receive) = std::sync::mpsc::sync_channel(0);
            let (start_send, start_receive) = std::sync::mpsc::sync_channel(0);
            let (stopped_send, stopped_receive) = std::sync::mpsc::sync_channel(0);
            joins.push(std::thread::spawn(move || {
                participant.lease.owner.raw.help_observed_at(
                    point,
                    || {
                        observed_send.send(()).unwrap();
                        start_receive.recv().unwrap();
                    },
                    || {
                        stopped_send.send(()).unwrap();
                        receive.recv().unwrap();
                    },
                );
                drop(participant);
            }));
            observed_receive.recv().unwrap();
            starts.push((start_send, stopped_receive));
            resumes.push(send);
        }
        for (start, stopped) in starts {
            start.send(()).unwrap();
            stopped.recv().unwrap();
        }
        assert_bounded(&catalog);
        // complete the canonical operation without resuming any stopped mutator
        for _ in 0..4 {
            if helper.phase().is_ok() {
                break;
            }
        }
        assert_eq!(helper.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
        let Membership::Live(tickets) = helper.observe(record).unwrap().membership else {
            panic!()
        };
        for (domain, ticket) in tickets.into_iter().enumerate() {
            assert_eq!(
                run(
                    &mut helper,
                    Command::Acknowledge {
                        key: record,
                        domain,
                        ticket
                    }
                ),
                Progress::Committed
            );
        }
        assert_eq!(
            run(&mut helper, Command::Retire(record)),
            Progress::Committed
        );
        assert!(helper.release_retired(record).unwrap());
        let replacement = create(&mut helper, 0);
        assert_eq!(advance_cycle(&mut helper, 0), Progress::Committed);
        assert_bounded(&catalog);
        for resume in resumes {
            resume.send(()).unwrap();
        }
        for join in joins {
            join.join().unwrap();
        }
        assert_eq!(finish(&mut owner), Progress::Committed);
        assert_eq!(
            helper.observe(replacement).unwrap().membership,
            Membership::Prepared
        );
        assert_eq!(
            run(&mut helper, Command::Abandon(replacement)),
            Progress::Committed
        );
        assert!(helper.release_retired(replacement).unwrap());
        assert_empty(&catalog);
    }
}

#[test]
fn quota_is_not_returned_until_owned_request_storage_is_destroyed() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    prepare_attempt(&mut p, Command::Advance { domain: 0 });
    let observed = catalog.clone();
    p.lease.before_release = Some(Box::new(move || {
        assert_eq!(observed.registered(), (1, 0));
        assert!(matches!(
            observed.register_mutator(),
            Err(RegisterError::Limit)
        ));
        assert_empty(&observed);
    }));
    drop(p);
    assert_eq!(catalog.registered(), (0, 0));
}

#[test]
fn private_cell_creation_retains_its_membership_across_a_full_cell_pool() {
    let raw = Catalog::with_pool_capacities(2, 32, 8, 1);
    let old = raw.prepare_cell(0, 8).unwrap();
    let mut creation = raw.begin_create(1, 16).unwrap();
    assert!(creation.poll(&raw).unwrap().is_none());
    for _ in 0..64 {
        assert!(creation.poll(&raw).unwrap().is_none());
        assert_eq!(
            raw.pool_occupancy().1,
            2,
            "private membership was discarded"
        );
        assert_eq!(raw.cell_pool_occupancy(), 1);
    }
    let attempt = raw
        .abandon(raw.snapshot().unwrap(), old.try_clone().unwrap())
        .unwrap();
    assert_eq!(raw.commit(&attempt), Outcome::Committed);
    drop(attempt);
    assert!(raw.release_retired(&old));
    drop(old);
    assert_eq!(raw.pool_occupancy().1, 1);
    assert!(creation.poll(&raw).unwrap().is_none());
    let cell = creation.poll(&raw).unwrap().unwrap();
    assert_eq!(cell.slot(), 1);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
    assert!(matches!(creation.poll(&raw), Err(CreationError::Finished)));
}

#[test]
fn creation_rejects_foreign_polls_and_losing_announcements_release_backing() {
    let raw = Catalog::with_pool_capacities(1, 16, 8, 4);
    let other = Catalog::new();
    let mut a = raw.begin_create(0, 8).unwrap();
    let mut b = raw.begin_create(0, 16).unwrap();
    assert!(matches!(a.poll(&other), Err(CreationError::Foreign)));
    for _ in 0..2 {
        assert!(a.poll(&raw).unwrap().is_none());
        assert!(b.poll(&raw).unwrap().is_none());
    }
    assert_eq!(raw.pool_occupancy().1, 2);
    assert_eq!(raw.cell_pool_occupancy(), 2);
    let cell = a.poll(&raw).unwrap().unwrap();
    assert!(matches!(b.poll(&raw), Err(CreationError::Occupied)));
    assert_eq!(raw.pool_occupancy().1, 1);
    assert_eq!(raw.cell_pool_occupancy(), 1);
    assert_eq!(raw.discover(0).unwrap().unwrap().identity, cell.identity);
}

#[test]
fn helper_can_create_and_retire_with_all_mutators_stopped_before_announcement() {
    let catalog = fixture(8, 1);
    let mut stopped = Vec::new();
    for _ in 0..8 {
        let mut p = catalog.register_mutator().unwrap();
        p.submit(Command::Create {
            slot: 0,
            address: 8,
        })
        .unwrap();
        for _ in 0..3 {
            assert_eq!(p.poll(), Progress::Pending);
        }
        stopped.push(p);
    }
    assert_eq!(catalog.raw.cell_pool_occupancy(), 8);
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut helper, 0);
    assert_bounded(&catalog);
    for p in &mut stopped {
        assert_eq!(finish(p), Progress::Rejected(RequestError::Conflict));
    }
    assert_eq!(
        run(&mut helper, Command::Abandon(record)),
        Progress::Committed
    );
    assert!(helper.release_retired(record).unwrap());
    assert_empty(&catalog);
}

#[test]
fn a_pending_installed_facade_request_keeps_its_original_completion() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut p, 0);
    prepare_attempt(&mut p, Command::Insert(record));
    crate::FAIL_SHARE.with(|remaining| remaining.set(Some(1)));
    assert_eq!(p.poll(), Progress::Pending);
    assert_eq!(p.submit(Command::Abandon(record)), Err(RequestError::Busy));
    assert_eq!(helper.phase(), Err(RequestError::Contended));
    assert_eq!(helper.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
    assert_eq!(finish(&mut p), Progress::Committed);
    assert_eq!(p.poll(), Progress::Idle);
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
}

#[test]
fn failed_creation_announcement_can_lose_without_removing_the_winner() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    p.submit(Command::Create {
        slot: 0,
        address: 8,
    })
    .unwrap();
    for _ in 0..3 {
        assert_eq!(p.poll(), Progress::Pending);
    }
    crate::FAIL_SHARE.with(|remaining| remaining.set(Some(0)));
    assert_eq!(p.poll(), Progress::Pending);
    assert_eq!(helper.discover(0).unwrap(), None);
    let winner = create(&mut helper, 0);
    assert_eq!(finish(&mut p), Progress::Rejected(RequestError::Conflict));
    assert_eq!(p.discover(0).unwrap().unwrap().key, winner);
    assert_eq!(catalog.raw.pool_occupancy().1, 1);
    assert_eq!(catalog.raw.cell_pool_occupancy(), 1);
}

#[test]
fn failed_membership_acquisition_does_not_reject_a_valid_command() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    let record = create(&mut p, 0);
    p.submit(Command::Insert(record)).unwrap();
    // Fail the owning membership load after snapshot and registry discovery.
    // The diagnostic may observe validity; that must preserve the command.
    crate::FAIL_LOAD.with(|remaining| remaining.set(Some(2)));
    assert_eq!(p.poll(), Progress::Pending);
    assert_eq!(finish(&mut p), Progress::Committed);
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
}

#[test]
fn an_exact_budget_reserves_backing_and_every_participant_before_construction() {
    let capacities = Capacities::derive(3, 2, 1).unwrap();
    let footprint = Footprint::for_capacities(capacities).unwrap();
    assert_eq!(
        footprint.participants(),
        3 * std::mem::size_of::<Participant>()
    );
    assert_eq!(
        footprint.total(),
        footprint.backing() + footprint.participants()
    );
    let short = Budget::new(footprint.total() - 1);
    assert!(matches!(
        BoundedCatalog::new_in(capacities, &short),
        Err(ConfigError::BudgetExceeded)
    ));
    assert_eq!(short.used(), 0);
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    assert_eq!(catalog.footprint(), footprint);
    assert_eq!(catalog.charged_bytes(), footprint.total());
    assert_eq!(catalog.reserved_storage_bytes(), 0);
    assert_eq!(budget.used(), footprint.total());
    assert!(matches!(
        BoundedCatalog::new_in(capacities, &budget),
        Err(ConfigError::BudgetExceeded)
    ));
    let cloned = catalog.clone();
    drop(catalog);
    assert_eq!(budget.used(), footprint.total());
    drop(cloned);
    assert_eq!(budget.used(), 0);
}

#[test]
fn a_stopped_participant_retains_the_whole_catalog_charge_but_a_copied_key_does_not() {
    let capacities = Capacities::derive(1, 1, 1).unwrap();
    let footprint = Footprint::for_capacities(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    let mut p = catalog.register_mutator().unwrap();
    let key = create(&mut p, 0);
    prepare_attempt(&mut p, Command::Insert(key));
    drop(catalog);
    assert_eq!(budget.used(), footprint.total());
    let observation = budget.clone();
    p.lease.before_release = Some(Box::new(move || {
        assert_eq!(observation.used(), footprint.total());
    }));
    drop(p);
    assert_eq!(budget.used(), 0);
    assert_eq!(key.slot(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn cancellation_of_a_private_budget_reservation_refunds_once() {
    let budget = Budget::new(4096);
    let result = std::panic::catch_unwind(|| {
        let _charge = budget.reserve(4096).unwrap();
        assert_eq!(budget.used(), 4096);
        panic!("private construction cancelled");
    });
    assert!(result.is_err());
    assert_eq!(budget.used(), 0);
    let charge = budget.reserve(4096).unwrap();
    assert_eq!(budget.used(), 4096);
    drop(charge);
    assert_eq!(budget.used(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_last_handles_refund_exactly_once() {
    for _ in 0..if cfg!(miri) { 2 } else { 32 } {
        let capacities = Capacities::derive(1, 0, 1).unwrap();
        let footprint = Footprint::for_capacities(capacities).unwrap();
        let budget = Budget::new(footprint.total());
        let a = BoundedCatalog::new_in(capacities, &budget).unwrap();
        let b = a.clone();
        let gate = Arc::new(std::sync::Barrier::new(2));
        let worker = {
            let gate = gate.clone();
            std::thread::spawn(move || {
                gate.wait();
                drop(a);
            })
        };
        gate.wait();
        drop(b);
        worker.join().unwrap();
        assert_eq!(budget.used(), 0);
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn competing_catalogs_cannot_overdraw_a_shared_budget() {
    let capacities = Capacities::derive(1, 0, 1).unwrap();
    let footprint = Footprint::for_capacities(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let gate = Arc::new(std::sync::Barrier::new(2));
    let worker = {
        let budget = budget.clone();
        let gate = gate.clone();
        std::thread::spawn(move || {
            gate.wait();
            BoundedCatalog::new_in(capacities, &budget)
        })
    };
    gate.wait();
    let right = BoundedCatalog::new_in(capacities, &budget);
    let left = worker.join().unwrap();
    let winner = match (left, right) {
        (Ok(catalog), Err(ConfigError::BudgetExceeded | ConfigError::BudgetContended))
        | (Err(ConfigError::BudgetExceeded | ConfigError::BudgetContended), Ok(catalog)) => catalog,
        _ => panic!("shared budget did not have exactly one winner"),
    };
    assert_eq!(budget.used(), footprint.total());
    drop(winner);
    assert_eq!(budget.used(), 0);
}

#[test]
fn generation_limits_are_checked_and_default_width_is_preserved() {
    let capacities = Capacities::derive(1, 1, 1).unwrap();
    assert_eq!(capacities.generation_limit(), pool::MAX_GENERATION);
    assert_eq!(
        capacities.with_generation_limit(0),
        Err(ConfigError::InvalidGenerationLimit)
    );
    assert_eq!(
        capacities.with_generation_limit(pool::MAX_GENERATION + 1),
        Err(ConfigError::InvalidGenerationLimit)
    );
    assert_eq!(
        capacities
            .with_generation_limit(3)
            .unwrap()
            .generation_limit(),
        3
    );
}

#[test]
fn exhausted_backing_returns_a_terminal_error_instead_of_pending_forever() {
    let capacities = Capacities::derive(1, 1, 1)
        .unwrap()
        .with_generation_limit(3)
        .unwrap();
    let footprint = Footprint::for_capacities(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    let mut participant = catalog.register_mutator().unwrap();
    let domain = participant.phase().unwrap().domains[0];
    let command = if domain.mode == crate::DomainMode::Inactive {
        Command::Start {
            domain: 0,
            mode: crate::DomainMode::Full,
        }
    } else if domain.floor == crate::RETIREMENT_FLOOR {
        Command::Finish { domain: 0 }
    } else {
        Command::Advance { domain: 0 }
    };
    participant.submit(command).unwrap();
    let mut completed = 0;
    let mut rejected = false;
    for _ in 0..1000 {
        let controls_before = catalog.raw.pool_occupancy().0;
        match participant.poll() {
            Progress::Pending => {}
            Progress::Committed => {
                completed += 1;
                let domain = participant.phase().unwrap().domains[0];
                let command = if domain.mode == crate::DomainMode::Inactive {
                    Command::Start {
                        domain: 0,
                        mode: crate::DomainMode::Full,
                    }
                } else if domain.floor == crate::RETIREMENT_FLOOR {
                    Command::Finish { domain: 0 }
                } else {
                    Command::Advance { domain: 0 }
                };
                participant.submit(command).unwrap();
            }
            Progress::Rejected(RequestError::GenerationExhausted) => {
                // Effect-free requests need one private controller. Exhaustion
                // must reject even though no partial outcomes were allocated.
                assert_eq!(controls_before, 1);
                rejected = true;
                break;
            }
            other => panic!("unexpected outcome before exhaustion: {other:?}"),
        }
    }
    assert!(
        rejected,
        "exhausted numeric backing did not return a terminal error"
    );
    assert!(completed > 0);
    assert_eq!(participant.poll(), Progress::Idle);
    assert_eq!(participant.phase().unwrap().revision, completed + 1);
    assert_eq!(
        participant.phase().unwrap().domains.map(|d| d.pending),
        [0, 0]
    );
    assert!(catalog.raw.generation_exhaustion_observed());
    assert_empty(&catalog);
    drop((participant, catalog));
    assert_eq!(budget.used(), 0);
}

#[test]
fn exhausted_effect_backing_releases_both_private_phase_outcomes() {
    let capacities = Capacities::derive(1, 1, 1)
        .unwrap()
        .with_generation_limit(3)
        .unwrap();
    let budget = Budget::new(Footprint::for_capacities(capacities).unwrap().total());
    let catalog = BoundedCatalog::new_in(capacities, &budget).unwrap();
    let mut participant = catalog.register_mutator().unwrap();
    let record = create(&mut participant, 0);
    let before = participant.phase().unwrap();
    // Selected controller probes are 1, 2. Exhaust just the final Busy
    // location, leaving the combined phase successor available beforehand.
    for _ in 0..3 {
        assert!(catalog.raw.probe_control_generation(2, before).is_none());
    }
    participant.submit(Command::Insert(record)).unwrap();
    let mut rejected = false;
    for _ in 0..16 {
        let controls_before = catalog.raw.pool_occupancy().0;
        match participant.poll() {
            Progress::Pending => {}
            Progress::Rejected(RequestError::GenerationExhausted) => {
                assert_eq!(controls_before, 2);
                rejected = true;
                break;
            }
            other => panic!("unexpected effect preparation outcome: {other:?}"),
        }
    }
    assert!(rejected, "effect backing exhaustion was not terminal");
    assert_eq!(participant.poll(), Progress::Idle);
    assert_eq!(participant.phase().unwrap(), before);
    assert_eq!(
        participant.observe(record).unwrap().membership,
        Membership::Prepared
    );
    assert_eq!(catalog.raw.pool_occupancy(), (1, 1));
    drop((participant, catalog));
    assert_eq!(budget.used(), 0);
}

#[test]
fn exhausted_creation_backing_releases_partial_state_without_announcing() {
    for exhaust_cells in [false, true] {
        let capacities = Capacities::derive(1, 1, 1)
            .unwrap()
            .with_generation_limit(1)
            .unwrap();
        let catalog = BoundedCatalog::new(capacities).unwrap();
        let mut p = catalog.register_mutator().unwrap();
        let (_, memberships, cells) = capacities.pools();
        let (count, polls) = if exhaust_cells {
            (cells, 3)
        } else {
            (memberships, 2)
        };
        for _ in 0..count {
            p.submit(Command::Create {
                slot: 0,
                address: 8,
            })
            .unwrap();
            for _ in 0..polls {
                assert_eq!(p.poll(), Progress::Pending);
            }
            assert!(p.cancel());
        }
        assert_empty(&catalog);
        assert_eq!(
            run(
                &mut p,
                Command::Create {
                    slot: 0,
                    address: 8
                }
            ),
            Progress::Rejected(RequestError::GenerationExhausted)
        );
        assert_eq!(p.discover(0).unwrap(), None);
        assert_eq!(p.phase().unwrap().revision, 1);
        assert_empty(&catalog);
    }
}

#[test]
fn an_exhausted_membership_proposal_is_not_retried_forever_or_reported_as_empty() {
    let capacities = Capacities::derive(2, 1, 1)
        .unwrap()
        .with_generation_limit(1)
        .unwrap();
    let catalog = BoundedCatalog::new(capacities).unwrap();
    let mut p = catalog.register_mutator().unwrap();
    let record = create(&mut p, 0);
    for _ in 1..capacities.pools().1 {
        p.submit(Command::Create {
            slot: 1,
            address: 16,
        })
        .unwrap();
        for _ in 0..2 {
            assert_eq!(p.poll(), Progress::Pending);
        }
        assert!(p.cancel());
    }
    assert_eq!(
        run(&mut p, Command::Insert(record)),
        Progress::Rejected(RequestError::GenerationExhausted)
    );
    assert_eq!(p.observe(record).unwrap().membership, Membership::Prepared);
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [0, 0]);
    assert_eq!(catalog.raw.pool_occupancy(), (1, 1));
    assert_eq!(catalog.raw.cell_pool_occupancy(), 1);
}

#[test]
fn installed_final_generation_work_finishes_after_an_exhausted_probe() {
    let capacities = Capacities::derive(1, 1, 1)
        .unwrap()
        .with_generation_limit(1)
        .unwrap();
    let catalog = BoundedCatalog::new(capacities).unwrap();
    let mut owner = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let phase = helper.phase().unwrap();
    let record = create(&mut owner, 0);
    prepare_attempt(&mut owner, Command::Insert(record));
    install(&owner);
    assert_eq!(
        catalog.raw.probe_control_generation(0, phase),
        Some(pool::AllocationError::GenerationExhausted)
    );
    assert!(catalog.raw.generation_exhaustion_observed());
    assert_eq!(helper.phase(), Err(RequestError::Contended));
    assert_eq!(helper.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
    assert_eq!(finish(&mut owner), Progress::Committed);
    let Membership::Live(tickets) = helper.observe(record).unwrap().membership else {
        panic!()
    };
    for (domain, ticket) in tickets.into_iter().enumerate() {
        assert_eq!(
            run(
                &mut helper,
                Command::Acknowledge {
                    key: record,
                    domain,
                    ticket
                }
            ),
            Progress::Committed
        );
    }
    assert_eq!(
        run(&mut helper, Command::Retire(record)),
        Progress::Committed
    );
    assert!(helper.release_retired(record).unwrap());
    assert_empty(&catalog);
}

#[test]
fn bounded_trace_commands_preserve_retention_and_require_white_retirement() {
    use crate::TraceRetention;
    let catalog = BoundedCatalog::new(Capacities::derive(1, 1, 1).unwrap()).unwrap();
    let mut owner = catalog.register_mutator().unwrap();
    let record = create(&mut owner, 0);
    assert_eq!(
        run(&mut owner, Command::TraceInsert(record)),
        Progress::Committed
    );
    let shade = Command::Shade {
        key: record,
        domain: 0,
        retention: TraceRetention::Strong,
    };
    assert_eq!(run(&mut owner, shade), Progress::Committed);
    assert_eq!(
        run(&mut owner, shade),
        Progress::Rejected(RequestError::AlreadyMarked)
    );
    assert_eq!(
        run(&mut owner, Command::Retire(record)),
        Progress::Rejected(RequestError::Conflict)
    );
    let Membership::Tracing(marks) = owner.observe(record).unwrap().membership else {
        panic!()
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Strong));
    assert_eq!(
        run(
            &mut owner,
            Command::TraceAcknowledge {
                key: record,
                domain: 0,
                ticket: marks[0].ticket
            }
        ),
        Progress::Committed
    );
    for _ in 0..crate::RETIREMENT_FLOOR {
        assert_eq!(advance_cycle(&mut owner, 0), Progress::Committed);
    }
    assert_eq!(
        run(
            &mut owner,
            Command::TraceRetire {
                key: record,
                domain: 0
            }
        ),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        run(&mut owner, shade),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(advance_cycle(&mut owner, 0), Progress::Committed);
    for _ in 0..crate::RETIREMENT_FLOOR {
        assert_eq!(advance_cycle(&mut owner, 0), Progress::Committed);
    }
    assert_eq!(
        run(
            &mut owner,
            Command::TraceRetire {
                key: record,
                domain: 0
            }
        ),
        Progress::Committed
    );
    assert!(owner.release_retired(record).unwrap());
    assert_empty(&catalog);
}

#[test]
fn a_stopped_tracing_originator_does_not_block_helper_epoch_progress() {
    use crate::TraceRetention;
    let catalog = BoundedCatalog::new(Capacities::derive(1, 1, 1).unwrap()).unwrap();
    let mut owner = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut owner, 0);
    assert_eq!(
        run(&mut owner, Command::TraceInsert(record)),
        Progress::Committed
    );
    prepare_attempt(
        &mut owner,
        Command::Shade {
            key: record,
            domain: 0,
            retention: TraceRetention::Strong,
        },
    );
    install(&owner);
    assert_eq!(helper.phase(), Err(RequestError::Contended));
    let Membership::Tracing(marks) = helper.observe(record).unwrap().membership else {
        panic!()
    };
    assert_eq!(helper.phase().unwrap().domains[0].pending, 1);
    assert_eq!(
        run(
            &mut helper,
            Command::TraceAcknowledge {
                key: record,
                domain: 0,
                ticket: marks[0].ticket
            }
        ),
        Progress::Committed
    );
    for _ in 0..=crate::RETIREMENT_FLOOR {
        assert_eq!(advance_cycle(&mut helper, 0), Progress::Committed);
    }
    assert_eq!(
        run(
            &mut helper,
            Command::Shade {
                key: record,
                domain: 0,
                retention: TraceRetention::Final
            }
        ),
        Progress::Committed
    );
    let current = helper.observe(record).unwrap();
    let Membership::Tracing(marks) = current.membership else {
        panic!()
    };
    assert_eq!(marks[0].epoch, 2);
    assert_eq!(marks[0].retention, Some(TraceRetention::Final));
    assert_eq!(finish(&mut owner), Progress::Committed);
    assert_eq!(helper.observe(record).unwrap(), current);
    assert_eq!(helper.phase().unwrap().domains[0].pending, 1);
}

#[test]
fn failed_trace_membership_load_keeps_a_valid_shade_request_pending() {
    let catalog = BoundedCatalog::new(Capacities::derive(1, 1, 1).unwrap()).unwrap();
    let mut owner = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut owner, 0);
    assert_eq!(
        run(&mut owner, Command::TraceInsert(record)),
        Progress::Committed
    );
    let before = helper.phase().unwrap();
    owner
        .submit(Command::Shade {
            key: record,
            domain: 0,
            retention: crate::TraceRetention::Strong,
        })
        .unwrap();
    crate::FAIL_LOAD.with(|counter| counter.set(Some(2)));
    assert_eq!(owner.poll(), Progress::Pending);
    assert_eq!(helper.phase().unwrap(), before);
    assert_eq!(finish(&mut owner), Progress::Committed);
    assert_eq!(helper.phase().unwrap().domains[0].pending, 1);
}

fn advance_cycle(participant: &mut Participant, domain: usize) -> Progress {
    let entry = participant.phase().unwrap().domains[domain];
    if entry.floor == crate::RETIREMENT_FLOOR {
        assert_eq!(
            run(participant, Command::Finish { domain }),
            Progress::Committed
        );
        run(
            participant,
            Command::Start {
                domain,
                mode: entry.mode,
            },
        )
    } else {
        run(participant, Command::Advance { domain })
    }
}

#[test]
fn facade_publication_and_promotion_preserve_both_domain_obligations() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    for _ in 0..crate::RETIREMENT_FLOOR {
        assert_eq!(
            run(&mut p, Command::Advance { domain: 1 }),
            Progress::Committed
        );
    }
    assert_eq!(
        run(&mut p, Command::Finish { domain: 1 }),
        Progress::Committed
    );
    assert_eq!(
        run(
            &mut p,
            Command::Start {
                domain: 1,
                mode: crate::DomainMode::Old
            }
        ),
        Progress::Committed
    );
    let record = create(&mut p, 0);
    assert_eq!(run(&mut p, Command::Publish(record)), Progress::Committed);
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [1, 0]);
    assert_eq!(run(&mut p, Command::Promote(record)), Progress::Committed);
    assert_eq!(p.phase().unwrap().domains.map(|d| d.pending), [1, 1]);
    let Membership::Tracing(state) = p.observe(record).unwrap().membership else {
        panic!()
    };
    assert_eq!(state.generation, crate::Generation::Old);
    assert_eq!(state.old_birth, 2);
    assert_eq!(
        run(&mut p, Command::Promote(record)),
        Progress::Rejected(RequestError::Conflict)
    );
    for domain in 0..2 {
        assert_eq!(
            run(&mut p, Command::Advance { domain }),
            Progress::Rejected(RequestError::Conflict)
        );
        assert_eq!(
            run(
                &mut p,
                Command::TraceAcknowledge {
                    key: record,
                    domain,
                    ticket: state[domain].ticket
                }
            ),
            Progress::Committed
        );
        assert_eq!(
            run(&mut p, Command::Advance { domain }),
            Progress::Committed
        );
    }
}

#[test]
fn facade_rejects_inactive_discovery_and_invalid_domain_lifecycle_without_spinning() {
    let catalog = fixture(1, 1);
    let mut p = catalog.register_mutator().unwrap();
    assert_eq!(
        run(&mut p, Command::Finish { domain: 0 }),
        Progress::Rejected(RequestError::Conflict)
    );
    for _ in 0..crate::RETIREMENT_FLOOR {
        assert_eq!(
            run(&mut p, Command::Advance { domain: 0 }),
            Progress::Committed
        );
    }
    assert_eq!(
        run(&mut p, Command::Advance { domain: 0 }),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        run(&mut p, Command::Finish { domain: 0 }),
        Progress::Committed
    );
    let record = create(&mut p, 0);
    assert_eq!(run(&mut p, Command::Publish(record)), Progress::Committed);
    assert_eq!(
        run(
            &mut p,
            Command::Shade {
                key: record,
                domain: 0,
                retention: crate::TraceRetention::Strong
            }
        ),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        run(
            &mut p,
            Command::Start {
                domain: 0,
                mode: crate::DomainMode::Old
            }
        ),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(
        run(
            &mut p,
            Command::Start {
                domain: 2,
                mode: crate::DomainMode::Full
            }
        ),
        Progress::Rejected(RequestError::InvalidDomain)
    );
    assert_eq!(
        run(
            &mut p,
            Command::Start {
                domain: 0,
                mode: crate::DomainMode::Young
            }
        ),
        Progress::Committed
    );
}

#[test]
fn supplemental_storage_stays_reserved_through_the_last_participant() {
    let capacities = Capacities::derive(3, 2, 1).unwrap();
    let base = Footprint::for_records::<()>(capacities).unwrap();
    let supplemental = 4096;
    let total = base.total() + supplemental;
    let short = Budget::new(total - 1);
    assert!(matches!(
        BoundedCatalog::<()>::with_records_and_storage_in(capacities, &short, supplemental),
        Err(ConfigError::BudgetExceeded)
    ));
    assert_eq!(short.used(), 0);
    let budget = Budget::new(total);
    let catalog =
        BoundedCatalog::<()>::with_records_and_storage_in(capacities, &budget, supplemental)
            .unwrap();
    assert_eq!(catalog.footprint(), base);
    assert_eq!(catalog.charged_bytes(), total);
    assert_eq!(catalog.reserved_storage_bytes(), supplemental);
    let participant = catalog.register_helper().unwrap();
    drop(catalog);
    assert_eq!(budget.used(), total);
    drop(participant);
    assert_eq!(budget.used(), 0);
}

#[test]
fn supplemental_storage_overflow_does_not_reserve_any_budget() {
    let capacities = Capacities::derive(1, 0, 1).unwrap();
    let budget = Budget::new(usize::MAX);
    assert!(matches!(
        BoundedCatalog::<()>::with_records_and_storage_in(capacities, &budget, usize::MAX),
        Err(ConfigError::CapacityOverflow)
    ));
    assert_eq!(budget.used(), 0);
}

#[test]
fn phase_only_facade_requests_yield_twice_without_allocating_or_publishing() {
    for scan in [false, true] {
        let capacities = Capacities::derive(1, 1, 1)
            .unwrap()
            .with_roots(if scan { 2 } else { 0 })
            .unwrap();
        let catalog = BoundedCatalog::new(capacities).unwrap();
        let before = catalog.raw.snapshot().unwrap().phase();
        let mut worker = catalog.register_mutator().unwrap();
        if scan {
            worker
                .submit_root(RootCommand::ScanBatch { domain: 0 })
                .unwrap();
        } else {
            worker.submit(Command::Advance { domain: 0 }).unwrap();
        }
        assert_eq!(worker.poll(), Progress::Pending);
        assert_eq!(worker.poll(), Progress::Pending);
        assert_eq!(catalog.raw.pool_occupancy(), (2, 0));
        for _ in 0..2 {
            assert_eq!(
                worker.poll(),
                Progress::Pending,
                "phase request did not yield before installation"
            );
            assert_eq!(
                catalog.raw.snapshot().unwrap().phase(),
                before,
                "phase published during its preparation window"
            );
            assert_eq!(
                catalog.raw.pool_occupancy(),
                (2, 0),
                "phase yield allocated another controller"
            );
        }
        assert_eq!(worker.poll(), Progress::Committed);
        let after = catalog.raw.snapshot().unwrap().phase();
        assert_eq!(after.revision, before.revision + 1);
        if scan {
            assert_eq!(after.root_cursors[0], 2);
        } else {
            assert_eq!(after.domains[0].floor, 1);
        }
        assert_empty(&catalog);
    }
}

#[test]
fn phase_yield_cancellation_and_drop_release_every_private_prefix() {
    for scan in [false, true] {
        for prefix in 0..=2 {
            for discard in [false, true] {
                let capacities = Capacities::derive(1, 1, 1)
                    .unwrap()
                    .with_roots(if scan { 2 } else { 0 })
                    .unwrap();
                let catalog = BoundedCatalog::new(capacities).unwrap();
                let before = catalog.raw.snapshot().unwrap().phase();
                let mut worker = catalog.register_mutator().unwrap();
                if scan {
                    worker
                        .submit_root(RootCommand::ScanBatch { domain: 0 })
                        .unwrap();
                } else {
                    worker.submit(Command::Advance { domain: 0 }).unwrap();
                }
                for _ in 0..2 + prefix {
                    assert_eq!(worker.poll(), Progress::Pending);
                }
                assert_eq!(catalog.raw.pool_occupancy(), (2, 0));
                if discard {
                    worker.discard_request();
                } else {
                    assert!(worker.cancel());
                }
                assert_eq!(worker.poll(), Progress::Idle);
                assert_eq!(catalog.raw.snapshot().unwrap().phase(), before);
                assert_eq!(catalog.registered(), (1, 0));
                assert_empty(&catalog);
                drop(worker);
                assert_eq!(catalog.registered(), (0, 0));
            }
            // Exercise automatic request destruction at the same private prefix.
            let capacities = Capacities::derive(1, 1, 1)
                .unwrap()
                .with_roots(if scan { 2 } else { 0 })
                .unwrap();
            let catalog = BoundedCatalog::new(capacities).unwrap();
            let before = catalog.raw.snapshot().unwrap().phase();
            let mut worker = catalog.register_mutator().unwrap();
            if scan {
                worker
                    .submit_root(RootCommand::ScanBatch { domain: 0 })
                    .unwrap();
            } else {
                worker.submit(Command::Advance { domain: 0 }).unwrap();
            }
            for _ in 0..2 + prefix {
                assert_eq!(worker.poll(), Progress::Pending);
            }
            drop(worker);
            assert_eq!(catalog.raw.snapshot().unwrap().phase(), before);
            assert_eq!(catalog.registered(), (0, 0));
            assert_empty(&catalog);
        }
    }
}

#[test]
fn a_stopped_phase_yield_cannot_block_a_helper_or_finish_its_restarted_epoch() {
    fn run_guarded(
        worker: &mut Participant<(), GuardedCommand>,
        command: GuardedCommand,
    ) -> Progress {
        worker.submit(command).unwrap();
        for _ in 0..4096 {
            let result = worker.poll_in(&());
            if result != Progress::Pending {
                return result;
            }
        }
        panic!("guarded phase fixture did not finish");
    }
    let catalog = fixture(1, 1);
    let mut helper = catalog.register_record_helper().unwrap();
    for _ in 0..crate::RETIREMENT_FLOOR {
        assert_eq!(
            run_guarded(&mut helper, GuardedCommand::Advance { domain: 0 }),
            Progress::Committed
        );
    }
    let epoch = helper.phase_in(&()).unwrap().domains[0].epoch;
    let mut stopped = catalog.register_record_mutator().unwrap();
    stopped
        .submit(GuardedCommand::FinishEpoch { domain: 0, epoch })
        .unwrap();
    assert_eq!(stopped.poll_in(&()), Progress::Pending);
    assert_eq!(stopped.poll_in(&()), Progress::Pending);
    assert!(matches!(stopped.state, RequestState::Yielding(..)));
    assert_eq!(
        run_guarded(
            &mut helper,
            GuardedCommand::FinishEpoch { domain: 0, epoch }
        ),
        Progress::Committed
    );
    assert_eq!(
        run_guarded(
            &mut helper,
            GuardedCommand::Start {
                domain: 0,
                mode: crate::DomainMode::Full
            }
        ),
        Progress::Committed
    );
    let current = helper.phase_in(&()).unwrap();
    assert_eq!(current.domains[0].epoch, epoch + 1);
    for _ in 0..2 {
        assert_eq!(stopped.poll_in(&()), Progress::Pending);
    }
    assert_eq!(stopped.poll_in(&()), Progress::Pending); // Original expected head lost.
    assert_eq!(helper.phase_in(&()).unwrap(), current);
    assert_eq!(
        stopped.poll_in(&()),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(helper.phase_in(&()).unwrap(), current);
    assert_empty(&catalog);
}

#[test]
fn effect_bearing_facade_preparation_prepares_one_successor_before_installation() {
    let catalog = fixture(1, 1);
    let mut worker = catalog.register_mutator().unwrap();
    let key = create(&mut worker, 0);
    worker.submit(Command::Insert(key)).unwrap();
    assert_eq!(worker.poll(), Progress::Pending);
    for _ in 0..2 {
        assert_eq!(worker.poll(), Progress::Pending);
    }
    let RequestState::Attempt(_, attempt) = &worker.state else {
        panic!("effect preparation gained an extra delay")
    };
    assert!(!attempt.is_phase_only());
    assert_eq!(catalog.raw.pool_occupancy().0, 3);
    assert_eq!(worker.poll(), Progress::Committed);
    assert!(matches!(
        worker.observe(key).unwrap().membership,
        Membership::Live(_)
    ));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn passive_observation_parked_busy_allows_real_help_and_pool_reuse() {
    use crate::state::observation_tests::AFTER_ACQUIRE;
    use std::sync::mpsc::channel;
    use std::time::Duration;
    let catalog = fixture(2, 1);
    let mut owner = catalog.register_mutator().unwrap();
    let mut observer = catalog.register_mutator().unwrap();
    let mut helper = catalog.register_helper().unwrap();
    let record = create(&mut owner, 0);
    prepare_attempt(&mut owner, Command::Publish(record));
    install(&owner);
    let (acquired_tx, acquired_rx) = channel();
    let (release_tx, release_rx) = channel();
    std::thread::scope(|scope| {
        let parked = scope.spawn(move || {
            AFTER_ACQUIRE.with(|slot| {
                *slot.borrow_mut() = Some(Box::new(move || {
                    acquired_tx.send(()).unwrap();
                    release_rx.recv_timeout(Duration::from_secs(30)).unwrap();
                }))
            });
            observer.possible_phase_outcomes().unwrap()
        });
        acquired_rx.recv_timeout(Duration::from_secs(30)).unwrap();
        // This real helper drives the installed record-bearing operation while
        // the observer retains its original Busy controller and children.
        assert_eq!(helper.phase_in(&()), Err(RequestError::Contended));
        assert!(helper.phase_in(&()).is_ok());
        assert_eq!(finish(&mut owner), Progress::Committed);
        assert!(matches!(
            owner.observe(record).unwrap().membership,
            Membership::Tracing(_)
        ));
        assert_bounded(&catalog);
        // Complete real tracing effects while the older Busy node stays pinned.
        let Membership::Tracing(marks) = owner.observe(record).unwrap().membership else {
            panic!("tracing publication")
        };
        for domain in [0, 1] {
            assert_eq!(
                run(
                    &mut helper,
                    Command::TraceAcknowledge {
                        key: record,
                        domain,
                        ticket: marks[domain].ticket,
                    }
                ),
                Progress::Committed
            );
            assert_bounded(&catalog);
        }
        release_tx.send(()).unwrap();
        let possible = parked.join().unwrap();
        assert_ne!(
            possible[0].domains[0].pending,
            possible[1].domains[0].pending
        );
    });
    assert_bounded(&catalog);
    drop((owner, helper));
    assert_eq!(catalog.registered(), (0, 0));
}
