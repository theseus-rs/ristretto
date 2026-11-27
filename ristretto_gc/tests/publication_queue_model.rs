//! Production FIFO transitions with linearizable owning-link operations under Loom.
//! Includes selected admission/claim handshakes. `ArcSwap` reclamation internals
//! are outside this model; native forced schedules exercise actual `ArcSwap` links.
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use loom::sync::{Arc, Mutex, RwLock};
use loom::thread;

#[path = "../src/publication_queue_state.rs"]
mod state;
use state::Queue;
#[path = "../src/publication_state.rs"]
mod publication_state;
use publication_state::{Clock, REVISION};

struct Link<T>(Mutex<Option<Arc<T>>>);
#[expect(
    clippy::unwrap_used,
    reason = "a poisoned model link must fail its schedule"
)]
impl<T> Link<T> {
    fn new(value: Option<Arc<T>>) -> Self {
        Self(Mutex::new(value))
    }
    fn load(&self) -> Option<Arc<T>> {
        self.0.lock().unwrap().clone()
    }
    fn is_none_or(&self, predicate: impl FnOnce(&T) -> bool) -> bool {
        self.0.lock().unwrap().as_deref().is_none_or(predicate)
    }
    fn swap(&self, value: Option<Arc<T>>) -> Option<Arc<T>> {
        std::mem::replace(&mut *self.0.lock().unwrap(), value)
    }
    fn compare(&self, expected: Option<&Arc<T>>, value: Option<Arc<T>>) -> bool {
        let mut slot = self.0.lock().unwrap();
        if same(slot.as_ref(), expected) {
            let old = std::mem::replace(&mut *slot, value);
            drop(slot);
            drop(old);
            true
        } else {
            false
        }
    }
}
fn same<T>(left: Option<&Arc<T>>, right: Option<&Arc<T>>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => Arc::ptr_eq(left, right),
        (None, None) => true,
        _ => false,
    }
}
// Loom lacks Arc::into_inner. This models ownership, not its bounded-stack
// destruction guarantee; the native long-chain fixture checks that separately.
fn take_owned<T>(value: Arc<T>) -> Option<T> {
    Arc::try_unwrap(value).ok()
}

fn model(action: impl Fn() + Send + Sync + 'static) {
    let mut builder = loom::model::Builder::new();
    builder.preemption_bound = Some(2);
    builder.max_permutations = Some(10_000);
    builder.max_branches = 10_000;
    builder.check(action);
}

#[test]
fn shared_prepared_record_is_inserted_once_by_racing_helpers() {
    model(|| {
        let queue = Arc::new(Queue::new());
        let prepared = Arc::new(queue.prepare(42));
        let helper = {
            let (queue, prepared) = (queue.clone(), prepared.clone());
            thread::spawn(move || while !queue.help(&prepared) {})
        };
        while !queue.help(&prepared) {}
        let first = queue.pop();
        helper.join().unwrap();
        assert!(queue.help(&prepared));
        assert_eq!(first, Some(42));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.pending(), 0);
        drop(prepared);
        assert_eq!(queue.retained_nodes(), 2);
    });
}

#[test]
fn consumption_and_late_helpers_preserve_single_insertion() {
    model(|| {
        let queue = Arc::new(Queue::new());
        let prepared = Arc::new(queue.prepare(42));
        let helper = {
            let (queue, prepared) = (queue.clone(), prepared.clone());
            thread::spawn(move || {
                queue.test_help(&prepared, thread::yield_now, thread::yield_now);
            })
        };
        // Another intent changes the shared record's possible predecessor and
        // lets the consumer tombstone it before the original helper returns.
        queue.push(7);
        let mut found: Vec<_> = queue.pop().into_iter().collect();
        while !queue.help(&prepared) {}
        while let Some(value) = queue.pop() {
            found.push(value);
        }
        helper.join().unwrap();
        assert!(queue.help(&prepared));
        assert_eq!(queue.pop(), None);
        found.sort_unstable();
        assert_eq!(found, [7, 42]);
        assert_eq!(queue.pending(), 0);
        drop(prepared);
        assert_eq!(queue.retained_nodes(), 2);
    });
}

#[test]
fn private_preparation_can_cancel_after_phase_closure() {
    model(|| {
        let queue = Arc::new(Queue::new());
        let clock = Clock::new(64 * REVISION);
        let prepared = queue.prepare(42);
        queue.push(7);
        assert_eq!(queue.pop(), Some(7));
        assert!(!queue.help(&prepared));
        let cancellation = thread::spawn(move || drop(prepared));
        assert!(queue.is_empty());
        assert!(clock.transition(clock.observe().unwrap(), 0, 1));
        cancellation.join().unwrap();
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    });
}

#[test]
fn publishers_preserve_each_stream_and_consume_each_item_once() {
    model(|| {
        let queue = Arc::new(Queue::new());
        let first = {
            let queue = queue.clone();
            thread::spawn(move || {
                queue.push(1);
                queue.push(2);
            })
        };
        let second = {
            let queue = queue.clone();
            thread::spawn(move || {
                queue.push(3);
                queue.push(4);
            })
        };
        let mut found: Vec<_> = queue.pop().into_iter().collect();
        first.join().unwrap();
        second.join().unwrap();
        while let Some(value) = queue.pop() {
            found.push(value);
        }
        assert_eq!(found.len(), 4);
        assert!(found.iter().position(|&n| n == 1) < found.iter().position(|&n| n == 2));
        assert!(found.iter().position(|&n| n == 3) < found.iter().position(|&n| n == 4));
        found.sort_unstable();
        assert_eq!(found, [1, 2, 3, 4]);
        assert!(queue.is_empty());
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    });
}

#[test]
fn racing_consumers_claim_distinct_items() {
    model(|| {
        let queue = Arc::new(Queue::new());
        queue.push(1);
        queue.push(2);
        let first = {
            let queue = queue.clone();
            thread::spawn(move || queue.pop())
        };
        let second = {
            let queue = queue.clone();
            thread::spawn(move || queue.pop())
        };
        let mut found = [first.join().unwrap(), second.join().unwrap()];
        found.sort_unstable();
        assert_eq!(found, [Some(1), Some(2)]);
        assert!(queue.is_empty());
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    });
}

#[test]
fn delayed_publication_cannot_reuse_a_detached_empty_link() {
    model(|| {
        let queue = Arc::new(Queue::new());
        let first = {
            let queue = queue.clone();
            thread::spawn(move || queue.test_push_before_link(1, thread::yield_now))
        };
        queue.push(2);
        let mut found: Vec<_> = queue.pop().into_iter().collect();
        first.join().unwrap();
        while let Some(value) = queue.pop() {
            found.push(value);
        }
        found.sort_unstable();
        assert_eq!(found, [1, 2]);
        assert!(queue.is_empty());
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    });
}

#[test]
fn zero_pending_cannot_close_across_an_admitted_publication() {
    model(|| {
        let queue = Arc::new(Queue::new());
        let clock = Arc::new(Clock::new(64 * REVISION));
        assert!(clock.transition(clock.observe().unwrap(), 0, 1));
        let published = Arc::new(AtomicBool::new(false));
        let producer = {
            let (queue, clock, published) = (queue.clone(), clock.clone(), published.clone());
            thread::spawn(move || {
                let _admission = clock.enter();
                if clock.flags(0) & 1 != 0 {
                    queue.test_push_after_link(42, thread::yield_now);
                    published.store(true, Ordering::Release);
                }
            })
        };
        let closed = clock.observe().is_some_and(|observed| {
            queue.pending() == 0 && queue.is_empty() && clock.transition(observed, 0, 0)
        });
        producer.join().unwrap();
        assert_eq!(closed, !published.load(Ordering::Acquire));
        assert_eq!(queue.pop(), if closed { None } else { Some(42) });
        assert_eq!(queue.pending(), 0);
    });
}

#[test]
fn a_popped_unreserved_claim_still_excludes_closure() {
    model(|| {
        let queue = Arc::new(Queue::new());
        queue.push(42);
        let claims = Arc::new(RwLock::new(()));
        let in_flight = Arc::new(AtomicUsize::new(0));
        let processed = Arc::new(AtomicBool::new(false));
        let consumer = {
            let (queue, claims, in_flight, processed) = (
                queue.clone(),
                claims.clone(),
                in_flight.clone(),
                processed.clone(),
            );
            thread::spawn(move || {
                let claim = claims.read().unwrap();
                assert_eq!(queue.pop(), Some(42));
                thread::yield_now();
                in_flight.fetch_add(1, Ordering::AcqRel);
                drop(claim);
                processed.store(true, Ordering::Release);
                in_flight.fetch_sub(1, Ordering::AcqRel);
            })
        };
        if let Ok(_closure) = claims.try_write()
            && queue.pending() == 0
            && in_flight.load(Ordering::Acquire) == 0
        {
            assert!(processed.load(Ordering::Acquire));
        }
        consumer.join().unwrap();
        assert!(queue.is_empty());
        assert_eq!(queue.pending(), 0);
    });
}
