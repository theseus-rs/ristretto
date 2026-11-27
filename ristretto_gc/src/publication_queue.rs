//! Contention fallback with atomic links and owning, reclaimable nodes.
use arc_swap::ArcSwapOption;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[path = "publication_queue_state.rs"]
mod state;
pub(crate) use state::{Prepared, Queue};

struct Link<T>(ArcSwapOption<T>);

impl<T> Link<T> {
    fn new(value: Option<Arc<T>>) -> Self {
        Self(ArcSwapOption::from(value))
    }
    fn load(&self) -> Option<Arc<T>> {
        self.0.load_full()
    }
    fn is_none_or(&self, predicate: impl FnOnce(&T) -> bool) -> bool {
        self.0.load().as_deref().is_none_or(predicate)
    }
    fn swap(&self, value: Option<Arc<T>>) -> Option<Arc<T>> {
        self.0.swap(value)
    }
    fn compare(&self, expected: Option<&Arc<T>>, value: Option<Arc<T>>) -> bool {
        // The borrowed owner remains live through the CAS, excluding pointer ABA.
        let pointer = expected.map_or(std::ptr::null(), Arc::as_ptr);
        let previous = self.0.compare_and_swap(pointer, value);
        same(previous.as_ref(), expected)
    }
}

fn same<T>(left: Option<&Arc<T>>, right: Option<&Arc<T>>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => Arc::ptr_eq(left, right),
        (None, None) => true,
        _ => false,
    }
}

fn take_owned<T>(value: Arc<T>) -> Option<T> {
    Arc::into_inner(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_preparation_and_cancellation_preserve_queue_and_payload_ownership() {
        let queue = Queue::new();
        let payload = Arc::new(());
        let prepared = queue.prepare(payload.clone());
        assert_eq!(queue.pending(), 1);
        assert_eq!(queue.retained_nodes(), 3);
        assert!(queue.is_empty());
        assert_eq!(Arc::strong_count(&payload), 2);
        queue.push(Arc::new(()));
        assert!(queue.pop().is_some());
        assert!(!queue.help(&prepared)); // Retargets the now detached predecessor.
        drop(prepared);
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
        assert_eq!(Arc::strong_count(&payload), 1);
        assert_eq!(queue.pop(), None);
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn a_prepared_record_cannot_publish_to_another_queue() {
        let first = Queue::new();
        let second = Queue::new();
        let prepared = first.prepare(42);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            second.help(&prepared);
        }));
        assert!(result.is_err());
        assert!(first.is_empty() && second.is_empty());
        drop(prepared);
        assert_eq!((first.pending(), second.pending()), (0, 0));
    }

    #[test]
    fn consumed_record_cannot_be_republished_and_retains_no_history() {
        let queue = Queue::new();
        let payload = Arc::new(());
        let prepared = queue.prepare(payload.clone());
        while !queue.help(&prepared) {}
        assert!(Arc::ptr_eq(
            &queue.pop().expect("published payload"),
            &payload
        ));
        for _ in 0..1000 {
            queue.push(Arc::new(()));
            assert!(queue.pop().is_some());
            assert!(queue.help(&prepared));
            assert!(queue.is_empty());
        }
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 4); // Two queue nodes plus record/predecessor.
        assert_eq!(Arc::strong_count(&payload), 1);
        drop(prepared);
        assert_eq!(queue.retained_nodes(), 2);
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn unwind_before_or_after_link_preserves_reservation_and_ownership() {
        for after_link in [false, true] {
            let queue = Queue::new();
            let payload = Arc::new(());
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let prepared = queue.prepare(payload.clone());
                queue.test_help(
                    &prepared,
                    || {
                        assert!(after_link, "unwind before link");
                    },
                    || {
                        panic!("unwind after link");
                    },
                );
            }));
            assert!(result.is_err());
            assert_eq!(queue.pending(), usize::from(after_link));
            assert_eq!(queue.pop().is_some(), after_link);
            assert_eq!(queue.pending(), 0);
            assert_eq!(queue.retained_nodes(), 2);
            assert_eq!(Arc::strong_count(&payload), 1);
        }
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn shared_record_finishes_while_original_stops_before_or_after_link() {
        use std::sync::mpsc;
        use std::time::Duration;
        for after_link in [false, true] {
            let queue = Arc::new(Queue::new());
            let prepared = Arc::new(queue.prepare(41));
            let (entered, observed) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            let producer = {
                let (queue, prepared) = (queue.clone(), prepared.clone());
                std::thread::spawn(move || {
                    let stall = || {
                        entered.send(()).expect("stopped publisher");
                        resumed
                            .recv_timeout(Duration::from_secs(10))
                            .expect("resume");
                    };
                    if after_link {
                        queue.test_help(&prepared, || {}, stall);
                    } else {
                        queue.test_help(&prepared, stall, || {});
                    }
                })
            };
            observed
                .recv_timeout(Duration::from_secs(10))
                .expect("paused publisher");
            if after_link {
                // Pop without a helping producer first: consumption itself must
                // preserve insertion evidence before removing the predecessor link.
                assert_eq!(queue.pop(), Some(41));
            } else {
                // Invalidate the stopped helper's predecessor. The same record
                // must move to another append location without letting that old
                // helper publish again when it eventually resumes.
                queue.push(7);
                assert_eq!(queue.pop(), Some(7));
                while !queue.help(&prepared) {}
                assert_eq!(queue.pop(), Some(41));
            }
            for item in 0..1000 {
                queue.push(item);
                assert_eq!(queue.pop(), Some(item));
            }
            assert!(queue.help(&prepared));
            assert!(queue.is_empty());
            assert_eq!(queue.pending(), 0);
            assert_eq!(queue.retained_nodes(), if after_link { 4 } else { 5 });
            resume.send(()).expect("resume producer");
            producer.join().expect("late publisher");
            assert!(queue.help(&prepared));
            assert!(queue.is_empty());
            drop(prepared);
            assert_eq!(queue.retained_nodes(), 2);
        }
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn competing_helpers_and_consumers_deliver_each_record_once() {
        use std::time::{Duration, Instant};
        const ITEMS: usize = 2000;
        let queue = Arc::new(Queue::new());
        let records: Arc<Vec<_>> = Arc::new((0..ITEMS).map(|item| queue.prepare(item)).collect());
        let seen: Arc<Vec<_>> = Arc::new((0..ITEMS).map(|_| AtomicUsize::new(0)).collect());
        let done = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        for helper in 0..4 {
            let (queue, records, done) = (queue.clone(), records.clone(), done.clone());
            workers.push(std::thread::spawn(move || {
                let deadline = Instant::now() + Duration::from_secs(20);
                for offset in 0..ITEMS {
                    let record = &records[(offset + helper * (ITEMS / 4)) % ITEMS];
                    while !queue.help(record) {
                        assert!(Instant::now() < deadline, "helping failed to converge");
                    }
                }
                done.fetch_add(1, Ordering::Release);
            }));
        }
        for _ in 0..4 {
            let (queue, seen, done) = (queue.clone(), seen.clone(), done.clone());
            workers.push(std::thread::spawn(move || {
                let deadline = Instant::now() + Duration::from_secs(20);
                loop {
                    if let Some(item) = queue.pop() {
                        assert_eq!(seen[item].fetch_add(1, Ordering::Relaxed), 0);
                    } else if done.load(Ordering::Acquire) == 4 && queue.is_empty() {
                        break;
                    } else {
                        std::thread::yield_now();
                    }
                    assert!(Instant::now() < deadline, "consumption failed to converge");
                }
            }));
        }
        for worker in workers {
            worker.join().expect("helper or consumer");
        }
        assert!(seen.iter().all(|count| count.load(Ordering::Acquire) == 1));
        assert_eq!(queue.pending(), 0);
        assert!(queue.is_empty());
        drop(records);
        assert_eq!(queue.retained_nodes(), 2);
    }

    #[test]
    fn fifo_releases_consumed_payloads_and_returns_to_two_nodes() {
        let queue = Queue::new();
        let values: Vec<_> = (0..16).map(|_| Arc::new(())).collect();
        for value in &values {
            queue.push(value.clone());
        }
        assert_eq!(queue.pending(), 16);
        for value in &values {
            let found = queue.pop().expect("queued value");
            assert!(Arc::ptr_eq(&found, value));
            drop(found);
            assert_eq!(Arc::strong_count(value), 1, "empty head retained payload");
        }
        assert!(queue.is_empty());
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn stalled_linked_producer_does_not_hide_work_or_stop_later_publication() {
        use std::sync::mpsc;
        use std::time::Duration;
        let queue = Arc::new(Queue::new());
        let (linked, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let producer = {
            let queue = queue.clone();
            std::thread::spawn(move || {
                queue.test_push_after_link(41, || {
                    linked.send(()).expect("linked");
                    resumed
                        .recv_timeout(Duration::from_secs(10))
                        .expect("resume");
                });
            })
        };
        observed
            .recv_timeout(Duration::from_secs(10))
            .expect("published link");
        let (finished, result) = mpsc::channel();
        let helper = {
            let queue = queue.clone();
            std::thread::spawn(move || {
                queue.push(42);
                finished
                    .send((queue.pop(), queue.pop(), queue.is_empty()))
                    .expect("helped");
            })
        };
        let completed = result.recv_timeout(Duration::from_secs(1));
        resume.send(()).expect("resume producer");
        producer.join().expect("producer");
        helper.join().expect("helper");
        assert_eq!(completed, Ok((Some(41), Some(42), true)));
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn detached_tail_cannot_accept_a_delayed_publication() {
        use std::sync::mpsc;
        use std::time::Duration;
        let queue = Arc::new(Queue::new());
        let (prepared, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let producer = {
            let queue = queue.clone();
            std::thread::spawn(move || {
                queue.test_push_before_link(42, || {
                    prepared.send(()).expect("tail read");
                    resumed
                        .recv_timeout(Duration::from_secs(10))
                        .expect("resume");
                });
            })
        };
        observed
            .recv_timeout(Duration::from_secs(10))
            .expect("prepared link");
        queue.push(41);
        assert_eq!(queue.pop(), Some(41));
        assert!(queue.is_empty());
        // One reserved, unlinked item; its caller still owns publication protection.
        assert_eq!(queue.pending(), 1);
        resume.send(()).expect("resume producer");
        producer.join().expect("producer");
        assert_eq!(queue.pop(), Some(42));
        assert!(queue.is_empty());
        assert_eq!(queue.retained_nodes(), 2);
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn delayed_head_reader_retains_one_detached_node_without_a_chain() {
        use std::sync::mpsc;
        use std::time::Duration;
        let queue = Arc::new(Queue::<usize>::new());
        let (entered, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let reader = {
            let queue = queue.clone();
            std::thread::spawn(move || {
                queue.test_pop_after_head(|| {
                    entered.send(()).expect("head read");
                    resumed
                        .recv_timeout(Duration::from_secs(10))
                        .expect("resume");
                })
            })
        };
        observed
            .recv_timeout(Duration::from_secs(10))
            .expect("held head");
        for item in 0..1000 {
            queue.push(item);
        }
        for item in 0..1000 {
            assert_eq!(queue.pop(), Some(item));
        }
        let retained = queue.retained_nodes();
        resume.send(()).expect("resume reader");
        assert_eq!(reader.join().expect("reader"), None);
        assert_eq!(retained, 3, "delayed head retained the traversed chain");
        assert_eq!(queue.retained_nodes(), 2);
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn long_queue_destruction_uses_bounded_stack() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let queue = Queue::new();
                let witness = Arc::new(());
                let detached = queue.prepare(None);
                queue.push(None);
                assert_eq!(queue.pop(), Some(None));
                // The second predecessor retains the live suffix after queue teardown.
                let live = queue.prepare(None);
                for _ in 0..100_000 {
                    queue.push(Some(witness.clone()));
                }
                drop(queue);
                drop(detached);
                assert_eq!(Arc::strong_count(&witness), 100_001);
                drop(live);
                assert_eq!(Arc::strong_count(&witness), 1);
            })
            .expect("small-stack thread")
            .join()
            .expect("iterative queue destruction");
    }
    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn multiple_producers_and_consumers_deliver_each_item_once() {
        use std::time::{Duration, Instant};
        const PRODUCERS: usize = 4;
        const ITEMS: usize = 2000;
        let queue = Arc::new(Queue::new());
        let done = Arc::new(AtomicUsize::new(0));
        let seen: Arc<Vec<_>> = Arc::new(
            (0..PRODUCERS * ITEMS)
                .map(|_| AtomicUsize::new(0))
                .collect(),
        );
        let mut workers = Vec::new();
        for producer in 0..PRODUCERS {
            let (queue, done) = (queue.clone(), done.clone());
            workers.push(std::thread::spawn(move || {
                for item in 0..ITEMS {
                    queue.push(producer * ITEMS + item);
                }
                done.fetch_add(1, Ordering::Release);
            }));
        }
        for _ in 0..PRODUCERS {
            let (queue, done, seen) = (queue.clone(), done.clone(), seen.clone());
            workers.push(std::thread::spawn(move || {
                let deadline = Instant::now() + Duration::from_secs(10);
                loop {
                    if let Some(item) = queue.pop() {
                        assert_eq!(seen[item].fetch_add(1, Ordering::Relaxed), 0);
                    } else if done.load(Ordering::Acquire) == PRODUCERS && queue.is_empty() {
                        break;
                    } else {
                        std::thread::yield_now();
                    }
                    assert!(Instant::now() < deadline, "queue stress did not converge");
                }
            }));
        }
        for worker in workers {
            worker.join().expect("concurrent queue worker");
        }
        assert!(seen.iter().all(|item| item.load(Ordering::Acquire) == 1));
        assert!(queue.is_empty());
        assert_eq!(queue.pending(), 0);
        assert_eq!(queue.retained_nodes(), 2);
    }
}
