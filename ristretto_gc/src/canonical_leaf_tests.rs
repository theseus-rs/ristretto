//! Real private leaves under the isolated canonical controller; no public Gc path.
use super::*;
use crate::canonical_catalog as gc_catalog;
use crate::gc::publication_owner::canonical_leaf::LeafRecord;
use gc_catalog::{Catalog, Membership, Outcome, Preparation, Record, RecordError, Snapshot};

#[path = "canonical_leaf_lifecycle_tests.rs"]
mod lifecycle;

#[path = "canonical_leaf_access_tests.rs"]
mod access;

struct CertifiedLeaf(Arc<AtomicUsize>);
// SAFETY: this value contains only an ordinary counter and has no managed edges
// or tracing actions. Its destructor is still outside any bounded-step claim.
// SAFETY: this fixture has no managed edges and requires no tracing.
unsafe impl Trace for CertifiedLeaf {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {
        panic!("certified leaf was traced");
    }
}
impl Drop for CertifiedLeaf {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn leaf(gc: &GarbageCollector, drops: &Arc<AtomicUsize>) -> LeafRecord {
    prepare(gc, CertifiedLeaf(drops.clone()), false)
        .try_into_canonical_leaf()
        .unwrap_or_else(|_| panic!("private leaf transfer"))
}
fn catalog() -> Catalog<LeafRecord> {
    Catalog::with_record_capacities_and_limit(3, 64, 128, 32, 100_000)
}
fn snapshot(c: &Catalog<LeafRecord>, gc: &GarbageCollector) -> Snapshot<LeafRecord> {
    for _ in 0..16 {
        if let Some(snapshot) = c.snapshot_in(gc) {
            return snapshot;
        }
    }
    panic!("controller did not settle")
}
fn commit(
    c: &Catalog<LeafRecord>,
    gc: &GarbageCollector,
    preparation: Option<Preparation<LeafRecord>>,
) {
    let mut preparation = preparation.expect("valid leaf operation");
    let attempt = (0..32)
        .find_map(|_| preparation.poll(c).unwrap())
        .expect("prepared operation");
    for _ in 0..32 {
        match c.commit_in(&attempt, gc) {
            Outcome::Pending => {}
            Outcome::Committed => return,
            other => panic!("unexpected result: {other:?}"),
        }
    }
    panic!("leaf commit did not settle")
}

#[test]
fn private_leaf_transfer_preserves_charge_and_never_enters_legacy_indexes() -> crate::Result<()> {
    for counted in [false, true] {
        let gc = collector(counted, RootStorage::Contexts);
        let drops = Arc::new(AtomicUsize::new(0));
        let record = leaf(&gc, &drops);
        let charge = gc.managed_bytes();
        assert!(charge > 0);
        assert!(gc.metadata_for_address(record.address()).is_none());
        assert!(!record.published_for_test());
        reclaim(&gc)?;
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(gc.managed_bytes(), charge);
        drop(record);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn unsupported_or_rooted_transfer_returns_the_original_publication() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Slots);
    let leaf_drops = Arc::new(AtomicUsize::new(0));
    let rooted = prepare(&gc, CertifiedLeaf(leaf_drops.clone()), true);
    let address = rooted.root.as_ref().unwrap().ptr;
    let rooted = rooted
        .try_into_canonical_leaf()
        .err()
        .expect("parent root must be transferred separately");
    assert_eq!(rooted.root.as_ref().unwrap().ptr, address);
    assert!(rooted.registration.is_some());
    assert_eq!(leaf_drops.load(Ordering::Relaxed), 0);
    drop(rooted);
    assert_eq!(leaf_drops.load(Ordering::Relaxed), 1);
    let child = Arc::new(AtomicUsize::new(0));
    let parent = Arc::new(AtomicUsize::new(0));
    let publication = prepare(&gc, graph(&gc, &child, &parent), false);
    let identity = publication.metadata.as_ref().unwrap().id;
    let publication = publication
        .try_into_canonical_leaf()
        .err()
        .expect("graph must not enter leaf adapter");
    assert_eq!(publication.metadata.as_ref().unwrap().id, identity);
    reclaim(&gc)?;
    assert_eq!(child.load(Ordering::Relaxed), 0);
    drop(publication);
    reclaim(&gc)?;
    assert_eq!(parent.load(Ordering::Relaxed), 1);
    assert_eq!(child.load(Ordering::Relaxed), 1);
    Ok(())
}

#[test]
fn foreign_heap_cannot_admit_or_certify_a_real_leaf_record() {
    let gc = collector(true, RootStorage::Slots);
    let foreign = collector(true, RootStorage::Slots);
    let drops = Arc::new(AtomicUsize::new(0));
    let record = leaf(&gc, &drops);
    assert!(matches!(record.enter(&foreign), Err(RecordError::Foreign)));
    {
        let _guard = record.enter(&gc).expect("exact heap admission");
        assert!(!record.published_for_test());
    }
    drop(record);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn canonical_leaf_publication_keeps_real_storage_until_exact_retirement() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Slots);
    let drops = Arc::new(AtomicUsize::new(0));
    let record = leaf(&gc, &drops);
    let address = record.address();
    let identity = record.identity();
    let c = catalog();
    let mut creation = c
        .begin_create_with(0, address, record)
        .unwrap_or_else(|_| panic!("record preparation"));
    let cell = (0..16)
        .find_map(|_| creation.poll(&c).unwrap())
        .expect("record creation");
    drop(creation);
    assert_eq!(cell.record().identity(), identity);
    commit(
        &c,
        &gc,
        c.begin_publish(snapshot(&c, &gc), cell.try_clone().unwrap()),
    );
    assert!(cell.record().published_for_test());
    assert!(gc.metadata_for_address(address).is_none());
    reclaim(&gc)?;
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    let Membership::Tracing(marks) = cell.membership().unwrap() else {
        panic!("canonical publication")
    };
    for domain in 0..2 {
        commit(
            &c,
            &gc,
            c.begin_trace_acknowledge(
                snapshot(&c, &gc),
                cell.try_clone().unwrap(),
                domain,
                marks[domain].ticket,
            ),
        );
        for _ in 0..gc_catalog::RETIREMENT_FLOOR {
            commit(&c, &gc, c.begin_advance(snapshot(&c, &gc), domain));
        }
        commit(&c, &gc, c.begin_finish(snapshot(&c, &gc), domain));
    }
    commit(
        &c,
        &gc,
        c.begin_start(snapshot(&c, &gc), 0, gc_catalog::DomainMode::Full),
    );
    for _ in 0..gc_catalog::RETIREMENT_FLOOR {
        commit(&c, &gc, c.begin_advance(snapshot(&c, &gc), 0));
    }
    commit(
        &c,
        &gc,
        c.begin_trace_retire(snapshot(&c, &gc), cell.try_clone().unwrap(), 0),
    );
    assert!(c.release_retired(&cell));
    assert_eq!(
        drops.load(Ordering::Relaxed),
        0,
        "stopped exact owners retain storage"
    );
    drop(cell);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn a_surviving_leaf_record_does_not_keep_the_heap_alive() {
    let gc = collector(true, RootStorage::Slots);
    let weak = Arc::downgrade(&gc);
    let drops = Arc::new(AtomicUsize::new(0));
    let record = leaf(&gc, &drops);
    let identity = record.identity();
    drop(gc);
    assert!(weak.upgrade().is_none());
    assert_eq!(record.identity(), identity);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(record);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

fn finish_domain(c: &Catalog<LeafRecord>, gc: &GarbageCollector, domain: usize) {
    while snapshot(c, gc).phase().domains[domain].floor < gc_catalog::RETIREMENT_FLOOR {
        for slot in 0..3 {
            let Some(cell) = c.discover(slot).unwrap() else {
                continue;
            };
            let Membership::Tracing(marks) = cell.membership().unwrap() else {
                continue;
            };
            for (index, mark) in marks.marks.into_iter().enumerate() {
                if mark.ticket != 0 {
                    commit(
                        c,
                        gc,
                        c.begin_trace_acknowledge(
                            snapshot(c, gc),
                            cell.try_clone().unwrap(),
                            index,
                            mark.ticket,
                        ),
                    );
                }
            }
        }
        commit(c, gc, c.begin_advance(snapshot(c, gc), domain));
    }
}

#[test]
fn a_nested_helper_certifies_and_reclaims_before_the_original_leaf_guard_returns() {
    let gc = collector(true, RootStorage::Contexts);
    let c = Arc::new(catalog());
    let drops = Arc::new(AtomicUsize::new(0));
    let other_drops = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicBool::new(false));
    let record = leaf(&gc, &drops);
    let old_identity = record.identity();
    let helped = c.clone();
    let heap = gc.clone();
    let done = completed.clone();
    let retained = drops.clone();
    let other = other_drops.clone();
    record.pause_after_admission(move || {
        // The original operation is installed, and its real record is borrowed.
        // This nested helper can finish it without returning through that guard.
        assert!(helped.snapshot_in(&heap).is_none());
        let current = helped.discover(0).unwrap().unwrap();
        assert!(current.record().published_for_test());
        let other_record = leaf(&heap, &other);
        let mut creation = helped
            .begin_create_with(1, other_record.address(), other_record)
            .unwrap_or_else(|_| panic!("unrelated leaf"));
        let other_cell = (0..16)
            .find_map(|_| creation.poll(&helped).unwrap())
            .unwrap();
        drop(creation);
        commit(
            &helped,
            &heap,
            helped.begin_publish(snapshot(&helped, &heap), other_cell.try_clone().unwrap()),
        );
        for domain in 0..2 {
            finish_domain(&helped, &heap, domain);
            commit(
                &helped,
                &heap,
                helped.begin_finish(snapshot(&helped, &heap), domain),
            );
        }
        commit(
            &helped,
            &heap,
            helped.begin_start(snapshot(&helped, &heap), 0, gc_catalog::DomainMode::Full),
        );
        finish_domain(&helped, &heap, 0);
        for cell in [&current, &other_cell] {
            commit(
                &helped,
                &heap,
                helped.begin_trace_retire(snapshot(&helped, &heap), cell.try_clone().unwrap(), 0),
            );
            assert!(helped.release_retired(cell));
        }
        drop(other_cell);
        assert_eq!(other.load(Ordering::Relaxed), 1);
        assert_eq!(retained.load(Ordering::Relaxed), 0);
        let replacement = leaf(&heap, &other);
        let mut creation = helped
            .begin_create_with(0, replacement.address(), replacement)
            .unwrap_or_else(|_| panic!("reused discovery slot"));
        let replacement = (0..16)
            .find_map(|_| creation.poll(&helped).unwrap())
            .unwrap();
        assert_ne!(replacement.identity, current.identity);
        assert_ne!(replacement.record().identity(), old_identity);
        assert_eq!(replacement.membership(), Some(Membership::Prepared));
        done.store(true, Ordering::Release);
    });
    let mut creation = c
        .begin_create_with(0, record.address(), record)
        .unwrap_or_else(|_| panic!("leaf record"));
    let cell = (0..16).find_map(|_| creation.poll(&c).unwrap()).unwrap();
    drop(creation);
    commit(
        &c,
        &gc,
        c.begin_publish(snapshot(&c, &gc), cell.try_clone().unwrap()),
    );
    assert!(completed.load(Ordering::Acquire));
    assert_eq!(cell.membership(), Some(Membership::Retired));
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(cell);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    let replacement = c.discover(0).expect("discovery").expect("replacement");
    assert_eq!(replacement.membership(), Some(Membership::Prepared));
    assert!(!replacement.record().published_for_test());
    commit(
        &c,
        &gc,
        c.begin_abandon(snapshot(&c, &gc), replacement.try_clone().unwrap()),
    );
    assert!(c.release_retired(&replacement));
    drop(replacement);
    assert_eq!(other_drops.load(Ordering::Relaxed), 2);
    assert_eq!(gc.managed_bytes(), 0);
}
