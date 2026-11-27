//! Coalescing request flags must preserve the tickets of every preceding publisher.
#![cfg(not(target_family = "wasm"))]

use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use loom::thread;

#[test]
fn every_issued_ticket_is_captured_or_leaves_another_full_request() {
    loom::model(|| {
        let issued = Arc::new(AtomicU64::new(0));
        let requested = Arc::new(AtomicBool::new(false));
        let request = || {
            let issued = issued.clone();
            let requested = requested.clone();
            thread::spawn(move || {
                issued.fetch_add(1, Ordering::AcqRel);
                requested.swap(true, Ordering::AcqRel);
            })
        };
        // The earlier issuer can publish its flag after the later issuer.
        let first = request();
        let second = request();
        let cutoff = if requested.swap(false, Ordering::AcqRel) {
            issued.load(Ordering::Acquire)
        } else {
            0
        };
        first.join().unwrap();
        second.join().unwrap();
        assert!(cutoff == 2 || requested.load(Ordering::Acquire));
    });
}
