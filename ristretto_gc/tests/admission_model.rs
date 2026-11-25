//! Production counted publication and mark-state protocols under Loom.
//! These reduced models alone do NOT establish complete collector correctness:
//! only selected phase boundaries, reference decisions and two-generation
//! handoffs are represented. In particular, a storage lease is not graph reachability.
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use loom::sync::{Arc, Mutex};
use loom::thread;

#[path = "../src/publication_state.rs"]
mod publication_state;
use publication_state::{Birth, Clock, REVISION};

#[path = "../src/mark_state.rs"]
mod mark_state;
use mark_state::{EPOCH_STRIDE, MarkWord, Retention};

#[test]
fn stronger_discovery_retraces_descendants_without_downgrading() {
    loom::model(|| {
        let parent = Arc::new(MarkWord::default());
        let child = Arc::new(MarkWord::default());
        let weaker = {
            let (parent, child) = (parent.clone(), child.clone());
            thread::spawn(move || {
                if parent.mark(EPOCH_STRIDE, Retention::Final) {
                    // This earlier traversal may finish after the stronger one.
                    child.mark(EPOCH_STRIDE, Retention::Final);
                }
            })
        };
        assert!(parent.mark(EPOCH_STRIDE, Retention::Strong));
        child.mark(EPOCH_STRIDE, Retention::Strong);
        weaker.join().unwrap();
        assert_eq!(parent.retention(EPOCH_STRIDE), Some(Retention::Strong));
        assert!(child.marked_before(EPOCH_STRIDE, false));
    });
}

#[test]
fn a_closed_boundary_cannot_upgrade_an_earlier_retained_mark() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(8 * REVISION));
        let mark = Arc::new(MarkWord::default());
        assert!(mark.mark(EPOCH_STRIDE, Retention::Final));
        let queued = Arc::new(AtomicBool::new(false));
        let root = {
            let (clock, mark, queued) = (clock.clone(), mark.clone(), queued.clone());
            thread::spawn(move || {
                let _admitted = clock.enter();
                let floor = Retention::at_phase(usize::try_from(clock.flags(0)).unwrap());
                if mark.mark(EPOCH_STRIDE, Retention::Strong.max(floor)) {
                    queued.store(true, Ordering::Release);
                }
            })
        };
        let closed = clock.observe().is_some_and(|observed| {
            !queued.load(Ordering::Acquire) && clock.transition(observed, 0, CLOSED)
        });
        root.join().unwrap();
        if closed {
            assert_eq!(mark.retention(EPOCH_STRIDE), Some(Retention::Final));
            assert!(!mark.marked_before(EPOCH_STRIDE, false));
        }
    });
}

#[test]
fn new_epochs_and_black_publication_do_not_inherit_retention() {
    loom::model(|| {
        let mark = MarkWord::default();
        assert!(!mark.marked(EPOCH_STRIDE));
        assert!(mark.mark(EPOCH_STRIDE, Retention::Phantom));
        assert!(!mark.marked_before(EPOCH_STRIDE, true));
        assert!(mark.mark(2 * EPOCH_STRIDE, Retention::Final));
        assert!(mark.retention(EPOCH_STRIDE).is_none());
        assert!(mark.marked_before(2 * EPOCH_STRIDE, true));
        mark.reset();
        assert_eq!(mark.word(), 0);
        mark.mark_black(EPOCH_STRIDE);
        assert!(mark.marked_before(EPOCH_STRIDE, false));
        mark.mark_born(EPOCH_STRIDE);
        assert!(mark.marked(EPOCH_STRIDE));
        assert!(mark.retention(EPOCH_STRIDE).is_none());
    });
}

const MARK: u64 = 0;
const CLOSED: u64 = 1;

#[expect(
    clippy::unwrap_used,
    reason = "a model thread panic must fail the model"
)]
fn late_publication(initial: u64, limit: u64, deferred_stamp: bool) {
    loom::model(move || {
        let clock = Arc::new(Clock::new(limit * REVISION));
        for _ in 0..initial {
            drop(clock.enter());
        }
        let queued = Arc::new(AtomicBool::new(false));
        let publisher = {
            let (clock, queued) = (clock.clone(), queued.clone());
            thread::spawn(move || {
                let admitted = if deferred_stamp {
                    clock.enter_barrier()
                } else {
                    clock.enter()
                };
                let phase = clock.flags(0);
                // No phase can change after stamping until this admission ends.
                assert_eq!(clock.flags(0), phase);
                if phase == MARK {
                    queued.store(true, Ordering::Release);
                    if deferred_stamp {
                        admitted.changed();
                    }
                }
            })
        };
        let mut closed = false;
        // The second attempt covers a saturated counter's phase-preserving rebase.
        for _ in 0..2 {
            if let Some(observed) = clock.observe()
                && !queued.load(Ordering::Acquire)
                && clock.transition(observed, 0, CLOSED)
            {
                closed = true;
                break;
            }
        }
        publisher.join().unwrap();
        assert!(!closed || !queued.load(Ordering::Acquire));
        assert_eq!(clock.active(), 0);
    });
}

#[test]
fn an_atomic_entrant_observes_the_new_phase_or_excludes_closure() {
    late_publication(0, 3, false);
}

#[test]
fn saturation_never_wraps_a_revision_into_a_live_closure_token() {
    late_publication(1, 1, false);
}

#[test]
fn a_completed_publisher_invalidates_an_earlier_empty_observation() {
    loom::model(|| {
        let clock = Clock::new(3 * REVISION);
        let observed = clock.observe().unwrap();
        {
            let _admitted = clock.enter();
        }
        assert_eq!(clock.active(), 0);
        assert!(!clock.transition(observed, 0, CLOSED));
    });
}

#[test]
fn a_stalled_admission_never_prevents_another_publisher_from_entering() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(3 * REVISION));
        let stalled = clock.enter();
        let publisher = {
            let clock = clock.clone();
            thread::spawn(move || {
                let _admitted = clock.enter();
                let phase = clock.flags(0);
                assert_eq!(phase, MARK);
            })
        };
        publisher.join().unwrap();
        assert!(clock.observe().is_none());
        drop(stalled);
        assert!(clock.transition(clock.observe().unwrap(), 0, CLOSED));
    });
}

#[test]
fn white_weak_upgrade_and_logical_retirement_have_one_winner() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(3 * REVISION));
        // Registered and strongly marked, respectively. The object shard orders
        // validation/root installation with logical removal; no storage is borrowed.
        let object = Arc::new(Mutex::new((true, false)));
        let queued = Arc::new(AtomicBool::new(false));
        let upgraded = Arc::new(AtomicBool::new(false));
        let publisher = {
            let (clock, object, queued, upgraded) = (
                clock.clone(),
                object.clone(),
                queued.clone(),
                upgraded.clone(),
            );
            thread::spawn(move || {
                let _admitted = clock.enter();
                let phase = clock.flags(0);
                let mut object = object.lock().unwrap();
                if object.0 && (phase == MARK || object.1) {
                    object.1 = true;
                    queued.store(true, Ordering::Release);
                    upgraded.store(true, Ordering::Release);
                }
            })
        };
        if let Some(observed) = clock.observe()
            && !queued.load(Ordering::Acquire)
            && clock.transition(observed, 0, CLOSED)
        {
            let mut object = object.lock().unwrap();
            if !object.1 {
                object.0 = false;
            }
        }
        publisher.join().unwrap();
        assert!(!upgraded.load(Ordering::Acquire) || object.lock().unwrap().0);
    });
}

#[test]
fn one_domain_transition_preserves_the_other_domain() {
    loom::model(|| {
        let clock = Clock::new(publication_state::MAX_REVISION);
        assert!(clock.transition(clock.observe().unwrap(), 1, 13));
        assert!(clock.transition(clock.observe().unwrap(), 0, 7));
        let admitted = clock.enter();
        assert_eq!(clock.flags(0), 7);
        assert_eq!(clock.flags(1), 13);
        assert!(clock.observe().is_none());
        drop(admitted);
        assert!(clock.transition(clock.observe().unwrap(), 1, 0));
        assert_eq!(clock.flags(0), 7);
    });
}

#[test]
fn an_ungated_drain_keeps_its_reservation_until_work_is_visible() {
    loom::model(|| {
        let clock = Clock::new(publication_state::MAX_REVISION);
        let pending = Arc::new(AtomicUsize::new(1));
        let queued = Arc::new(AtomicBool::new(false));
        let flusher = {
            let (pending, queued) = (pending.clone(), queued.clone());
            thread::spawn(move || {
                queued.store(true, Ordering::Release);
                pending.fetch_sub(1, Ordering::AcqRel);
            })
        };
        let closed = clock.observe().is_some_and(|observed| {
            pending.load(Ordering::Acquire) == 0
                && !queued.load(Ordering::Acquire)
                && clock.transition(observed, 0, CLOSED)
        });
        flusher.join().unwrap();
        assert!(!closed);
    });
}

#[test]
fn a_barrier_publishing_to_both_domains_excludes_either_closure() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
        let queued = Arc::new([AtomicBool::new(false), AtomicBool::new(false)]);
        let publisher = {
            let (clock, queued) = (clock.clone(), queued.clone());
            thread::spawn(move || {
                let _admitted = clock.enter();
                for (index, queue) in queued.iter().enumerate() {
                    if clock.flags(index) == MARK {
                        queue.store(true, Ordering::Release);
                    }
                }
            })
        };
        let closed: [bool; 2] = std::array::from_fn(|index| {
            clock.observe().is_some_and(|observed| {
                !queued.get(index).unwrap().load(Ordering::Acquire)
                    && clock.transition(observed, index, CLOSED)
            })
        });
        publisher.join().unwrap();
        for (closed, queue) in closed.into_iter().zip(queued.iter()) {
            assert!(!closed || !queue.load(Ordering::Acquire));
        }
    });
}

#[test]
fn final_retention_preserves_logical_weak_clearing_and_excludes_white_retirement() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
        assert!(clock.transition(clock.observe().unwrap(), 0, CLOSED));
        // Registered, retention color (0 white, 1 strong, 2 final), rooted.
        let object = Arc::new(Mutex::new((true, 0, false)));
        let queued = Arc::new(AtomicBool::new(false));
        let retaining = {
            let (clock, object, queued) = (clock.clone(), object.clone(), queued.clone());
            thread::spawn(move || {
                let _admitted = clock.enter();
                let mut object = object.lock().unwrap();
                if object.0 && (clock.flags(0) == CLOSED || object.1 != 0) {
                    object.1 = 2;
                    object.2 = true;
                    queued.store(true, Ordering::Release);
                }
            })
        };
        // Physical slot processing observes frozen strong eligibility whether it
        // wins or loses the race with the other slot's retaining upgrade.
        assert_ne!(object.lock().unwrap().1, 1);
        if let Some(observed) = clock.observe()
            && !queued.load(Ordering::Acquire)
            && clock.transition(observed, 0, 5)
        {
            let mut object = object.lock().unwrap();
            if object.1 == 0 {
                object.0 = false;
            }
        }
        retaining.join().unwrap();
        let object = object.lock().unwrap();
        assert_ne!(object.1, 1);
        assert!(!object.2 || object.0);
    });
}

#[test]
fn pending_owner_defers_queue_delivery_until_publication_or_failure() {
    for succeeds in [false, true] {
        loom::model(move || {
            let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
            let pending = Arc::new(AtomicBool::new(false));
            // Owner assigned, registered/bound, activated, notifications.
            let slot = Arc::new(Mutex::new((false, false, false, 0)));
            let publisher = {
                let (clock, slot, pending) = (clock.clone(), slot.clone(), pending.clone());
                thread::spawn(move || {
                    {
                        let _admitted = clock.enter();
                        let mut slot = slot.lock().unwrap();
                        slot.0 = true;
                        pending.store(true, Ordering::Release);
                    }
                    // Binding and pending-owner release can wait without
                    // admission. The construction root remains until afterward.
                    slot.lock().unwrap().1 = succeeds;
                    let slot = slot.lock().unwrap();
                    assert!(!slot.2, "queue delivery preceded construction completion");
                    pending.store(false, Ordering::Release);
                })
            };
            let decide = || {
                if pending.load(Ordering::Acquire) {
                    return;
                }
                if let Ok(mut slot) = slot.try_lock()
                    && !pending.load(Ordering::Acquire)
                    && slot.0
                    && slot.1
                    && !slot.2
                {
                    slot.2 = true;
                    slot.3 += 1;
                }
            };
            decide();
            publisher.join().unwrap();
            decide();
            decide();
            assert_eq!(slot.lock().unwrap().3, usize::from(succeeds));
            assert_eq!(clock.active(), 0);
        });
    }
}

#[test]
fn two_generation_final_retention_keeps_storage_without_reversing_weak_clearing() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
        // Production flags: active | marking<<1 | reference_stage<<2.
        assert!(clock.transition(clock.observe().unwrap(), 0, 11)); // Young final tracing.
        assert!(clock.transition(clock.observe().unwrap(), 1, 3)); // Full strong tracing.
        let parent = Arc::new(MarkWord::default());
        let child = Arc::new(MarkWord::default());
        assert!(parent.mark(EPOCH_STRIDE, Retention::Final));
        let flight = Arc::new(AtomicUsize::new(1));
        let queued = Arc::new(AtomicBool::new(false));
        let trace = {
            let (clock, parent, child, flight, queued) = (
                clock.clone(),
                parent.clone(),
                child.clone(),
                flight.clone(),
                queued.clone(),
            );
            thread::spawn(move || {
                let _admission = clock.enter();
                let inherited = parent.retention(EPOCH_STRIDE).unwrap();
                let floor = Retention::at_phase(usize::try_from(clock.flags(1) >> 2).unwrap());
                if child.mark(EPOCH_STRIDE, inherited.max(floor)) {
                    queued.store(true, Ordering::Release);
                }
                // Continuation/publication precedes releasing the reservation.
                flight.fetch_sub(1, Ordering::AcqRel);
            })
        };
        // The sole coordinator cannot finish the young domain or close full
        // tracing across a callback/publication gap.
        let ended_young = clock.observe().is_some_and(|observed| {
            flight.load(Ordering::Acquire) == 0 && clock.transition(observed, 0, 0)
        });
        let closed_full = clock.observe().is_some_and(|observed| {
            clock.flags(0) == 0
                && !queued.load(Ordering::Acquire)
                && clock.transition(observed, 1, 5)
        });
        trace.join().unwrap();
        assert!(!closed_full);
        if !ended_young {
            assert!(clock.transition(clock.observe().unwrap(), 0, 0));
        }
        assert!(queued.swap(false, Ordering::AcqRel));
        assert!(clock.transition(clock.observe().unwrap(), 1, 5));
        assert!(!child.marked_before(EPOCH_STRIDE, false));
        assert!(child.marked(EPOCH_STRIDE));
        assert!(clock.transition(clock.observe().unwrap(), 1, 21));
        assert!(child.marked(EPOCH_STRIDE));
    });
}

#[test]
fn promotion_publishes_old_liveness_before_old_age_becomes_visible() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
        assert!(clock.transition(clock.observe().unwrap(), 1, 21));
        let mark = Arc::new(MarkWord::default());
        mark.mark_born(EPOCH_STRIDE);
        let old_age = Arc::new(AtomicBool::new(false));
        let promotion = {
            let (clock, mark, old_age) = (clock.clone(), mark.clone(), old_age.clone());
            thread::spawn(move || {
                let _admission = clock.enter();
                mark.mark_black(EPOCH_STRIDE);
                old_age.store(true, Ordering::Release);
            })
        };
        // Weak eligibility samples age and the mark independently. Seeing the
        // old age must imply strong eligibility, not merely physical Born liveness.
        if old_age.load(Ordering::Acquire) {
            assert!(mark.marked_before(EPOCH_STRIDE, false));
        }
        promotion.join().unwrap();
        assert!(old_age.load(Ordering::Acquire));
        assert!(mark.marked_before(EPOCH_STRIDE, false));
    });
}

#[test]
fn an_old_callback_borrow_excludes_its_conflicting_young_scan() {
    loom::model(|| {
        // The same metadata scan mutex protects old tracing/copying callbacks
        // and the young remembered-owner traversal. It is not a global flight gate.
        let scan = Arc::new(Mutex::new(()));
        let borrowed = Arc::new(AtomicBool::new(false));
        let entered = Arc::new(AtomicBool::new(false));
        let callback = {
            let (scan, borrowed, entered) = (scan.clone(), borrowed.clone(), entered.clone());
            thread::spawn(move || {
                let _scan = scan.lock().unwrap();
                borrowed.store(true, Ordering::Release);
                entered.store(true, Ordering::Release);
                thread::yield_now();
                borrowed.store(false, Ordering::Release);
            })
        };
        if entered.load(Ordering::Acquire)
            && let Ok(_scan) = scan.try_lock()
        {
            // Only a successful remembered scan can complete/remove its task.
            assert!(!borrowed.load(Ordering::Acquire));
        }
        callback.join().unwrap();
        assert!(scan.try_lock().is_ok());
    });
}

#[test]
fn a_context_attached_after_root_enumeration_publishes_across_phase_closure() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
        assert!(clock.transition(clock.observe().unwrap(), 0, 3));
        let root = Arc::new(MarkWord::default());
        let queued = Arc::new(AtomicBool::new(false));
        let publisher = {
            let (clock, root, queued) = (clock.clone(), root.clone(), queued.clone());
            thread::spawn(move || {
                // Attaching an empty context requires no acknowledgement.
                // A later root publication must shade, even when the root
                // enumeration cursor already passed the new registration.
                let _admission = clock.enter();
                if clock.flags(0) == 3 {
                    root.mark(EPOCH_STRIDE, Retention::Strong);
                    queued.store(true, Ordering::Release);
                } else {
                    root.mark_black(EPOCH_STRIDE);
                }
            })
        };
        let closed = clock.observe().is_some_and(|observed| {
            !queued.load(Ordering::Acquire) && clock.transition(observed, 0, 21)
        });
        publisher.join().unwrap();
        assert!(root.marked_before(EPOCH_STRIDE, false));
        if closed {
            // Either the publication was black after closure, or queued work
            // invalidated the observation and closure had to retry.
            assert!(!queued.load(Ordering::Acquire));
        } else {
            assert!(queued.load(Ordering::Acquire));
        }
    });
}

#[test]
fn detached_context_records_remain_owned_and_charged_through_the_drain_gap() {
    loom::model(|| {
        struct Context {
            dropped: Arc<AtomicBool>,
        }
        impl Drop for Context {
            fn drop(&mut self) {
                self.dropped.store(true, Ordering::Release);
            }
        }
        let clock = Clock::new(publication_state::MAX_REVISION);
        let dropped = Arc::new(AtomicBool::new(false));
        let context = Arc::new(Context {
            dropped: dropped.clone(),
        });
        // Registry::ready owns nonempty contexts independently of task handles.
        let ready = Arc::new(Mutex::new(Some(context.clone())));
        let pending = Arc::new(AtomicUsize::new(1));
        let queued = Arc::new(AtomicBool::new(false));
        drop(context);
        assert!(!dropped.load(Ordering::Acquire));
        let drain = {
            let (ready, pending, queued) = (ready.clone(), pending.clone(), queued.clone());
            thread::spawn(move || {
                let record_owner = ready.lock().unwrap().take().unwrap();
                // Pop removes the ready entry before publishing the last edge.
                // The pending reservation bridges that otherwise invisible gap.
                thread::yield_now();
                queued.store(true, Ordering::Release);
                pending.fetch_sub(1, Ordering::AcqRel);
                drop(record_owner);
            })
        };
        let closed = clock.observe().is_some_and(|observed| {
            pending.load(Ordering::Acquire) == 0
                && !queued.load(Ordering::Acquire)
                && clock.transition(observed, 0, CLOSED)
        });
        drain.join().unwrap();
        assert!(!closed);
        assert!(queued.load(Ordering::Acquire));
        assert_eq!(pending.load(Ordering::Acquire), 0);
        assert!(dropped.load(Ordering::Acquire));
    });
}

#[test]
fn a_deferred_stamp_preserves_publication_across_closure() {
    late_publication(0, 3, true);
}

#[test]
fn a_saturated_deferred_stamp_never_reuses_a_live_observation() {
    late_publication(1, 1, true);
}

#[test]
fn completed_no_op_barriers_preserve_an_earlier_observation() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(3 * REVISION));
        let observed = clock.observe().unwrap();
        let barrier = {
            let clock = clock.clone();
            thread::spawn(move || {
                drop(clock.enter_barrier());
            })
        };
        barrier.join().unwrap();
        assert_eq!(clock.active(), 0);
        assert!(clock.transition(observed, 0, CLOSED));
    });
}

#[test]
fn an_unstamped_admission_still_excludes_closure_and_rebase() {
    loom::model(|| {
        let clock = Clock::new(REVISION);
        let observed = clock.observe().unwrap();
        let admitted = clock.enter_barrier();
        assert!(!clock.transition(observed, 0, CLOSED));
        assert!(clock.observe().is_none());
        admitted.changed(); // saturate while active
        assert!(clock.observe().is_none());
        assert_eq!(clock.flags(0), MARK);
        drop(admitted);
        assert!(clock.observe().is_none()); // quiescent rebase
        assert!(clock.transition(clock.observe().unwrap(), 0, CLOSED));
    });
}

#[test]
fn a_read_only_certificate_does_not_admit_and_tolerates_no_op_count_changes() {
    loom::model(|| {
        let clock = Clock::new(publication_state::MAX_REVISION);
        let active = clock.enter_barrier();
        let snapshot = clock.snapshot().unwrap();
        assert_eq!(snapshot.flags(0), MARK);
        assert_eq!(clock.active(), 1);
        drop(active);
        assert!(clock.unchanged(snapshot));
        assert_eq!(clock.active(), 0);
        let snapshot = clock.snapshot().unwrap();
        clock.disable_certification();
        assert!(!clock.unchanged(snapshot));
        assert!(clock.snapshot().is_none());
    });
}

#[test]
fn phase_aba_and_numeric_revision_reuse_invalidate_read_only_certificates() {
    loom::model(|| {
        let clock = Clock::new(2 * REVISION);
        let snapshot = clock.snapshot().unwrap();
        assert!(clock.transition(clock.observe().unwrap(), 0, CLOSED));
        assert!(clock.transition(clock.observe().unwrap(), 0, MARK));
        assert!(clock.observe().is_none()); // flags/revision are numerically initial again
        assert!(!clock.unchanged(snapshot));
        assert!(clock.snapshot().is_none());
        // Ordinary counted operation continues after the optional fast path retires.
        assert!(clock.transition(clock.observe().unwrap(), 0, CLOSED));
    });
}

#[test]
fn a_certificate_cannot_accept_retention_from_a_later_epoch() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(publication_state::MAX_REVISION));
        assert!(clock.transition(clock.observe().unwrap(), 0, 3));
        let epoch = Arc::new(AtomicU64::new(EPOCH_STRIDE));
        let mark = Arc::new(MarkWord::default());
        assert!(mark.mark(EPOCH_STRIDE, Retention::Strong));
        let snapshot = clock.snapshot().unwrap();
        let checker = {
            let (clock, epoch, mark) = (clock.clone(), epoch.clone(), mark.clone());
            thread::spawn(move || {
                let observed_epoch = epoch.load(Ordering::Acquire);
                if mark.retention(observed_epoch) == Some(Retention::Strong)
                    && clock.unchanged(snapshot)
                {
                    assert_eq!(observed_epoch, EPOCH_STRIDE);
                }
            })
        };
        assert!(clock.transition(clock.observe().unwrap(), 0, 0));
        epoch.store(2 * EPOCH_STRIDE, Ordering::Release);
        mark.reset();
        assert!(clock.transition(clock.observe().unwrap(), 0, 3));
        assert!(mark.mark(2 * EPOCH_STRIDE, Retention::Strong));
        checker.join().unwrap();
    });
}

/// Models only reservation invisibility and validated commit against a production
/// phase clock. The `ArcSwap` map/page-lifetime implementation is tested separately.
#[test]
fn an_invisible_slot_does_not_prevent_retirement_or_resurrect_after_it() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(8 * REVISION));
        let address = Arc::new(AtomicUsize::new(0));
        let queued = Arc::new(AtomicBool::new(false));
        // Space is reserved already, but has no address and no admission.
        assert!(clock.observe().is_some());
        let publisher = {
            let (clock, address, queued) = (clock.clone(), address.clone(), queued.clone());
            thread::spawn(move || {
                let _admission = clock.enter();
                if clock.flags(0) == CLOSED {
                    return false; // target identity/liveness no longer eligible
                }
                address.store(1, Ordering::Release);
                queued.store(true, Ordering::Release); // insertion shading
                true
            })
        };
        let retired = clock.observe().is_some_and(|observed| {
            address.load(Ordering::Acquire) == 0
                && !queued.load(Ordering::Acquire)
                && clock.transition(observed, 0, CLOSED)
        });
        let committed = publisher.join().unwrap();
        assert!(!retired || !committed);
        assert_eq!(address.load(Ordering::Acquire) != 0, committed);
        assert_eq!(clock.active(), 0);
    });
}

#[path = "../src/root_slot_state.rs"]
mod root_slot_state;

#[test]
fn a_stale_root_remover_cannot_clear_a_reused_slot() {
    loom::model(|| {
        let slot = Arc::new(root_slot_state::Slot::default());
        slot.generation.store(1, Ordering::Relaxed);
        slot.address.store(11, Ordering::Release);
        let stale = {
            let slot = slot.clone();
            thread::spawn(move || slot.take(1))
        };
        let reused = slot.take(1).is_some();
        if reused {
            // Only after take and the deletion barrier is the occupied bit
            // released to the writer. This model elides that later barrier.
            slot.generation.store(2, Ordering::Relaxed);
            slot.address.store(22, Ordering::Release);
        }
        let other = stale.join().unwrap();
        if reused {
            assert_eq!(other, None);
            assert_eq!(slot.address.load(Ordering::Acquire), 22);
            assert_eq!(slot.generation.load(Ordering::Acquire), 2);
        } else {
            assert_eq!(other, Some(11));
            assert_eq!(slot.address.load(Ordering::Acquire), 0);
        }
    });
}

#[test]
fn root_matching_cannot_pair_a_new_generation_with_the_previous_address() {
    loom::model(|| {
        let slot = Arc::new(root_slot_state::Slot::default());
        slot.prepare_generation(1);
        slot.address.store(11, Ordering::Release);
        let reader = {
            let slot = slot.clone();
            thread::spawn(move || {
                let previous = slot.matching_generation(11);
                let replacement = slot.matching_generation(22);
                assert_ne!(previous, Some(2));
                assert_ne!(replacement, Some(1));
            })
        };
        assert_eq!(slot.take(1), Some(11));
        // Production reservation first acquires occupancy released by deletion,
        // then publishes its new generation before committing its new address.
        slot.prepare_generation(2);
        slot.address.store(22, Ordering::Release);
        reader.join().unwrap();
    });
}

#[test]
fn start_sees_the_index_or_the_birth_sees_the_active_domain() {
    for (index, saturated) in [(0, false), (1, false), (0, true), (1, true)] {
        loom::model(move || {
            let clock = Arc::new(Clock::new(8 * REVISION));
            if saturated {
                for _ in 0..8 {
                    drop(clock.enter());
                }
            }
            let indexed = Arc::new(AtomicBool::new(false));
            let birth = {
                let (clock, indexed) = (clock.clone(), indexed.clone());
                thread::spawn(move || {
                    // No synchronization through the object shard: old handoff
                    // and minor collection begin by reading nursery membership.
                    indexed.store(true, Ordering::Relaxed);
                    !matches!(clock.indexed_birth(true), Birth::Idle)
                })
            };
            if saturated {
                // Rebase may intervene between the idle birth and phase start.
                assert!(clock.observe().is_none());
            }
            let observation = clock.observe().unwrap();
            assert!(clock.transition(observation, index, 3));
            let seen_by_scanner = indexed.load(Ordering::Relaxed);
            let saw_active = birth.join().unwrap();
            assert!(seen_by_scanner || saw_active);
        });
    }
}

#[test]
fn epoch_bound_birth_samples_both_stages_without_holding_phase_admission() {
    loom::model(|| {
        let clock = Clock::new(8 * REVISION);
        clock.disable_certification();
        assert!(clock.transition(clock.observe().unwrap(), 0, 3));
        assert!(clock.transition(clock.observe().unwrap(), 1, 21));
        let Birth::Epochs(snapshot) = clock.indexed_birth(true) else {
            panic!("expected an epoch-bound fallback")
        };
        assert_eq!([snapshot.stage(0), snapshot.stage(1)], [Some(0), Some(5)]);
        assert!(clock.transition(clock.observe().unwrap(), 1, 0));
        assert!(!clock.certify_epochs(snapshot, false));
        assert_eq!(clock.active(), 0);
    });
}

#[path = "../src/birth_visibility.rs"]
mod birth_visibility;

#[test]
fn a_sweeper_can_only_see_a_newborn_after_its_mark_is_owned() {
    loom::model(|| {
        let clock = Arc::new(Clock::new(8 * REVISION));
        let visibility = Arc::new(birth_visibility::BirthVisibility::default());
        let indexed = Arc::new(AtomicBool::new(false));
        let marked = Arc::new(AtomicBool::new(false));
        let publisher = {
            let (clock, visibility, indexed, marked) = (
                clock.clone(),
                visibility.clone(),
                indexed.clone(),
                marked.clone(),
            );
            thread::spawn(move || {
                assert!(!visibility.indexed());
                visibility.index();
                indexed.store(true, Ordering::Relaxed);
                loop {
                    match clock.indexed_birth(true) {
                        Birth::Inline(snapshot) => {
                            marked.store(true, Ordering::Relaxed);
                            if clock.unchanged(snapshot) && visibility.publish_if_unobserved() {
                                return;
                            }
                            visibility.retry();
                        }
                        Birth::Idle if visibility.publish_if_unobserved() => return,
                        Birth::Idle => visibility.retry(),
                        Birth::Epochs(_) => {
                            panic!("model has a certifiable clock")
                        }
                    }
                }
            })
        };
        assert!(clock.transition(clock.observe().unwrap(), 1, 3));
        if indexed.load(Ordering::Relaxed) && visibility.visible() {
            marked.store(true, Ordering::Relaxed);
        }
        // A sweep after the root cursor may pass a private birth. If it sees
        // a live descriptor, either the root cursor or the publisher marked it.
        if indexed.load(Ordering::Relaxed) && visibility.visible() {
            assert!(marked.load(Ordering::Relaxed));
        }
        publisher.join().unwrap();
        assert!(visibility.is_live());
        assert!(marked.load(Ordering::Relaxed));
    });
}

#[test]
fn rearmed_graph_is_scanned_at_start_or_retries_into_strong_tracing() {
    for numeric in [false, true] {
        loom::model(move || {
            let clock = Arc::new(Clock::new(16 * REVISION));
            assert!(clock.transition(clock.observe().unwrap(), 1, 21));
            if !numeric {
                clock.disable_certification();
            }
            let visibility = Arc::new(birth_visibility::BirthVisibility::default());
            visibility.index();
            assert!(!visibility.visible()); // A scan skipped the first attempt.
            let publisher = {
                let (clock, visibility) = (clock.clone(), visibility.clone());
                thread::spawn(move || {
                    visibility.retry();
                    match clock.indexed_birth(false) {
                        Birth::Idle => visibility.publish_if_unobserved(),
                        Birth::Inline(snapshot) => {
                            snapshot.stage(1) == Some(5)
                                && clock.unchanged(snapshot)
                                && visibility.publish_if_unobserved()
                        }
                        // A graph owes a traversal in the new strong phase.
                        Birth::Epochs(s) => {
                            s.stage(1) == Some(5)
                                && clock.certify_epochs(s, false)
                                && visibility.publish_if_unobserved()
                        }
                    }
                })
            };
            assert!(clock.transition(clock.observe().unwrap(), 1, 0));
            assert!(clock.transition(clock.observe().unwrap(), 1, 3));
            let scanned = visibility.visible();
            if publisher.join().unwrap() {
                assert!(
                    scanned,
                    "new strong scan missed a graph that published without traversal"
                );
            }
        });
    }
}

#[test]
fn saturation_and_rebase_preserve_epoch_bound_publication_without_admission() {
    loom::model(|| {
        let clock = Clock::new(2 * REVISION);
        assert!(clock.transition(clock.observe().unwrap(), 0, 3));
        let Birth::Inline(snapshot) = clock.indexed_birth(true) else {
            panic!("fresh leaf can initialize its own strong mark")
        };
        drop(clock.enter());
        assert!(!clock.unchanged(snapshot));
        for _ in 0..2 {
            for leaf in [true, false] {
                let Birth::Epochs(snapshot) = clock.indexed_birth(leaf) else {
                    panic!("birth requires owning epoch identity")
                };
                assert_eq!(clock.active(), 0);
                assert!(clock.certify_epochs(snapshot, !leaf));
            }
            // The first observation rebases, and subsequent ones can proceed.
            if let Some(o) = clock.observe() {
                assert!(clock.transition(o, 0, 3));
            }
        }
    });
}

#[test]
fn linked_graph_traversal_defeats_an_earlier_empty_queue_observation() {
    for saturated in [false, true] {
        loom::model(move || {
            let clock = Arc::new(Clock::new(8 * REVISION));
            assert!(clock.transition(clock.observe().unwrap(), 1, 3));
            if saturated {
                for _ in 0..7 {
                    drop(clock.enter());
                }
            }
            let linked = Arc::new(AtomicBool::new(false));
            let visibility = Arc::new(birth_visibility::BirthVisibility::default());
            visibility.index();
            let publisher = {
                let (clock, linked, visibility) =
                    (clock.clone(), linked.clone(), visibility.clone());
                thread::spawn(move || {
                    let snapshot = match clock.indexed_birth(false) {
                        Birth::Epochs(snapshot) => snapshot,
                        Birth::Inline(late) => {
                            assert_ne!(late.stage(1), Some(0));
                            return false;
                        }
                        Birth::Idle => panic!("active cycle"),
                    };
                    if snapshot.stage(1) != Some(0) {
                        return false;
                    }
                    linked.store(true, Ordering::Release);
                    clock.certify_epochs(snapshot, true) && visibility.publish_if_unobserved()
                })
            };
            if saturated {
                assert!(clock.observe().is_none());
            }
            let observed = clock.observe().unwrap();
            // These operations use the same exclusion as an actual mark claim and
            // closure. A claim on a private graph rejects its publisher's activation.
            let traced = linked.swap(false, Ordering::AcqRel) && visibility.visible();
            let closed = clock.transition(observed, 1, 5);
            if publisher.join().unwrap() && closed {
                assert!(traced, "a graph escaped strong tracing");
            }
        });
    }
}

#[test]
fn a_stale_graph_ticket_cannot_claim_a_later_live_attempt() {
    loom::model(|| {
        let cancelled = Arc::new(AtomicBool::new(false));
        let visibility = Arc::new(birth_visibility::BirthVisibility::default());
        visibility.index();
        assert!(!visibility.visible());
        let publisher = {
            let (cancelled, visibility) = (cancelled.clone(), visibility.clone());
            thread::spawn(move || {
                cancelled.store(true, Ordering::Release);
                visibility.retry();
                visibility.publish_if_unobserved()
            })
        };
        if visibility.visible() {
            assert!(cancelled.load(Ordering::Acquire));
        }
        publisher.join().unwrap();
    });
}

#[test]
fn first_graph_ticket_cannot_visit_a_later_live_attempt() {
    loom::model(|| {
        let visibility = Arc::new(birth_visibility::BirthVisibility::default());
        visibility.index();
        assert!(visibility.first_attempt());
        let worker = {
            let visibility = visibility.clone();
            thread::spawn(move || visibility.claim_first())
        };
        visibility.retry();
        assert!(!visibility.first_attempt());
        assert!(visibility.publish_if_unobserved());
        assert!(!worker.join().unwrap());
        assert!(visibility.visible());
    });
}
