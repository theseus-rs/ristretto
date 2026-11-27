//! Actual private payload binding; no canonical collector publication yet.
use super::*;
use crate::gc::publication_owner::shared::{EnterError, ReserveError};

#[test]
fn discarded_unbound_records_do_not_consume_payload_binding() -> crate::Result<()> {
    for counted in [false, true] {
        for roots in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = collector(counted, roots);
            let child = Arc::new(AtomicUsize::new(0));
            let parent = Arc::new(AtomicUsize::new(0));
            let (header, origin) = prepare(&gc, graph(&gc, &child, &parent), true).share();
            let erased = header.erase();
            let helper = erased
                .acquire(&gc)
                .unwrap_or_else(|_| panic!("private helper"));
            // This models private creation losing before discovery/record entry.
            let lost = helper
                .prepare_record()
                .unwrap_or_else(|_| panic!("first record"));
            let identity = lost.identity();
            drop(lost);
            let winner = helper
                .prepare_record()
                .unwrap_or_else(|_| panic!("replacement record"));
            let loser = helper
                .prepare_record()
                .unwrap_or_else(|_| panic!("different record"));
            let first = winner.enter(&gc).unwrap_or_else(|_| panic!("winner binds"));
            let second = winner
                .enter(&gc)
                .unwrap_or_else(|_| panic!("same record helps"));
            assert!(matches!(loser.enter(&gc), Err(EnterError::BoundElsewhere)));
            drop(origin);
            drop(helper);
            reclaim(&gc)?;
            assert_eq!(first.view().metadata().id, identity);
            assert_eq!(second.view().metadata().id, identity);
            assert_eq!(parent.load(Ordering::Relaxed), 0);
            assert_eq!(child.load(Ordering::Relaxed), 0);
            drop(first);
            assert_eq!(parent.load(Ordering::Relaxed), 0);
            drop(second);
            assert_eq!(parent.load(Ordering::Relaxed), 1);
            reclaim(&gc)?;
            assert_eq!(child.load(Ordering::Relaxed), 1);
            assert_eq!(gc.managed_bytes(), 0);
            assert!(matches!(
                winner.enter(&gc),
                Err(EnterError::Admission(AcquireError::Closed))
            ));
            assert!(matches!(
                loser.enter(&gc),
                Err(EnterError::Admission(AcquireError::Closed))
            ));
            let weak = Arc::downgrade(&gc);
            drop(gc);
            assert!(weak.upgrade().is_none());
            assert_eq!(winner.identity(), identity);
            assert_eq!(loser.identity(), identity);
        }
    }
    Ok(())
}

#[test]
fn foreign_heap_cannot_claim_record_binding() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Contexts);
    let foreign = collector(true, RootStorage::Contexts);
    let drops = Arc::new(AtomicUsize::new(0));
    let (header, origin) = prepare(&gc, Leaf(drops.clone()), true).share();
    let erased = header.erase();
    let helper = erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("private helper"));
    let first = helper
        .prepare_record()
        .unwrap_or_else(|_| panic!("first record"));
    let other = helper
        .prepare_record()
        .unwrap_or_else(|_| panic!("other record"));
    assert!(matches!(
        first.enter(&foreign),
        Err(EnterError::Admission(AcquireError::Foreign))
    ));
    let guard = other
        .enter(&gc)
        .unwrap_or_else(|_| panic!("foreign entry did not bind"));
    assert_eq!(guard.view().metadata().id, other.identity());
    assert!(matches!(first.enter(&gc), Err(EnterError::BoundElsewhere)));
    drop(guard);
    drop(helper);
    drop(origin);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    reclaim(&gc)?;
    Ok(())
}

#[test]
fn binding_survives_guard_and_record_drop_without_authorizing_replacement() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Slots);
    let drops = Arc::new(AtomicUsize::new(0));
    let (header, origin) = prepare(&gc, Leaf(drops.clone()), true).share();
    let erased = header.erase();
    let helper = erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("private helper"));
    let winner = helper.prepare_record().unwrap_or_else(|_| panic!("record"));
    drop(winner.enter(&gc).unwrap_or_else(|_| panic!("bind")));
    drop(winner);
    let replacement = helper
        .prepare_record()
        .unwrap_or_else(|_| panic!("new nonce"));
    assert!(matches!(
        replacement.enter(&gc),
        Err(EnterError::BoundElsewhere)
    ));
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(helper);
    drop(origin);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    reclaim(&gc)?;
    Ok(())
}

#[test]
fn exhausted_record_identity_does_not_announce_or_prevent_restoration() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Tree);
    let drops = Arc::new(AtomicUsize::new(0));
    let (header, origin) = prepare(&gc, Leaf(drops.clone()), true).share();
    let erased = header.erase();
    let helper = erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("private helper"));
    helper.exhaust_record_nonces_for_test();
    assert!(matches!(
        helper.prepare_record(),
        Err(ReserveError::Exhausted)
    ));
    assert!(matches!(
        helper.prepare_record(),
        Err(ReserveError::Exhausted)
    ));
    drop(helper);
    let publication = origin
        .restore()
        .unwrap_or_else(|_| panic!("failed preparation announced ownership"));
    drop(publication);
    reclaim(&gc)?;
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    Ok(())
}

#[test]
fn record_preparation_and_admission_do_not_lock_the_result_slot() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Contexts);
    let drops = Arc::new(AtomicUsize::new(0));
    let (header, origin) = prepare(&gc, Leaf(drops.clone()), true).share();
    let erased = header.erase();
    let helper = erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("private helper"));
    origin.with_result_slot_locked(|| {
        let record = helper
            .prepare_record()
            .unwrap_or_else(|_| panic!("prepare without result lock"));
        let guard = record
            .enter(&gc)
            .unwrap_or_else(|_| panic!("enter without result lock"));
        assert_eq!(guard.view().metadata().id, record.identity());
    });
    drop(helper);
    drop(origin);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    reclaim(&gc)?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn stopped_record_guard_protects_its_graph_while_unrelated_gc_progresses() -> crate::Result<()> {
    let gc = collector(true, RootStorage::Contexts);
    let child = Arc::new(AtomicUsize::new(0));
    let parent = Arc::new(AtomicUsize::new(0));
    let (header, origin) = prepare(&gc, graph(&gc, &child, &parent), true).share();
    let erased = header.erase();
    let helper = erased
        .acquire(&gc)
        .unwrap_or_else(|_| panic!("private helper"));
    let record = helper.prepare_record().unwrap_or_else(|_| panic!("record"));
    drop(helper);
    let (entered, observed) = std::sync::mpsc::channel();
    let (release, resumed) = std::sync::mpsc::channel();
    let retained = std::thread::scope(|threads| -> crate::Result<_> {
        let gc_ref = &gc;
        let worker = threads.spawn(move || {
            let guard = record
                .enter(gc_ref)
                .unwrap_or_else(|_| panic!("guard before origin closure"));
            entered.send(()).unwrap_or_else(|_| panic!("entered"));
            resumed.recv().unwrap_or_else(|_| panic!("resumed"));
            assert_eq!(guard.view().metadata().id, record.identity());
            drop(guard);
            record
        });
        observed.recv().unwrap_or_else(|_| panic!("guard retained"));
        drop(origin);
        let unrelated = Arc::new(AtomicUsize::new(0));
        drop(Gc::new(&gc, Leaf(unrelated.clone())));
        let result = reclaim(&gc);
        // Release before checking assertions so a failure cannot strand the helper.
        let parent_before = parent.load(Ordering::Relaxed);
        let child_before = child.load(Ordering::Relaxed);
        let unrelated_after = unrelated.load(Ordering::Relaxed);
        release.send(()).unwrap_or_else(|_| panic!("resume helper"));
        let retained = worker.join().unwrap_or_else(|_| panic!("helper joined"));
        result?;
        assert_eq!(parent_before, 0);
        assert_eq!(child_before, 0);
        assert_eq!(unrelated_after, 1);
        Ok(retained)
    })?;
    assert_eq!(parent.load(Ordering::Relaxed), 1);
    reclaim(&gc)?;
    assert_eq!(child.load(Ordering::Relaxed), 1);
    assert!(matches!(
        retained.enter(&gc),
        Err(EnterError::Admission(AcquireError::Closed))
    ));
    Ok(())
}
