use super::super::edges::EdgeToken;
use super::super::edges::transaction_tests::{
    acknowledge, catalog, commit, key, object, restart, snapshot,
};
use super::super::{RootToken, pool};
use super::*;
use crate::canonical_catalog::{FAIL_LOAD, RETIREMENT_FLOOR};

fn edge(c: &Catalog, slot: usize, source: &RootToken, target: Option<&RootToken>) -> EdgeToken {
    let mut creation = c.begin_create_edge(slot).unwrap();
    let edge = (0..32).find_map(|_| creation.poll(c).unwrap()).unwrap();
    commit(
        c,
        c.begin_publish_edge(
            snapshot(c),
            edge.try_share().unwrap(),
            source.try_share().unwrap(),
            target.map(|root| root.try_share().unwrap()),
        ),
    );
    edge
}
fn remove(c: &Catalog, root: &RootToken) {
    commit(
        c,
        c.begin_remove_root(snapshot(c), root.try_share().unwrap()),
    );
}
fn scan_roots(c: &Catalog) {
    for _ in 0..c.root_capacity() {
        commit(c, c.begin_scan_root(snapshot(c), 0));
    }
}
fn begin(c: &Catalog, source: &Pooled<Cell>) -> EdgeTraversal {
    let Some(Membership::Tracing(marks)) = source.membership() else {
        panic!()
    };
    c.begin_edge_traversal(snapshot(c), source.try_clone().unwrap(), 0, marks[0].ticket)
        .unwrap()
}
fn finish(c: &Catalog, traversal: &mut EdgeTraversal) -> EdgeTraversalProgress {
    for _ in 0..128 {
        let progress = traversal.poll(c, &()).unwrap();
        if progress != EdgeTraversalProgress::Pending {
            return progress;
        }
    }
    panic!("edge traversal did not settle");
}

#[test]
fn edge_traversal_follows_a_chain_before_acknowledging_each_ticket() {
    let c = catalog();
    let (a, ar) = object(&c, 0);
    let (b, br) = object(&c, 1);
    let (d, dr) = object(&c, 2);
    let _ab = edge(&c, 0, &ar, Some(&br));
    let _bd = edge(&c, 1, &br, Some(&dr));
    remove(&c, &br);
    remove(&c, &dr);
    restart(&c, &[&a, &b, &d]);
    scan_roots(&c);
    for source in [&a, &b, &d] {
        assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
        assert!(c.begin_advance(snapshot(&c), 0).is_none());
        let mut scan = begin(&c, source);
        assert_eq!(finish(&c, &mut scan), EdgeTraversalProgress::Complete);
        assert_eq!(scan.scanned_slots(), c.edge_capacity());
        assert_eq!(scan.poll(&c, &()), Err(EdgeTraversalError::Finished));
    }
    assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    for source in [a, b, d] {
        assert!(
            c.begin_trace_retire(snapshot(&c), source, 0).is_none(),
            "transitively reachable object became reclaimable"
        );
    }
}

#[test]
fn edge_traversal_stopped_private_scanners_do_not_own_ticket_completion() {
    for stopped in 0..7 {
        let c = catalog();
        let (a, ar) = object(&c, 0);
        let (b, br) = object(&c, 1);
        let _edge = edge(&c, 0, &ar, Some(&br));
        remove(&c, &br);
        restart(&c, &[&a, &b]);
        scan_roots(&c);
        let mut old = begin(&c, &a);
        for _ in 0..stopped {
            assert_eq!(old.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
        }
        let mut other = begin(&c, &a);
        assert_eq!(finish(&c, &mut other), EdgeTraversalProgress::Complete);
        assert_eq!(finish(&c, &mut old), EdgeTraversalProgress::Superseded);
        assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
        assert_eq!(
            finish(&c, &mut begin(&c, &b)),
            EdgeTraversalProgress::Complete
        );
        assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
    }
}

#[test]
fn edge_traversal_stronger_source_ticket_requires_a_new_complete_scan() {
    let c = catalog();
    let (a, ar) = object(&c, 0);
    let (b, br) = object(&c, 1);
    let _edge = edge(&c, 0, &ar, Some(&br));
    restart(&c, &[&a, &b]);
    commit(
        &c,
        c.begin_shade(
            snapshot(&c),
            a.try_clone().unwrap(),
            0,
            TraceRetention::Phantom,
        ),
    );
    let mut old = begin(&c, &a);
    while old.scanned_slots() == 0 {
        assert_eq!(old.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
    }
    commit(
        &c,
        c.begin_shade(
            snapshot(&c),
            a.try_clone().unwrap(),
            0,
            TraceRetention::Strong,
        ),
    );
    assert_eq!(finish(&c, &mut old), EdgeTraversalProgress::Superseded);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 2);
    assert_eq!(
        finish(&c, &mut begin(&c, &a)),
        EdgeTraversalProgress::Complete
    );
    let Some(Membership::Tracing(marks)) = b.membership() else {
        panic!()
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Strong));
    assert_ne!(marks[0].ticket, 0);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
}

#[test]
fn edge_traversal_insertion_behind_cursor_retains_target_work() {
    let c = catalog();
    let (a, ar) = object(&c, 0);
    let (b, br) = object(&c, 1);
    let edge = edge(&c, 0, &ar, None);
    restart(&c, &[&a, &b]);
    commit(
        &c,
        c.begin_shade(
            snapshot(&c),
            a.try_clone().unwrap(),
            0,
            TraceRetention::Strong,
        ),
    );
    let mut scan = begin(&c, &a);
    assert_eq!(scan.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
    assert_eq!(scan.scanned_slots(), 1);
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            ar.try_share().unwrap(),
            Some(br.try_share().unwrap()),
        ),
    );
    remove(&c, &br);
    assert_eq!(finish(&c, &mut scan), EdgeTraversalProgress::Complete);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
    let Some(Membership::Tracing(marks)) = b.membership() else {
        panic!()
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Strong));
    assert_ne!(marks[0].ticket, 0);
}

#[test]
fn edge_traversal_failed_observations_do_not_advance_or_acknowledge() {
    let c = catalog();
    let (a, ar) = object(&c, 0);
    let (b, br) = object(&c, 1);
    let _edge = edge(&c, 0, &ar, Some(&br));
    remove(&c, &br);
    restart(&c, &[&a, &b]);
    scan_roots(&c);
    let before = snapshot(&c).phase();
    for failure in 0..6 {
        let mut scan = begin(&c, &a);
        FAIL_LOAD.with(|n| n.set(Some(failure)));
        assert_eq!(scan.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
        FAIL_LOAD.with(|n| n.set(None));
        assert_eq!(
            scan.scanned_slots(),
            0,
            "ignored observation failure {failure}"
        );
        assert_eq!(snapshot(&c).phase(), before);
    }
    assert_eq!(
        finish(&c, &mut begin(&c, &a)),
        EdgeTraversalProgress::Complete
    );
}

#[test]
fn edge_inventory_does_not_retain_an_unreachable_cycle() {
    let c = catalog();
    let (a, ar) = object(&c, 0);
    let (b, br) = object(&c, 1);
    let ab = edge(&c, 0, &ar, Some(&br));
    let ba = edge(&c, 1, &br, Some(&ar));
    let retained = [ab.try_share().unwrap(), ba.try_share().unwrap()];
    assert!(ab.release_to_source().is_ok());
    assert!(ba.release_to_source().is_ok());
    remove(&c, &ar);
    remove(&c, &br);
    restart(&c, &[&a, &b]);
    scan_roots(&c);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    for object in [&a, &b] {
        commit(
            &c,
            c.begin_trace_retire(snapshot(&c), object.try_clone().unwrap(), 0),
        );
        assert!(c.release_retired(object));
    }
    for retained in retained {
        assert!(!retained.removal_requested());
        commit(
            &c,
            c.begin_orphan_edge_removal(snapshot(&c), retained.try_clone().unwrap()),
        );
        assert!(c.release_retired_edge(&retained));
    }
    assert!(c.discover(key(&a).slot).unwrap().is_none());
    assert!(c.discover(key(&b).slot).unwrap().is_none());
}

#[test]
fn edge_traversal_rejects_foreign_and_unsupported_domain_authority() {
    let c = catalog();
    let foreign = catalog();
    let (a, _ar) = object(&c, 0);
    let (b, _) = object(&foreign, 0);
    let Some(Membership::Tracing(marks)) = a.membership() else {
        panic!()
    };
    for (s, cell, domain, ticket) in [
        (
            snapshot(&foreign),
            a.try_clone().unwrap(),
            0,
            marks[0].ticket,
        ),
        (snapshot(&c), b, 0, marks[0].ticket),
        (snapshot(&c), a.try_clone().unwrap(), 2, marks[0].ticket),
        (snapshot(&c), a.try_clone().unwrap(), 0, 0),
    ] {
        assert!(c.begin_edge_traversal(s, cell, domain, ticket).is_none());
    }
    let mut scan = begin(&c, &a);
    assert_eq!(scan.poll(&foreign, &()), Err(EdgeTraversalError::Foreign));
    assert_eq!(scan.scanned_slots(), 0);
    acknowledge(&c, &a);
    assert_eq!(
        scan.poll(&c, &()).unwrap(),
        EdgeTraversalProgress::Superseded
    );
    scan_roots(&c);
    for _ in 0..RETIREMENT_FLOOR {
        commit(&c, c.begin_advance(snapshot(&c), 0));
    }
    commit(&c, c.begin_finish(snapshot(&c), 0));
    commit(&c, c.begin_start(snapshot(&c), 0, DomainMode::Young));
    scan_roots(&c);
    let Some(Membership::Tracing(marks)) = a.membership() else {
        panic!()
    };
    assert_ne!(marks[0].ticket, 0);
    assert!(
        c.begin_edge_traversal(snapshot(&c), a, 0, marks[0].ticket)
            .is_none()
    );
}

#[test]
fn edge_traversal_installed_acknowledgement_is_completed_by_another_helper() {
    let c = catalog();
    let (a, ar) = object(&c, 0);
    let _edge = edge(&c, 0, &ar, Some(&ar));
    let mut scan = begin(&c, &a);
    for _ in 0..32 {
        if scan.install_pending_for_test(&c) {
            break;
        }
        assert_eq!(scan.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
    }
    // Busy is installed but this scanner is never polled during completion.
    assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
    assert_eq!(finish(&c, &mut scan), EdgeTraversalProgress::Complete);
}

fn indexed_catalog() -> Catalog {
    Catalog::with_record_root_capacities_and_limit(3, 256, 256, 128, 8, 64, pool::MAX_GENERATION)
        .with_edge_capacities(6, 64, 128, pool::MAX_GENERATION)
        .with_source_edge_degree(2)
}

#[test]
fn indexed_edges_reject_misplaced_publication_before_any_effect() {
    let c = indexed_catalog();
    let (_, source) = object(&c, 1);
    let (_, target) = object(&c, 2);
    let mut creation = c.begin_create_edge(0).unwrap();
    let misplaced = (0..32).find_map(|_| creation.poll(&c).unwrap()).unwrap();
    let before = snapshot(&c).phase();
    assert!(
        c.begin_publish_edge(
            snapshot(&c),
            misplaced.try_share().unwrap(),
            source.try_share().unwrap(),
            Some(target.try_share().unwrap())
        )
        .is_none(),
        "indexed publication escaped source range"
    );
    assert_eq!(snapshot(&c).phase(), before);
    assert_eq!(misplaced.membership(), Some(EdgeMembership::Prepared));
    let valid = edge(&c, 3, &source, Some(&target));
    assert!(matches!(valid.membership(), Some(EdgeMembership::Live(_))));
}

#[test]
fn indexed_traversal_checks_the_last_source_slot_without_scanning_other_sources() {
    let c = indexed_catalog();
    let (source, sr) = object(&c, 1);
    let (target, tr) = object(&c, 2);
    let _edge = edge(&c, 3, &sr, Some(&tr));
    remove(&c, &tr);
    restart(&c, &[&source, &target]);
    scan_roots(&c);
    let mut scan = begin(&c, &source);
    assert_eq!(finish(&c, &mut scan), EdgeTraversalProgress::Complete);
    assert_eq!(
        scan.scanned_slots(),
        2,
        "indexed traversal did not use the complete source range"
    );
    let Some(Membership::Tracing(marks)) = target.membership() else {
        panic!()
    };
    assert_eq!(
        marks[0].retention,
        Some(TraceRetention::Strong),
        "last indexed edge was skipped"
    );
    assert_ne!(marks[0].ticket, 0, "last indexed edge was skipped");
    assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
    let mut child = begin(&c, &target);
    assert_eq!(finish(&c, &mut child), EdgeTraversalProgress::Complete);
    assert_eq!(child.scanned_slots(), 2);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 0);
}

#[test]
fn indexed_traversal_failed_reads_preserve_the_source_offset_and_ticket() {
    let c = indexed_catalog();
    let (source, sr) = object(&c, 1);
    let (target, tr) = object(&c, 2);
    let _edge = edge(&c, 2, &sr, Some(&tr));
    remove(&c, &tr);
    restart(&c, &[&source, &target]);
    scan_roots(&c);
    let before = snapshot(&c).phase();
    for failure in 0..6 {
        let mut scan = begin(&c, &source);
        FAIL_LOAD.with(|n| n.set(Some(failure)));
        assert_eq!(scan.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
        FAIL_LOAD.with(|n| n.set(None));
        assert_eq!(
            scan.scanned_slots(),
            0,
            "indexed failed read advanced the range"
        );
        assert_eq!(snapshot(&c).phase(), before);
    }
    assert_eq!(
        finish(&c, &mut begin(&c, &source)),
        EdgeTraversalProgress::Complete
    );
}

#[test]
fn indexed_traversal_stopped_scanners_do_not_own_the_source_ticket() {
    for stopped in 0..8 {
        let c = indexed_catalog();
        let (source, sr) = object(&c, 1);
        let (target, tr) = object(&c, 2);
        let _edge = edge(&c, 3, &sr, Some(&tr));
        remove(&c, &tr);
        restart(&c, &[&source, &target]);
        scan_roots(&c);
        let mut old = begin(&c, &source);
        for _ in 0..stopped {
            assert_eq!(old.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
        }
        assert_eq!(
            finish(&c, &mut begin(&c, &source)),
            EdgeTraversalProgress::Complete
        );
        assert_eq!(finish(&c, &mut old), EdgeTraversalProgress::Superseded);
        assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
    }
}

#[test]
fn indexed_traversal_mutation_behind_cursor_keeps_target_work() {
    let c = indexed_catalog();
    let (a, ar) = object(&c, 1);
    let (b, br) = object(&c, 2);
    let edge = edge(&c, 2, &ar, None);
    restart(&c, &[&a, &b]);
    commit(
        &c,
        c.begin_shade(
            snapshot(&c),
            a.try_clone().unwrap(),
            0,
            TraceRetention::Strong,
        ),
    );
    let mut scan = begin(&c, &a);
    assert_eq!(scan.poll(&c, &()).unwrap(), EdgeTraversalProgress::Pending);
    assert_eq!(scan.scanned_slots(), 1);
    commit(
        &c,
        c.begin_replace_edge(
            snapshot(&c),
            edge.try_share().unwrap(),
            ar.try_share().unwrap(),
            Some(br.try_share().unwrap()),
        ),
    );
    remove(&c, &br);
    assert_eq!(finish(&c, &mut scan), EdgeTraversalProgress::Complete);
    assert_eq!(snapshot(&c).phase().domains[0].pending, 1);
    let Some(Membership::Tracing(marks)) = b.membership() else {
        panic!()
    };
    assert_eq!(marks[0].retention, Some(TraceRetention::Strong));
    assert_ne!(marks[0].ticket, 0);
}
