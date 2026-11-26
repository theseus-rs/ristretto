use super::*;
use crate::record_fixture::Fixture;
fn catalog() -> Catalog<Fixture> {
    Catalog::with_record_root_capacities_and_limit(2, 256, 256, 128, 2, 64, pool::MAX_GENERATION)
}
fn snapshot(c: &Catalog<Fixture>) -> Snapshot<Fixture> {
    for _ in 0..16 {
        if let Some(s) = c.snapshot_in(&7) {
            return s;
        }
    }
    panic!("root controller did not settle");
}
fn cell(c: &Catalog<Fixture>) -> Pooled<Cell<Fixture>> {
    let mut create = c.begin_create_with(0, 8, Fixture::new()).unwrap();
    for _ in 0..16 {
        if let Some(cell) = create.poll(c).unwrap() {
            return cell;
        }
    }
    panic!("cell did not announce");
}
fn root(c: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>) -> RootToken<Fixture> {
    let mut create = c.begin_create_root(0, cell.try_clone().unwrap()).unwrap();
    for _ in 0..16 {
        if let Some(root) = create.poll(c).unwrap() {
            return root;
        }
    }
    panic!("root did not announce");
}
fn prepare(c: &Catalog<Fixture>, p: Option<Preparation<Fixture>>) -> Attempt<Fixture> {
    let mut p = p.expect("valid root preparation");
    for _ in 0..16 {
        if let Some(a) = p.poll(c).unwrap() {
            return a;
        }
    }
    panic!("root preparation did not settle");
}
fn commit(c: &Catalog<Fixture>, p: Option<Preparation<Fixture>>) {
    let a = prepare(c, p);
    for _ in 0..16 {
        match c.commit_in(&a, &7) {
            Outcome::Pending => {}
            Outcome::Committed => return,
            other => panic!("root commit: {other:?}"),
        }
    }
    panic!("root commit did not settle");
}
fn drain(c: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>) {
    let Some(Membership::Tracing(marks)) = cell.membership() else {
        panic!("not tracing")
    };
    for domain in 0..2 {
        if marks[domain].ticket != 0 {
            commit(
                c,
                c.begin_trace_acknowledge(
                    snapshot(c),
                    cell.try_clone().unwrap(),
                    domain,
                    marks[domain].ticket,
                ),
            );
        }
    }
}
fn scan(c: &Catalog<Fixture>, domain: usize) {
    for _ in 0..c.root_capacity() {
        commit(c, c.begin_scan_root(snapshot(c), domain));
    }
}

#[test]
fn root_publication_and_scan_gate_close_both_visibility_and_work() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell);
    assert_eq!(root.status(), RootStatus::Prepared);
    assert!(!c.release_retired_root(&root.try_share().unwrap()));
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert_eq!(root.status(), RootStatus::Live);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 1]);
    drain(&c, &cell);
    assert!(
        c.begin_advance(snapshot(&c), 0).is_none(),
        "strong closed before roots were enumerated"
    );
    scan(&c, 0);
    assert_eq!(snapshot(&c).phase().root_cursors, [2, 0]);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    assert!(c.begin_advance(snapshot(&c), 1).is_none());
    scan(&c, 1);
    commit(&c, c.begin_advance(snapshot(&c), 1));
    assert_eq!(root.status(), RootStatus::Live);
}

#[test]
fn a_helper_between_object_and_root_effects_can_be_completed_without_its_stack() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell);
    let attempt = prepare(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert!(c.install_and_stop(&attempt));
    c.help_record_at(&7, 1, || {
        assert!(matches!(cell.membership(), Some(Membership::Tracing(_))));
        assert_eq!(root.status(), RootStatus::Prepared);
        let phase = snapshot(&c).phase();
        assert_eq!(
            root.status(),
            RootStatus::Live,
            "phase escaped an incomplete root effect"
        );
        assert_eq!(phase.domains.map(|d| d.pending), [1, 1]);
    });
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Committed);
    assert_eq!(root.status(), RootStatus::Live);
}

#[test]
fn late_root_publication_cannot_restore_a_retired_entry_after_slot_reuse() {
    let c = catalog();
    let cell = cell(&c);
    let token = root(&c, &cell);
    let old = token.try_share().unwrap();
    let attempt = prepare(
        &c,
        c.begin_publish_root(snapshot(&c), old.try_clone().unwrap()),
    );
    assert!(c.install_and_stop(&attempt));
    let mut replacement = None;
    c.help_record_at(&7, 1, || {
        snapshot(&c);
        commit(
            &c,
            c.begin_remove_root(snapshot(&c), old.try_clone().unwrap()),
        );
        assert_eq!(old.status(), RootStatus::Retired);
        drop(token);
        assert!(c.release_retired_root(&old));
        let next = root(&c, &cell);
        assert_ne!(next.identity, old.identity);
        replacement = Some(next);
    });
    assert_eq!(
        old.status(),
        RootStatus::Retired,
        "late helper revived an old root"
    );
    let replacement = replacement.unwrap();
    assert_eq!(
        replacement.status(),
        RootStatus::Prepared,
        "late helper activated slot reuse"
    );
    assert!(!c.release_retired_root(&old));
    assert_eq!(
        c.discover_root(0).unwrap().unwrap().identity,
        replacement.identity
    );
}

#[test]
fn cancelled_root_publication_leaves_both_effects_private() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell);
    let attempt = prepare(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert!(c.install_and_stop(&attempt));
    assert!(attempt.cancel());
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Cancelled);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
    assert_eq!(root.status(), RootStatus::Prepared);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [0, 0]);
    commit(
        &c,
        c.begin_cancel_root(snapshot(&c), root.try_share().unwrap()),
    );
    let retained = root.try_share().unwrap();
    drop(root);
    assert!(c.release_retired_root(&retained));
}

#[test]
fn a_live_root_is_rediscovered_in_the_next_epoch_before_retirement() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    drain(&c, &cell);
    scan(&c, 0);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Full));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 0);
    assert!(c.begin_advance(snapshot(&c), 0).is_none());
    scan(&c, 0);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
    drain(&c, &cell);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    assert!(
        c.begin_trace_retire(snapshot(&c), cell.try_clone().unwrap(), 0)
            .is_none()
    );
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert_eq!(root.status(), RootStatus::Retired);
}

#[test]
fn live_token_prevents_retired_slot_reuse_until_durable_drop() {
    let c = catalog();
    let cell = cell(&c);
    let token = root(&c, &cell);
    let owned = token.try_share().unwrap();
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), owned.try_clone().unwrap()),
    );
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), owned.try_clone().unwrap()),
    );
    assert_eq!(token.status(), RootStatus::Retired);
    assert!(
        !c.release_retired_root(&owned),
        "retired slot reused while its token is still held"
    );
    assert!(!owned.removal_requested());
    drop(token);
    assert!(owned.removal_requested());
    assert!(c.release_retired_root(&owned));
}

#[test]
fn dropping_a_root_posts_removal_without_dropping_its_required_deletion_edge() {
    let c = catalog();
    let cell = cell(&c);
    let token = root(&c, &cell);
    let owned = token.try_share().unwrap();
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), owned.try_clone().unwrap()),
    );
    drain(&c, &cell);
    scan(&c, 0);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Full));
    drop(token);
    assert!(owned.removal_requested());
    assert_eq!(
        owned.status(),
        RootStatus::Live,
        "drop bypassed canonical deletion"
    );
    assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
    commit(&c, c.begin_scan_root(snapshot(&c), 0));
    assert_eq!(owned.status(), RootStatus::Retired);
    assert_eq!(
        snapshot(&c).phase().domains[0].pending,
        1,
        "root drop lost a snapshot edge"
    );
    assert!(c.release_retired_root(&owned));
    assert!(c.begin_advance(snapshot(&c), 0).is_none());
}

#[test]
fn a_dropped_prepared_root_can_be_cancelled_before_activation() {
    let c = catalog();
    let cell = cell(&c);
    let token = root(&c, &cell);
    let owned = token.try_share().unwrap();
    assert!(
        c.begin_requested_root_removal(snapshot(&c), owned.try_clone().unwrap())
            .is_none()
    );
    drop(token);
    commit(
        &c,
        c.begin_requested_root_removal(snapshot(&c), owned.try_clone().unwrap()),
    );
    assert_eq!(owned.status(), RootStatus::Retired);
    assert_eq!(cell.membership(), Some(Membership::Prepared));
    assert!(
        c.begin_publish_root(snapshot(&c), owned.try_clone().unwrap())
            .is_none()
    );
    assert!(c.release_retired_root(&owned));
}

#[test]
fn root_barrier_floor_table_preserves_generational_exclusions_and_births() {
    use crate::state::root_barrier_plan;
    for mode in [
        DomainMode::Inactive,
        DomainMode::Young,
        DomainMode::Full,
        DomainMode::Old,
    ] {
        for generation in [Generation::Young, Generation::Old] {
            for floor in 0..=RETIREMENT_FLOOR {
                for born in [false, true] {
                    let phase = Phase {
                        revision: 10,
                        root_cursors: [0, 0],
                        reference_cursors: [0; 2],
                        domains: [
                            Domain {
                                mode: DomainMode::Inactive,
                                epoch: 2,
                                floor: 0,
                                pending: 0,
                            },
                            Domain {
                                mode,
                                epoch: 7,
                                floor,
                                pending: 0,
                            },
                        ],
                    };
                    let mut state = TraceState::unmarked(phase);
                    state.generation = generation;
                    state.marks[1].born = born;
                    state.old_birth = if mode == DomainMode::Old && born {
                        7
                    } else {
                        0
                    };
                    let membership = Membership::Tracing(state);
                    let result = root_barrier_plan(membership, phase, false);
                    let collected = mode == DomainMode::Full
                        || (mode == DomainMode::Young && generation == Generation::Young)
                        || (mode == DomainMode::Old && generation == Generation::Old);
                    if floor == RETIREMENT_FLOOR && collected && !born {
                        assert_eq!(result, Err(TraceError::Conflict));
                        assert_eq!(
                            root_barrier_plan(membership, phase, true),
                            Ok((membership, phase))
                        );
                        continue;
                    }
                    let excluded = mode == DomainMode::Inactive
                        || floor == RETIREMENT_FLOOR
                        || (mode == DomainMode::Young && generation == Generation::Old)
                        || (mode == DomainMode::Old && generation == Generation::Young && born);
                    let (Membership::Tracing(after), changed) = result.unwrap() else {
                        panic!();
                    };
                    if excluded {
                        assert_eq!(after, state);
                        assert_eq!(changed, phase);
                    } else {
                        let expected = match floor {
                            0 => TraceRetention::Strong,
                            1 | 2 => TraceRetention::Final,
                            3 | 4 => TraceRetention::Phantom,
                            _ => unreachable!(),
                        };
                        assert_eq!(after[1].retention, Some(expected));
                        let queued = !(mode == DomainMode::Old && generation == Generation::Young);
                        assert_eq!(changed.domains[1].pending, usize::from(queued));
                        assert_eq!(after[1].ticket, if queued { 11 } else { 0 });
                        assert!(!after[1].born);
                        assert_eq!(after.old_birth, state.old_birth);
                    }
                    assert_eq!(changed.domains[0], phase.domains[0]);
                }
            }
        }
    }
}

#[test]
fn root_activation_cannot_shade_across_other_domain_retirement_or_counter_exhaustion() {
    use crate::state::root_barrier_plan;
    let mut phase = Phase {
        revision: 10,
        root_cursors: [0, 0],
        reference_cursors: [0; 2],
        domains: [
            Domain {
                mode: DomainMode::Full,
                epoch: 2,
                floor: RETIREMENT_FLOOR,
                pending: 0,
            },
            Domain {
                mode: DomainMode::Full,
                epoch: 7,
                floor: 0,
                pending: 0,
            },
        ],
    };
    let membership = Membership::Tracing(TraceState::unmarked(phase));
    assert_eq!(
        root_barrier_plan(membership, phase, false),
        Err(TraceError::Conflict)
    );
    assert_eq!(
        root_barrier_plan(membership, phase, true),
        Ok((membership, phase))
    );
    phase.domains[0].mode = DomainMode::Inactive;
    phase.domains[1].pending = usize::MAX;
    assert_eq!(
        root_barrier_plan(membership, phase, false),
        Err(TraceError::Exhausted)
    );
    phase.domains[1].pending = 0;
    phase.revision = u64::MAX;
    assert_eq!(
        root_barrier_plan(membership, phase, true),
        Err(TraceError::Exhausted)
    );
}

fn batch_catalog(roots: usize) -> Catalog<Fixture> {
    Catalog::with_record_root_capacities_and_limit(
        2,
        256,
        256,
        128,
        roots,
        64,
        pool::MAX_GENERATION,
    )
}
fn batch_root_at(
    c: &Catalog<Fixture>,
    target: &Pooled<Cell<Fixture>>,
    slot: usize,
) -> RootToken<Fixture> {
    let mut creation = c
        .begin_create_root(slot, target.try_clone().unwrap())
        .unwrap();
    for _ in 0..16 {
        if let Some(root) = creation.poll(c).unwrap() {
            return root;
        }
    }
    panic!("batch root did not announce");
}

#[test]
fn scan_batch_bounds_empty_ranges_and_preserves_single_slot_api() {
    let c = batch_catalog(20);
    commit(&c, c.begin_scan_root(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 1);
    for cursor in [9, 17, 20] {
        commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
        assert_eq!(snapshot(&c).phase().root_cursors[0], cursor);
        assert_eq!(snapshot(&c).phase().root_cursors[1], 0);
    }
    assert!(c.begin_scan_root_batch(snapshot(&c), 0).is_none());
    assert!(c.begin_scan_root_batch(snapshot(&c), 2).is_none());
    commit(&c, c.begin_advance(snapshot(&c), 0));
    assert!(c.begin_scan_root_batch(snapshot(&c), 0).is_none());
}

#[test]
fn scan_batch_failed_owning_observation_never_publishes_partial_cursor() {
    let c = batch_catalog(20);
    for failed_load in 0..8 {
        let current = snapshot(&c);
        FAIL_LOAD.with(|count| count.set(Some(failed_load)));
        assert!(c.begin_scan_root_batch(current, 0).is_none());
        assert_eq!(snapshot(&c).phase().root_cursors[0], 0);
    }
    let attempt = prepare(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert!(attempt.cancel());
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Cancelled);
    assert_eq!(snapshot(&c).phase().root_cursors[0], 0);
    commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 8);
}

#[test]
fn scan_batch_late_publication_behind_cursor_preserves_both_domain_work() {
    let c = batch_catalog(8);
    let target = cell(&c);
    let root = batch_root_at(&c, &target, 3);
    commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 8);
    assert_eq!(root.status(), RootStatus::Prepared);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert_eq!(root.status(), RootStatus::Live);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 1]);
    assert!(c.begin_advance(snapshot(&c), 0).is_none());
    drain(&c, &target);
    commit(&c, c.begin_advance(snapshot(&c), 0));
}

#[test]
fn scan_batch_concurrent_publication_supersedes_private_cursor_proposal() {
    let c = batch_catalog(8);
    let target = cell(&c);
    let root = batch_root_at(&c, &target, 3);
    let attempt = prepare(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Retry);
    assert_eq!(snapshot(&c).phase().root_cursors[0], 0);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 1]);
    // Fully shaded targets need no independent owner. Keep the next fallible
    // share armed across preparation to prove that path never needed it.
    let current = snapshot(&c);
    FAIL_SHARE.with(|fault| fault.set(Some(0)));
    let scan = c.begin_scan_root_batch(current, 0);
    assert_eq!(FAIL_SHARE.with(|fault| fault.take()), Some(0));
    commit(&c, scan);
    assert_eq!(snapshot(&c).phase().root_cursors[0], 8);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 1]);
    assert_eq!(root.status(), RootStatus::Live);
}

#[test]
fn scan_batch_stops_at_prepared_cancellation_effect() {
    let c = batch_catalog(20);
    let target = cell(&c);
    let root = batch_root_at(&c, &target, 3);
    let retained = root.try_share().unwrap();
    drop(root);
    commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 4);
    assert_eq!(retained.status(), RootStatus::Retired);
    assert_eq!(target.membership(), Some(Membership::Prepared));
    commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 12);
}

#[test]
fn scan_batch_stops_at_live_removal_without_revoking_later_root() {
    let c = batch_catalog(20);
    let target = cell(&c);
    let first = batch_root_at(&c, &target, 3);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), first.try_share().unwrap()),
    );
    let second = batch_root_at(&c, &target, 5);
    commit(
        &c,
        c.begin_activate_root(snapshot(&c), second.try_share().unwrap()),
    );
    let retained = first.try_share().unwrap();
    drop(first);
    commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 4);
    assert_eq!(retained.status(), RootStatus::Retired);
    assert_eq!(second.status(), RootStatus::Live);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 1]);
}

#[test]
fn scan_batch_stops_at_new_epoch_shading_effect_before_later_slots() {
    let c = batch_catalog(20);
    let target = cell(&c);
    let root = batch_root_at(&c, &target, 3);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    drain(&c, &target);
    while snapshot(&c).phase().root_cursors[0] < c.root_capacity() {
        commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    }
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Full));
    assert_eq!(snapshot(&c).phase().domains[0].epoch, 2);
    // A real shading effect must retain the target before the cursor can move.
    let current = snapshot(&c);
    FAIL_SHARE.with(|fault| fault.set(Some(0)));
    assert!(c.begin_scan_root_batch(current, 0).is_none());
    assert_eq!(FAIL_SHARE.with(|fault| fault.take()), None);
    assert_eq!(snapshot(&c).phase().root_cursors[0], 0);
    commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().root_cursors[0], 4);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 0]);
    assert_eq!(root.status(), RootStatus::Live);
    assert!(c.begin_advance(snapshot(&c), 0).is_none());
}

#[test]
fn stable_root_observation_rejects_failed_loads_and_foreign_catalogs() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell);
    assert_eq!(c.root_is_stably_live_in(&root, &7), Some(false));
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert_eq!(c.root_is_stably_live_in(&root, &7), Some(true));
    let foreign = catalog();
    assert_eq!(
        foreign.root_is_stably_live_in(&root, &7),
        None,
        "foreign root produced a stable certificate"
    );
    FAIL_LOAD.with(|slot| slot.set(Some(0)));
    assert_eq!(c.root_is_stably_live_in(&root, &7), None);
    FAIL_LOAD.with(|slot| slot.set(Some(1)));
    assert_eq!(c.root_is_stably_live_in(&root, &7), Some(true));
    assert_eq!(FAIL_LOAD.with(|slot| slot.take()), Some(0));
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert_eq!(c.root_is_stably_live_in(&root, &7), Some(false));
    cell.record().release_origin();
}
