//! Source-bound quota atomics; backing allocation and destruction are not modeled.
use super::RegisterError;
use loom::sync::Arc;
use loom::sync::atomic::{AtomicUsize, Ordering};
use loom::thread;
#[expect(
    clippy::duplicate_mod,
    reason = "exercise the production quota atomics with Loom"
)]
#[path = "quota.rs"]
mod quota;

#[test]
fn competing_registrations_cannot_exceed_the_limit() {
    loom::model(|| {
        let count = Arc::new(AtomicUsize::new(0));
        let other = count.clone();
        let worker = thread::spawn(move || quota::acquire(&other, 1));
        let right = quota::acquire(&count, 1);
        let left = worker.join().unwrap();
        assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
        assert_eq!(count.load(Ordering::Acquire), 1);
        quota::release(&count);
        assert!(quota::acquire(&count, 1).is_ok());
        quota::release(&count);
        assert_eq!(count.load(Ordering::Acquire), 0);
    });
}
