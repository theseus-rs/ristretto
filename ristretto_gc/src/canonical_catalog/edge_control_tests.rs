use super::super::{Attempt, Cell, Membership, Outcome, Preparation, allocate};
use super::*;
fn cell(catalog: &Catalog, slot: usize) -> Pooled<Cell> {
    let mut creation = catalog.begin_create_with(slot, 8 * (slot + 1), ()).unwrap();
    for _ in 0..4 {
        if let Some(cell) = creation.poll(catalog).unwrap() {
            return cell;
        }
    }
    panic!("finite creation");
}
fn fixture() -> (Catalog, Pooled<Cell>, Pooled<Cell>, EdgeToken, Attempt) {
    let catalog = Catalog::with_record_capacities_and_limit(2, 32, 32, 32, 32)
        .with_edge_capacities(1, 16, 16, 32);
    let a = cell(&catalog, 0);
    let b = cell(&catalog, 1);
    let mut edge = catalog.begin_create_edge(0).unwrap();
    assert!(edge.poll(&catalog).unwrap().is_none());
    assert!(edge.poll(&catalog).unwrap().is_none());
    let edge = edge.poll(&catalog).unwrap().unwrap();
    let snapshot = catalog.snapshot_in(&()).unwrap();
    let phase = snapshot.phase();
    let effect = catalog
        .effect(
            share(&a).unwrap(),
            a.state.load().unwrap(),
            Membership::Retired,
        )
        .unwrap();
    let secondary = catalog
        .effect(
            share(&b).unwrap(),
            b.state.load().unwrap(),
            Membership::Retired,
        )
        .unwrap();
    let mut preparation: Preparation = catalog
        .begin_prepare(snapshot, phase, Some(effect))
        .unwrap();
    preparation.secondary_effect = Some(secondary);
    preparation.edge = Some(EdgeEffect {
        slot: edge.try_share().unwrap(),
        expected: edge.state.load().unwrap(),
        proposed: allocate(
            &catalog.edges.as_ref().unwrap().states,
            EdgeMembership::Retired,
        )
        .unwrap(),
    });
    assert!(preparation.poll(&catalog).unwrap().is_none());
    let attempt = preparation.poll(&catalog).unwrap().unwrap();
    (catalog, a, b, edge, attempt)
}
#[test]
fn secondary_and_edge_acquisition_fail_before_irreversible_commit() {
    for successful_shares in [1, 2] {
        let (catalog, a, b, edge, attempt) = fixture();
        assert!(catalog.install_and_stop(&attempt));
        crate::canonical_catalog::FAIL_SHARE.with(|fault| fault.set(Some(successful_shares)));
        catalog.help_record_at(&(), 0, || {
            panic!("effect ran before all proposals were retained")
        });
        assert_eq!(a.membership(), Some(Membership::Prepared));
        assert_eq!(b.membership(), Some(Membership::Prepared));
        assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
        assert!(attempt.cancel());
        assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Cancelled);
    }
}
#[test]
fn helper_completes_all_effects_after_owner_stops_at_first_membership() {
    let (catalog, a, b, edge, attempt) = fixture();
    catalog.effect_and_stop(&attempt);
    assert_eq!(a.membership(), Some(Membership::Retired));
    assert_eq!(b.membership(), Some(Membership::Prepared));
    assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
    assert!(!attempt.cancel());
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
    assert_eq!(b.membership(), Some(Membership::Retired));
    assert_eq!(edge.membership(), Some(EdgeMembership::Retired));
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Committed);
}
#[test]
fn cancelled_multi_effect_proposal_changes_no_membership() {
    let (catalog, a, b, edge, attempt) = fixture();
    assert!(attempt.cancel());
    assert_eq!(catalog.commit_in(&attempt, &()), Outcome::Cancelled);
    assert_eq!(a.membership(), Some(Membership::Prepared));
    assert_eq!(b.membership(), Some(Membership::Prepared));
    assert_eq!(edge.membership(), Some(EdgeMembership::Prepared));
}
