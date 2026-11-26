//! Enrollment publishes its slot before sharing an RMW with the reference
//! notifier. Either the poll sees the linked queue or a helper sees the slot.
#![cfg(not(target_family = "wasm"))]

use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, Ordering};
use loom::thread;

#[test]
fn linked_reference_cannot_miss_a_waiting_handler() {
    loom::model(|| {
        let enrolled = Arc::new(AtomicBool::new(false));
        let slot = Arc::new(AtomicBool::new(false));
        let queue = Arc::new(AtomicBool::new(false));
        let poll = {
            let (slot, enrolled, queue) = (slot.clone(), enrolled.clone(), queue.clone());
            thread::spawn(move || {
                slot.store(true, Ordering::Release);
                enrolled.swap(true, Ordering::AcqRel);
                queue.load(Ordering::Acquire)
            })
        };
        queue.store(true, Ordering::Release);
        let woke = enrolled.fetch_or(false, Ordering::AcqRel) && slot.load(Ordering::Acquire);
        assert!(poll.join().unwrap() || woke);
    });
}

/// An old notifier can retain a sampled ticket across cancellation and slot
/// reuse. Loom checks both the mutable accesses and the observed payload.
#[test]
fn only_the_winning_ticket_touches_the_callback_cell() {
    use loom::cell::Cell;
    use loom::sync::atomic::AtomicUsize;

    struct Slot {
        occupied: AtomicBool,
        ticket: AtomicUsize,
        value: Cell<usize>,
    }

    impl Slot {
        fn take(&self, ticket: usize) -> bool {
            if self
                .ticket
                .compare_exchange(ticket, 0, Ordering::AcqRel, Ordering::Relaxed)
                .is_err()
            {
                return false;
            }
            assert_eq!(self.value.replace(0), ticket);
            self.occupied.store(false, Ordering::Release);
            true
        }
    }
    loom::model(|| {
        let slot = Arc::new(Slot {
            occupied: AtomicBool::new(true),
            ticket: AtomicUsize::new(1),
            value: Cell::new(1),
        });
        let notifier = {
            let slot = slot.clone();
            thread::spawn(move || slot.take(1))
        };
        let cancelled = slot.take(1);
        if slot
            .occupied
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            assert_eq!(slot.value.replace(2), 0);
            slot.ticket.store(2, Ordering::Release);
            assert!(slot.take(2));
        }
        assert_ne!(cancelled, notifier.join().unwrap());
    });
}
