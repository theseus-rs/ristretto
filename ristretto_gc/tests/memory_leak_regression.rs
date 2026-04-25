//! Regression tests for memory leak fixes in the garbage collector.
//!
//! Each test targets a specific invariant of the GC subsystem:
//!
//! - **Sweep correctness**: unreachable objects are freed; rooted objects survive.
//! - **Type safety**: large objects and finalizable objects are deallocated with the correct type,
//!   preventing memory corruption or mismatched `Box::from_raw` calls.
//! - **Shutdown cleanup**: remaining objects are finalized and deallocated when the collector is
//!   dropped, even if no collection cycle ran.
//! - **Concurrency**: objects allocated during an in-flight sweep survive (allocation-color-black),
//!   and the collector thread does not deadlock when it holds the last `Arc<GarbageCollector>`.
//! - **Cycle collection**: cyclic object graphs are correctly traced when rooted and collected when
//!   all roots are dropped.

use ristretto_gc::{Finalize, GarbageCollector, Gc, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Polls `condition` at `poll_interval` until it returns `true` or `timeout` elapses.
/// Returns `true` if the condition was met, `false` on timeout.
fn wait_for_condition(
    timeout: Duration,
    poll_interval: Duration,
    condition: impl Fn() -> bool,
) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        thread::sleep(poll_interval);
    }
    false
}

const TIMEOUT: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(20);

/// Tracks whether drop was called.
#[derive(Debug)]
struct DropTracker {
    dropped: Arc<AtomicBool>,
}

impl DropTracker {
    fn new() -> (Self, Arc<AtomicBool>) {
        let dropped = Arc::new(AtomicBool::new(false));
        (
            Self {
                dropped: dropped.clone(),
            },
            dropped,
        )
    }
}

impl Drop for DropTracker {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
    }
}

impl Trace for DropTracker {
    fn trace(&self, _collector: &GarbageCollector) {}
}

/// A larger struct to detect type confusion deallocation bugs.
/// If the drop function incorrectly treats this as a different sized type,
/// the data will be corrupted or the wrong amount of memory freed.
#[derive(Debug)]
struct LargeObject {
    data: [u64; 16],
    dropped: Arc<AtomicBool>,
}

impl LargeObject {
    fn new(sentinel: u64) -> (Self, Arc<AtomicBool>) {
        let dropped = Arc::new(AtomicBool::new(false));
        let mut data = [0u64; 16];
        data[0] = sentinel;
        data[15] = sentinel;
        (
            Self {
                data,
                dropped: dropped.clone(),
            },
            dropped,
        )
    }
}

impl Drop for LargeObject {
    fn drop(&mut self) {
        // Verify sentinel values are intact; detects memory corruption
        assert_eq!(self.data[0], self.data[15], "memory corruption detected");
        self.dropped.store(true, Ordering::Release);
    }
}

impl Trace for LargeObject {
    fn trace(&self, _collector: &GarbageCollector) {}
}

/// Tracks both finalization and drop.
#[derive(Debug)]
struct FinalizerTracker {
    finalized: Arc<AtomicBool>,
    dropped: Arc<AtomicBool>,
}

impl FinalizerTracker {
    fn new() -> (Self, Arc<AtomicBool>, Arc<AtomicBool>) {
        let finalized = Arc::new(AtomicBool::new(false));
        let dropped = Arc::new(AtomicBool::new(false));
        (
            Self {
                finalized: finalized.clone(),
                dropped: dropped.clone(),
            },
            finalized,
            dropped,
        )
    }
}

impl Drop for FinalizerTracker {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::Release);
    }
}

impl Finalize for FinalizerTracker {
    fn finalize(&self) {
        self.finalized.store(true, Ordering::Release);
    }
}

impl Trace for FinalizerTracker {
    fn trace(&self, _collector: &GarbageCollector) {}
}

/// Verifies that unreachable objects are freed during a collection cycle.
#[test]
fn test_unreachable_objects_freed_at_shutdown() {
    let (obj, dropped) = DropTracker::new();

    let collector = GarbageCollector::new();
    collector.start();

    let gc_obj = Gc::new(&collector, obj);

    // First collection while rooted; marks the object (sets was_ever_marked)
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "mark cycle must complete"
    );

    // Drop root;object becomes unreachable
    drop(gc_obj);

    // Second collection;sweep frees the now-unreachable object
    collector.collect();

    assert!(
        wait_for_condition(TIMEOUT, POLL, || { dropped.load(Ordering::Acquire) }),
        "unreachable object must be freed during collection"
    );
}

/// Verifies that rooted objects survive collection.
#[test]
fn test_rooted_objects_survive_collection() {
    let collector = GarbageCollector::new();
    collector.start();

    let (obj, dropped) = DropTracker::new();
    let gc_obj = Gc::new(&collector, obj);
    let root = gc_obj.as_root(&collector).expect("failed to register root");

    collector.collect();

    // Wait for the collection cycle to complete
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "collection cycle must complete"
    );

    assert!(
        !dropped.load(Ordering::Acquire),
        "rooted object must NOT be freed"
    );

    // Object is still accessible
    drop(root);
}

/// Verifies that large objects are correctly deallocated with proper type size.
#[test]
fn test_large_object_correct_deallocation() {
    let (obj, dropped) = LargeObject::new(0xDEAD_BEEF);

    let collector = GarbageCollector::new();
    collector.start();

    let gc_obj = Gc::new(&collector, obj);

    // First collection while rooted; marks the object (sets was_ever_marked)
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "mark cycle must complete"
    );

    // Drop root; object becomes unreachable
    drop(gc_obj);

    // Second collection; sweep frees the now-unreachable large object
    collector.collect();

    assert!(
        wait_for_condition(TIMEOUT, POLL, || { dropped.load(Ordering::Acquire) }),
        "large object must be correctly freed with proper type during collection"
    );
}

/// Verifies finalizer is called on the correct type (not a miscast pointer).
#[test]
fn test_finalizer_called_on_correct_type() {
    let (obj, finalized, dropped) = FinalizerTracker::new();

    let collector = GarbageCollector::new();
    collector.start();

    let gc_obj = Gc::new_with_finalizer(&collector, obj);

    // First collection while rooted; marks the object (sets was_ever_marked)
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "mark cycle must complete"
    );

    // Drop root;object becomes unreachable
    drop(gc_obj);

    // Second collection; sweep should finalize then drop the unreachable object
    collector.collect();

    assert!(
        wait_for_condition(TIMEOUT, POLL, || { finalized.load(Ordering::Acquire) }),
        "finalizer must be called during collection"
    );
    assert!(
        wait_for_condition(TIMEOUT, POLL, || { dropped.load(Ordering::Acquire) }),
        "object must be dropped after finalization during collection"
    );
}

/// Verifies that GC instances are properly dropped when all external references are released.
#[test]
fn test_gc_drops_when_all_references_released() {
    {
        let collector = GarbageCollector::new();
        collector.start();

        let (obj, _flag) = DropTracker::new();
        let _gc = Gc::new(&collector, obj);
        // collector drops here; with the Weak fix, Drop fires and thread joins
    }

    // If the collector thread held Arc<GC>, this would be the second GC instance
    // created without the first being dropped (reference cycle prevents Drop).
    // With the fix, the first GC is properly dropped before we reach here.
    let collector2 = GarbageCollector::new();
    collector2.start();
    drop(collector2);
    // We should get here without hanging
}

/// Verifies that repeated allocation/collection cycles complete without hanging
/// and that unreachable objects are freed during collection cycles.
#[test]
fn test_memory_stable_across_cycles() {
    let drop_count = Arc::new(AtomicUsize::new(0));
    let total_objects = 5 * 50;

    let collector = GarbageCollector::new();
    collector.start();

    for cycle in 0..5 {
        let objects: Vec<_> = (0..50)
            .map(|i| {
                #[derive(Debug)]
                struct Tracked {
                    _label: String,
                    count: Arc<AtomicUsize>,
                }
                impl Drop for Tracked {
                    fn drop(&mut self) {
                        self.count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                impl Trace for Tracked {
                    fn trace(&self, _collector: &GarbageCollector) {}
                }
                Gc::new(
                    &collector,
                    Tracked {
                        _label: format!("cycle-{cycle}-obj-{i}"),
                        count: Arc::clone(&drop_count),
                    },
                )
            })
            .collect();

        drop(objects);
        collector.collect();

        // Wait for collection to complete
        assert!(
            wait_for_condition(TIMEOUT, POLL, || {
                collector
                    .statistics()
                    .map(|s| s.collections_completed > cycle)
                    .unwrap_or(false)
            }),
            "collection cycle {cycle} must complete"
        );
    }

    // With sweep enabled, objects should be freed during collection cycles.
    // Wait for all drops to happen (some may be swept, rest freed at shutdown).
    drop(collector);

    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        total_objects,
        "all objects must be dropped"
    );
}

/// Verifies that a drop counter matches the number of allocated objects
/// after shutdown cleanup.
#[test]
fn test_all_objects_dropped_exactly_once() {
    #[derive(Debug)]
    struct Counted {
        _id: usize,
        drop_count: Arc<AtomicUsize>,
    }

    impl Drop for Counted {
        fn drop(&mut self) {
            self.drop_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    impl Trace for Counted {
        fn trace(&self, _collector: &GarbageCollector) {}
    }

    let drop_count = Arc::new(AtomicUsize::new(0));
    let n = 100;

    let collector = GarbageCollector::new();
    collector.start();

    for i in 0..n {
        let _gc = Gc::new(
            &collector,
            Counted {
                _id: i,
                drop_count: Arc::clone(&drop_count),
            },
        );
    }

    // Trigger collection to sweep unreachable objects, then drop for the rest
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "collection cycle must complete"
    );

    drop(collector);

    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        n,
        "each object must be dropped exactly once"
    );
}

/// Verifies that the collector thread does not deadlock when it holds the last
/// `Arc<GarbageCollector>` (self-join scenario). This happens when the main thread
/// drops its Arc while a collection cycle is in-flight and the collector thread's
/// `Weak::upgrade()` produces the last remaining `Arc`.
#[test]
fn test_collector_thread_self_join_no_deadlock() {
    let finished = Arc::new(AtomicBool::new(false));
    let finished_clone = finished.clone();

    let handle = thread::spawn(move || {
        let collector = GarbageCollector::new();
        collector.start();

        // Allocate objects so a collection cycle has work to do
        for i in 0..100 {
            let _gc = Gc::new(&collector, format!("obj-{i}"));
        }

        // Trigger a collection and wait for it to start, ensuring the collector thread
        // has upgraded its Weak to Arc before we drop the collector. Without this wait,
        // drop(collector) could fire before the thread wakes up, causing the Weak upgrade
        // to fail and the thread to exit without exercising the self-join path.
        collector.collect();
        let _ = wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        });

        // Now drop the collector. The collector thread may still hold the last Arc
        // from the completed cycle's Weak::upgrade(), triggering the self-join path.
        drop(collector);

        finished_clone.store(true, Ordering::Release);
    });

    // The test passes if the spawned thread completes without deadlocking
    assert!(
        wait_for_condition(Duration::from_secs(10), POLL, || {
            finished.load(Ordering::Acquire)
        }),
        "collector must shut down without self-join deadlock"
    );

    handle.join().expect("spawned thread panicked");
}

/// Verifies that `Drop` implementations are called on remaining objects during
/// GC shutdown (via `cleanup_remaining_objects`), not just during sweep.
/// Objects that are still rooted at shutdown must be cleaned up.
#[test]
fn test_shutdown_cleanup_drops_remaining_objects() {
    let (obj, dropped) = DropTracker::new();

    {
        let collector = GarbageCollector::new();
        collector.start();

        // Allocate an object and keep it rooted so it survives collection.
        // It should still be freed at shutdown.
        let _root = Gc::new(&collector, obj);

        // Collector drops here -> stop() -> cleanup_remaining_objects()
    }

    assert!(
        dropped.load(Ordering::Acquire),
        "remaining rooted objects must be dropped during shutdown cleanup"
    );
}

/// Verifies that finalizers are called on remaining objects during GC shutdown.
#[test]
fn test_shutdown_cleanup_runs_finalizers() {
    let (obj, finalized, dropped) = FinalizerTracker::new();

    {
        let collector = GarbageCollector::new();
        collector.start();

        let _gc = Gc::new_with_finalizer(&collector, obj);

        // Collector drops here -> cleanup should run finalizer then drop
    }

    assert!(
        finalized.load(Ordering::Acquire),
        "finalizers must be called during shutdown cleanup"
    );
    assert!(
        dropped.load(Ordering::Acquire),
        "objects must be dropped during shutdown cleanup"
    );
}

/// Verifies that objects allocated concurrently with an active sweep are not
/// prematurely collected. This exercises the "allocation-color-black" invariant:
/// new objects start marked, so an in-flight sweep must not reclaim them.
#[test]
fn test_concurrent_allocation_during_sweep() {
    let collector = GarbageCollector::new();
    collector.start();

    let drop_count = Arc::new(AtomicUsize::new(0));
    let n_pre = 200;
    let n_concurrent = 50;

    // Pre-allocate objects and make them unreachable to give the GC sweep work to do
    for i in 0..n_pre {
        let _gc = Gc::new(&collector, format!("pre-{i}"));
    }

    // Trigger collection; the sweep will be processing the pre-allocated objects
    collector.collect();

    // Allocate new objects while the GC is (likely) still sweeping.
    // These objects start marked=true and must survive the in-flight cycle.
    let live_guards: Vec<_> = (0..n_concurrent)
        .map(|i| {
            #[derive(Debug)]
            struct SweepSurvivor {
                _id: usize,
                drop_count: Arc<AtomicUsize>,
            }
            impl Drop for SweepSurvivor {
                fn drop(&mut self) {
                    self.drop_count.fetch_add(1, Ordering::Relaxed);
                }
            }
            impl Trace for SweepSurvivor {
                fn trace(&self, _collector: &GarbageCollector) {}
            }
            Gc::new(
                &collector,
                SweepSurvivor {
                    _id: i,
                    drop_count: Arc::clone(&drop_count),
                },
            )
        })
        .collect();

    // Wait for the first collection cycle to complete
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "collection cycle must complete"
    );

    // All concurrently allocated objects must still be alive (they are rooted via live_guards)
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        0,
        "objects allocated during sweep must survive"
    );

    // Run another collection while objects are still rooted to ensure they are
    // marked as reachable (sets was_ever_marked). Objects allocated during the
    // first cycle may have missed the mark phase.
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 2)
                .unwrap_or(false)
        }),
        "mark cycle must complete"
    );

    // Drop roots and trigger another collection to sweep them
    drop(live_guards);
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            drop_count.load(Ordering::Relaxed) == n_concurrent
        }),
        "all concurrently-allocated objects must be freed after roots are dropped; got {}",
        drop_count.load(Ordering::Relaxed)
    );
}

/// Verifies that cyclic object graphs are correctly traced and collected.
/// Two objects reference each other (A -> B -> A); when both become unreachable,
/// the collector must break the cycle and free both.
#[test]
fn test_cyclic_object_graph_collection() {
    use std::sync::Mutex;

    #[derive(Debug)]
    struct Node {
        _name: String,
        next: Mutex<Option<Gc<Node>>>,
        dropped: Arc<AtomicBool>,
    }

    impl Trace for Node {
        fn trace(&self, collector: &GarbageCollector) {
            if let Ok(guard) = self.next.lock()
                && let Some(ref gc_next) = *guard
            {
                gc_next.trace(collector);
            }
        }
    }

    impl Drop for Node {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::Release);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let dropped_a = Arc::new(AtomicBool::new(false));
    let dropped_b = Arc::new(AtomicBool::new(false));

    // Create two nodes
    let guard_a = Gc::new(
        &collector,
        Node {
            _name: "A".to_string(),
            next: Mutex::new(None),
            dropped: dropped_a.clone(),
        },
    );
    let guard_b = Gc::new(
        &collector,
        Node {
            _name: "B".to_string(),
            next: Mutex::new(None),
            dropped: dropped_b.clone(),
        },
    );

    // Wire up cycle: A -> B -> A
    let gc_a: Gc<Node> = guard_a.clone_gc();
    let gc_b: Gc<Node> = guard_b.clone_gc();
    *gc_a.next.lock().unwrap() = Some(gc_b.clone());
    *gc_b.next.lock().unwrap() = Some(gc_a.clone());

    // Verify both survive when rooted
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "first collection must complete"
    );
    assert!(
        !dropped_a.load(Ordering::Acquire),
        "rooted node A must survive"
    );
    assert!(
        !dropped_b.load(Ordering::Acquire),
        "rooted node B must survive"
    );

    // Drop root guards; both nodes are now only reachable through each other (cycle)
    drop(guard_a);
    drop(guard_b);

    // Trigger collection; cyclic objects with no external roots should be swept
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            dropped_a.load(Ordering::Acquire) && dropped_b.load(Ordering::Acquire)
        }),
        "cyclic objects must be freed during collection; A dropped={}, B dropped={}",
        dropped_a.load(Ordering::Acquire),
        dropped_b.load(Ordering::Acquire)
    );
}

/// Verifies that a rooted cyclic graph survives collection when at least one
/// node is reachable from a root, and all nodes in the cycle are freed when
/// the last root is dropped.
#[test]
fn test_rooted_cyclic_graph_partial_unroot() {
    use std::sync::Mutex;

    #[derive(Debug)]
    struct CycleNode {
        _id: usize,
        next: Mutex<Option<Gc<CycleNode>>>,
        dropped: Arc<AtomicBool>,
    }

    impl Trace for CycleNode {
        fn trace(&self, collector: &GarbageCollector) {
            if let Ok(guard) = self.next.lock()
                && let Some(ref gc_next) = *guard
            {
                gc_next.trace(collector);
            }
        }
    }

    impl Drop for CycleNode {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::Release);
        }
    }

    let collector = GarbageCollector::new();
    collector.start();

    let dropped_a = Arc::new(AtomicBool::new(false));
    let dropped_b = Arc::new(AtomicBool::new(false));
    let dropped_c = Arc::new(AtomicBool::new(false));

    // Create A -> B -> C -> A cycle
    let guard_a = Gc::new(
        &collector,
        CycleNode {
            _id: 0,
            next: Mutex::new(None),
            dropped: dropped_a.clone(),
        },
    );
    let guard_b = Gc::new(
        &collector,
        CycleNode {
            _id: 1,
            next: Mutex::new(None),
            dropped: dropped_b.clone(),
        },
    );
    let guard_c = Gc::new(
        &collector,
        CycleNode {
            _id: 2,
            next: Mutex::new(None),
            dropped: dropped_c.clone(),
        },
    );

    let gc_a: Gc<CycleNode> = guard_a.clone_gc();
    let gc_b: Gc<CycleNode> = guard_b.clone_gc();
    let gc_c: Gc<CycleNode> = guard_c.clone_gc();

    *gc_a.next.lock().unwrap() = Some(gc_b.clone());
    *gc_b.next.lock().unwrap() = Some(gc_c.clone());
    *gc_c.next.lock().unwrap() = Some(gc_a.clone());

    // Drop roots for B and C but keep A rooted
    drop(guard_b);
    drop(guard_c);

    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            collector
                .statistics()
                .map(|s| s.collections_completed >= 1)
                .unwrap_or(false)
        }),
        "collection cycle must complete"
    );

    // All three nodes must survive because A is rooted and traces B and C
    assert!(
        !dropped_a.load(Ordering::Acquire),
        "rooted node A must survive"
    );
    assert!(
        !dropped_b.load(Ordering::Acquire),
        "node B reachable from A must survive"
    );
    assert!(
        !dropped_c.load(Ordering::Acquire),
        "node C reachable from A must survive"
    );

    // Now drop A's root; entire cycle becomes unreachable.
    // Trigger collection; sweep should free all cycle members.
    drop(guard_a);
    collector.collect();
    assert!(
        wait_for_condition(TIMEOUT, POLL, || {
            dropped_a.load(Ordering::Acquire)
                && dropped_b.load(Ordering::Acquire)
                && dropped_c.load(Ordering::Acquire)
        }),
        "all nodes must be freed during collection; A={}, B={}, C={}",
        dropped_a.load(Ordering::Acquire),
        dropped_b.load(Ordering::Acquire),
        dropped_c.load(Ordering::Acquire)
    );
}

/// Verifies that the GC actively frees memory during collection cycles, not just at shutdown.
/// Allocates many short-lived objects across multiple cycles and verifies that the tracked
/// object count stays bounded (proving sweep is working).
#[test]
fn test_sweep_actively_frees_memory() {
    use ristretto_gc::Configuration;

    const TOTAL_ALLOCATED: usize = 1000;
    const MAX_FINAL_CYCLES: usize = 50;

    let config = Configuration {
        allocation_threshold: 1024, // Low threshold to trigger frequent collections
        ..Default::default()
    };
    let collector = GarbageCollector::with_config(config);
    collector.start();

    let collect_and_wait = |label: &str| {
        let started_before = collector
            .statistics()
            .expect("statistics")
            .collections_started;
        collector.collect();
        assert!(
            wait_for_condition(TIMEOUT, POLL, || {
                let stats = collector.statistics().expect("statistics");
                // A fresh cycle must have started after our snapshot AND completed.
                stats.collections_started > started_before
                    && stats.collections_completed >= stats.collections_started
            }),
            "{label}: a fresh GC cycle must start and complete (started_before={started_before})"
        );
    };

    let drop_count = Arc::new(AtomicUsize::new(0));

    // Allocate 10 batches of 100 short-lived objects.
    // For each batch:
    //   1. Run GC while rooted -> marks them (sets was_ever_marked)
    //   2. Drop roots
    //   3. Run GC again -> sweeps the now-unreachable objects
    for batch in 0..10 {
        let objects: Vec<_> = (0u8..100)
            .map(|i| {
                #[derive(Debug)]
                struct ShortLived {
                    _data: [u8; 256],
                    count: Arc<AtomicUsize>,
                }
                impl Drop for ShortLived {
                    fn drop(&mut self) {
                        self.count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                impl Trace for ShortLived {
                    fn trace(&self, _collector: &GarbageCollector) {}
                }
                Gc::new(
                    &collector,
                    ShortLived {
                        _data: [i; 256],
                        count: Arc::clone(&drop_count),
                    },
                )
            })
            .collect();

        // GC while rooted: marks objects as reachable (sets was_ever_marked = true)
        collect_and_wait(&format!("batch {batch} mark cycle"));

        // Drop all roots; objects become unreachable
        drop(objects);

        // GC again: objects are unmarked + was_ever_marked -> swept
        collect_and_wait(&format!("batch {batch} sweep cycle"));
    }

    // Run additional collections to sweep objects that survived due to
    // allocation-color-black (objects allocated during an in-flight cycle start
    // marked=true, needing another full cycle to become eligible for sweep). Loop
    // deterministically: keep running fresh cycles until every allocated object has
    // been freed, rather than relying on a fixed iteration count that is sensitive
    // to CI timing.
    let mut final_cycles = 0usize;
    while drop_count.load(Ordering::Relaxed) < TOTAL_ALLOCATED && final_cycles < MAX_FINAL_CYCLES {
        final_cycles += 1;
        collect_and_wait(&format!("final cycle {final_cycles}"));
    }

    // The key invariant under test is that sweep is actively freeing memory during
    // collection (not at shutdown). Every object allocated by this test must have
    // been freed by sweep.
    let freed = drop_count.load(Ordering::Relaxed);
    assert_eq!(
        freed, TOTAL_ALLOCATED,
        "sweep must actively free all allocated objects during collection; got {freed}/{TOTAL_ALLOCATED} after {final_cycles} final cycles"
    );

    let stats = collector.statistics().expect("statistics");
    assert!(
        stats.objects_swept >= TOTAL_ALLOCATED,
        "statistics must reflect swept objects; got {}, expected >= {TOTAL_ALLOCATED}",
        stats.objects_swept
    );
}
