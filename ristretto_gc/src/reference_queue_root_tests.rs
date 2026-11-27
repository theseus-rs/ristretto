//! Progress at the reservation and commit of Java queue roots.
use super::*;

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn parked_reference_owner_root_preparation_preserves_peer_progress() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    struct Owner(GcWeak<u64>);
    // SAFETY: the slot is a conditional edge, with ownership bound during attachment.
    unsafe impl Trace for Owner {
        fn trace(&self, _: &GarbageCollector) {}
        unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
            self.0.set_owner(owner.address());
        }
    }
    let mut observations = Vec::new();
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let target = Gc::try_new(&gc, 42_u64)?;
        let owner = Gc::try_new(&gc, 7_u64)?;
        let weak = GcWeak::new(&target, ReferenceKind::Weak, owner.as_ptr().addr());
        let fresh_target = Gc::try_new(&gc, 123_u64)?;
        let fresh_slot = GcWeak::new(&fresh_target, ReferenceKind::Weak, 0);
        drop(target);
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
            entered,
            resume: release,
        });
        *lock(&gc.tree_root_insert_probe) = Some(GarbageCollector::pause_metadata_preparation);
        thread::scope(|scope| -> Result<()> {
            let heap = &gc;
            let first = scope.spawn(move || heap.collect_full());
            parked
                .recv_timeout(Duration::from_secs(4))
                .expect("preparing the queue root");
            let reader = gc.publication.try_read().is_some();
            let ticket = gc.request_full_cycle();
            let mut polls = 0;
            while polls < 2048 && gc.completed_full_ticket() < ticket {
                gc.step();
                polls += 1;
            }
            let completed = gc.completed_full_ticket() >= ticket;
            let (sent, received) = mpsc::channel();
            let allocation = scope.spawn(move || {
                let value = Gc::try_new(heap, 73_u64).expect("allocation with headroom");
                sent.send(()).unwrap();
                value
            });
            let (ready, report) = mpsc::channel();
            let conditional = scope.spawn(move || {
                let value = Gc::try_new(heap, Owner(fresh_slot)).expect("conditional owner");
                ready.send(()).unwrap();
                value
            });
            let allocated = received.recv_timeout(Duration::from_secs(1)).is_ok();
            let attached = report.recv_timeout(Duration::from_secs(1)).is_ok();
            resume.send(()).unwrap();
            first.join().unwrap()?;
            let value = allocation.join().unwrap();
            assert_eq!(**value, 73);
            let conditional = conditional.join().unwrap();
            assert_eq!(
                conditional.0.upgrade().unwrap().identity(),
                fresh_target.identity()
            );
            observations.push((counted, reader, allocated, attached, completed, polls));
            Ok(())
        })?;
        gc.collect_full()?;
        assert_eq!(
            gc.take_pending_reference::<u64>()?.unwrap().identity(),
            owner.identity()
        );
        assert!(gc.take_pending_reference::<u64>()?.is_none());
        assert!(weak.upgrade().is_none());
    }
    assert!(
        observations.iter().all(|o| o.2 && o.3 && o.4),
        "parked root preparation (counted, reader, scalar, conditional, ticket, polls): {observations:?}"
    );
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn parked_private_notice_cannot_retain_or_deliver_a_rebound_owner() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for kind in [ReferenceKind::Weak, ReferenceKind::Final] {
            for rebind in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .incremental_step_size(1)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let a = Gc::try_new(&gc, 7_u64)?;
                let b = Gc::try_new(&gc, 99_u64)?;
                let target = Gc::try_new(&gc, 42_u64)?;
                let link = GcWeak::new(&target, kind, a.as_ptr().addr());
                let witness_a = GcWeak::new(&a, ReferenceKind::Weak, 0);
                let witness_b = GcWeak::new(&b, ReferenceKind::Weak, 0);
                drop(target);
                let (entered, parked) = mpsc::channel();
                let (resume, release) = mpsc::channel();
                thread::scope(|scope| -> Result<()> {
                    let heap = &gc;
                    let first = scope.spawn(move || {
                        VACANT_NOTICE_PAUSE.set(Some(ClaimPause {
                            entered,
                            resume: release,
                        }));
                        heap.collect_full()
                    });
                    parked
                        .recv_timeout(Duration::from_secs(4))
                        .expect("reserved before commit");
                    assert!(!gc.has_pending_references());
                    if kind == ReferenceKind::Weak {
                        assert!(link.slot.pending.load(Ordering::Acquire));
                    } else {
                        assert!(!link.slot.activated.load(Ordering::Acquire));
                    }
                    link.set_owner(if rebind { b.as_ptr().addr() } else { 0 });
                    let ticket = gc.request_full_cycle();
                    for _ in 0..2048 {
                        gc.step();
                        if gc.completed_full_ticket() >= ticket {
                            break;
                        }
                    }
                    assert!(gc.completed_full_ticket() >= ticket);
                    let notice = gc.take_pending_reference::<u64>()?;
                    assert_eq!(
                        notice.as_ref().map(Gc::identity),
                        rebind.then(|| b.identity())
                    );
                    drop(notice);
                    assert!(gc.take_pending_reference::<u64>()?.is_none());
                    link.clear();
                    drop(a);
                    drop(b);
                    gc.collect_full()?;
                    assert!(witness_a.upgrade().is_none());
                    assert!(witness_b.upgrade().is_none());
                    resume.send(()).expect("resume stale preparer");
                    first.join().unwrap()?;
                    assert!(gc.take_pending_reference::<u64>()?.is_none());
                    assert_eq!(gc.roots.len(), 0);
                    assert_eq!(gc.pending_references.get().unwrap().pending(), 0);
                    Ok(())
                })?;
            }
        }
    }
    Ok(())
}

#[test]
fn independent_notice_preparation_and_link_finish_during_one_step() -> Result<()> {
    for counted in [false, true] {
        for (kind, stage) in [(ReferenceKind::Weak, 4), (ReferenceKind::Final, 2)] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .independent_phases(true)
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let owner = Gc::try_new(&gc, 21_u64)?;
            let target = Gc::try_new(&gc, 13_u64)?;
            let reference = GcWeak::new(&target, kind, owner.as_ptr().addr());
            drop(target);
            gc.collect();
            at_reference_stage(&gc, 1, stage);
            let held = lock(&gc.coordinator);
            gc.step();
            assert!(gc.has_pending_references());
            drop(held);
            assert_eq!(
                gc.take_pending_reference::<u64>()?.unwrap().identity(),
                owner.identity()
            );
            assert!(gc.take_pending_reference::<u64>()?.is_none());
            reference.clear();
            finish(&gc);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn parked_late_notice_with_a_busy_slot_yields_to_another_final_owner() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let dead_owner = Gc::try_new(&gc, 1_u64)?;
        let owner_a = Gc::try_new(&gc, 2_u64)?;
        let owner_b = Gc::try_new(&gc, 3_u64)?;
        let first = Gc::try_new(&gc, 19_u64)?;
        let second = Gc::try_new(&gc, 23_u64)?;
        gc.next_weak_id.store(usize::MAX - 1, Ordering::Relaxed);
        let source_a = GcWeak::new(&first, ReferenceKind::Final, dead_owner.as_ptr().addr());
        let source_b = GcWeak::new(&second, ReferenceKind::Final, dead_owner.as_ptr().addr());
        drop((dead_owner, first, second));
        gc.collect();
        at_reference_stage(&gc, 1, 2);
        assert!((0..64).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).cursor == usize::MAX
        }));
        let a = source_a.clone();
        a.set_owner(owner_a.as_ptr().addr());
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        thread::scope(|scope| -> Result<()> {
            let heap = &gc;
            let producer = scope.spawn(move || {
                VACANT_NOTICE_PAUSE.set(Some(ClaimPause {
                    entered,
                    resume: release,
                }));
                heap.step();
            });
            parked
                .recv_timeout(Duration::from_secs(4))
                .expect("preparing late slot");
            let held = a.slot.decision();
            let b = source_b.clone();
            b.set_owner(owner_b.as_ptr().addr());
            assert!((0..32).any(|_| {
                gc.step();
                gc.has_pending_references()
            }));
            assert_eq!(
                gc.take_pending_reference::<u64>()?.unwrap().identity(),
                owner_b.identity()
            );
            assert!(!a.slot.activated.load(Ordering::Acquire));
            drop(held);
            finish(&gc);
            assert_eq!(
                gc.take_pending_reference::<u64>()?.unwrap().identity(),
                owner_a.identity()
            );
            resume.send(()).unwrap();
            producer.join().unwrap();
            assert!(gc.take_pending_reference::<u64>()?.is_none());
            assert_eq!(gc.pending_references.get().unwrap().pending(), 0);
            a.clear();
            b.clear();
            Ok(())
        })?;
    }
    Ok(())
}
