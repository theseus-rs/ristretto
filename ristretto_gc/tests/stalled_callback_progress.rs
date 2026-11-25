//! A stalled reclaimed object's host callback must not delay another mutator's recovery.
#![cfg(not(target_family = "wasm"))]
#![expect(unsafe_code, reason = "the fixture contains no managed edges")]
#![expect(clippy::expect_used, clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, Finalize, GarbageCollector, Gc, Result, Trace, TraceProgress};
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

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

fn exercise(stage: Stage) -> Result<()> {
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
    let first_gc = gc.clone();
    let first = std::thread::spawn(move || first_gc.collect_full());
    ready
        .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }))
        .map_err(|error| {
            ristretto_gc::Error::CollectionPhaseError(format!("host callback: {error}"))
        })?;
    // The first snapshot kept this object alive. A later recovery cycle must
    // reclaim it while the earlier detached callback is still stalled.
    drop(garbage);
    let mut allocation = Box::pin(Gc::try_new_async(&gc, 11_u64));
    let mut context = Context::from_waker(Waker::noop());
    let mut result = None;
    for _ in 0..2048 {
        if let Poll::Ready(value) = allocation.as_mut().poll(&mut context) {
            result = Some(value);
            break;
        }
    }
    let still_parked = !callback.released.load(Ordering::Acquire);
    let progress = gc.statistics()?;
    drop(allocation);
    drop(resume);
    first.join().map_err(|_| {
        ristretto_gc::Error::CollectionPhaseError("first mutator panicked".into())
    })??;
    let allocated = result.expect("another mutator's recovery awaited the host callback")?;
    assert!(still_parked, "host callback failure deadline expired");
    assert!(
        progress.collections_started >= 2 && progress.managed_bytes_freed >= scalar_size,
        "a later cycle did not reclaim the unrelated object"
    );
    assert_eq!(**allocated, 11);
    drop(allocated);
    gc.stop()?;
    assert_eq!(callback.finalized.load(Ordering::Relaxed), 1);
    assert_eq!(callback.cleared.load(Ordering::Relaxed), 1);
    assert_eq!(callback.dropped.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn default_recovery_progresses_during_a_stalled_finalizer() -> Result<()> {
    exercise(Stage::Finalize)
}
#[test]
fn default_recovery_progresses_during_a_stalled_clear_callback() -> Result<()> {
    exercise(Stage::Clear)
}
#[test]
fn default_recovery_progresses_during_a_stalled_destructor() -> Result<()> {
    exercise(Stage::Drop)
}
