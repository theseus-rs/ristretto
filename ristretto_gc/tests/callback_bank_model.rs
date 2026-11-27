//! Reduced native callback-bank handoffs. The checked cell covers one reused
//! slot. The successor flag models observation of a published owning link.
#![cfg(not(target_family = "wasm"))]
#![allow(unsafe_code)] // Loom tracks the raw cell accesses made by winning actors.
use loom::cell::UnsafeCell;
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::thread;

#[test]
fn a_reused_slot_has_one_owner_for_each_published_ticket() {
    loom::model(|| {
        let occupied = Arc::new(AtomicBool::new(true));
        let head = Arc::new(AtomicUsize::new(0));
        let tail = Arc::new(AtomicUsize::new(1));
        let slot = Arc::new(UnsafeCell::new(Some(0)));
        let claim = || {
            let (head, tail, occupied, slot) =
                (head.clone(), tail.clone(), occupied.clone(), slot.clone());
            thread::spawn(move || {
                let ticket = head.load(Ordering::Relaxed);
                if ticket < tail.load(Ordering::Acquire)
                    && head
                        .compare_exchange(ticket, ticket + 1, Ordering::Relaxed, Ordering::Relaxed)
                        .is_ok()
                {
                    let value = slot.with_mut(|p| unsafe { (*p).take() });
                    assert_eq!(value, Some(ticket));
                    occupied.store(false, Ordering::Release);
                }
            })
        };
        let first = claim();
        let other = claim();
        if !occupied.load(Ordering::Acquire) {
            slot.with_mut(|p| unsafe { *p = Some(1) });
            occupied.store(true, Ordering::Relaxed);
            tail.store(2, Ordering::Release);
        }
        first.join().unwrap();
        other.join().unwrap();
        let claimed = head.load(Ordering::Relaxed);
        let remaining = slot.with(|p| unsafe { *p });
        assert_eq!(
            remaining,
            (claimed < tail.load(Ordering::Relaxed)).then_some(claimed)
        );
    });
}

#[test]
fn seeing_a_successor_requires_a_fresh_empty_check_on_the_sealed_bank() {
    loom::model(|| {
        let tail = Arc::new(AtomicUsize::new(0));
        let linked = Arc::new(AtomicBool::new(false));
        let producer = {
            let (tail, linked) = (tail.clone(), linked.clone());
            thread::spawn(move || {
                tail.store(1, Ordering::Release);
                linked.store(true, Ordering::Release);
            })
        };
        if tail.load(Ordering::Acquire) == 0 && linked.load(Ordering::Acquire) {
            assert_eq!(tail.load(Ordering::Acquire), 1);
        }
        producer.join().unwrap();
    });
}
