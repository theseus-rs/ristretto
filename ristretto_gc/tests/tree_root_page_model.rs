//! Selected atomic transitions of the ordinary numeric root registry.
#![cfg(not(target_family = "wasm"))]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use loom::thread;
use std::num::NonZeroU64;
const OWNED_TREE_ROOT: u64 = 1 << 63;
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
