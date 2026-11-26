use super::*;
use std::collections::BTreeMap;

#[test]
fn persistent_tree_matches_ordered_membership_and_pending_counts() {
    let mut tree = Tree::default();
    let mut oracle = BTreeMap::new();
    let mut seed = 17_u64;
    let mut saved = Vec::new();
    for index in 0..5000 {
        seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
        let key = seed % 150 + 1;
        if seed & 3 == 0 { tree = tree.remove(key); oracle.remove(&key); }
        else { let record = Record { identity: key, address: key as usize * 8, work: [seed & 1, (seed >> 1) & 1] }; tree = tree.put(record); oracle.insert(key, record); }
        assert_eq!(tree.count(), oracle.len());
        for domain in 0..2 { assert_eq!(tree.pending_count(domain), oracle.values().filter(|v| v.work[domain] != 0).count()); }
        let mut actual = Vec::new();let mut after=0;
        while let Some(record)=tree.next(after) { after=record.identity;actual.push(record); }
        assert_eq!(actual,oracle.values().copied().collect::<Vec<_>>());
        if index%1000==0 { saved.push((tree.clone(),actual)); }
    }
    for (snapshot,expected) in saved {
        let mut actual=Vec::new();let mut after=0;
        while let Some(record)=snapshot.next(after) {after=record.identity;actual.push(record);}
        assert_eq!(actual,expected);
    }
}

#[test]
fn multiple_memberships_and_domain_boundaries_share_one_commit() {
    let catalog = Catalog::new();
    let first = Catalog::insert(catalog.snapshot(), 1, 8).unwrap();
    assert!(catalog.commit_stopped_after_cas(&first));
    let second = Catalog::insert(catalog.snapshot(), 2, 16).unwrap();
    assert_eq!(catalog.commit(&second),Outcome::Committed);
    for domain in 0..2 {
        assert!(Catalog::advance(catalog.snapshot(),domain).is_none());
        for identity in 1..=2 {
            let snapshot=catalog.snapshot();let ticket=snapshot.entries.get(identity).unwrap().work[domain];
            assert_eq!(catalog.commit(&Catalog::acknowledge(snapshot,identity,domain,ticket).unwrap()),Outcome::Committed);
        }
        let close=Catalog::advance(catalog.snapshot(),domain).unwrap();
        assert_eq!(catalog.commit(&close),Outcome::Committed);
    }
    assert_eq!(catalog.commit(&Catalog::retire(catalog.snapshot(),1,8).unwrap()),Outcome::Committed);
    assert_eq!(catalog.commit(&Catalog::insert(catalog.snapshot(),3,8).unwrap()),Outcome::Committed);
    assert_eq!(catalog.commit(&first),Outcome::Committed);
    assert!(catalog.snapshot().entries.get(1).is_none());
    assert_eq!(catalog.snapshot().entries.get(3).unwrap().address,8);
}

#[test]
fn late_work_acknowledgement_cannot_discard_a_new_obligation() {
    let catalog=Catalog::new();
    assert_eq!(catalog.commit(&Catalog::insert(catalog.snapshot(),1,8).unwrap()),Outcome::Committed);
    let snapshot=catalog.snapshot();let ticket=snapshot.entries.get(1).unwrap().work[0];
    let old_ack=Catalog::acknowledge(snapshot,1,0,ticket).unwrap();
    let new_work=Catalog::require(catalog.snapshot(),1,0).unwrap();
    assert_eq!(catalog.commit(&new_work),Outcome::Committed);
    assert_eq!(catalog.commit(&old_ack),Outcome::Rejected);
    assert!(Catalog::acknowledge(catalog.snapshot(),1,0,ticket).is_none());
    assert!(Catalog::advance(catalog.snapshot(),0).is_none());
}
