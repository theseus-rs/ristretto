use super::*;
#[cfg(not(target_family = "wasm"))]
use std::sync::mpsc;
use std::task::Wake;
#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

#[cfg(not(target_family = "wasm"))]
struct Parked {
    entered: mpsc::Sender<()>,
    release: Mutex<mpsc::Receiver<()>>,
}
#[cfg(not(target_family = "wasm"))]
impl Wake for Parked {
    fn wake(self: Arc<Self>) {
        assert!(self.entered.send(()).is_ok(), "test receiver disconnected");
        assert!(
            lock(&self.release)
                .recv_timeout(Duration::from_secs(10))
                .is_ok(),
            "callback release timed out"
        );
    }
}

#[cfg(not(target_family = "wasm"))]
struct Notice(AtomicUsize);
#[cfg(not(target_family = "wasm"))]
impl Wake for Notice {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg(not(target_family = "wasm"))]
thread_local! {
    static BEFORE_DESTINATION_COMMIT: std::cell::Cell<Option<Box<dyn FnOnce()>>> = const { std::cell::Cell::new(None) };
    static AFTER_WORKER_COUNT_DECREMENT: std::cell::Cell<Option<Box<dyn FnOnce()>>> =
        const { std::cell::Cell::new(None) };
    static AFTER_CALLBACK_CLAIM: std::cell::Cell<Option<Box<dyn FnOnce()>>> = const { std::cell::Cell::new(None) };
    static AFTER_BUDGET_SAMPLE: std::cell::Cell<Option<Box<dyn FnOnce()>>> = const { std::cell::Cell::new(None) };
    static AFTER_PHYSICAL_PROBE: std::cell::Cell<Option<Box<dyn FnOnce()>>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn after_callback_claim() {
    if let Some(hook) = AFTER_CALLBACK_CLAIM.take() {
        hook();
    }
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn after_index_budget_sample() {
    if let Some(hook) = AFTER_BUDGET_SAMPLE.take() {
        hook();
    }
}
#[cfg(not(target_family = "wasm"))]
pub(super) fn after_physical_probe() {
    if let Some(hook) = AFTER_PHYSICAL_PROBE.take() {
        hook();
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_paused_physical_probe_leaves_logical_wakes_and_cancellation_available() {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().compact_heap(true).build());
    let logical = Arc::new(Notice(AtomicUsize::new(0)));
    let physical = Arc::new(Notice(AtomicUsize::new(0)));
    gc.recovery
        .register(
            1,
            None,
            Waiter {
                bytes: Some(1),
                waker: Waker::from(logical.clone()),
                space_seen: None,
                grant: None,
            },
            None,
        )
        .unwrap();
    let layout = Layout::new::<[u8; 16]>();
    assert!(gc.region_heap().unwrap().can_allocate(layout));
    let id = gc
        .recovery
        .register(
            1,
            None,
            Waiter {
                bytes: None,
                waker: Waker::from(physical.clone()),
                space_seen: None,
                grant: None,
            },
            Some(layout),
        )
        .unwrap();
    let (entered, parked) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    let (done, delivered) = mpsc::channel();
    std::thread::scope(|scope| {
        let first = gc.clone();
        scope.spawn(move || {
            AFTER_PHYSICAL_PROBE.set(Some(Box::new(move || {
                entered.send(()).unwrap();
                resume.recv_timeout(Duration::from_secs(10)).unwrap();
            })));
            first.recovery.notify_progress(&first);
        });
        parked.recv_timeout(Duration::from_secs(10)).unwrap();
        scope.spawn(|| {
            gc.recovery.remove(1, Some(id));
            gc.recovery.notify_progress(&gc);
            done.send(logical.0.load(Ordering::SeqCst)).unwrap();
        });
        let before_resume = delivered.recv_timeout(Duration::from_secs(2));
        release.send(()).unwrap();
        assert_eq!(
            before_resume,
            Ok(1),
            "a paused physical probe held up an unrelated wake"
        );
    });
    assert_eq!(
        physical.0.load(Ordering::SeqCst),
        0,
        "stale sample woke a cancelled request"
    );
    assert_eq!(gc.recovery.waiter_count(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn another_helper_delivers_wakes_while_a_callback_is_parked() {
    for worker_exit in [false, true] {
        let gc = GarbageCollector::new();
        let (entered, parked) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let blocker = Waker::from(Arc::new(Parked {
            entered,
            release: Mutex::new(resume),
        }));
        let waiting = Arc::new(Notice(AtomicUsize::new(0)));
        let cancelled = Arc::new(Notice(AtomicUsize::new(0)));
        let register = |waker: &Waker| {
            gc.recovery
                .register(
                    1,
                    None,
                    Waiter {
                        bytes: Some(1),
                        waker: waker.clone(),
                        space_seen: None,
                        grant: None,
                    },
                    None,
                )
                .unwrap()
        };
        register(&blocker);
        register(&Waker::from(waiting.clone()));
        let cancelled_id = register(&Waker::from(cancelled.clone()));
        std::thread::scope(|scope| {
            scope.spawn(|| {
                if worker_exit {
                    gc.recovery.notify_all(&gc);
                } else {
                    gc.recovery.notify_progress(&gc);
                }
            });
            parked.recv_timeout(Duration::from_secs(10)).unwrap();
            gc.recovery.remove(1, Some(cancelled_id));
            gc.recovery.notify_progress(&gc);
            let delivered = waiting.0.load(Ordering::SeqCst);
            release.send(()).unwrap();
            assert_eq!(delivered, 1, "another helper must reach ready waiters");
        });
        assert_eq!(waiting.0.load(Ordering::SeqCst), 1);
        assert_eq!(cancelled.0.load(Ordering::SeqCst), 0);
        assert_eq!(gc.recovery.waiter_count(), 0);
    }
}

#[test]
fn synchronous_reentry_drains_the_ready_set_without_recursive_wakes() {
    struct Reenter {
        gc: Arc<GarbageCollector>,
        inside: AtomicBool,
        count: AtomicUsize,
    }
    impl Wake for Reenter {
        fn wake(self: Arc<Self>) {
            assert!(!self.inside.swap(true, Ordering::SeqCst));
            self.count.fetch_add(1, Ordering::SeqCst);
            self.gc.recovery.notify_progress(&self.gc);
            self.inside.store(false, Ordering::SeqCst);
        }
    }
    let gc = GarbageCollector::new();
    let owner = Arc::new(Reenter {
        gc: gc.clone(),
        inside: AtomicBool::new(false),
        count: AtomicUsize::new(0),
    });
    let waker = Waker::from(owner.clone());
    for _ in 0..128 {
        gc.recovery
            .register(
                1,
                None,
                Waiter {
                    bytes: Some(1),
                    waker: waker.clone(),
                    space_seen: None,
                    grant: None,
                },
                None,
            )
            .unwrap();
    }
    gc.recovery.notify_progress(&gc);
    assert_eq!(owner.count.load(Ordering::SeqCst), 128);
    assert_eq!(gc.recovery.waiter_count(), 0);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn collection_reclaims_an_unrelated_object_while_the_waiter_index_is_parked() {
    use crate::{Configuration, Gc, GcWeak, ReferenceKind};
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(16)
            .metadata_shards(1)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let root = Gc::try_new(&gc, 1u64).unwrap();
    let weak = GcWeak::new(&root, ReferenceKind::Weak, 0);
    let credit = gc.reserve_bytes(8).unwrap();
    let notice = Arc::new(Notice(AtomicUsize::new(0)));
    gc.recovery
        .register(
            u64::MAX,
            None,
            Waiter {
                bytes: Some(8),
                waker: Waker::from(notice.clone()),
                space_seen: None,
                grant: None,
            },
            None,
        )
        .unwrap();
    let (start, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let (done, finished) = mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            gc.recovery.with_parked_waiter_index(move || {
                start.send(()).unwrap();
                release.recv_timeout(Duration::from_secs(8)).unwrap();
            });
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        drop((credit, root));
        let old = gc.completed_full_ticket();
        let heap = gc.clone();
        scope.spawn(move || {
            heap.collect_full().unwrap();
            done.send((heap.completed_full_ticket(), heap.managed_bytes()))
                .unwrap();
        });
        let before_release = finished.recv_timeout(Duration::from_secs(1));
        resume.send(()).unwrap();
        assert!(
            matches!(before_release,Ok((ticket,0)) if ticket > old),
            "the full collection and physical refund must complete before releasing the waiter index: {before_release:?}"
        );
    });
    assert!(weak.upgrade().is_none());
    let end = std::time::Instant::now() + Duration::from_secs(3);
    while notice.0.load(Ordering::Acquire) == 0 && std::time::Instant::now() < end {
        std::thread::yield_now();
    }
    assert_eq!(
        notice.0.load(Ordering::Acquire),
        1,
        "the deferred wake must be delivered exactly once without a further collector step"
    );
}

#[cfg(not(target_family = "wasm"))]
fn waiting(bytes: Option<usize>, seen: Option<Arc<AtomicBool>>, notice: &Arc<Notice>) -> Waiter {
    Waiter {
        grant: None,
        bytes,
        waker: Waker::from(notice.clone()),
        space_seen: seen,
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_retried_index_update_does_not_offer_old_headroom_to_a_later_waiter() {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let notice = Arc::new(Notice(AtomicUsize::new(0)));
    let seen = Arc::new(AtomicBool::new(false));
    let first = gc
        .recovery
        .register(u64::MAX, None, waiting(Some(8), None, &notice), None)
        .unwrap();
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    std::thread::scope(|scope| {
        let heap = &gc;
        let helper = scope.spawn(move || {
            AFTER_BUDGET_SAMPLE.set(Some(Box::new(move || {
                entered.send(()).unwrap();
                release.recv_timeout(Duration::from_secs(5)).unwrap();
            })));
            heap.recovery.notify_progress(heap);
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        let charge = gc.reserve_private_bytes(8).unwrap();
        gc.recovery.remove(u64::MAX, Some(first));
        let late = gc
            .recovery
            .register(
                u64::MAX,
                None,
                waiting(Some(8), Some(seen.clone()), &notice),
                None,
            )
            .unwrap();
        resume.send(()).unwrap();
        helper.join().unwrap();
        assert!(!seen.load(Ordering::Acquire));
        assert_eq!(notice.0.load(Ordering::Acquire), 0);
        gc.recovery.remove(u64::MAX, Some(late));
        drop(charge);
    });
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn published_fit_is_visible_while_the_callback_claimant_is_parked() {
    for native_refund in [false, true] {
        let gc = GarbageCollector::with_config(
            crate::Configuration::builder().max_heap_bytes(8).build(),
        );
        let notice = Arc::new(Notice(AtomicUsize::new(0)));
        let seen = Arc::new(AtomicBool::new(false));
        gc.recovery
            .register(
                u64::MAX,
                None,
                waiting(Some(8), Some(seen.clone()), &notice),
                None,
            )
            .unwrap();
        let (enter, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        std::thread::scope(|scope| {
            let heap = &gc;
            let helper = scope.spawn(move || {
                AFTER_CALLBACK_CLAIM.set(Some(Box::new(move || {
                    enter.send(()).unwrap();
                    release.recv_timeout(Duration::from_secs(5)).unwrap();
                })));
                if native_refund {
                    heap.recovery.take_refund_waker(heap).unwrap().wake();
                } else {
                    heap.recovery.notify_progress(heap);
                }
            });
            parked.recv_timeout(Duration::from_secs(3)).unwrap();
            let stolen = gc.reserve_private_bytes(8).unwrap();
            let witnessed = seen.load(Ordering::Acquire);
            resume.send(()).unwrap();
            helper.join().unwrap();
            assert!(witnessed);
            assert_eq!(notice.0.load(Ordering::Acquire), 1);
            drop(stolen);
        });
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn old_physical_probe_cannot_wake_a_replacement_registration() {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().compact_heap(true).build());
    let first = Arc::new(Notice(AtomicUsize::new(0)));
    let second = Arc::new(Notice(AtomicUsize::new(0)));
    let layout = Some(Layout::new::<u64>());
    let id = gc
        .recovery
        .register(u64::MAX, None, waiting(None, None, &first), layout)
        .unwrap();
    let (enter, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    std::thread::scope(|scope| {
        let heap = &gc;
        let helper = scope.spawn(move || {
            AFTER_PHYSICAL_PROBE.set(Some(Box::new(move || {
                enter.send(()).unwrap();
                release.recv_timeout(Duration::from_secs(5)).unwrap();
            })));
            heap.recovery.notify_progress(heap);
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        gc.recovery
            .register(u64::MAX, Some(id), waiting(None, None, &second), layout)
            .unwrap();
        resume.send(()).unwrap();
        helper.join().unwrap();
    });
    assert_eq!(first.0.load(Ordering::Acquire), 0);
    assert_eq!(second.0.load(Ordering::Acquire), 0);
    gc.recovery.notify_progress(&gc);
    assert_eq!(second.0.load(Ordering::Acquire), 1);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_stopped_snapshot_writer_cannot_retain_cancelled_callbacks() {
    let gc = GarbageCollector::new();
    let mut registered = Vec::new();
    for _ in 0..64 {
        let notice = Arc::new(Notice(AtomicUsize::new(0)));
        let id = gc
            .recovery
            .register(u64::MAX, None, waiting(None, None, &notice), None)
            .unwrap();
        registered.push((id, Arc::downgrade(&notice)));
    }
    let (enter, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            gc.recovery.with_parked_waiter_index(move || {
                enter.send(()).unwrap();
                release.recv_timeout(Duration::from_secs(5)).unwrap();
            });
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        for (id, weak) in &registered {
            gc.recovery.remove(u64::MAX, Some(*id));
            assert!(weak.upgrade().is_none());
        }
        resume.send(()).unwrap();
    });
    assert_eq!(gc.recovery.waiter_count(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::panic_in_result_fn, reason = "forced mixed waiter schedule")]
fn a_reentrant_last_worker_exit_delivers_the_other_waiter_kind() -> Result<()> {
    struct Exit(Mutex<Option<WorkerRegistration>>);
    impl Wake for Exit {
        fn wake(self: Arc<Self>) {
            drop(lock(&self.0).take());
        }
    }
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let held = gc.reserve_private_bytes(8)?;
    let demand = gc.grant_registry.register(&gc, 8)?;
    let worker = WorkerRegistration::new(&gc);
    let hot = Arc::new(Notice(AtomicUsize::new(0)));
    let mut observation = gc.observe_space(8)?;
    let mut sleeping = Box::pin(
        CollectionWait::for_space(&gc, u64::MAX, &mut observation)
            .with_grant(&demand, false, false),
    );
    assert!(
        sleeping
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(hot.clone())))
            .is_pending()
    );
    gc.recovery.register(
        u64::MAX,
        None,
        Waiter {
            bytes: Some(0),
            waker: Waker::from(Arc::new(Exit(Mutex::new(Some(worker))))),
            grant: None,
            space_seen: None,
        },
        None,
    )?;
    gc.recovery.notify_progress(&gc);
    assert_eq!(hot.0.load(Ordering::Acquire), 1);
    assert_eq!(gc.recovery.waiter_count(), 0);
    drop((sleeping, held));
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::panic_in_result_fn, reason = "forced mixed waiter schedule")]
fn parked_allocation_callback_leaves_last_exit_visible_to_ticket_waiters() -> Result<()> {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let held = gc.reserve_private_bytes(8)?;
    let worker = WorkerRegistration::new(&gc);
    let demand = gc.grant_registry.register(&gc, 8)?;
    let mut observation = gc.observe_space(8)?;
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let blocker = Waker::from(Arc::new(Parked {
        entered,
        release: Mutex::new(release),
    }));
    let mut sleeping = Box::pin(
        CollectionWait::for_space(&gc, u64::MAX, &mut observation)
            .with_grant(&demand, false, false),
    );
    assert!(
        sleeping
            .as_mut()
            .poll(&mut Context::from_waker(&blocker))
            .is_pending()
    );
    let cold = Arc::new(Notice(AtomicUsize::new(0)));
    gc.recovery
        .register(u64::MAX, None, waiting(None, None, &cold), None)?;
    std::thread::scope(|scope| {
        scope.spawn(move || drop(worker));
        let entered = parked.recv_timeout(Duration::from_secs(2));
        gc.recovery.notify_progress(&gc);
        let count = cold.0.load(Ordering::Acquire);
        assert!(resume.send(()).is_ok());
        assert!(entered.is_ok());
        assert_eq!(count, 1);
    });
    drop((sleeping, held));
    assert_eq!(gc.recovery.waiter_count(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "check a single budget sample across waiter kinds"
)]
fn one_headroom_sample_cannot_offer_the_same_bytes_in_both_waiter_indexes() -> Result<()> {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let mut charge = Some(gc.reserve_private_bytes(8)?);
    let demand = gc.grant_registry.register(&gc, 8)?;
    let mut observation = gc.observe_space(8)?;
    let counter = Arc::new(Notice(AtomicUsize::new(0)));
    let mut hot = Box::pin(
        CollectionWait::for_space(&gc, u64::MAX, &mut observation)
            .with_grant(&demand, false, false),
    );
    // Install using the same first-poll path with no native dispatcher.
    hot.as_mut()
        .get_mut()
        .register(&Waker::from(counter.clone()))?;
    let hot_seen = hot.space_seen.clone().expect("hot receipt");
    let cold_seen = Arc::new(AtomicBool::new(false));
    gc.recovery.register(
        u64::MAX,
        None,
        waiting(Some(8), Some(cold_seen.clone()), &counter),
        None,
    )?;
    drop(charge.take());
    gc.recovery.notify_progress(&gc);
    assert_eq!(
        usize::from(hot_seen.load(Ordering::Acquire))
            + usize::from(cold_seen.load(Ordering::Acquire)),
        1
    );
    assert_eq!(counter.0.load(Ordering::Acquire), 1);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "check priority of mixed size waiters"
)]
fn one_offer_services_the_smallest_waiting_allocations_first() -> Result<()> {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(16).build());
    let charge = gc.reserve_private_bytes(16)?;
    let large_demand = gc.grant_registry.register(&gc, 16)?;
    let mut observation = gc.observe_space(16)?;
    let large = Arc::new(Notice(AtomicUsize::new(0)));
    let small = Arc::new(Notice(AtomicUsize::new(0)));
    let mut hot = CollectionWait::for_space(&gc, u64::MAX, &mut observation).with_grant(
        &large_demand,
        false,
        false,
    );
    hot.register(&Waker::from(large.clone()))?;
    for _ in 0..2 {
        gc.recovery
            .register(u64::MAX, None, waiting(Some(8), None, &small), None)?;
    }
    drop(charge);
    gc.recovery.notify_progress(&gc);
    assert_eq!(large.0.load(Ordering::Acquire), 0);
    assert_eq!(small.0.load(Ordering::Acquire), 2);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn after_worker_count_decrement() {
    if let Some(hook) = AFTER_WORKER_COUNT_DECREMENT.take() {
        hook();
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_peer_can_deliver_last_worker_exit_before_the_exiting_thread_notifies() -> Result<()> {
    for (refund_only, cancel_allocation) in [(false, false), (true, false), (true, true)] {
        last_worker_exit_with_parked_publisher(refund_only, cancel_allocation)?;
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[expect(clippy::panic_in_result_fn, reason = "forced last-worker schedule")]
fn last_worker_exit_with_parked_publisher(
    refund_only: bool,
    cancel_allocation: bool,
) -> Result<()> {
    struct Signal(mpsc::Sender<()>);
    impl Wake for Signal {
        fn wake(self: Arc<Self>) {
            self.0.send(()).unwrap();
        }
    }
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let held = gc.reserve_private_bytes(7)?;
    let refund = gc.reserve_private_bytes(1)?;
    let worker = WorkerRegistration::new(&gc);
    let demand = gc.grant_registry.register(&gc, 8)?;
    let mut observation = gc.observe_space(8)?;
    let (allocation, allocated) = mpsc::channel();
    let (ticket, notified) = mpsc::channel();
    let mut sleeping = Box::pin(
        CollectionWait::for_space(&gc, u64::MAX, &mut observation)
            .with_grant(&demand, false, false),
    );
    assert!(
        sleeping
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(Arc::new(Signal(
                allocation
            )))))
            .is_pending()
    );
    let sleeping = (!cancel_allocation).then_some(sleeping);
    gc.recovery.register(
        u64::MAX,
        None,
        Waiter {
            bytes: None,
            grant: None,
            space_seen: None,
            waker: Waker::from(Arc::new(Signal(ticket))),
        },
        None,
    )?;
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(move || {
            AFTER_WORKER_COUNT_DECREMENT.set(Some(Box::new(move || {
                entered.send(()).unwrap();
                release.recv_timeout(Duration::from_secs(10)).unwrap();
            })));
            drop(worker);
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        let workers = gc.recovery.registered_workers();
        if refund_only {
            // A single returned byte cannot satisfy the eight-byte allocation.
            // The native refund notifier must discover the published exit.
            drop(refund);
        } else {
            gc.recovery.notify_progress(&gc);
        }
        let wakes = (
            cancel_allocation || allocated.recv_timeout(Duration::from_secs(1)).is_ok(),
            notified.recv_timeout(Duration::from_secs(1)).is_ok(),
        );
        resume.send(()).unwrap();
        assert_eq!(workers, 0);
        assert_eq!(
            wakes,
            (true, true),
            "both sleepers must wake before the exiting thread resumes"
        );
    });
    drop((sleeping, held));
    assert_eq!(gc.recovery.waiter_count(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::panic_in_result_fn, reason = "forced executor migration")]
fn repeated_polls_keep_the_waiter_slot_and_wake_the_current_executor() -> Result<()> {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let held = gc.reserve_private_bytes(8)?;
    let worker = WorkerRegistration::new(&gc);
    let demand = gc.grant_registry.register(&gc, 8)?;
    let mut observation = gc.observe_space(8)?;
    let mut wait = Box::pin(
        CollectionWait::for_space(&gc, u64::MAX, &mut observation)
            .with_grant(&demand, false, false),
    );
    let first = Arc::new(Notice(AtomicUsize::new(0)));
    let second = Arc::new(Notice(AtomicUsize::new(0)));
    for notice in [&first, &first, &second, &second] {
        assert!(
            wait.as_mut()
                .poll(&mut Context::from_waker(&Waker::from(notice.clone())))
                .is_pending()
        );
        assert!(
            wait.first_arm.is_some(),
            "repoll replaced the occupied demand slot"
        );
        assert_eq!(gc.recovery.waiter_count(), 1);
    }
    assert_eq!(Arc::strong_count(&first), 1);
    drop(worker);
    assert_eq!(first.0.load(Ordering::Acquire), 0);
    assert_eq!(second.0.load(Ordering::Acquire), 1);
    assert_eq!(Arc::strong_count(&second), 1);
    assert_eq!(gc.recovery.waiter_count(), 0);
    drop((wait, held));
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn before_destination_commit() {
    if let Some(hook) = BEFORE_DESTINATION_COMMIT.take() {
        hook();
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "forced delivery during executor migration"
)]
fn a_claim_during_executor_migration_never_revives_the_delivered_slot() -> Result<()> {
    for replace_again in [false, true] {
        let gc = GarbageCollector::with_config(
            crate::Configuration::builder().max_heap_bytes(8).build(),
        );
        let held = gc.reserve_private_bytes(8)?;
        let worker = WorkerRegistration::new(&gc);
        let demand = gc.grant_registry.register(&gc, 8)?;
        let mut observation = gc.observe_space(8)?;
        let mut wait = Box::pin(
            CollectionWait::for_space(&gc, u64::MAX, &mut observation)
                .with_grant(&demand, false, false),
        );
        let first = Arc::new(Notice(AtomicUsize::new(0)));
        let second = Arc::new(Notice(AtomicUsize::new(0)));
        let last = Arc::new(Notice(AtomicUsize::new(0)));
        for notice in std::iter::once(&first).chain(replace_again.then_some(&second)) {
            assert!(
                wait.as_mut()
                    .poll(&mut Context::from_waker(&Waker::from(notice.clone())))
                    .is_pending()
            );
        }
        let replacement = Arc::new(Mutex::new(None));
        let next = replacement.clone();
        let heap = gc.clone();
        BEFORE_DESTINATION_COMMIT.set(Some(Box::new(move || {
            drop(worker);
            *lock(&next) = Some(WorkerRegistration::new(&heap));
        })));
        assert!(
            wait.as_mut()
                .poll(&mut Context::from_waker(&Waker::from(last.clone())))
                .is_pending()
        );
        assert_eq!(first.0.load(Ordering::Acquire), usize::from(!replace_again));
        assert_eq!(second.0.load(Ordering::Acquire), usize::from(replace_again));
        assert_eq!(last.0.load(Ordering::Acquire), 0);
        assert_eq!(Arc::strong_count(&first), 1);
        assert_eq!(Arc::strong_count(&second), 1);
        assert_eq!(gc.recovery.waiter_count(), 1);
        drop(lock(&replacement).take());
        assert_eq!(last.0.load(Ordering::Acquire), 1);
        assert_eq!(Arc::strong_count(&last), 1);
        assert_eq!(gc.recovery.waiter_count(), 0);
        drop((wait, held));
    }
    Ok(())
}

#[test]
fn cached_executor_identity_comes_from_the_owned_waker_clone() {
    use std::task::{RawWaker, RawWakerVTable};
    static FIRST: AtomicUsize = AtomicUsize::new(0);
    static SECOND: AtomicUsize = AtomicUsize::new(0);
    static SECOND_TASK: AtomicBool = AtomicBool::new(false);
    fn route() -> *const () {
        std::ptr::from_ref(if SECOND_TASK.load(Ordering::Acquire) {
            &SECOND
        } else {
            &FIRST
        })
        .cast()
    }
    fn deliver(p: *const ()) {
        let count = if p == std::ptr::from_ref(&FIRST).cast() {
            &FIRST
        } else {
            &SECOND
        };
        count.fetch_add(1, Ordering::AcqRel);
    }
    fn snapshot(p: *const ()) -> RawWaker {
        RawWaker::new(p, &SNAPSHOT)
    }
    static SNAPSHOT: RawWakerVTable = RawWakerVTable::new(snapshot, deliver, deliver, |_| {});
    static INPUT: RawWakerVTable = RawWakerVTable::new(
        |_| snapshot(route()),
        |_| deliver(route()),
        |_| deliver(route()),
        |_| {},
    );
    fn input() -> Waker {
        // SAFETY: all callbacks and counters are static. A clone captures the
        // current task. Its source token is retired before issuing a new one.
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &INPUT)) }
    }
    let recovery = Recovery::default();
    recovery.armed.store(1, Ordering::Release);
    let owned = input().clone();
    let mut target = grant_wake::WakeTarget::of(&owned);
    let arm = grant_wake::Arm::new(1, 1, 8, false, None, owned);
    SECOND_TASK.store(true, Ordering::Release);
    assert!(arm.refresh(&mut target, &input()));
    arm.close(&recovery).0.unwrap().wake();
    assert_eq!(FIRST.load(Ordering::Acquire), 0);
    assert_eq!(SECOND.load(Ordering::Acquire), 1);
}
