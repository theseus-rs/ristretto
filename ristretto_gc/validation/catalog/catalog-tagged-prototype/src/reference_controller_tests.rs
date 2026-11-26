use crate::record_fixture::Fixture;
use crate::reference::{ObjectKey, ReferenceKind};
use crate::*;
fn catalog() -> Catalog<Fixture> {
    Catalog::with_record_root_capacities_and_limit(2, 256, 256, 128, 2, 64, pool::MAX_GENERATION)
        .with_references(2, 64, 256, pool::MAX_GENERATION)
}
fn snapshot(c: &Catalog<Fixture>) -> Snapshot<Fixture> {
    (0..16)
        .find_map(|_| c.snapshot_in(&7))
        .expect("settled controller")
}
fn cell(c: &Catalog<Fixture>) -> Pooled<Cell<Fixture>> {
    let mut create = c.begin_create_with(0, 8, Fixture::new()).unwrap();
    (0..16)
        .find_map(|_| create.poll(c).unwrap())
        .expect("cell created")
}
fn root(c: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>, slot: usize) -> RootToken<Fixture> {
    let mut create = c
        .begin_create_root(slot, cell.try_clone().unwrap())
        .unwrap();
    (0..16)
        .find_map(|_| create.poll(c).unwrap())
        .expect("root created")
}
fn reference(c: &Catalog<Fixture>, slot: usize) -> ReferenceToken<Fixture> {
    let mut create: ReferenceCreation<Fixture> = c.begin_create_reference(slot).unwrap();
    (0..16)
        .find_map(|_| create.poll(c).unwrap())
        .expect("reference created")
}
fn prepare(c: &Catalog<Fixture>, p: Option<Preparation<Fixture>>) -> Attempt<Fixture> {
    let mut p = p.expect("valid preparation");
    (0..16)
        .find_map(|_| p.poll(c).unwrap())
        .expect("prepared attempt")
}
fn commit(c: &Catalog<Fixture>, p: Option<Preparation<Fixture>>) {
    let a = prepare(c, p);
    assert_eq!(c.commit_in(&a, &7), Outcome::Committed);
}
fn publish(c: &Catalog<Fixture>, root: &RootToken<Fixture>, reference: &ReferenceToken<Fixture>) {
    commit(
        c,
        c.begin_publish_root(snapshot(c), root.try_share().unwrap()),
    );
    commit(
        c,
        c.begin_publish_reference(
            snapshot(c),
            reference.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Weak,
        ),
    );
}

#[test]
fn reference_publication_and_clear_preserve_exact_target_identity() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let reference = reference(&c, 0);
    publish(&c, &root, &reference);
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!();
    };
    assert_eq!(
        state.target,
        Some(ObjectKey {
            identity: cell.identity,
            slot: cell.slot()
        })
    );
    assert_eq!(state.owner, None);
    assert_eq!(state.born_after, [0, 0]);
    commit(
        &c,
        c.begin_clear_reference(snapshot(&c), reference.try_share().unwrap()),
    );
    let ReferenceMembership::Live(cleared) = reference.membership().unwrap() else {
        panic!();
    };
    assert_eq!(cleared.target, None);
    assert_eq!(cleared.kind, state.kind);
    assert_eq!(root.status(), RootStatus::Live);
}

#[test]
fn clear_and_upgrade_cannot_both_use_the_same_old_controller() {
    for clear_first in [false, true] {
        let c = catalog();
        let cell = cell(&c);
        let source_root = root(&c, &cell, 0);
        let reference = reference(&c, 0);
        publish(&c, &source_root, &reference);
        let upgraded = root(&c, &cell, 1);
        let upgrade = prepare(
            &c,
            c.begin_upgrade_reference(
                snapshot(&c),
                reference.try_share().unwrap(),
                upgraded.try_share().unwrap(),
            ),
        );
        let clear = prepare(
            &c,
            c.begin_clear_reference(snapshot(&c), reference.try_share().unwrap()),
        );
        let (first, stale) = if clear_first {
            (&clear, &upgrade)
        } else {
            (&upgrade, &clear)
        };
        assert_eq!(c.commit_in(first, &7), Outcome::Committed);
        assert_eq!(c.commit_in(stale, &7), Outcome::Retry);
        if clear_first {
            assert_eq!(upgraded.status(), RootStatus::Prepared);
            assert!(
                c.begin_upgrade_reference(
                    snapshot(&c),
                    reference.try_share().unwrap(),
                    upgraded.try_share().unwrap()
                )
                .is_none()
            );
        } else {
            assert_eq!(upgraded.status(), RootStatus::Live);
            commit(
                &c,
                c.begin_clear_reference(snapshot(&c), reference.try_share().unwrap()),
            );
            assert_eq!(upgraded.status(), RootStatus::Live);
        }
    }
}

#[test]
fn failed_reference_effect_share_leaves_installed_work_cancellable() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let reference = reference(&c, 0);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    let publish = prepare(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            reference.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Weak,
        ),
    );
    FAIL_SHARE.with(|fail| fail.set(Some(1)));
    assert_eq!(c.commit_in(&publish, &7), Outcome::Pending);
    assert!(
        publish.cancel(),
        "reference effect share happened after COMMIT"
    );
    assert_eq!(c.commit_in(&publish, &7), Outcome::Cancelled);
    assert_eq!(reference.membership(), Some(ReferenceMembership::Prepared));
}

#[test]
fn failed_target_discovery_is_not_a_cleared_clone() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let source = reference(&c, 0);
    let destination = reference(&c, 1);
    publish(&c, &root, &source);
    let current = snapshot(&c);
    FAIL_LOAD.with(|fail| fail.set(Some(2)));
    assert!(
        c.begin_clone_reference(
            current,
            source.try_share().unwrap(),
            destination.try_share().unwrap()
        )
        .is_none()
    );
    assert_eq!(
        destination.membership(),
        Some(ReferenceMembership::Prepared)
    );
    commit(
        &c,
        c.begin_clone_reference(
            snapshot(&c),
            source.try_share().unwrap(),
            destination.try_share().unwrap(),
        ),
    );
    let ReferenceMembership::Live(clone) = destination.membership().unwrap() else {
        panic!();
    };
    assert!(clone.target.is_some());
}

#[test]
fn a_late_reference_effect_cannot_restore_cleared_or_reused_slot_state() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let old = reference(&c, 0);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    let publish = prepare(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            old.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Weak,
        ),
    );
    assert!(c.install_and_stop(&publish));
    let retained = old.try_share().unwrap();
    let mut replacement = None;
    c.help_record_at(&7, 1, || {
        assert_eq!(c.commit_in(&publish, &7), Outcome::Committed);
        commit(
            &c,
            c.begin_clear_reference(snapshot(&c), old.try_share().unwrap()),
        );
        drop(old);
        commit(
            &c,
            c.begin_retire_reference(snapshot(&c), retained.try_clone().unwrap()),
        );
        assert!(c.release_retired_reference(&retained));
        let new = reference(&c, 0);
        commit(
            &c,
            c.begin_publish_reference(
                snapshot(&c),
                new.try_share().unwrap(),
                root.try_share().unwrap(),
                ReferenceKind::Final,
            ),
        );
        replacement = Some(new);
    });
    let replacement = replacement.unwrap();
    assert_ne!(retained.identity, replacement.identity);
    assert_eq!(retained.membership(), Some(ReferenceMembership::Retired));
    let ReferenceMembership::Live(current) = replacement.membership().unwrap() else {
        panic!();
    };
    assert_eq!(current.kind, ReferenceKind::Final);
    assert!(!c.release_retired_reference(&retained));
}

#[test]
fn dropped_prepared_reference_has_durable_retirement_and_exact_reuse() {
    let c = catalog();
    let reference = reference(&c, 0);
    let owned = reference.try_share().unwrap();
    assert!(
        c.begin_retire_reference(snapshot(&c), owned.try_clone().unwrap())
            .is_none()
    );
    drop(reference);
    assert!(owned.removal_requested());
    commit(
        &c,
        c.begin_retire_reference(snapshot(&c), owned.try_clone().unwrap()),
    );
    assert!(c.release_retired_reference(&owned));
    assert!(c.discover_reference(0).unwrap().is_none());
}

#[test]
fn foreign_sources_slots_and_invalid_indices_are_rejected_before_effects() {
    let c = catalog();
    let foreign = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let reference = reference(&foreign, 0);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    assert!(matches!(
        c.begin_create_reference(2),
        Err(ReferencePrepareError::InvalidSlot)
    ));
    assert!(
        c.begin_publish_reference(
            snapshot(&c),
            reference.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Weak
        )
        .is_none()
    );
    assert!(
        foreign
            .begin_publish_reference(
                snapshot(&foreign),
                reference.try_share().unwrap(),
                root.try_share().unwrap(),
                ReferenceKind::Weak
            )
            .is_none()
    );
    assert_eq!(reference.membership(), Some(ReferenceMembership::Prepared));
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn delayed_reference_cas_cannot_replace_a_newer_retirement_certificate() {
    use std::sync::mpsc::channel;
    let c = Arc::new(catalog());
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let token = reference(&c, 0);
    let retained = token.try_share().unwrap();
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    let publish = prepare(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            token.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Weak,
        ),
    );
    assert!(c.install_and_stop(&publish));
    let (entered, entry) = channel();
    let (resume, resumed) = channel();
    token.pause_next_effect(move || {
        entered.send(()).unwrap();
        resumed.recv().unwrap();
    });
    let helper = {
        let c = c.clone();
        std::thread::spawn(move || c.help_record_at(&7, 1, || {}))
    };
    entry.recv().unwrap();
    assert_eq!(c.commit_in(&publish, &7), Outcome::Committed);
    commit(
        &c,
        c.begin_clear_reference(snapshot(&c), retained.try_clone().unwrap()),
    );
    drop(token);
    commit(
        &c,
        c.begin_retire_reference(snapshot(&c), retained.try_clone().unwrap()),
    );
    assert!(c.release_retired_reference(&retained));
    let replacement = reference(&c, 0);
    commit(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            replacement.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Final,
        ),
    );
    resume.send(()).unwrap();
    helper.join().unwrap();
    assert_eq!(retained.membership(), Some(ReferenceMembership::Retired));
    let ReferenceMembership::Live(state) = replacement.membership().unwrap() else {
        panic!();
    };
    assert_eq!(state.kind, ReferenceKind::Final);
}

fn drain(c: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>) {
    let Some(Membership::Tracing(marks)) = cell.membership() else {
        panic!();
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
fn scan_stage(c: &Catalog<Fixture>) {
    for _ in 0..c.reference_capacity() {
        commit(c, c.begin_scan_reference(snapshot(c), 0));
    }
}
fn next_epoch(c: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>) {
    next_epoch_mode(c, cell, DomainMode::Full);
}
fn next_epoch_mode(c: &Catalog<Fixture>, cell: &Pooled<Cell<Fixture>>, mode: DomainMode) {
    drain(c, cell);
    while snapshot(c).phase().root_cursors[0] < c.root_capacity() {
        commit(c, c.begin_scan_root(snapshot(c), 0));
    }
    while snapshot(c).phase().domains[0].floor < RETIREMENT_FLOOR {
        if snapshot(c).phase().domains[0].floor != 0 {
            while snapshot(c).phase().reference_cursors[0] < c.reference_capacity() {
                commit(c, c.begin_scan_reference(snapshot(c), 0));
            }
            drain(c, cell);
        }
        commit(c, c.begin_advance(snapshot(c), 0));
    }
    commit(c, c.begin_finish(snapshot(c), 0));
    commit(c, c.begin_start(snapshot(c), 0, mode));
    for _ in 0..c.root_capacity() {
        commit(c, c.begin_scan_root(snapshot(c), 0));
    }
}
fn host_reference(
    kind: ReferenceKind,
) -> (
    Catalog<Fixture>,
    Pooled<Cell<Fixture>>,
    ReferenceToken<Fixture>,
) {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let reference = reference(&c, 0);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    commit(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            reference.try_share().unwrap(),
            root.try_share().unwrap(),
            kind,
        ),
    );
    let owned = root.try_share().unwrap();
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), owned.try_clone().unwrap()),
    );
    drop(root);
    assert!(c.release_retired_root(&owned));
    drop(owned);
    next_epoch(&c, &cell);
    (c, cell, reference)
}

#[test]
fn every_host_reference_stage_certifies_decision_and_retention_before_advancing() {
    for kind in [
        ReferenceKind::Soft,
        ReferenceKind::Weak,
        ReferenceKind::Final,
        ReferenceKind::Phantom,
        ReferenceKind::PhantomRetained,
    ] {
        let (c, cell, reference) = host_reference(kind);
        commit(&c, c.begin_advance(snapshot(&c), 0));
        let decision_stage = match kind {
            ReferenceKind::Soft | ReferenceKind::Weak => 1,
            ReferenceKind::Final => 2,
            ReferenceKind::Phantom | ReferenceKind::PhantomRetained => 3,
        };
        for stage in 1..5 {
            assert!(
                c.begin_advance(snapshot(&c), 0).is_none(),
                "reference scan was skipped"
            );
            scan_stage(&c);
            let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
                panic!();
            };
            assert_eq!(state.activated, stage >= decision_stage);
            assert!(
                !state.pending,
                "host reference gained a managed notification"
            );
            if stage >= decision_stage {
                assert_eq!(state.target.is_some(), kind.retaining());
            }
            let Some(Membership::Tracing(marks)) = cell.membership() else {
                panic!();
            };
            if stage == decision_stage && kind.retaining() {
                assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
                assert_eq!(
                    marks[0].retention,
                    Some(if kind == ReferenceKind::Final {
                        TraceRetention::Final
                    } else {
                        TraceRetention::Phantom
                    })
                );
                assert!(
                    c.begin_advance(snapshot(&c), 0).is_none(),
                    "retention traversal was skipped"
                );
                drain(&c, &cell);
            }
            commit(&c, c.begin_advance(snapshot(&c), 0));
            assert_eq!(snapshot(&c).phase().reference_cursors[0], 0);
        }
        assert_eq!(
            c.begin_trace_retire(snapshot(&c), cell.try_clone().unwrap(), 0)
                .is_none(),
            kind.retaining()
        );
    }
}

#[test]
fn activated_retained_phantom_keeps_its_target_at_final_stage_next_epoch() {
    let (c, cell, reference) = host_reference(ReferenceKind::PhantomRetained);
    next_epoch(&c, &cell);
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!();
    };
    assert!(state.activated);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
    let Some(Membership::Tracing(marks)) = cell.membership() else {
        panic!();
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Final));
    assert!(!marks[0].phantom_decided);
}

#[test]
fn failed_reference_proposal_share_cancels_both_retention_and_scan_cursor() {
    let (c, cell, reference) = host_reference(ReferenceKind::Final);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    let before = cell.membership();
    let attempt = prepare(&c, c.begin_scan_reference(snapshot(&c), 0));
    // Busy owner, target effect owner, then reference effect owner.
    FAIL_SHARE.with(|fail| fail.set(Some(2)));
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Pending);
    assert!(
        attempt.cancel(),
        "COMMIT preceded all prepared effect ownership"
    );
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Cancelled);
    assert_eq!(cell.membership(), before);
    let phase = snapshot(&c).phase();
    assert_eq!(phase.domains[0].pending, 0);
    assert_eq!(phase.reference_cursors[0], 0);
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!();
    };
    assert!(!state.activated);
}

#[test]
fn another_helper_certifies_both_retaining_effects_before_scan_completion() {
    let (c, cell, reference) = host_reference(ReferenceKind::Final);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    let attempt = prepare(&c, c.begin_scan_reference(snapshot(&c), 0));
    assert!(c.install_and_stop(&attempt));
    c.help_record_at(&7, 1, || {
        let Some(Membership::Tracing(marks)) = cell.membership() else {
            panic!();
        };
        assert_eq!(marks[0].retention, Some(TraceRetention::Final));
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!();
        };
        assert!(!state.activated);
        assert_eq!(c.commit_in(&attempt, &7), Outcome::Committed);
        let phase = snapshot(&c).phase();
        assert_eq!(phase.reference_cursors[0], 1);
        assert_eq!(phase.domains[0].pending, 1);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!();
        };
        assert!(state.activated);
    });
}

#[test]
fn retained_phantom_group_decision_survives_promotion_before_later_slots() {
    let c = catalog();
    let cell = cell(&c);
    let root = root(&c, &cell, 0);
    let retained = reference(&c, 0);
    let ordinary = reference(&c, 1);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), root.try_share().unwrap()),
    );
    commit(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            retained.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::PhantomRetained,
        ),
    );
    commit(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            ordinary.try_share().unwrap(),
            root.try_share().unwrap(),
            ReferenceKind::Phantom,
        ),
    );
    let owned = root.try_share().unwrap();
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), owned.try_clone().unwrap()),
    );
    drop(root);
    assert!(c.release_retired_root(&owned));
    next_epoch_mode(&c, &cell, DomainMode::Young);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    commit(&c, c.begin_scan_reference(snapshot(&c), 0));
    let Some(Membership::Tracing(marks)) = cell.membership() else {
        panic!();
    };
    assert!(marks[0].phantom_decided);
    commit(&c, c.begin_promote(snapshot(&c), cell.try_clone().unwrap()));
    let Some(Membership::Tracing(marks)) = cell.membership() else {
        panic!();
    };
    assert_eq!(marks.generation, Generation::Old);
    assert!(marks.reference_live(snapshot(&c).phase().domains[0], 0, true));
    assert!(marks[0].phantom_decided);
    commit(&c, c.begin_scan_reference(snapshot(&c), 0));
    let ReferenceMembership::Live(state) = ordinary.membership().unwrap() else {
        panic!();
    };
    assert!(state.activated);
    assert_eq!(
        state.target, None,
        "promotion erased the earlier phantom group decision"
    );
}

struct Managed {
    c: Catalog<Fixture>,
    target: Pooled<Cell<Fixture>>,
    owner: Pooled<Cell<Fixture>>,
    reference: ReferenceToken<Fixture>,
}
impl Managed {
    fn new(
        kind: ReferenceKind,
        same_target: bool,
        owner_retention: Option<TraceRetention>,
    ) -> Self {
        let c = Catalog::with_record_root_capacities_and_limit(
            3,
            256,
            256,
            128,
            6,
            64,
            pool::MAX_GENERATION,
        )
        .with_references(2, 64, 256, pool::MAX_GENERATION);
        let target = cell(&c);
        let owner = if same_target {
            target.try_clone().unwrap()
        } else {
            let mut create = c.begin_create_with(1, 16, Fixture::new()).unwrap();
            (0..16).find_map(|_| create.poll(&c).unwrap()).unwrap()
        };
        let target_root = root(&c, &target, 0);
        commit(
            &c,
            c.begin_publish_root(snapshot(&c), target_root.try_share().unwrap()),
        );
        let owner_root = root(&c, &owner, 1);
        if same_target {
            commit(
                &c,
                c.begin_activate_root(snapshot(&c), owner_root.try_share().unwrap()),
            );
        } else {
            commit(
                &c,
                c.begin_publish_root(snapshot(&c), owner_root.try_share().unwrap()),
            );
        }
        let reference = reference(&c, 0);
        commit(
            &c,
            c.begin_publish_reference(
                snapshot(&c),
                reference.try_share().unwrap(),
                target_root.try_share().unwrap(),
                kind,
            ),
        );
        commit(
            &c,
            c.begin_bind_reference_owner(
                snapshot(&c),
                reference.try_share().unwrap(),
                owner_root.try_share().unwrap(),
            ),
        );
        for token in [target_root, owner_root] {
            let root = token.try_share().unwrap();
            commit(
                &c,
                c.begin_remove_root(snapshot(&c), root.try_clone().unwrap()),
            );
            drop(token);
            assert!(c.release_retired_root(&root));
        }
        for domain in 0..2 {
            while snapshot(&c).phase().domains[domain].floor < RETIREMENT_FLOOR {
                drain(&c, &target);
                drain(&c, &owner);
                if snapshot(&c).phase().domains[domain].floor == 0 {
                    while snapshot(&c).phase().root_cursors[domain] < c.root_capacity() {
                        commit(&c, c.begin_scan_root(snapshot(&c), domain));
                    }
                } else {
                    while snapshot(&c).phase().reference_cursors[domain] < c.reference_capacity() {
                        commit(&c, c.begin_scan_reference(snapshot(&c), domain));
                    }
                }
                commit(&c, c.begin_advance(snapshot(&c), domain));
            }
            commit(&c, c.begin_finish(snapshot(&c), domain));
        }
        for domain in 0..2 {
            commit(&c, c.begin_start(snapshot(&c), domain, DomainMode::Full));
            for _ in 0..c.root_capacity() {
                commit(&c, c.begin_scan_root(snapshot(&c), domain));
            }
        }
        if let Some(retention) = owner_retention {
            commit(
                &c,
                c.begin_shade(snapshot(&c), owner.try_clone().unwrap(), 0, retention),
            );
            drain(&c, &owner);
        }
        commit(&c, c.begin_advance(snapshot(&c), 0));
        Self {
            c,
            target,
            owner,
            reference,
        }
    }
    fn state(&self) -> crate::reference::ReferenceState {
        let ReferenceMembership::Live(state) = self.reference.membership().unwrap() else {
            panic!()
        };
        state
    }
    fn advance(&self) {
        let c = &self.c;
        while snapshot(c).phase().reference_cursors[0] < c.reference_capacity() {
            commit(c, c.begin_scan_reference(snapshot(c), 0));
        }
        drain(c, &self.target);
        drain(c, &self.owner);
        commit(c, c.begin_advance(snapshot(c), 0));
    }
}

#[test]
fn managed_notifications_keep_custody_through_token_drop_and_transfer_exactly_once() {
    for kind in [
        ReferenceKind::Soft,
        ReferenceKind::Weak,
        ReferenceKind::Final,
        ReferenceKind::Phantom,
        ReferenceKind::PhantomRetained,
    ] {
        let m = Managed::new(kind, false, Some(TraceRetention::Strong));
        let c = &m.c;
        let notification = root(c, &m.owner, 2);
        let decision_stage = match kind {
            ReferenceKind::Soft | ReferenceKind::Weak => 1,
            ReferenceKind::Final => 2,
            _ => 3,
        };
        let queue_stage = if kind == ReferenceKind::Final { 2 } else { 4 };
        for stage in 1..=queue_stage {
            if stage == queue_stage {
                assert!(
                    c.begin_scan_reference(snapshot(c), 0).is_none(),
                    "scan silently lost a required root"
                );
                assert_eq!(snapshot(c).phase().reference_cursors[0], 0);
                commit(
                    c,
                    c.begin_scan_reference_with_root(
                        snapshot(c),
                        0,
                        notification.try_share().unwrap(),
                    ),
                );
            } else {
                commit(c, c.begin_scan_reference(snapshot(c), 0));
            }
            let state = m.state();
            assert_eq!(state.activated, stage >= decision_stage);
            assert_eq!(
                state.pending,
                stage >= decision_stage && stage < queue_stage
            );
            assert_eq!(state.queued_root.is_some(), stage == queue_stage);
            if stage < queue_stage {
                m.advance();
            }
        }
        let held = notification.try_share().unwrap();
        assert!(held.is_internal());
        drop(notification);
        assert!(held.removal_requested());
        assert!(
            c.begin_requested_root_removal(snapshot(c), held.try_clone().unwrap())
                .is_none()
        );
        assert!(
            c.begin_remove_root(snapshot(c), held.try_clone().unwrap())
                .is_none()
        );
        assert!(!c.release_retired_root(&held));
        let caller = root(c, &m.owner, 3);
        commit(
            c,
            c.begin_take_reference(
                snapshot(c),
                m.reference.try_share().unwrap(),
                caller.try_share().unwrap(),
            ),
        );
        assert_eq!(caller.status(), RootStatus::Live);
        assert!(!caller.is_internal());
        assert_eq!(held.status(), RootStatus::Retired);
        assert!(m.state().queued_root.is_none());
        assert!(
            c.begin_take_reference(
                snapshot(c),
                m.reference.try_share().unwrap(),
                caller.try_share().unwrap()
            )
            .is_none()
        );
        assert!(c.release_retired_root(&held));
    }
}

#[test]
fn dead_managed_owners_do_not_retain_targets_or_receive_notifications() {
    for kind in [
        ReferenceKind::Weak,
        ReferenceKind::Final,
        ReferenceKind::PhantomRetained,
    ] {
        let m = Managed::new(kind, false, None);
        for _ in 1..5 {
            m.advance();
        }
        assert!(!m.state().pending);
        assert!(m.state().queued_root.is_none());
        assert_eq!(m.state().activated, !kind.retaining());
        assert!(
            m.c.begin_trace_retire(snapshot(&m.c), m.target.try_clone().unwrap(), 0)
                .is_some()
        );
        assert!(
            m.c.begin_trace_retire(snapshot(&m.c), m.owner.try_clone().unwrap(), 0)
                .is_some()
        );
    }
}

#[test]
fn final_notification_couples_two_object_effects_before_the_reference_cursor() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    let c = &m.c;
    m.advance();
    let notification = root(c, &m.owner, 2);
    let attempt = prepare(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
    );
    assert!(c.install_and_stop(&attempt));
    c.help_record_at(&7, 1, || {
        let Some(Membership::Tracing(target)) = m.target.membership() else {
            panic!()
        };
        let Some(Membership::Tracing(owner)) = m.owner.membership() else {
            panic!()
        };
        assert_eq!(target[0].retention, Some(TraceRetention::Final));
        assert!(!owner[1].marked(2));
        assert_eq!(notification.status(), RootStatus::Prepared);
        assert!(!m.state().activated);
        assert_eq!(c.commit_in(&attempt, &7), Outcome::Committed);
        let phase = snapshot(c).phase();
        assert_eq!(phase.domains.map(|d| d.pending), [1, 1]);
        let Some(Membership::Tracing(owner)) = m.owner.membership() else {
            panic!()
        };
        assert!(owner[1].marked(2));
        assert_ne!(owner[1].ticket, 0);
        assert_eq!(phase.reference_cursors[0], 1);
        assert!(notification.is_internal());
        assert!(m.state().queued_root.is_some());
    });
}

#[test]
fn every_notification_effect_owner_is_acquired_before_commit() {
    for failure in [1, 2, 3] {
        let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
        let c = &m.c;
        m.advance();
        let notification = root(c, &m.owner, 2);
        let before = (m.target.membership(), m.owner.membership(), m.state());
        let attempt = prepare(
            c,
            c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
        );
        FAIL_SHARE.with(|fail| fail.set(Some(failure)));
        assert_eq!(c.commit_in(&attempt, &7), Outcome::Pending);
        assert!(attempt.cancel(), "effect owner acquired after COMMIT");
        assert_eq!(c.commit_in(&attempt, &7), Outcome::Cancelled);
        assert_eq!(
            before,
            (m.target.membership(), m.owner.membership(), m.state())
        );
        assert_eq!(notification.status(), RootStatus::Prepared);
        let phase = snapshot(c).phase();
        assert_eq!(phase.domains.map(|d| d.pending), [0, 0]);
        assert_eq!(phase.reference_cursors[0], 0);
    }
}

#[test]
fn a_self_reference_merges_retention_and_owner_rooting_without_lost_work() {
    let m = Managed::new(ReferenceKind::Final, true, Some(TraceRetention::Phantom));
    let c = &m.c;
    m.advance();
    let notification = root(c, &m.owner, 2);
    commit(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
    );
    let Some(Membership::Tracing(marks)) = m.target.membership() else {
        panic!()
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Final));
    assert_eq!(marks[1].retention, Some(TraceRetention::Strong));
    assert_eq!(snapshot(c).phase().domains.map(|d| d.pending), [1, 1]);
    assert!(m.state().queued_root.is_some());
}

#[test]
fn internal_notification_custody_cannot_authorize_new_host_handles() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Strong));
    let c = &m.c;
    m.advance();
    let notification = root(c, &m.owner, 2);
    commit(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
    );
    let destination = reference(c, 1);
    assert!(
        c.begin_publish_reference(
            snapshot(c),
            destination.try_share().unwrap(),
            notification.try_share().unwrap(),
            ReferenceKind::Weak
        )
        .is_none()
    );
    assert!(
        c.begin_bind_reference_owner(
            snapshot(c),
            m.reference.try_share().unwrap(),
            notification.try_share().unwrap()
        )
        .is_none()
    );
    let wrong = root(c, &m.target, 3);
    assert!(
        c.begin_take_reference(
            snapshot(c),
            m.reference.try_share().unwrap(),
            wrong.try_share().unwrap()
        )
        .is_none()
    );
    assert!(m.state().queued_root.is_some());
    assert_eq!(wrong.status(), RootStatus::Prepared);
}

#[test]
fn clear_and_slot_drop_preserve_a_queued_notification_until_delivery() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Strong));
    m.advance();
    let c = &m.c;
    let notification = root(c, &m.owner, 2);
    commit(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
    );
    commit(
        c,
        c.begin_clear_reference(snapshot(c), m.reference.try_share().unwrap()),
    );
    let slot = m.reference.try_share().unwrap();
    drop(m.reference);
    assert!(
        c.begin_retire_reference(snapshot(c), slot.try_clone().unwrap())
            .is_none()
    );
    let caller = root(c, &m.owner, 3);
    let attempt = prepare(
        c,
        c.begin_take_reference(
            snapshot(c),
            slot.try_clone().unwrap(),
            caller.try_share().unwrap(),
        ),
    );
    assert!(c.install_and_stop(&attempt));
    c.help_record_at(&7, 1, || {
        assert_eq!(c.commit_in(&attempt, &7), Outcome::Committed);
        assert_eq!(caller.status(), RootStatus::Live);
        assert_eq!(notification.status(), RootStatus::Retired);
        commit(
            c,
            c.begin_retire_reference(snapshot(c), slot.try_clone().unwrap()),
        );
        assert!(c.release_retired_reference(&slot));
    });
    assert_eq!(slot.membership(), Some(ReferenceMembership::Retired));
}

#[test]
fn weak_notification_waits_for_owner_reachability_established_by_final_tracing() {
    let m = Managed::new(ReferenceKind::Weak, false, None);
    let c = &m.c;
    commit(c, c.begin_scan_reference(snapshot(c), 0));
    assert!(m.state().pending);
    assert!(m.state().target.is_none());
    m.advance();
    commit(
        c,
        c.begin_shade(
            snapshot(c),
            m.owner.try_clone().unwrap(),
            0,
            TraceRetention::Final,
        ),
    );
    m.advance();
    m.advance();
    assert_eq!(snapshot(c).phase().domains[0].floor, 4);
    let notification = root(c, &m.owner, 2);
    commit(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
    );
    assert!(!m.state().pending);
    assert!(m.state().queued_root.is_some());
    assert_eq!(notification.target_identity(), m.owner.identity);
}

#[test]
fn failed_owner_observation_never_becomes_a_dead_owner_or_completed_scan() {
    for failure in 0..4 {
        let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
        let c = &m.c;
        m.advance();
        let notification = root(c, &m.owner, 2);
        let current = snapshot(c);
        FAIL_LOAD.with(|fail| fail.set(Some(failure)));
        assert!(
            c.begin_scan_reference_with_root(current, 0, notification.try_share().unwrap())
                .is_none()
        );
        let phase = snapshot(c).phase();
        assert_eq!(phase.reference_cursors[0], 0);
        assert_eq!(phase.domains.map(|d| d.pending), [0, 0]);
        assert!(!m.state().activated);
        commit(
            c,
            c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
        );
        assert!(m.state().queued_root.is_some());
    }
}

fn construction_fixture() -> (
    Catalog<Fixture>,
    Pooled<Cell<Fixture>>,
    ReferenceToken<Fixture>,
    RootToken<Fixture>,
) {
    let c = Catalog::with_record_root_capacities_and_limit(
        3,
        256,
        256,
        128,
        6,
        64,
        pool::MAX_GENERATION,
    )
    .with_references(2, 64, 256, pool::MAX_GENERATION);
    let target = cell(&c);
    let source = root(&c, &target, 0);
    let reference = reference(&c, 0);
    publish(&c, &source, &reference);
    let reservation = root(&c, &target, 1);
    commit(
        &c,
        c.begin_reserve_reference_owner(
            snapshot(&c),
            reference.try_share().unwrap(),
            reservation.try_share().unwrap(),
        ),
    );
    let source_root = source.try_share().unwrap();
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), source_root.try_clone().unwrap()),
    );
    drop(source);
    assert!(c.release_retired_root(&source_root));
    (c, target, reference, reservation)
}
fn extra_cell(c: &Catalog<Fixture>, slot: usize, record: Fixture) -> Pooled<Cell<Fixture>> {
    let mut create = c.begin_create_with(slot, (slot + 1) * 8, record).unwrap();
    (0..16).find_map(|_| create.poll(c).unwrap()).unwrap()
}
fn finish_domain(c: &Catalog<Fixture>, cells: &[&Pooled<Cell<Fixture>>], domain: usize) {
    while snapshot(c).phase().domains[domain].floor < RETIREMENT_FLOOR {
        if snapshot(c).phase().domains[domain].floor == 0 {
            while snapshot(c).phase().root_cursors[domain] < c.root_capacity() {
                commit(c, c.begin_scan_root(snapshot(c), domain));
            }
        } else {
            while snapshot(c).phase().reference_cursors[domain] < c.reference_capacity() {
                commit(c, c.begin_scan_reference(snapshot(c), domain));
            }
        }
        for cell in cells {
            drain(c, cell);
        }
        commit(c, c.begin_advance(snapshot(c), domain));
    }
}

#[test]
fn a_retained_construction_token_allows_cycles_and_unrelated_reclamation() {
    let (c, target, reference, reservation) = construction_fixture();
    let garbage = extra_cell(&c, 1, Fixture::new());
    commit(
        &c,
        c.begin_publish(snapshot(&c), garbage.try_clone().unwrap()),
    );
    finish_domain(&c, &[&target, &garbage], 0);
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Full));
    finish_domain(&c, &[&target, &garbage], 0);
    assert_eq!(reservation.status(), RootStatus::Live);
    assert!(reservation.is_internal());
    assert!(
        c.begin_cancel_reference_owner(snapshot(&c), reference.try_share().unwrap())
            .is_none()
    );
    assert!(
        c.begin_trace_retire(snapshot(&c), target.try_clone().unwrap(), 0)
            .is_none()
    );
    commit(
        &c,
        c.begin_trace_retire(snapshot(&c), garbage.try_clone().unwrap(), 0),
    );
    assert!(c.release_retired(&garbage));
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!()
    };
    assert!(!state.activated);
    assert!(state.construction_root.is_some());
}

#[test]
fn dropped_construction_tokens_are_cancelled_with_deletion_work_before_scan_completion() {
    let (c, target, reference, reservation) = construction_fixture();
    finish_domain(&c, &[&target], 0);
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Full));
    let held = reservation.try_share().unwrap();
    drop(reservation);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
    let cancellation = prepare(
        &c,
        c.begin_cancel_reference_owner(snapshot(&c), reference.try_share().unwrap()),
    );
    assert!(c.install_and_stop(&cancellation));
    c.help_record_at(&7, 1, || {
        let Some(Membership::Tracing(marks)) = target.membership() else {
            panic!()
        };
        assert!(marks[0].marked(2));
        assert_eq!(held.status(), RootStatus::Live);
        assert_eq!(c.commit_in(&cancellation, &7), Outcome::Committed);
        assert_eq!(held.status(), RootStatus::Retired);
        assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!()
        };
        assert!(state.construction_root.is_none());
    });
    assert!(c.release_retired_root(&held));
}

#[test]
fn reference_scan_can_finish_a_dropped_construction_and_slot_without_the_originator() {
    let (c, target, reference, reservation) = construction_fixture();
    let slot = reference.try_share().unwrap();
    let held = reservation.try_share().unwrap();
    drop(reference);
    assert!(
        c.begin_retire_reference(snapshot(&c), slot.try_clone().unwrap())
            .is_none()
    );
    drop(reservation);
    // Root scanning must leave custody for the reference controller to release.
    for _ in 0..c.root_capacity() {
        commit(&c, c.begin_scan_root(snapshot(&c), 0));
    }
    drain(&c, &target);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    commit(&c, c.begin_scan_reference(snapshot(&c), 0));
    assert_eq!(snapshot(&c).phase().reference_cursors[0], 1);
    assert_eq!(held.status(), RootStatus::Retired);
    assert_eq!(slot.membership(), Some(ReferenceMembership::Retired));
    assert!(c.release_retired_root(&held));
    assert!(c.release_retired_reference(&slot));
}

#[test]
fn owner_publication_holds_its_graph_until_binding_and_root_release_are_certified() {
    let (c, _target, reference, reservation) = construction_fixture();
    let record = Fixture::new();
    let owner = extra_cell(&c, 1, record.clone());
    let owner_root = root(&c, &owner, 2);
    let attempt = prepare(
        &c,
        c.begin_publish_reference_owner(
            snapshot(&c),
            reference.try_share().unwrap(),
            owner_root.try_share().unwrap(),
        ),
    );
    assert!(c.install_and_stop(&attempt));
    c.help_record_at(&7, 1, || {
        assert!(matches!(owner.membership(), Some(Membership::Tracing(_))));
        assert_eq!(record.0.published.load(Ordering::Acquire), 0);
        assert_eq!(reservation.status(), RootStatus::Live);
        assert_eq!(owner_root.status(), RootStatus::Prepared);
        record.release_origin();
        assert_eq!(c.commit_in(&attempt, &7), Outcome::Committed);
        assert_eq!(record.0.published.load(Ordering::Acquire), 1);
        assert_eq!(reservation.status(), RootStatus::Retired);
        assert_eq!(owner_root.status(), RootStatus::Live);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!()
        };
        assert!(state.construction_root.is_none());
        assert_eq!(
            state.owner,
            Some(ObjectKey {
                identity: owner.identity,
                slot: owner.slot()
            })
        );
    });
    assert_eq!(record.0.handles.load(Ordering::Acquire), 0);
    assert_eq!(record.0.last_release.load(Ordering::Acquire), 1);
}

#[test]
fn closed_owner_publication_cancels_without_releasing_construction_protection() {
    let (c, _target, reference, reservation) = construction_fixture();
    let record = Fixture::new();
    let owner = extra_cell(&c, 1, record.clone());
    let owner_root = root(&c, &owner, 2);
    let attempt = prepare(
        &c,
        c.begin_publish_reference_owner(
            snapshot(&c),
            reference.try_share().unwrap(),
            owner_root.try_share().unwrap(),
        ),
    );
    record.release_origin();
    assert_eq!(c.commit_in(&attempt, &7), Outcome::Cancelled);
    assert_eq!(owner.membership(), Some(Membership::Prepared));
    assert_eq!(owner_root.status(), RootStatus::Prepared);
    assert_eq!(reservation.status(), RootStatus::Live);
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!()
    };
    assert!(state.owner.is_none());
    assert!(state.construction_root.is_some());
    drop(reservation);
    commit(
        &c,
        c.begin_cancel_reference_owner(snapshot(&c), reference.try_share().unwrap()),
    );
}

#[test]
fn construction_clear_preserves_protection_and_clone_does_not_copy_its_custody() {
    let (c, _target, reference, reservation) = construction_fixture();
    let clone = reference_controller_clone_slot(&c, &reference);
    let ReferenceMembership::Live(clone_state) = clone.membership().unwrap() else {
        panic!()
    };
    assert!(clone_state.target.is_some());
    assert!(clone_state.construction_root.is_none());
    commit(
        &c,
        c.begin_clear_reference(snapshot(&c), reference.try_share().unwrap()),
    );
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!()
    };
    assert!(state.target.is_none());
    assert!(state.construction_root.is_some());
    let owner = extra_cell(&c, 1, Fixture::new());
    let owner_root = root(&c, &owner, 2);
    commit(
        &c,
        c.begin_publish_reference_owner(
            snapshot(&c),
            reference.try_share().unwrap(),
            owner_root.try_share().unwrap(),
        ),
    );
    assert_eq!(reservation.status(), RootStatus::Retired);
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!()
    };
    assert!(state.target.is_none());
    assert!(state.owner.is_some());
}
fn reference_controller_clone_slot(
    c: &Catalog<Fixture>,
    source: &ReferenceToken<Fixture>,
) -> ReferenceToken<Fixture> {
    let clone = reference(c, 1);
    commit(
        c,
        c.begin_clone_reference(
            snapshot(c),
            source.try_share().unwrap(),
            clone.try_share().unwrap(),
        ),
    );
    clone
}

#[test]
fn one_owner_can_publish_multiple_reserved_slots_without_waiting_for_their_origin() {
    let (c, target, first, first_hold) = construction_fixture();
    let second = reference_controller_clone_slot(&c, &first);
    let second_hold = root(&c, &target, 2);
    commit(
        &c,
        c.begin_reserve_reference_owner(
            snapshot(&c),
            second.try_share().unwrap(),
            second_hold.try_share().unwrap(),
        ),
    );
    let record = Fixture::new();
    let owner = extra_cell(&c, 1, record.clone());
    for slot in [&first, &second] {
        commit(
            &c,
            c.begin_prepare_reference_owner(
                snapshot(&c),
                slot.try_share().unwrap(),
                owner.try_clone().unwrap(),
            ),
        );
        assert!(
            c.begin_finish_reference_owner(snapshot(&c), slot.try_share().unwrap())
                .is_none()
        );
    }
    // A paused constructor leaves both referents protected, but no cycle waiting.
    finish_domain(&c, &[&target], 0);
    assert_eq!(first_hold.status(), RootStatus::Live);
    assert_eq!(second_hold.status(), RootStatus::Live);
    let owner_root = root(&c, &owner, 3);
    commit(
        &c,
        c.begin_publish_root(snapshot(&c), owner_root.try_share().unwrap()),
    );
    record.release_origin();
    assert_eq!(record.0.last_release.load(Ordering::Acquire), 1);
    commit(
        &c,
        c.begin_finish_reference_owner(snapshot(&c), first.try_share().unwrap()),
    );
    // The second captured reservation can be completed by the next reference scan.
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Full));
    for _ in 0..c.root_capacity() {
        commit(&c, c.begin_scan_root(snapshot(&c), 0));
    }
    drain(&c, &target);
    drain(&c, &owner);
    commit(&c, c.begin_advance(snapshot(&c), 0));
    scan_stage(&c);
    for (slot, held) in [(&first, &first_hold), (&second, &second_hold)] {
        let ReferenceMembership::Live(state) = slot.membership().unwrap() else {
            panic!()
        };
        assert_eq!(
            state.owner,
            Some(ObjectKey {
                identity: owner.identity,
                slot: owner.slot()
            })
        );
        assert!(state.construction_root.is_none());
        assert_eq!(held.status(), RootStatus::Retired);
    }
}

#[test]
fn abandoned_prepared_owner_does_not_revoke_a_live_construction_lifetime() {
    let (c, target, reference, reservation) = construction_fixture();
    let owner = extra_cell(&c, 1, Fixture::new());
    commit(
        &c,
        c.begin_prepare_reference_owner(
            snapshot(&c),
            reference.try_share().unwrap(),
            owner.try_clone().unwrap(),
        ),
    );
    commit(
        &c,
        c.begin_abandon(snapshot(&c), owner.try_clone().unwrap()),
    );
    assert!(c.release_retired(&owner));
    finish_domain(&c, &[&target], 0);
    assert_eq!(reservation.status(), RootStatus::Live);
    assert!(
        c.begin_finish_reference_owner(snapshot(&c), reference.try_share().unwrap())
            .is_none()
    );
    assert!(
        c.begin_cancel_reference_owner(snapshot(&c), reference.try_share().unwrap())
            .is_none()
    );
    drop(reservation);
    commit(
        &c,
        c.begin_cancel_reference_owner(snapshot(&c), reference.try_share().unwrap()),
    );
    let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
        panic!()
    };
    assert!(state.construction_root.is_none());
    assert_eq!(
        state.owner,
        Some(ObjectKey {
            identity: owner.identity,
            slot: owner.slot()
        })
    );
}

#[test]
#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn delayed_notification_root_cas_cannot_reactivate_a_consumed_root() {
    use std::sync::mpsc::channel;
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    m.advance();
    let c = Arc::new(m.c);
    let notification = root(&c, &m.owner, 2);
    commit(
        &c,
        c.begin_scan_reference_with_root(snapshot(&c), 0, notification.try_share().unwrap()),
    );
    let caller = root(&c, &m.owner, 3);
    let take = prepare(
        &c,
        c.begin_take_reference(
            snapshot(&c),
            m.reference.try_share().unwrap(),
            caller.try_share().unwrap(),
        ),
    );
    assert!(c.install_and_stop(&take));
    let (entered, entry) = channel();
    let (resume, resumed) = channel();
    caller.pause_next_effect(move || {
        entered.send(()).unwrap();
        resumed.recv().unwrap();
    });
    let helper = {
        let c = c.clone();
        std::thread::spawn(move || c.help_record_at(&7, 1, || {}))
    };
    entry.recv().unwrap();
    assert_eq!(c.commit_in(&take, &7), Outcome::Committed);
    let old = caller.try_share().unwrap();
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), old.try_clone().unwrap()),
    );
    drop(caller);
    assert!(c.release_retired_root(&old));
    let replacement = root(&c, &m.owner, 3);
    resume.send(()).unwrap();
    helper.join().unwrap();
    assert_eq!(old.status(), RootStatus::Retired);
    assert_eq!(replacement.status(), RootStatus::Prepared);
    assert_ne!(old.identity, replacement.identity);
}

fn reserved_root(
    c: &Catalog<Fixture>,
    reference: &ReferenceToken<Fixture>,
    kind: ReferenceRootKind,
    target: &Pooled<Cell<Fixture>>,
) -> RootToken<Fixture> {
    let mut creation = c
        .begin_create_reference_root(
            reference.try_share().unwrap(),
            kind,
            target.try_clone().unwrap(),
        )
        .unwrap();
    (0..16).find_map(|_| creation.poll(c).unwrap()).unwrap()
}

#[test]
fn late_reserved_root_announcement_cannot_occupy_a_replacement_references_capacity() {
    let c = catalog();
    let target = cell(&c);
    let source = root(&c, &target, 0);
    let old = reference(&c, 0);
    publish(&c, &source, &old);
    let old_slot = old.try_share().unwrap();
    let mut delayed = c
        .begin_create_reference_root(
            old.try_share().unwrap(),
            ReferenceRootKind::Construction,
            target.try_clone().unwrap(),
        )
        .unwrap();
    assert!(delayed.poll(&c).unwrap().is_none());
    drop(old);
    commit(
        &c,
        c.begin_retire_reference(snapshot(&c), old_slot.try_clone().unwrap()),
    );
    assert!(c.release_retired_reference(&old_slot));
    let replacement = reference(&c, 0);
    commit(
        &c,
        c.begin_publish_reference(
            snapshot(&c),
            replacement.try_share().unwrap(),
            source.try_share().unwrap(),
            ReferenceKind::Weak,
        ),
    );
    let late = delayed.poll(&c).unwrap().expect("old private announcement");
    assert!(
        c.discover_root(late.slot()).unwrap().is_none(),
        "old creator occupied the new reference's reserve"
    );
    let current = reserved_root(&c, &replacement, ReferenceRootKind::Construction, &target);
    assert_eq!(late.slot(), current.slot());
    assert_ne!(late.identity, current.identity);
    assert_eq!(
        c.discover_root(current.slot()).unwrap().unwrap().identity,
        current.identity
    );
    assert!(
        c.begin_reserve_reference_owner(
            snapshot(&c),
            replacement.try_share().unwrap(),
            late.try_share().unwrap()
        )
        .is_none()
    );
    assert!(
        c.begin_activate_root(snapshot(&c), late.try_share().unwrap())
            .is_none()
    );
    commit(
        &c,
        c.begin_reserve_reference_owner(
            snapshot(&c),
            replacement.try_share().unwrap(),
            current.try_share().unwrap(),
        ),
    );
    drop(delayed);
    drop(old_slot);
    drop(late);
    assert_eq!(current.status(), RootStatus::Live);
    assert_eq!(
        c.discover_root(current.slot()).unwrap().unwrap().identity,
        current.identity
    );
}

#[test]
fn reserved_roles_keep_notification_capacity_available_while_construction_token_is_held() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    m.advance();
    let c = &m.c;
    let construction = reserved_root(c, &m.reference, ReferenceRootKind::Construction, &m.owner);
    assert!(
        c.begin_scan_reference_with_root(snapshot(c), 0, construction.try_share().unwrap())
            .is_none()
    );
    let notification = reserved_root(c, &m.reference, ReferenceRootKind::Notification, &m.owner);
    commit(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, notification.try_share().unwrap()),
    );
    assert_eq!(notification.status(), RootStatus::Live);
    assert_eq!(construction.status(), RootStatus::Prepared);
    let caller = root(c, &m.owner, 3);
    commit(
        c,
        c.begin_take_reference(
            snapshot(c),
            m.reference.try_share().unwrap(),
            caller.try_share().unwrap(),
        ),
    );
    // Reserved identity prevents a retained old token from pinning the location.
    let held = notification.try_share().unwrap();
    assert!(!held.removal_requested());
    assert!(c.release_retired_root(&held));
    let replacement = reserved_root(c, &m.reference, ReferenceRootKind::Notification, &m.owner);
    assert_ne!(replacement.identity, held.identity);
    assert!(
        c.begin_activate_root(snapshot(c), held.try_clone().unwrap())
            .is_none()
    );
}

#[test]
fn reference_retirement_cleans_embedded_roots_before_releasing_its_registry_slot() {
    let c = catalog();
    let target = cell(&c);
    let source = root(&c, &target, 0);
    let reference = reference(&c, 0);
    publish(&c, &source, &reference);
    let unused = reserved_root(&c, &reference, ReferenceRootKind::Construction, &target);
    let slot = reference.try_share().unwrap();
    drop(reference);
    commit(
        &c,
        c.begin_retire_reference(snapshot(&c), slot.try_clone().unwrap()),
    );
    assert!(!c.release_retired_reference(&slot));
    commit(
        &c,
        c.begin_cancel_root(snapshot(&c), unused.try_share().unwrap()),
    );
    assert!(!unused.removal_requested());
    assert!(c.release_retired_root(&unused.try_share().unwrap()));
    assert!(c.release_retired_reference(&slot));
    assert!(
        c.begin_reserve_reference_owner(
            snapshot(&c),
            slot.try_clone().unwrap(),
            unused.try_share().unwrap()
        )
        .is_none()
    );
}

#[test]
fn reserved_root_adoption_rejects_a_different_exact_reference_with_the_same_owner() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    m.advance();
    let c = &m.c;
    let other = reference_controller_clone_slot(c, &m.reference);
    let wrong = reserved_root(c, &other, ReferenceRootKind::Notification, &m.owner);
    assert!(
        c.begin_scan_reference_with_root(snapshot(c), 0, wrong.try_share().unwrap())
            .is_none()
    );
    assert_eq!(snapshot(c).phase().reference_cursors[0], 0);
    assert_eq!(wrong.status(), RootStatus::Prepared);
    let right = reserved_root(c, &m.reference, ReferenceRootKind::Notification, &m.owner);
    commit(
        c,
        c.begin_scan_reference_with_root(snapshot(c), 0, right.try_share().unwrap()),
    );
    assert!(m.state().queued_root.is_some());
}

#[test]
fn failed_embedded_root_discovery_cannot_skip_a_reserved_construction_root() {
    for failure in [0, 1] {
        let c = catalog();
        let target = cell(&c);
        let source = root(&c, &target, 0);
        let reference = reference(&c, 0);
        publish(&c, &source, &reference);
        let reserved = reserved_root(&c, &reference, ReferenceRootKind::Construction, &target);
        commit(
            &c,
            c.begin_reserve_reference_owner(
                snapshot(&c),
                reference.try_share().unwrap(),
                reserved.try_share().unwrap(),
            ),
        );
        for _ in 0..c.external_root_capacity() {
            commit(&c, c.begin_scan_root(snapshot(&c), 0));
        }
        let before = snapshot(&c).phase().root_cursors[0];
        let current = snapshot(&c);
        FAIL_LOAD.with(|n| n.set(Some(failure)));
        assert!(c.begin_scan_root(current, 0).is_none());
        assert_eq!(snapshot(&c).phase().root_cursors[0], before);
        commit(&c, c.begin_scan_root(snapshot(&c), 0));
        assert_eq!(snapshot(&c).phase().root_cursors[0], before + 1);
    }
}

#[test]
fn notification_plan_retains_its_exact_work_across_failed_owning_acquisitions() {
    for failure in 0..4 {
        let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
        m.advance();
        let c = &m.c;
        let ReferenceScan::NeedsRoot(plan) = c.begin_reference_scan(snapshot(c), 0).unwrap() else {
            panic!()
        };
        assert_eq!(plan.reference_identity(), m.reference.identity);
        assert_eq!(plan.reference_slot(), m.reference.slot);
        assert_eq!(
            plan.owner(),
            ObjectKey {
                identity: m.owner.identity,
                slot: m.owner.slot()
            }
        );
        let before = m.target.membership();
        FAIL_LOAD.with(|n| n.set(Some(0)));
        assert!(matches!(
            plan.begin_root_creation(c),
            Err(RootPrepareError::Contended)
        ));
        let mut creation = plan.begin_root_creation(c).unwrap();
        assert!(creation.poll(c).unwrap().is_none());
        let notification = creation.poll(c).unwrap().unwrap();
        let root = notification.try_share().unwrap();
        FAIL_SHARE.with(|n| n.set(Some(failure)));
        assert!(plan.try_finish(c, root).is_none());
        assert_eq!(m.target.membership(), before);
        assert_eq!(snapshot(c).phase().reference_cursors[0], 0);
        assert!(!m.state().activated);
        commit(c, plan.try_finish(c, notification.try_share().unwrap()));
        assert!(m.state().queued_root.is_some());
    }
}

#[test]
fn a_deferred_notification_plan_cannot_refresh_its_snapshot_after_explicit_clear() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    m.advance();
    let c = &m.c;
    let ReferenceScan::NeedsRoot(plan) = c.begin_reference_scan(snapshot(c), 0).unwrap() else {
        panic!()
    };
    let mut creation = plan.begin_root_creation(c).unwrap();
    assert!(creation.poll(c).unwrap().is_none());
    let notification = creation.poll(c).unwrap().unwrap();
    commit(
        c,
        c.begin_clear_reference(snapshot(c), m.reference.try_share().unwrap()),
    );
    let stale = prepare(c, plan.try_finish(c, notification.try_share().unwrap()));
    assert_eq!(c.commit_in(&stale, &7), Outcome::Retry);
    assert_eq!(notification.status(), RootStatus::Prepared);
    assert!(m.state().queued_root.is_none());
    let ReferenceScan::Ready(preparation) = c.begin_reference_scan(snapshot(c), 0).unwrap() else {
        panic!()
    };
    commit(c, Some(preparation));
    assert_eq!(snapshot(c).phase().domains.map(|d| d.pending), [0, 0]);
    commit(
        c,
        c.begin_cancel_root(snapshot(c), notification.try_share().unwrap()),
    );
    assert!(c.release_retired_root(&notification.try_share().unwrap()));
}

#[test]
fn an_independent_scan_finishes_a_reserved_notification_without_the_creators_plan_or_token() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    m.advance();
    let c = &m.c;
    let ReferenceScan::NeedsRoot(origin) = c.begin_reference_scan(snapshot(c), 0).unwrap() else {
        panic!()
    };
    let mut creation = origin.begin_root_creation(c).unwrap();
    assert!(creation.poll(c).unwrap().is_none());
    let token = creation.poll(c).unwrap().unwrap();
    let stale = prepare(c, origin.try_finish(c, token.try_share().unwrap()));
    let ReferenceScan::NeedsRoot(helper) = c.begin_reference_scan(snapshot(c), 0).unwrap() else {
        panic!()
    };
    FAIL_LOAD.with(|n| n.set(Some(0)));
    assert!(helper.discover_root().is_err());
    let discovered = helper.discover_root().unwrap().unwrap();
    commit(c, helper.try_finish(c, discovered));
    assert_eq!(c.commit_in(&stale, &7), Outcome::Retry);
    assert_eq!(token.status(), RootStatus::Live);
    assert!(m.state().queued_root.is_some());
    assert_eq!(snapshot(c).phase().domains.map(|d| d.pending), [1, 1]);
    drop(token);
    let caller = root(c, &m.owner, 3);
    commit(
        c,
        c.begin_take_reference(
            snapshot(c),
            m.reference.try_share().unwrap(),
            caller.try_share().unwrap(),
        ),
    );
    assert_eq!(caller.status(), RootStatus::Live);
}

#[test]
fn a_nested_helper_retires_and_reuses_the_caller_slot_before_the_original_root_cas() {
    let m = Managed::new(ReferenceKind::Final, false, Some(TraceRetention::Final));
    m.advance();
    let c = Arc::new(m.c);
    let notification = root(&c, &m.owner, 2);
    commit(
        &c,
        c.begin_scan_reference_with_root(snapshot(&c), 0, notification.try_share().unwrap()),
    );
    let caller = root(&c, &m.owner, 3);
    let old = caller.try_share().unwrap();
    let take = Arc::new(prepare(
        &c,
        c.begin_take_reference(
            snapshot(&c),
            m.reference.try_share().unwrap(),
            caller.try_share().unwrap(),
        ),
    ));
    assert!(c.install_and_stop(&take));
    caller.pause_next_effect({
        let c = c.clone();
        let take = take.clone();
        let old = old.try_clone().unwrap();
        let owner = m.owner.try_clone().unwrap();
        move || {
            assert_eq!(c.commit_in(&take, &7), Outcome::Committed);
            commit(
                &c,
                c.begin_remove_root(snapshot(&c), old.try_clone().unwrap()),
            );
            assert!(c.release_retired_root(&old));
            let replacement = root(&c, &owner, 3);
            assert_ne!(old.identity, replacement.identity);
        }
    });
    drop(caller);
    c.help_record_at(&7, 1, || {});
    assert_eq!(
        old.status(),
        RootStatus::Retired,
        "late actual CAS revived a consumed root"
    );
    let replacement = c.discover_root(3).unwrap().unwrap();
    assert_ne!(replacement.identity, old.identity);
    assert_eq!(replacement.status(), RootStatus::Prepared);
}

#[test]
fn retry_reserved_reference_root_rejects_rotation_without_losing_its_fixed_role() {
    let c = catalog();
    let cell = cell(&c);
    let reference = reference(&c, 0);
    for kind in [
        ReferenceRootKind::Construction,
        ReferenceRootKind::Notification,
    ] {
        let mut create = c
            .begin_create_reference_root(
                reference.try_share().unwrap(),
                kind,
                cell.try_clone().unwrap(),
            )
            .unwrap();
        assert!(matches!(
            create.poll_rotating(&c),
            Err(CreationError::FixedLocation)
        ));
        assert!(create.poll(&c).unwrap().is_none());
        assert!(matches!(
            create.poll_rotating(&c),
            Err(CreationError::FixedLocation)
        ));
        let root = create.poll(&c).unwrap().expect("original fixed slot");
        assert!(root.is_internal());
        assert_eq!(root.target_identity(), cell.identity);
        assert_eq!(
            c.discover_root(root.slot()).unwrap().unwrap().identity,
            root.identity
        );
    }
}

#[test]
fn scan_batch_preserves_reserved_construction_custody_and_failed_discovery_boundary() {
    for failure in [0, 1] {
        let c = catalog();
        let target = cell(&c);
        let source = root(&c, &target, 0);
        let reference = reference(&c, 0);
        publish(&c, &source, &reference);
        let reserved = reserved_root(&c, &reference, ReferenceRootKind::Construction, &target);
        commit(
            &c,
            c.begin_reserve_reference_owner(
                snapshot(&c),
                reference.try_share().unwrap(),
                reserved.try_share().unwrap(),
            ),
        );
        for _ in 0..c.external_root_capacity() {
            commit(&c, c.begin_scan_root(snapshot(&c), 0));
        }
        let before = snapshot(&c).phase().root_cursors[0];
        let current = snapshot(&c);
        FAIL_LOAD.with(|n| n.set(Some(failure)));
        assert!(c.begin_scan_root_batch(current, 0).is_none());
        assert_eq!(snapshot(&c).phase().root_cursors[0], before);
        commit(&c, c.begin_scan_root_batch(snapshot(&c), 0));
        assert_eq!(snapshot(&c).phase().root_cursors[0], c.root_capacity());
        assert_eq!(reserved.status(), RootStatus::Live);
        assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1, 1]);
        let ReferenceMembership::Live(state) = reference.membership().unwrap() else {
            panic!("reserved reference is not live");
        };
        assert_eq!(state.construction_root.unwrap().identity, reserved.identity);
    }
}
