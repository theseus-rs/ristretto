//! Selection and certificate ordering only; record guards and pointer owners
//! are checked by real catalog tests, not represented by these two models.
use super::{CANCEL, COMMIT, UNDECIDED};
use loom::sync::Arc;
use loom::sync::atomic::{AtomicU8, Ordering};
use loom::thread;
#[expect(
    clippy::duplicate_mod,
    reason = "exercise the production decision atomics with Loom"
)]
#[path = "decision.rs"]
mod decision;

#[test]
fn commit_and_cancel_choose_one_immutable_outcome() {
    loom::model(|| {
        let selected = Arc::new(AtomicU8::new(UNDECIDED));
        let other = selected.clone();
        let worker = thread::spawn(move || decision::choose(&other, CANCEL));
        let committed = decision::choose(&selected, COMMIT);
        let cancelled = worker.join().unwrap();
        assert_ne!(committed.is_ok(), cancelled.is_ok());
        let outcome = if committed.is_ok() { COMMIT } else { CANCEL };
        assert_eq!(selected.load(Ordering::Acquire), outcome);
        assert_eq!(decision::choose(&selected, COMMIT), Err(outcome));
        assert_eq!(decision::choose(&selected, CANCEL), Err(outcome));
    });
}

#[test]
fn completion_acquisition_observes_the_preceding_effect() {
    loom::model(|| {
        let completed = Arc::new(AtomicU8::new(UNDECIDED));
        let effect = Arc::new(AtomicU8::new(0));
        let other_completed = completed.clone();
        let other_effect = effect.clone();
        let worker = thread::spawn(move || {
            other_effect.store(1, Ordering::Relaxed);
            decision::certify(&other_completed, COMMIT);
        });
        if completed.load(Ordering::Acquire) == COMMIT {
            assert_eq!(effect.load(Ordering::Relaxed), 1);
        }
        worker.join().unwrap();
        assert_eq!(completed.load(Ordering::Acquire), COMMIT);
        assert_eq!(effect.load(Ordering::Relaxed), 1);
    });
}
