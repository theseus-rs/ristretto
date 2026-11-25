//! Production identity-list transitions with linearizable owning links.
//! `ArcSwap` internals and allocator behavior are outside this bounded model.
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::sync::{Arc, Mutex};
use loom::thread;
#[path = "../src/identity_index_state.rs"]
mod state;
use state::{Cleanup, Index, Removal};

use state::Edge;
struct Link(Mutex<Edge>);
struct OwnedLink<T>(Mutex<Option<Arc<T>>>);
#[expect(clippy::unwrap_used, reason = "poisoned model links fail the schedule")]
impl<T> OwnedLink<T> {
    fn new(value: Option<Arc<T>>) -> Self {
        Self(Mutex::new(value))
    }
    fn load(&self) -> Option<Arc<T>> {
        self.0.lock().unwrap().clone()
    }
    fn swap(&self, value: Option<Arc<T>>) -> Option<Arc<T>> {
        std::mem::replace(&mut *self.0.lock().unwrap(), value)
    }
    fn compare(&self, expected: &Arc<T>, value: Arc<T>) -> bool {
        let mut slot = self.0.lock().unwrap();
        if slot.as_ref().is_some_and(|old| Arc::ptr_eq(old, expected)) {
            let old = slot.replace(value);
            drop(slot);
            drop(old);
            true
        } else {
            false
        }
    }
}
#[expect(clippy::unwrap_used, reason = "poisoned model links fail the schedule")]
impl Link {
    fn new(value: Edge) -> Self {
        Self(Mutex::new(value))
    }
    fn load(&self) -> Edge {
        self.0.lock().unwrap().clone()
    }
    fn swap(&self, value: Edge) -> Edge {
        std::mem::replace(&mut *self.0.lock().unwrap(), value)
    }
    fn compare(&self, expected: &Edge, value: Edge) -> bool {
        let mut slot = self.0.lock().unwrap();
        if slot.same(expected) {
            let old = std::mem::replace(&mut *slot, value);
            drop(slot);
            drop(old);
            true
        } else {
            false
        }
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

fn insert(index: &Index, key: u64) {
    index.insert(index.prepare(key, 100));
}
fn remove(index: &Index, key: u64) -> bool {
    let mut cursor = Removal::default();
    loop {
        if let Some(removed) = index.remove_step(key, 100, &mut cursor) {
            return removed;
        }
    }
}
fn clean(index: &Index) {
    let mut cursor = Cleanup::default();
    while !index.cleanup_step(&mut cursor) {}
}

#[test]
fn adjacent_deletions_cannot_unlink_through_a_detached_predecessor() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        insert(&index, 2);
        insert(&index, 3);
        let other = index.clone();
        let worker = thread::spawn(move || assert!(remove(&other, 2)));
        assert!(remove(&index, 1));
        worker.join().unwrap();
        assert_eq!(index.get(3), Some(100));
        assert_eq!(index.get(2), None);
        assert_eq!(index.get(1), None);
        clean(&index);
        assert_eq!(index.retained_nodes(), 1);
    });
}

#[test]
fn racing_removers_have_exactly_one_logical_winner() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        let other = index.clone();
        let worker = thread::spawn(move || remove(&other, 1));
        let first = remove(&index, 1);
        assert_ne!(first, worker.join().unwrap());
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    });
}

#[test]
fn lookup_racing_deletion_and_address_reuse_preserves_identity() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        let other = index.clone();
        let worker = thread::spawn(move || {
            assert!(remove(&other, 1));
            insert(&other, 2);
        });
        assert!(matches!(index.get(1), None | Some(100)));
        worker.join().unwrap();
        assert_eq!(index.get(1), None);
        assert_eq!(index.get(2), Some(100));
        assert!(!remove(&index, 1));
        clean(&index);
        assert_eq!(index.retained_nodes(), 1);
    });
}

#[test]
fn concurrent_head_insertions_survive_cleanup() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        let other = index.clone();
        let worker = thread::spawn(move || insert(&other, 2));
        assert!(remove(&index, 1));
        insert(&index, 3);
        clean(&index);
        worker.join().unwrap();
        assert_eq!(index.get(1), None);
        assert_eq!(index.get(2), Some(100));
        assert_eq!(index.get(3), Some(100));
        clean(&index);
        assert_eq!(index.retained_nodes(), 2);
    });
}

#[test]
fn a_late_child_remover_cannot_detach_a_still_reachable_node() {
    model(|| {
        let index = Arc::new(Index::new(1));
        for key in 1..=3 {
            insert(&index, key);
        }
        let (marked, observed) = loom::sync::mpsc::channel();
        let (release, resume) = loom::sync::mpsc::channel();
        let other = index.clone();
        let worker = thread::spawn(move || {
            let mut cursor = Removal::default();
            loop {
                if let Some(result) = other.test_remove_step(1, 100, &mut cursor, || {
                    marked.send(()).unwrap();
                    resume.recv().unwrap();
                }) {
                    return result;
                }
            }
        });
        observed.recv().unwrap();
        assert!(remove(&index, 2));
        release.send(()).unwrap();
        assert!(worker.join().unwrap());
        assert_eq!(index.get(1), None);
        assert_eq!(index.get(2), None);
        assert_eq!(index.get(3), Some(100));
        clean(&index);
        assert_eq!(index.retained_nodes(), 1);
    });
}

#[test]
fn a_private_insertion_tolerates_a_head_that_returns_to_the_same_node() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        let (captured, observed) = loom::sync::mpsc::channel();
        let (release, resume) = loom::sync::mpsc::channel();
        let other = index.clone();
        let worker = thread::spawn(move || {
            let prepared = other.prepare(3, 100);
            other.test_insert(prepared, || {
                captured.send(()).unwrap();
                resume.recv().unwrap();
            });
        });
        observed.recv().unwrap();
        insert(&index, 2);
        assert!(remove(&index, 2));
        release.send(()).unwrap();
        worker.join().unwrap();
        assert_eq!(index.get(1), Some(100));
        assert_eq!(index.get(2), None);
        assert_eq!(index.get(3), Some(100));
        clean(&index);
        assert_eq!(index.retained_nodes(), 2);
    });
}

#[test]
fn a_deleted_head_can_return_without_republishing_its_successor_snapshot() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        let (marked, observed) = loom::sync::mpsc::channel();
        let (release, resume) = loom::sync::mpsc::channel();
        let other = index.clone();
        let worker = thread::spawn(move || {
            other.test_remove_step(1, 100, &mut Removal::default(), || {
                marked.send(()).unwrap();
                resume.recv().unwrap();
            })
        });
        observed.recv().unwrap();
        insert(&index, 2);
        assert!(remove(&index, 2));
        release.send(()).unwrap();
        assert_eq!(worker.join().unwrap(), Some(true));
        assert_eq!(index.get(1), None);
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    });
}

#[test]
fn competing_helpers_publish_one_shared_identity() {
    model(|| {
        let index = Arc::new(Index::new(1));
        let prepared = Arc::new(index.prepare(1, 100));
        let other = index.clone();
        let record = prepared.clone();
        let worker = thread::spawn(move || while !other.help(&record) {});
        while !index.help(&prepared) {}
        worker.join().unwrap();
        assert!(remove(&index, 1));
        assert!(!remove(&index, 1));
        assert!(index.help(&prepared));
        assert_eq!(index.get(1), None);
        drop(prepared);
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    });
}

#[test]
fn a_stopped_insertion_winner_is_certified_before_head_replacement() {
    model(|| {
        let index = Arc::new(Index::new(1));
        let prepared = Arc::new(index.prepare(1, 100));
        let (linked, observed) = loom::sync::mpsc::channel();
        let (release, resume) = loom::sync::mpsc::channel();
        let other = index.clone();
        let record = prepared.clone();
        let worker = thread::spawn(move || {
            other.test_help(
                &record,
                || {},
                || {
                    linked.send(()).unwrap();
                    resume.recv().unwrap();
                },
            )
        });
        observed.recv().unwrap();
        // Neither operation helps the original intent directly. They must
        // record its successful insertion before replacing its head version.
        insert(&index, 2);
        assert!(remove(&index, 1));
        assert!(remove(&index, 2));
        assert!(index.help(&prepared));
        assert_eq!(index.get(1), None);
        release.send(()).unwrap();
        assert!(worker.join().unwrap());
        drop(prepared);
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    });
}

#[test]
fn a_late_insertion_cas_cannot_replay_after_retirement() {
    model(|| {
        let index = Arc::new(Index::new(1));
        insert(&index, 1);
        let prepared = Arc::new(index.prepare(2, 100));
        let (captured, observed) = loom::sync::mpsc::channel();
        let (release, resume) = loom::sync::mpsc::channel();
        let other = index.clone();
        let record = prepared.clone();
        let worker = thread::spawn(move || {
            other.test_help(
                &record,
                || {
                    captured.send(()).unwrap();
                    resume.recv().unwrap();
                },
                || {},
            )
        });
        observed.recv().unwrap();
        assert!(index.help(&prepared));
        assert!(remove(&index, 2)); // Same node 1 returns with a NEW header.
        release.send(()).unwrap();
        assert!(!worker.join().unwrap());
        assert!(index.help(&prepared));
        assert_eq!(index.get(2), None);
        assert_eq!(index.get(1), Some(100));
        assert!(remove(&index, 1));
        drop(prepared);
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    });
}

#[test]
fn retargeting_a_shared_intent_keeps_each_proposal_immutable() {
    model(|| {
        let index = Arc::new(Index::new(1));
        let prepared = Arc::new(index.prepare(1, 100));
        insert(&index, 2); // The original expected version is permanently gone.
        let other = index.clone();
        let record = prepared.clone();
        let worker = thread::spawn(move || while !other.help(&record) {});
        while !index.help(&prepared) {}
        worker.join().unwrap();
        assert_eq!(index.get(2), Some(100));
        assert!(remove(&index, 1));
        assert!(!remove(&index, 1));
        assert!(remove(&index, 2));
        drop(prepared);
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    });
}
