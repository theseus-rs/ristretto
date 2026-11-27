//! The same one-shot decision is shared by the registry and both epoch queues.
#![cfg(not(target_family = "wasm"))]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicU8, Ordering};
use loom::thread;

#[path = "../src/root_attempt_state.rs"]
mod root_attempt_state;
use root_attempt_state::Decision;

#[test]
fn scanning_and_two_domain_claims_agree_with_commit_even_after_removal() {
    loom::model(|| {
        let attempt = Arc::new(Decision::default());
        let first = {
            let attempt = attempt.clone();
            thread::spawn(move || attempt.owes_trace())
        };
        let scanner = {
            let attempt = attempt.clone();
            thread::spawn(move || attempt.scan())
        };
        let committed = attempt.commit();
        assert_eq!(attempt.live(), committed);
        assert_eq!(attempt.take(), committed);
        // No deletion barrier pays the second domain's tracing debt.
        assert_eq!(attempt.owes_trace(), committed);
        assert_eq!(first.join().unwrap(), committed);
        assert!(!scanner.join().unwrap() || committed);
        assert!(!attempt.live());
        assert!(!attempt.take());
        assert!(!attempt.commit());
    });
}

#[test]
fn scanner_and_compatibility_take_race_a_certified_drop() {
    loom::model(|| {
        let record = Arc::new(Decision::default());
        assert!(record.commit());
        assert!(record.begin_removal());
        let scanner = {
            let record = record.clone();
            thread::spawn(move || record.scan())
        };
        let compatibility = {
            let record = record.clone();
            thread::spawn(move || record.take())
        };
        let removed = record.finish_removal();
        let observed = scanner.join().unwrap();
        assert!(!(removed && observed));
        assert_ne!(compatibility.join().unwrap(), removed);
        assert!(record.owes_trace());
        assert!(!record.live());
        assert!(!record.begin_removal());
    });
}

#[test]
fn revoking_a_source_cannot_commit_an_unrelated_deletion_attempt() {
    loom::model(|| {
        let source = Arc::new(Decision::default());
        assert!(source.commit() && source.begin_removal());
        let debt = Arc::new(Decision::default());
        let descriptor = Arc::new(AtomicU8::new(1));
        let revoker = {
            let source = source.clone();
            let descriptor = descriptor.clone();
            thread::spawn(move || {
                assert!(source.take());
                descriptor.store(2, Ordering::Release);
            })
        };
        let captured = descriptor.load(Ordering::Acquire);
        let committed = source.removal_pending() && debt.commit();
        if committed {
            assert_eq!(captured, 1);
        }
        revoker.join().unwrap();
        assert!(!source.removal_pending());
        assert_eq!(debt.owes_trace(), committed);
    });
}
