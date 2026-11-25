//! Design checks for replacing counted admission with durable, helpable publication.
//!
//! The counterexamples deliberately remove the epoch/graph exclusion currently
//! supplied by production admission. They do not demonstrate a bug in the current
//! admitted collector. The positive cell model is a prerequisite, not an integrated
//! publication protocol: registry discovery, graph ownership and phase closure
//! still need a runtime design and full-collector verification.
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use loom::sync::{Arc, mpsc};
use loom::thread;

#[path = "../src/mark_state.rs"]
#[expect(
    dead_code,
    reason = "exercise selected production mark transitions in rejected designs"
)]
mod mark_state;
use mark_state::{EPOCH_STRIDE, MarkWord, Retention};

/// A completed operation is not enough to stop a helper which already checked
/// its completion bit. Replay must itself have an atomic validity condition.
#[test]
#[should_panic(expected = "late helper rewound the epoch")]
fn rejected_completed_bit_allows_stale_epoch_replay() {
    loom::model(|| {
        let mark = Arc::new(MarkWord::default());
        let done = Arc::new(AtomicBool::new(false));
        let (observed_tx, observed_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let delayed = {
            let (mark, done) = (mark.clone(), done.clone());
            thread::spawn(move || {
                let needs_help = !done.load(Ordering::Acquire);
                observed_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                if needs_help {
                    mark.mark(EPOCH_STRIDE, Retention::Strong);
                }
            })
        };
        observed_rx.recv().unwrap();
        assert!(mark.mark(EPOCH_STRIDE, Retention::Strong));
        done.store(true, Ordering::Release);
        // Proposed early acknowledgement would permit a later cycle here.
        assert!(mark.mark(2 * EPOCH_STRIDE, Retention::Strong));
        resume_tx.send(()).unwrap();
        delayed.join().unwrap();
        assert!(
            mark.marked(2 * EPOCH_STRIDE),
            "late helper rewound the epoch"
        );
    });
}

#[test]
#[should_panic(expected = "late helper replaced a live identity")]
fn rejected_completed_bit_allows_registry_resurrection() {
    loom::model(|| {
        let index = Arc::new(AtomicU64::new(0));
        let done = Arc::new(AtomicBool::new(false));
        let (observed_tx, observed_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let delayed = {
            let (index, done) = (index.clone(), done.clone());
            thread::spawn(move || {
                let needs_help = !done.load(Ordering::Acquire);
                observed_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                if needs_help {
                    // Equivalent to replaying insert(address, old_metadata).
                    index.store(1, Ordering::Release);
                }
            })
        };
        observed_rx.recv().unwrap();
        index.store(1, Ordering::Release);
        done.store(true, Ordering::Release);
        index.store(0, Ordering::Release); // retirement
        index.store(2, Ordering::Release); // reuse by a different identity
        resume_tx.send(()).unwrap();
        delayed.join().unwrap();
        assert_eq!(
            index.load(Ordering::Acquire),
            2,
            "late helper replaced a live identity"
        );
    });
}

#[test]
#[should_panic(expected = "mark winner had not published its traversal")]
fn rejected_mark_bit_is_not_a_completed_publication() {
    loom::model(|| {
        let mark = Arc::new(MarkWord::default());
        let queued = Arc::new(AtomicBool::new(false));
        let (marked_tx, marked_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let delayed = {
            let (mark, queued) = (mark.clone(), queued.clone());
            thread::spawn(move || {
                assert!(mark.mark(EPOCH_STRIDE, Retention::Strong));
                marked_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                queued.store(true, Ordering::Release);
            })
        };
        marked_rx.recv().unwrap();
        // A second helper sees the mark but cannot infer queue publication.
        assert!(!mark.mark(EPOCH_STRIDE, Retention::Strong));
        let incorrectly_closed = !queued.load(Ordering::Acquire);
        resume_tx.send(()).unwrap();
        delayed.join().unwrap();
        assert!(
            !incorrectly_closed,
            "mark winner had not published its traversal"
        );
    });
}

/// Minimal canonical membership/traversal cell. This is MODEL-ONLY and deliberately
/// does not call the production registry/mark/queue mutation methods. All externally
/// visible effects in this example are represented by its one CAS, so a delayed
/// helper cannot perform a separate stale effect after another helper acknowledges.
/// A real implementation additionally needs discovery, rooting, phases and budgets.
#[derive(Debug)]
struct PublicationCell(AtomicU64);

const PREPARED: u64 = 0;
const QUEUED: u64 = 1;
const TRACED: u64 = 2;
const RETIRED: u64 = 3;
const STATE_MASK: u64 = 3;

/// A generation is immutable and may not wrap while a helper can retain this token.
#[derive(Clone, Copy, Debug)]
struct Ticket(u64);

impl PublicationCell {
    fn new(generation: u64) -> (Self, Ticket) {
        assert!(generation > 0 && generation <= u64::MAX >> 2);
        let ticket = Ticket(generation << 2);
        (Self(AtomicU64::new(ticket.0 | PREPARED)), ticket)
    }

    fn transition(&self, ticket: Ticket, from: u64, to: u64) -> bool {
        self.0
            .compare_exchange(
                ticket.0 | from,
                ticket.0 | to,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    fn publish(&self, ticket: Ticket) -> bool {
        // QUEUED is both registry membership and a discoverable traversal record.
        // There is no separate side effect after this CAS for a late helper to repeat.
        self.transition(ticket, PREPARED, QUEUED)
    }

    fn state(&self, ticket: Ticket) -> Option<u64> {
        let word = self.0.load(Ordering::Acquire);
        (word & !STATE_MASK == ticket.0).then_some(word & STATE_MASK)
    }

    fn recycle(&self, ticket: Ticket) -> Option<Ticket> {
        let next = ticket.0.checked_add(STATE_MASK + 1)?;
        self.0
            .compare_exchange(
                ticket.0 | RETIRED,
                next | PREPARED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .ok()
            .map(|_| Ticket(next))
    }
}

#[test]
fn canonical_commit_is_helpable_while_original_publisher_is_stopped() {
    loom::model(|| {
        let (cell, ticket) = PublicationCell::new(1);
        let cell = Arc::new(cell);
        let (observed_tx, observed_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let original = {
            let cell = cell.clone();
            thread::spawn(move || {
                // The proposal assumes the immutable record is already discoverable.
                observed_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                assert!(!cell.publish(ticket));
            })
        };
        observed_rx.recv().unwrap();
        assert!(cell.publish(ticket));
        assert_eq!(cell.state(ticket), Some(QUEUED));
        assert!(cell.transition(ticket, QUEUED, TRACED));
        // Unrelated reclamation need not consult the original publisher's stack.
        let (garbage, garbage_ticket) = PublicationCell::new(1);
        assert!(garbage.publish(garbage_ticket));
        assert!(garbage.transition(garbage_ticket, QUEUED, TRACED));
        assert!(garbage.transition(garbage_ticket, TRACED, RETIRED));
        resume_tx.send(()).unwrap();
        original.join().unwrap();
        assert_eq!(cell.state(ticket), Some(TRACED));
    });
}

#[test]
fn competing_helpers_publish_exactly_one_logical_traversal() {
    loom::model(|| {
        let (cell, ticket) = PublicationCell::new(1);
        let cell = Arc::new(cell);
        let helper = {
            let cell = cell.clone();
            thread::spawn(move || cell.publish(ticket))
        };
        let own = cell.publish(ticket);
        assert_ne!(own, helper.join().unwrap());
        assert_eq!(cell.state(ticket), Some(QUEUED));
        assert!(cell.transition(ticket, QUEUED, TRACED));
        assert!(!cell.transition(ticket, QUEUED, TRACED));
    });
}

#[test]
fn delayed_helper_cannot_republish_retired_or_reused_membership() {
    loom::model(|| {
        let (cell, ticket) = PublicationCell::new(1);
        let cell = Arc::new(cell);
        let (observed_tx, observed_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let delayed = {
            let cell = cell.clone();
            thread::spawn(move || {
                assert_eq!(cell.state(ticket), Some(PREPARED));
                observed_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                assert!(!cell.publish(ticket));
                assert!(!cell.transition(ticket, TRACED, RETIRED));
            })
        };
        observed_rx.recv().unwrap();
        assert!(cell.publish(ticket));
        assert!(cell.transition(ticket, QUEUED, TRACED));
        assert!(cell.transition(ticket, TRACED, RETIRED));
        assert!(!cell.publish(ticket));
        let replacement = cell.recycle(ticket).unwrap();
        assert!(cell.publish(replacement));
        resume_tx.send(()).unwrap();
        delayed.join().unwrap();
        assert_eq!(cell.state(ticket), None);
        assert_eq!(cell.state(replacement), Some(QUEUED));
    });
}

#[test]
fn cancellation_races_commit_without_half_published_membership() {
    loom::model(|| {
        let (cell, ticket) = PublicationCell::new(1);
        let cell = Arc::new(cell);
        let helper = {
            let cell = cell.clone();
            thread::spawn(move || cell.publish(ticket))
        };
        let cancelled = cell.transition(ticket, PREPARED, RETIRED);
        let published = helper.join().unwrap();
        assert_ne!(cancelled, published);
        assert_eq!(
            cell.state(ticket),
            Some(if cancelled { RETIRED } else { QUEUED })
        );
        // Cancellation after commit requires a separate owning-root release protocol;
        // it is not modeled by changing a published record back to PREPARED.
    });
}

#[test]
fn generation_exhaustion_cannot_reuse_a_stale_ticket() {
    loom::model(|| {
        let (cell, ticket) = PublicationCell::new(u64::MAX >> 2);
        assert!(cell.transition(ticket, PREPARED, RETIRED));
        assert!(cell.recycle(ticket).is_none());
        assert_eq!(cell.state(ticket), Some(RETIRED));
        assert!(!cell.publish(ticket));
    });
}

#[test]
#[should_panic(expected = "uncoordinated announcement crossed phase closure")]
fn rejected_canonical_cell_alone_does_not_coordinate_phase_closure() {
    loom::model(|| {
        let (cell, ticket) = PublicationCell::new(1);
        let cell = Arc::new(cell);
        let phase_closed = Arc::new(AtomicBool::new(false));
        let (observed_tx, observed_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let publisher = {
            let (cell, phase_closed) = (cell.clone(), phase_closed.clone());
            thread::spawn(move || {
                let old_phase = !phase_closed.load(Ordering::Acquire);
                observed_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                assert!(old_phase);
                assert!(cell.publish(ticket));
            })
        };
        observed_rx.recv().unwrap();
        assert_eq!(cell.state(ticket), Some(PREPARED));
        phase_closed.store(true, Ordering::Release);
        resume_tx.send(()).unwrap();
        publisher.join().unwrap();
        assert_ne!(
            cell.state(ticket),
            Some(QUEUED),
            "uncoordinated announcement crossed phase closure"
        );
    });
}

/// One-record abstraction in which announcement and both phase boundaries share
/// a CAS word. This demonstrates the required atomic relation, not how an
/// unbounded heap's catalog should be represented. No external effects follow CAS.
struct CoupledCell(AtomicU64);

impl CoupledCell {
    fn close(&self, domain: usize) -> bool {
        assert!(domain < 2);
        let snapshot = self.0.load(Ordering::Acquire);
        if snapshot & STATE_MASK == QUEUED {
            return false;
        }
        self.0
            .compare_exchange(
                snapshot,
                snapshot | (4 << domain),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    fn publish_snapshot(&self, snapshot: u64) -> bool {
        // This abstract request is allowed only before either reference boundary.
        // A failed snapshot requires fresh eligibility validation, not replay.
        if snapshot != PREPARED {
            return false;
        }
        self.0
            .compare_exchange(snapshot, QUEUED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

#[test]
fn coupled_announcement_and_closure_cannot_both_win() {
    loom::model(|| {
        let cell = Arc::new(CoupledCell(AtomicU64::new(PREPARED)));
        let publisher = {
            let cell = cell.clone();
            thread::spawn(move || {
                let snapshot = cell.0.load(Ordering::Acquire);
                cell.publish_snapshot(snapshot)
            })
        };
        let closed = cell.close(0);
        let committed = publisher.join().unwrap();
        assert_ne!(closed, committed);
        if committed {
            assert_eq!(cell.0.load(Ordering::Acquire), QUEUED);
            assert!(!cell.close(1));
        } else {
            assert!(!cell.publish_snapshot(cell.0.load(Ordering::Acquire)));
        }
    });
}

#[test]
fn coupled_commit_closes_both_domains_without_resuming_original_publisher() {
    loom::model(|| {
        let cell = Arc::new(CoupledCell(AtomicU64::new(PREPARED)));
        let (observed_tx, observed_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let original = {
            let cell = cell.clone();
            thread::spawn(move || {
                let snapshot = cell.0.load(Ordering::Acquire);
                observed_tx.send(()).unwrap();
                resume_rx.recv().unwrap();
                assert!(!cell.publish_snapshot(snapshot));
            })
        };
        observed_rx.recv().unwrap();
        assert!(cell.publish_snapshot(cell.0.load(Ordering::Acquire)));
        assert!(!cell.close(0));
        assert!(!cell.close(1));
        assert_eq!(
            cell.0
                .compare_exchange(QUEUED, TRACED, Ordering::AcqRel, Ordering::Acquire),
            Ok(QUEUED)
        );
        assert!(cell.close(0));
        assert!(cell.close(1));
        resume_tx.send(()).unwrap();
        original.join().unwrap();
        assert_eq!(cell.0.load(Ordering::Acquire), TRACED | 4 | 8);
    });
}
