//! A stalled reclaimed object's host callback must not delay another mutator's recovery.
#![cfg(not(target_family = "wasm"))]
#![expect(unsafe_code, reason = "the fixture contains no managed edges")]
#![expect(clippy::expect_used, clippy::panic_in_result_fn)]
use ristretto_gc::{
    Configuration, Finalize, GarbageCollector, Gc, GcRootGuard, Result, Trace, TraceProgress,
};
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Finalize,
    Clear,
    Drop,
}
struct Callback {
    stage: Stage,
    entered: mpsc::Sender<()>,
    resume: Mutex<mpsc::Receiver<()>>,
    released: AtomicBool,
    finalized: AtomicUsize,
    cleared: AtomicUsize,
    dropped: AtomicUsize,
}
impl Callback {
    fn pause(&self, stage: Stage) {
        if self.stage == stage {
            self.entered.send(()).expect("observer live");
            let _ = self
                .resume
                .lock()
                .expect("callback lock")
                .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }));
            self.released.store(true, Ordering::Release);
        }
    }
}
struct Payload(Arc<Callback>);
// SAFETY: callback control and counters contain no managed edges.
unsafe impl Trace for Payload {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
    fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
        self.0.cleared.fetch_add(1, Ordering::Relaxed);
        self.0.pause(Stage::Clear);
        TraceProgress::Complete
    }
}
impl Finalize for Payload {
    fn finalize(&self) {
        self.0.finalized.fetch_add(1, Ordering::Relaxed);
        self.0.pause(Stage::Finalize);
    }
}
impl Drop for Payload {
    fn drop(&mut self) {
        self.0.dropped.fetch_add(1, Ordering::Relaxed);
        self.0.pause(Stage::Drop);
    }
}

fn wait_for_registered_worker_sweep(gc: &GarbageCollector) -> Result<()> {
    assert_eq!(gc.registered_workers(), 1);
    let mut request = Box::pin(gc.collect_full_async());
    let mut cx = Context::from_waker(Waker::noop());
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Poll::Ready(result) = request.as_mut().poll(&mut cx) {
            return result;
        }
        assert!(
            Instant::now() < deadline,
            "native worker did not finish the full ticket"
        );
        std::thread::yield_now();
    }
}

#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "closed schedule channels fail the test"
)]
fn explicit_collection_reclaims_queued_objects_while_an_older_drop_is_parked() -> Result<()> {
    struct Counted(Arc<AtomicUsize>);
    // SAFETY: this fixture owns only a host-side destructor counter.
    unsafe impl Trace for Counted {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(1)
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let (entered, started) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        let callback = Arc::new(Callback {
            stage: Stage::Drop,
            entered,
            resume: Mutex::new(release),
            released: AtomicBool::new(false),
            finalized: AtomicUsize::new(0),
            cleared: AtomicUsize::new(0),
            dropped: AtomicUsize::new(0),
        });
        drop(Gc::try_new(&gc, Payload(callback.clone()))?);
        gc.start();
        gc.collect();
        started
            .recv_timeout(Duration::from_secs(5))
            .expect("older destructor started");
        let drops = Arc::new(AtomicUsize::new(0));
        for _ in 0..32 {
            drop(Gc::try_new(&gc, Counted(drops.clone()))?);
        }
        // A registered native worker retires the second group. Only the original
        // callback runner is parked, and no address lease protects this group.
        wait_for_registered_worker_sweep(&gc)?;
        assert_eq!(gc.statistics()?.live_objects, 0);
        let (send, recv) = mpsc::channel();
        let (completed, still_parked) = std::thread::scope(|scope| {
            let gc = gc.clone();
            let drops = drops.clone();
            scope.spawn(move || {
                send.send(gc.collect_full().map(|()| drops.load(Ordering::Acquire)))
                    .expect("report fresh explicit collection");
            });
            let result = recv.recv_timeout(Duration::from_secs(3));
            let parked = !callback.released.load(Ordering::Acquire);
            resume.send(()).expect("release older destructor");
            (result, parked)
        });
        gc.stop()?;
        assert!(still_parked);
        assert_eq!(completed.expect("independent ticket completed")?, 32);
        assert_eq!(drops.load(Ordering::Acquire), 32);
        assert_eq!(callback.dropped.load(Ordering::Acquire), 1);
    }
    Ok(())
}

#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "closed channels fail the forced schedule"
)]
fn a_parked_first_destructor_does_not_capture_the_rest_of_its_sweep_batch() -> Result<()> {
    struct Group {
        first: AtomicBool,
        entered: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
        drops: AtomicUsize,
    }
    struct Peer(Arc<Group>);
    // SAFETY: the entire group contains only host scheduling controls.
    unsafe impl Trace for Peer {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Peer {
        fn drop(&mut self) {
            if !self.0.first.swap(true, Ordering::AcqRel) {
                self.0.entered.send(()).expect("first drop observer");
                self.0
                    .release
                    .lock()
                    .expect("drop gate")
                    .recv_timeout(Duration::from_secs(10))
                    .expect("release the first destructor");
            }
            self.0.drops.fetch_add(1, Ordering::Release);
        }
    }
    const PEERS: usize = 128;
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(1)
                .metadata_shards(1)
                .incremental_step_size(256)
                .max_pause_time_us(0)
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let (entered, parked) = mpsc::channel();
        let (release, gate) = mpsc::channel();
        let group = Arc::new(Group {
            first: AtomicBool::new(false),
            entered,
            release: Mutex::new(gate),
            drops: AtomicUsize::new(0),
        });
        for _ in 0..PEERS {
            drop(Gc::try_new(&gc, Peer(group.clone()))?);
        }
        gc.start();
        gc.collect();
        parked
            .recv_timeout(Duration::from_secs(3))
            .expect("first callback");
        wait_for_registered_worker_sweep(&gc)?;
        let (done, result) = mpsc::channel();
        let heap = gc.clone();
        let progress = group.clone();
        let later = std::thread::spawn(move || {
            let result = heap.collect_full();
            done.send(progress.drops.load(Ordering::Acquire))
                .expect("later caller");
            result
        });
        let completed = result.recv_timeout(Duration::from_secs(3));
        release.send(()).expect("release first callback");
        later.join().expect("independent full")?;
        gc.stop()?;
        assert_eq!(
            completed.expect("later full returned"),
            PEERS - 1,
            "parked destructor stranded independent storage within its batch"
        );
        assert_eq!(group.drops.load(Ordering::Acquire), PEERS);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

struct Ready(mpsc::SyncSender<()>);

#[cfg(all(not(miri), panic = "unwind"))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "closed schedule channels fail the test"
)]
fn shutdown_observes_failure_from_a_foreground_callback_it_did_not_run() -> Result<()> {
    struct Fails(
        mpsc::Sender<std::thread::ThreadId>,
        Mutex<mpsc::Receiver<()>>,
    );
    // SAFETY: the two channels carry only host signals.
    unsafe impl Trace for Fails {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            self.0
                .send(std::thread::current().id())
                .expect("callback observer");
            self.1
                .lock()
                .expect("failure gate")
                .recv_timeout(Duration::from_secs(5))
                .expect("release helper");
            panic!("intentional foreground clearing failure")
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let older = Arc::new(Callback {
        stage: Stage::Drop,
        entered,
        resume: Mutex::new(release),
        released: AtomicBool::new(false),
        finalized: AtomicUsize::new(0),
        cleared: AtomicUsize::new(0),
        dropped: AtomicUsize::new(0),
    });
    drop(Gc::try_new(&gc, Payload(older.clone()))?);
    gc.start();
    gc.collect();
    parked
        .recv_timeout(Duration::from_secs(3))
        .expect("native callback parked");
    let (started, running) = mpsc::channel();
    let (finish, fail) = mpsc::channel();
    drop(Gc::try_new(&gc, Fails(started, Mutex::new(fail)))?);
    wait_for_registered_worker_sweep(&gc)?;
    assert_eq!(gc.statistics()?.live_objects, 0);
    let heap = gc.clone();
    let helper = std::thread::spawn(move || heap.collect_full());
    assert_eq!(
        running.recv_timeout(Duration::from_secs(3)),
        Ok(helper.thread().id())
    );
    let heap = gc.clone();
    let stopper = std::thread::spawn(move || heap.stop());
    let deadline = Instant::now() + Duration::from_secs(3);
    while gc.registered_workers() != 0 {
        assert!(Instant::now() < deadline);
        std::thread::yield_now();
    }
    resume.send(()).expect("release native callback");
    let deadline = Instant::now() + Duration::from_secs(3);
    while !older.released.load(Ordering::Acquire) {
        assert!(Instant::now() < deadline);
        std::thread::yield_now();
    }
    finish.send(()).expect("allow clearing failure");
    helper.join().expect("foreground helper")?;
    assert!(matches!(
        stopper.join().expect("stopper"),
        Err(ristretto_gc::Error::SyncError(_))
    ));
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

impl Wake for Ready {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = self.0.try_send(());
    }
}

fn exercise(stage: Stage, registered: bool, release_donor: bool) -> Result<()> {
    exercise_payload(
        stage,
        registered,
        release_donor,
        Payload,
        |gc, payload| Ok(Gc::new_with_finalizer(gc, payload)),
        [1, 1],
        7_u64,
    )
}

fn exercise_payload<T: Trace + Send + Sync + 'static, D: Trace + Send + Sync + 'static>(
    stage: Stage,
    registered: bool,
    release_donor: bool,
    make: impl FnOnce(Arc<Callback>) -> T,
    allocate: impl FnOnce(&Arc<GarbageCollector>, T) -> Result<GcRootGuard<T>>,
    [finalized, cleared]: [usize; 2],
    donor: D,
) -> Result<()> {
    let (entered, ready) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let callback = Arc::new(Callback {
        stage,
        entered,
        resume: Mutex::new(release),
        released: AtomicBool::new(false),
        finalized: AtomicUsize::new(0),
        cleared: AtomicUsize::new(0),
        dropped: AtomicUsize::new(0),
    });
    let payload = make(callback.clone());
    let payload_size = Gc::allocation_size(&payload)?;
    let donor_size = Gc::allocation_size(&donor)?;
    let gc = GarbageCollector::with_config(Configuration {
        threads: 1,
        max_heap_bytes: payload_size + donor_size,
        allocation_threshold: usize::MAX,
        incremental_step_size: 1,
        max_pause_time_us: 0,
        ..Configuration::default()
    });
    let doomed = allocate(&gc, payload)?;
    let garbage = Gc::try_new(&gc, donor)?;
    assert_eq!(gc.managed_bytes(), payload_size + donor_size);
    drop(doomed);
    let first = if registered {
        gc.start();
        gc.collect();
        None
    } else {
        let first_gc = gc.clone();
        Some(std::thread::spawn(move || first_gc.collect_full()))
    };
    ready
        .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }))
        .map_err(|error| {
            ristretto_gc::Error::CollectionPhaseError(format!("host callback: {error}"))
        })?;
    // The first snapshot kept this object alive. A later recovery cycle must
    // reclaim it while the earlier detached callback is still stalled.
    let retained = (!release_donor).then_some(garbage);
    let mut allocation = Box::pin(Gc::try_new_async(&gc, 11_u64));
    let (signal, runnable) = mpsc::sync_channel(1);
    let waker = Waker::from(Arc::new(Ready(signal)));
    let mut context = Context::from_waker(&waker);
    let deadline = Instant::now() + Duration::from_secs(if cfg!(miri) { 20 } else { 2 });
    let mut result = None;
    for _ in 0..2048 {
        if let Poll::Ready(value) = allocation.as_mut().poll(&mut context) {
            result = Some(value);
            break;
        }
        if registered
            && runnable
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .is_err()
        {
            break;
        }
    }
    let still_parked = !callback.released.load(Ordering::Acquire);
    let progress = gc.statistics()?;
    drop(allocation);
    drop(resume);
    if let Some(first) = first {
        first.join().map_err(|_| {
            ristretto_gc::Error::CollectionPhaseError("first mutator panicked".into())
        })??;
    }
    let result = result.expect("another mutator's recovery awaited the host callback");
    assert!(still_parked, "host callback failure deadline expired");
    assert!(progress.collections_started >= 2);
    if release_donor {
        let allocated = result?;
        // Physical headroom is visible as soon as the allocation is funded.
        // Each worker flushes its cumulative work counters after its step.
        assert_eq!(
            progress.bytes_managed,
            payload_size + Gc::allocation_size(&11_u64)?
        );
        assert_eq!(**allocated, 11);
    } else {
        assert!(matches!(result, Err(ristretto_gc::Error::OutOfMemory)));
        assert_eq!(progress.bytes_managed, payload_size + donor_size);
        assert_eq!(progress.managed_bytes_freed, 0);
    }
    drop(retained);
    gc.stop()?;
    let replacement_charge = if release_donor {
        Gc::allocation_size(&11_u64)?
    } else {
        0
    };
    assert_eq!(
        gc.statistics()?.managed_bytes_freed,
        payload_size + donor_size + replacement_charge
    );
    assert_eq!(callback.finalized.load(Ordering::Relaxed), finalized);
    assert_eq!(callback.cleared.load(Ordering::Relaxed), cleared);
    assert_eq!(callback.dropped.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn default_recovery_progresses_during_a_stalled_finalizer() -> Result<()> {
    exercise(Stage::Finalize, false, true)
}
#[test]
fn default_recovery_progresses_during_a_stalled_clear_callback() -> Result<()> {
    exercise(Stage::Clear, false, true)
}
#[test]
fn default_recovery_progresses_during_a_stalled_destructor() -> Result<()> {
    exercise(Stage::Drop, false, true)
}

#[cfg(not(miri))]
#[test]
fn registered_worker_recovers_an_independent_donor_during_finalize() -> Result<()> {
    exercise(Stage::Finalize, true, true)
}

#[cfg(not(miri))]
#[test]
fn registered_worker_recovers_an_independent_donor_during_clear() -> Result<()> {
    exercise(Stage::Clear, true, true)
}

#[cfg(not(miri))]
#[test]
fn registered_worker_recovers_an_independent_donor_during_drop() -> Result<()> {
    exercise(Stage::Drop, true, true)
}

// The destructor exists even when the tracing table omits both trace and clear.
struct DropOnly {
    _payload: Payload,
}
// SAFETY: this payload contains only the host-side callback state.
unsafe impl Trace for DropOnly {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn needs_clear() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}

#[cfg(not(miri))]
#[test]
fn ordinary_wrapped_clear_preserves_independent_allocation_progress() -> Result<()> {
    exercise_payload(
        Stage::Clear,
        true,
        true,
        |c| Some(Mutex::new(Payload(c))),
        Gc::try_new,
        [0, 1],
        7_u64,
    )
}

#[cfg(not(miri))]
#[test]
fn ordinary_destructor_without_trace_or_clear_preserves_independent_allocation_progress()
-> Result<()> {
    exercise_payload(
        Stage::Drop,
        true,
        true,
        |c| {
            Some(DropOnly {
                _payload: Payload(c),
            })
        },
        Gc::try_new,
        [0, 0],
        7_u64,
    )
}

#[cfg(not(miri))]
#[test]
fn primitive_buffer_donor_refunds_behind_a_stalled_host_destructor() -> Result<()> {
    exercise_payload(
        Stage::Drop,
        true,
        true,
        |c| DropOnly {
            _payload: Payload(c),
        },
        Gc::try_new,
        [0, 0],
        vec![5_u8; 256],
    )?;
    exercise_payload(
        Stage::Drop,
        true,
        true,
        |c| DropOnly {
            _payload: Payload(c),
        },
        Gc::try_new,
        [0, 0],
        Some(String::from("an independent string backing allocation")),
    )
}

#[cfg(not(miri))]
mod bounded_lock_donor {
    use super::*;

    fn check<D: Trace + Send + Sync + 'static>(donor: D) -> Result<()> {
        exercise_payload(
            Stage::Drop,
            true,
            true,
            |callback| DropOnly {
                _payload: Payload(callback),
            },
            Gc::try_new,
            [0, 0],
            donor,
        )
    }

    #[test]
    fn standard_mutex() -> Result<()> {
        check(Mutex::new(vec![5_u8; 256]))
    }

    #[test]
    fn standard_rwlock() -> Result<()> {
        check(std::sync::RwLock::new(vec![5_u8; 256]))
    }

    #[test]
    fn collector_mutex() -> Result<()> {
        check(ristretto_gc::sync::Mutex::new(vec![5_u8; 256]))
    }

    #[test]
    fn managed_rwlock() -> Result<()> {
        check(ristretto_gc::sync::RwLock::new(vec![5_u8; 256]))
    }

    #[test]
    fn managed_host_heap_buffer() -> Result<()> {
        let buffer = ristretto_gc::HeapBuffer::from(vec![5_u8; 256]);
        check(ristretto_gc::sync::RwLock::new(buffer))
    }

    #[test]
    fn managed_region_heap_buffer() -> Result<()> {
        let regions = ristretto_gc::region::RegionHeap::new(1024, 4096, 0);
        let buffer = ristretto_gc::HeapBuffer::try_with_fn(Some(&regions), 256, |_| Ok(5_u8))?;
        assert_eq!(regions.statistics().occupied_bytes, 256);
        check(ristretto_gc::sync::RwLock::new(buffer))?;
        assert_eq!(regions.statistics().occupied_bytes, 0);
        Ok(())
    }

    #[test]
    fn nested() -> Result<()> {
        check(Some(Mutex::new(ristretto_gc::sync::RwLock::new(
            String::from("backing"),
        ))))
    }
}

#[cfg(not(miri))]
#[test]
fn a_heap_buffer_element_destructor_leaves_the_worker_available() -> Result<()> {
    exercise_payload(
        Stage::Drop,
        true,
        true,
        |callback| ristretto_gc::HeapBuffer::from(vec![Payload(callback)]),
        Gc::try_new,
        [0, 0],
        7_u64,
    )
}

#[cfg(not(miri))]
#[test]
fn parked_cleanup_keeps_its_charge_until_physical_release() -> Result<()> {
    for stage in [Stage::Finalize, Stage::Clear, Stage::Drop] {
        exercise(stage, true, false)?;
    }
    Ok(())
}

#[cfg(not(miri))]
#[test]
fn callback_shutdown_cannot_wait_for_itself_and_the_pool_can_restart() -> Result<()> {
    struct Reentrant(std::sync::Weak<GarbageCollector>, mpsc::Sender<bool>);
    // SAFETY: the weak heap reference and test notification contain no managed edges.
    unsafe impl Trace for Reentrant {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Finalize for Reentrant {
        fn finalize(&self) {
            if let Some(gc) = self.0.upgrade() {
                let error = matches!(gc.stop(), Err(ristretto_gc::Error::CollectionPhaseError(_)));
                self.1.send(error).expect("shutdown observer");
            }
        }
    }
    let gc = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    for _ in 0..2 {
        let (send, receive) = mpsc::channel();
        let owner = Gc::new_with_finalizer(&gc, Reentrant(Arc::downgrade(&gc), send));
        drop(owner);
        gc.start();
        gc.collect();
        assert_eq!(receive.recv_timeout(Duration::from_secs(5)), Ok(true));
        gc.stop()?;
        assert_eq!(gc.registered_workers(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(miri))]
#[test]
fn caller_driven_cleanup_cannot_join_its_own_retirement() -> Result<()> {
    struct Reentrant(std::sync::Weak<GarbageCollector>, mpsc::Sender<bool>);
    // SAFETY: neither field contains managed edges.
    unsafe impl Trace for Reentrant {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Reentrant {
        fn drop(&mut self) {
            if let Some(gc) = self.0.upgrade() {
                let rejected =
                    matches!(gc.stop(), Err(ristretto_gc::Error::CollectionPhaseError(_)));
                let _ = self.1.send(rejected);
            }
        }
    }
    let gc = GarbageCollector::new();
    let (tx, rx) = mpsc::channel();
    drop(Gc::try_new(&gc, Reentrant(Arc::downgrade(&gc), tx))?);
    let helper = gc.clone();
    let collector = std::thread::spawn(move || helper.collect_full());
    assert_eq!(rx.recv_timeout(Duration::from_secs(2)), Ok(true));
    collector
        .join()
        .map_err(|_| ristretto_gc::Error::CollectionPhaseError("caller panicked".into()))??;
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(miri))]
#[test]
fn callback_panic_refunds_once_and_stop_reports_the_failure() -> Result<()> {
    struct Panics(mpsc::Sender<()>, Arc<AtomicUsize>);
    // SAFETY: channels and the destruction counter contain no managed edges.
    unsafe impl Trace for Panics {
        fn trace(&self, _: &GarbageCollector) {}

        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            self.0.send(()).expect("failure observer");
            panic!("test callback panic");
        }
    }
    impl Finalize for Panics {
        fn finalize(&self) {}
    }
    impl Drop for Panics {
        fn drop(&mut self) {
            self.1.fetch_add(1, Ordering::Relaxed);
        }
    }
    let gc = GarbageCollector::with_config(Configuration::builder().threads(1).build());
    let destroyed = Arc::new(AtomicUsize::new(0));
    let (send, entered) = mpsc::channel();
    drop(Gc::new_with_finalizer(&gc, Panics(send, destroyed.clone())));
    gc.start();
    gc.collect();
    assert_eq!(entered.recv_timeout(Duration::from_secs(5)), Ok(()));
    assert!(matches!(gc.stop(), Err(ristretto_gc::Error::SyncError(_))));
    assert_eq!(destroyed.load(Ordering::Relaxed), 1);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(miri))]
#[test]
fn first_shutdown_finishes_unrelated_reclamation_after_a_callback_failure() -> Result<()> {
    struct Fails(mpsc::Sender<()>, Mutex<mpsc::Receiver<()>>);
    // SAFETY: both channels contain host notifications only.
    unsafe impl Trace for Fails {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            self.0.send(()).expect("shutdown observer");
            let _ = self
                .1
                .lock()
                .expect("callback gate")
                .recv_timeout(Duration::from_secs(10));
            panic!("host cleanup failed");
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let (started, parked) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    let survivor = Gc::try_new(&gc, 97_u64)?;
    drop(Gc::try_new(&gc, Fails(started, Mutex::new(resume)))?);
    gc.start();
    gc.collect();
    assert_eq!(parked.recv_timeout(Duration::from_secs(3)), Ok(()));
    let helper = gc.clone();
    let stopper = std::thread::spawn(move || helper.stop());
    let deadline = Instant::now() + Duration::from_secs(3);
    while gc.registered_workers() != 0 {
        assert!(Instant::now() < deadline);
        std::thread::yield_now();
    }
    drop(survivor);
    drop(release);
    let outcome = stopper
        .join()
        .map_err(|_| ristretto_gc::Error::CollectionPhaseError("shutdown panicked".into()))?;
    assert!(matches!(outcome, Err(ristretto_gc::Error::SyncError(_))));
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(miri))]
#[test]
fn batched_finalizers_preserve_offsets_and_release_each_physical_charge() -> Result<()> {
    const ITEMS: usize = 64;
    struct Sliced {
        id: usize,
        offset: usize,
        initialized: AtomicBool,
        short: Arc<AtomicBool>,
        finished: mpsc::Sender<(usize, usize)>,
    }
    // SAFETY: the payload contains only counters and a host channel.
    unsafe impl Trace for Sliced {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
            assert!(self.initialized.load(Ordering::Acquire));
            assert_eq!(offset, self.offset);
            assert!((1..=32).contains(&budget));
            if budget < 32 {
                self.short.store(true, Ordering::Release);
            }
            self.offset = (offset + budget).min(37);
            if self.offset == 37 {
                TraceProgress::Complete
            } else {
                TraceProgress::Continue(self.offset)
            }
        }
    }
    impl Finalize for Sliced {
        fn finalize(&self) {
            assert!(!self.initialized.swap(true, Ordering::AcqRel));
        }
    }
    impl Drop for Sliced {
        fn drop(&mut self) {
            let _ = self.finished.send((self.id, self.offset));
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .incremental_step_size(64)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let (tx, rx) = mpsc::channel();
    let short = Arc::new(AtomicBool::new(false));
    let roots: Vec<_> = (0..ITEMS)
        .map(|id| {
            Gc::new_with_finalizer(
                &gc,
                Sliced {
                    id,
                    offset: 0,
                    initialized: AtomicBool::new(false),
                    short: short.clone(),
                    finished: tx.clone(),
                },
            )
        })
        .collect();
    drop(roots);
    gc.start();
    gc.collect();
    let mut completions = Vec::new();
    for _ in 0..ITEMS {
        completions.push(
            rx.recv_timeout(Duration::from_secs(3))
                .map_err(|err| ristretto_gc::Error::CollectionPhaseError(err.to_string()))?,
        );
    }
    gc.stop()?;
    completions.sort_unstable();
    assert!(completions.into_iter().eq((0..ITEMS).map(|id| (id, 37))));
    assert!(short.load(Ordering::Acquire));
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
