//! A stalled reclaimed object's host callback must not delay another mutator's recovery.
#![cfg(not(target_family = "wasm"))]
#![expect(unsafe_code, reason = "the fixture contains no managed edges")]
#![expect(clippy::expect_used, clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, Finalize, GarbageCollector, Gc, Result, Trace, TraceProgress};
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

struct Ready(mpsc::SyncSender<()>);

impl Wake for Ready {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = self.0.try_send(());
    }
}

fn exercise(stage: Stage, registered: bool, release_donor: bool) -> Result<()> {
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
    let payload = Payload(callback.clone());
    let payload_size = Gc::allocation_size(&payload)?;
    let scalar_size = Gc::allocation_size(&7_u64)?;
    let gc = GarbageCollector::with_config(Configuration {
        threads: 1,
        max_heap_bytes: payload_size + scalar_size,
        allocation_threshold: usize::MAX,
        incremental_step_size: 1,
        max_pause_time_us: 0,
        ..Configuration::default()
    });
    let doomed = Gc::new_with_finalizer(&gc, payload);
    let garbage = Gc::try_new(&gc, 7_u64)?;
    assert_eq!(gc.managed_bytes(), payload_size + scalar_size);
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
        assert!(progress.managed_bytes_freed >= scalar_size);
        assert_eq!(**allocated, 11);
    } else {
        assert!(matches!(result, Err(ristretto_gc::Error::OutOfMemory)));
        assert_eq!(progress.managed_bytes_freed, 0);
    }
    drop(retained);
    gc.stop()?;
    assert_eq!(callback.finalized.load(Ordering::Relaxed), 1);
    assert_eq!(callback.cleared.load(Ordering::Relaxed), 1);
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
