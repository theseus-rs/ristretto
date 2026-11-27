//! Selected atomic transitions of the ordinary numeric root registry.
#![cfg(not(target_family = "wasm"))]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use loom::thread;
use std::num::NonZeroU64;
const OWNED_TREE_ROOT: u64 = 1 << 63;
#[path = "../src/publication_state.rs"]
mod publication_state;
#[path = "../src/tree_root_page_state.rs"]
mod state;
use state::{Claim, Entry, Occupancy};

#[test]
fn reserving_and_sealing_an_empty_root_page_are_mutually_exclusive() {
    loom::model(|| {
        let occupancy = Arc::new(Occupancy::default());
        let other = occupancy.clone();
        let writer = thread::spawn(move || other.claim(1));
        let sealed = occupancy.seal_empty();
        assert_eq!(
            writer.join().unwrap(),
            if sealed {
                Claim::Sealed
            } else {
                Claim::Reserved
            }
        );
        assert_eq!(occupancy.slots(), u64::from(!sealed));
    });
}

#[test]
fn a_scan_or_stale_removal_cannot_pair_a_reused_id_with_the_previous_address() {
    loom::model(|| {
        let entry = Arc::new(Entry::default());
        let first = NonZeroU64::new(OWNED_TREE_ROOT | 1).unwrap();
        let second = NonZeroU64::new(2).unwrap();
        entry.publish(first, 10);
        let other = entry.clone();
        let reader = thread::spawn(move || {
            if let Some(pair) = other.read() {
                assert!(pair == (first, 10) || pair == (second, 20));
            }
        });
        assert_eq!(entry.remove(None), None);
        assert_eq!(entry.remove(Some(first)), Some(10));
        entry.publish(second, 20);
        assert_eq!(entry.remove(Some(first)), None);
        reader.join().unwrap();
        assert_eq!(entry.remove(None), Some(20));
    });
}

#[test]
fn independently_prepared_roots_do_not_lose_membership_during_retirement() {
    loom::model(|| {
        let occupancy = Arc::new(Occupancy::default());
        assert_eq!(occupancy.claim(1), Claim::Reserved);
        let other = occupancy.clone();
        let writer = thread::spawn(move || other.claim(2));
        if occupancy.release(1) {
            occupancy.seal_empty();
        }
        let claimed = writer.join().unwrap();
        if claimed == Claim::Reserved {
            assert_eq!(occupancy.slots(), 2);
            assert!(!occupancy.seal_empty());
        } else {
            assert_eq!(claimed, Claim::Sealed);
            assert_eq!(occupancy.slots(), 0);
        }
    });
}

#[test]
fn a_scanner_can_veto_only_the_exact_tree_removal_it_observes() {
    loom::model(|| {
        let entry = Arc::new(Entry::default());
        let first = NonZeroU64::new(u64::MAX).unwrap();
        let second = NonZeroU64::new(4).unwrap();
        entry.publish(first, 10);
        assert_eq!(entry.begin_removal(first), Some(10));
        let scanning = entry.clone();
        let scan = thread::spawn(move || scanning.scan());
        let committed = entry.removal_pending(first) && entry.finish_removal(first);
        if committed {
            entry.publish(second, 20);
        }
        match scan.join().unwrap() {
            state::Scan::Root(address) => assert_eq!(address, if committed { 20 } else { 10 }),
            state::Scan::Vacant => assert!(committed),
            state::Scan::Retry => {}
        }
        if !committed {
            assert_eq!(entry.remove(Some(first)), Some(10));
            entry.publish(second, 20);
        }
        assert!(!entry.removal_pending(first) && !entry.finish_removal(first));
        assert_eq!(entry.remove(Some(first)), None);
        assert_eq!(entry.read(), Some((second, 20)));
    });
}

#[test]
fn an_admitted_remover_and_a_covered_remover_cannot_both_take_one_generation() {
    loom::model(|| {
        let entry = Arc::new(Entry::default());
        let exact = NonZeroU64::new(OWNED_TREE_ROOT | 7).unwrap();
        entry.publish(exact, 11);
        assert_eq!(entry.begin_removal(exact), Some(11));
        let other = entry.clone();
        let compatible = thread::spawn(move || other.remove(Some(exact)));
        let covered = entry.finish_removal(exact);
        assert_eq!(compatible.join().unwrap(), (!covered).then_some(11));
        let replacement = NonZeroU64::new(8).unwrap();
        entry.publish(replacement, 22);
        assert_eq!(entry.remove(Some(exact)), None);
        assert!(!entry.finish_removal(exact));
        assert_eq!(entry.remove(None), Some(22));
    });
}

#[test]
fn new_epoch_scanner_observes_removal_intent_published_through_the_clock() {
    use publication_state::{Birth, Clock, REVISION};

    loom::model(|| {
        let entry = Arc::new(Entry::default());
        let exact = NonZeroU64::new(OWNED_TREE_ROOT | 9).unwrap();
        entry.publish(exact, 11);
        let clock = Arc::new(Clock::new(64 * REVISION));
        let scanning = entry.clone();
        let phases = clock.clone();
        let scanner = thread::spawn(move || {
            assert!(phases.transition(phases.observe().unwrap(), 1, 1));
            scanning.scan()
        });
        let detached = entry.begin_removal(exact).is_some()
            && matches!(clock.indexed_root(), Birth::Idle)
            && entry.finish_removal(exact);
        if scanner.join().unwrap() == state::Scan::Root(11) {
            assert!(!detached);
        }
    });
}
