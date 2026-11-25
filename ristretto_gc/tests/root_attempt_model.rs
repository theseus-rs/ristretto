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
