use super::super::{Attempt, Cell, Membership, Outcome, Preparation, RootToken, Snapshot, pool};
use super::*;
use crate::canonical_catalog::reference::ObjectKey;
use crate::canonical_catalog::{DomainMode, FAIL_LOAD, FAIL_SHARE, RETIREMENT_FLOOR};

pub(crate) fn catalog() -> Catalog {
    Catalog::with_record_root_capacities_and_limit(3, 256, 256, 128, 8, 64, pool::MAX_GENERATION)
        .with_edge_capacities(2, 64, 128, pool::MAX_GENERATION)
}

pub(crate) fn snapshot(c: &Catalog) -> Snapshot {
    for _ in 0..32 {
        if let Some(s) = c.snapshot_in(&()) {
            return s;
        }
    }
    panic!("edge controller did not settle");
}

fn prepare(c: &Catalog, p: Option<Preparation>) -> Attempt {
    let mut p = p.expect("valid edge preparation");
    for _ in 0..32 {
        if let Some(a) = p.poll(c).unwrap() {
            return a;
        }
    }
    panic!("edge preparation did not settle");
}

pub(crate) fn commit(c: &Catalog, p: Option<Preparation>) {
    let a = prepare(c, p);
    for _ in 0..32 {
        match c.commit_in(&a, &()) {
            Outcome::Pending => {}
            Outcome::Committed => return,
            other => panic!("edge commit: {other:?}"),
        }
    }
    panic!("edge commit did not settle");
}

fn root(c: &Catalog, object: &Owned<Cell>, slot: usize) -> RootToken {
    let mut creation = c
        .begin_create_root(slot, object.try_clone().unwrap())
        .unwrap();
    for _ in 0..32 {
        if let Some(root) = creation.poll(c).unwrap() {
            return root;
        }
    }
    panic!("root did not announce");
}

pub(crate) fn object(c: &Catalog, slot: usize) -> (Owned<Cell>, RootToken) {
    let mut creation = c.begin_create_with(slot, slot + 8, ()).unwrap();
    for _ in 0..32 {
        if let Some(object) = creation.poll(c).unwrap() {
            let root = root(c, &object, slot);
            commit(
                c,
                c.begin_publish_root(snapshot(c), root.try_share().unwrap()),
            );
            return (object, root);
        }
    }
    panic!("object did not announce");
}

fn edge(c: &Catalog) -> EdgeToken {
    let mut creation = c.begin_create_edge(0).unwrap();
    for _ in 0..32 {
        if let Some(edge) = creation.poll(c).unwrap() {
            return edge;
        }
    }
    panic!("edge did not announce");
}

pub(crate) fn key(object: &Cell) -> ObjectKey {
    ObjectKey {
        identity: object.identity,
        slot: object.slot(),
    }
}

fn state(edge: &EdgeSlot) -> EdgeState {
    let Some(EdgeMembership::Live(state)) = edge.membership() else {
        panic!("edge not live")
    };
    state
}

pub(crate) fn acknowledge(c: &Catalog, object: &Owned<Cell>) {
    let Some(Membership::Tracing(marks)) = object.membership() else {
        panic!("not tracing")
    };
    for domain in 0..2 {
        if marks[domain].ticket != 0 {
            commit(
                c,
                c.begin_trace_acknowledge(
                    snapshot(c),
                    object.try_clone().unwrap(),
                    domain,
                    marks[domain].ticket,
                ),
            );
        }
    }
}

pub(crate) fn restart(c: &Catalog, objects: &[&Owned<Cell>]) {
    for object in objects {
        acknowledge(c, object);
    }
    for domain in 0..2 {
        for _ in 0..c.root_capacity() {
            commit(c, c.begin_scan_root(snapshot(c), domain));
        }
        for _ in 0..RETIREMENT_FLOOR {
            commit(c, c.begin_advance(snapshot(c), domain));
        }
        commit(c, c.begin_finish(snapshot(c), domain));
    }
    // Root scans apply barriers to both active domains. Finish both old epochs
    // before restarting either one, so this fixture starts with no new tickets.
    for domain in 0..2 {
        commit(c, c.begin_start(snapshot(c), domain, DomainMode::Full));
    }
    assert_eq!(snapshot(c).phase().domains.map(|d| d.pending), [0; 2]);
}

#[test]
fn edge_publication_replacement_null_and_exact_slot_release() {
    let c = catalog();
    let (owner, source) = object(&c, 0);
    let (a, target_a) = object(&c, 1);
    let (b, target_b) = object(&c, 2);
    let edge = edge(&c);
    assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_a.try_share().unwrap()),
        ),
    );
    assert_eq!(
        state(&edge),
        EdgeState {
            source: key(&owner),
            target: Some(key(&a))
        }
    );
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_b.try_share().unwrap()),
        ),
    );
    assert_eq!(state(&edge).target, Some(key(&b)));
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            None,
        ),
    );
    assert_eq!(state(&edge).target, None);
    let old = edge.try_share().unwrap();
    drop(edge);
    commit(
        &c,
        c.begin_requested_edge_removal(snapshot(&c), old.try_clone().unwrap()),
    );
    assert_eq!(old.membership(), Some(EdgeMembership::Retired));
    assert!(c.release_retired_edge(&old));
    let replacement = self::edge(&c);
    assert_ne!(replacement.identity, old.identity);
    assert!(!c.release_retired_edge(&old));
    assert_eq!(
        c.discover_edge(0).unwrap().unwrap().identity,
        replacement.identity
    );
}

#[test]
fn edge_same_target_installs_one_ticket_per_domain() {
    let c = catalog();
    let (owner, source) = object(&c, 0);
    let (target, target_root) = object(&c, 1);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_root.try_share().unwrap()),
        ),
    );
    restart(&c, &[&owner, &target]);
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_root.try_share().unwrap()),
        ),
    );
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1; 2]);
    acknowledge(&c, &target);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [0; 2]);
}

#[test]
fn edge_distinct_old_and_new_targets_keep_exact_pending_counts() {
    let c = catalog();
    let (owner, source) = object(&c, 0);
    let (a, target_a) = object(&c, 1);
    let (b, target_b) = object(&c, 2);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_a.try_share().unwrap()),
        ),
    );
    restart(&c, &[&owner, &a, &b]);
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_b.try_share().unwrap()),
        ),
    );
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [2; 2]);
    acknowledge(&c, &a);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1; 2]);
    acknowledge(&c, &b);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [0; 2]);
}

#[test]
fn edge_child_root_rejects_replacement_and_source_revocation_races() {
    let c = catalog();
    let (_, source) = object(&c, 0);
    let (a, target_a) = object(&c, 1);
    let (b, target_b) = object(&c, 2);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_a.try_share().unwrap()),
        ),
    );
    let child_a = root(&c, &a, 3);
    let read_a = prepare(
        &c,
        c.begin_root_edge_target(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            child_a.try_share().unwrap(),
        ),
    );
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_b.try_share().unwrap()),
        ),
    );
    assert_eq!(c.commit_in(&read_a, &()), Outcome::Retry);
    assert_eq!(child_a.status(), RootStatus::Prepared);
    assert!(
        c.begin_root_edge_target(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            child_a.try_share().unwrap()
        )
        .is_none()
    );
    let child_b = root(&c, &b, 4);
    let read_b = prepare(
        &c,
        c.begin_root_edge_target(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            child_b.try_share().unwrap(),
        ),
    );
    commit(
        &c,
        c.begin_remove_root(snapshot(&c), source.try_share().unwrap()),
    );
    assert_eq!(c.commit_in(&read_b, &()), Outcome::Retry);
    assert!(
        c.begin_root_edge_target(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            child_b.try_share().unwrap()
        )
        .is_none()
    );
    assert_eq!(child_b.status(), RootStatus::Prepared);
}

#[test]
fn edge_private_cancellation_and_failed_observations_preserve_work() {
    let c = catalog();
    let (owner, source) = object(&c, 0);
    let (a, target_a) = object(&c, 1);
    let (b, target_b) = object(&c, 2);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_a.try_share().unwrap()),
        ),
    );
    restart(&c, &[&owner, &a, &b]);
    let before = snapshot(&c).phase();
    drop(
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_b.try_share().unwrap()),
        )
        .unwrap(),
    );
    assert_eq!(snapshot(&c).phase(), before);
    assert_eq!(state(&edge).target, Some(key(&a)));
    for failure in 0..7 {
        let s = snapshot(&c);
        let edge = edge.try_share().unwrap();
        let source = source.try_share().unwrap();
        let target = target_b.try_share().unwrap();
        FAIL_LOAD.with(|slot| slot.set(Some(failure)));
        let result = c.begin_replace_edge(s, edge, source, Some(target));
        FAIL_LOAD.with(|slot| slot.set(None));
        assert!(result.is_none(), "failed observation {failure} was ignored");
        assert_eq!(snapshot(&c).phase(), before);
    }
    assert_eq!(state(&edge).target, Some(key(&a)));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn edge_stopped_effect_is_helped_and_cannot_overwrite_a_later_version() {
    let c = Arc::new(catalog());
    let (owner, source) = object(&c, 0);
    let (a, target_a) = object(&c, 1);
    let (b, target_b) = object(&c, 2);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_a.try_share().unwrap()),
        ),
    );
    restart(&c, &[&owner, &a, &b]);
    let attempt = prepare(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_b.try_share().unwrap()),
        ),
    );
    let (observer, old, source) = (
        c.clone(),
        edge.try_share().unwrap(),
        source.try_share().unwrap(),
    );
    edge.pause_next_effect(move || {
        assert_eq!(
            snapshot(&observer).phase().domains.map(|d| d.pending),
            [2; 2]
        );
        assert_eq!(state(&old).target, Some(key(&b)));
        commit(
            &observer,
            observer.begin_replace_edge(snapshot(&observer), old, source, None),
        );
    });
    assert!(c.install_and_stop(&attempt));
    snapshot(&c);
    assert_eq!(c.commit_in(&attempt, &()), Outcome::Committed);
    assert_eq!(
        state(&edge).target,
        None,
        "late effect restored replaced edge"
    );
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [2; 2]);
}

#[test]
fn edge_child_root_committed_before_unlink_keeps_independent_protection() {
    let c = catalog();
    let (owner, source) = object(&c, 0);
    let (target, target_root) = object(&c, 1);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_root.try_share().unwrap()),
        ),
    );
    let child = root(&c, &target, 3);
    commit(
        &c,
        c.begin_root_edge_target(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            child.try_share().unwrap(),
        ),
    );
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            None,
        ),
    );
    for root in [&source, &target_root] {
        commit(
            &c,
            c.begin_remove_root(snapshot(&c), root.try_share().unwrap()),
        );
    }
    restart(&c, &[&owner, &target]);
    for _ in 0..c.root_capacity() {
        commit(&c, c.begin_scan_root(snapshot(&c), 0));
    }
    assert_eq!(child.status(), RootStatus::Live);
    assert_eq!(state(&edge).target, None);
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [1; 2]);
    acknowledge(&c, &target);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    assert!(
        c.begin_trace_retire(snapshot(&c), target, 0).is_none(),
        "unlink revoked the independently committed child root"
    );
}

#[test]
fn edge_foreign_owners_and_snapshots_cannot_change_local_state() {
    let c = catalog();
    let foreign = catalog();
    let (_, source) = object(&c, 0);
    let (_, target) = object(&c, 1);
    let (other_cell, other_root) = object(&foreign, 0);
    let edge = edge(&c);
    let other_edge = self::edge(&foreign);
    let before = snapshot(&c).phase();
    for (s, e, owner, child) in [
        (
            snapshot(&foreign),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            target.try_share().unwrap(),
        ),
        (
            snapshot(&c),
            other_edge.try_share().unwrap(),
            source.try_share().unwrap(),
            target.try_share().unwrap(),
        ),
        (
            snapshot(&c),
            edge.try_share().unwrap(),
            other_root.try_share().unwrap(),
            target.try_share().unwrap(),
        ),
        (
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            other_root.try_share().unwrap(),
        ),
    ] {
        assert!(c.begin_publish_edge(s, e, owner, Some(child)).is_none());
    }
    assert_eq!(snapshot(&c).phase(), before);
    assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target.try_share().unwrap()),
        ),
    );
    let before = snapshot(&c).phase();
    let foreign_child = root(&foreign, &other_cell, 1);
    assert!(
        c.begin_root_edge_target(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            foreign_child.try_share().unwrap()
        )
        .is_none()
    );
    assert!(
        c.begin_replace_edge(
            snapshot(&foreign),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            None
        )
        .is_none()
    );
    assert!(
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            target.try_share().unwrap(),
            None
        )
        .is_none()
    );
    let other = other_edge.try_share().unwrap();
    drop(other_edge);
    assert!(
        c.begin_requested_edge_removal(snapshot(&c), other.try_clone().unwrap())
            .is_none()
    );
    assert!(!c.release_retired_edge(&other));
    assert_eq!(snapshot(&c).phase(), before);
}

#[test]
fn edge_creation_cancellation_collision_and_exhaustion_are_finite() {
    let c = catalog();
    assert!(matches!(
        c.begin_create_edge(c.edge_capacity()),
        Err(EdgePrepareError::InvalidSlot)
    ));
    let foreign = catalog();
    let mut a = c.begin_create_edge(0).unwrap();
    let mut b = c.begin_create_edge(0).unwrap();
    assert!(matches!(a.poll(&foreign), Err(CreationError::Foreign)));
    assert!(a.poll(&c).unwrap().is_none());
    drop(a);
    assert!(c.discover_edge(0).unwrap().is_none());
    let edge = edge(&c);
    assert!(matches!(
        c.begin_create_edge(0),
        Err(EdgePrepareError::Occupied)
    ));
    assert!(b.poll(&c).unwrap().is_none());
    assert!(b.poll(&c).unwrap().is_none());
    assert!(matches!(b.poll(&c), Err(CreationError::Occupied)));
    assert!(matches!(b.poll(&c), Err(CreationError::Finished)));
    let observed = edge.try_share().unwrap();
    drop(edge);
    commit(
        &c,
        c.begin_requested_edge_removal(snapshot(&c), observed.try_clone().unwrap()),
    );
    assert!(c.release_retired_edge(&observed));
    assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [0; 2]);

    let tiny: Catalog =
        Catalog::with_record_root_capacities_and_limit(1, 32, 32, 8, 1, 8, pool::MAX_GENERATION)
            .with_edge_capacities(1, 1, 1, 1);
    let mut cancelled = tiny.begin_create_edge(0).unwrap();
    assert!(cancelled.poll(&tiny).unwrap().is_none());
    drop(cancelled);
    let mut exhausted = tiny.begin_create_edge(0).unwrap();
    assert!(matches!(
        exhausted.poll(&tiny),
        Err(CreationError::GenerationExhausted)
    ));
    assert!(tiny.discover_edge(0).unwrap().is_none());
    assert!(tiny.generation_exhaustion_observed());
}

#[test]
fn edge_all_effect_owners_are_acquired_before_commit() {
    for failure in 0..3 {
        let c = catalog();
        let (owner, source) = object(&c, 0);
        let (a, target_a) = object(&c, 1);
        let (b, target_b) = object(&c, 2);
        let edge = edge(&c);
        commit(
            &c,
            c.begin_publish_edge(
                snapshot(&c),
                edge.try_share().unwrap(),
                source.try_share().unwrap(),
                Some(target_a.try_share().unwrap()),
            ),
        );
        restart(&c, &[&owner, &a, &b]);
        let attempt = prepare(
            &c,
            c.begin_replace_edge(
                snapshot(&c),
                edge.try_share().unwrap(),
                source.try_share().unwrap(),
                Some(target_b.try_share().unwrap()),
            ),
        );
        assert!(c.install_and_stop(&attempt));
        FAIL_SHARE.with(|n| n.set(Some(failure)));
        assert!(c.snapshot_in(&()).is_none());
        FAIL_SHARE.with(|n| n.set(None));
        assert!(
            attempt.cancel(),
            "effect owner {failure} acquired after COMMIT"
        );
        assert_eq!(c.commit_in(&attempt, &()), Outcome::Cancelled);
        assert_eq!(state(&edge).target, Some(key(&a)));
        assert_eq!(snapshot(&c).phase().domains.map(|d| d.pending), [0; 2]);
    }
}

#[test]
fn edge_stale_object_keys_cannot_attach_to_reused_slots() {
    let c = catalog();
    let (owner, source) = object(&c, 0);
    let (target, target_root) = object(&c, 1);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_root.try_share().unwrap()),
        ),
    );
    let old_state = state(&edge);
    for token in [source, target_root] {
        let root = token.try_share().unwrap();
        commit(
            &c,
            c.begin_remove_root(snapshot(&c), root.try_clone().unwrap()),
        );
        drop(token);
        assert!(c.release_retired_root(&root));
    }
    restart(&c, &[&owner, &target]);
    for _ in 0..c.root_capacity() {
        commit(&c, c.begin_scan_root(snapshot(&c), 0));
    }
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    // Both source and target are unreachable in this raw fixture. Their edge
    // retains only identities; it cannot pin this unreachable component.
    for object in [&owner, &target] {
        commit(
            &c,
            c.begin_trace_retire(snapshot(&c), object.try_clone().unwrap(), 0),
        );
        assert!(c.release_retired(object));
    }
    let (new_owner, new_source) = object(&c, 0);
    let (new_target, new_root) = object(&c, 1);
    assert_eq!(old_state.source.slot, new_owner.slot());
    assert_ne!(old_state.source.identity, new_owner.identity);
    assert_eq!(old_state.target.unwrap().slot, new_target.slot());
    assert_ne!(old_state.target.unwrap().identity, new_target.identity);
    assert!(
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            new_source.try_share().unwrap(),
            Some(new_root.try_share().unwrap())
        )
        .is_none()
    );
    let before = snapshot(&c).phase().domains;
    let membership = new_target.membership();
    let old_edge = edge.try_share().unwrap();
    drop(edge);
    commit(
        &c,
        c.begin_requested_edge_removal(snapshot(&c), old_edge.try_clone().unwrap()),
    );
    assert_eq!(snapshot(&c).phase().domains, before);
    assert_eq!(
        new_target.membership(),
        membership,
        "stale deletion shaded slot reuse"
    );
    assert!(c.release_retired_edge(&old_edge));
}

#[test]
fn edge_slot_generation_exhaustion_is_reported_without_reannouncement() {
    let c: Catalog =
        Catalog::with_record_root_capacities_and_limit(1, 32, 32, 8, 1, 8, pool::MAX_GENERATION)
            .with_edge_capacities(1, 1, 8, 1);
    let edge = edge(&c);
    let retained = edge.try_share().unwrap();
    drop(edge);
    commit(
        &c,
        c.begin_requested_edge_removal(snapshot(&c), retained.try_clone().unwrap()),
    );
    assert!(c.release_retired_edge(&retained));
    drop(retained);
    let mut creation = c.begin_create_edge(0).unwrap();
    assert!(creation.poll(&c).unwrap().is_none());
    assert!(matches!(
        creation.poll(&c),
        Err(CreationError::GenerationExhausted)
    ));
    assert!(c.generation_exhaustion_observed());
    assert!(c.discover_edge(0).unwrap().is_none());
}

#[test]
fn edge_source_handoff_requires_publication_and_does_not_request_removal() {
    let c = catalog();
    let (_, source) = object(&c, 0);
    let token = edge(&c);
    let token = match token.release_to_source() {
        Ok(()) => panic!("Prepared edge lost its cancellation owner"),
        Err(token) => token,
    };
    assert!(!token.removal_requested());
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            token.try_share().unwrap(),
            source.try_share().unwrap(),
            None,
        ),
    );
    let observed = token.try_share().unwrap();
    assert!(token.release_to_source().is_ok());
    assert!(!observed.removal_requested());
    assert!(
        c.begin_requested_edge_removal(snapshot(&c), observed.try_clone().unwrap())
            .is_none()
    );
    assert!(
        c.begin_orphan_edge_removal(snapshot(&c), observed.try_clone().unwrap())
            .is_none()
    );
    for failure in 0..3 {
        let s = snapshot(&c);
        let edge = observed.try_clone().unwrap();
        FAIL_LOAD.with(|n| n.set(Some(failure)));
        assert!(c.begin_orphan_edge_removal(s, edge).is_none());
        FAIL_LOAD.with(|n| n.set(None));
        assert!(matches!(
            observed.membership(),
            Some(EdgeMembership::Live(_))
        ));
    }
}

#[test]
fn indexed_stale_sources_cannot_reattach_edges_after_object_slot_reuse() {
    let c = Catalog::with_record_root_capacities_and_limit(
        3,
        256,
        256,
        128,
        8,
        64,
        pool::MAX_GENERATION,
    )
    .with_edge_capacities(6, 64, 128, pool::MAX_GENERATION)
    .with_source_edge_degree(2);
    let (owner, source) = object(&c, 0);
    let (target, target_root) = object(&c, 1);
    let edge = edge(&c);
    commit(
        &c,
        c.begin_publish_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target_root.try_share().unwrap()),
        ),
    );
    let old_state = state(&edge);
    for token in [source, target_root] {
        let root = token.try_share().unwrap();
        commit(
            &c,
            c.begin_remove_root(snapshot(&c), root.try_clone().unwrap()),
        );
        drop(token);
        assert!(c.release_retired_root(&root));
    }
    restart(&c, &[&owner, &target]);
    for _ in 0..c.root_capacity() {
        commit(&c, c.begin_scan_root(snapshot(&c), 0));
    }
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    // Both source and target are unreachable in this raw fixture. Their edge
    // retains only identities; it cannot pin this unreachable component.
    for object in [&owner, &target] {
        commit(
            &c,
            c.begin_trace_retire(snapshot(&c), object.try_clone().unwrap(), 0),
        );
        assert!(c.release_retired(object));
    }
    let (new_owner, new_source) = object(&c, 0);
    let (new_target, new_root) = object(&c, 1);
    assert_eq!(old_state.source.slot, new_owner.slot());
    assert_ne!(old_state.source.identity, new_owner.identity);
    assert_eq!(old_state.target.unwrap().slot, new_target.slot());
    assert_ne!(old_state.target.unwrap().identity, new_target.identity);
    assert!(
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            new_source.try_share().unwrap(),
            Some(new_root.try_share().unwrap())
        )
        .is_none()
    );
    let before = snapshot(&c).phase().domains;
    let membership = new_target.membership();
    let old_edge = edge.try_share().unwrap();
    drop(edge);
    commit(
        &c,
        c.begin_requested_edge_removal(snapshot(&c), old_edge.try_clone().unwrap()),
    );
    assert_eq!(snapshot(&c).phase().domains, before);
    assert_eq!(
        new_target.membership(),
        membership,
        "stale deletion shaded slot reuse"
    );
    assert!(c.release_retired_edge(&old_edge));
}

#[test]
#[cfg(not(target_family = "wasm"))]
#[should_panic(expected = "source degree already configured")]
fn indexed_geometry_cannot_be_reconfigured() {
    let _ = Catalog::<()>::with_record_root_capacities_and_limit(
        1,
        32,
        32,
        8,
        1,
        8,
        pool::MAX_GENERATION,
    )
    .with_edge_capacities(2, 8, 16, pool::MAX_GENERATION)
    .with_source_edge_degree(2)
    .with_source_edge_degree(2);
}

#[test]
#[cfg(not(target_family = "wasm"))]
#[should_panic(expected = "setup owner already shared")]
fn indexed_geometry_cannot_be_added_after_publication() {
    let c = Catalog::<()>::with_record_root_capacities_and_limit(
        1,
        32,
        32,
        8,
        1,
        8,
        pool::MAX_GENERATION,
    )
    .with_edge_capacities(2, 8, 16, pool::MAX_GENERATION);
    let (_object, _root) = object(&c, 0);
    let _ = c.with_source_edge_degree(2);
}
